//! 命令上下文
//!
//! 提供统一的请求上下文，包含租户、用户、追踪等信息

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 命令执行上下文
///
/// 包含执行命令所需的所有上下文信息，在整个调用链中透明传递
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContext {
    /// 租户 ID（多租户隔离）
    pub tenant_id: String,

    /// 用户 ID
    pub user_id: String,

    /// 追踪 ID（用于分布式追踪）
    pub trace_id: Uuid,

    /// 区域设置（语言/时区）
    pub locale: String,

    /// 请求时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// 扩展元数据
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

impl CommandContext {
    /// 创建新的命令上下文
    pub fn new(tenant_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            user_id: user_id.into(),
            trace_id: Uuid::now_v7(),
            locale: "zh-CN".to_string(),
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// 设置追踪 ID
    pub fn with_trace_id(mut self, trace_id: Uuid) -> Self {
        self.trace_id = trace_id;
        self
    }

    /// 设置区域设置
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 创建 tracing span
    pub fn create_span(&self, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "cqrs_operation",
            operation = operation,
            tenant_id = %self.tenant_id,
            user_id = %self.user_id,
            trace_id = %self.trace_id,
            locale = %self.locale,
        )
    }
}

/// 共享的命令上下文（用于跨异步边界传递）
pub type SharedContext = Arc<CommandContext>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = CommandContext::new("tenant-001", "user-123")
            .with_locale("en-US")
            .with_metadata("ip", "192.168.1.1");

        assert_eq!(ctx.tenant_id, "tenant-001");
        assert_eq!(ctx.user_id, "user-123");
        assert_eq!(ctx.locale, "en-US");
        assert_eq!(ctx.metadata.get("ip"), Some(&"192.168.1.1".to_string()));
    }

    #[test]
    fn test_span_creation() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let span = ctx.create_span("test_operation");

        // Span 应该被正确创建
        assert_eq!(span.metadata().unwrap().name(), "cqrs_operation");
    }
}
