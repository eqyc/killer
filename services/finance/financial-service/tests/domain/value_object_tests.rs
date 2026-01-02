//! Value Object Tests
//!
//! Tests for domain value objects including:
//! - JournalEntryStatus (state machine)
//! - DebitCredit (借贷方向)
//! - PeriodStatus (期间状态)
//! - ValidityRange (有效期范围)
//! - ProfitCenter (利润中心)
//! - ID value objects

use chrono::NaiveDate;
use killer_financial_service::domain::value_objects::{
    DebitCredit, FiscalPeriodId, JournalEntryId, JournalEntryStatus, PeriodStatus, ProfitCenter,
    ValidityRange,
};

// =============================================================================
// Journal Entry Status Tests
// =============================================================================

mod journal_entry_status_tests {
    use super::*;

    #[test]
    fn test_draft_is_modifiable() {
        assert!(JournalEntryStatus::Draft.is_modifiable());
        assert!(JournalEntryStatus::Draft.can_post());
        assert!(!JournalEntryStatus::Draft.can_reverse());
    }

    #[test]
    fn test_posted_is_not_modifiable() {
        assert!(!JournalEntryStatus::Posted.is_modifiable());
        assert!(!JournalEntryStatus::Posted.can_post());
        assert!(JournalEntryStatus::Posted.can_reverse());
    }

    #[test]
    fn test_reversed_is_final() {
        assert!(!JournalEntryStatus::Reversed.is_modifiable());
        assert!(!JournalEntryStatus::Reversed.can_post());
        assert!(!JournalEntryStatus::Reversed.can_reverse());
    }

    #[test]
    fn test_status_display_format() {
        assert_eq!(format!("{}", JournalEntryStatus::Draft), "草稿");
        assert_eq!(format!("{}", JournalEntryStatus::Posted), "已过账");
        assert_eq!(format!("{}", JournalEntryStatus::Reversed), "已冲销");
    }

    #[test]
    fn test_status_serialization() {
        use serde_json;

        let draft = serde_json::to_string(&JournalEntryStatus::Draft).unwrap();
        assert!(draft.contains("draft"));

        let posted = serde_json::to_string(&JournalEntryStatus::Posted).unwrap();
        assert!(posted.contains("posted"));

        let reversed = serde_json::to_string(&JournalEntryStatus::Reversed).unwrap();
        assert!(reversed.contains("reversed"));
    }

    #[test]
    fn test_status_deserialization() {
        use serde_json;

        let draft: JournalEntryStatus = serde_json::from_str("\"draft\"").unwrap();
        assert_eq!(draft, JournalEntryStatus::Draft);

        let posted: JournalEntryStatus = serde_json::from_str("\"posted\"").unwrap();
        assert_eq!(posted, JournalEntryStatus::Posted);

        let reversed: JournalEntryStatus = serde_json::from_str("\"reversed\"").unwrap();
        assert_eq!(reversed, JournalEntryStatus::Reversed);
    }
}

// =============================================================================
// DebitCredit Tests
// =============================================================================

mod debit_credit_tests {
    use super::*;

    #[test]
    fn test_debit_opposite_is_credit() {
        assert_eq!(DebitCredit::Debit.opposite(), DebitCredit::Credit);
    }

    #[test]
    fn test_credit_opposite_is_debit() {
        assert_eq!(DebitCredit::Credit.opposite(), DebitCredit::Debit);
    }

    #[test]
    fn test_debit_is_debit() {
        assert!(DebitCredit::Debit.is_debit());
        assert!(!DebitCredit::Debit.is_credit());
    }

    #[test]
    fn test_credit_is_credit() {
        assert!(DebitCredit::Credit.is_credit());
        assert!(!DebitCredit::Credit.is_debit());
    }

    #[test]
    fn test_display_format() {
        assert_eq!(format!("{}", DebitCredit::Debit), "借");
        assert_eq!(format!("{}", DebitCredit::Credit), "贷");
    }

    #[test]
    fn test_serialization() {
        use serde_json;

        let debit = serde_json::to_string(&DebitCredit::Debit).unwrap();
        assert!(debit.contains("debit"));

        let credit = serde_json::to_string(&DebitCredit::Credit).unwrap();
        assert!(credit.contains("credit"));
    }
}

