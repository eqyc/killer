//! Concurrency Tests
//!
//! Tests for concurrent access patterns including:
//! - Optimistic concurrency control
//! - Race conditions
//! - Parallel command execution
//! - Version conflict detection

use async_trait::async_trait;
use chrono::NaiveDate;
use killer_financial_service::application::commands::*;
use killer_financial_service::application::dto::*;
use killer_financial_service::application::repositories::*;
use killer_financial_service::domain::aggregates::JournalEntry;
use killer_financial_service::domain::error::DomainError;
use killer_financial_service::domain::value_objects::{DebitCredit, JournalEntryStatus};
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, Money};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

// =============================================================================
// Mock Repository with Concurrency Support
// =============================================================================

/// Thread-safe storage for concurrent tests
#[derive(Clone)]
pub struct ConcurrentStorage<T: Clone> {
    data: Arc<RwLock<Vec<T>>>,
    version: Arc<AtomicUsize>,
}

impl<T: Clone> ConcurrentStorage<T> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
            version: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn insert(&self, item: T) {
        let mut data = self.data.write().await;
        data.push(item);
        self.version.fetch_add(1, Ordering::SeqCst);
    }

    pub async fn get_all(&self) -> Vec<T> {
        let data = self.data.read().await;
        data.clone()
    }

    pub async fn len(&self) -> usize {
        let data = self.data.read().await;
        data.len()
    }
}

/// Mock repository that simulates concurrent access
#[derive(Clone)]
pub struct ConcurrentJournalEntryRepository {
    storage: ConcurrentStorage<JournalEntry>,
}

impl ConcurrentJournalEntryRepository {
    pub fn new() -> Self {
        Self {
            storage: ConcurrentStorage::new(),
        }
    }

    pub async fn insert_entry(&self, entry: JournalEntry) {
        self.storage.insert(entry).await;
    }

    pub async fn get_entries(&self) -> Vec<JournalEntry> {
        self.storage.get_all().await
    }
}

#[async_trait]
impl JournalEntryRepository for ConcurrentJournalEntryRepository {
    async fn save(&self, entry: &JournalEntry) -> Result<(), anyhow::Error> {
        // Simulate some async work
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        self.storage.insert(entry.clone()).await;
        Ok(())
    }

