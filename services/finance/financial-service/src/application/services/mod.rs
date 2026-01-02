//! 应用服务模块
//!
//! 提供高级业务服务，协调多个聚合根和领域服务

pub mod journal_entry_application_service;
pub mod reporting_service;

// =============================================================================
// 共享类型
// =============================================================================

use crate::application::error::ApplicationError;
use std::sync::Arc;

/// 应用服务结果
pub type ServiceResult<T> = Result<T, ApplicationError>;
