//! 领域值对象
//!
//! 定义财务领域的值对象和枚举类型

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// 凭证状态
// =============================================================================

/// 会计凭证状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalEntryStatus {
    /// 草稿 - 可编辑
    Draft,
    /// 已过账 - 不可修改，仅可冲销
    Posted,
    /// 已冲销
    Reversed,
}

impl fmt::Display for JournalEntryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "草稿"),
            Self::Posted => write!(f, "已过账"),
            Self::Reversed => write!(f, "已冲销"),
        }
    }
}

impl JournalEntryStatus {
    /// 是否可以修改
    pub fn is_modifiable(&self) -> bool {
        matches!(self, Self::Draft)
    }

    /// 是否可以过账
    pub fn can_post(&self) -> bool {
        matches!(self, Self::Draft)
    }

    /// 是否可以冲销
    pub fn can_reverse(&self) -> bool {
        matches!(self, Self::Posted)
    }
}

// =============================================================================
// 借贷方向
// =============================================================================

/// 借贷方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebitCredit {
    /// 借方
    Debit,
    /// 贷方
    Credit,
}

impl fmt::Display for DebitCredit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debit => write!(f, "借"),
            Self::Credit => write!(f, "贷"),
        }
    }
}

impl DebitCredit {
    /// 获取相反方向
    pub fn opposite(&self) -> Self {
        match self {
            Self::Debit => Self::Credit,
            Self::Credit => Self::Debit,
        }
    }

    /// 是否为借方
    pub fn is_debit(&self) -> bool {
        matches!(self, Self::Debit)
    }

    /// 是否为贷方
    pub fn is_credit(&self) -> bool {
        matches!(self, Self::Credit)
    }
}

// =============================================================================
// 会计期间状态
// =============================================================================

/// 会计期间状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodStatus {
    /// 开放 - 可以过账
    Open,
    /// 结账中 - 不可过账
    Closing,
    /// 已关闭 - 不可过账
    Closed,
}

impl fmt::Display for PeriodStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "开放"),
            Self::Closing => write!(f, "结账中"),
            Self::Closed => write!(f, "已关闭"),
        }
    }
}

impl PeriodStatus {
    /// 是否允许过账
    pub fn allows_posting(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// 是否可以关闭
    pub fn can_close(&self) -> bool {
        matches!(self, Self::Open | Self::Closing)
    }

    /// 是否可以重新开放
    pub fn can_reopen(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

// =============================================================================
// 凭证 ID
// =============================================================================

/// 会计凭证唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JournalEntryId {
    /// 租户ID
    pub tenant_id: String,
    /// 公司代码
    pub company_code: String,
    /// 会计年度
    pub fiscal_year: i32,
    /// 凭证号
    pub document_number: String,
}

impl JournalEntryId {
    /// 创建新的凭证ID
    pub fn new(
        tenant_id: impl Into<String>,
        company_code: impl Into<String>,
        fiscal_year: i32,
        document_number: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            company_code: company_code.into(),
            fiscal_year,
            document_number: document_number.into(),
        }
    }
}

impl fmt::Display for JournalEntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.tenant_id, self.company_code, self.fiscal_year, self.document_number
        )
    }
}

// =============================================================================
// 会计期间 ID
// =============================================================================

/// 会计期间唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FiscalPeriodId {
    /// 租户ID
    pub tenant_id: String,
    /// 公司代码
    pub company_code: String,
    /// 会计年度
    pub fiscal_year: i32,
    /// 期间 (1-16)
    pub period: u8,
}

impl FiscalPeriodId {
    /// 创建新的期间ID
    pub fn new(
        tenant_id: impl Into<String>,
        company_code: impl Into<String>,
        fiscal_year: i32,
        period: u8,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            company_code: company_code.into(),
            fiscal_year,
            period,
        }
    }
}

impl fmt::Display for FiscalPeriodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.tenant_id, self.company_code, self.fiscal_year, self.period
        )
    }
}

// =============================================================================
// 有效期范围
// =============================================================================

/// 有效期范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidityRange {
    /// 开始日期
    pub valid_from: NaiveDate,
    /// 结束日期
    pub valid_to: NaiveDate,
}

impl ValidityRange {
    /// 创建新的有效期范围
    pub fn new(valid_from: NaiveDate, valid_to: NaiveDate) -> Self {
        Self {
            valid_from,
            valid_to,
        }
    }

    /// 检查日期是否在有效期内
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.valid_from && date <= self.valid_to
    }

    /// 检查是否与另一个范围重叠
    pub fn overlaps(&self, other: &ValidityRange) -> bool {
        self.valid_from <= other.valid_to && self.valid_to >= other.valid_from
    }
}

impl fmt::Display for ValidityRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 至 {}", self.valid_from, self.valid_to)
    }
}

// =============================================================================
// 利润中心（值对象）
// =============================================================================

/// 利润中心
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfitCenter(String);

impl ProfitCenter {
    /// 创建新的利润中心
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    /// 获取代码
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProfitCenter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ProfitCenter {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ProfitCenter {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_status() {
        assert!(JournalEntryStatus::Draft.is_modifiable());
        assert!(!JournalEntryStatus::Posted.is_modifiable());
        assert!(JournalEntryStatus::Posted.can_reverse());
        assert!(!JournalEntryStatus::Reversed.can_reverse());
    }

    #[test]
    fn test_debit_credit() {
        assert_eq!(DebitCredit::Debit.opposite(), DebitCredit::Credit);
        assert_eq!(DebitCredit::Credit.opposite(), DebitCredit::Debit);
        assert!(DebitCredit::Debit.is_debit());
        assert!(DebitCredit::Credit.is_credit());
    }

    #[test]
    fn test_period_status() {
        assert!(PeriodStatus::Open.allows_posting());
        assert!(!PeriodStatus::Closing.allows_posting());
        assert!(!PeriodStatus::Closed.allows_posting());
    }

    #[test]
    fn test_validity_range() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }
}
