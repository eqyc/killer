//! 审计追踪中间件
//!
//! 记录每请求的审计日志到数据库或 Kafka

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Status};
use tracing::{debug, error, info, span, Instrument, Level};
use uuid::Uuid;

/// 审计操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    /// 创建
    Create,
    /// 更新
    Update,
    /// 删除
    Delete,
    /// 过账
    Post,
    /// 冲销
    Reverse,
    /// 读取
    Read,
    /// 查询
    Query,
}

/// 审计状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditStatus {
    /// 成功
    Success,
    /// 失败
    Failure,
    /// 部分成功
    Partial,
}

/// 审计记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    /// 审计 ID
    pub id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 用户 ID
    pub user_id: Uuid,

    /// 操作类型
    pub action: AuditAction,

    /// 资源类型（如 "journal_entry"）
    pub resource_type: String,

    /// 资源 ID
    pub resource_id: String,

    /// 操作状态
    pub status: AuditStatus,

    /// 错误消息（如果失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// 请求跟踪 ID
    pub trace_id: Uuid,

    /// 关联 ID（用于关联相关操作）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,

    /// 客户端 IP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,

    /// 请求时间
    pub timestamp: DateTime<Utc>,

    /// 持续时间（毫秒）
    pub duration_ms: u64,

    /// 附加元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AuditRecord {
    /// 创建新的审计记录
    pub fn new(
        tenant_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        resource_type: &str,
        resource_id: &str,
        trace_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            user_id,
            action,
            resource_type: resource_type.to_string(),
            resource_id: resource_id.to_string(),
            status: AuditStatus::Success,
            error_message: None,
            trace_id,
            correlation_id: None,
            client_ip: None,
            timestamp: Utc::now(),
            duration_ms: 0,
            metadata: None,
        }
    }

    /// 设置状态
    pub fn with_status(mut self, status: AuditStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置错误消息
    pub fn with_error(mut self, error: &str) -> Self {
        self.status = AuditStatus::Failure;
        self.error_message = Some(error.to_string());
        self
    }

    /// 设置持续时间
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// 设置客户端 IP
    pub fn with_client_ip(mut self, ip: &str) -> Self {
        self.client_ip = Some(ip.to_string());
        self
    }

    /// 设置关联 ID
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// 设置元数据
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// 审计存储 trait - 支持多种存储后端
#[async_trait::async_trait]
pub trait AuditStorage: Send + Sync {
    /// 保存审计记录
    async fn save(&self, record: &AuditRecord) -> Result<(), anyhow::Error>;
}

/// 数据库审计存储
#[derive(Clone)]
pub struct DbAuditStorage {
    /// 发送通道
    tx: mpsc::Sender<AuditRecord>,
}

impl DbAuditStorage {
    pub fn new(tx: mpsc::Sender<AuditRecord>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl AuditStorage for DbAuditStorage {
    async fn save(&self, record: &AuditRecord) -> Result<(), anyhow::Error> {
        self.tx.send(record.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to send audit record: {}", e))
    }
}

/// Kafka 审计存储
#[derive(Clone)]
pub struct KafkaAuditStorage {
    /// Kafka 生产者（这里使用简化版本，实际应该使用 rdkafka）
    _producer: Arc<()>, // 占位，实际使用时替换为真正的 Kafka producer
    _topic: String,
}

impl KafkaAuditStorage {
    pub fn new(_topic: String) -> Self {
        Self {
            _producer: Arc::new(()),
            _topic,
        }
    }
}

#[async_trait::async_trait]
impl AuditStorage for KafkaAuditStorage {
    async fn save(&self, _record: &AuditRecord) -> Result<(), anyhow::Error> {
        // 实际实现应该发送消息到 Kafka
        Ok(())
    }
}

/// 审计中间件
#[derive(Clone)]
pub struct AuditMiddleware {
    /// 审计存储
    storage: Arc<dyn AuditStorage>,

    /// 启用审计的操作
    enabled_actions: Vec<AuditAction>,
}

impl AuditMiddleware {
    /// 创建新的审计中间件
    pub fn new(storage: Arc<dyn AuditStorage>, enabled_actions: Vec<AuditAction>) -> Self {
        Self {
            storage,
            enabled_actions,
        }
    }

    /// 检查是否应该审计该操作
    pub fn should_audit(&self, action: &AuditAction) -> bool {
        self.enabled_actions.is_empty() || self.enabled_actions.contains(action)
    }
}

/// 审计上下文 - 用于在请求中传递审计信息
#[derive(Debug, Clone)]
pub struct AuditContext {
    /// 审计记录
    record: AuditRecord,

    /// 记录开始时间
    start_time: std::time::Instant,
}

impl AuditContext {
    /// 创建新的审计上下文
    pub fn new(
        tenant_id: Uuid,
        user_id: Uuid,
        action: AuditAction,
        resource_type: &str,
        resource_id: &str,
        trace_id: Uuid,
    ) -> Self {
        Self {
            record: AuditRecord::new(
                tenant_id,
                user_id,
                action,
                resource_type,
                resource_id,
                trace_id,
            ),
            start_time: std::time::Instant::now(),
        }
    }

    /// 完成审计（成功）
    pub fn complete(mut self) -> AuditRecord {
        self.record.duration_ms = self.start_time.elapsed().as_millis() as u64;
        self.record
    }

    /// 完成审计（失败）
    pub fn complete_with_error(mut self, error: &str) -> AuditRecord {
        self.record.duration_ms = self.start_time.elapsed().as_millis() as u64;
        self.record.with_error(error)
    }

    /// 设置资源 ID
    pub fn with_resource_id(mut self, resource_id: &str) -> Self {
        self.record.resource_id = resource_id.to_string();
        self
    }

    /// 设置元数据
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.record = self.record.with_metadata(metadata);
        self
    }
}

/// 从请求中提取审计信息
pub fn extract_audit_info_from_request<T>(
    request: &Request<T>,
    action: AuditAction,
) -> Option<(AuditContext, Uuid)> {
    let trace_id = request
        .metadata()
        .get("x-trace-id")
        .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok())
        .unwrap_or_else(Uuid::new_v4);

    // 尝试从 auth context 获取信息
    // 这里简化处理，实际应该从 request.extensions() 获取 AuthContext
    let tenant_id = request
        .metadata()
        .get("x-tenant-id")
        .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok())
        .unwrap_or_else(Uuid::nil);

    let user_id = request
        .metadata()
        .get("x-user-id")
        .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok())
        .unwrap_or_else(Uuid::nil);

    if tenant_id == Uuid::nil() || user_id == Uuid::nil() {
        return None;
    }

    Some((
        AuditContext::new(
            tenant_id,
            user_id,
            action,
            "journal_entry",
            "",
            trace_id,
        ),
        trace_id,
    ))
}