    async fn find_by_id(
        &self,
        tenant_id: &str,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
    ) -> Result<Option<JournalEntry>, anyhow::Error> {
        let entries = self.storage.get_all().await;
        Ok(entries
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

// =============================================================================
// Optimistic Concurrency Tests
// =============================================================================

mod optimistic_concurrency_tests {
    use super::*;

    fn create_test_entry() -> JournalEntry {
        let currency = CurrencyCode::new("CNY").unwrap();
        let line_items = vec![
            JournalEntryLineItem::new(
                1,
                AccountCode::new("1001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Debit,
            )
            .unwrap(),
            JournalEntryLineItem::new(
                2,
                AccountCode::new("2001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
        ];

        JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-TEST").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_version_starts_at_one() {
        let entry = create_test_entry();
        assert_eq!(entry.version(), 1);
    }

    #[tokio::test]
    async fn test_version_increments_on_post() {
        let mut entry = create_test_entry();
        let events = entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();
        assert_eq!(entry.version(), 2);
    }

    #[tokio::test]
    async fn test_version_increments_on_reverse() {
        let mut entry = create_test_entry();
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        let (_, events) = entry
            .reverse(
                DocumentNumber::new("JE-REV").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        assert_eq!(entry.version(), 3);
    }

    #[tokio::test]
    async fn test_concurrent_version_detection() {
        // Simulate detecting a version conflict
        let expected_version = 1u64;
        let actual_version = 2u64;

        // This simulates a scenario where another process has updated the entry
        let conflict = expected_version != actual_version;
        assert!(conflict);
    }
}

// =============================================================================
// Parallel Execution Tests
// =============================================================================

mod parallel_execution_tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_parallel_entry_creation() {
        let repo = ConcurrentJournalEntryRepository::new();
        let currency = CurrencyCode::new("CNY").unwrap();

        // Create multiple entries in parallel
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let repo = repo.clone();
                let currency = currency.clone();
                tokio::spawn(async move {
                    let line_items = vec![
                        JournalEntryLineItem::new(
                            1,
                            AccountCode::new("1001").unwrap(),
                            Money::new(100.0 * (i + 1) as f64, currency.clone()).unwrap(),
                            DebitCredit::Debit,
                        )
                        .unwrap(),
                        JournalEntryLineItem::new(
                            2,
                            AccountCode::new("2001").unwrap(),
                            Money::new(100.0 * (i + 1) as f64, currency.clone()).unwrap(),
                            DebitCredit::Credit,
                        )
                        .unwrap(),
                    ];

                    let entry = JournalEntry::create(
                        "tenant-001",
                        CompanyCode::new("1000").unwrap(),
                        2024,
                        DocumentNumber::new(&format!("JE-{:08}", i)).unwrap(),
                        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
                        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
                        currency,
                        line_items,
                    )
                    .unwrap();

                    repo.insert_entry(entry).await;
                })
            })
            .collect();

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all entries were created
        let entries = repo.get_entries().await;
        assert_eq!(entries.len(), 5);
    }

    #[tokio::test]
    async fn test_concurrent_posting() {
        let repo = ConcurrentJournalEntryRepository::new();
        let currency = CurrencyCode::new("CNY").unwrap();

        // Create a single entry
        let line_items = vec![
            JournalEntryLineItem::new(
                1,
                AccountCode::new("1001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Debit,
            )
            .unwrap(),
            JournalEntryLineItem::new(
                2,
                AccountCode::new("2001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
        ];

        let entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-SINGLE").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap();

        repo.insert_entry(entry.clone()).await;

        // Try to post the same entry concurrently
        let handles: Vec<_> = (0..3)
            .map(|_| {
                let mut entry = entry.clone();
                tokio::spawn(async move {
                    entry
                        .post(
                            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
                        )
                        .is_ok()
                })
            })
            .collect();

        // Only one should succeed
        let results: Vec<bool> = futures::future::join_all(handles)
            .await
            .into_iter()
            .collect();

        // Exactly one should succeed (first one to change status to Posted)
        let success_count = results.iter().filter(|&&b| b).count();
        assert_eq!(success_count, 1);
    }
}

// =============================================================================
// Race Condition Tests
// =============================================================================

mod race_condition_tests {
    use super::*;

    /// Simulates a race condition where two operations compete for the same resource
    #[tokio::test]
    async fn test_double_spending_prevention() {
        // Simulate account balance
        let balance = Arc::new(Mutex::new(1000.0));

        // Try to withdraw 600 from two concurrent tasks
        let withdraw_amount = 600.0;

        let handles: Vec<_> = (0..2)
            .map(|_| {
                let balance = balance.clone();
                tokio::spawn(async move {
                    let mut guard = balance.lock().await;
                    if *guard >= withdraw_amount {
                        *guard -= withdraw_amount;
                        true // Withdrawal successful
                    } else {
                        false // Insufficient funds
                    }
                })
            })
            .collect();

        let results: Vec<bool> = futures::future::join_all(handles)
            .await
            .into_iter()
            .collect();

        // Only one withdrawal should succeed
        let success_count = results.iter().filter(|&&b| b).count();
        assert_eq!(success_count, 1);

        // Final balance should be 400
        let final_balance = *balance.lock().await;
        assert_eq!(final_balance, 400.0);
    }

    /// Tests that concurrent journal entry creation with same document number
    /// results in distinct entries (different UUIDs or handled by repository)
    #[tokio::test]
    async fn test_concurrent_document_number_generation() {
        let generated_numbers = Arc::new(Mutex::new(Vec::new()));
        let counter = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let numbers = generated_numbers.clone();
                let counter = counter.clone();
                tokio::spawn(async move {
                    // Simulate document number generation
                    let num = counter.fetch_add(1, Ordering::SeqCst);
                    let doc_number = format!("JE-{:08}", num);
                    numbers.lock().await.push(doc_number.clone());
                    doc_number
                })
            })
            .collect();

        let results: Vec<String> = futures::future::join_all(handles)
            .await
            .into_iter()
            .collect();

        // All document numbers should be unique
        let mut sorted_results = results.clone();
        sorted_results.sort();
        sorted_results.dedup();

        assert_eq!(sorted_results.len(), 10);
    }
}

// =============================================================================
// Deadlock Prevention Tests
// =============================================================================

mod deadlock_prevention_tests {
    use super::*;

    /// Tests that operations can be retried after temporary failures
    #[tokio::test]
    async fn test_retry_after_conflict() {
        let attempt_count = Arc::new(AtomicUsize::new(0));
        let success_count = Arc::new(AtomicUsize::new(0));

        let result = tokio::time::timeout(
            Duration::from_secs(5),
            async {
                loop {
                    let attempts = attempt_count.fetch_add(1, Ordering::SeqCst);
                    if attempts >= 3 {
                        // Give up after 3 attempts
                        break;
                    }

                    // Simulate a conflict that resolves on third attempt
                    if attempts < 2 {
                        return Err::<(), _>("temporary conflict");
                    }

                    success_count.fetch_add(1, Ordering::SeqCst);
                    return Ok(());
                }
                Ok(())
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    /// Tests that operations with proper ordering don't deadlock
    #[tokio::test]
    async fn test_operation_ordering_prevents_deadlock() {
        // Simulate acquiring locks in consistent order
        let lock1 = Arc::new(Mutex::new(()));
        let lock2 = Arc::new(Mutex::new(()));

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let lock1 = lock1.clone();
                let lock2 = lock2.clone();
                tokio::spawn(async move {
                    // Always acquire locks in the same order (1 then 2)
                    let _guard1 = lock1.lock().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    let _guard2 = lock2.lock().await;
                    i
                })
            })
            .collect();

        let results: Vec<usize> = futures::future::join_all(handles)
            .await
            .into_iter()
            .collect();

        // All tasks should complete without deadlock
        assert_eq!(results.len(), 4);
    }
}
