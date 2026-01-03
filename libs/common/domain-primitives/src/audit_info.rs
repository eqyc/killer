//! 审计信息值对象
//!
//! 用于跟踪领域对象的创建和修改信息

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 审计信息
///
/// 记录领域对象的创建者、创建时间、修改者和修改时间
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditInfo {
    /// 创建者
    created_by: String,
    /// 创建时间
    created_at: DateTime<Utc>,
    /// 修改者
    updated_by: String,
    /// 修改时间
    updated_at: DateTime<Utc>,
}

impl AuditInfo {
    /// 创建新的审计信息
    pub fn new(by: String, at: DateTime<Utc>) -> Self {
        Self {
            created_by: by.clone(),
            created_at: at,
            updated_by: by,
            updated_at: at,
        }
    }

    /// 创建新的审计信息（简化版本）
    pub fn system() -> Self {
        let now = Utc::now();
        Self::new("SYSTEM".to_string(), now)
    }

    /// 创建新的审计信息（带时间戳）
    pub fn with_timestamp(by: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            created_by: by.clone(),
            created_at: timestamp,
            updated_by: by,
            updated_at: timestamp,
        }
    }

    /// 获取创建者
    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    /// 获取创建时间
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 获取修改者
    pub fn updated_by(&self) -> &str {
        &self.updated_by
    }

    /// 获取修改时间
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// 更新修改者和修改时间
    pub fn update(&mut self, updated_by: impl Into<String>) {
        self.updated_by = updated_by.into();
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_info_new() {
        let now = Utc::now();
        let audit = AuditInfo::new("user1".to_string(), now);

        assert_eq!(audit.created_by(), "user1");
        assert_eq!(audit.updated_by(), "user1");
        assert_eq!(audit.created_at(), now);
        assert_eq!(audit.updated_at(), now);
    }

    #[test]
    fn test_audit_info_system() {
        let audit = AuditInfo::system();

        assert_eq!(audit.created_by(), "SYSTEM");
        assert_eq!(audit.updated_by(), "SYSTEM");
    }

    #[test]
    fn test_audit_info_update() {
        let now = Utc::now();
        let mut audit = AuditInfo::new("user1".to_string(), now);

        audit.update("user2");

        assert_eq!(audit.created_by(), "user1");
        assert_eq!(audit.updated_by(), "user2");
        assert!(audit.updated_at() > audit.created_at());
    }
}
