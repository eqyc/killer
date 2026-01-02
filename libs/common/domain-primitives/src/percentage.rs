//! 百分比（Percentage）
//!
//! 本模块实现百分比值对象。
//!
//! # 用途
//! 百分比在 ERP 系统中广泛使用：
//! - 税率（如增值税 13%）
//! - 折扣率（如 10% 折扣）
//! - 分配比例（如成本分摊）
//! - 完成进度（如项目进度 80%）
//!
//! # 精度
//! 默认精度为 2 位小数，支持 0.01% 的精度
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::Percentage;
//! use rust_decimal_macros::dec;
//!
//! // 创建百分比
//! let tax_rate = Percentage::new(dec!(13)).unwrap();
//!
//! // 计算
//! let base = dec!(100);
//! let tax = tax_rate.apply_to(base);
//! assert_eq!(tax, dec!(13));
//! ```

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 百分比精度：2 位小数
pub const PERCENTAGE_SCALE: u32 = 2;

/// 百分比
///
/// 表示一个百分比值，是一个不可变的值对象。
///
/// # 存储方式
/// 内部存储百分比值（如 13 表示 13%），而非小数形式（0.13）
///
/// # 精度
/// 默认 2 位小数，支持 0.01% 的精度
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Percentage {
    /// 百分比值（如 13 表示 13%）
    #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,
}

impl Percentage {
    /// 创建新的百分比
    ///
    /// # 参数
    /// - `value`: 百分比值（如 13 表示 13%）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::new(dec!(13.5)).unwrap();
    /// assert_eq!(rate.value(), dec!(13.5));
    /// ```
    pub fn new(value: Decimal) -> DomainResult<Self> {
        let rounded = value.round_dp_with_strategy(PERCENTAGE_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self { value: rounded })
    }

    /// 创建零百分比
    pub fn zero() -> Self {
        Self {
            value: Decimal::ZERO,
        }
    }

    /// 创建 100%
    pub fn hundred() -> Self {
        Self {
            value: dec!(100),
        }
    }

    /// 从小数创建百分比
    ///
    /// # 参数
    /// - `decimal`: 小数形式（如 0.13 表示 13%）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::from_decimal(dec!(0.13)).unwrap();
    /// assert_eq!(rate.value(), dec!(13));
    /// ```
    pub fn from_decimal(decimal: Decimal) -> DomainResult<Self> {
        Self::new(decimal * dec!(100))
    }

    /// 从字符串解析百分比
    ///
    /// # 参数
    /// - `value_str`: 百分比字符串（如 "13.5"）
    pub fn from_str(value_str: &str) -> DomainResult<Self> {
        let value = Decimal::from_str(value_str).map_err(|_| {
            DomainError::percentage_invalid(format!("无效的百分比格式: {}", value_str))
        })?;
        Self::new(value)
    }

    /// 获取百分比值
    pub fn value(&self) -> Decimal {
        self.value
    }

    /// 转换为小数形式
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::new(dec!(13)).unwrap();
    /// assert_eq!(rate.to_decimal(), dec!(0.13));
    /// ```
    pub fn to_decimal(&self) -> Decimal {
        self.value / dec!(100)
    }

    /// 判断是否为零
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// 判断是否为正
    pub fn is_positive(&self) -> bool {
        self.value > Decimal::ZERO
    }

    /// 判断是否为负
    pub fn is_negative(&self) -> bool {
        self.value < Decimal::ZERO
    }

    /// 判断是否为 100%
    pub fn is_hundred(&self) -> bool {
        self.value == dec!(100)
    }

    /// 判断是否在 0-100% 范围内
    pub fn is_in_range(&self) -> bool {
        self.value >= Decimal::ZERO && self.value <= dec!(100)
    }

    /// 应用百分比到基数
    ///
    /// # 参数
    /// - `base`: 基数
    ///
    /// # 返回
    /// base * percentage / 100
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::new(dec!(13)).unwrap();
    /// let result = rate.apply_to(dec!(100));
    /// assert_eq!(result, dec!(13));
    /// ```
    pub fn apply_to(&self, base: Decimal) -> Decimal {
        base * self.value / dec!(100)
    }

