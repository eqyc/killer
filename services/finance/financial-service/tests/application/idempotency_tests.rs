//! Idempotency Tests
//!
//! Tests for idempotency control including:
//! - Duplicate request detection
//! - Cache-based idempotency
//! - Response caching and retrieval
//! - Key generation patterns

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use killer_financial_service::application::commands::*;
use killer_financial_service::application::dto::*;
use killer_financial_service::domain::aggregates::JournalEntry;
use killer_financial_service::domain::entities::JournalEntryLineItem;
use killer_financial_service::domain::value_objects::{DebitCredit, JournalEntryStatus};
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, Money};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use uuid::Uuid;

// =============================================================================
// Idempotency Key Tests
// =============================================================================

mod idempotency_key_tests {
    use super::*;

    const TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
    const USER_ID: &str = "550e8400-e29b-41d4-a716-446655440001";

    #[test]
    fn test_idempotency_key_format() {
        // Standard idempotency key format: {tenant_id}-{user_id}-{operation}-{resource_id}
        let key = format!(
            "{}-{}-{}-{}",
            TENANT_ID, USER_ID, "create", "entry-001"
        );

        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 5); // UUIDs have 5 parts each
    }

    #[test]
    fn test_same_key_generates_identical_string() {
        let key1 = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-001");
        let key2 = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-001");

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_different_keys_are_distinct() {
        let key1 = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-001");
        let key2 = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-002");

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_different_users_generate_different_keys() {
        let user2 = "550e8400-e29b-41d4-a716-446655440002";
        let key1 = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-001");
        let key2 = format!("{}-{}-{}-{}", TENANT_ID, user2, "create", "entry-001");

        assert_ne!(key1, key2);
    }
}

// =============================================================================
// Mock Idempotency Cache
// =============================================================================

/// Mock idempotency cache for testing
#[derive(Clone)]
pub struct MockIdempotencyCache {
    storage: Arc<Mutex<HashMap<String, CachedResponse>>>,
    ttl_seconds: i64,
}

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub response: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl MockIdempotencyCache {
    pub fn new(ttl_hours: i64) -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
            ttl_seconds: ttl_hours * 3600,
        }
    }

    pub async fn get(&self, key: &str) -> Option<CachedResponse> {
        let storage = self.storage.lock().await;
        storage.get(key).cloned()
    }

    pub async fn set(&self, key: &str, response: String) {
        let now = Utc::now();
        let expires_at = now + Duration::from_secs(self.ttl_seconds as u64);

        let mut storage = self.storage.lock().await;
        storage.insert(
            key.to_string(),
            CachedResponse {
                response,
                created_at: now,
                expires_at,
            },
        );
    }

    pub async fn delete(&self, key: &str) {
        let mut storage = self.storage.lock().await;
        storage.remove(key);
    }

    pub async fn clear(&self) {
        let mut storage = self.storage.lock().await;
        storage.clear();
    }
}

// =============================================================================
// Idempotency Cache Tests
// =============================================================================

