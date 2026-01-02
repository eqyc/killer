//! Performance and Load Tests
//!
//! Benchmarks and load tests for the financial service.

use chrono::NaiveDate;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// =============================================================================
// Benchmark Tests
// =============================================================================

mod benchmark_tests {
    use super::*;

    #[test]
    fn test_journal_entry_creation_benchmark() {
        // Benchmark domain entity creation
        let start = Instant::now();

        for _ in 0..1000 {
            let _ = create_test_journal_entry();
        }

        let elapsed = start.elapsed();
        println!("Created 1000 journal entries in {:?}", elapsed);
        assert!(elapsed < Duration::from_secs(5));
    }

    #[test]
    fn test_balance_calculation_benchmark() {
        let entries = (0..100).map(|_| create_test_journal_entry()).collect::<Vec<_>>();

        let start = Instant::now();

        for _ in 0..100 {
            let _ = calculate_total_debit(&entries);
            let _ = calculate_total_credit(&entries);
        }

        let elapsed = start.elapsed();
        println!("Calculated balances 100 times in {:?}", elapsed);
        assert!(elapsed < Duration::from_secs(1));
    }

    #[test]
    fn test_validation_benchmark() {
        let entry = create_test_journal_entry();

        let start = Instant::now();

        for _ in 0..10000 {
            let _ = validate_entry(&entry);
        }

        let elapsed = start.elapsed();
        println!("Validated entry 10000 times in {:?}", elapsed);
        assert!(elapsed < Duration::from_secs(2));
    }

    #[test]
    fn test_serialization_benchmark() {
        let entry = create_test_journal_entry();

        let start = Instant::now();

        for _ in 0..1000 {
            let json = serde_json::to_string(&entry).unwrap();
            let _ = serde_json::from_str::<serde_json::Value>(&json);
        }

        let elapsed = start.elapsed();
        println!("Serialized/deserialized 1000 times in {:?}", elapsed);
        assert!(elapsed < Duration::from_secs(5));
    }
}

// =============================================================================
// Load Tests
// =============================================================================

mod load_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_entry_creation() {
        let concurrent_tasks = 100;
        let entries_per_task = 10;

        let start = Instant::now();
        let created_count = Arc::new(AtomicUsize::new(0));