    /// 计算基数加上百分比
    ///
    /// # 参数
    /// - `base`: 基数
    ///
    /// # 返回
    /// base * (1 + percentage / 100)
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::new(dec!(10)).unwrap();
    /// let result = rate.add_to(dec!(100));
    /// assert_eq!(result, dec!(110));
    /// ```
    pub fn add_to(&self, base: Decimal) -> Decimal {
        base + self.apply_to(base)
    }

    /// 计算基数减去百分比
    ///
    /// # 参数
    /// - `base`: 基数
    ///
    /// # 返回
    /// base * (1 - percentage / 100)
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let discount = Percentage::new(dec!(10)).unwrap();
    /// let result = discount.subtract_from(dec!(100));
    /// assert_eq!(result, dec!(90));
    /// ```
    pub fn subtract_from(&self, base: Decimal) -> Decimal {
        base - self.apply_to(base)
    }

    /// 百分比相加
    pub fn add(&self, other: &Percentage) -> DomainResult<Self> {
        Self::new(self.value + other.value)
    }

    /// 百分比相减
    pub fn subtract(&self, other: &Percentage) -> DomainResult<Self> {
        Self::new(self.value - other.value)
    }

    /// 取反
    pub fn negate(&self) -> Self {
        Self {
            value: -self.value,
        }
    }

    /// 取绝对值
    pub fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    /// 计算补数（100% - 当前值）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::Percentage;
    /// use rust_decimal_macros::dec;
    ///
    /// let rate = Percentage::new(dec!(30)).unwrap();
    /// let complement = rate.complement().unwrap();
    /// assert_eq!(complement.value(), dec!(70));
    /// ```
    pub fn complement(&self) -> DomainResult<Self> {
        Self::new(dec!(100) - self.value)
    }
}

impl PartialEq for Percentage {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Percentage {}

impl Hash for Percentage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_string().hash(state);
    }
}

impl PartialOrd for Percentage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Percentage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.value)
    }
}

// ============================================================================
// 常用百分比常量
// ============================================================================

impl Percentage {
    /// 中国增值税标准税率 13%
    pub fn vat_standard_cn() -> Self {
        Self { value: dec!(13) }
    }

    /// 中国增值税低税率 9%
    pub fn vat_reduced_cn() -> Self {
        Self { value: dec!(9) }
    }

    /// 中国增值税低税率 6%
    pub fn vat_low_cn() -> Self {
        Self { value: dec!(6) }
    }

    /// 中国小规模纳税人税率 3%
    pub fn vat_small_scale_cn() -> Self {
        Self { value: dec!(3) }
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_percentage() {
        let rate = Percentage::new(dec!(13.5)).unwrap();
        assert_eq!(rate.value(), dec!(13.5));
    }

    #[test]
    fn test_percentage_precision() {
        let rate = Percentage::new(dec!(13.555)).unwrap();
        assert_eq!(rate.value(), dec!(13.56)); // 舍入到 2 位
    }

    #[test]
    fn test_zero() {
        let zero = Percentage::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.value(), dec!(0));
    }

    #[test]
    fn test_hundred() {
        let hundred = Percentage::hundred();
        assert!(hundred.is_hundred());
        assert_eq!(hundred.value(), dec!(100));
    }

    #[test]
    fn test_from_decimal() {
        let rate = Percentage::from_decimal(dec!(0.13)).unwrap();
        assert_eq!(rate.value(), dec!(13));
    }

    #[test]
    fn test_from_str() {
        let rate = Percentage::from_str("13.5").unwrap();
        assert_eq!(rate.value(), dec!(13.5));
    }

