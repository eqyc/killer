//! 物料编号（Material Number）
//!
//! 本模块实现物料编号值对象。
//!
//! # SAP 参考
//! - 表: MARA（物料主数据）
//! - 字段: MATNR（物料编号，18 位或 40 位）
//! - 新物料编号（S/4 HANA）支持 40 位字符
//!
//! # 编码规则
//! - 传统: 18 位，支持前导零
//! - S/4 HANA: 最多 40 位，支持字母数字
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::MaterialNumber;
//!
//! // 创建物料编号
//! let material = MaterialNumber::new("MAT-001").unwrap();
//!
//! // 带前导零的物料编号
//! let material2 = MaterialNumber::with_leading_zeros("1001", 18).unwrap();
//! assert_eq!(material2.number(), "000000000000001001");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 传统物料编号长度（SAP ECC）
pub const MATERIAL_NUMBER_LENGTH_CLASSIC: usize = 18;

/// 新物料编号最大长度（SAP S/4 HANA）
pub const MATERIAL_NUMBER_LENGTH_EXTENDED: usize = 40;

/// 物料编号
///
/// 表示一个物料的唯一标识，是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: MARA
/// - 字段: MATNR
///
/// # 编码规则
/// - 长度: 最多 40 位（S/4 HANA）
/// - 字符: 字母、数字、连字符、下划线
/// - 支持前导零
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialNumber {
    /// 物料编号
    /// 对应 MARA.MATNR
    number: String,
}

impl MaterialNumber {
    /// 创建新的物料编号
    ///
    /// # 参数
    /// - `number`: 物料编号
    ///
    /// # 错误
    /// - 编号为空
    /// - 编号超过 40 位
    /// - 编号包含非法字符
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::MaterialNumber;
    ///
    /// let material = MaterialNumber::new("MAT-001").unwrap();
    /// assert_eq!(material.number(), "MAT-001");
    /// ```
    pub fn new(number: impl Into<String>) -> DomainResult<Self> {
        let number = number.into().trim().to_uppercase();

        // 验证编号
        if number.is_empty() {
            return Err(DomainError::material_number_invalid(
                number,
                "物料编号不能为空",
            ));
        }

        if number.len() > MATERIAL_NUMBER_LENGTH_EXTENDED {
            return Err(DomainError::material_number_invalid(
                number,
                format!("物料编号长度不能超过 {} 位", MATERIAL_NUMBER_LENGTH_EXTENDED),
            ));
        }

        // 验证字符（允许字母、数字、连字符、下划线）
        if !number
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DomainError::material_number_invalid(
                number,
                "物料编号只能包含字母、数字、连字符和下划线",
            ));
        }

        Ok(Self { number })
    }

    /// 创建带前导零的物料编号
    ///
    /// # 参数
    /// - `number`: 物料编号（不含前导零）
    /// - `length`: 目标长度
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::MaterialNumber;
    ///
    /// let material = MaterialNumber::with_leading_zeros("1001", 18).unwrap();
    /// assert_eq!(material.number(), "000000000000001001");
    /// ```
    pub fn with_leading_zeros(number: impl Into<String>, length: usize) -> DomainResult<Self> {
        let number = number.into().trim().to_uppercase();

        // 只对纯数字编号添加前导零
        if number.chars().all(|c| c.is_ascii_digit()) {
            let padded = format!("{:0>width$}", number, width = length);
            Self::new(padded)
        } else {
            Self::new(number)
        }
    }

    /// 获取物料编号
    pub fn number(&self) -> &str {
        &self.number
    }

    /// 获取不含前导零的物料编号
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::MaterialNumber;
    ///
    /// let material = MaterialNumber::new("000000000000001001").unwrap();
    /// assert_eq!(material.without_leading_zeros(), "1001");
    /// ```
    pub fn without_leading_zeros(&self) -> &str {
        // 只对纯数字编号去除前导零
        if self.number.chars().all(|c| c.is_ascii_digit()) {
            let trimmed = self.number.trim_start_matches('0');
            if trimmed.is_empty() {
                "0"
            } else {
                trimmed
            }
        } else {
            &self.number
        }
    }

    /// 判断是否为纯数字编号
    pub fn is_numeric(&self) -> bool {
        self.number.chars().all(|c| c.is_ascii_digit())
    }

    /// 判断是否为传统格式（18 位）
    pub fn is_classic_format(&self) -> bool {
        self.number.len() <= MATERIAL_NUMBER_LENGTH_CLASSIC
    }

    /// 转换为传统格式（18 位，带前导零）
    ///
    /// # 错误
    /// - 编号超过 18 位
    pub fn to_classic_format(&self) -> DomainResult<Self> {
        if self.number.len() > MATERIAL_NUMBER_LENGTH_CLASSIC {
            return Err(DomainError::material_number_invalid(
                self.number.clone(),
                "编号超过 18 位，无法转换为传统格式",
            ));
        }

        if self.is_numeric() {
            Self::with_leading_zeros(&self.number, MATERIAL_NUMBER_LENGTH_CLASSIC)
        } else {
            Ok(Self {
                number: format!("{:>width$}", self.number, width = MATERIAL_NUMBER_LENGTH_CLASSIC),
            })
        }
    }
}

