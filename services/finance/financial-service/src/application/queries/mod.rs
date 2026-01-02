//! 查询处理器模块
//!
//! 实现 CQRS 读模型，处理所有业务查询
//! 查询直接访问读模型（ClickHouse/Redis），不经过聚合根

pub mod get_journal_entry;
pub mod list_journal_entries;
pub mod get_account_balance;
pub mod get_trial_balance;

use crate::application::dto::*;
use crate::application::error::ApplicationError;
use killer_cqrs::prelude::*;
use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::{debug, info, Span};
use uuid::Uuid;

// =============================================================================
// 共享类型
// =============================================================================

/// 查询处理上下文
pub struct QueryContext {
    /// 租户 ID
    pub tenant_id: Uuid,
    /// 关联 ID
    pub correlation_id: Uuid,
    /// 时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl QueryContext {
    pub fn new(tenant_id: Uuid) -> Self {
        Self {
            tenant_id,
            correlation_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 查询结果类型
pub type QueryResult<T> = Result<T, ApplicationError>;

/// 记录查询指标
pub fn record_query_metrics(query_name: &str, success: bool, duration: std::time::Duration) {
    let status = if success { "success" } else { "failure" };
    counter!("queries_total", "query" = query_name, "status" = status);
    histogram!("queries_duration", "query" = query_name);
}

/// 验证租户访问
pub fn validate_query_tenant_access(
    tenant_id: Uuid,
    entity_tenant_id: Uuid,
) -> Result<(), ApplicationError> {
    if tenant_id != entity_tenant_id {
        return Err(ApplicationError::forbidden(
            "Access denied to this resource".to_string(),
        ));
    }
    Ok(())
}

/// 验证查询参数
pub fn validate_pagination_params(
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<(u32, u32), ApplicationError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(20);

    if page < 1 {
        return Err(ApplicationError::validation_failed(
            "Page must be greater than 0".to_string(),
        ));
    }

    if page_size < 1 || page_size > 100 {
        return Err(ApplicationError::validation_failed(
            "Page size must be between 1 and 100".to_string(),
        ));
    }

    Ok((page, page_size))
}

/// 验证日期范围
pub fn validate_date_range(
    from: Option<chrono::NaiveDate>,
    to: Option<chrono::NaiveDate>,
) -> Result<(), ApplicationError> {
    if let (Some(from_date), Some(to_date)) = (from, to) {
        if from_date > to_date {
            return Err(ApplicationError::validation_failed(
                "Date range is invalid: from date must be before to date".to_string(),
            ));
        }
    }
    Ok(())
}

/// 验证金额范围
pub fn validate_amount_range(
    min: Option<f64>,
    max: Option<f64>,
) -> Result<(), ApplicationError> {
    if let (Some(min_amount), Some(max_amount)) = (min, max) {
        if min_amount < 0.0 || max_amount < 0.0 {
            return Err(ApplicationError::validation_failed(
                "Amounts must be non-negative".to_string(),
            ));
        }
        if min_amount > max_amount {
            return Err(ApplicationError::validation_failed(
                "Amount range is invalid: min must be less than max".to_string(),
            ));
        }
    }
    Ok(())
}

/// 验证会计年度
pub fn validate_fiscal_year(fiscal_year: i32) -> Result<(), ApplicationError> {
    if fiscal_year < 1970 || fiscal_year > 9999 {
        return Err(ApplicationError::validation_failed(format!(
            "Invalid fiscal year: {} (must be between 1970 and 9999)",
            fiscal_year
        )));
    }
    Ok(())
}
