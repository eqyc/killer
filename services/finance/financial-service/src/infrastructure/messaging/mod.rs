//! 消息模块
//!
//! 提供 Kafka 消息发布和消费支持：
//! - OutboxPublisher: 从发件箱拉取事件并发布到 Kafka
//! - KafkaProducer: 事件序列化并发布
//! - KafkaConsumer: 消费外部服务事件
//!
//! ## 主题命名规范
//!
//! - killer.finance.events: 财务领域事件
//! - killer.logistics.events: 物流领域事件（如物料凭证）
//! - 分区策略: tenant_id 作为分区键，确保同一租户的事件有序

#[cfg(feature = "messaging")]
pub mod kafka_producer;

#[cfg(feature = "messaging")]
pub mod outbox_publisher;

#[cfg(feature = "messaging")]
pub mod event_serializer;

#[cfg(feature = "messaging")]
pub mod kafka_consumer;

#[cfg(feature = "messaging")]
pub use kafka_producer::KafkaEventProducer;

#[cfg(feature = "messaging")]
pub use outbox_publisher::OutboxPublisherWorker;

#[cfg(feature = "messaging")]
pub use event_serializer::{EventSerializer, SerializedEventEnvelope};

#[cfg(feature = "messaging")]
pub use kafka_consumer::{KafkaEventConsumer, KafkaConsumerBuilder};

// =============================================================================
// 共享类型
// =============================================================================

/// Kafka 消息配置
#[derive(Debug, Clone)]
pub struct KafkaMessageConfig {
    /// 事件主题前缀
    pub events_topic_prefix: String,
    /// 生产者配置
    pub producer: KafkaProducerConfig,
    /// 消费者配置
    pub consumer: KafkaConsumerConfig,
}

/// Kafka 生产者配置
#[derive(Debug, Clone)]
pub struct KafkaProducerConfig {
    /// acks 模式
    pub acks: String,
    /// 重试次数
    pub retries: i32,
    /// 批量大小
    pub batch_size: usize,
    /// linger ms
    pub linger_ms: u64,
    /// 压缩
    pub compression: String,
}

/// Kafka 消费者配置
#[derive(Debug, Clone)]
pub struct KafkaConsumerConfig {
    /// 自动提交
    pub enable_auto_commit: bool,
    /// 自动提交间隔
    pub auto_commit_interval_ms: u32,
    /// 会话超时
    pub session_timeout_ms: u32,
    /// 最大轮询记录数
    pub max_poll_records: usize,
}

/// Kafka 消息指标
#[derive(Default)]
pub struct KafkaMetrics {
    events_published_total: prometheus::IntCounterVec,
    events_consumed_total: prometheus::IntCounterVec,
    publish_duration: prometheus::HistogramVec,
    consume_duration: prometheus::HistogramVec,
    partition_lag: prometheus::GaugeVec,
}

impl KafkaMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            events_published_total: prometheus::register_int_counter_vec!(
                "kafka_events_published_total",
                "Total Kafka events published",
                &["topic", "status"]
            )?,
            events_consumed_total: prometheus::register_int_counter_vec!(
                "kafka_events_consumed_total",
                "Total Kafka events consumed",
                &["topic", "status"]
            )?,
            publish_duration: prometheus::register_histogram_vec!(
                "kafka_publish_duration_seconds",
                "Kafka publish duration in seconds",
                &["topic"]
            )?,
            consume_duration: prometheus::register_histogram_vec!(
                "kafka_consume_duration_seconds",
                "Kafka consume duration in seconds",
                &["topic"]
            )?,
            partition_lag: prometheus::register_gauge_vec!(
                "kafka_partition_lag",
                "Kafka consumer partition lag",
                &["topic", "partition"]
            )?,
        })
    }

    pub fn record_publish(&self, topic: &str, success: bool, duration: std::time::Duration) {
        let status = if success { "success" } else { "failure" };
        self.events_published_total
            .with_label_values(&[topic, status])
            .inc();
        self.publish_duration
            .with_label_values(&[topic])
            .observe(duration.as_secs_f64());
    }

    pub fn record_consume(&self, topic: &str, success: bool, duration: std::time::Duration) {
        let status = if success { "success" } else { "failure" };
        self.events_consumed_total
            .with_label_values(&[topic, status])
            .inc();
        self.consume_duration
            .with_label_values(&[topic])
            .observe(duration.as_secs_f64());
    }

    pub fn update_partition_lag(&self, topic: &str, partition: i32, lag: i64) {
        self.partition_lag
            .with_label_values(&[topic, &partition.to_string()])
            .set(lag);
    }
}

/// 序列化后的 Kafka 事件
#[derive(Debug, Clone)]
pub struct SerializedEvent {
    /// 事件 ID
    pub event_id: uuid::Uuid,
    /// 事件类型
    pub event_type: String,
    /// 事件版本
    pub schema_version: u32,
    /// 聚合根类型
    pub aggregate_type: String,
    /// 聚合根 ID
    pub aggregate_id: String,
    /// 租户 ID
    pub tenant_id: String,
    /// 发生时间
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    /// 序列化后的负载
    pub payload: Vec<u8>,
    /// 负载内容类型
    pub content_type: String,
}
