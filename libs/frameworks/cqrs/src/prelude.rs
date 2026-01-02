//! CQRS 框架Prelude
//!
//! 重新导出所有常用类型，方便一次性导入

// Context
pub use crate::context::{CommandContext, SharedContext};

// Error
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
pub use crate::event_bus::DefaultEventBus;

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
