//! Outbox 发布器 Worker
//!
//! 从发件箱表拉取待发布事件，序列化后发布到 Kafka
//! 支持指数退避重试、批量处理、幂等性保证

use crate::infrastructure::messaging::{EventSerializer, KafkaEventProducer, KafkaMetrics};
use killer_cqrs::event::{OutboxEvent, OutboxRepository};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn, Level, Span};
use uuid::Uuid;

// =============================================================================
// 配置
// =============================================================================

/// Outbox 发布器配置
#[derive(Debug, Clone)]
pub struct OutboxPublisherConfig {
    /// 轮询间隔（毫秒）
    pub poll_interval_ms: u64,
    /// 批量大小
    pub batch_size: usize,
    /// 最大重试次数
    pub max_retries: u32,
    /// 初始退避时间（毫秒）
    pub initial_backoff_ms: u64,
    /// 最大退避时间（秒）
    pub max_backoff_secs: u64,
    /// 退避乘数
    pub backoff_multiplier: f64,
    /// 清理已发布事件的间隔（秒）
    pub cleanup_interval_secs: u64,
    /// 清理多少天前的已发布事件
    pub cleanup_older_than_days: i64,
}

impl Default for OutboxPublisherConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 100,
            batch_size: 100,
            max_retries: 5,
            initial_backoff_ms: 100,
            max_backoff_secs: 300,
            backoff_multiplier: 2.0,
            cleanup_interval_secs: 3600,
            cleanup_older_than_days: 7,
        }
    }
}

// =============================================================================
// 发布器
// =============================================================================

/// Outbox 发布器 Worker
pub struct OutboxPublisherWorker<R, P>
where
    R: OutboxRepository,
    P: KafkaEventProducer,
{
    /// Outbox 仓储
    outbox_repo: Arc<R>,
    /// Kafka 生产者
    producer: Arc<P>,
    /// 序列化器
    serializer: Arc<EventSerializer>,
    /// 指标
    metrics: Arc<KafkaMetrics>,
    /// 配置
    config: OutboxPublisherConfig,
    /// 停止信号
    stop_tx: broadcast::Sender<()>,
    /// 运行时状态
    running: Arc<tokio::sync::atomic::AtomicBool>,
}

