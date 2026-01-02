//! 领域事件
//!
//! 定义财务领域的所有事件类型

use chrono::NaiveDate;
use killer_domain_primitives::{CompanyCode, CurrencyCode, DocumentNumber};
use serde::{Deserialize, Serialize};

// =============================================================================
// 领域事件枚举
// =============================================================================

/// 领域事件
///
/// 所有领域事件都是不可变的，记录已发生的业务事实
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum DomainEvent {
    /// 会计凭证已过账
    JournalEntryPosted {
        tenant_id: String,
        company_code: CompanyCode,
        fiscal_year: i32,
        document_number: DocumentNumber,
        posting_date: NaiveDate,
        currency: CurrencyCode,
        total_debit: f64,
        total_credit: f64,
    },

    /// 会计凭证已冲销
    JournalEntryReversed {
        tenant_id: String,
        company_code: CompanyCode,
        fiscal_year: i32,
        original_document_number: DocumentNumber,
        reversal_document_number: DocumentNumber,
        reversal_date: NaiveDate,
    },

    /// 会计期间已关闭
    FiscalPeriodClosed {
        tenant_id: String,
        company_code: CompanyCode,
        fiscal_year: i32,
        period: u8,
    },

    /// 会计期间已开放
    FiscalPeriodOpened {
        tenant_id: String,
        company_code: CompanyCode,
        fiscal_year: i32,
        period: u8,
    },
}

impl DomainEvent {
    /// 获取事件的租户ID
    pub fn tenant_id(&self) -> &str {
        match self {
            DomainEvent::JournalEntryPosted { tenant_id, .. } => tenant_id,
            DomainEvent::JournalEntryReversed { tenant_id, .. } => tenant_id,
            DomainEvent::FiscalPeriodClosed { tenant_id, .. } => tenant_id,
            DomainEvent::FiscalPeriodOpened { tenant_id, .. } => tenant_id,
        }
    }

    /// 获取事件类型名称
    pub fn event_type_name(&self) -> &'static str {
        match self {
            DomainEvent::JournalEntryPosted { .. } => "journal_entry_posted",
            DomainEvent::JournalEntryReversed { .. } => "journal_entry_reversed",
            DomainEvent::FiscalPeriodClosed { .. } => "fiscal_period_closed",
            DomainEvent::FiscalPeriodOpened { .. } => "fiscal_period_opened",
        }
    }

    /// 获取聚合根类型
    pub fn aggregate_type(&self) -> &'static str {
        match self {
            DomainEvent::JournalEntryPosted { .. } => "journal_entry",
            DomainEvent::JournalEntryReversed { .. } => "journal_entry",
            DomainEvent::FiscalPeriodClosed { .. } => "fiscal_period",
            DomainEvent::FiscalPeriodOpened { .. } => "fiscal_period",
        }
    }

    /// 获取聚合根ID（字符串表示）
    pub fn aggregate_id(&self) -> String {
        match self {
            DomainEvent::JournalEntryPosted {
                company_code,
                fiscal_year,
                document_number,
                ..
            } => format!(
                "{}/{}/{}",
                company_code.as_str(),
                fiscal_year,
                document_number.as_str()
            ),
            DomainEvent::JournalEntryReversed {
                company_code,
                fiscal_year,
                original_document_number,
                ..
            } => format!(
                "{}/{}/{}",
                company_code.as_str(),
                fiscal_year,
                original_document_number.as_str()
            ),
            DomainEvent::FiscalPeriodClosed {
                company_code,
                fiscal_year,
                period,
                ..
            } => format!(
                "{}/{}/{}",
                company_code.as_str(),
                fiscal_year,
                period
            ),
            DomainEvent::FiscalPeriodOpened {
                company_code,
                fiscal_year,
                period,
                ..
            } => format!(
                "{}/{}/{}",
                company_code.as_str(),
                fiscal_year,
                period
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_posted_event() {
        let event = DomainEvent::JournalEntryPosted {
            tenant_id: "tenant-001".to_string(),
            company_code: CompanyCode::new("1000").unwrap(),
            fiscal_year: 2024,
            document_number: DocumentNumber::new("JE-001").unwrap(),
            posting_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency: CurrencyCode::new("CNY").unwrap(),
            total_debit: 1000.0,
            total_credit: 1000.0,
        };

        assert_eq!(event.tenant_id(), "tenant-001");
        assert_eq!(event.event_type_name(), "journal_entry_posted");
        assert_eq!(event.aggregate_type(), "journal_entry");
    }

    #[test]
    fn test_fiscal_period_closed_event() {
        let event = DomainEvent::FiscalPeriodClosed {
            tenant_id: "tenant-001".to_string(),
            company_code: CompanyCode::new("1000").unwrap(),
            fiscal_year: 2024,
            period: 1,
        };

        assert_eq!(event.tenant_id(), "tenant-001");
        assert_eq!(event.event_type_name(), "fiscal_period_closed");
        assert_eq!(event.aggregate_type(), "fiscal_period");
    }

    #[test]
    fn test_event_serialization() {
        let event = DomainEvent::JournalEntryPosted {
            tenant_id: "tenant-001".to_string(),
            company_code: CompanyCode::new("1000").unwrap(),
            fiscal_year: 2024,
            document_number: DocumentNumber::new("JE-001").unwrap(),
            posting_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency: CurrencyCode::new("CNY").unwrap(),
            total_debit: 1000.0,
            total_credit: 1000.0,
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: DomainEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event, deserialized);
    }
}
