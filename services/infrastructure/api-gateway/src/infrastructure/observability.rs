//! 可观测性模块
//!
//! 提供追踪、指标、审计日志功能

use crate::domain::{AuditLogEntry, GatewayMetrics, RequestContext, RequestInfo};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::{AtomicU64, Ordering}, Arc},
    time::Duration,
};
use tokio::sync::RwLock;
use tracing::{span, Level};

/// 简化的指标注册表
#[derive(Debug, Clone, Default)]
pub struct MetricsRegistry {
    /// 请求计数器
    requests_total: AtomicU64,
    /// 成功请求计数器
    requests_success: AtomicU64,
    /// 客户端错误计数器
    requests_client_error: AtomicU64,
    /// 服务端错误计数器
    requests_server_error: AtomicU64,
    /// 限流拒绝计数器
    rate_limited_total: AtomicU64,
    /// 熔断器拒绝计数器
    circuit_breaker_rejected: AtomicU64,
    /// 认证失败计数器
    auth_failed: AtomicU64,
    /// 授权失败计数器
    authz_failed: AtomicU64,
    /// 活跃请求数
    active_requests: AtomicU64,
    /// 熔断器状态
    circuit_breaker_state: AtomicU64,
}

impl MetricsRegistry {
    /// 创建指标注册表
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录请求
    pub fn record_request(&self, path: &str, method: &str, status: u16, duration: Duration) {
        self.requests_total.fetch_add(1, Ordering::SeqCst);

        if status < 400 {
            self.requests_success.fetch_add(1, Ordering::SeqCst);
        } else if status < 500 {
            self.requests_client_error.fetch_add(1, Ordering::SeqCst);
        } else {
            self.requests_server_error.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// 记录限流
    pub fn record_rate_limited(&self, _limit_name: &str) {
        self.rate_limited_total.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录熔断器拒绝
    pub fn record_circuit_breaker_rejected(&self, _circuit: &str) {
        self.circuit_breaker_rejected.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录认证失败
    pub fn record_auth_failed(&self, _reason: &str) {
        self.auth_failed.fetch_add(1, Ordering::SeqCst);
    }

    /// 记录授权失败
    pub fn record_authz_failed(&self, _permission: &str) {
        self.authz_failed.fetch_add(1, Ordering::SeqCst);
    }

    /// 更新活跃请求数
    pub fn set_active_requests(&self, count: usize) {
        self.active_requests.store(count as u64, Ordering::SeqCst);
    }

    /// 更新熔断器状态
    pub fn set_circuit_breaker_state(&self, _circuit: &str, state: u8) {
        self.circuit_breaker_state.store(state as u64, Ordering::SeqCst);
    }

    /// 获取指标快照
    pub fn snapshot(&self) -> GatewayMetrics {
        GatewayMetrics {
            total_requests: self.requests_total.load(Ordering::SeqCst),
            successful_requests: self.requests_success.load(Ordering::SeqCst),
            failed_requests: self.requests_client_error.load(Ordering::SeqCst) + self.requests_server_error.load(Ordering::SeqCst),
            client_errors: self.requests_client_error.load(Ordering::SeqCst),
            server_errors: self.requests_server_error.load(Ordering::SeqCst),
            total_latency_ms: 0,
            avg_latency_ms: 0,
            p99_latency_ms: 0,
            rate_limited_count: self.rate_limited_total.load(Ordering::SeqCst),
            circuit_breaker_rejected_count: self.circuit_breaker_rejected.load(Ordering::SeqCst),
            auth_failed_count: self.auth_failed.load(Ordering::SeqCst),
            authz_failed_count: self.authz_failed.load(Ordering::SeqCst),
        }
    }
}

/// 审计日志服务
#[derive(Debug, Clone)]
pub struct AuditLogService {
    /// 审计日志存储
    storage: Arc<dyn AuditStorage>,
    /// 敏感字段
    sensitive_fields: Vec<String>,
    /// 排除的路径
    excluded_paths: Vec<String>,
    /// 审计日志队列
    queue: Arc<RwLock<Vec<AuditLogEntry>>>,
}

impl AuditLogService {
    /// 创建审计日志服务
    pub fn new(storage: Arc<dyn AuditStorage>, sensitive_fields: Vec<String>, excluded_paths: Vec<String>) -> Self {
        Self {
            storage,
            sensitive_fields,
            excluded_paths,
            queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 记录审计日志
    pub async fn log(&self, entry: AuditLogEntry) {
        // 添加到队列
        let mut queue = self.queue.write().await;
        queue.push(entry);

        // 如果队列太长，处理日志
        if queue.len() > 1000 {
            self.flush().await;
        }
    }

    /// 刷新日志队列
    pub async fn flush(&self) {
        let mut queue = self.queue.write().await;
        if queue.is_empty() {
            return;
        }

        let entries = std::mem::replace(&mut *queue, Vec::new());
        for entry in entries {
            let masked_entry = self.mask_sensitive_fields(entry);
            if !self.should_exclude(&masked_entry) {
                if let Err(e) = self.storage.store(&masked_entry).await {
                    tracing::error!("Failed to store audit log: {}", e);
                }
            }
        }
    }

    /// 脱敏敏感字段
    fn mask_sensitive_fields(&self, mut entry: AuditLogEntry) -> AuditLogEntry {
        for field in &self.sensitive_fields {
            if let Some(value) = entry.metadata.get(field) {
                entry.metadata.insert(field.clone(), format!("***REDACTED-{}***", field));
            }
        }
        entry
    }

    /// 检查是否应该排除
    fn should_exclude(&self, entry: &AuditLogEntry) -> bool {
        self.excluded_paths.iter().any(|p| entry.path.starts_with(p))
    }
}

/// 审计存储 trait
#[async_trait]
pub trait AuditStorage: Send + Sync {
    async fn store(&self, entry: &AuditLogEntry) -> Result<(), anyhow::Error>;
}

/// 内存审计存储
#[derive(Debug, Clone, Default)]
pub struct MemoryAuditStorage {
    entries: Arc<RwLock<Vec<AuditLogEntry>>>,
}

impl MemoryAuditStorage {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl AuditStorage for MemoryAuditStorage {
    async fn store(&self, entry: &AuditLogEntry) -> Result<(), anyhow::Error> {
        let mut entries = self.entries.write().await;
        entries.push(entry.clone());
        Ok(())
    }
}

/// 追踪 Span 信息
#[derive(Debug, Clone)]
pub struct SpanInfo {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub operation_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub attributes: HashMap<String, String>,
    pub events: Vec<SpanEvent>,
    pub status: SpanStatus,
}

/// 追踪事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub attributes: HashMap<String, String>,
}

/// 追踪状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpanStatus {
    Unset,
    Ok,
    Error,
}

/// 简化的追踪服务
#[derive(Debug, Clone, Default)]
pub struct TracingService;

impl TracingService {
    /// 创建新的追踪服务
    pub fn new() -> Self {
        Self
    }

    /// 提取追踪上下文
    pub fn extract_context(&self, _headers: &HashMap<String, String>) -> Option<(String, String)> {
        None
    }

    /// 注入追踪上下文
    pub fn inject_context(&self, _trace_id: &str, _span_id: &str, _headers: &mut HashMap<String, String>) {
        // 空实现
    }

    /// 创建 Span
    pub fn start_span(&self, name: &str, _attrs: HashMap<String, String>) -> SpanInfo {
        SpanInfo {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: format!("{:x}", rand::random::<u64>()),
            parent_span_id: None,
            operation_name: name.to_string(),
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            status: SpanStatus::Unset,
        }
    }

    /// 结束 Span
    pub fn end_span(&mut self, span: &mut SpanInfo) {
        span.end_time = Some(Utc::now());
    }

    /// 记录 Span 事件
    pub fn add_event(&mut self, span: &mut SpanInfo, name: &str, attrs: HashMap<String, String>) {
        span.events.push(SpanEvent {
            timestamp: Utc::now(),
            name: name.to_string(),
            attributes: attrs,
        });
    }

    /// 设置 Span 状态
    pub fn set_status(&mut self, span: &mut SpanInfo, status: SpanStatus) {
        span.status = status;
    }
}

/// 初始化追踪
pub fn init_tracing(_service_name: &str, _otlp_endpoint: Option<&str>) {
    // 简化的追踪初始化
    tracing_subscriber::fmt::init();
}
