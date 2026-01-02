//! 物料凭证过账事件处理器（FI-MM 集成）
//!
//! 处理来自 materials-service 的 MaterialDocumentPostedEvent
//! 自动生成会计凭证，实现 FI-MM 集成

use crate::application::commands::*;
use crate::application::dto::*;
use crate::application::error::ApplicationError;
use crate::application::events::*;
use crate::domain::events::*;
use crate::domain::repositories::*;
use crate::domain::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, error, info, span, Level};

// =============================================================================
// 事件定义（来自物料服务）
// =============================================================================

/// 物料凭证过账事件（从 Kafka 消费）
#[derive(Debug, Clone)]
pub struct MaterialDocumentPostedEvent {
    /// 事件元数据
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub correlation_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// 物料凭证信息
    pub material_document_number: String,
    pub company_code: String,
    pub posting_date: chrono::NaiveDate,
    pub document_date: chrono::NaiveDate,

    /// 行项目
    pub items: Vec<MaterialDocumentItem>,
}

#[derive(Debug, Clone)]
pub struct MaterialDocumentItem {
    pub line_number: u32,
    pub material_number: String,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub amount: f64,
    pub currency_code: String,
    pub cost_center: Option<String>,
    pub order_number: Option<String>,
    pub movement_type: String, // e.g., "101" for GR, "102" for GR reversal
}

// =============================================================================
// 事件处理器
// =============================================================================

/// 物料凭证事件处理器（FI-MM 集成）
#[derive(Clone)]
pub struct MaterialDocumentPostedHandler<JR, DOC, EB>
where
    JR: JournalEntryRepository,
    DOC: DocumentNumberGenerator,
    EB: EventBus,
{
    /// 凭证仓储
    journal_entry_repo: Arc<JR>,
    /// 凭证号生成器
    document_number_gen: Arc<DOC>,
    /// 事件总线
    event_bus: Arc<EB>,
    /// 重试配置
    retry_config: RetryConfig,
}

impl<JR, DOC, EB> MaterialDocumentPostedHandler<JR, DOC, EB>
where
    JR: JournalEntryRepository,
    DOC: DocumentNumberGenerator,
    EB: EventBus,
{
    pub fn new(
        journal_entry_repo: Arc<JR>,
        document_number_gen: Arc<DOC>,
        event_bus: Arc<EB>,
    ) -> Self {
        Self {
            journal_entry_repo,
            document_number_gen,
            event_bus,
            retry_config: RetryConfig::default(),
        }
    }

    /// 将物料凭证行转换为会计凭证行
    fn convert_to_journal_entry_lines(
        &self,
        items: &[MaterialDocumentItem],
        fiscal_year: i32,
    ) -> Result<Vec<JournalEntryLineItemRequest>, ApplicationError> {
        let current_year = chrono::Utc::now().year();

        items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                // 确定借贷方向：收货为借方，冲销为贷方
                let debit_credit = if item.movement_type == "101" {
                    "D".to_string() // GR - 借方
                } else if item.movement_type == "102" {
                    "C".to_string() // GR reversal - 贷方
                } else {
                    // 默认根据业务逻辑判断
                    "D".to_string()
                };

                Ok(JournalEntryLineItemRequest {
                    line_number: item.line_number,
                    account_code: self.get_valuation_account(&item.material_number)?,
                    amount: item.amount,
                    debit_credit,
                    cost_center: item.cost_center.clone(),
                    profit_center: None,
                    text: Some(format!(
                        "物料: {} 数量: {} {} 参考: {}",
                        item.material_number,
                        item.quantity,
                        item.unit_of_measure,
                        item.movement_type
                    )),
                    functional_area: None,
                    business_area: None,
                    order_number: item.order_number.clone(),
                })
            })
            .collect()
    }

    /// 获取物料的评估科目（从主数据服务获取）
    fn get_valuation_account(
        &self,
        _material_number: &str,
    ) -> Result<String, ApplicationError> {
        // TODO: 从主数据服务获取物料的评估科目
        // 这里需要调用 master-data-service 的 API
        // 暂时返回默认科目，实际实现需要从缓存或服务获取
        Ok("14010000".to_string()) // 原材料科目
    }
}

