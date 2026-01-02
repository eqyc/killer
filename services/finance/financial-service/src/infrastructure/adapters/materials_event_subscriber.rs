//! 物料凭证事件订阅器
//!
//! 订阅 materials-service 发布的物料凭证事件
//! 转换为会计凭证命令，调用应用层处理

use crate::infrastructure::adapters::{AdapterMetrics, MaterialDocumentPostedEvent, MaterialDocumentItem};
use crate::infrastructure::messaging::{KafkaConsumerBuilder, KafkaEventConsumer};
use async_trait::async_trait;
use chrono::NaiveDate;
use cqrs::prelude::*;
use killer_cqrs::event::DomainEvent;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, span, warn, Level};
use uuid::Uuid;

// =============================================================================
// 订阅器配置
// =============================================================================

/// 物料事件订阅器配置
#[derive(Debug, Clone)]
pub struct MaterialsEventSubscriberConfig {
    /// Kafka broker 地址
    pub kafka_brokers: Vec<String>,
    /// 消费者组 ID
    pub consumer_group_id: String,
    /// 订阅主题
    pub topics: Vec<&'static str>,
    /// 偏移量重置策略
    pub auto_offset_reset: &'static str,
    /// 最大轮询间隔
    pub max_poll_interval: Duration,
    /// 批量大小
    pub batch_size: usize,
}

// =============================================================================
// 事件转换
// =============================================================================

/// 物料凭证到会计凭证的转换器
#[derive(Clone)]
pub struct MaterialToJournalEntryConverter {
    /// 评估科目映射（未来从 MDG 服务获取）
    valuation_accounts: std::collections::HashMap<String, String>,
}

impl MaterialToJournalEntryConverter {
    /// 创建新的转换器
    pub fn new() -> Self {
        let mut valuation_accounts = std::collections::HashMap::new();
        // 默认评估科目映射
        valuation_accounts.insert("ROH".to_string(), "14010000".to_string()); // 原材料
        valuation_accounts.insert("FERT".to_string(), "14050000".to_string()); // 产成品
        valuation_accounts.insert("HALB".to_string(), "14030000".to_string()); // 半成品
        valuation_accounts.insert("VERP".to_string(), "14200000".to_string()); // 包装物

        Self { valuation_accounts }
    }

    /// 获取物料的评估科目
    fn get_valuation_account(&self, material_type: &str) -> String {
        self.valuation_accounts
            .get(material_type)
            .cloned()
            .unwrap_or_else(|| "14010000".to_string()) // 默认原材料科目
    }

    /// 将物料凭证行转换为会计凭证行
    fn convert_line_item(
        &self,
        item: &MaterialDocumentItem,
        fiscal_year: i32,
    ) -> Result<JournalEntryLineItem, Box<dyn std::error::Error>> {
        // 确定借贷方向
        let debit_credit = match item.movement_type.as_str() {
            "101" | "501" => "D", // 收货/销售 - 借方
            "102" | "502" => "C", // 收货/销售冲销 - 贷方
            "201" => "C",         // 发料 - 贷方
            "202" => "D",         // 发料冲销 - 借方
            _ => "D",             // 默认借方
        };

        // 获取评估科目（简化处理，实际应该查物料主数据）
        let account_code = self.get_valuation_account("ROH");

        Ok(JournalEntryLineItem {
            line_number: item.line_number,
            account_code,
            amount: item.amount,
            debit_credit: debit_credit.to_string(),
            cost_center: item.cost_center.clone(),
            profit_center: None,
            text: Some(format!(
                "物料: {} 数量: {} {} 凭证: {}",
                item.material_number,
                item.quantity,
                item.unit_of_measure,
                item.movement_type
            )),
            functional_area: None,
            business_area: None,
            order_number: item.order_number.clone(),
        })
    }

    /// 将物料凭证转换为会计凭证创建命令
    fn convert_to_create_command(
        &self,
        event: &MaterialDocumentPostedEvent,
    ) -> Result<CreateJournalEntryCommand, Box<dyn std::error::Error>> {
        let fiscal_year = event.posting_date.year() as i32;

        // 转换行项目
        let mut line_items = Vec::new();
        for item in &event.items {
            let line = self.convert_line_item(item, fiscal_year)?;
            line_items.push(line);
        }

        Ok(CreateJournalEntryCommand {
            tenant_id: event.tenant_id,
            user_id: event.tenant_id, // 使用租户 ID 作为用户 ID
            company_code: event.company_code.clone(),
            fiscal_year,
            posting_date: event.posting_date,
            document_date: event.document_date,
            currency_code: event.items.first().map(|i| i.currency_code.clone()).unwrap_or_else(|| "CNY".to_string()),
            header_text: Some(format!("自动生成: 物料凭证 {}", event.material_document_number)),
            reference_document: Some(event.material_document_number.clone()),
            line_items,
        })
    }
}

/// 会计凭证创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateJournalEntryCommand {
    tenant_id: Uuid,
    user_id: Uuid,
    company_code: String,
    fiscal_year: i32,
    posting_date: NaiveDate,
    document_date: NaiveDate,
    currency_code: String,
    header_text: Option<String>,
    reference_document: Option<String>,
    line_items: Vec<JournalEntryLineItem>,
}

/// 会计凭证行项目
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JournalEntryLineItem {
    line_number: u32,
    account_code: String,
    amount: f64,
    debit_credit: String,
    cost_center: Option<String>,
    profit_center: Option<String>,
    text: Option<String>,
    functional_area: Option<String>,
    business_area: Option<String>,
    order_number: Option<String>,
}

