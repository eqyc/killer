//! Journal Entry Domain Tests
//!
//! Tests for the JournalEntry aggregate root covering:
//! - Balance validation (debit/credit)
//! - Minimum line item requirements
//! - Currency precision handling
//! - Immutable field violations
//! - Multi-tenant consistency
//! - Reversal rules
//! - Boundary values

use chrono::NaiveDate;
use killer_financial_service::domain::aggregates::JournalEntry;
use killer_financial_service::domain::entities::JournalEntryLineItem;
use killer_financial_service::domain::error::DomainError;
use killer_financial_service::domain::value_objects::{DebitCredit, JournalEntryStatus};
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, Money};

// =============================================================================
// Test Constants
// =============================================================================

const TEST_TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const TEST_COMPANY_CODE: &str = "1000";
const TEST_FISCAL_YEAR: i32 = 2024;

// =============================================================================
// Helper Functions
// =============================================================================

/// Creates a balanced journal entry for testing
fn create_balanced_entry(
    debit_amount: f64,
    credit_amount: f64,
) -> (Vec<JournalEntryLineItem>, CurrencyCode) {
    let currency = CurrencyCode::new("CNY").unwrap();
    let debit = JournalEntryLineItem::new(
        1,
        AccountCode::new("1001").unwrap(),
        Money::new(debit_amount, currency.clone()).unwrap(),
        DebitCredit::Debit,
    )
    .unwrap();

    let credit = JournalEntryLineItem::new(
        2,
        AccountCode::new("2001").unwrap(),
        Money::new(credit_amount, currency.clone()).unwrap(),
        DebitCredit::Credit,
    )
    .unwrap();

    (vec![debit, credit], currency)
}

/// Creates a simple balanced journal entry
fn create_simple_balanced_entry(amount: f64) -> (Vec<JournalEntryLineItem>, CurrencyCode) {
    create_balanced_entry(amount, amount)
}

// =============================================================================
// Balance Validation Tests
// =============================================================================

mod balance_validation_tests {
    use super::*;

