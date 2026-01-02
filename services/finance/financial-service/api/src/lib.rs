//! KILLER ERP Financial Service API Layer
//!
//! API 层作为外部系统的统一入口，负责：
//! - gRPC 服务暴露（tonic）
//! - HTTP REST 服务（tonic-web/axum）
//! - 认证授权中间件
//! - 幂等性控制
//! - 审计追踪
//! - 错误标准化
//!
//! ## 架构
//!
//! ```text
//! +------------------+     +------------------+     +-------------------+
//! | External Client  | --> | API Layer        | --> | Application Layer |
//! | (gRPC/REST)      |     | - Auth/Authz     |     | - CQRS Commands   |
//! |                  |     | - Idempotency    |     | - CQRS Queries    |
//! |                  |     | - Audit          |     |                   |
//! +------------------+     +------------------+     +-------------------+
//! ```

pub mod config;
pub mod error;
pub mod middleware;
pub mod server;
pub mod services;

// Proto generated code (after compilation)
#[path = "prost/killer.finance.v1.rs"]
pub mod finance_v1;

pub use config::ApiConfig;
pub use error::{ApiError, ApiResult, ErrorCode, ErrorDetail};
pub use middleware::auth::{AuthContext, AuthInterceptor};
pub use middleware::idempotency::IdempotencyMiddleware;
pub use middleware::audit::AuditMiddleware;
pub use server::ApiServer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