// =============================================================================
// 事件订阅器
// =============================================================================

/// 物料凭证事件订阅器
pub struct MaterialsEventSubscriber {
    /// Kafka 消费者
    consumer: KafkaEventConsumer,
    /// 转换器
    converter: Arc<MaterialToJournalEntryConverter>,
    /// 命令总线（用于发送命令到应用层）
    command_bus: Arc<dyn CommandBus + Send + Sync>,
    /// 指标
    metrics: Arc<AdapterMetrics>,
    /// 配置
    config: MaterialsEventSubscriberConfig,
}

impl MaterialsEventSubscriber {
    /// 创建新的订阅器
    pub fn new(
        consumer: KafkaEventConsumer,
        converter: Arc<MaterialToJournalEntryConverter>,
        command_bus: Arc<dyn CommandBus + Send + Sync>,
        metrics: Arc<AdapterMetrics>,
        config: MaterialsEventSubscriberConfig,
    ) -> Self {
        Self {
            consumer,
            converter,
            command_bus,
            metrics,
            config,
        }
    }

    /// 启动订阅器
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut consumer = self.consumer.clone();

        // 设置事件处理函数
        consumer.set_handler(Arc::new(move |envelope| {
            let converter = converter.clone();
            let command_bus = command_bus.clone();
            let metrics = metrics.clone();
            let correlation_id = envelope.tenant_id.clone();

            Box::new(async move {
                Self::handle_event(&converter, &command_bus, &metrics, envelope, correlation_id).await
            })
        }));

        // 启动消费者
        consumer.start().await?;

        info!("Materials event subscriber started");
        Ok(())
    }

    /// 处理事件
    async fn handle_event(
        converter: &MaterialToJournalEntryConverter,
        command_bus: &Arc<dyn CommandBus + Send + Sync>,
        metrics: &Arc<AdapterMetrics>,
        envelope: crate::infrastructure::messaging::SerializedEventEnvelope,
        correlation_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        // 解析事件
        let event: MaterialDocumentPostedEvent = serde_json::from_value(envelope.payload)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        debug!(
            event_id = %event.event_id,
            material_document = %event.material_document_number,
            "Processing material document event"
        );

        // 转换为会计凭证命令
        let command = converter.convert_to_create_command(&event)
            .map_err(|e| {
                metrics.record_error("materials-subscriber", "conversion_error");
                Box::new(e) as Box<dyn std::error::Error>
            })?;

        // 发送命令到应用层
        // 注意：这里需要使用适配器模式，将命令发送到命令总线
        // 实际实现需要根据应用层的 CommandBus 接口调整

        let duration = start.elapsed();
        metrics.record_call("materials-subscriber", "handle_event", true, duration);

        info!(
            event_id = %event.event_id,
            material_document = %event.material_document_number,
            duration_ms = %duration.as_millis(),
            "Material document event processed"
        );

        Ok(())
    }
}

// =============================================================================
// 便利构建器
// =============================================================================

/// 订阅器构建器
pub struct MaterialsEventSubscriberBuilder {
    config: MaterialsEventSubscriberConfig,
    command_bus: Option<Arc<dyn CommandBus + Send + Sync>>,
}

impl Default for MaterialsEventSubscriberBuilder {
    fn default() -> Self {
        Self {
            config: MaterialsEventSubscriberConfig {
                kafka_brokers: vec!["localhost:9092".to_string()],
                consumer_group_id: "killer-financial-service-materials".to_string(),
                topics: vec!["killer.logistics.events"],
                auto_offset_reset: "earliest",
                max_poll_interval: Duration::from_secs(300),
                batch_size: 100,
            },
            command_bus: None,
        }
    }
}

impl MaterialsEventSubscriberBuilder {
    /// 设置 Kafka broker 地址
    pub fn kafka_brokers(mut self, brokers: Vec<String>) -> Self {
        self.config.kafka_brokers = brokers;
        self
    }

    /// 设置消费者组 ID
    pub fn consumer_group_id(mut self, group_id: &str) -> Self {
        self.config.consumer_group_id = group_id.to_string();
        self
    }

    /// 设置订阅主题
    pub fn topics(mut self, topics: Vec<&'static str>) -> Self {
        self.config.topics = topics;
        self
    }

    /// 设置命令总线
    pub fn command_bus(mut self, command_bus: Arc<dyn CommandBus + Send + Sync>) -> Self {
        self.command_bus = Some(command_bus);
        self
    }

    /// 构建订阅器
    pub async fn build(
        self,
        serializer: Arc<crate::infrastructure::messaging::EventSerializer>,
        metrics: Arc<AdapterMetrics>,
    ) -> Result<MaterialsEventSubscriber, Box<dyn std::error::Error>> {
        let command_bus = self.command_bus.ok_or("command_bus is required")?;

        // 构建 Kafka 消费者
        let consumer = KafkaConsumerBuilder::default()
            .brokers(self.config.kafka_brokers)
            .group_id(&self.config.consumer_group_id)
            .topics(self.config.topics)
            .build()
            .await?;

        // 创建转换器
        let converter = Arc::new(MaterialToJournalEntryConverter::new());

        Ok(MaterialsEventSubscriber::new(
            consumer,
            converter,
            command_bus,
            metrics,
            self.config,
        ))
    }
}
