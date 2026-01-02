//! 读模型投影模块
//!
//! 将领域事件投影到 ClickHouse 读模型
//! 支持批量插入、幂等处理、延迟监控

use crate::infrastructure::messaging::{KafkaConsumerBuilder, KafkaEventConsumer, SerializedEventEnvelope};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn, Level};
use uuid::Uuid;

// =============================================================================
// ClickHouse 配置
// =============================================================================

/// ClickHouse 配置
#[derive(Debug, Clone)]
pub struct ClickHouseConfig {
    /// 连接 URL
    pub url: String,
    /// 数据库名
    pub database: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 操作超时
    pub command_timeout: Duration,
}

// =============================================================================
// ClickHouse 客户端
// =============================================================================

/// ClickHouse 客户端封装
#[derive(Clone)]
pub struct ClickHouseClient {
    /// HTTP 端点
    endpoint: String,
    /// 数据库名
    database: String,
    /// 用户名
    username: String,
    /// 密码
    password: String,
    /// HTTP 客户端
    client: reqwest::Client,
}

impl ClickHouseClient {
    /// 创建新的客户端
    pub fn new(config: &ClickHouseConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.command_timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            endpoint: format!("{}/?database={}", config.url, config.database),
            database: config.database.clone(),
            username: config.username.clone(),
            password: config.password.clone(),
            client,
        }
    }

    /// 执行查询
    pub async fn query(&self, sql: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.username, Some(&self.password))
            .body(sql.to_string())
            .send()
            .await?
            .error_for_status()?;

        if !response.status().is_success() {
            let text = response.text().await?;
            return Err(format!("ClickHouse query failed: {}", text).into());
        }

        Ok(())
    }

    /// 插入数据
    pub async fn insert(&self, table: &str, values: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let values_str = values.join(",\n");
        let sql = format!(
            "INSERT INTO {} (tenant_id, document_number, line_number, account_code, amount, debit_credit, posting_date, company_code, fiscal_year, cost_center, profit_center, text, created_at) VALUES {}",
            table,
            values_str
        );

        self.query(&sql).await?;
        Ok(())
    }

    /// 批量插入
    pub async fn insert_batch(&self, table: &str, batches: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        for batch in batches {
            self.insert(table, std::slice::from_ref(batch)).await?;
        }
        Ok(())
    }
}

// =============================================================================
// 投影表结构
// =============================================================================

