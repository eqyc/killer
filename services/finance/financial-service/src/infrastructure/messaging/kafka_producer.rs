//! Kafka 事件生产者
//!
//! 将事件发布到 Kafka 主题
//! 支持分区键、压缩、重试

use crate::infrastructure::messaging::{EventSerializer, KafkaMetrics, SerializedEventEnvelope};
use async_trait::async_trait;
use chrono::Utc;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, span, Level, warn};

// =============================================================================
// Kafka 生产者配置
// =============================================================================

/// Kafka 生产者配置
#[derive(Debug, Clone)]
pub struct KafkaProducerConfig {
    /// Broker 地址列表
    pub brokers: Vec<String>,
    /// 客户端 ID
    pub client_id: String,
    /// ACKS 模式
    pub acks: String,
    /// 重试次数
    pub retries: i32,
    /// 批量大小（字节）
    pub batch_size: usize,
    /// Linger ms
    pub linger_ms: u64,
    /// 压缩类型
    pub compression: String,
    /// 请求超时
    pub request_timeout_ms: u32,
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
// Kafka 事件生产者
// =============================================================================

/// Kafka 事件生产者
#[derive(Clone)]
pub struct KafkaEventProducer {
    /// Kafka 生产者
    producer: FutureProducer,
    /// 序列化器
    serializer: Arc<EventSerializer>,
    /// 指标
    metrics: Arc<KafkaMetrics>,
    /// 主题名称
    topic: String,
    /// 默认分区数
    default_partitions: i32,
}

impl KafkaEventProducer {
    /// 创建新的生产者
    pub async fn new(
        config: &KafkaProducerConfig,
        topic: &str,
        serializer: Arc<EventSerializer>,
        metrics: Arc<KafkaMetrics>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_config = ClientConfig::new();

        // 基础配置
        client_config
            .set("bootstrap.servers", &config.brokers.join(","))
            .set("client.id", &config.client_id)
            .set("acks", &config.acks)
            .set("retries", config.retries.to_string())
            .set("batch.size", config.batch_size.to_string())
            .set("linger.ms", config.linger_ms.to_string())
            .set("compression.type", &config.compression)
            .set("request.timeout.ms", config.request_timeout_ms.to_string())
            .set("message.timeout.ms", "300000") // 5分钟超时
            .set("enable.idempotence", "true")
            .set("max.in.flight.requests.per.connection", "5");

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

        let producer = client_config
            .create()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        info!(topic, "Kafka producer created");

        Ok(Self {
            producer,
            serializer,
            metrics,
            topic: topic.to_string(),
            default_partitions: 3, // 默认分区数
        })
    }

    /// 发布事件信封
    pub async fn publish_envelope(
        &self,
        envelope: &SerializedEventEnvelope,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let span = span!(
            Level::DEBUG,
            "KafkaProducer.publish",
            event_id = %envelope.event_id,
            topic = %self.topic
        );
        let _guard = span.enter();

        let start = std::time::Instant::now();

        // 序列化事件
        let payload = self
            .serializer
            .serialize_envelope(envelope)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 获取分区键
        let partition_key = envelope.partition_key();

        // 计算分区
        let partition = self
            .get_partition(&partition_key)
            .unwrap_or(0);

        // 构建 Kafka 消息
        let record = FutureRecord::to(&self.topic)
            .key(partition_key.as_bytes())
            .partition(partition)
            .payload(&payload)
            .headers(OwnedHeaders::new()
                .insert(Header {
                    key: "event_type",
                    value: Some(envelope.event_type.as_bytes()),
                })
                .insert(Header {
                    key: "tenant_id",
                    value: Some(envelope.tenant_id.as_bytes()),
                })
                .insert(Header {
                    key: "schema_version",
                    value: Some(envelope.schema_version.to_string().as_bytes()),
                })
                .insert(Header {
                    key: "occurred_at",
                    value: Some(envelope.occurred_at.to_rfc3339().as_bytes()),
                })
            );

        // 发送消息
        let delivery_future = self.producer.send(record, Duration::from_secs(10));

        match delivery_future.await {
            Ok((partition, offset)) => {
                let duration = start.elapsed();
                self.metrics.record_publish(&self.topic, true, duration);

                debug!(
                    %partition,
                    offset = %offset,
                    duration_ms = %duration.as_millis(),
                    "Event published to Kafka"
                );
                Ok(())
            }
            Err((e, _)) => {
                let duration = start.elapsed();
                self.metrics.record_publish(&self.topic, false, duration);

                error!(error = %e, "Failed to publish event to Kafka");
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    /// 批量发布事件
    pub async fn publish_batch(
        &self,
        envelopes: &[SerializedEventEnvelope],
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut success_count = 0;
        let mut errors = Vec::new();

        for envelope in envelopes {
            if let Err(e) = self.publish_envelope(envelope).await {
                errors.push(e);
            } else {
                success_count += 1;
            }
        }

        if !errors.is_empty() {
            warn!(
                success = success_count,
                failed = errors.len(),
                "Batch publish completed with errors"
            );
        }

        Ok(success_count)
    }

    /// 根据分区键计算分区
    fn get_partition(&self, key: &str) -> Option<i32> {
        // 使用一致性哈希将分区键映射到分区
        let hash = xxhash_rust::xxh3::xxh3_64(key.as_bytes());
        let partition = hash % self.default_partitions as u64;
        Some(partition as i32)
    }
}

/// 异步发布 trait
#[async_trait]
pub trait AsyncEventPublisher: Send + Sync {
    /// 异步发布事件
    async fn publish(&self, envelope: SerializedEventEnvelope) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl AsyncEventPublisher for Arc<KafkaEventProducer> {
    async fn publish(&self, envelope: SerializedEventEnvelope) -> Result<(), Box<dyn std::error::Error>> {
        self.publish_envelope(&envelope).await
    }
}

// =============================================================================
// 便利构建器
// =============================================================================

/// 生产者构建器
pub struct KafkaProducerBuilder {
    config: KafkaProducerConfig,
    topic: String,
    serializer: Option<Arc<EventSerializer>>,
    metrics: Option<Arc<KafkaMetrics>>,
}

impl Default for KafkaProducerBuilder {
    fn default() -> Self {
        Self {
            config: KafkaProducerConfig {
                brokers: Vec::new(),
                client_id: "killer-financial-service".to_string(),
                acks: "all".to_string(),
                retries: 3,
                batch_size: 16384,
                linger_ms: 5,
                compression: "lz4".to_string(),
                request_timeout_ms: 30000,
                sasl: None,
                ssl: None,
            },
            topic: "killer.finance.events".to_string(),
            serializer: None,
            metrics: None,
        }
    }
}

impl KafkaProducerBuilder {
    /// 设置 broker 地址
    pub fn brokers(mut self, brokers: Vec<String>) -> Self {
        self.config.brokers = brokers;
        self
    }

    /// 设置主题
    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = topic.to_string();
        self
    }

    /// 设置客户端 ID
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.config.client_id = client_id.to_string();
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

    /// 构建生产者
    pub async fn build(self) -> Result<KafkaEventProducer, Box<dyn std::error::Error>> {
        let serializer = self.serializer.unwrap_or_else(|| Arc::new(EventSerializer::default()));
        let metrics = self.metrics.unwrap_or_else(|| Arc::new(KafkaMetrics::new().unwrap_or_default()));

        KafkaEventProducer::new(&self.config, &self.topic, serializer, metrics).await
    }
}