/// 带审计的请求处理
pub async fn with_audit<T, F, Fut>(
    middleware: &AuditMiddleware,
    request: &Request<()>,
    action: AuditAction,
    resource_id: &str,
    f: F,
) -> Result<T, Status>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, Status>>,
{
    if !middleware.should_audit(&action) {
        return f().await;
    }

    let (mut audit_ctx, trace_id) = match extract_audit_info_from_request(request, action) {
        Some(ctx) => (ctx, trace_id),
        None => {
            // 无法提取审计信息，降级处理
            return f().await;
        }
    };

    audit_ctx = audit_ctx.with_resource_id(resource_id);

    let start_time = std::time::Instant::now();
    let span = span!(Level::INFO, "audit", trace_id = %trace_id, action = ?action, resource_id = resource_id);

    match f().instrument(span).await {
        Ok(response) => {
            let duration_ms = start_time.elapsed().as_millis() as u64;
            let record = audit_ctx.complete().with_duration(duration_ms);

            // 异步保存审计记录
            if let Err(e) = middleware.storage.save(&record).await {
                error!(%trace_id, "Failed to save audit record: {}", e);
            }

            Ok(response)
        }
        Err(status) => {
            let duration_ms = start_time.elapsed().as_millis() as u64;
            let record = audit_ctx
                .complete_with_error(status.message())
                .with_duration(duration_ms);

            if let Err(e) = middleware.storage.save(&record).await {
                error!(%trace_id, "Failed to save audit record: {}", e);
            }

            Err(status)
        }
    }
}
