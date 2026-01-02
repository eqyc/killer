//! Fiscal Period Domain Tests
//!
//! Tests for the FiscalPeriod aggregate root covering:
//! - Period creation and validation
//! - Period status transitions (Open -> Closing -> Closed -> Open)
//! - Special periods (13-16)
//! - Date range validation
//! - Multi-tenant isolation

use chrono::NaiveDate;
use killer_financial_service::domain::aggregates::FiscalPeriod;
use killer_financial_service::domain::error::DomainError;
use killer_financial_service::domain::value_objects::PeriodStatus;
use killer_domain_primitives::CompanyCode;

// =============================================================================
// Test Constants
// =============================================================================

const TEST_TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const TEST_COMPANY_CODE: &str = "1000";
const TEST_FISCAL_YEAR: i32 = 2024;

// =============================================================================
// Creation Tests
// =============================================================================

mod creation_tests {
    use super::*;

    #[test]
    fn test_create_standard_period() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(period.is_ok());
        let period = period.unwrap();
        assert_eq!(period.period(), 1);
        assert_eq!(period.status(), PeriodStatus::Open);
        assert_eq!(period.fiscal_year(), TEST_FISCAL_YEAR);
    }

    #[test]
    fn test_create_period_with_description() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.set_description("January 2024");

        assert_eq!(period.description(), Some("January 2024"));
    }

    #[test]
    fn test_period_number_zero_rejected() {
        let result = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            0, // Invalid period number
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_period_number_17_rejected() {
        let result = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            17, // Invalid period number (max is 16)
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_start_date_after_end_date_rejected() {
        let result = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(), // Start after end
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_same_start_end_date_allowed() {
        let result = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(), // Same day
        );

        assert!(result.is_ok());
    }
}

// =============================================================================
// Period Status Transition Tests
// =============================================================================

mod status_transition_tests {
    use super::*;

    #[test]
    fn test_open_to_closing() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let result = period.start_closing();

        assert!(result.is_ok());
        assert_eq!(period.status(), PeriodStatus::Closing);
    }

    #[test]
    fn test_closing_to_closed() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.start_closing().unwrap();
        let events = period.close().unwrap();

        assert_eq!(period.status(), PeriodStatus::Closed);
        assert_eq!(events.len(), 1);
        assert!(!period.allows_posting());
    }

    #[test]
    fn test_open_to_closed_directly() {
        // Can close directly from open without going through closing
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let events = period.close().unwrap();

        assert_eq!(period.status(), PeriodStatus::Closed);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_closed_to_open() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.close().unwrap();
        let events = period.open().unwrap();

        assert_eq!(period.status(), PeriodStatus::Open);
        assert_eq!(events.len(), 1);
        assert!(period.allows_posting());
    }

    #[test]
    fn test_cannot_reopen_already_open_period() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let result = period.open();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::InvalidPeriodStatus { .. }));
    }

    #[test]
    fn test_cannot_close_already_closed_period() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.close().unwrap();
        let result = period.close();

        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_closing_already_closed_period() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.close().unwrap();
        let result = period.start_closing();

        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_closing_already_closing_period() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.start_closing().unwrap();
        let result = period.start_closing();

        assert!(result.is_err());
    }
}

// =============================================================================
// Special Period Tests (13-16)
// =============================================================================

mod special_period_tests {
    use super::*;

    #[test]
    fn test_period_13_is_special() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            13,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_special_period());
    }

    #[test]
    fn test_period_16_is_special() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            16,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_special_period());
    }

    #[test]
    fn test_period_12_is_not_special() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            12,
            NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .unwrap();

        assert!(!period.is_special_period());
    }

    #[test]
    fn test_all_special_periods_creation_allowed() {
        for period_num in 13..=16 {
            let result = FiscalPeriod::create(
                TEST_TENANT_ID,
                CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
                TEST_FISCAL_YEAR,
                period_num,
                NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            );

            assert!(result.is_ok(), "Period {} should be creatable", period_num);
        }
    }
}

