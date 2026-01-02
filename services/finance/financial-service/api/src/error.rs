//! API 错误类型和标准化错误处理
//!
//! 将领域错误、应用错误映射为 gRPC 状态码和结构化错误响应

use crate::services::journal_entry::{GetAccountBalanceError, GetTrialBalanceError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tonic::{Code, Status};
use uuid::Uuid;

use killer_financial_service::application::error::ApplicationError;

// =============================================================================
// 错误码定义
// =============================================================================

/// 错误码枚举 - 对应 gRPC 状态码和业务错误类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    /// 验证失败 - 参数无效
    ValidationFailed,
    /// 业务规则违反 - 前置条件不满足
    BusinessRuleViolation,
    /// 资源未找到
    NotFound,
    /// 资源已存在（冲突）
    AlreadyExists,
    /// 并发冲突（乐观锁）
    Conflict,
    /// 权限不足
    PermissionDenied,
    /// 资源耗尽（限流）
    ResourceExhausted,
    /// 内部服务器错误
    Internal,
    /// 服务不可用
    Unavailable,
    /// 未实现
    Unimplemented,
}

impl ErrorCode {
    /// 转换为 gRPC 状态码
    pub fn to_grpc_code(&self) -> Code {
        match self {
            ErrorCode::ValidationFailed => Code::InvalidArgument,
            ErrorCode::NotFound => Code::NotFound,
            ErrorCode::AlreadyExists | ErrorCode::Conflict => Code::AlreadyExists,
            ErrorCode::PermissionDenied => Code::PermissionDenied,
            ErrorCode::ResourceExhausted => Code::ResourceExhausted,
            ErrorCode::BusinessRuleViolation => Code::FailedPrecondition,
            ErrorCode::Internal => Code::Internal,
            ErrorCode::Unavailable => Code::Unavailable,
            ErrorCode::Unimplemented => Code::Unimplemented,
        }
    }

    /// 获取错误码字符串
    pub fn as_str(&self) -> &str {
        match self {
            ErrorCode::ValidationFailed => "VALIDATION_FAILED",
            ErrorCode::BusinessRuleViolation => "BUSINESS_RULE_VIOLATION",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::AlreadyExists => "ALREADY_EXISTS",
            ErrorCode::Conflict => "CONFLICT",
            ErrorCode::PermissionDenied => "PERMISSION_DENIED",
            ErrorCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
            ErrorCode::Internal => "INTERNAL",
            ErrorCode::Unavailable => "UNAVAILABLE",
            ErrorCode::Unimplemented => "UNIMPLEMENTED",
        }
    }
}

impl From<Code> for ErrorCode {
    fn from(code: Code) -> Self {
        match code {
            Code::InvalidArgument => ErrorCode::ValidationFailed,
            Code::NotFound => ErrorCode::NotFound,
            Code::AlreadyExists => ErrorCode::AlreadyExists,
            Code::PermissionDenied => ErrorCode::PermissionDenied,
            Code::ResourceExhausted => ErrorCode::ResourceExhausted,
            Code::FailedPrecondition => ErrorCode::BusinessRuleViolation,
            Code::Aborted => ErrorCode::Conflict,
            Code::Internal => ErrorCode::Internal,
            Code::Unavailable => ErrorCode::Unavailable,
            Code::Unimplemented => ErrorCode::Unimplemented,
            Code::Ok => ErrorCode::Internal,
            _ => ErrorCode::Internal,
        }
    }
}

// =============================================================================
// API 错误响应
// =============================================================================

/// API 错误详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// 字段路径（如 "line_items[0].amount"）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// 错误类型
    pub r#type: String,

    /// 错误消息
    pub message: String,

    /// 违反的值（可选，用于调试）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl ErrorDetail {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: Some(field.to_string()),
            r#type: "error".to_string(),
            message: message.to_string(),
            value: None,
        }
    }

    pub fn with_value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }
}

/// API 错误结构
#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<Vec<ErrorDetail>>,
    pub trace_id: Option<uuid::Uuid>,
}

