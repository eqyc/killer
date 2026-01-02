//! 应用层错误
//!
//! 定义应用层错误类型，与 CQRS 框架的 AppError 区分

use thiserror::Error;

/// 应用层错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ApplicationError {
    /// 验证失败
    #[error("验证失败: {message}")]
    ValidationFailed { message: String, field: Option<String> },

    /// 业务规则违反
    #[error("业务规则违反: {code} - {message}")]
    BusinessRuleViolation { code: &'static str, message: String },

    /// 资源未找到
    #[error("资源未找到: {resource_type} {identifier}")]
    NotFound { resource_type: String, identifier: String },

    /// 资源已存在
    #[error("资源已存在: {resource_type} {identifier}")]
    AlreadyExists { resource_type: String, identifier: String },

    /// 冲突（例如版本冲突）
    #[error("冲突: {message}")]
    Conflict { message: String },

    /// 禁止访问
    #[error("禁止访问: {message}")]
    Forbidden { message: String },

    /// 未实现
    #[error("未实现: {message}")]
    NotImplemented { message: String },

    /// 基础设施错误
    #[error("基础设施错误: {message}")]
    InfrastructureError { message: String },

    /// 外部服务错误
    #[error("外部服务错误: {service} - {message}")]
    ExternalServiceError { service: String, message: String },
}

impl ApplicationError {
    /// 创建验证失败错误
    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
            field: None,
        }
    }

    /// 创建验证失败错误（带字段）
    pub fn validation_failed_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// 创建业务规则违反错误
    pub fn business_rule_violation(code: &'static str, message: impl Into<String>) -> Self {
        Self::BusinessRuleViolation {
            code,
            message: message.into(),
        }
    }

    /// 创建未找到错误
    pub fn not_found(resource_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// 创建已存在错误
    pub fn already_exists(resource_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::AlreadyExists {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// 创建冲突错误
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
        }
    }

    /// 创建禁止访问错误
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden {
            message: message.into(),
        }
    }

    /// 创建未实现错误
    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::NotImplemented {
            message: message.into(),
        }
    }

    /// 创建基础设施错误
    pub fn infrastructure_error(message: impl Into<String>) -> Self {
        Self::InfrastructureError {
            message: message.into(),
        }
    }

    /// 创建外部服务错误
    pub fn external_service_error(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalServiceError {
            service: service.into(),
            message: message.into(),
        }
    }

    /// 获取错误代码
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::ValidationFailed { .. } => "VALIDATION_FAILED",
            Self::BusinessRuleViolation { code, .. } => code,
            Self::NotFound { .. } => "NOT_FOUND",
            Self::AlreadyExists { .. } => "ALREADY_EXISTS",
            Self::Conflict { .. } => "CONFLICT",
            Self::Forbidden { .. } => "FORBIDDEN",
            Self::NotImplemented { .. } => "NOT_IMPLEMENTED",
            Self::InfrastructureError { .. } => "INFRASTRUCTURE_ERROR",
            Self::ExternalServiceError { .. } => "EXTERNAL_SERVICE_ERROR",
        }
    }
}

/// 应用结果类型
pub type ApplicationResult<T> = Result<T, ApplicationError>;
