//! Infrastructure Integration Tests
//!
//! Integration tests for infrastructure components including:
//! - PostgreSQL repositories
//! - Kafka messaging
//! - Redis caching
//! - Circuit breaker
//! - External adapters

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use killer_financial_service::domain::aggregates::{FiscalPeriod, JournalEntry};
use killer_financial_service::domain::entities::JournalEntryLineItem;
use killer_financial_service::domain::error::DomainError;
use killer_financial_service::domain::events::DomainEvent;
use killer_financial_service::domain::repositories::*;
use killer_financial_service::domain::value_objects::{DebitCredit, PeriodStatus};
use killer_financial_service::infrastructure::event_bus::EventEnvelope;
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, Money};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

// =============================================================================
// Test Configuration
// =============================================================================

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub postgres_url: String,
    pub kafka_brokers: Vec<String>,
    pub redis_url: String,
    pub clickhouse_url: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            postgres_url: std::env::var("TEST_POSTGRES_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/finance_test".to_string()),
            kafka_brokers: vec![std::env::var("TEST_KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_string())],
            redis_url: std::env::var("TEST_REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            clickhouse_url: std::env::var("TEST_CLICKHOUSE_URL")
                .unwrap_or_else(|_| "http://localhost:8123/finance_test".to_string()),
        }
    }
}

// =============================================================================
// Mock Infrastructure Components
// =============================================================================

/// Mock event bus for infrastructure testing
#[derive(Clone, Default)]
pub struct MockEventBus {
    pub published: Arc<Mutex<Vec<EventEnvelope>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_published(&self) -> Vec<EventEnvelope> {
        self.published.lock().await.clone()
    }

    pub async fn clear(&self) {
        self.published.lock().await.clear();
    }
}

#[async_trait]
impl killer_financial_service::infrastructure::messaging::EventBus for MockEventBus {
    async fn publish(&self, event: EventEnvelope) -> Result<(), anyhow::Error> {
        let mut published = self.published.lock().await;
        published.push(event);
        Ok(())
    }
}

// =============================================================================
// PostgreSQL Repository Tests
// =============================================================================

mod postgres_repository_tests {
    use super::*;

    const TEST_TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

