//! API 中间件模块
//!
//! 提供认证授权、幂等性控制、审计追踪等横切关注点

pub mod auth;
pub mod idempotency;
pub mod audit;
pub mod tracing;
pub mod metrics;

// Re-exports
pub use auth::{extract_auth_context, AuthContext, AuthInterceptor};
pub use idempotency::{IdempotencyKey, IdempotencyMiddleware};
pub use audit::{AuditMiddleware, AuditRecord};