    #[test]
    fn test_from_str_invalid() {
        let result = Percentage::from_str("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_decimal() {
        let rate = Percentage::new(dec!(13)).unwrap();
        assert_eq!(rate.to_decimal(), dec!(0.13));
    }

    #[test]
    fn test_is_positive() {
        let positive = Percentage::new(dec!(10)).unwrap();
        let negative = Percentage::new(dec!(-10)).unwrap();
        let zero = Percentage::zero();

        assert!(positive.is_positive());
        assert!(!negative.is_positive());
        assert!(!zero.is_positive());
    }

    #[test]
    fn test_is_in_range() {
        let in_range = Percentage::new(dec!(50)).unwrap();
        let below = Percentage::new(dec!(-10)).unwrap();
        let above = Percentage::new(dec!(110)).unwrap();

        assert!(in_range.is_in_range());
        assert!(!below.is_in_range());
        assert!(!above.is_in_range());
    }

    #[test]
    fn test_apply_to() {
        let rate = Percentage::new(dec!(13)).unwrap();
        let result = rate.apply_to(dec!(100));

        assert_eq!(result, dec!(13));
    }

    #[test]
    fn test_add_to() {
        let rate = Percentage::new(dec!(10)).unwrap();
        let result = rate.add_to(dec!(100));

        assert_eq!(result, dec!(110));
    }

    #[test]
    fn test_subtract_from() {
        let discount = Percentage::new(dec!(10)).unwrap();
        let result = discount.subtract_from(dec!(100));

        assert_eq!(result, dec!(90));
    }

    #[test]
    fn test_add() {
        let a = Percentage::new(dec!(10)).unwrap();
        let b = Percentage::new(dec!(5)).unwrap();
        let sum = a.add(&b).unwrap();

        assert_eq!(sum.value(), dec!(15));
    }

    #[test]
    fn test_subtract() {
        let a = Percentage::new(dec!(10)).unwrap();
        let b = Percentage::new(dec!(3)).unwrap();
        let diff = a.subtract(&b).unwrap();

        assert_eq!(diff.value(), dec!(7));
    }

    #[test]
    fn test_negate() {
        let rate = Percentage::new(dec!(10)).unwrap();
        let negated = rate.negate();

        assert_eq!(negated.value(), dec!(-10));
    }

    #[test]
    fn test_abs() {
        let negative = Percentage::new(dec!(-10)).unwrap();
        let absolute = negative.abs();

        assert_eq!(absolute.value(), dec!(10));
    }

    #[test]
    fn test_complement() {
        let rate = Percentage::new(dec!(30)).unwrap();
        let complement = rate.complement().unwrap();

        assert_eq!(complement.value(), dec!(70));
    }

    #[test]
    fn test_equality() {
        let a = Percentage::new(dec!(10)).unwrap();
        let b = Percentage::new(dec!(10)).unwrap();
        let c = Percentage::new(dec!(20)).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_ordering() {
        let small = Percentage::new(dec!(10)).unwrap();
        let large = Percentage::new(dec!(20)).unwrap();

        assert!(small < large);
        assert!(large > small);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Percentage::new(dec!(10)).unwrap());
        set.insert(Percentage::new(dec!(10)).unwrap());
        set.insert(Percentage::new(dec!(20)).unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let rate = Percentage::new(dec!(13.5)).unwrap();
        assert_eq!(format!("{}", rate), "13.5%");
    }

    #[test]
    fn test_vat_constants() {
        assert_eq!(Percentage::vat_standard_cn().value(), dec!(13));
        assert_eq!(Percentage::vat_reduced_cn().value(), dec!(9));
        assert_eq!(Percentage::vat_low_cn().value(), dec!(6));
        assert_eq!(Percentage::vat_small_scale_cn().value(), dec!(3));
    }

    #[test]
    fn test_serialization() {
        let rate = Percentage::new(dec!(13.5)).unwrap();
        let json = serde_json::to_string(&rate).unwrap();

        assert!(json.contains("\"value\":\"13.5\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"value":"13.5"}"#;
        let rate: Percentage = serde_json::from_str(json).unwrap();

        assert_eq!(rate.value(), dec!(13.5));
    }

    #[test]
    fn test_clone() {
        let original = Percentage::new(dec!(10)).unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