/// 凭证行项目读模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLineReadModel {
    /// 租户 ID
    pub tenant_id: Uuid,
    /// 凭证号
    pub document_number: String,
    /// 行号
    pub line_number: u32,
    /// 会计科目代码
    pub account_code: String,
    /// 金额
    pub amount: f64,
    /// 借贷方向
    pub debit_credit: String,
    /// 过账日期
    pub posting_date: NaiveDate,
    /// 公司代码
    pub company_code: String,
    /// 会计年度
    pub fiscal_year: i32,
    /// 成本中心
    pub cost_center: Option<String>,
    /// 利润中心
    pub profit_center: Option<String>,
    /// 行项目文本
    pub text: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 试算平衡表读模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceReadModel {
    /// 租户 ID
    pub tenant_id: Uuid,
    /// 公司代码
    pub company_code: String,
    /// 会计年度
    pub fiscal_year: i32,
    /// 会计科目代码
    pub account_code: String,
    /// 科目名称
    pub account_name: Option<String>,
    /// 借方金额
    pub debit_amount: f64,
    /// 贷方金额
    pub credit_amount: f64,
    /// 期间
    pub period: u8,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// 投影器实现
// =============================================================================

/// 事件投影器配置
#[derive(Debug, Clone)]
pub struct ProjectionWorkerConfig {
    /// Kafka 配置
    pub kafka_brokers: Vec<String>,
    pub consumer_group_id: String,
    pub topics: Vec<&'static str>,
    /// 批量处理配置
    pub batch_size: usize,
    pub batch_timeout: Duration,
    /// ClickHouse 配置
    pub clickhouse: ClickHouseConfig,
    /// 重试配置
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for ProjectionWorkerConfig {
    fn default() -> Self {
        Self {
            kafka_brokers: vec!["localhost:9092".to_string()],
            consumer_group_id: "killer-financial-service-projection".to_string(),
            topics: vec!["killer.finance.events"],
            batch_size: 100,
            batch_timeout: Duration::from_secs(10),
            clickhouse: ClickHouseConfig {
                url: "http://localhost:8123".to_string(),
                database: "financial".to_string(),
                username: "default".to_string(),
                password: "".to_string(),
                max_connections: 10,
                command_timeout: Duration::from_secs(30),
            },
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

/// 投影 Worker
pub struct ProjectionWorker {
    /// ClickHouse 客户端
    clickhouse: Arc<ClickHouseClient>,
    /// Kafka 消费者
    consumer: KafkaEventConsumer,
    /// 指标
    metrics: Arc<ProjectionMetrics>,
    /// 配置
    config: ProjectionWorkerConfig,
    /// 事件缓冲
    buffer: Arc<tokio::sync::Mutex<Vec<SerializedEventEnvelope>>>,
    /// 最后刷新时间
    last_flush: Arc<tokio::sync::Mutex<Instant>>,
    /// 停止信号
    stop_tx: broadcast::Sender<()>,
}

impl ProjectionWorker {
    /// 创建新的投影 Worker
    pub async fn new(
        config: ProjectionWorkerConfig,
        serializer: Arc<crate::infrastructure::messaging::EventSerializer>,
        metrics: Arc<ProjectionMetrics>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // 创建 ClickHouse 客户端
        let clickhouse = Arc::new(ClickHouseClient::new(&config.clickhouse));

        // 初始化表结构
        Self::init_tables(&clickhouse).await?;

        // 创建 Kafka 消费者
        let consumer = KafkaConsumerBuilder::default()
            .brokers(config.kafka_brokers.clone())
            .group_id(&config.consumer_group_id)
            .topics(config.topics.clone())
            .build()
            .await?;

        Ok(Self {
            clickhouse,
            consumer,
            metrics,
            config,
            buffer: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            last_flush: Arc::new(tokio::sync::Mutex::new(Instant::now())),
            stop_tx: broadcast::channel(1).0,
        })
    }

    /// 初始化表结构
    async fn init_tables(clickhouse: &Arc<ClickHouseClient>) -> Result<(), Box<dyn std::error::Error>> {
        // 创建 journal_lines 表
        let create_journal_lines = r#"
            CREATE TABLE IF NOT EXISTS journal_lines (
                tenant_id UUID,
                document_number String,
                line_number UInt32,
                account_code String,
                amount Decimal(18, 2),
                debit_credit String,
                posting_date Date,
                company_code String,
                fiscal_year Int32,
                cost_center String,
                profit_center String,
                text String,
                created_at DateTime,
                event_id UUID,
                _event_time DateTime DEFAULT now()
            ) ENGINE = ReplacingMergeTree(_event_time)
            PARTITION BY (toYYYYMM(posting_date), tenant_id)
            ORDER BY (tenant_id, company_code, fiscal_year, account_code, posting_date)
            SETTINGS index_granularity = 8192
        "#;

        clickhouse.query(create_journal_lines).await?;

        // 创建 trial_balance 表
        let create_trial_balance = r#"
            CREATE TABLE IF NOT EXISTS trial_balance (
                tenant_id UUID,
                company_code String,
                fiscal_year Int32,
                account_code String,
                account_name String,
                debit_amount Decimal(18, 2),
                credit_amount Decimal(18, 2),
                period UInt8,
                updated_at DateTime,
                _event_time DateTime DEFAULT now()
            ) ENGINE = SummingMergeTree((debit_amount, credit_amount))
            PARTITION BY (fiscal_year, tenant_id)
            ORDER BY (tenant_id, company_code, fiscal_year, account_code, period)
            SETTINGS index_granularity = 8192
        "#;

        clickhouse.query(create_trial_balance).await?;

        info!("ClickHouse tables initialized");
        Ok(())
    }

    /// 启动投影 Worker
    pub async fn start(&self) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
        let mut consumer = self.consumer.clone();
        let buffer = self.buffer.clone();
        let clickhouse = self.clickhouse.clone();
        let metrics = self.metrics.clone();
        let config = self.config.clone();
        let last_flush = self.last_flush.clone();
        let mut stop_rx = self.stop_tx.subscribe();

        let handle = tokio::spawn(async move {
            // 启动定期刷新任务
            let flush_handle = Self::start_flush_task(
                buffer.clone(),
                clickhouse.clone(),
                metrics.clone(),
                config.batch_size,
                config.batch_timeout,
                stop_rx.clone(),
            );

            // 设置事件处理函数
            consumer.set_handler(Arc::new(move |envelope| {
                let buffer = buffer.clone();
                let metrics = metrics.clone();

                Box::new(async move {
                    let mut buf = buffer.lock().await;
                    buf.push(envelope);

                    if buf.len() >= config.batch_size {
                        Self::flush(&buffer, &clickhouse, &metrics).await;
                    }

                    Ok(())
                })
            }));

            // 启动消费者
            consumer.start().await?;

            // 等待停止信号
            let _ = stop_rx.recv().await;

            // 停止消费者
            consumer.stop().await;

            // 刷新剩余事件
            Self::flush(&buffer, &clickhouse, &metrics).await;

            info!("Projection worker stopped");
        });

        Ok(handle)
    }

    /// 启动定期刷新任务
    fn start_flush_task(
        buffer: Arc<tokio::sync::Mutex<Vec<SerializedEventEnvelope>>>,
        clickhouse: Arc<ClickHouseClient>,
        metrics: Arc<ProjectionMetrics>,
        batch_size: usize,
        batch_timeout: Duration,
        mut stop_rx: broadcast::Receiver<()>,
    ) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(batch_timeout);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let buf = buffer.lock().await;
                        if !buf.is_empty() {
                            Self::flush(&buffer, &clickhouse, &metrics).await;
                        }
                    }
                    _ = stop_rx.recv() => {
                        break;
                    }
                }
            }
        });
    }

    /// 刷新缓冲到 ClickHouse
    async fn flush(
        buffer: &Arc<tokio::sync::Mutex<Vec<SerializedEventEnvelope>>>,
        clickhouse: &Arc<ClickHouseClient>,
        metrics: &Arc<ProjectionMetrics>,
    ) {
        let mut buf = buffer.lock().await;
        if buf.is_empty() {
            return;
        }

        let events = buf.clone();
        buf.clear();

        let start = std::time::Instant::now();
        let count = events.len();

        // 处理事件并批量插入
        let values: Vec<String> = events
            .iter()
            .filter_map(|e| Self::event_to_insert_values(e).ok())
            .collect();

        if !values.is_empty() {
            if let Err(e) = clickhouse.insert_batch("journal_lines", &values).await {
                metrics.record_projection_error("insert_error");
                error!(error = %e, "Failed to insert to ClickHouse");
            } else {
                metrics.record_events_projected(count);
                debug!(count, "Events projected to ClickHouse");
            }
        }

        let duration = start.elapsed();
        metrics.record_projection_duration(duration);
    }

    /// 将事件转换为插入值
    fn event_to_insert_values(envelope: &SerializedEventEnvelope) -> Result<String, Box<dyn std::error::Error>> {
        // 解析事件类型
        let event_type = envelope.event_type.as_str();

        match event_type {
            "journal_entry_posted" => {
                // 解析载荷
                let payload = &envelope.payload;
                let tenant_id = payload["tenant_id"].as_str().unwrap_or("");
                let document_number = payload["document_number"].as_str().unwrap_or("");
                let company_code = payload["company_code"].as_str().unwrap_or("");
                let fiscal_year = payload["fiscal_year"].as_i64().unwrap_or(0);
                let posting_date = payload["posting_date"].as_str().unwrap_or("");

                // 行项目在 line_items 中
                let line_items = payload["line_items"].as_array().unwrap_or(&vec![]);

                let mut values = Vec::new();
                for (i, item) in line_items.iter().enumerate() {
                    let line_number = i as u32 + 1;
                    let account_code = item["account_code"].as_str().unwrap_or("");
                    let amount = item["amount"].as_f64().unwrap_or(0.0);
                    let debit_credit = item["debit_credit"].as_str().unwrap_or("");
                    let cost_center = item["cost_center"].as_str().unwrap_or("");
                    let profit_center = item["profit_center"].as_str().unwrap_or("");
                    let text = item["text"].as_str().unwrap_or("");

                    let value = format!(
                        "({}, '{}', {}, '{}', {}, '{}', '{}', '{}', {}, '{}', '{}', '{}', now(), '{}')",
                        tenant_id,
                        document_number,
                        line_number,
                        account_code,
                        amount,
                        debit_credit,
                        posting_date,
                        company_code,
                        fiscal_year,
                        cost_center,
                        profit_center,
                        text,
                        envelope.event_id
                    );
                    values.push(value);
                }

                Ok(values.join(",\n"))
            }
            _ => {
                // 跳过不支持的事件类型
                Ok(String::new())
            }
        }
    }

    /// 停止投影 Worker
    pub async fn stop(&self) {
        let _ = self.stop_tx.send(());
    }
}

