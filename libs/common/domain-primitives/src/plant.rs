//! 工厂代码（Plant）
//!
//! 本模块实现工厂代码值对象。
//!
//! # SAP 参考
//! - 表: T001W（工厂主数据）
//! - 字段: WERKS（工厂代码，4 位）
//! - 字段: NAME1（工厂名称）
//! - 字段: BUKRS（公司代码）
//!
//! # 组织架构
//! 工厂是 SAP 物流组织的核心单元：
//! - 每个工厂关联一个公司代码
//! - 库存管理以工厂为基本单位
//! - 生产计划以工厂为执行单位
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::Plant;
//!
//! // 创建工厂代码
//! let plant = Plant::new("1001").unwrap();
//! assert_eq!(plant.code(), "1001");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 工厂代码长度
pub const PLANT_CODE_LENGTH: usize = 4;

/// 工厂
///
/// 表示一个工厂代码，是 SAP 物流组织的核心单元。
/// 这是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: T001W
/// - 字段: WERKS（4 位字母数字）
///
/// # 编码规则
/// - 长度: 4 位
/// - 字符: 字母和数字
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plant {
    /// 工厂代码
    /// 对应 T001W.WERKS
    code: String,

    /// 工厂名称（可选）
    /// 对应 T001W.NAME1
    name: Option<String>,

    /// 关联的公司代码（可选）
    /// 对应 T001W.BUKRS
    company_code: Option<String>,
}

impl Plant {
    /// 创建新的工厂代码
    ///
    /// # 参数
    /// - `code`: 工厂代码（4 位）
    ///
    /// # 错误
    /// - 代码不是 4 位
    /// - 代码包含非法字符
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Plant;
    ///
    /// let plant = Plant::new("1001").unwrap();
    /// assert_eq!(plant.code(), "1001");
    /// ```
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();

        // 验证长度
        if code.len() != PLANT_CODE_LENGTH {
            return Err(DomainError::plant_invalid(
                code,
                format!("工厂代码必须是 {} 位", PLANT_CODE_LENGTH),
            ));
        }

        // 验证字符
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::plant_invalid(
                code,
                "工厂代码只能包含字母和数字",
            ));
        }

        Ok(Self {
            code,
            name: None,
            company_code: None,
        })
    }

    /// 设置工厂名称
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置关联的公司代码
    pub fn with_company_code(mut self, company_code: impl Into<String>) -> Self {
        self.company_code = Some(company_code.into().to_uppercase());
        self
    }

    /// 获取工厂代码
    pub fn code(&self) -> &str {
        &self.code
    }

    /// 获取工厂名称
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// 获取关联的公司代码
    pub fn company_code(&self) -> Option<&str> {
        self.company_code.as_deref()
    }
}

impl PartialEq for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for Plant {}

impl Hash for Plant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl fmt::Display for Plant {
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
    fn test_create_plant() {
        let plant = Plant::new("1001").unwrap();
        assert_eq!(plant.code(), "1001");
    }

    #[test]
    fn test_create_with_letters() {
        let plant = Plant::new("WH01").unwrap();
        assert_eq!(plant.code(), "WH01");
    }

    #[test]
    fn test_create_with_lowercase() {
        let plant = Plant::new("wh01").unwrap();
        assert_eq!(plant.code(), "WH01");
    }

    #[test]
    fn test_invalid_length() {
        let result = Plant::new("100");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_chars() {
        let result = Plant::new("10-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_name() {
        let plant = Plant::new("1001")
            .unwrap()
            .with_name("上海工厂");

        assert_eq!(plant.name(), Some("上海工厂"));
    }

    #[test]
    fn test_with_company_code() {
        let plant = Plant::new("1001")
            .unwrap()
            .with_company_code("1000");

        assert_eq!(plant.company_code(), Some("1000"));
    }

    #[test]
    fn test_equality() {
        let a = Plant::new("1001").unwrap();
        let b = Plant::new("1001").unwrap().with_name("名称");
        let c = Plant::new("1002").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Plant::new("1001").unwrap());
        set.insert(Plant::new("1001").unwrap());
        set.insert(Plant::new("1002").unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let plant = Plant::new("1001").unwrap();
        assert_eq!(format!("{}", plant), "1001");
    }

    #[test]
    fn test_serialization() {
        let plant = Plant::new("1001").unwrap().with_name("上海工厂");
        let json = serde_json::to_string(&plant).unwrap();

        assert!(json.contains("\"code\":\"1001\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"code":"1001","name":"上海工厂","company_code":null}"#;
        let plant: Plant = serde_json::from_str(json).unwrap();

        assert_eq!(plant.code(), "1001");
        assert_eq!(plant.name(), Some("上海工厂"));
    }

    #[test]
    fn test_clone() {
        let original = Plant::new("1001").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
