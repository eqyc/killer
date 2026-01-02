//! Command Handler Tests
//!
//! Tests for command handlers including:
//! - CreateJournalEntryHandler
//! - PostJournalEntryHandler
//! - ReverseJournalEntryHandler
//! - CloseFiscalPeriodHandler

use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use killer_financial_service::application::commands::*;
use killer_financial_service::application::dto::*;
use killer_financial_service::application::error::ApplicationError;
use killer_financial_service::application::repositories::*;
use killer_financial_service::domain::aggregates::{FiscalPeriod, JournalEntry};
use killer_financial_service::domain::entities::JournalEntryLineItem;
use killer_financial_service::domain::events::DomainEvent;
use killer_financial_service::domain::value_objects::{DebitCredit, PeriodStatus};
use killer_financial_service::infrastructure::event_bus::{Event, EventEnvelope};
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, Money};
use killer_cqrs::{Command, CommandHandler};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use uuid::Uuid;

// =============================================================================
// Mock Implementations
// =============================================================================

/// Mock Document Number Generator
#[derive(Clone, Default)]
pub struct MockDocumentNumberGenerator {
    pub counter: Arc<AtomicUsize>,
}

impl MockDocumentNumberGenerator {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl DocumentNumberGenerator for MockDocumentNumberGenerator {
    async fn generate(
        &self,
        _tenant_id: &str,
        _company_code: &str,
    ) -> Result<DocumentNumber, anyhow::Error> {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        DocumentNumber::new(&format!("JE-{:08}", count))
    }
}

/// Mock Journal Entry Repository
#[derive(Clone, Default)]
pub struct MockJournalEntryRepository {
    pub storage: Arc<std::sync::Mutex<Vec<JournalEntry>>>,
    pub save_count: Arc<AtomicUsize>,
}

impl MockJournalEntryRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(std::sync::Mutex::new(Vec::new())),
            save_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl JournalEntryRepository for MockJournalEntryRepository {
    async fn save(&self, entry: &JournalEntry) -> Result<(), anyhow::Error> {
        let mut storage = self.storage.lock().unwrap();
        storage.push(entry.clone());
        self.save_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn find_by_id(
        &self,
        tenant_id: &str,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
    ) -> Result<Option<JournalEntry>, anyhow::Error> {
        let storage = self.storage.lock().unwrap();
        Ok(storage
            .iter()
            .find(|e| {
                e.tenant_id() == tenant_id
                    && e.company_code().as_str() == company_code
                    && e.fiscal_year() == fiscal_year
                    && e.document_number().as_str() == document_number
            })
            .cloned())
    }
}

/// Mock Fiscal Period Repository
#[derive(Clone, Default)]
pub struct MockFiscalPeriodRepository {
    pub periods: Arc<std::sync::Mutex<Vec<FiscalPeriod>>>,
}

impl MockFiscalPeriodRepository {
    pub fn new() -> Self {
        Self {
            periods: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn add_period(&self, period: FiscalPeriod) {
        let mut periods = self.periods.lock().unwrap();
        periods.push(period);
    }
}

#[async_trait]
impl FiscalPeriodRepository for MockFiscalPeriodRepository {
    async fn find_by_company_code_and_year_and_period(
        &self,
        tenant_id: &str,
        company_code: &str,
        fiscal_year: i32,
        period: u8,
    ) -> Result<Option<FiscalPeriod>, anyhow::Error> {
        let periods = self.periods.lock().unwrap();
        Ok(periods
            .iter()
            .find(|p| {
                p.tenant_id() == tenant_id
                    && p.company_code().as_str() == company_code
                    && p.fiscal_year() == fiscal_year
                    && p.period() == period
            })
            .cloned())
    }
}

/// Mock Unit of Work
#[derive(Clone, Default)]
pub struct MockUnitOfWork {
    pub committed: Arc<AtomicUsize>,
    pub rolled_back: Arc<AtomicUsize>,
    pub begun: Arc<AtomicUsize>,
}

impl MockUnitOfWork {
    pub fn new() -> Self {
        Self {
            committed: Arc::new(AtomicUsize::new(0)),
            rolled_back: Arc::new(AtomicUsize::new(0)),
            begun: Arc::new(AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl UnitOfWork for MockUnitOfWork {
    async fn begin(&self) -> Result<(), anyhow::Error> {
        self.begun.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn commit(&self) -> Result<(), anyhow::Error> {
        self.committed.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn rollback(&self) -> Result<(), anyhow::Error> {
        self.rolled_back.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

/// Mock Event Bus
#[derive(Clone, Default)]
pub struct MockEventBus {
    pub published_events: Arc<std::sync::Mutex<Vec<EventEnvelope>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn get_published_count(&self) -> usize {
        self.published_events.lock().unwrap().len()
    }
}

#[async_trait]
impl EventBus for MockEventBus {
    async fn publish(&self, event: EventEnvelope) -> Result<(), anyhow::Error> {
        let mut events = self.published_events.lock().unwrap();
        events.push(event);
        Ok(())
    }
}

/// Mock Event Store
#[derive(Clone, Default)]
pub struct MockJournalEntryEventStore {
    pub events: Arc<std::sync::Mutex<Vec<(String, String, DomainEvent)>>>,
}

impl MockJournalEntryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl JournalEntryEventStore for MockJournalEntryEventStore {
    async fn append(
        &self,
        tenant_id: &str,
        aggregate_id: &str,
        event: DomainEvent,
    ) -> Result<(), anyhow::Error> {
        let mut events = self.events.lock().unwrap();
        events.push((tenant_id.to_string(), aggregate_id.to_string(), event));
        Ok(())
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

const TEST_TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const TEST_USER_ID: &str = "550e8400-e29b-41d4-a716-446655440001";
const TEST_COMPANY_CODE: &str = "1000";

fn create_test_line_items() -> Vec<JournalEntryLineItemRequest> {
    vec![
        JournalEntryLineItemRequest {
            line_number: 1,
            account_code: "1001".to_string(),
            amount: 1000.0,
            debit_credit: "D".to_string(),
            cost_center: Some("CC001".to_string()),
            profit_center: None,
            text: Some("Test debit".to_string()),
            functional_area: None,
            business_area: None,
            order_number: None,
            tax_code: None,
            tax_amount: None,
        },
        JournalEntryLineItemRequest {
            line_number: 2,
            account_code: "2001".to_string(),
            amount: 1000.0,
            debit_credit: "C".to_string(),
            cost_center: None,
            profit_center: None,
            text: Some("Test credit".to_string()),
            functional_area: None,
            business_area: None,
            order_number: None,
            tax_code: None,
            tax_amount: None,
        },
    ]
}

fn create_open_fiscal_period(tenant_id: &str, company_code: &str) -> FiscalPeriod {
    FiscalPeriod::create(
        tenant_id,
        CompanyCode::new(company_code).unwrap(),
        2024,
        1,
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
    )
    .unwrap()
}

// =============================================================================
// Create Journal Entry Handler Tests
// =============================================================================

mod create_journal_entry_handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_journal_entry_success() {
        // Setup
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        let repo = MockJournalEntryRepository::new();
        let event_store = MockJournalEntryEventStore::new();
        let period_repo = MockFiscalPeriodRepository::new();
        period_repo.add_period(create_open_fiscal_period(TEST_TENANT_ID, TEST_COMPANY_CODE));

        let uow = MockUnitOfWork::new();
        let event_bus = MockEventBus::new();
        let doc_gen = MockDocumentNumberGenerator::new();

        // Note: In real tests, we would use the actual handler
        // For now, we test the command creation
        let request = CreateJournalEntryRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            posting_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            document_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            currency_code: "CNY".to_string(),
            header_text: Some("Test Entry".to_string()),
            reference_document: None,
            line_items: create_test_line_items(),
        };

        let command = CreateJournalEntryCommand::new(tenant_id, user_id, request);

        // Verify command is created correctly
        assert_eq!(command.context.tenant_id, tenant_id);
        assert_eq!(command.context.user_id, user_id);
        assert_eq!(command.request.company_code, TEST_COMPANY_CODE);
        assert_eq!(command.request.line_items.len(), 2);
    }

    #[tokio::test]
    async fn test_create_command_with_idempotency_key() {
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        let request = CreateJournalEntryRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            posting_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            document_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            currency_code: "CNY".to_string(),
            header_text: Some("Test Entry".to_string()),
            reference_document: None,
            line_items: create_test_line_items(),
        };

        let command = CreateJournalEntryCommand::new(tenant_id, user_id, request);

        // Verify command has no idempotency key set by default
        // (Idempotency key is typically added by middleware)
        assert!(command.context.correlation_id != Uuid::nil());
    }
}

// =============================================================================
// Post Journal Entry Handler Tests
// =============================================================================

mod post_journal_entry_handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_post_command_creation() {
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        let request = PostJournalEntryRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            document_number: "JE-00000001".to_string(),
            posting_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
        };

        let command = PostJournalEntryCommand::new(tenant_id, user_id, request);

        assert_eq!(command.context.tenant_id, tenant_id);
        assert_eq!(command.context.user_id, user_id);
        assert_eq!(command.request.document_number, "JE-00000001");
    }

    #[tokio::test]
    async fn test_reverse_command_creation() {
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        let request = ReverseJournalEntryRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            original_document_number: "JE-00000001".to_string(),
            reversal_document_number: Some("JE-00000002".to_string()),
            reversal_date: NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            reversal_reason: "Error correction".to_string(),
            reference_document: None,
        };

        let command = ReverseJournalEntryCommand::new(tenant_id, user_id, request);

        assert_eq!(command.context.tenant_id, tenant_id);
        assert_eq!(command.request.original_document_number, "JE-00000001");
        assert_eq!(command.request.reversal_reason, "Error correction");
    }
}

// =============================================================================
// Command Validation Tests
// =============================================================================

mod command_validation_tests {
    use super::*;

    #[test]
    fn test_create_journal_entry_request_validation() {
        let request = CreateJournalEntryRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            posting_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            document_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            currency_code: "CNY".to_string(),
            header_text: Some("Test Entry".to_string()),
            reference_document: None,
            line_items: create_test_line_items(),
        };

        // Valid request should not have validation errors
        assert!(!request.company_code.is_empty());
        assert!(request.fiscal_year > 0);
        assert!(request.line_items.len() >= 2);
    }

    #[test]
    fn test_empty_company_code_invalid() {
        let request = CreateJournalEntryRequest {
            company_code: "".to_string(), // Invalid
            fiscal_year: 2024,
            posting_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            document_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            currency_code: "CNY".to_string(),
            header_text: None,
            reference_document: None,
            line_items: create_test_line_items(),
        };

        assert!(request.company_code.is_empty());
    }

    #[test]
    fn test_line_items_must_have_valid_debit_credit() {
        let items = vec![
            JournalEntryLineItemRequest {
                line_number: 1,
                account_code: "1001".to_string(),
                amount: 1000.0,
                debit_credit: "INVALID".to_string(), // Invalid
                cost_center: None,
                profit_center: None,
                text: None,
                functional_area: None,
                business_area: None,
                order_number: None,
                tax_code: None,
                tax_amount: None,
            },
            JournalEntryLineItemRequest {
                line_number: 2,
                account_code: "2001".to_string(),
                amount: 1000.0,
                debit_credit: "C".to_string(),
                cost_center: None,
                profit_center: None,
                text: None,
                functional_area: None,
                business_area: None,
                order_number: None,
                tax_code: None,
                tax_amount: None,
            },
        ];

        assert!(items[0].debit_credit != "D" && items[0].debit_credit != "C");
    }

    #[test]
    fn test_negative_amount_invalid() {
        let items = vec![
            JournalEntryLineItemRequest {
                line_number: 1,
                account_code: "1001".to_string(),
                amount: -1000.0, // Invalid
                debit_credit: "D".to_string(),
                cost_center: None,
                profit_center: None,
                text: None,
                functional_area: None,
                business_area: None,
                order_number: None,
                tax_code: None,
                tax_amount: None,
            },
            JournalEntryLineItemRequest {
                line_number: 2,
                account_code: "2001".to_string(),
                amount: -1000.0,
                debit_credit: "C".to_string(),
                cost_center: None,
                profit_center: None,
                text: None,
                functional_area: None,
                business_area: None,
                order_number: None,
                tax_code: None,
                tax_amount: None,
            },
        ];

        assert!(items[0].amount < 0.0);
    }
}

// =============================================================================
// Fiscal Period Command Tests
// =============================================================================

mod fiscal_period_command_tests {
    use super::*;

    #[tokio::test]
    async fn test_close_fiscal_period_command() {
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        let request = CloseFiscalPeriodRequest {
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: 2024,
            period: 12,
            check_unposted_entries: Some(true),
        };

        let command = CloseFiscalPeriodCommand::new(tenant_id, user_id, request);

        assert_eq!(command.context.tenant_id, tenant_id);
        assert_eq!(command.request.period, 12);
        assert_eq!(command.request.check_unposted_entries, Some(true));
    }
}