// =============================================================================
// Period Status Tests
// =============================================================================

mod period_status_tests {
    use super::*;

    #[test]
    fn test_open_allows_posting() {
        assert!(PeriodStatus::Open.allows_posting());
        assert!(PeriodStatus::Open.can_close());
        assert!(!PeriodStatus::Open.can_reopen());
    }

    #[test]
    fn test_closing_blocks_posting() {
        assert!(!PeriodStatus::Closing.allows_posting());
        assert!(!PeriodStatus::Closing.can_close()); // Already closing
        assert!(!PeriodStatus::Closing.can_reopen());
    }

    #[test]
    fn test_closed_blocks_posting() {
        assert!(!PeriodStatus::Closed.allows_posting());
        assert!(!PeriodStatus::Closed.can_close());
        assert!(PeriodStatus::Closed.can_reopen());
    }

    #[test]
    fn test_status_display_format() {
        assert_eq!(format!("{}", PeriodStatus::Open), "开放");
        assert_eq!(format!("{}", PeriodStatus::Closing), "结账中");
        assert_eq!(format!("{}", PeriodStatus::Closed), "已关闭");
    }

    #[test]
    fn test_status_serialization() {
        use serde_json;

        let open = serde_json::to_string(&PeriodStatus::Open).unwrap();
        assert!(open.contains("open"));

        let closing = serde_json::to_string(&PeriodStatus::Closing).unwrap();
        assert!(closing.contains("closing"));

        let closed = serde_json::to_string(&PeriodStatus::Closed).unwrap();
        assert!(closed.contains("closed"));
    }
}

// =============================================================================
// Validity Range Tests
// =============================================================================

mod validity_range_tests {
    use super::*;

    #[test]
    fn test_validity_range_creation() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert_eq!(range.valid_from, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(range.valid_to, NaiveDate::from_ymd_opt(2024, 1, 31).unwrap());
    }

    #[test]
    fn test_contains_date_within_range() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
    }

    #[test]
    fn test_does_not_contain_date_before_range() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(!range.contains(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()));
    }

    #[test]
    fn test_does_not_contain_date_after_range() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }

    #[test]
    fn test_contains_on_start_date() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
    }

    #[test]
    fn test_contains_on_end_date() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()));
    }

    #[test]
    fn test_overlaps_with_partial_overlap() {
        let range1 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        let range2 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(),
        );

        assert!(range1.overlaps(&range2));
    }

    #[test]
    fn test_overlaps_with_complete_containment() {
        let range1 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        let range2 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
        );

        assert!(range1.overlaps(&range2));
    }

    #[test]
    fn test_no_overlap_when_ranges_are_separated() {
        let range1 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        let range2 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(),
        );

        assert!(!range1.overlaps(&range2));
    }

    #[test]
    fn test_no_overlap_when_ranges_only_touch() {
        let range1 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        // range2 starts right after range1 ends
        let range2 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(),
        );

        assert!(!range1.overlaps(&range2));
    }

    #[test]
    fn test_display_format() {
        let range = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        let display = format!("{}", range);
        assert!(display.contains("至"));
    }
}

// =============================================================================
// Profit Center Tests
// =============================================================================

mod profit_center_tests {
    use super::*;

    #[test]
    fn test_profit_center_creation() {
        let pc = ProfitCenter::new("PC001");
        assert_eq!(pc.as_str(), "PC001");
    }

    #[test]
    fn test_profit_center_from_string() {
        let pc: ProfitCenter = "PC002".into();
        assert_eq!(pc.as_str(), "PC002");
    }

    #[test]
    fn test_profit_center_display() {
        let pc = ProfitCenter::new("PROFIT-001");
        assert_eq!(format!("{}", pc), "PROFIT-001");
    }

    #[test]
    fn test_profit_center_equality() {
        let pc1 = ProfitCenter::new("PC001");
        let pc2 = ProfitCenter::new("PC001");
        let pc3 = ProfitCenter::new("PC002");

        assert_eq!(pc1, pc2);
        assert_ne!(pc1, pc3);
    }

