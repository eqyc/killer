//! 领域错误类型
//!
//! 定义财务领域的所有业务规则违反错误

use thiserror::Error;

/// 领域错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    /// 借贷不平衡
    #[error("借贷不平衡: 借方 {debit}, 贷方 {credit}, 差额 {difference}")]
    UnbalancedEntry {
        debit: String,
        credit: String,
        difference: String,
    },

    /// 会计期间已关闭
    #[error("会计期间已关闭: {company_code} {fiscal_year}/{period}")]
    PeriodClosed {
        company_code: String,
        fiscal_year: i32,
        period: u8,
    },

    /// 行项目数量不足
    #[error("行项目数量不足: 至少需要 {required} 行，实际 {actual} 行")]
    InsufficientLineItems {
        required: usize,
        actual: usize,
    },

    /// 无效的过账日期
    #[error("无效的过账日期: {date} 不在会计期间 {period_start} 至 {period_end} 范围内")]
    InvalidPostingDate {
        date: String,
        period_start: String,
        period_end: String,
    },

    /// 凭证已过账
    #[error("凭证已过账: {document_number}，不可修改")]
    AlreadyPosted {
        document_number: String,
    },

    /// 凭证已冲销
    #[error("凭证已冲销: {document_number}")]
    AlreadyReversed {
        document_number: String,
    },

    /// 并发冲突
    #[error("并发冲突: 期望版本 {expected}，实际版本 {actual}")]
    ConcurrencyConflict {
        expected: u64,
        actual: u64,
    },

    /// 租户不匹配
    #[error("租户不匹配: 期望 {expected}，实际 {actual}")]
    TenantMismatch {
        expected: String,
        actual: String,
    },

    /// 无效的金额
    #[error("无效的金额: {reason}")]
    InvalidAmount {
        reason: String,
    },

    /// 无效的会计科目
    #[error("无效的会计科目: {account_code}")]
    InvalidAccount {
        account_code: String,
    },

    /// 期间状态无效
    #[error("期间状态无效: 当前状态 {current_status}，不允许操作 {operation}")]
    InvalidPeriodStatus {
        current_status: String,
        operation: String,
    },

    /// 存在未过账凭证
    #[error("存在未过账凭证: {count} 个凭证未过账")]
    UnpostedEntriesExist {
        count: usize,
    },

    /// 业务规则违反
    #[error("业务规则违反: {rule}")]
    BusinessRuleViolation {
        rule: String,
    },

    /// 验证失败
    #[error("验证失败: {message}")]
    ValidationError {
        message: String,
    },
}

/// 领域结果类型
pub type DomainResult<T> = Result<T, DomainError>;