impl PartialEq for MaterialNumber {
    fn eq(&self, other: &Self) -> bool {
        // 比较时忽略前导零（仅对纯数字编号）
        self.without_leading_zeros() == other.without_leading_zeros()
    }
}

impl Eq for MaterialNumber {}

impl Hash for MaterialNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 使用去除前导零的编号计算哈希
        self.without_leading_zeros().hash(state);
    }
}

impl fmt::Display for MaterialNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_material_number() {
        let material = MaterialNumber::new("MAT-001").unwrap();
        assert_eq!(material.number(), "MAT-001");
    }

    #[test]
    fn test_create_numeric() {
        let material = MaterialNumber::new("1001").unwrap();
        assert_eq!(material.number(), "1001");
        assert!(material.is_numeric());
    }

    #[test]
    fn test_create_with_lowercase() {
        let material = MaterialNumber::new("mat-001").unwrap();
        assert_eq!(material.number(), "MAT-001");
    }

    #[test]
    fn test_create_with_whitespace() {
        let material = MaterialNumber::new("  MAT-001  ").unwrap();
        assert_eq!(material.number(), "MAT-001");
    }

    #[test]
    fn test_invalid_empty() {
        let result = MaterialNumber::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_too_long() {
        let long_number = "A".repeat(41);
        let result = MaterialNumber::new(long_number);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_special_chars() {
        let result = MaterialNumber::new("MAT@001");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_with_underscore() {
        let material = MaterialNumber::new("MAT_001").unwrap();
        assert_eq!(material.number(), "MAT_001");
    }

    #[test]
    fn test_with_leading_zeros() {
        let material = MaterialNumber::with_leading_zeros("1001", 18).unwrap();
        assert_eq!(material.number(), "000000000000001001");
    }

    #[test]
    fn test_with_leading_zeros_alphanumeric() {
        // 字母数字编号不添加前导零
        let material = MaterialNumber::with_leading_zeros("MAT-001", 18).unwrap();
        assert_eq!(material.number(), "MAT-001");
    }

    #[test]
    fn test_without_leading_zeros() {
        let material = MaterialNumber::new("000000000000001001").unwrap();
        assert_eq!(material.without_leading_zeros(), "1001");
    }

    #[test]
    fn test_without_leading_zeros_all_zeros() {
        let material = MaterialNumber::new("0000").unwrap();
        assert_eq!(material.without_leading_zeros(), "0");
    }

    #[test]
    fn test_without_leading_zeros_alphanumeric() {
        let material = MaterialNumber::new("MAT-001").unwrap();
        assert_eq!(material.without_leading_zeros(), "MAT-001");
    }

    #[test]
    fn test_is_numeric() {
        let numeric = MaterialNumber::new("1001").unwrap();
        let alphanumeric = MaterialNumber::new("MAT-001").unwrap();

        assert!(numeric.is_numeric());
        assert!(!alphanumeric.is_numeric());
    }

    #[test]
    fn test_is_classic_format() {
        let classic = MaterialNumber::new("1001").unwrap();
        let extended = MaterialNumber::new("A".repeat(20)).unwrap();

        assert!(classic.is_classic_format());
        assert!(!extended.is_classic_format());
    }

    #[test]
    fn test_to_classic_format() {
        let material = MaterialNumber::new("1001").unwrap();
        let classic = material.to_classic_format().unwrap();

        assert_eq!(classic.number(), "000000000000001001");
    }

    #[test]
    fn test_to_classic_format_too_long() {
        let material = MaterialNumber::new("A".repeat(20)).unwrap();
        let result = material.to_classic_format();

        assert!(result.is_err());
    }

    #[test]
    fn test_equality_same_number() {
        let a = MaterialNumber::new("1001").unwrap();
        let b = MaterialNumber::new("1001").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_equality_with_leading_zeros() {
        let a = MaterialNumber::new("1001").unwrap();
        let b = MaterialNumber::new("000000000000001001").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_hash_with_leading_zeros() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(MaterialNumber::new("1001").unwrap());
        set.insert(MaterialNumber::new("000000000000001001").unwrap());

        assert_eq!(set.len(), 1); // 应该视为相同
    }

    #[test]
    fn test_display() {
        let material = MaterialNumber::new("MAT-001").unwrap();
        assert_eq!(format!("{}", material), "MAT-001");
    }

    #[test]
    fn test_serialization() {
        let material = MaterialNumber::new("MAT-001").unwrap();
        let json = serde_json::to_string(&material).unwrap();

        assert!(json.contains("\"number\":\"MAT-001\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"number":"MAT-001"}"#;
        let material: MaterialNumber = serde_json::from_str(json).unwrap();

        assert_eq!(material.number(), "MAT-001");
    }

    #[test]
    fn test_clone() {
        let original = MaterialNumber::new("MAT-001").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
