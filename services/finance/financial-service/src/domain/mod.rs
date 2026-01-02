//! 财务领域层
//!
//! 纯领域层实现，遵循 DDD 原则
//!
//! # 架构
//!
//! - **aggregates**: 聚合根（JournalEntry, FiscalPeriod）
//! - **entities**: 实体（JournalEntryLineItem）
//! - **value_objects**: 值对象（Status, DebitCredit, ID 等）
//! - **events**: 领域事件
//! - **services**: 领域服务
//! - **repositories**: 仓储接口
//! - **error**: 领域错误
//!
//! # 核心概念
//!
//! ## 聚合根
//!
//! - **JournalEntry**: 会计凭证聚合根，对应 SAP ACDOCA 表
//! - **FiscalPeriod**: 会计期间聚合根
//!
//! ## 不变式
//!
//! - 借贷必须平衡（精确到货币精度）
//! - 凭证至少包含 2 行项目
//! - 过账日期必须在开放的会计期间内
//! - 已过账的凭证不可修改，仅可冲销
//! - 所有行项目的 tenant_id 必须一致
//!
//! ## 事件驱动
//!
//! 所有状态变更都会产生领域事件：
//! - JournalEntryPosted
//! - JournalEntryReversed
//! - FiscalPeriodClosed
//! - FiscalPeriodOpened

pub mod aggregates;
pub mod entities;
pub mod error;
pub mod events;
pub mod repositories;
pub mod services;
pub mod value_objects;

// Re-exports for convenience
pub use aggregates::{FiscalPeriod, JournalEntry};
pub use entities::JournalEntryLineItem;
pub use error::{DomainError, DomainResult};
pub use events::DomainEvent;
pub use repositories::{FiscalPeriodRepository, JournalEntryRepository};
pub use services::{BalanceCalculator, PeriodCloseService};
pub use value_objects::{
    DebitCredit, FiscalPeriodId, JournalEntryId, JournalEntryStatus, PeriodStatus, ProfitCenter,
    ValidityRange,
};