impl ApiError {
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            details: None,
            trace_id: None,
        }
    }

    pub fn validation_failed(message: &str) -> Self {
        Self::new(ErrorCode::ValidationFailed, message)
    }

    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::new(
            ErrorCode::NotFound,
            &format!("{} {} not found", resource, id),
        )
    }

    pub fn conflict(message: &str) -> Self {
        Self::new(ErrorCode::Conflict, message)
    }

    pub fn permission_denied(message: &str) -> Self {
        Self::new(ErrorCode::PermissionDenied, message)
    }

    pub fn with_trace_id(mut self, trace_id: uuid::Uuid) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    pub fn with_details(mut self, details: Vec<ErrorDetail>) -> Self {
        self.details = Some(details);
        self
    }
}

/// API 结果类型
pub type ApiResult<T> = Result<T, ApiError>;

/// API 错误响应结构（用于 HTTP JSON 响应）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// 错误码
    pub code: String,

    /// 错误消息
    pub message: String,

    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ErrorDetail>>,

    /// 跟踪 ID（用于日志关联）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl ApiErrorResponse {
    pub fn from_error(err: &ApiError) -> Self {
        Self {
            code: err.code.as_str().to_string(),
            message: err.message.clone(),
            details: err.details.clone(),
            trace_id: err.trace_id.map(|t| t.to_string()),
            timestamp: Utc::now(),
        }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ApiErrorResponse {}

// =============================================================================
// From trait implementations for error conversion
// =============================================================================

impl From<ApiError> for Status {
    fn from(err: ApiError) -> Self {
        let code = err.code.to_grpc_code();
        let mut status = Status::new(code, err.message);

        if let Some(details) = err.details {
            let details_json = serde_json::to_string(&details).unwrap_or_default();
            status = status.with_details(details_json.into_bytes());
        }

        if let Some(trace_id) = err.trace_id {
            status = status.add_option_metadata("x-trace-id", trace_id.as_bytes());
        }

        status
    }
}

impl From<Status> for ApiErrorResponse {
    fn from(status: Status) -> Self {
        let code = ErrorCode::from(status.code());
        Self {
            code: code.as_str().to_string(),
            message: status.message().to_string(),
            details: None,
            trace_id: status.metadata().get("x-trace-id").and_then(|v| {
                std::str::from_utf8(v).ok().map(|s| s.to_string())
            }),
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// Domain error mapping
// =============================================================================

/// 将领域错误映射为 API 错误
pub fn map_domain_error(
    error: &killer_financial_service::domain::DomainError,
    trace_id: uuid::Uuid,
) -> ApiError {
    use killer_financial_service::domain::DomainError::*;

    match error {
        UnbalancedEntry { debit, credit } => ApiError::new(
            ErrorCode::BusinessRuleViolation,
            &format!("借方金额 {} 与贷方金额 {} 不平衡", debit, credit),
        )
        .with_trace_id(trace_id)
        .with_details(vec![ErrorDetail::new("line_items", "借贷必须平衡")]),

        PeriodClosed {
            company_code,
            fiscal_year,
            period,
        } => ApiError::new(
            ErrorCode::BusinessRuleViolation,
            &format!("公司代码 {} 的 {} 年度第 {} 会计期间已关闭", company_code, fiscal_year, period),
        )
        .with_trace_id(trace_id),

        InsufficientLineItems { min_required, actual } => ApiError::new(
            ErrorCode::ValidationFailed,
            &format!("至少需要 {} 行项目，当前只有 {} 行", min_required, actual),
        )
        .with_trace_id(trace_id)
        .with_details(vec![ErrorDetail::new(
            "line_items",
            &format!("至少需要 {} 行项目", min_required),
        )]),

        InvalidPostingDate { date, reason } => ApiError::new(
            ErrorCode::ValidationFailed,
            &format!("过账日期 {} 无效: {}", date, reason),
        )
        .with_trace_id(trace_id)
        .with_details(vec![ErrorDetail::new("posting_date", reason)]),

        AlreadyPosted { document_number } => ApiError::new(
            ErrorCode::Conflict,
            &format!("凭证 {} 已过账，不能重复操作", document_number),
        )
        .with_trace_id(trace_id),

        AlreadyReversed { document_number } => ApiError::new(
            ErrorCode::Conflict,
            &format!("凭证 {} 已冲销，不能重复冲销", document_number),
        )
        .with_trace_id(trace_id),

        ConcurrencyConflict {
            document_number,
            expected_version,
            actual_version,
        } => ApiError::new(
            ErrorCode::Conflict,
            &format!(
                "凭证 {} 的版本冲突，期望版本 {}，实际版本 {}",
                document_number, expected_version, actual_version
            ),
        )
        .with_trace_id(trace_id),

        InvalidAccountCode { code } => ApiError::new(
            ErrorCode::ValidationFailed,
            &format!("会计科目代码 {} 无效", code),
        )
        .with_trace_id(trace_id)
        .with_details(vec![ErrorDetail::new("account_code", "科目代码格式无效")]),

        InvalidAmount { amount } => ApiError::new(
            ErrorCode::ValidationFailed,
            &format!("金额 {} 无效，必须为正数", amount),
        )
        .with_trace_id(trace_id)
        .with_details(vec![ErrorDetail::new("amount", "金额必须为正数")]),

        _ => ApiError::new(
            ErrorCode::Internal,
            &format!("未知领域错误: {:?}", error),
        )
        .with_trace_id(trace_id),
    }
}

/// 将应用错误映射为 API 错误
pub fn map_application_error(error: &ApplicationError, trace_id: uuid::Uuid) -> ApiError {
    match error {
        ApplicationError::ValidationFailed { message, field } => {
            let mut err = ApiError::validation_failed(message);
            err.trace_id = Some(trace_id);
            if let Some(f) = field {
                err.details = Some(vec![ErrorDetail::new(f, message)]);
            }
            err
        }
        ApplicationError::NotFound {
            resource_type,
            identifier,
        } => ApiError::not_found(resource_type, identifier).with_trace_id(trace_id),
        ApplicationError::Conflict { message } => ApiError::conflict(message).with_trace_id(trace_id),
        ApplicationError::Forbidden { message } => {
            ApiError::permission_denied(message).with_trace_id(trace_id)
        }
        ApplicationError::BusinessRuleViolation { code: _, message } => {
            let mut err = ApiError::new(ErrorCode::BusinessRuleViolation, message);
            err.trace_id = Some(trace_id);
            err
        }
        ApplicationError::InfrastructureError { message } => {
            let mut err = ApiError::new(ErrorCode::Internal, message);
            err.trace_id = Some(trace_id);
            err
        }
        ApplicationError::ExternalServiceError { service, message } => {
            let mut err = ApiError::new(
                ErrorCode::Unavailable,
                &format!("外部服务 {} 错误: {}", service, message),
            );
            err.trace_id = Some(trace_id);
            err
        }
        _ => {
            let mut err = ApiError::new(ErrorCode::Internal, &format!("{:?}", error));
            err.trace_id = Some(trace_id);
            err
        }
    }
}

// =============================================================================
// gRPC Status helpers
// =============================================================================

/// 创建 gRPC 验证错误状态
pub fn invalid_argument(field: &str, message: &str) -> Status {
    Status::invalid_argument(format!("{}: {}", field, message))
}

/// 创建 gRPC 未找到错误状态
pub fn not_found(resource: &str, id: &str) -> Status {
    Status::not_found(format!("{} {} not found", resource, id))
}

/// 创建 gRPC 权限不足错误状态
pub fn permission_denied(message: &str) -> Status {
    Status::permission_denied(message)
}

/// 创建 gRPC 冲突错误状态
pub fn conflict(message: &str) -> Status {
    Status::already_exists(message)
}

/// 创建 gRPC 前置条件错误状态
pub fn failed_precondition(message: &str) -> Status {
    Status::failed_precondition(message)
}

/// 从验证错误创建 gRPC 状态
pub fn validation_errors_to_status(errors: &validator::ValidationErrors) -> Status {
    let violations: Vec<FieldViolation> = errors
        .field_errors()
        .map(|(field, error_list)| {
            error_list
                .iter()
                .map(|e| FieldViolation {
                    field: field.to_string(),
                    description: e.message.clone().unwrap_or_default(),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let details_json = serde_json::to_string(&violations).unwrap_or_default();
    Status::invalid_argument("validation failed").with_details(details_json.into_bytes())
}

/// 字段验证错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldViolation {
    pub field: String,
    pub description: String,
}
