//! 事件处理器模块
//!
//! 实现领域事件消费者，更新读模型和处理跨服务集成
//! 支持重试机制和补偿事务

pub mod journal_entry_posted;
pub mod journal_entry_reversed;
pub mod fiscal_period_closed;
pub mod material_document_posted;

use crate::application::error::ApplicationError;
use killer_cqrs::prelude::*;
use metrics::{counter, histogram};
use std::time::Duration;
use tracing::{debug, error, info, warn};

// =============================================================================
// 共享类型
// =============================================================================

/// 事件处理结果
pub type EventResult = Result<(), ApplicationError>;

/// 事件处理统计
#[derive(Debug, Default)]
pub struct EventHandlerStats {
    pub processed: u64,
    pub failed: u64,
    pub retries: u64,
}

impl EventHandlerStats {
    pub fn increment_processed(&mut self) {
        self.processed += 1;
    }

    pub fn increment_failed(&mut self) {
        self.failed += 1;
    }

    pub fn increment_retries(&mut self) {
        self.retries += 1;
    }
}

/// 记录事件处理指标
pub fn record_event_metrics(event_name: &str, handler: &str, success: bool, duration: Duration) {
    let status = if success { "success" } else { "failure" };
    counter!("events_processed_total", "event" = event_name, "handler" = handler, "status" = status);
    histogram!("events_duration", "event" = event_name, "handler" = handler);
}

/// 重试配置
#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

/// 带重试的事件处理
pub async fn with_retry<T, F, Fut>(
    operation: F,
    config: RetryConfig,
) -> Result<T, ApplicationError>
where
    F: Fn(u32) -> Fut,
    Fut: std::future::Future<Output = Result<T, ApplicationError>>,
{
    let mut last_error = None;
    let mut delay = config.initial_delay;

    for attempt in 0..=config.max_retries {
        match operation(attempt).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < config.max_retries {
                    warn!("Event processing failed, attempt {}/{}, retrying in {:?}",
                        attempt + 1, config.max_retries + 1, delay);

                    tokio::time::sleep(delay).await;
                    delay = (delay * config.backoff_multiplier).min(config.max_delay);
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| ApplicationError::infrastructure_error(
        "Event processing failed after all retries".to_string(),
    )))
}