#[async_trait::async_trait]
impl<JR, DOC, EB> EventHandler<MaterialDocumentPostedEvent> for MaterialDocumentPostedHandler<JR, DOC, EB>
where
    JR: JournalEntryRepository + Send + Sync,
    DOC: DocumentNumberGenerator + Send + Sync,
    EB: EventBus + Send + Sync,
{
    async fn handle(
        &self,
        envelope: &EventEnvelope<MaterialDocumentPostedEvent>,
    ) -> EventResult {
        let start_time = std::time::Instant::now();
        let event = &envelope.payload;
        let span = span!(Level::DEBUG, "MaterialDocumentPostedHandler", tenant_id = %event.tenant_id, material_document = %event.material_document_number);
        let _guard = span.enter();

        async move {
            let tenant_id = event.tenant_id;
            let material_document_number = event.material_document_number.clone();

            debug!(%tenant_id, %material_document_number, "Processing MaterialDocumentPostedEvent");

            // 1. 检查是否已处理（幂等性）
            // TODO: 检查是否已有与此物料凭证关联的会计凭证

            // 2. 生成会计凭证号
            let document_number = self
                .document_number_gen
                .generate(&tenant_id, &event.company_code)
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to generate document number: {:?}",
                    e
                )))?;

            // 3. 将物料凭证行转换为会计凭证行
            let line_items = self.convert_to_journal_entry_lines(
                &event.items,
                event.posting_date.year() as i32,
            )?;

            // 4. 创建会计凭证
            let request = CreateJournalEntryRequest {
                company_code: event.company_code.clone(),
                fiscal_year: event.posting_date.year() as i32,
                posting_date: Some(event.posting_date),
                document_date: Some(event.document_date),
                currency_code: event.items.first()
                    .map(|i| i.currency_code.clone())
                    .unwrap_or_else(|| "CNY".to_string()),
                header_text: Some(format!("自动生成: 物料凭证 {}", material_document_number)),
                reference_document: Some(material_document_number.clone()),
                line_items,
            };

            // 5. 创建并保存凭证聚合根
            let line_items_domain = line_items_from_dto(request.line_items.clone())
                .map_err(|e| ApplicationError::validation_failed(format!(
                    "Failed to convert line items: {:?}",
                    e
                )))?;

            let aggregate = JournalEntry::create(
                tenant_id,
                document_number.clone(),
                request.company_code,
                request.fiscal_year,
                request.posting_date.unwrap(),
                request.document_date.unwrap(),
                request.currency_code,
                request.header_text,
                request.reference_document,
                line_items_domain,
            ).map_err(|e| ApplicationError::business_rule_violation(
                "CREATE_FAILED",
                format!("Failed to create journal entry: {:?}", e),
            ))?;

            // 6. 保存凭证
            self.journal_entry_repo
                .save(&aggregate)
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to save journal entry: {:?}",
                    e
                )))?;

            // 7. 发布会计凭证创建事件
            let events = aggregate.take_events();
            for domain_event in events {
                self.event_bus.publish(domain_event).await.map_err(|e|
                    ApplicationError::infrastructure_error(format!(
                        "Failed to publish event: {:?}", e
                    ))
                )?;
            }

            // 8. 发布物料凭证会计凭证已生成事件
            // self.event_bus.publish(JournalEntryCreatedFromMaterialEvent {
            //     event_id: Uuid::new_v4(),
            //     tenant_id,
            //     correlation_id: event.correlation_id,
            //     timestamp: chrono::Utc::now(),
            //     material_document_number,
            //     journal_entry_number: document_number,
            // }).await?;

            info!(%tenant_id, material_document = %material_document_number, journal_entry = %document_number, "Material document converted to journal entry successfully");

            Ok(())
        }
        .await
    }
}
