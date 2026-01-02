//! DTO 与领域对象映射器
//!
//! 负责 DTO 与领域对象之间的转换
//! 遵循单向数据流原则：外部输入 → DTO → 领域对象 → 事件/输出 DTO

use serde::{Deserialize, Serialize};
use crate::domain::{
    DomainError, DomainEvent, FiscalPeriod, JournalEntry, JournalEntryLineItem,
};
use crate::application::dto::*;

// =============================================================================
// 错误映射
// =============================================================================

/// 将领域错误映射为应用错误
pub fn map_domain_error(error: DomainError) -> super::error::ApplicationError {
    use DomainError::*;
    match error {
        UnbalancedEntry { .. } => super::error::ApplicationError::business_rule_violation(
            "UNBALANCED_ENTRY", &format!("{:?}", error)
        ),
        PeriodClosed { .. } => super::error::ApplicationError::not_found("Period", format!("{:?}", error)),
        InsufficientLineItems { .. } => super::error::ApplicationError::validation_failed(format!("{:?}", error)),
        InvalidPostingDate { .. } => super::error::ApplicationError::validation_failed(format!("{:?}", error)),
        AlreadyPosted { .. } => super::error::ApplicationError::conflict(format!("{:?}", error)),
        AlreadyReversed { .. } => super::error::ApplicationError::conflict(format!("{:?}", error)),
        ConcurrencyConflict { .. } => super::error::ApplicationError::conflict(format!("{:?}", error)),
        InvalidAccountCode { .. } => super::error::ApplicationError::validation_failed(format!("{:?}", error)),
        InvalidAmount { .. } => super::error::ApplicationError::validation_failed(format!("{:?}", error)),
        _ => super::error::ApplicationError::infrastructure_error(format!("{:?}", error)),
    }
}

// =============================================================================
// 凭证聚合根 → DTO 映射
// =============================================================================

/// 将凭证聚合根映射为详情 DTO
impl From<JournalEntry> for JournalEntryDetail {
    fn from(aggregate: JournalEntry) -> Self {
        let (total_debit, total_credit) = aggregate.totals();

        JournalEntryDetail {
            document_number: aggregate.document_number().to_string(),
            fiscal_year: aggregate.fiscal_year(),
            company_code: aggregate.company_code().to_string(),
            posting_date: aggregate.posting_date(),
            document_date: aggregate.document_date(),
            currency_code: aggregate.currency_code().to_string(),
            status: aggregate.status().to_string(),
            header_text: aggregate.header_text().cloned(),
            reference_document: aggregate.reference_document().cloned(),
            total_debit: total_debit.as_f64(),
            total_credit: total_credit.as_f64(),
            line_items: aggregate.line_items().iter().map(Into::into).collect(),
            version: aggregate.version(),
            created_at: aggregate.created_at(),
            posted_at: aggregate.posted_at(),
        }
    }
}

/// 将凭证聚合根映射为摘要 DTO
impl From<&JournalEntry> for JournalEntrySummary {
    fn from(aggregate: &JournalEntry) -> Self {
        let (total_debit, total_credit) = aggregate.totals();

        JournalEntrySummary {
            document_number: aggregate.document_number().to_string(),
            fiscal_year: aggregate.fiscal_year(),
            posting_date: aggregate.posting_date(),
            document_date: aggregate.document_date(),
            currency_code: aggregate.currency_code().to_string(),
            status: aggregate.status().to_string(),
            total_amount: total_debit.as_f64() + total_credit.as_f64(),
            line_count: aggregate.line_items().len() as u32,
            header_text: aggregate.header_text().cloned(),
        }
    }
}

// =============================================================================
// 行项目实体 → DTO 映射
// =============================================================================

/// 将行项目实体映射为详情 DTO
impl From<&JournalEntryLineItem> for JournalEntryLineItemDetail {
    fn from(item: &JournalEntryLineItem) -> Self {
        JournalEntryLineItemDetail {
            line_number: item.line_number(),
            account_code: item.account_code().to_string(),
            amount: item.amount().as_f64(),
            debit_credit: item.debit_credit().to_string(),
            cost_center: item.cost_center().cloned(),
            profit_center: item.profit_center().cloned(),
            text: item.text().cloned(),
            functional_area: item.functional_area().cloned(),
            business_area: item.business_area().cloned(),
            order_number: item.order_number().cloned(),
        }
    }
}

// =============================================================================
// 领域事件 → DTO 映射（用于日志/审计）
// =============================================================================

/// 将领域事件映射为审计 DTO
impl From<&DomainEvent> for JournalEntryAuditLog {
    fn from(event: &DomainEvent) -> Self {
        match event {
            DomainEvent::JournalEntryPosted {
                document_number,
                posting_date,
                total_debit,
                total_credit,
                ..
            } => JournalEntryAuditLog {
                document_number: document_number.to_string(),
                action: "POSTED".to_string(),
                timestamp: chrono::Utc::now(),
                user_id: None,
                changes: serde_json::json!({
                    "posting_date": posting_date,
                    "total_debit": total_debit,
                    "total_credit": total_credit,
                }),
            },
            DomainEvent::JournalEntryReversed {
                original_document_number,
                reversal_date,
                ..
            } => JournalEntryAuditLog {
                document_number: original_document_number.to_string(),
                action: "REVERSED".to_string(),
                timestamp: chrono::Utc::now(),
                user_id: None,
                changes: serde_json::json!({
                    "reversal_date": reversal_date,
                }),
            },
            _ => JournalEntryAuditLog {
                document_number: "N/A".to_string(),
                action: event.event_type_name().to_uppercase(),
                timestamp: chrono::Utc::now(),
                user_id: None,
                changes: serde_json::json!({}),
            },
        }
    }
}

// =============================================================================
// 审计日志 DTO（内部使用）
// =============================================================================

/// 凭证审计日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryAuditLog {
    pub document_number: String,
    pub action: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<uuid::Uuid>,
    pub changes: serde_json::Value,
}