impl<R, P> OutboxPublisherWorker<R, P>
where
    R: OutboxRepository + 'static,
    P: KafkaEventProducer + 'static,
{
    /// 创建新的发布器
    pub fn new(
        outbox_repo: Arc<R>,
        producer: Arc<P>,
        serializer: Arc<EventSerializer>,
        metrics: Arc<KafkaMetrics>,
        config: Option<OutboxPublisherConfig>,
    ) -> Self {
        Self {
            outbox_repo,
            producer,
            serializer,
            metrics,
            config: config.unwrap_or_default(),
            stop_tx: broadcast::channel(1).0,
            running: Arc::new(tokio::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// 启动发布器
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self
            .running
            .compare_exchange(false, true, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst)
            .is_err()
        {
            return Err("Outbox publisher is already running".into());
        }

        info!("Starting outbox publisher worker");

        // 启动发布循环
        let publisher = self.clone();
        tokio::spawn(async move {
            publisher.publish_loop().await;
        });

        // 启动清理循环
        let publisher = self.clone();
        tokio::spawn(async move {
            publisher.cleanup_loop().await;
        });

        info!("Outbox publisher worker started");
        Ok(())
    }

    /// 停止发布器
    pub async fn stop(&self) {
        if self
            .running
            .compare_exchange(true, false, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        info!("Stopping outbox publisher worker");
        let _ = self.stop_tx.send(());
        info!("Outbox publisher worker stopped");
    }

    /// 发布循环
    async fn publish_loop(&self) {
        let mut interval = interval(Duration::from_millis(self.config.poll_interval_ms));
        let mut backoff = Duration::from_millis(self.config.initial_backoff_ms);

        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            interval.tick().await;

            match self.process_batch().await {
                Ok(processed) => {
                    if processed > 0 {
                        // 有处理成功，重置退避
                        backoff = Duration::from_millis(self.config.initial_backoff_ms);
                        debug!(count = processed, "Processed batch");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Error processing batch, will retry");
                    sleep(backoff).await;
                    backoff = (backoff * self.config.backoff_multiplier as u32)
                        .min(Duration::from_secs(self.config.max_backoff_secs));
                }
            }
        }
    }

    /// 清理循环
    async fn cleanup_loop(&self) {
        let mut interval = interval(Duration::from_secs(self.config.cleanup_interval_secs));

        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            interval.tick().await;

            if let Err(e) = self.cleanup_old_events().await {
                error!(error = %e, "Error cleaning up old events");
            }
        }
    }

    /// 处理一批事件
    async fn process_batch(&self) -> Result<usize, Box<dyn std::error::Error>> {
        // 1. 从 Outbox 获取未发布事件
        let events = self
            .outbox_repo
            .get_unpublished(self.config.batch_size)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        if events.is_empty() {
            return Ok(0);
        }

        let mut success_count = 0;
        let mut failed_events = Vec::new();

        // 2. 处理每个事件
        for event in events {
            match self.publish_event(&event).await {
                Ok(_) => {
                    // 标记为已发布
                    if let Err(e) = self.outbox_repo.mark_published(event.event_id).await {
                        error!(event_id = %event.event_id, error = %e, "Failed to mark event as published");
                    }
                    success_count += 1;
                }
                Err(e) => {
                    error!(event_id = %event.event_id, error = %e, "Failed to publish event");

                    // 检查是否超过最大重试次数
                    if event.retry_count >= self.config.max_retries as i32 {
                        // 标记为失败
                        if let Err(mark_err) = self
                            .outbox_repo
                            .mark_failed(event.event_id, format!("Max retries exceeded: {:?}", e))
                            .await
                        {
                            error!(event_id = %event.event_id, error = %mark_err, "Failed to mark event as failed");
                        }
                    } else {
                        // 保留待重试
                        failed_events.push(event);
                    }
                }
            }
        }

        // 3. 如果有失败事件，记录警告
        if !failed_events.is_empty() {
            warn!(
                failed = failed_events.len(),
                "Some events failed to publish"
            );
        }

        Ok(success_count)
    }

    /// 发布单个事件
    async fn publish_event(&self, event: &OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let span = span!(Level::DEBUG, "PublishEvent", event_id = %event.event_id);
        let _guard = span.enter();

        // 反序列化事件负载
        let payload: serde_json::Value = serde_json::from_str(&event.payload)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 构建事件信封
        let envelope = SerializedEventEnvelope {
            event_id: event.event_id,
            event_type: event.event_name.clone(),
            schema_version: 1,
            aggregate_type: event.aggregate_type.clone(),
            aggregate_id: event.aggregate_id.clone(),
            aggregate_version: 0,
            tenant_id: event.tenant_id.clone(),
            occurred_at: event.occurred_at,
            payload,
            metadata: HashMap::new(),
        };

        // 发布到 Kafka
        self.producer.publish_envelope(&envelope).await?;

        let duration = start.elapsed();
        self.metrics.record_publish(&self.producer.topic, true, duration);

        info!(event_id = %event.event_id, duration_ms = %duration.as_millis(), "Event published");

        Ok(())
    }

    /// 清理旧的已发布事件
    async fn cleanup_old_events(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(self.config.cleanup_older_than_days);

        let count = self
            .outbox_repo
            .delete_published_before(cutoff)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        if count > 0 {
            info!(deleted_count = count, "Cleaned up old outbox events");
        }

        Ok(count)
    }

    fn clone(&self) -> Self {
        Self {
            outbox_repo: self.outbox_repo.clone(),
            producer: self.producer.clone(),
            serializer: self.serializer.clone(),
            metrics: self.metrics.clone(),
            config: self.config.clone(),
            stop_tx: self.stop_tx.clone(),
            running: self.running.clone(),
        }
    }
}

// =============================================================================
// 指标记录
// =============================================================================

struct PublisherMetrics {
    events_processed_total: prometheus::IntCounterVec,
    events_published_total: prometheus::IntCounterVec,
    events_failed_total: prometheus::IntCounterVec,
    publish_duration: prometheus::HistogramVec,
    batch_size: prometheus::HistogramVec,
    retry_count: prometheus::HistogramVec,
}

impl PublisherMetrics {
    fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            events_processed_total: prometheus::register_int_counter_vec!(
                "outbox_events_processed_total",
                "Total outbox events processed",
                &["status"]
            )?,
            events_published_total: prometheus::register_int_counter_vec!(
                "outbox_events_published_total",
                "Total outbox events published to Kafka",
                &["topic"]
            )?,
            events_failed_total: prometheus::register_int_counter_vec!(
                "outbox_events_failed_total",
                "Total outbox events failed",
                &["reason"]
            )?,
            publish_duration: prometheus::register_histogram_vec!(
                "outbox_publish_duration_seconds",
                "Outbox publish duration in seconds",
                &["topic"]
            )?,
            batch_size: prometheus::register_histogram_vec!(
                "outbox_batch_size",
                "Outbox batch size",
                &[]
            )?,
            retry_count: prometheus::register_histogram_vec!(
                "outbox_retry_count",
                "Outbox event retry count",
                &[]
            )?,
        })
    }
}