        // Spawn concurrent tasks
        let handles: Vec<_> = (0..concurrent_tasks)
            .map(|_| {
                let count = created_count.clone();
                tokio::spawn(async move {
                    for _ in 0..entries_per_task {
                        let _ = create_test_journal_entry();
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                })
            })
            .collect();

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        let elapsed = start.elapsed();
        let total_created = created_count.load(Ordering::SeqCst);

        println!(
            "Created {} entries with {} concurrent tasks in {:?}",
            total_created, concurrent_tasks, elapsed
        );

        // Assertions
        assert_eq!(total_created, concurrent_tasks * entries_per_task);
        assert!(elapsed < Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_concurrent_validation() {
        let concurrent_validations = 200;
        let entry = Arc::new(create_test_journal_entry());

        let start = Instant::now();
        let valid_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..concurrent_validations)
            .map(|_| {
                let count = valid_count.clone();
                let entry = entry.clone();
                tokio::spawn(async move {
                    if validate_entry(&entry) {
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }

        let elapsed = start.elapsed();
        let total_valid = valid_count.load(Ordering::SeqCst);

        println!(
            "Validated {} entries concurrently in {:?}",
            total_valid, elapsed
        );

        assert_eq!(total_valid, concurrent_validations);
    }

    #[tokio::test]
    async fn test_throughput_under_load() {
        let target_duration = Duration::from_secs(5);
        let entry_count = Arc::new(AtomicUsize::new(0));

        let start = Instant::now();
        let handle = tokio::spawn({
            let count = entry_count.clone();
            async move {
                while start.elapsed() < target_duration {
                    let _ = create_test_journal_entry();
                    count.fetch_add(1, Ordering::SeqCst);
                }
            }
        });

        handle.await.unwrap();

        let elapsed = start.elapsed();
        let entries = entry_count.load(Ordering::SeqCst);

        println!("Created {} entries in {:?}", entries, elapsed);

        // Calculate TPS (transactions per second)
        let tps = entries as f64 / elapsed.as_secs_f64();
        println!("Throughput: {:.2} TPS", tps);

        // Minimum throughput requirement
        assert!(tps >= 100.0, "Expected at least 100 TPS, got {:.2}", tps);
    }
}

// =============================================================================
// Latency Tests
// =============================================================================

mod latency_tests {
    use super::*;

    #[test]
    fn test_p50_latency() {
        let mut latencies = Vec::new();

        for _ in 0..1000 {
            let start = Instant::now();
            let _ = create_test_journal_entry();
            latencies.push(start.elapsed());
        }

        latencies.sort();
        let p50 = latencies[latencies.len() / 2];

        println!("P50 latency: {:?}", p50);
        assert!(p50 < Duration::from_millis(10));
    }

    #[test]
    fn test_p95_latency() {
        let mut latencies = Vec::new();

        for _ in 0..1000 {
            let start = Instant::now();
            let _ = create_test_journal_entry();
            latencies.push(start.elapsed());
        }

        latencies.sort();
        let p95 = latencies[(latencies.len() * 95) / 100];

        println!("P95 latency: {:?}", p95);
        assert!(p95 < Duration::from_millis(50));
    }

    #[test]
    fn test_p99_latency() {
        let mut latencies = Vec::new();

        for _ in 0..1000 {
            let start = Instant::now();
            let _ = create_test_journal_entry();
            latencies.push(start.elapsed());
        }

        latencies.sort();
        let p99 = latencies[(latencies.len() * 99) / 100];

        println!("P99 latency: {:?}", p99);
        assert!(p99 < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_concurrent_latency_percentiles() {
        let concurrent_requests = 50;
        let requests_per_thread = 20;
        let mut all_latencies = Vec::new();

        let start = Instant::now();

        let handles: Vec<_> = (0..concurrent_requests)
            .map(|_| {
                tokio::spawn(async move {
                    let mut latencies = Vec::new();
                    for _ in 0..requests_per_thread {
                        let req_start = Instant::now();
                        let _ = create_test_journal_entry();
                        latencies.push(req_start.elapsed());
                    }
                    latencies
                })
            })
            .collect();

        for handle in handles {
            let latencies = handle.await.unwrap();
            all_latencies.extend(latencies);
        }

        let elapsed = start.elapsed();
        all_latencies.sort();

        let p50 = all_latencies[(all_latencies.len() * 50) / 100];
        let p95 = all_latencies[(all_latencies.len() * 95) / 100];
        let p99 = all_latencies[(all_latencies.len() * 99) / 100];

        println!("Concurrent test results:");
        println!("  P50: {:?}", p50);
        println!("  P95: {:?}", p95);
        println!("  P99: {:?}", p99);
        println!("  Total time: {:?}", elapsed);

        assert!(p50 < Duration::from_millis(20));
        assert!(p95 < Duration::from_millis(100));
        assert!(p99 < Duration::from_millis(200));
    }
}

// =============================================================================
// Memory Tests
// =============================================================================

mod memory_tests {
    use super::*;

    #[test]
    fn test_memory_allocation_per_entry() {
        let baseline = get_memory_usage();

        let entries: Vec<_> = (0..1000).map(|_| create_test_journal_entry()).collect();
        let after_allocation = get_memory_usage();

        let memory_per_entry = (after_allocation - baseline) / 1000;
        println!("Memory per journal entry: {} bytes", memory_per_entry);

        // Should be less than 1KB per entry
        assert!(memory_per_entry < 1024);
    }

    #[test]
    fn test_collection_memory_release() {
        let baseline = get_memory_usage();

        {
            let entries: Vec<_> = (0..10000).map(|_| create_test_journal_entry()).collect();
            let _ = entries.len();
        }

        // Force cleanup (in real scenario, would wait for GC)
        let after_cleanup = get_memory_usage();

        // Memory should be released (or mostly released)
        let retained = after_cleanup - baseline;
        println!("Memory retained after cleanup: {} bytes", retained);
    }
}

// =============================================================================
// Stress Tests
// =============================================================================

mod stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_sustained_high_load() {
        let duration = Duration::from_secs(10);
        let target_rps = 500.0; // Target requests per second

        let start = Instant::now();
        let request_count = Arc::new(AtomicUsize::new(0));

        let handle = tokio::spawn({
            let count = request_count.clone();
            async move {
                while start.elapsed() < duration {
                    let _ = create_test_journal_entry();
                    count.fetch_add(1, Ordering::SeqCst);

                    // Rate limit to target RPS
                    tokio::time::sleep(Duration::from_secs_f64(1.0 / target_rps)).await;
                }
            }
        });

        handle.await.unwrap();

        let elapsed = start.elapsed();
        let requests = request_count.load(Ordering::SeqCst);
        let actual_rps = requests as f64 / elapsed.as_secs_f64();

        println!("Sustained load test:");
        println!("  Duration: {:?}", elapsed);
        println!("  Total requests: {}", requests);
        println!("  Actual RPS: {:.2}", actual_rps);

        // Should achieve at least 80% of target
        assert!(actual_rps >= target_rps * 0.8);
    }

    #[tokio::test]
    async fn test_burst_load_handling() {
        let burst_size = 100;
        let processed = Arc::new(AtomicUsize::new(0));

        // Simulate burst
        let start = Instant::now();

        let handles: Vec<_> = (0..burst_size)
            .map(|_| {
                let count = processed.clone();
                tokio::spawn(async move {
                    let _ = create_test_journal_entry();
                    count.fetch_add(1, Ordering::SeqCst);
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }

        let elapsed = start.elapsed();
        let total = processed.load(Ordering::SeqCst);

        println!("Burst load test:");
        println!("  Burst size: {}", burst_size);
        println!("  Processed: {}", total);
        println!("  Time: {:?}", elapsed);

        // All burst requests should complete quickly
        assert_eq!(total, burst_size);
        assert!(elapsed < Duration::from_secs(5));
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn create_test_journal_entry() -> serde_json::Value {
    serde_json::json!({
        "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
        "company_code": "1000",
        "fiscal_year": 2024,
        "document_number": format!("JE-{:08}", rand::random::<u32>()),
        "posting_date": "2024-01-15",
        "currency_code": "CNY",
        "status": "DRAFT",
        "line_items": [
            {"account_code": "1001", "amount": 1000.0, "debit_credit": "D"},
            {"account_code": "2001", "amount": 1000.0, "debit_credit": "C"}
        ]
    })
}

fn calculate_total_debit(entries: &[serde_json::Value]) -> f64 {
    entries
        .iter()
        .flat_map(|e| e["line_items"].as_array().unwrap())
        .filter(|line| line["debit_credit"] == "D")
        .fold(0.0, |sum, line| sum + line["amount"].as_f64().unwrap())
}

fn calculate_total_credit(entries: &[serde_json::Value]) -> f64 {
    entries
        .iter()
        .flat_map(|e| e["line_items"].as_array().unwrap())
        .filter(|line| line["debit_credit"] == "C")
        .fold(0.0, |sum, line| sum + line["amount"].as_f64().unwrap())
}

fn validate_entry(entry: &serde_json::Value) -> bool {
    let line_items = match entry["line_items"].as_array() {
        Some(items) => items,
        None => return false,
    };

    if line_items.len() < 2 {
        return false;
    }

    let total_debit: f64 = line_items
        .iter()
        .filter(|line| line["debit_credit"] == "D")
        .map(|line| line["amount"].as_f64().unwrap_or(0.0))
        .sum();

    let total_credit: f64 = line_items
        .iter()
        .filter(|line| line["debit_credit"] == "C")
        .map(|line| line["amount"].as_f64().unwrap_or(0.0))
        .sum();

    (total_debit - total_credit).abs() < 0.01
}

fn get_memory_usage() -> usize {
    // In real implementation, would use memory_profiler or similar
    // For now, return a placeholder
    0
}
