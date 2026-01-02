//! CQRS 框架
//!
//! 生产级命令查询职责分离模式实现，支撑高并发、强一致性的企业级业务。
//!
//! ## 核心特性
//!
//! - **命令系统**: 强类型的 Command/CommandHandler，支持幂等性和验证
//! - **事件系统**: 领域事件 + 发件箱模式，确保事件不丢失
//! - **查询系统**: 高性能只读操作，支持 Redis 缓存
//! - **事务安全**: Unit of Work 确保业务操作原子性
//! - **可观测性**: 集成 tracing，自动生成审计日志
//! - **中间件**: 日志、验证、审计等横切关注点
//!
//! ## 模块结构
//!
//! - [`context`] - 命令上下文，提供租户、用户、追踪信息
//! - [`error`] - 统一的错误类型和错误码
//! - [`command`] - 命令系统和处理器
//! - [`event`] - 领域事件和事件信封
//! - [`event_bus`] - 事件总线（同步/异步分发）
//! - [`query`] - 查询系统和处理器
//! - [`middleware`] - 中间件装饰器
//! - [`uow`] - 事务单元
//! - [`registry`] - Handler 注册表
//! - [`testkit`] - 测试工具包
//! - [`axum`] - Axum 框架集成

#![warn(missing_docs, unreachable_pub)]
#![deny(rust_2018_idioms, nonstandard_style)]
#![allow(unsafe_code)]

// Re-export core types
pub use crate::context::{CommandContext, SharedContext};
pub use crate::error::{AppError, Result};

// Command system
pub use crate::command::{
    Command, CommandHandler, CommandHandlerDecorator, LoggingDecorator,
    ValidationDecorator,
};

// Event system
pub use crate::event::{
    DomainEvent, EventBus, EventEnvelope, EventHandler, OutboxEvent, OutboxRepository,
};

// Event bus
pub use event_bus::DefaultEventBus;

// Query system
pub use crate::query::{Query, QueryHandler, QueryHandlerDecorator};

// Middleware
pub use crate::middleware::{
    AuditLog, AuditLogDecorator, AuditLogRepository, IdempotencyChecker, IdempotencyDecorator,
};

// Unit of Work
pub use crate::uow::{UnitOfWork, UnitOfWorkDecorator};

// Registry
pub use crate::registry::{
    CqrsApplication, CqrsApplicationBuilder, CommandHandlerRegistry, EventHandlerRegistry,
    QueryHandlerRegistry,
};

// Test kit
pub use crate::testkit::{
    MockEventBus, MockEventHandler, MockOutboxRepository, MockQueryHandler,
};

// Axum integration
#[cfg(feature = "axum")]
pub use crate::axum::{
    ApiResponse, ErrorResponse, ExtractedCommandContext,
};

mod context;
mod error;
mod command;
mod event;
mod event_bus;
mod query;
mod middleware;
mod uow;
mod registry;
mod testkit;

#[cfg(feature = "axum")]
mod axum;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestCommand {
        value: String,
    }

    impl Command for TestCommand {
        type Output = String;

        fn command_name() -> &'static str {
            "TestCommand"
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        message: String,
    }

    impl DomainEvent for TestEvent {
        fn event_name() -> &'static str {
            "TestEvent"
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestQuery {
        id: String,
    }

    impl Query for TestQuery {
        type Output = String;

        fn query_name() -> &'static str {
            "TestQuery"
        }
    }

    #[test]
    fn test_lib_exports() {
        // 确保所有核心类型都能正确导出
        let _ctx = CommandContext::new("tenant", "user");
        let _error = AppError::not_found("test");
        let _cmd = TestCommand { value: "test".to_string() };
        let _event = TestEvent { message: "test".to_string() };
        let _query = TestQuery { id: "test".to_string() };
    }
}
