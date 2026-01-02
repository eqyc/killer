//! Kafka 事件消费者
//!
//! 从 Kafka 主题消费事件
//! 支持分区分配、偏移量管理、错误处理

use crate::infrastructure::messaging::{EventSerializer, KafkaMetrics, SerializedEventEnvelope};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::Offset::Offset;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn, Level, Span};
use uuid::Uuid;

// =============================================================================
// 消费者配置
// =============================================================================

/// Kafka 消费者配置
#[derive(Debug, Clone)]
pub struct KafkaConsumerConfig {
    /// Broker 地址列表
    pub brokers: Vec<String>,
    /// 消费者组 ID
    pub group_id: String,
    /// 客户端 ID
    pub client_id: String,
    /// 自动提交
    pub enable_auto_commit: bool,
    /// 自动提交间隔
    pub auto_commit_interval_ms: u32,
    /// 会话超时
    pub session_timeout_ms: u32,
    /// 最大轮询记录数
    pub max_poll_records: usize,
    /// 主题
    pub topics: Vec<String>,
    /// SASL 认证
    pub sasl: Option<KafkaSaslConfig>,
    /// SSL 配置
    pub ssl: Option<KafkaSslConfig>,
}

#[derive(Debug, Clone)]
pub struct KafkaSaslConfig {
    pub mechanism: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct KafkaSslConfig {
    pub ca_location: String,
    pub certificate_location: String,
    pub key_location: String,
    pub key_password: Option<String>,
}

// =============================================================================
// 事件处理器
// =============================================================================

/// 事件处理函数类型
pub type EventHandlerFn =
    Arc<dyn Fn(SerializedEventEnvelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// 消费者处理器 trait
#[async_trait::async_trait]
pub trait EventConsumerHandler: Send + Sync {
    /// 处理事件
    async fn handle(&self, envelope: SerializedEventEnvelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

// =============================================================================
// Kafka 事件消费者
// =============================================================================

/// Kafka 事件消费者
#[derive(Clone)]
pub struct KafkaEventConsumer {
    /// Kafka 消费者
    consumer: StreamConsumer,
    /// 序列化器
    serializer: Arc<EventSerializer>,
    /// 指标
    metrics: Arc<KafkaMetrics>,
    /// 事件处理函数
    handler: Option<EventHandlerFn>,
    /// 停止信号
    stop_tx: broadcast::Sender<()>,
    /// 运行状态
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl KafkaEventConsumer {
    /// 创建新的消费者
    pub async fn new(
        config: &KafkaConsumerConfig,
        serializer: Arc<EventSerializer>,
        metrics: Arc<KafkaMetrics>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_config = ClientConfig::new();

        // 基础配置
        client_config
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("group.id", &config.group_id)
            .set("client.id", &config.client_id)
            .set("enable.auto.commit", config.enable_auto_commit.to_string())
            .set("auto.commit.interval.ms", config.auto_commit_interval_ms.to_string())
            .set("session.timeout.ms", config.session_timeout_ms.to_string())
            .set("max.poll.records", config.max_poll_records.to_string())
            .set("auto.offset.reset", "earliest")
            .set("enable.partition.eof", "false");

        // SASL 认证
        if let Some(sasl) = &config.sasl {
            client_config
                .set("security.protocol", "SASL_SSL")
                .set("sasl.mechanism", &sasl.mechanism)
                .set("sasl.username", &sasl.username)
                .set("sasl.password", &sasl.password);
        }

        // SSL 配置
        if let Some(ssl) = &config.ssl {
            client_config
                .set("ssl.ca.location", &ssl.ca_location)
                .set("ssl.certificate.location", &ssl.certificate_location)
                .set("ssl.key.location", &ssl.key_location);
            if let Some(password) = &ssl.key_password {
                client_config.set("ssl.key.password", password);
            }
        }

        // 日志配置
        client_config.set("log.level", RDKafkaLogLevel::Warning.to_string());

        let consumer: StreamConsumer = client_config
            .create()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 订阅主题
        consumer.subscribe(&config.topics).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        info!(topics = ?config.topics, group_id = %config.group_id, "Kafka consumer created and subscribed");

        Ok(Self {
            consumer,
            serializer,
            metrics,
            handler: None,
            stop_tx: broadcast::channel(1).0,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// 设置事件处理函数
    pub fn set_handler(&mut self, handler: EventHandlerFn) {
        self.handler = Some(handler);
    }

    /// 启动消费循环
    pub async fn start(&self) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
        if self
            .running
            .compare_exchange(false, true, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst)
            .is_err()
        {
            return Err("Consumer is already running".into());
        }

        info!("Starting Kafka consumer");

        let consumer = self.consumer.clone();
        let serializer = self.serializer.clone();
        let metrics = self.metrics.clone();
        let handler = self.handler.clone();
        let mut stop_rx = self.stop_tx.subscribe();
        let topics = self.consumer.subscription().unwrap_or_default();

        let handle = tokio::spawn(async move {
            let mut poll_interval = tokio::time::interval(Duration::from_millis(100));

            loop {
                tokio::select! {
                    _ = stop_rx.recv() => {
                        info!("Stopping Kafka consumer");
                        break;
                    }
                    _ = poll_interval.tick() => {
                        match consumer.recv().await {
                            Ok(message) => {
                                if let Err(e) = Self::process_message(&consumer, &serializer, &metrics, &handler, &message).await {
                                    error!(error = %e, "Error processing message");
                                }
                            }
                            Err(e) => {
                                debug!(error = %e, "Error receiving message");
                            }
                        }
                    }
                }
            }
        });

        info!("Kafka consumer started");
        Ok(handle)
    }

    /// 停止消费者
    pub async fn stop(&self) {
        if self
            .running
            .compare_exchange(true, false, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        let _ = self.stop_tx.send(());
        info!("Kafka consumer stop signal sent");
    }

    /// 处理单条消息
    async fn process_message(
        consumer: &StreamConsumer,
        serializer: &EventSerializer,
        metrics: &KafkaMetrics,
        handler: &Option<EventHandlerFn>,
        message: &BorrowedMessage<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let topic = message.topic();
        let partition = message.partition();
        let offset = message.offset();

        // 提取消息头
        let headers = message.headers().map(|h| {
            let mut map = HashMap::new();
            for i in 0..h.count() {
                if let (Some(key), Some(value)) = (h.get(i).key, h.get(i).value) {
                    map.insert(key.to_string(), String::from_utf8_lossy(value).to_string());
                }
            }
            map
        });

        debug!(topic, partition, offset, "Received message");

        // 解析消息负载
        let payload = match message.payload() {
            Some(data) => data,
            None => {
                warn!(topic, partition, offset, "Empty message payload, committing offset");
                consumer.commit_message(message, rdkafka::consumer::CommitMode::Async)?;
                return Ok(());
            }
        };

        // 反序列化事件
        let envelope: SerializedEventEnvelope = match serializer.deserialize_envelope(payload, None) {
            Ok(e) => e,
            Err(e) => {
                error!(error = %e, "Failed to deserialize event, committing offset");
                consumer.commit_message(message, rdkafka::consumer::CommitMode::Async)?;
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }
        };

        // 调用处理函数
        if let Some(handler) = handler {
            if let Err(e) = handler(envelope.clone()).await {
                error!(event_id = %envelope.event_id, error = %e, "Handler failed");
                // 不提交偏移量，让消息重新消费
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }
        }

        // 提交偏移量
        consumer.commit_message(message, rdkafka::consumer::CommitMode::Async)?;

        let duration = start.elapsed();
        metrics.record_consume(topic, true, duration);

        debug!(event_id = %envelope.event_id, duration_ms = %duration.as_millis(), "Message processed");

        Ok(())
    }
}

// =============================================================================
// 便利构建器
// =============================================================================

/// 消费者构建器
pub struct KafkaConsumerBuilder {
    config: KafkaConsumerConfig,
    serializer: Option<Arc<EventSerializer>>,
    metrics: Option<Arc<KafkaMetrics>>,
    handler: Option<EventHandlerFn>,
}

impl Default for KafkaConsumerBuilder {
    fn default() -> Self {
        Self {
            config: KafkaConsumerConfig {
                brokers: Vec::new(),
                group_id: "killer-financial-service".to_string(),
                client_id: "killer-financial-service".to_string(),
                enable_auto_commit: false,
                auto_commit_interval_ms: 5000,
                session_timeout_ms: 30000,
                max_poll_records: 100,
                topics: Vec::new(),
                sasl: None,
                ssl: None,
            },
            serializer: None,
            metrics: None,
            handler: None,
        }
    }
}

impl KafkaConsumerBuilder {
    /// 设置 broker 地址
    pub fn brokers(mut self, brokers: Vec<String>) -> Self {
        self.config.brokers = brokers;
        self
    }

    /// 设置消费者组
    pub fn group_id(mut self, group_id: &str) -> Self {
        self.config.group_id = group_id.to_string();
        self
    }

    /// 设置订阅主题
    pub fn topics(mut self, topics: Vec<&str>) -> Self {
        self.config.topics = topics.iter().map(|s| s.to_string()).collect();
        self
    }

    /// 设置 SASL 认证
    pub fn sasl(mut self, mechanism: &str, username: &str, password: &str) -> Self {
        self.config.sasl = Some(KafkaSaslConfig {
            mechanism: mechanism.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        });
        self
    }

    /// 设置事件处理函数
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(SerializedEventEnvelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(handler));
        self
    }

    /// 构建消费者
    pub async fn build(self) -> Result<KafkaEventConsumer, Box<dyn std::error::Error>> {
        let serializer = self.serializer.unwrap_or_else(|| Arc::new(EventSerializer::default()));
        let metrics = self.metrics.unwrap_or_else(|| Arc::new(KafkaMetrics::new().unwrap_or_default()));

        let mut consumer = KafkaEventConsumer::new(&self.config, serializer, metrics).await?;

        if let Some(handler) = self.handler {
            consumer.set_handler(handler);
        }

        Ok(consumer)
    }
}
