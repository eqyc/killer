//! Axum 集成
//!
//! 提供将 Command 和 Query 直接映射为 API Endpoint 的辅助工具

use crate::{
    command::{Command, CommandHandler},
    context::CommandContext,
    error::{AppError, Result},
    query::{Query, QueryHandler},
};
use axum::{
    body::Body,
    extract::Request,
    response::IntoResponse,
    Json,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;

// Re-export for convenience
pub use axum::response::Response;

/// 扩展的 Command 上下文（从请求头提取）
#[derive(Debug, Clone)]
pub struct ExtractedCommandContext {
    /// 租户 ID
    pub tenant_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 追踪 ID
    pub trace_id: Option<String>,
    /// 区域设置
    pub locale: Option<String>,
}

impl ExtractedCommandContext {
    /// 从请求头提取上下文
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Result<Self, AppError> {
        let tenant_id = headers
            .get("X-Tenant-ID")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::unauthorized("Missing X-Tenant-ID header"))?
            .to_string();

        let user_id = headers
            .get("X-User-ID")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::unauthorized("Missing X-User-ID header"))?
            .to_string();

        let trace_id = headers
            .get("X-Trace-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let locale = headers
            .get("Accept-Language")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(Self {
            tenant_id,
            user_id,
            trace_id,
            locale,
        })
    }

    /// 转换为 CommandContext
    pub fn to_command_context(&self, trace_id: uuid::Uuid) -> CommandContext {
        CommandContext::new(&self.tenant_id, &self.user_id)
            .with_trace_id(trace_id)
            .with_locale(self.locale.as_deref().unwrap_or("zh-CN"))
    }
}

/// 错误响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn from_error(error: &AppError) -> Self {
        Self {
            error: error.to_string(),
            code: error.error_code().to_string(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// API 响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorResponse {
                error: message.to_string(),
                code: code.to_string(),
                details: None,
            }),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{command::Command, context::CommandContext, query::Query};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    struct CreateUserCommand {
        #[validate(length(min = 1, max = 100))]
        name: String,
        #[validate(email)]
        email: String,
    }

    impl Command for CreateUserCommand {
        type Output = String;

        fn command_name() -> &'static str {
            "CreateUserCommand"
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct GetUserQuery {
        user_id: String,
    }

    impl Query for GetUserQuery {
        type Output = String;

        fn query_name() -> &'static str {
            "GetUserQuery"
        }
    }

    #[tokio::test]
    async fn test_error_response() {
        let error = AppError::not_found("User not found");
        let response = ErrorResponse::from_error(&error);

        assert_eq!(response.code, "NOT_FOUND");
        assert_eq!(response.error, "Not found: User not found");
    }

    #[tokio::test]
    async fn test_api_response() {
        let response = ApiResponse::<String>::success("test".to_string());
        assert!(response.success);
        assert_eq!(response.data, Some("test".to_string()));
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_extracted_command_context() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("X-Tenant-ID", "tenant-001".parse().unwrap());
        headers.insert("X-User-ID", "user-123".parse().unwrap());
        headers.insert("X-Trace-ID", "550e8400-e29b-41d4-a716-446655440000".parse().unwrap());

        let ctx = ExtractedCommandContext::from_headers(&headers).unwrap();
        assert_eq!(ctx.tenant_id, "tenant-001");
        assert_eq!(ctx.user_id, "user-123");
        assert!(ctx.trace_id.is_some());
    }
}
