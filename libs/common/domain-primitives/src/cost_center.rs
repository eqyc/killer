//! 成本中心（Cost Center）
//!
//! 本模块实现成本中心值对象。
//!
//! # SAP 参考
//! - 表: CSKS（成本中心主数据）
//! - 表: CSKT（成本中心文本）
//! - 字段: KOSTL（成本中心，10 位）
//! - 字段: KOKRS（控制范围，4 位）
//! - 字段: KTEXT（成本中心描述）
//!
//! # 组织架构
//! 成本中心是 SAP 管理会计（CO）的核心对象：
//! - 用于归集和分析成本
//! - 每个成本中心属于一个控制范围
//! - 成本中心可以分层组织
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::CostCenter;
//!
//! // 创建成本中心
//! let cc = CostCenter::new("1000100001", "1000").unwrap();
//! assert_eq!(cc.code(), "1000100001");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 成本中心代码最大长度
pub const COST_CENTER_MAX_LENGTH: usize = 10;

/// 控制范围代码长度
pub const CONTROLLING_AREA_LENGTH: usize = 4;

/// 成本中心
///
/// 表示一个成本中心，是 SAP 管理会计的核心对象。
/// 这是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: CSKS
/// - 字段: KOSTL（10 位字母数字）
/// - 字段: KOKRS（4 位控制范围）
///
/// # 编码规则
/// - 代码长度: 最多 10 位
/// - 控制范围: 4 位
/// - 字符: 字母和数字
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCenter {
    /// 成本中心代码
    /// 对应 CSKS.KOSTL
    code: String,

    /// 控制范围
    /// 对应 CSKS.KOKRS
    controlling_area: String,

    /// 成本中心描述（可选）
    /// 对应 CSKT.KTEXT
    description: Option<String>,
}

impl CostCenter {
    /// 创建新的成本中心
    ///
    /// # 参数
    /// - `code`: 成本中心代码（最多 10 位）
    /// - `controlling_area`: 控制范围代码（4 位）
    ///
    /// # 错误
    /// - 代码为空或超过 10 位
    /// - 控制范围不是 4 位
    /// - 代码包含非法字符
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::CostCenter;
    ///
    /// let cc = CostCenter::new("1000100001", "1000").unwrap();
    /// ```
    pub fn new(code: impl Into<String>, controlling_area: impl Into<String>) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();
        let controlling_area = controlling_area.into().trim().to_uppercase();

        // 验证成本中心代码
        if code.is_empty() {
            return Err(DomainError::cost_center_invalid(
                code,
                "成本中心代码不能为空",
            ));
        }

        if code.len() > COST_CENTER_MAX_LENGTH {
            return Err(DomainError::cost_center_invalid(
                code,
                format!("成本中心代码长度不能超过 {} 位", COST_CENTER_MAX_LENGTH),
            ));
        }

        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::cost_center_invalid(
                code,
                "成本中心代码只能包含字母和数字",
            ));
        }

        // 验证控制范围
        if controlling_area.len() != CONTROLLING_AREA_LENGTH {
            return Err(DomainError::cost_center_invalid(
                code,
                format!("控制范围必须是 {} 位", CONTROLLING_AREA_LENGTH),
            ));
        }

        if !controlling_area.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::cost_center_invalid(
                code,
                "控制范围只能包含字母和数字",
            ));
        }

        Ok(Self {
            code,
            controlling_area,
            description: None,
        })
    }

    /// 创建带前导零的成本中心
    ///
    /// # 参数
    /// - `code`: 成本中心代码（不含前导零）
    /// - `controlling_area`: 控制范围
    /// - `length`: 目标长度
    pub fn with_leading_zeros(
        code: impl Into<String>,
        controlling_area: impl Into<String>,
        length: usize,
    ) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();
        let padded = format!("{:0>width$}", code, width = length);
        Self::new(padded, controlling_area)
    }

    /// 设置成本中心描述
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 获取成本中心代码
    pub fn code(&self) -> &str {
        &self.code
    }

    /// 获取控制范围
    pub fn controlling_area(&self) -> &str {
        &self.controlling_area
    }

    /// 获取成本中心描述
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// 获取不含前导零的代码
    pub fn without_leading_zeros(&self) -> &str {
        self.code.trim_start_matches('0')
    }

    /// 判断两个成本中心是否属于同一控制范围
    pub fn same_controlling_area(&self, other: &CostCenter) -> bool {
        self.controlling_area == other.controlling_area
    }
}

impl PartialEq for CostCenter {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.controlling_area == other.controlling_area
    }
}

impl Eq for CostCenter {}

impl Hash for CostCenter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        self.controlling_area.hash(state);
    }
}

impl fmt::Display for CostCenter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cost_center() {
        let cc = CostCenter::new("1000100001", "1000").unwrap();
        assert_eq!(cc.code(), "1000100001");
        assert_eq!(cc.controlling_area(), "1000");
    }

    #[test]
    fn test_create_with_lowercase() {
        let cc = CostCenter::new("abc123", "abcd").unwrap();
        assert_eq!(cc.code(), "ABC123");
        assert_eq!(cc.controlling_area(), "ABCD");
    }

    #[test]
    fn test_invalid_empty_code() {
        let result = CostCenter::new("", "1000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_code_too_long() {
        let result = CostCenter::new("12345678901", "1000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_controlling_area() {
        let result = CostCenter::new("1001", "100");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_leading_zeros() {
        let cc = CostCenter::with_leading_zeros("1001", "1000", 10).unwrap();
        assert_eq!(cc.code(), "0000001001");
    }

    #[test]
    fn test_without_leading_zeros() {
        let cc = CostCenter::new("0000001001", "1000").unwrap();
        assert_eq!(cc.without_leading_zeros(), "1001");
    }

    #[test]
    fn test_with_description() {
        let cc = CostCenter::new("1001", "1000")
            .unwrap()
            .with_description("生产部门");

        assert_eq!(cc.description(), Some("生产部门"));
    }

    #[test]
    fn test_same_controlling_area() {
        let a = CostCenter::new("1001", "1000").unwrap();
        let b = CostCenter::new("1002", "1000").unwrap();
        let c = CostCenter::new("1001", "2000").unwrap();

        assert!(a.same_controlling_area(&b));
        assert!(!a.same_controlling_area(&c));
    }

    #[test]
    fn test_equality() {
        let a = CostCenter::new("1001", "1000").unwrap();
        let b = CostCenter::new("1001", "1000").unwrap();
        let c = CostCenter::new("1001", "2000").unwrap();
        let d = CostCenter::new("1002", "1000").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CostCenter::new("1001", "1000").unwrap());
        set.insert(CostCenter::new("1001", "1000").unwrap());
        set.insert(CostCenter::new("1002", "1000").unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let cc = CostCenter::new("1000100001", "1000").unwrap();
        assert_eq!(format!("{}", cc), "1000100001");
    }

    #[test]
    fn test_serialization() {
        let cc = CostCenter::new("1001", "1000")
            .unwrap()
            .with_description("生产部门");
        let json = serde_json::to_string(&cc).unwrap();

        assert!(json.contains("\"code\":\"1001\""));
        assert!(json.contains("\"controlling_area\":\"1000\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"code":"1001","controlling_area":"1000","description":null}"#;
        let cc: CostCenter = serde_json::from_str(json).unwrap();

        assert_eq!(cc.code(), "1001");
        assert_eq!(cc.controlling_area(), "1000");
    }

    #[test]
    fn test_clone() {
        let original = CostCenter::new("1001", "1000").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
