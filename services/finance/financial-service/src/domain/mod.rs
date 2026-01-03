//! 财务服务领域层
//!
//! 包含总账会计、应收账款、应付账款、固定资产、银行会计等领域的聚合根、实体、值对象

pub mod aggregates;
pub mod entities;
pub mod value_objects;
pub mod repositories;
pub mod events;
pub mod services;

// Re-exports for easier access
pub use aggregates::*;
pub use entities::*;
pub use value_objects::*;
pub use repositories::*;
pub use events::*;
