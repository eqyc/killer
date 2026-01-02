//! CQRS 错误类型
//!
//! 定义统一的错误类型，支持细粒度的错误码分类

use thiserror::Error;

/// CQRS 应用错误
#[derive(Error, Debug)]
pub enum AppError {
    /// 领域冲突错误 (409)
    /// 例如：重复的订单号、库存不足等业务规则冲突
    #[error("Domain conflict: {message}")]
    DomainConflict {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// 验证失败 (400)
    /// 例如：字段格式错误、必填字段缺失等
    #[error("Validation failed: {message}")]
    ValidationFailed {
        message: String,
        field: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// 未授权 (401)
    #[error("Unauthorized: {message}")]
    Unauthorized {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// 禁止访问 (403)
    #[error("Forbidden: {message}")]
    Forbidden {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// 资源未找到 (404)
    #[error("Not found: {message}")]
    NotFound {
        message: String,
        resource_type: Option<String>,
        resource_id: Option<String>,
    },

    /// 基础设施故障 (500)
    /// 例如：数据库连接失败、消息队列故障等
    #[error("Infrastructure failure: {message}")]
    InfrastructureFailure {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// 幂等性冲突
    /// 检测到重复的命令执行
    #[error("Idempotency conflict: command already executed with id {command_id}")]
    IdempotencyConflict { command_id: String },

    /// 并发冲突 (409)
    /// 乐观锁版本冲突
    #[error("Concurrency conflict: expected version {expected}, got {actual}")]
    ConcurrencyConflict { expected: i64, actual: i64 },

    /// 内部错误 (500)
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// 获取 HTTP 状态码
    pub fn status_code(&self) -> u16 {
        match self {
            Self::DomainConflict { .. } => 409,
            Self::ValidationFailed { .. } => 400,
            Self::Unauthorized { .. } => 401,
            Self::Forbidden { .. } => 403,
            Self::NotFound { .. } => 404,
            Self::InfrastructureFailure { .. } => 500,
            Self::IdempotencyConflict { .. } => 409,
            Self::ConcurrencyConflict { .. } => 409,
            Self::Internal(_) => 500,
        }
    }

    /// 获取错误码（用于客户端识别）
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::DomainConflict { .. } => "DOMAIN_CONFLICT",
            Self::ValidationFailed { .. } => "VALIDATION_FAILED",
            Self::Unauthorized { .. } => "UNAUTHORIZED",
            Self::Forbidden { .. } => "FORBIDDEN",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::InfrastructureFailure { .. } => "INFRASTRUCTURE_FAILURE",
            Self::IdempotencyConflict { .. } => "IDEMPOTENCY_CONFLICT",
            Self::ConcurrencyConflict { .. } => "CONCURRENCY_CONFLICT",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// 是否为可重试错误
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::InfrastructureFailure { .. } | Self::ConcurrencyConflict { .. }
        )
    }

    /// 创建领域冲突错误
    pub fn domain_conflict(message: impl Into<String>) -> Self {
        Self::DomainConflict {
            message: message.into(),
            source: None,
        }
    }

    /// 创建验证失败错误
    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
            field: None,
            source: None,
        }
    }

    /// 创建验证失败错误（带字段名）
    pub fn validation_failed_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
            field: Some(field.into()),
            source: None,
        }
    }

    /// 创建未授权错误
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized {
            message: message.into(),
            source: None,
        }
    }

    /// 创建禁止访问错误
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden {
            message: message.into(),
            source: None,
        }
    }

    /// 创建资源未找到错误
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
            resource_type: None,
            resource_id: None,
        }
    }

    /// 创建资源未找到错误（带资源类型和 ID）
    pub fn not_found_resource(
        resource_type: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        let resource_type_str = resource_type.into();
        let resource_id_str = resource_id.into();
        Self::NotFound {
            message: format!("{} not found: {}", resource_type_str, resource_id_str),
            resource_type: Some(resource_type_str),
            resource_id: Some(resource_id_str),
        }
    }

    /// 创建基础设施故障错误
    pub fn infrastructure_failure(message: impl Into<String>) -> Self {
        Self::InfrastructureFailure {
            message: message.into(),
            source: None,
        }
    }

    /// 创建并发冲突错误
    pub fn concurrency_conflict(expected: i64, actual: i64) -> Self {
        Self::ConcurrencyConflict { expected, actual }
    }
}

/// CQRS 结果类型
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(AppError::domain_conflict("test").status_code(), 409);
        assert_eq!(AppError::validation_failed("test").status_code(), 400);
        assert_eq!(AppError::unauthorized("test").status_code(), 401);
        assert_eq!(AppError::forbidden("test").status_code(), 403);
        assert_eq!(AppError::not_found("test").status_code(), 404);
        assert_eq!(
            AppError::infrastructure_failure("test").status_code(),
            500
        );
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(
            AppError::domain_conflict("test").error_code(),
            "DOMAIN_CONFLICT"
        );
        assert_eq!(
            AppError::validation_failed("test").error_code(),
            "VALIDATION_FAILED"
        );
    }

    #[test]
    fn test_retryable() {
        assert!(AppError::infrastructure_failure("test").is_retryable());
        assert!(AppError::concurrency_conflict(1, 2).is_retryable());
        assert!(!AppError::validation_failed("test").is_retryable());
    }
}