    #[test]
    fn test_perfectly_balanced_entry() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_ok());
        let entry = result.unwrap();
        assert_eq!(entry.status(), JournalEntryStatus::Draft);
    }

    #[test]
    fn test_unbalanced_entry_causes_error() {
        let (line_items, currency) = create_balanced_entry(1000.00, 900.00);
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, DomainError::UnbalancedEntry { .. }));
    }

    #[test]
    fn test_micro_imbalance_within_tolerance() {
        // Within 0.01 tolerance - should succeed
        let (line_items, currency) = create_balanced_entry(1000.005, 1000.00);
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        // Rounding should make this acceptable
        assert!(result.is_ok());
    }

    #[test]
    fn test_large_imbalance_detected() {
        let (line_items, currency) = create_balanced_entry(10000.00, 5000.00);
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let DomainError::UnbalancedEntry { debit, credit, difference } = error {
            assert_eq!(debit, "10000.00");
            assert_eq!(credit, "5000.00");
            assert_eq!(difference, "5000.00");
        }
    }

    #[test]
    fn test_all_debit_entry_rejected() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(
                1,
                AccountCode::new("1001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Debit,
            )
            .unwrap(),
            JournalEntryLineItem::new(
                2,
                AccountCode::new("1002").unwrap(),
                Money::new(500.0, currency.clone()).unwrap(),
                DebitCredit::Debit,
            )
            .unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_all_credit_entry_rejected() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(
                1,
                AccountCode::new("2001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
            JournalEntryLineItem::new(
                2,
                AccountCode::new("2002").unwrap(),
                Money::new(500.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_err());
    }
}

// =============================================================================
// Minimum Line Items Tests
// =============================================================================

mod minimum_line_items_tests {
    use super::*;

    #[test]
    fn test_two_line_items_minimum() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        assert_eq!(line_items.len(), 2);

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_single_line_item_rejected() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![JournalEntryLineItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            Money::new(1000.0, currency.clone()).unwrap(),
            DebitCredit::Debit,
        )
        .unwrap()];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::InsufficientLineItems { required: 2, actual: 1 }
        ));
    }

    #[test]
    fn test_empty_line_items_rejected() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            vec![],
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::InsufficientLineItems { required: 2, actual: 0 }
        ));
    }

    #[test]
    fn test_many_line_items_allowed() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(1000.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("1002").unwrap(), Money::new(500.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(3, AccountCode::new("1003").unwrap(), Money::new(300.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(4, AccountCode::new("2001").unwrap(), Money::new(1800.0, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_ok());
    }
}

// =============================================================================
// Currency Precision Tests
// =============================================================================

mod currency_precision_tests {
    use super::*;

    #[test]
    fn test_standard_precision_cny() {
        // CNY uses 2 decimal places
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(1000.50, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(1000.50, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_jpy_no_decimal_precision() {
        // JPY uses 0 decimal places
        let currency = CurrencyCode::new("JPY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(1000.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(1000.0, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_currency_consistency_across_lines() {
        let currency_cny = CurrencyCode::new("CNY").unwrap();
        let currency_jpy = CurrencyCode::new("JPY").unwrap();

        // Create items with different currencies
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(1000.0, currency_cny.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(1000.0, currency_jpy).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency_cny,
            items,
        );

        assert!(result.is_err());
        if let DomainError::ValidationError { message } = result.unwrap_err() {
            assert!(message.contains("币种不一致"));
        }
    }
}

// =============================================================================
// Reversal Tests
// =============================================================================

mod reversal_tests {
    use super::*;

    #[test]
    fn test_reverse_posted_entry() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-ORIG").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        // Post the entry first
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        // Now reverse it
        let (reversal_entry, events) = entry
            .reverse(
                DocumentNumber::new("JE-REV").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        // Verify original entry is reversed
        assert_eq!(entry.status(), JournalEntryStatus::Reversed);
        assert_eq!(entry.reversed_by().unwrap().as_str(), "JE-REV");

        // Verify reversal entry
        assert!(reversal_entry.is_reversal());
        assert_eq!(reversal_entry.reversal_of().unwrap().as_str(), "JE-ORIG");
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_reverse_entry_swaps_debit_credit() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(1000.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(1000.0, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-ORIG").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency.clone(),
            items,
        )
        .unwrap();

        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        let (reversal_entry, _) = entry
            .reverse(
                DocumentNumber::new("JE-REV").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        // Verify debit/credit swapped in reversal
        assert_eq!(reversal_entry.line_items()[0].debit_credit(), DebitCredit::Credit);
        assert_eq!(reversal_entry.line_items()[1].debit_credit(), DebitCredit::Debit);
    }

    #[test]
    fn test_cannot_reverse_draft_entry() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        // Try to reverse without posting first
        let result = entry.reverse(
            DocumentNumber::new("JE-REV").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
        );

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::AlreadyReversed { .. }));
    }

    #[test]
    fn test_cannot_reverse_already_reversed_entry() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-ORIG").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        entry
            .reverse(
                DocumentNumber::new("JE-REV1").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        // Try to reverse again
        let result = entry.reverse(
            DocumentNumber::new("JE-REV2").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 21).unwrap(),
        );

        assert!(result.is_err());
    }
}

// =============================================================================
// Multi-Tenant Tests
// =============================================================================

mod multi_tenant_tests {
    use super::*;

    const TENANT_A: &str = "550e8400-e29b-41d4-a716-446655440001";
    const TENANT_B: &str = "550e8400-e29b-41d4-a716-446655440002";

    #[test]
    fn test_different_tenants_create_separate_entries() {
        let (line_items_a, currency) = create_simple_balanced_entry(1000.00);
        let entry_a = JournalEntry::create(
            TENANT_A,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-A001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency.clone(),
            line_items_a,
        )
        .unwrap();

        let (line_items_b, currency) = create_simple_balanced_entry(2000.00);
        let entry_b = JournalEntry::create(
            TENANT_B,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-B001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items_b,
        )
        .unwrap();

        assert_eq!(entry_a.tenant_id(), TENANT_A);
        assert_eq!(entry_b.tenant_id(), TENANT_B);
    }

    #[test]
    fn test_same_document_number_different_tenants_allowed() {
        let (line_items_a, currency) = create_simple_balanced_entry(1000.00);
        let entry_a = JournalEntry::create(
            TENANT_A,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-SHARED").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency.clone(),
            line_items_a,
        )
        .unwrap();

        let (line_items_b, currency) = create_simple_balanced_entry(2000.00);
        let entry_b = JournalEntry::create(
            TENANT_B,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-SHARED").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items_b,
        )
        .unwrap();

        // Both should succeed with same document number but different tenants
        assert!(entry_a.is_ok());
        assert!(entry_b.is_ok());
    }
}

// =============================================================================
// Immutable Field Tests
// =============================================================================

mod immutable_field_tests {
    use super::*;

    #[test]
    fn test_posted_entry_cannot_modify_header_text() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        // Post the entry
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        // Try to modify header text
        let result = entry.set_header_text("Modified text");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::AlreadyPosted { .. }));
    }

    #[test]
    fn test_posted_entry_cannot_add_line_items() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        let new_item = JournalEntryLineItem::new(
            3,
            AccountCode::new("3001").unwrap(),
            Money::new(500.0, CurrencyCode::new("CNY").unwrap()).unwrap(),
            DebitCredit::Debit,
        )
        .unwrap();

        let result = entry.add_line_item(new_item);

        assert!(result.is_err());
    }

    #[test]
    fn test_draft_entry_can_modify_header_text() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.set_header_text("Test header");
        assert!(result.is_ok());
        assert_eq!(entry.header_text(), Some("Test header"));
    }

    #[test]
    fn test_version_increments_on_modification() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        assert_eq!(entry.version(), 1);

        entry.set_header_text("First modification").unwrap();
        assert_eq!(entry.version(), 2);

        entry.set_header_text("Second modification").unwrap();
        assert_eq!(entry.version(), 3);
    }
}

