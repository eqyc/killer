//! 持久化模块
//!
//! 使用 sqlx 实现领域仓储接口
//! 提供 PostgreSQL 持久化支持，包括：
//! - JournalEntryRepository
//! - FiscalPeriodRepository
//! - OutboxRepository
//!
//! ## 表设计
//!
//! - `journal_entries`: 会计凭证抬头
//! - `journal_entry_lines`: 会计凭证行项目
//! - `fiscal_periods`: 会计期间
//! - `outbox_messages`: 事件发件箱
//! - `audit_log`: 审计日志（软删除）

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use killer_cqrs::event::OutboxEvent;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// Re-exports
pub mod journal_entry_repository;
pub mod fiscal_period_repository;
pub mod outbox_repository;
pub mod models;

pub use journal_entry_repository::PgJournalEntryRepository;
pub use fiscal_period_repository::PgFiscalPeriodRepository;
pub use outbox_repository::PgOutboxRepository;

// =============================================================================
// 共享类型
// =============================================================================

/// 凭证行项目数据库模型
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct JournalEntryLineDb {
    pub line_number: i32,
    pub account_code: String,
    pub amount: sqlx::types::Decimal,
    pub debit_credit: String,
    pub cost_center: Option<String>,
    pub profit_center: Option<String>,
    pub text: Option<String>,
    pub functional_area: Option<String>,
    pub business_area: Option<String>,
    pub order_number: Option<String>,
}

/// 凭证抬头数据库模型
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct JournalEntryDb {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub company_code: String,
    pub fiscal_year: i32,
    pub document_number: String,
    pub posting_date: NaiveDate,
    pub document_date: NaiveDate,
    pub currency_code: String,
    pub status: String,
    pub header_text: Option<String>,
    pub reference_document: Option<String>,
    pub total_debit: sqlx::types::Decimal,
    pub total_credit: sqlx::types::Decimal,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub posted_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 会计期间数据库模型
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct FiscalPeriodDb {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub company_code: String,
    pub fiscal_year: i32,
    pub period: i32,
    pub status: String,
    pub valid_from: NaiveDate,
    pub valid_to: NaiveDate,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 发件箱消息数据库模型
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct OutboxMessageDb {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: sqlx::types::Json<serde_json::Value>,
    pub schema_version: i32,
    pub occurred_at: DateTime<Utc>,
    pub status: String, // Pending, Sent, Failed
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

/// 审计日志数据库模型
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuditLogDb {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub table_name: String,
    pub record_id: Uuid,
    pub action: String,
    pub old_value: Option<sqlx::types::Json<serde_json::Value>>,
    pub new_value: Option<sqlx::types::Json<serde_json::Value>>,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// 共享工具函数
// =============================================================================

/// 数据库操作指标
pub struct DbMetrics {
    queries_total: prometheus::IntCounterVec,
    query_duration: prometheus::HistogramVec,
    connection_pool_size: prometheus::GaugeVec,
    connection_pool_idle: prometheus::GaugeVec,
}

impl DbMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            queries_total: prometheus::register_int_counter_vec!(
                "db_queries_total",
                "Total database queries",
                &["operation", "table", "status"]
            )?,
            query_duration: prometheus::register_histogram_vec!(
                "db_query_duration_seconds",
                "Database query duration in seconds",
                &["operation", "table"]
            )?,
            connection_pool_size: prometheus::register_gauge_vec!(
                "db_connection_pool_size",
                "Database connection pool size",
                &["pool"]
            )?,
            connection_pool_idle: prometheus::register_gauge_vec!(
                "db_connection_pool_idle",
                "Database connection pool idle connections",
                &["pool"]
            )?,
        })
    }

    pub fn record_query(&self, operation: &str, table: &str, success: bool, duration: std::time::Duration) {
        let status = if success { "success" } else { "failure" };
        self.queries_total.with_label_values(&[operation, table, status]).inc();
        self.query_duration
            .with_label_values(&[operation, table])
            .observe(duration.as_secs_f64());
    }
}

impl Default for DbMetrics {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// 带指标的数据库执行
async fn execute_with_metrics<F, T>(
    pool: &PgPool,
    metrics: &DbMetrics,
    operation: &str,
    table: &str,
    query: F,
) -> Result<T, sqlx::Error>
where
    F: FnOnce(&PgPool) -> sqlx::Result<T>,
{
    let start = std::time::Instant::now();
    let result = query(pool).await;
    let duration = start.elapsed();

    let success = result.is_ok();
    metrics.record_query(operation, table, success, duration);

    if let Err(ref e) = result {
        error!(%operation, %table, error = %e, "Database query failed");
    } else {
        debug!(%operation, %table, duration_ms = %duration.as_millis(), "Database query completed");
    }

    result
}

/// 软删除记录
pub async fn soft_delete(
    pool: &PgPool,
    table: &str,
    id: Uuid,
    tenant_id: Uuid,
) -> Result<(), sqlx::Error> {
    let query = format!(
        "UPDATE {} SET deleted_at = NOW() WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
        table
    );
    sqlx::query(&query).bind(id).bind(tenant_id).execute(pool).await?;
    Ok(())
}

/// 乐观锁更新
pub async fn update_with_version<T>(
    pool: &PgPool,
    table: &str,
    id: Uuid,
    tenant_id: Uuid,
    expected_version: i32,
    update_fields: &[(&str, sqlx::types::JsonValue)],
) -> Result<(), sqlx::Error> {
    let mut set_clauses = Vec::new();
    let mut params: Vec<sqlx::postgres::PgValue> = Vec::new();
    let mut param_idx = 1;

    for (field, value) in update_fields {
        set_clauses.push(format!("{} = ${}", field, param_idx));
        params.push(sqlx::postgres::PgValue::from(serde_json::to_string(value).unwrap()));
        param_idx += 1;
    }

    set_clauses.push(format!("version = version + 1"));
    set_clauses.push(format!("updated_at = NOW()"));
    params.push(sqlx::postgres::PgValue::from(id));
    param_idx += 1;
    params.push(sqlx::postgres::PgValue::from(tenant_id));
    param_idx += 1;
    params.push(sqlx::postgres::PgValue::from(expected_version));

    let query = format!(
        "UPDATE {} SET {} WHERE id = ${} AND tenant_id = ${} AND version = ${}",
        table,
        set_clauses.join(", "),
        param_idx - 3,
        param_idx - 2,
        param_idx - 1
    );

    let result = sqlx::query(&query)
        .bind_all(params)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
