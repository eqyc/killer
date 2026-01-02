//! 命令处理器模块
//!
//! 实现 CQRS 写模型，处理所有业务命令
//! 每个命令对应一个聚合根操作

pub mod create_journal_entry;
pub mod post_journal_entry;
pub mod reverse_journal_entry;
pub mod close_fiscal_period;

use crate::application::dto::*;
use crate::application::error::ApplicationError;
use crate::domain::repositories::*;
use crate::domain::services::*;
use crate::domain::*;
use killer_cqrs::{Command, CommandHandler, Result as CqrsResult};
use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::{debug, error, info, warn, Instrument};
use uuid::Uuid;

// =============================================================================
// 共享类型和实用函数
// =============================================================================

/// 命令处理上下文
pub struct CommandContext {
    /// 租户 ID
    pub tenant_id: Uuid,
    /// 用户 ID
    pub user_id: Uuid,
    /// 关联 ID（用于链路追踪）
    pub correlation_id: Uuid,
    /// 时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl CommandContext {
    pub fn new(tenant_id: Uuid, user_id: Uuid) -> Self {
        Self {
            tenant_id,
            user_id,
            correlation_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 命令处理结果
pub type CommandResult<T> = Result<T, ApplicationError>;

/// 记录命令指标
pub fn record_command_metrics(command_name: &str, success: bool, duration: std::time::Duration) {
    let status = if success { "success" } else { "failure" };
    counter!("commands_total", "command" = command_name, "status" = status);
    histogram!("commands_duration", "command" = command_name);
}

/// 验证租户访问权限
pub fn validate_tenant_access(
    tenant_id: Uuid,
    entity_tenant_id: Uuid,
) -> Result<(), ApplicationError> {
    if tenant_id != entity_tenant_id {
        warn!(
            "Tenant access violation: {} attempted to access resource of tenant {}",
            tenant_id, entity_tenant_id
        );
        return Err(ApplicationError::forbidden(
            "Access denied to this resource".to_string(),
        ));
    }
    Ok(())
}

/// 验证公司代码是否属于租户
pub async fn validate_company_code<B: CompanyCodeRepository>(
    _repo: &Arc<B>,
    _tenant_id: Uuid,
    _company_code: &str,
) -> Result<(), ApplicationError> {
    // TODO: Implement actual validation against master data
    // For now, we just check that the company code is not empty
    if _company_code.is_empty() {
        return Err(ApplicationError::validation_failed(
            "Company code cannot be empty".to_string(),
        ));
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

/// 验证凭证号格式
pub fn validate_document_number(document_number: &str) -> Result<(), ApplicationError> {
    if document_number.is_empty() {
        return Err(ApplicationError::validation_failed(
            "Document number cannot be empty".to_string(),
        ));
    }
    if document_number.len() > 16 {
        return Err(ApplicationError::validation_failed(
            "Document number too long (max 16 characters)".to_string(),
        ));
    }
    Ok(())
}

/// 从 DTO 创建行项目领域对象
pub fn line_items_from_dto(
    line_items: Vec<JournalEntryLineItemRequest>,
) -> Result<Vec<JournalEntryLineItem>, ApplicationError> {
    line_items
        .into_iter()
        .map(|item| {
            JournalEntryLineItem::new(
                item.line_number,
                item.account_code,
                Money::from_str(&item.amount.to_string(), "USD")
                    .map_err(|_| ApplicationError::validation_failed(format!(
                        "Invalid amount: {}",
                        item.amount
                    )))?,
                DebitCredit::from_str(&item.debit_credit).map_err(|_| {
                    ApplicationError::validation_failed(format!(
                        "Invalid debit_credit indicator: {} (must be 'D' or 'C')",
                        item.debit_credit
                    ))
                })?,
                item.cost_center,
                item.profit_center,
                item.text,
                item.functional_area,
                item.business_area,
                item.order_number,
            )
        })
        .collect()
}

/// 从 DTO 创建凭证聚合根
pub fn journal_entry_from_dto(
    request: CreateJournalEntryRequest,
    tenant_id: Uuid,
    document_number: String,
) -> Result<JournalEntry, ApplicationError> {
    let line_items = line_items_from_dto(request.line_items)?;

    JournalEntry::create(
        tenant_id,
        document_number,
        request.company_code,
        request.fiscal_year,
        request.posting_date.ok_or_else(|| ApplicationError::validation_failed(
            "Posting date is required".to_string(),
        ))?,
        request.document_date.ok_or_else(|| ApplicationError::validation_failed(
            "Document date is required".to_string(),
        ))?,
        request.currency_code,
        request.header_text,
        request.reference_document,
        line_items,
    )
}
