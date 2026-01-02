//! 应用层
//!
//! CQRS 模式实现，包含命令、查询、事件处理器
//!
//! ## 模块结构
//!
//! - `commands/` - 命令处理器（写模型）
//! - `queries/` - 查询处理器（读模型）
//! - `events/` - 事件处理器
//! - `dto/` - 数据传输对象
//! - `mapper/` - DTO 映射器
//! - `services/` - 应用服务
//! - `repositories/` - 读模型仓储接口
//! - `error.rs` - 应用错误类型

pub mod commands;
pub mod queries;
pub mod events;
pub mod dto;
pub mod mapper;
pub mod services;
pub mod repositories;
pub mod error;

// Re-exports
pub use error::{ApplicationError, ApplicationResult};
