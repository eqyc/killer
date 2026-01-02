//! 发件箱仓储 PostgreSQL 实现
//!
//! 实现 cqrs::OutboxRepository 接口
//! 使用 Transactional Outbox 模式确保事件持久化

use crate::infrastructure::persistence::{execute_with_metrics, DbMetrics, OutboxMessageDb};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use killer_cqrs::event::{OutboxEvent, OutboxRepository};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use std::sync::Arc;
use tracing::{debug, error, info, span, Level};
use uuid::Uuid;

// =============================================================================
// PostgreSQL Outbox 仓储实现
// =============================================================================

/// PostgreSQL 发件箱仓储
#[derive(Clone)]
pub struct PgOutboxRepository {
    /// 数据库连接池
    pool: Arc<PgPool>,
    /// 指标
    metrics: Arc<DbMetrics>,
    /// 表名
    table_name: &'static str,
}

impl PgOutboxRepository {
    /// 创建新的仓储实例
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            metrics: Arc::new(DbMetrics::new().unwrap_or_default()),
            table_name: "outbox_messages",
        }
    }
}

#[async_trait]
impl OutboxRepository for PgOutboxRepository {
    /// 保存事件到 Outbox
    async fn save(&self, event: OutboxEvent) -> Result<(), killer_cqrs::error::AppError> {
        let span = span!(Level::DEBUG, "OutboxRepository.save", event_id = %event.event_id);
        let _guard = span.enter();

        let query = format!(
            "INSERT INTO {} (id, tenant_id, aggregate_type, aggregate_id, event_type, payload, schema_version, occurred_at, status, attempts, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'Pending', 0, NOW())",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "save",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(event.event_id)
                    .bind(Uuid::parse_str(&event.tenant_id).unwrap_or(Uuid::nil()))
                    .bind(&event.aggregate_type)
                    .bind(&event.aggregate_id)
                    .bind(&event.event_name)
                    .bind(&event.payload)
                    .bind(1i32) // schema_version
                    .bind(event.occurred_at)
                    .execute(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("save", self.table_name, result.is_ok(), duration);

        result.map_err(|e| killer_cqrs::error::AppError::Internal(format!("Failed to save outbox event: {:?}", e)))?;

        debug!(event_id = %event.event_id, "Event saved to outbox");
        Ok(())
    }

    /// 批量保存事件
    async fn save_batch(&self, events: Vec<OutboxEvent>) -> Result<(), killer_cqrs::error::AppError> {
        if events.is_empty() {
            return Ok(());
        }

        let span = span!(Level::DEBUG, "OutboxRepository.save_batch", count = events.len());
        let _guard = span.enter();

        let mut query = format!(
            "INSERT INTO {} (id, tenant_id, aggregate_type, aggregate_id, event_type, payload, schema_version, occurred_at, status, attempts, created_at) VALUES ",
            self.table_name
        );

        let mut values = Vec::new();
        let mut params: Vec<sqlx::postgres::PgValue> = Vec::new();
        let mut param_idx = 1;

        for (i, event) in events.iter().enumerate() {
            let base = param_idx;
            values.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, 'Pending', 0, NOW())",
                base,
                base + 1,
                base + 2,
                base + 3,
                base + 4,
                base + 5,
                base + 6,
                base + 7
            ));

            params.push(sqlx::postgres::PgValue::from(event.event_id));
            params.push(sqlx::postgres::PgValue::from(
                Uuid::parse_str(&event.tenant_id).unwrap_or(Uuid::nil()),
            ));
            params.push(sqlx::postgres::PgValue::from(event.aggregate_type.clone()));
            params.push(sqlx::postgres::PgValue::from(event.aggregate_id.clone()));
            params.push(sqlx::postgres::PgValue::from(event.event_name.clone()));
            params.push(sqlx::postgres::PgValue::from(event.payload.clone()));
            params.push(sqlx::postgres::PgValue::from(1i32));
            params.push(sqlx::postgres::PgValue::from(event.occurred_at));

            param_idx += 8;
        }