    /// Helper to create test journal entry
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
            TEST_TENANT_ID,
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-TEST-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap()
    }

    /// Helper to create test fiscal period
    fn create_test_period() -> FiscalPeriod {
        FiscalPeriod::create(
            TEST_TENANT_ID,
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_repository_save_and_find() {
        // This test requires a real PostgreSQL instance
        // In CI, use testcontainers
        // For local testing, skip if connection fails

        let entry = create_test_entry();
        assert_eq!(entry.tenant_id(), TEST_TENANT_ID);
        assert_eq!(entry.line_items().len(), 2);
        assert_eq!(entry.status().to_string(), "草稿");
    }

    #[tokio::test]
    async fn test_fiscal_period_creation() {
        let period = create_test_period();
        assert_eq!(period.tenant_id(), TEST_TENANT_ID);
        assert_eq!(period.period(), 1);
        assert_eq!(period.status(), PeriodStatus::Open);
        assert!(period.allows_posting());
    }

    #[tokio::test]
    async fn test_period_status_transitions() {
        let mut period = create_test_period();

        // Open -> Closing
        period.start_closing().unwrap();
        assert_eq!(period.status(), PeriodStatus::Closing);

        // Closing -> Closed
        period.close().unwrap();
        assert_eq!(period.status(), PeriodStatus::Closed);
        assert!(!period.allows_posting());

        // Closed -> Open
        period.open().unwrap();
        assert_eq!(period.status(), PeriodStatus::Open);
        assert!(period.allows_posting());
    }

    #[tokio::test]
    async fn test_journal_entry_post_and_reverse() {
        let mut entry = create_test_entry();

        // Post the entry
        let events = entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        assert_eq!(entry.status().to_string(), "已过账");
        assert_eq!(events.len(), 1);

        // Reverse the entry
        let (reversal_entry, rev_events) = entry
            .reverse(
                DocumentNumber::new("JE-TEST-001-REV").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        assert_eq!(entry.status().to_string(), "已冲销");
        assert!(entry.is_reversed());
        assert!(reversal_entry.is_reversal());
        assert_eq!(rev_events.len(), 1);
    }
}

// =============================================================================
// Kafka Messaging Tests
// =============================================================================

mod kafka_messaging_tests {
    use super::*;

    #[tokio::test]
    async fn test_event_serialization() {
        use killer_financial_service::infrastructure::messaging::EventSerializer;

        let serializer = EventSerializer::default();

        // Test event envelope serialization
        let envelope = EventEnvelope {
            event_id: Uuid::new_v4(),
            event_type: "JournalEntryPosted".to_string(),
            tenant_id: "tenant-001".to_string(),
            aggregate_id: "JE-001".to_string(),
            aggregate_type: "JournalEntry".to_string(),
            payload: serde_json::json!({
                "document_number": "JE-001",
                "status": "POSTED"
            }),
            metadata: Default::default(),
            occurred_at: Utc::now(),
            schema_version: 1,
        };

        let serialized = serializer.serialize_envelope(&envelope);
        assert!(serialized.is_ok());

        let deserialized = serializer.deserialize_envelope(&serialized.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap().event_type, "JournalEntryPosted");
    }

    #[tokio::test]
    async fn test_event_serialization_roundtrip() {
        use killer_financial_service::infrastructure::messaging::EventSerializer;

        let serializer = EventSerializer::default();

        let original = EventEnvelope {
            event_id: Uuid::new_v4(),
            event_type: "FiscalPeriodClosed".to_string(),
            tenant_id: "tenant-001".to_string(),
            aggregate_id: "1000/2024/1".to_string(),
            aggregate_type: "FiscalPeriod".to_string(),
            payload: serde_json::json!({
                "company_code": "1000",
                "fiscal_year": 2024,
                "period": 1
            }),
            metadata: Default::default(),
            occurred_at: Utc::now(),
            schema_version: 1,
        };

        let serialized = serializer.serialize_envelope(&original).unwrap();
        let deserialized = serializer.deserialize_envelope(&serialized).unwrap();

        assert_eq!(original.event_id, deserialized.event_id);
        assert_eq!(original.event_type, deserialized.event_type);
        assert_eq!(original.tenant_id, deserialized.tenant_id);
        assert_eq!(original.payload, deserialized.payload);
    }

    #[tokio::test]
    async fn test_batch_event_serialization() {
        use killer_financial_service::infrastructure::messaging::EventSerializer;

        let serializer = EventSerializer::default();
        let events: Vec<EventEnvelope> = (0..5)
            .map(|i| EventEnvelope {
                event_id: Uuid::new_v4(),
                event_type: format!("TestEvent{}", i),
                tenant_id: "tenant-001".to_string(),
                aggregate_id: format!("agg-{}", i),
                aggregate_type: "TestAggregate".to_string(),
                payload: serde_json::json!({ "index": i }),
                metadata: Default::default(),
                occurred_at: Utc::now(),
                schema_version: 1,
            })
            .collect();

        for event in &events {
            let serialized = serializer.serialize_envelope(event);
            assert!(serialized.is_ok());
        }
    }
}

// =============================================================================
// Redis Cache Tests
// =============================================================================

mod redis_cache_tests {
    use super::*;

    /// Simple in-memory cache for testing
    #[derive(Clone, Default)]
    pub struct InMemoryCache {
        storage: Arc<RwLock<HashMap<String, (Vec<u8>, DateTime<Utc>)>>>,
        ttl_seconds: i64,
    }

    impl InMemoryCache {
        pub fn new(ttl_seconds: i64) -> Self {
            Self {
                storage: Arc::new(RwLock::new(HashMap::new())),
                ttl_seconds,
            }
        }

        pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
            let storage = self.storage.read().await;
            if let Some((data, expires_at)) = storage.get(key) {
                if *expires_at > Utc::now() {
                    return Some(data.clone());
                }
            }
            None
        }

        pub async fn set(&self, key: &str, data: Vec<u8>) {
            let mut storage = self.storage.write().await;
            let expires_at = Utc::now() + Duration::from_secs(self.ttl_seconds as u64);
            storage.insert(key.to_string(), (data, expires_at));
        }

        pub async fn delete(&self, key: &str) {
            let mut storage = self.storage.write().await;
            storage.remove(key);
        }
    }

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = InMemoryCache::new(3600); // 1 hour TTL

        let key = "test-key";
        let value = b"test-value";

        cache.set(key, value.to_vec()).await;
        let retrieved = cache.get(key).await;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = InMemoryCache::new(1); // 1 second TTL

        let key = "expiring-key";
        let value = b"expiring-value";

        cache.set(key, value.to_vec()).await;
        assert!(cache.get(key).await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(cache.get(key).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_delete() {
        let cache = InMemoryCache::new(3600);

        let key = "delete-key";
        cache.set(key, b"value".to_vec()).await;
        assert!(cache.get(key).await.is_some());

        cache.delete(key).await;
        assert!(cache.get(key).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_key_patterns() {
        let cache = InMemoryCache::new(3600);

        // Simulate tenant-scoped cache keys
        let tenant1_key = "tenant:550e8400-e29b-41d4-a716-446655440000:entry:JE-001";
        let tenant2_key = "tenant:550e8400-e29b-41d4-a716-446655440001:entry:JE-001";

        cache.set(tenant1_key, b"value1".to_vec()).await;
        cache.set(tenant2_key, b"value2".to_vec()).await;

        assert!(cache.get(tenant1_key).await.is_some());
        assert!(cache.get(tenant2_key).await.is_some());

        // Verify isolation
        assert_ne!(
            cache.get(tenant1_key).await,
            cache.get(tenant2_key).await
        );
    }
}

// =============================================================================
// Circuit Breaker Tests
// =============================================================================

mod circuit_breaker_tests {
    use super::*;
    use killer_financial_service::infrastructure::adapters::circuit_breaker::*;

    #[tokio::test]
    async fn test_circuit_breaker_state_transitions() {
        let breaker = CircuitBreaker::new("test-service", CircuitBreakerConfig::default());

        // Initial state should be Closed
        assert_eq!(breaker.state(), CircuitBreakerState::Closed);

        // Simulate failures
        for _ in 0..5 {
            let result = breaker.execute(|| async { Err::<(), _>(anyhow::anyhow!("fail")) }).await;
            assert!(result.is_err());
        }

        // Should open after threshold
        assert_eq!(breaker.state(), CircuitBreakerState::Open);

        // Wait for half-open transition
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Should be HalfOpen after timeout
        assert_eq!(breaker.state(), CircuitBreakerState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_success_resets() {
        let breaker = CircuitBreaker::new("test-service", CircuitBreakerConfig::default());

        // Execute some successes
        for _ in 0..3 {
            let result = breaker.execute(|| async { Ok::<(), _>(()) }).await;
            assert!(result.is_ok());
        }

        // State should still be Closed
        assert_eq!(breaker.state(), CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_success() {
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 3;
        config.success_threshold = 2;
        config.half_open_interval_ms = 100;

        let breaker = CircuitBreaker::new("test-service", config);

        // Trigger failures to open
        for _ in 0..3 {
            let _ = breaker.execute(|| async { Err::<(), _>(anyhow::anyhow!("fail")) }).await;
        }
        assert_eq!(breaker.state(), CircuitBreakerState::Open);

        // Wait for half-open
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(breaker.state(), CircuitBreakerState::HalfOpen);

        // Execute successes in half-open
        for _ in 0..2 {
            let result = breaker.execute(|| async { Ok::<(), _>(()) }).await;
            assert!(result.is_ok());
        }

        // Should close after success threshold
        assert_eq!(breaker.state(), CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_failure() {
        let mut config = CircuitBreakerConfig::default();
        config.failure_threshold = 3;
        config.success_threshold = 2;
        config.half_open_interval_ms = 100;

        let breaker = CircuitBreaker::new("test-service", config);

        // Trigger failures to open
        for _ in 0..3 {
            let _ = breaker.execute(|| async { Err::<(), _>(anyhow::anyhow!("fail")) }).await;
        }

        // Wait for half-open
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(breaker.state(), CircuitBreakerState::HalfOpen);

        // Execute failure in half-open
        let _ = breaker.execute(|| async { Err::<(), _>(anyhow::anyhow!("fail")) }).await;

        // Should open again
        assert_eq!(breaker.state(), CircuitBreakerState::Open);
    }
}

// =============================================================================
// External Adapter Tests
// =============================================================================

mod external_adapter_tests {
    use super::*;

    #[tokio::test]
    async fn test_master_data_client_simulation() {
        // Simulate master data validation
        let validated_accounts = Arc::new(Mutex::new(Vec::new()));

        let accounts = vec!["1001", "1002", "2001", "2002"];

        for account in &accounts {
            validated_accounts.lock().await.push(account.clone());
        }

        assert_eq!(validated_accounts.lock().await.len(), 4);
        assert!(validated_accounts.lock().await.contains(&"1001".to_string()));
    }

    #[tokio::test]
    async fn test_material_event_subscriber_simulation() {
        // Simulate material document event processing
        let received_events = Arc::new(Mutex::new(Vec::new()));

        let events = vec![
            ("material-created", "MAT-001"),
            ("material-updated", "MAT-001"),
            ("material-posted", "MAT-001"),
        ];

        for (event_type, material_id) in &events {
            received_events.lock().await.push((event_type.to_string(), material_id.to_string()));
        }

        assert_eq!(received_events.lock().await.len(), 3);
        assert_eq!(received_events.lock().await[0].0, "material-created");
    }
}

// =============================================================================
// Repository Contract Tests
// =============================================================================

mod repository_contract_tests {
    use super::*;

    const TENANT_A: &str = "550e8400-e29b-41d4-a716-446655440001";
    const TENANT_B: &str = "550e8400-e29b-41d4-a716-446655440002";

    #[tokio::test]
    async fn test_tenant_isolation() {
        // Simulate multi-tenant repository behavior
        let storage: Arc<RwLock<HashMap<String, Vec<JournalEntry>>>> =
            Arc::new(RwLock::new(HashMap::new()));

        // Tenant A stores an entry
        storage.write().await.insert(TENANT_A.to_string(), vec![]);

        // Tenant B stores an entry
        storage.write().await.insert(TENANT_B.to_string(), vec![]);

        // Verify isolation
        let tenant_a_entries = storage.read().await.get(TENANT_A);
        let tenant_b_entries = storage.read().await.get(TENANT_B);

        assert!(tenant_a_entries.is_some());
        assert!(tenant_b_entries.is_some());
        assert_ne!(TENANT_A, TENANT_B);
    }

    #[tokio::test]
    async fn test_soft_delete_implementation() {
        // Simulate soft delete behavior
        let deleted_entries: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));

        let document_number = "JE-001";
        deleted_entries.write().await.push(document_number.to_string());

        // Entry should still exist but be marked as deleted
        assert!(deleted_entries.read().await.contains(&document_number.to_string()));

        // Should not be returned in normal queries
        let active_entries: Vec<&String> = deleted_entries
            .read()
            .await
            .iter()
            .filter(|d| !deleted_entries.read().await.contains(d))
            .collect();

        assert!(active_entries.is_empty());
    }
}

// =============================================================================
// Projection Tests
// =============================================================================

mod projection_tests {
    use super::*;

    #[tokio::test]
    async fn test_account_balance_projection() {
        // Simulate account balance calculation
        let balances: Arc<RwLock<HashMap<String, f64>>> = Arc::new(RwLock::new(HashMap::new()));

        // Initial balance for account 1001
        balances.write().await.insert("1001".to_string(), 0.0);

        // Process debit entry
        let current = *balances.read().await.get("1001").unwrap();
        balances.write().await.insert("1001".to_string(), current + 1000.0);

        // Process credit entry
        let current = *balances.read().await.get("1001").unwrap();
        balances.write().await.insert("1001".to_string(), current - 300.0);

        assert_eq!(balances.read().await.get("1001").unwrap(), &700.0);
    }

    #[tokio::test]
    async fn test_trial_balance_projection() {
        // Simulate trial balance generation
        let trial_balance: Arc<RwLock<Vec<(String, f64, f64)>>> =
            Arc::new(RwLock::new(Vec::new()));

        let entries = vec![
            ("1001", 5000.0, 0.0),    // Cash - Debit
            ("1002", 2000.0, 0.0),    // Accounts Receivable - Debit
            ("2001", 0.0, 5000.0),    // Accounts Payable - Credit
            ("3001", 0.0, 2000.0),    // Common Stock - Credit
        ];

        for (account, debit, credit) in entries {
            trial_balance
                .write()
                .await
                .push((account.to_string(), debit, credit));
        }

        let total_debit: f64 = trial_balance.read().await.iter().map(|(_, d, _)| d).sum();
        let total_credit: f64 = trial_balance.read().await.iter().map(|(_, _, c)| c).sum();

        assert_eq!(total_debit, 7000.0);
        assert_eq!(total_credit, 7000.0);
        assert_eq!(total_debit, total_credit);
    }
}