// =============================================================================
// 投影指标
// =============================================================================

/// 投影指标
#[derive(Default)]
pub struct ProjectionMetrics {
    events_projected_total: prometheus::IntCounterVec,
    projection_errors_total: prometheus::IntCounterVec,
    projection_duration: prometheus::HistogramVec,
    buffer_size: prometheus::GaugeVec,
    projection_lag: prometheus::GaugeVec,
}

impl ProjectionMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            events_projected_total: prometheus::register_int_counter_vec!(
                "projection_events_projected_total",
                "Total events projected to ClickHouse",
                &["event_type"]
            )?,
            projection_errors_total: prometheus::register_int_counter_vec!(
                "projection_errors_total",
                "Total projection errors",
                &["error_type"]
            )?,
            projection_duration: prometheus::register_histogram_vec!(
                "projection_duration_seconds",
                "Projection duration in seconds",
                &[]
            )?,
            buffer_size: prometheus::register_gauge_vec!(
                "projection_buffer_size",
                "Current projection buffer size",
                &[]
            )?,
            projection_lag: prometheus::register_gauge_vec!(
                "projection_lag_seconds",
                "Projection lag in seconds",
                &["consumer_group"]
            )?,
        })
    }

    pub fn record_events_projected(&self, count: usize) {
        self.events_projected_total
            .with_label_values(&["all"])
            .inc_by(count as u64);
    }

    pub fn record_projection_error(&self, error_type: &str) {
        self.projection_errors_total
            .with_label_values(&[error_type])
            .inc();
    }

    pub fn record_projection_duration(&self, duration: std::time::Duration) {
        self.projection_duration
            .with_label_values(&[])
            .observe(duration.as_secs_f64());
    }
}