        query.push_str(&values.join(", "));
        query.push_str(" ON CONFLICT (id) DO NOTHING");

        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }

        let start = std::time::Instant::now();

        let result = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "save_batch",
            self.table_name,
            |pool| async move {
                q.execute(pool).await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("save_batch", self.table_name, result.is_ok(), duration);

        match result {
            Ok(result) => {
                debug!(event_count = events.len(), rows_affected = result.rows_affected(), "Batch events saved to outbox");
                Ok(())
            }
            Err(e) => Err(killer_cqrs::error::AppError::Internal(format!(
                "Failed to save batch outbox events: {:?}",
                e
            ))),
        }
    }

    /// 获取未发布的事件
    async fn get_unpublished(&self, limit: usize) -> Result<Vec<OutboxEvent>, killer_cqrs::error::AppError> {
        let span = span!(Level::DEBUG, "OutboxRepository.get_unpublished", %limit);
        let _guard = span.enter();

        let query = format!(
            "SELECT id, tenant_id, aggregate_type, aggregate_id, event_type, payload, occurred_at, attempts, last_error
             FROM {} WHERE status = 'Pending' ORDER BY occurred_at ASC LIMIT $1",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<OutboxMessageDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "get_unpublished",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(limit as i32)
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("get_unpublished", self.table_name, result.is_ok(), duration);

        match result {
            Ok(messages) => {
                let events: Result<Vec<OutboxEvent>, _> = messages
                    .iter()
                    .map(|msg| -> Result<OutboxEvent, killer_cqrs::error::AppError> {
                        Ok(OutboxEvent {
                            event_id: msg.id,
                            event_name: msg.event_type.clone(),
                            aggregate_id: msg.aggregate_id.clone(),
                            aggregate_type: msg.aggregate_type.clone(),
                            tenant_id: msg.tenant_id.to_string(),
                            payload: serde_json::to_string(&msg.payload.0)?,
                            occurred_at: msg.occurred_at,
                            published: false,
                            published_at: None,
                            retry_count: msg.attempts,
                            last_error: msg.last_error.clone(),
                        })
                    })
                    .collect();

                events.map_err(|e| killer_cqrs::error::AppError::Internal(format!("Failed to serialize events: {:?}", e)))
            }
            Err(e) => Err(killer_cqrs::error::AppError::Internal(format!(
                "Failed to get unpublished events: {:?}",
                e
            ))),
        }
    }

    /// 标记事件为已发布
    async fn mark_published(&self, event_id: Uuid) -> Result<(), killer_cqrs::error::AppError> {
        let span = span!(Level::DEBUG, "OutboxRepository.mark_published", %event_id);
        let _guard = span.enter();

        let query = format!(
            "UPDATE {} SET status = 'Sent', sent_at = NOW() WHERE id = $1 AND status != 'Sent'",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "mark_published",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(event_id)
                    .execute(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("mark_published", self.table_name, result.is_ok(), duration);

        match result {
            Ok(result) => {
                debug!(%event_id, rows_affected = result.rows_affected(), "Event marked as published");
                Ok(())
            }
            Err(e) => Err(killer_cqrs::error::AppError::Internal(format!(
                "Failed to mark event as published: {:?}",
                e
            ))),
        }
    }

    /// 标记事件为失败
    async fn mark_failed(&self, event_id: Uuid, error: String) -> Result<(), killer_cqrs::error::AppError> {
        let span = span!(Level::DEBUG, "OutboxRepository.mark_failed", %event_id);
        let _guard = span.enter();

        let query = format!(
            "UPDATE {} SET status = 'Failed', attempts = attempts + 1, last_error = $1 WHERE id = $2",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "mark_failed",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(error)
                    .bind(event_id)
                    .execute(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("mark_failed", self.table_name, result.is_ok(), duration);

        match result {
            Ok(_) => {
                warn!(%event_id, "Event marked as failed");
                Ok(())
            }
            Err(e) => Err(killer_cqrs::error::AppError::Internal(format!(
                "Failed to mark event as failed: {:?}",
                e
            ))),
        }
    }

    /// 删除已发布的旧事件
    async fn delete_published_before(&self, before: DateTime<Utc>) -> Result<u64, killer_cqrs::error::AppError> {
        let span = span!(Level::DEBUG, "OutboxRepository.delete_published_before");
        let _guard = span.enter();

        let query = format!(
            "DELETE FROM {} WHERE status = 'Sent' AND sent_at < $1",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "delete_published_before",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(before)
                    .execute(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("delete_published_before", self.table_name, result.is_ok(), duration);

        match result {
            Ok(result) => {
                let count = result.rows_affected();
                info!(deleted_count = count, "Deleted published outbox messages");
                Ok(count)
            }
            Err(e) => Err(killer_cqrs::error::AppError::Internal(format!(
                "Failed to delete published messages: {:?}",
                e
            ))),
        }
    }
}