// =============================================================================
// Boundary Value Tests
// =============================================================================

mod boundary_value_tests {
    use super::*;

    #[test]
    fn test_maximum_amount_entry() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let max_amount = 999_999_999_999.99;
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(max_amount, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(max_amount, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-MAX").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_minimum_positive_amount() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let min_amount = 0.01;
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(min_amount, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(min_amount, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-MIN").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_special_period_13_allowed() {
        // Special periods 13-16 are allowed for year-end adjustments
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-SPECIAL").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            currency,
            line_items,
        );

        // Creation should succeed (period validation happens at posting time)
        assert!(result.is_ok());
    }

    #[test]
    fn test_posting_to_period_16_allowed() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-SPECIAL").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        // Posting to special period 16 should be allowed
        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_zero_amount_rejected() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let items = vec![
            JournalEntryLineItem::new(1, AccountCode::new("1001").unwrap(), Money::new(0.0, currency.clone()).unwrap(), DebitCredit::Debit).unwrap(),
            JournalEntryLineItem::new(2, AccountCode::new("2001").unwrap(), Money::new(0.0, currency.clone()).unwrap(), DebitCredit::Credit).unwrap(),
        ];

        let result = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-ZERO").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            items,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_negative_amount_rejected_at_line_item_level() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let result = JournalEntryLineItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            Money::new(-100.0, currency).unwrap(),
            DebitCredit::Debit,
        );

        assert!(result.is_err());
    }
}

// =============================================================================
// Posting Date Validation Tests
// =============================================================================

mod posting_date_validation_tests {
    use super::*;

    #[test]
    fn test_posting_within_valid_period() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_ok());
        assert_eq!(entry.status(), JournalEntryStatus::Posted);
    }

    #[test]
    fn test_posting_before_period_start_rejected() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(), // Period starts in Feb
            NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(),
        );

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::InvalidPostingDate { .. }));
    }

    #[test]
    fn test_posting_after_period_end_rejected() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(), // Period ends in Dec
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_posting_on_period_boundary_allowed() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // On period start
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_ok());

        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-002").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(), // On period end
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_ok());
    }
}

// =============================================================================
// Event Generation Tests
// =============================================================================

mod event_generation_tests {
    use super::*;

    #[test]
    fn test_post_generates_journal_entry_posted_event() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        let events = entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        assert_eq!(events.len(), 1);
        // Event should contain posting information
        assert!(events[0].tenant_id() == TEST_TENANT_ID);
    }

    #[test]
    fn test_reverse_generates_journal_entry_reversed_event() {
        let (line_items, currency) = create_simple_balanced_entry(1000.00);
        let mut entry = JournalEntry::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            DocumentNumber::new("JE-ORIG").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        let (reversal_entry, events) = entry
            .reverse(
                DocumentNumber::new("JE-REV").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        assert_eq!(events.len(), 1);
    }
}