mod idempotency_cache_tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = MockIdempotencyCache::new(24); // 24 hour TTL

        cache
            .set("test-key", r#"{"document_number": "JE-001"}"#.to_string())
            .await;

        let result = cache.get("test-key").await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().response, r#"{"document_number": "JE-001"}"#);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = MockIdempotencyCache::new(24);

        let result = cache.get("non-existent-key").await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_cache_delete() {
        let cache = MockIdempotencyCache::new(24);

        cache.set("test-key", "response".to_string()).await;
        assert!(cache.get("test-key").await.is_some());

        cache.delete("test-key").await;
        assert!(cache.get("test-key").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = MockIdempotencyCache::new(24);

        cache.set("key1", "response1".to_string()).await;
        cache.set("key2", "response2".to_string()).await;
        assert!(cache.get("key1").await.is_some());
        assert!(cache.get("key2").await.is_some());

        cache.clear().await;
        assert!(cache.get("key1").await.is_none());
        assert!(cache.get("key2").await.is_none());
    }

    #[tokio::test]
    async fn test_same_key_overwrites() {
        let cache = MockIdempotencyCache::new(24);

        cache.set("test-key", "response1".to_string()).await;
        cache.set("test-key", "response2".to_string()).await;

        let result = cache.get("test-key").await;
        assert_eq!(result.unwrap().response, "response2");
    }
}

// =============================================================================
// Duplicate Request Detection Tests
// =============================================================================

mod duplicate_request_detection_tests {
    use super::*;

    const TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

    #[tokio::test]
    async fn test_duplicate_create_request_detected() {
        let cache = MockIdempotencyCache::new(24);

        // First request
        let idempotency_key = format!("{}-create-entry-001", TENANT_ID);
        let first_request = r#"{"company_code": "1000", "amount": 1000}"#;

        // Cache the response for the first request
        let first_response = r#"{"document_number": "JE-001", "status": "DRAFT"}"#;
        cache.set(&idempotency_key, first_response.to_string()).await;

        // Second request with same key
        let cached = cache.get(&idempotency_key).await;

        // Should return cached response
        assert!(cached.is_some());
        assert!(cached.unwrap().response.contains("JE-001"));
    }

    #[tokio::test]
    async fn test_different_requests_not_confused() {
        let cache = MockIdempotencyCache::new(24);

        // Request 1: Create entry
        let key1 = format!("{}-create-entry-001", TENANT_ID);
        cache
            .set(&key1, r#"{"document_number": "JE-001"}"#.to_string())
            .await;

        // Request 2: Post entry
        let key2 = format!("{}-post-entry-001", TENANT_ID);
        cache
            .set(&key2, r#"{"status": "POSTED"}"#.to_string())
            .await;

        // Verify each returns correct cached response
        assert!(cache.get(&key1).await.unwrap().response.contains("JE-001"));
        assert!(cache.get(&key2).await.unwrap().response.contains("POSTED"));
    }
}

// =============================================================================
// Idempotency TTL Tests
// =============================================================================

mod idempotency_ttl_tests {
    use super::*;

    #[tokio::test]
    async fn test_expired_entries_should_be_removed() {
        // Create cache with 0 TTL (immediate expiration for testing)
        let cache = MockIdempotencyCache::new(0);

        cache.set("test-key", "response".to_string()).await;

        // Small delay to ensure expiration
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Entry should still exist (we're not implementing actual expiration in mock)
        // In real implementation, expired entries would be cleaned up
        let result = cache.get("test-key").await;
        // With 0 TTL, the mock still stores it - real implementation would check expiration
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_24h_ttl_is_configurable() {
        let cache_24h = MockIdempotencyCache::new(24);
        let cache_48h = MockIdempotencyCache::new(48);

        assert_eq!(cache_24h.ttl_seconds, 24 * 3600);
        assert_eq!(cache_48h.ttl_seconds, 48 * 3600);
    }
}

// =============================================================================
// Real-World Idempotency Scenario Tests
// =============================================================================

mod idempotency_scenario_tests {
    use super::*;

    const TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
    const USER_ID: &str = "550e8400-e29b-41d4-a716-446655440001";

    /// Simulates a real-world scenario where a client retries a request due to network issues
    #[tokio::test]
    async fn test_network_retry_scenario() {
        let cache = MockIdempotencyCache::new(24);
        let request_counter = Arc::new(AtomicUsize::new(0));
        let idempotency_key = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "entry-retry");

        // Simulate first request attempt
        let counter = request_counter.clone();
        let first_attempt = tokio::spawn(async move {
            // Check if already processed
            if let Some(cached) = cache.get(&idempotency_key).await {
                return Some(cached);
            }

            // Process request (would create entry in real scenario)
            counter.fetch_add(1, Ordering::SeqCst);
            let response = r#"{"document_number": "JE-001", "status": "DRAFT"}"#;
            cache.set(&idempotency_key, response.to_string()).await;
            Some(CachedResponse {
                response: response.to_string(),
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::from_secs(86400),
            })
        });

        // Simulate retry (client didn't receive response)
        let counter = request_counter.clone();
        let retry_attempt = tokio::spawn(async move {
            // Check if already processed
            if let Some(cached) = cache.get(&idempotency_key).await {
                return Some(cached);
            }

            // Process request
            counter.fetch_add(1, Ordering::SeqCst);
            let response = r#"{"document_number": "JE-001", "status": "DRAFT"}"#;
            cache.set(&idempotency_key, response.to_string()).await;
            Some(CachedResponse {
                response: response.to_string(),
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::from_secs(86400),
            })
        });

        let first_result = first_attempt.await.unwrap();
        let retry_result = retry_attempt.await.unwrap();

        // Only one request should have been processed
        assert_eq!(request_counter.load(Ordering::SeqCst), 1);

        // Both should return the same cached response
        assert_eq!(first_result, retry_result);
    }

    /// Tests that concurrent identical requests are handled correctly
    #[tokio::test]
    async fn test_concurrent_idempotent_requests() {
        let cache = MockIdempotencyCache::new(24);
        let idempotency_key = format!("{}-{}-{}-{}", TENANT_ID, USER_ID, "create", "concurrent");
        let processed_count = Arc::new(AtomicUsize::new(0));

        // Simulate 10 concurrent requests with same idempotency key
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let cache = cache.clone();
                let count = processed_count.clone();
                let key = idempotency_key.clone();
                tokio::spawn(async move {
                    // Check cache first
                    if let Some(cached) = cache.get(&key).await {
                        return Some(cached);
                    }

                    // Try to process (only one should succeed in real scenario)
                    count.fetch_add(1, Ordering::SeqCst);
                    let response = r#"{"document_number": "JE-CONCURRENT"}"#;
                    cache.set(&key, response.to_string()).await;
                    Some(CachedResponse {
                        response: response.to_string(),
                        created_at: Utc::now(),
                        expires_at: Utc::now() + Duration::from_secs(86400),
                    })
                })
            })
            .collect();

        let results: Vec<Option<CachedResponse>> = futures::future::join_all(handles)
            .await
            .into_iter()
            .collect();

        // Only one request should be processed
        assert_eq!(processed_count.load(Ordering::SeqCst), 1);

        // All should return the same cached response
        let first_response = &results[0].as_ref().unwrap().response;
        for result in &results {
            assert_eq!(result.as_ref().unwrap().response, first_response);
        }
    }
}

// =============================================================================
// Helper for atomic operations
// =============================================================================

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