    #[test]
    fn test_profit_center_serialization() {
        use serde_json;

        let pc = ProfitCenter::new("PC001");
        let json = serde_json::to_string(&pc).unwrap();
        assert!(json.contains("PC001"));
    }
}

// =============================================================================
// Journal Entry ID Tests
// =============================================================================

mod journal_entry_id_tests {
    use super::*;

    #[test]
    fn test_journal_entry_id_creation() {
        let id = JournalEntryId::new(
            "tenant-001",
            "1000",
            2024,
            "JE-001",
        );

        assert_eq!(id.tenant_id, "tenant-001");
        assert_eq!(id.company_code, "1000");
        assert_eq!(id.fiscal_year, 2024);
        assert_eq!(id.document_number, "JE-001");
    }

    #[test]
    fn test_journal_entry_id_display() {
        let id = JournalEntryId::new(
            "tenant-001",
            "1000",
            2024,
            "JE-001",
        );

        let display = format!("{}", id);
        assert!(display.contains("tenant-001"));
        assert!(display.contains("1000"));
        assert!(display.contains("2024"));
        assert!(display.contains("JE-001"));
        // Format is tenant_id/company_code/fiscal_year/document_number
        assert!(display.contains('/'));
    }

    #[test]
    fn test_journal_entry_id_equality() {
        let id1 = JournalEntryId::new("tenant-001", "1000", 2024, "JE-001");
        let id2 = JournalEntryId::new("tenant-001", "1000", 2024, "JE-001");
        let id3 = JournalEntryId::new("tenant-002", "1000", 2024, "JE-001");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_journal_entry_id_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(JournalEntryId::new("tenant-001", "1000", 2024, "JE-001"));
        set.insert(JournalEntryId::new("tenant-001", "1000", 2024, "JE-001")); // Duplicate
        set.insert(JournalEntryId::new("tenant-002", "1000", 2024, "JE-001"));

        // Should have 2 unique entries
        assert_eq!(set.len(), 2);
    }
}

// =============================================================================
// Fiscal Period ID Tests
// =============================================================================

mod fiscal_period_id_tests {
    use super::*;

    #[test]
    fn test_fiscal_period_id_creation() {
        let id = FiscalPeriodId::new(
            "tenant-001",
            "1000",
            2024,
            1,
        );

        assert_eq!(id.tenant_id, "tenant-001");
        assert_eq!(id.company_code, "1000");
        assert_eq!(id.fiscal_year, 2024);
        assert_eq!(id.period, 1);
    }

    #[test]
    fn test_fiscal_period_id_display() {
        let id = FiscalPeriodId::new(
            "tenant-001",
            "1000",
            2024,
            1,
        );

        let display = format!("{}", id);
        assert!(display.contains("tenant-001"));
        assert!(display.contains("1000"));
        assert!(display.contains("2024"));
        assert!(display.contains('1'));
    }

    #[test]
    fn test_fiscal_period_id_equality() {
        let id1 = FiscalPeriodId::new("tenant-001", "1000", 2024, 1);
        let id2 = FiscalPeriodId::new("tenant-001", "1000", 2024, 1);
        let id3 = FiscalPeriodId::new("tenant-001", "1000", 2024, 2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_special_period_id() {
        let id = FiscalPeriodId::new(
            "tenant-001",
            "1000",
            2024,
            13, // Special period
        );

        assert_eq!(id.period, 13);
    }
}

// =============================================================================
// Clone and Copy Tests
// =============================================================================

mod clone_copy_tests {
    use super::*;

    #[test]
    fn test_status_can_be_cloned() {
        let status1 = JournalEntryStatus::Draft;
        let status2 = status1.clone();

        assert_eq!(status1, status2);
    }

    #[test]
    fn test_debit_credit_can_be_cloned() {
        let dc1 = DebitCredit::Debit;
        let dc2 = dc1.clone();

        assert_eq!(dc1, dc2);
    }

    #[test]
    fn test_validity_range_can_be_cloned() {
        let range1 = ValidityRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        let range2 = range1.clone();

        assert_eq!(range1, range2);
    }
}
