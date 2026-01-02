//! 指标中间件
//!
//! 提供 Prometheus 指标暴露

use prometheus::{Counter, Gauge, Histogram, HistogramOpts, IntCounter, IntCounterVec, Registry};
use std::sync::Arc;

/// API 指标
#[derive(Clone)]
pub struct ApiMetrics {
    /// gRPC 请求总数
    grpc_requests_total: Arc<IntCounterVec>,

    /// gRPC 请求持续时间
    grpc_request_duration: Arc<Histogram>,

    /// HTTP 请求总数
    http_requests_total: Arc<IntCounterVec>,

    /// HTTP 请求持续时间
    http_request_duration: Arc<Histogram>,

    /// 活跃请求数
    active_requests: Arc<Gauge>,

    /// 错误总数
    errors_total: Arc<IntCounterVec>,

    /// 幂等性命中数
    idempotency_hits: Arc<IntCounter>,

    /// 认证失败数
    auth_failures: Arc<IntCounter>,
}

impl ApiMetrics {
    /// 创建新的指标
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let grpc_requests_total = IntCounterVec::new(
            prometheus::opts!("grpc_requests_total", "Total gRPC requests"),
            &["service", "method", "status", "tenant_id"],
        )?;

        let grpc_request_duration = Arc::new(Histogram::new(
            HistogramOpts::new(
                "grpc_request_duration_seconds",
                "gRPC request duration in seconds",
            )
            .buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]),
        )?);

        let http_requests_total = IntCounterVec::new(
            prometheus::opts!("http_requests_total", "Total HTTP requests"),
            &["method", "path", "status", "tenant_id"],
        )?;

        let http_request_duration = Arc::new(Histogram::new(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds",
            )
            .buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]),
        )?;

        let active_requests = Gauge::new(
            "active_requests",
            "Number of currently active requests",
        )?;

        let errors_total = IntCounterVec::new(
            prometheus::opts!("errors_total", "Total errors"),
            &["service", "method", "error_type"],
        )?;

        let idempotency_hits = IntCounter::new(
            "idempotency_hits_total",
            "Total number of idempotency key hits",
        )?;

        let auth_failures = IntCounter::new(
            "auth_failures_total",
            "Total number of authentication failures",
        )?;

        registry.register(Box::new(grpc_requests_total.clone()))?;
        registry.register(Box::new(grpc_request_duration.clone()))?;
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration.clone()))?;
        registry.register(Box::new(active_requests.clone()))?;
        registry.register(Box::new(errors_total.clone()))?;
        registry.register(Box::new(idempotency_hits.clone()))?;
        registry.register(Box::new(auth_failures.clone()))?;

        Ok(Self {
            grpc_requests_total: Arc::new(grpc_requests_total),
            grpc_request_duration,
            http_requests_total: Arc::new(http_requests_total),
            http_request_duration,
            active_requests,
            errors_total: Arc::new(errors_total),
            idempotency_hits,
            auth_failures,
        })
    }

    /// 记录 gRPC 请求
    pub fn record_grpc_request(
        &self,
        service: &str,
        method: &str,
        status: &str,
        tenant_id: &str,
        duration: std::time::Duration,
    ) {
        self.grpc_requests_total
            .with_label_values(&[service, method, status, tenant_id])
            .inc();
        self.grpc_request_duration.observe(duration.as_secs_f64());
    }

    /// 记录 HTTP 请求
    pub fn record_http_request(
        &self,
        method: &str,
        path: &str,
        status: u16,
        tenant_id: &str,
        duration: std::time::Duration,
    ) {
        let status_str = status.to_string();
        self.http_requests_total
            .with_label_values(&[method, path, &status_str, tenant_id])
            .inc();
        self.http_request_duration.observe(duration.as_secs_f64());
    }

    /// 增加活跃请求数
    pub fn inc_active_requests(&self) {
        self.active_requests.inc();
    }

    /// 减少活跃请求数
    pub fn dec_active_requests(&self) {
        self.active_requests.dec();
    }

    /// 记录错误
    pub fn record_error(&self, service: &str, method: &str, error_type: &str) {
        self.errors_total
            .with_label_values(&[service, method, error_type])
            .inc();
    }

    /// 记录幂等性命中
    pub fn record_idempotency_hit(&self) {
        self.idempotency_hits.inc();
    }

    /// 记录认证失败
    pub fn record_auth_failure(&self) {
        self.auth_failures.inc();
    }
}

/// 指标中间件 builder
#[derive(Default)]
pub struct MetricsMiddlewareBuilder {
    metrics: Option<Arc<ApiMetrics>>,
}

impl MetricsMiddlewareBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_metrics(mut self, metrics: Arc<ApiMetrics>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn build(self) -> MetricsMiddleware {
        MetricsMiddleware {
            metrics: self.metrics.unwrap_or_else(|| Arc::new(create_default_metrics())),
        }
    }
}

/// 指标中间件
#[derive(Clone)]
pub struct MetricsMiddleware {
    metrics: Arc<ApiMetrics>,
}

impl MetricsMiddleware {
    pub fn new(metrics: Arc<ApiMetrics>) -> Self {
        Self { metrics }
    }
}

/// 创建默认指标（当没有注册表时使用）
fn create_default_metrics() -> ApiMetrics {
    let registry = Registry::new();

    ApiMetrics::new(&registry).unwrap_or_else(|_| {
        // 如果注册失败，返回一个空的指标实例
        panic!("Failed to create default metrics")
    })
}