// =============================================================================
// Date Range Tests
// =============================================================================

mod date_range_tests {
    use super::*;

    #[test]
    fn test_is_open_at_within_range() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
    }

    #[test]
    fn test_is_open_at_before_range() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()));
    }

    #[test]
    fn test_is_open_at_after_range() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }

    #[test]
    fn test_is_open_at_on_boundary() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        assert!(period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()));
    }

    #[test]
    fn test_closed_period_not_open_at_any_date() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.close().unwrap();

        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()));
    }

    #[test]
    fn test_closing_period_not_open() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.start_closing().unwrap();

        assert!(!period.allows_posting());
        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
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
    fn test_different_tenants_can_have_same_period() {
        let period_a = FiscalPeriod::create(
            TENANT_A,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let period_b = FiscalPeriod::create(
            TENANT_B,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert_eq!(period_a.tenant_id(), TENANT_A);
        assert_eq!(period_b.tenant_id(), TENANT_B);

        // Closing one tenant's period shouldn't affect the other
        let mut period_a_closed = period_a.clone();
        period_a_closed.close().unwrap();

        // Period B should still be open
        assert!(period_b.allows_posting());
    }

    #[test]
    fn test_tenant_isolation_on_status_changes() {
        let mut period_a = FiscalPeriod::create(
            TENANT_A,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let period_b = FiscalPeriod::create(
            TENANT_B,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        // Close period A
        period_a.close().unwrap();

        // Period B should still be open
        assert_eq!(period_b.status(), PeriodStatus::Open);
        assert!(period_b.allows_posting());
    }
}

// =============================================================================
// Version and Event Tests
// =============================================================================

mod version_and_event_tests {
    use super::*;

    #[test]
    fn test_version_starts_at_one() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert_eq!(period.version(), 1);
    }

    #[test]
    fn test_version_increments_on_status_change() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert_eq!(period.version(), 1);

        period.start_closing().unwrap();
        assert_eq!(period.version(), 2);

        period.close().unwrap();
        assert_eq!(period.version(), 3);

        period.open().unwrap();
        assert_eq!(period.version(), 4);
    }

    #[test]
    fn test_close_generates_fiscal_period_closed_event() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let events = period.close().unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tenant_id(), TEST_TENANT_ID);
        assert_eq!(events[0].period(), 1);
    }

    #[test]
    fn test_open_generates_fiscal_period_opened_event() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        period.close().unwrap();
        let events = period.open().unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tenant_id(), TEST_TENANT_ID);
    }

    #[test]
    fn test_start_closing_no_event() {
        let mut period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let result = period.start_closing();

        assert!(result.is_ok());
        // start_closing doesn't generate an event, just changes status
    }
}

// =============================================================================
// Validity Range Tests
// =============================================================================

mod validity_range_tests {
    use super::*;

    #[test]
    fn test_validity_range_contains() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let range = period.validity_range();
        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
    }

    #[test]
    fn test_validity_range_accessors() {
        let period = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            TEST_FISCAL_YEAR,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert_eq!(period.valid_from(), NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(period.valid_to(), NaiveDate::from_ymd_opt(2024, 1, 31).unwrap());
    }
}

// =============================================================================
// Fiscal Year Boundary Tests
// =============================================================================

mod fiscal_year_boundary_tests {
    use super::*;

    #[test]
    fn test_create_period_for_different_fiscal_years() {
        for year in [2023, 2024, 2025] {
            let result = FiscalPeriod::create(
                TEST_TENANT_ID,
                CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
                year,
                1,
                NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(year, 1, 31).unwrap(),
            );

            assert!(result.is_ok(), "Period for year {} should be creatable", year);
        }
    }

    #[test]
    fn test_year_end_special_periods() {
        // Period 13-16 are typically used for year-end adjustments
        let period_13 = FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new(TEST_COMPANY_CODE).unwrap(),
            2024,
            13,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .unwrap();

        assert!(period_13.is_special_period());
    }
}
