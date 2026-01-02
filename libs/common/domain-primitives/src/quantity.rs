//! 数量（Quantity）
//!
//! 本模块实现数量值对象，用于处理带计量单位的数量计算。
//!
//! # SAP 参考
//! - 表: EKPO（采购订单行项目）
//! - 字段: MENGE（数量）, MEINS（计量单位）
//! - 表: MSEG（物料凭证行项目）
//! - 字段: MENGE（数量，可为负数表示出库）
//! - 数据类型: QUAN（数量，3 位小数）
//!
//! # 精度处理
//! SAP 标准数量精度为 3 位小数，本模块遵循此标准。
//!
//! # 负数支持
//! 数量可以为负数，用于表示：
//! - 退货数量
//! - 出库数量
//! - 冲销数量
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::{Quantity, UnitOfMeasure};
//! use rust_decimal_macros::dec;
//!
//! // 创建数量
//! let qty = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
//!
//! // 单位换算
//! let qty_g = qty.convert_to(&UnitOfMeasure::gram()).unwrap();
//! assert_eq!(qty_g.value(), dec!(100000));
//! ```

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};
use crate::unit_of_measure::UnitOfMeasure;

/// 数量精度：3 位小数
///
/// SAP 标准数量精度
pub const QUANTITY_SCALE: u32 = 3;

/// 数量
///
/// 表示一个带计量单位的数量，是一个不可变的值对象。
/// 支持单位换算和基本算术运算。
///
/// # SAP 参考
/// - 数据类型: QUAN（数量）
/// - 精度: 3 位小数
/// - 表: EKPO.MENGE, MSEG.MENGE
///
/// # 不变性
/// - 数量精度不超过 3 位小数
/// - 必须关联有效的计量单位
/// - 同单位或可换算单位才能进行加减运算
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    /// 数量值
    /// 可以为负数（退货、出库等场景）
    #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,

    /// 计量单位
    unit: UnitOfMeasure,
}

impl Quantity {
    /// 创建新的数量
    ///
    /// # 参数
    /// - `value`: 数量值
    /// - `unit`: 计量单位
    ///
    /// # 返回
    /// 创建的 Quantity 实例，数量会自动舍入到 3 位小数
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Quantity, UnitOfMeasure};
    /// use rust_decimal_macros::dec;
    ///
    /// let qty = Quantity::new(dec!(100.123), UnitOfMeasure::kilogram()).unwrap();
    /// assert_eq!(qty.value(), dec!(100.123));
    /// ```
    pub fn new(value: Decimal, unit: UnitOfMeasure) -> DomainResult<Self> {
        let rounded = value.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            value: rounded,
            unit,
        })
    }

    /// 创建零数量
    ///
    /// # 参数
    /// - `unit`: 计量单位
    pub fn zero(unit: UnitOfMeasure) -> Self {
        Self {
            value: Decimal::ZERO,
            unit,
        }
    }

    /// 从字符串解析数量
    ///
    /// # 参数
    /// - `value_str`: 数量字符串
    /// - `unit`: 计量单位
    ///
    /// # 错误
    /// - 数量字符串格式无效
    pub fn from_str(value_str: &str, unit: UnitOfMeasure) -> DomainResult<Self> {
        let value = Decimal::from_str(value_str).map_err(|_| {
            DomainError::quantity_invalid_value(format!("无效的数量格式: {}", value_str))
        })?;
        Self::new(value, unit)
    }

    /// 获取数量值
    pub fn value(&self) -> Decimal {
        self.value
    }

    /// 获取计量单位
    pub fn unit(&self) -> &UnitOfMeasure {
        &self.unit
    }

    /// 判断数量是否为零
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// 判断数量是否为正
    pub fn is_positive(&self) -> bool {
        self.value > Decimal::ZERO
    }

    /// 判断数量是否为负
    ///
    /// 负数量通常表示退货、出库或冲销
    pub fn is_negative(&self) -> bool {
        self.value < Decimal::ZERO
    }

    /// 数量取反
    ///
    /// 用于生成冲销数量
    pub fn negate(&self) -> Self {
        Self {
            value: -self.value,
            unit: self.unit.clone(),
        }
    }

    /// 获取绝对值
    pub fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
            unit: self.unit.clone(),
        }
    }

    /// 数量相加
    ///
    /// 如果单位不同但可换算，会先将 other 换算到 self 的单位
    ///
    /// # 错误
    /// - 单位不兼容（不同维度）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Quantity, UnitOfMeasure};
    /// use rust_decimal_macros::dec;
    ///
    /// let a = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
    /// let b = Quantity::new(dec!(500), UnitOfMeasure::gram()).unwrap();
    /// let sum = a.add(&b).unwrap();
    /// assert_eq!(sum.value(), dec!(1.5)); // 1 KG + 500 G = 1.5 KG
    /// ```
    pub fn add(&self, other: &Quantity) -> DomainResult<Self> {
        let converted = other.convert_to(&self.unit)?;
        let sum = self.value + converted.value;
        let rounded = sum.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            value: rounded,
            unit: self.unit.clone(),
        })
    }

    /// 数量相减
    ///
    /// 如果单位不同但可换算，会先将 other 换算到 self 的单位
    ///
    /// # 错误
    /// - 单位不兼容（不同维度）
    pub fn subtract(&self, other: &Quantity) -> DomainResult<Self> {
        let converted = other.convert_to(&self.unit)?;
        let diff = self.value - converted.value;
        let rounded = diff.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            value: rounded,
            unit: self.unit.clone(),
        })
    }

    /// 数量乘法
    ///
    /// 用于计算总量等场景
    ///
    /// # 参数
    /// - `multiplier`: 乘数
    pub fn multiply(&self, multiplier: Decimal) -> Self {
        let result = self.value * multiplier;
        let rounded = result.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);
        Self {
            value: rounded,
            unit: self.unit.clone(),
        }
    }

    /// 数量除法
    ///
    /// # 参数
    /// - `divisor`: 除数
    ///
    /// # 错误
    /// - 除数为零
    pub fn divide(&self, divisor: Decimal) -> DomainResult<Self> {
        if divisor.is_zero() {
            return Err(DomainError::quantity_invalid_operation("除数不能为零"));
        }
        let result = self.value / divisor;
        let rounded = result.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            value: rounded,
            unit: self.unit.clone(),
        })
    }

    /// 单位换算
    ///
    /// 将数量换算到目标单位
    ///
    /// # 参数
    /// - `target_unit`: 目标计量单位
    ///
    /// # 错误
    /// - 单位不兼容（不同维度）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Quantity, UnitOfMeasure};
    /// use rust_decimal_macros::dec;
    ///
    /// let kg = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
    /// let g = kg.convert_to(&UnitOfMeasure::gram()).unwrap();
    /// assert_eq!(g.value(), dec!(1000));
    /// assert_eq!(g.unit().code(), "G");
    /// ```
    pub fn convert_to(&self, target_unit: &UnitOfMeasure) -> DomainResult<Self> {
        // 如果单位相同，直接返回克隆
        if self.unit == *target_unit {
            return Ok(self.clone());
        }

        // 获取换算因子
        let factor = self.unit.conversion_factor_to(target_unit)?;
        let converted_value = self.value * factor;
        let rounded = converted_value.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::MidpointNearestEven);

        Ok(Self {
            value: rounded,
            unit: target_unit.clone(),
        })
    }

    /// 检查是否可以换算到目标单位
    pub fn can_convert_to(&self, target_unit: &UnitOfMeasure) -> bool {
        self.unit.can_convert_to(target_unit)
    }

    /// 舍入到指定小数位
    ///
    /// # 参数
    /// - `scale`: 小数位数
    pub fn round(&self, scale: u32) -> Self {
        let rounded = self.value.round_dp_with_strategy(scale, RoundingStrategy::MidpointNearestEven);
        Self {
            value: rounded,
            unit: self.unit.clone(),
        }
    }

    /// 分配数量
    ///
    /// 将数量按比例分配到多个部分，确保总和等于原数量。
    ///
    /// # 参数
    /// - `ratios`: 分配比例
    ///
    /// # 错误
    /// - 比例为空
    /// - 所有比例都为零
    pub fn allocate(&self, ratios: &[Decimal]) -> DomainResult<Vec<Self>> {
        if ratios.is_empty() {
            return Err(DomainError::quantity_invalid_operation("分配比例不能为空"));
        }

        let total_ratio: Decimal = ratios.iter().sum();
        if total_ratio.is_zero() {
            return Err(DomainError::quantity_invalid_operation("分配比例总和不能为零"));
        }

        // 计算每个部分的数量
        let mut results: Vec<(Decimal, Decimal)> = ratios
            .iter()
            .map(|ratio| {
                let exact = self.value * (*ratio) / total_ratio;
                let rounded = exact.round_dp_with_strategy(QUANTITY_SCALE, RoundingStrategy::ToZero);
                let remainder = exact - rounded;
                (rounded, remainder)
            })
            .collect();

        // 计算舍入误差
        let allocated_sum: Decimal = results.iter().map(|(r, _)| *r).sum();
        let mut remaining = self.value - allocated_sum;

        // 使用最大余数法分配剩余数量
        let unit = Decimal::new(1, QUANTITY_SCALE);
        while remaining >= unit {
            let max_idx = results
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap();

            results[max_idx].0 += unit;
            results[max_idx].1 = Decimal::ZERO;
            remaining -= unit;
        }

        Ok(results
            .into_iter()
            .map(|(value, _)| Self {
                value,
                unit: self.unit.clone(),
            })
            .collect())
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        // 如果单位相同，直接比较值
        if self.unit == other.unit {
            return self.value == other.value;
        }

        // 如果单位可换算，换算后比较
        if let Ok(converted) = other.convert_to(&self.unit) {
            self.value == converted.value
        } else {
            false
        }
    }
}

impl Eq for Quantity {}

impl Hash for Quantity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 注意：由于 PartialEq 考虑了单位换算，Hash 实现需要谨慎
        // 这里使用单位代码和值的字符串表示
        self.value.to_string().hash(state);
        self.unit.code().hash(state);
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 如果单位相同，直接比较
        if self.unit == other.unit {
            return self.value.partial_cmp(&other.value);
        }

        // 如果单位可换算，换算后比较
        if let Ok(converted) = other.convert_to(&self.unit) {
            self.value.partial_cmp(&converted.value)
        } else {
            None
        }
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.code())
    }
}

// ============================================================================
// 常用单位快捷方法
// ============================================================================

impl Quantity {
    /// 创建千克数量
    pub fn kg(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::kilogram())
    }

    /// 创建克数量
    pub fn g(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::gram())
    }

    /// 创建米数量
    pub fn m(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::meter())
    }

    /// 创建升数量
    pub fn l(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::liter())
    }

    /// 创建件数量
    pub fn pc(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::piece())
    }

    /// 创建小时数量
    pub fn hour(value: Decimal) -> DomainResult<Self> {
        Self::new(value, UnitOfMeasure::hour())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_quantity() {
        let qty = Quantity::new(dec!(100.5), UnitOfMeasure::kilogram()).unwrap();
        assert_eq!(qty.value(), dec!(100.5));
        assert_eq!(qty.unit().code(), "KG");
    }

    #[test]
    fn test_quantity_precision() {
        // 超过 3 位小数会被舍入
        let qty = Quantity::new(dec!(100.12345), UnitOfMeasure::kilogram()).unwrap();
        assert_eq!(qty.value(), dec!(100.123));
    }

    #[test]
    fn test_quantity_zero() {
        let zero = Quantity::zero(UnitOfMeasure::kilogram());
        assert!(zero.is_zero());
        assert_eq!(zero.value(), dec!(0));
    }

    #[test]
    fn test_quantity_from_str() {
        let qty = Quantity::from_str("123.45", UnitOfMeasure::kilogram()).unwrap();
        assert_eq!(qty.value(), dec!(123.45));
    }

    #[test]
    fn test_quantity_from_str_invalid() {
        let result = Quantity::from_str("abc", UnitOfMeasure::kilogram());
        assert!(result.is_err());
    }

    #[test]
    fn test_is_positive() {
        let positive = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let negative = Quantity::new(dec!(-100), UnitOfMeasure::kilogram()).unwrap();
        let zero = Quantity::zero(UnitOfMeasure::kilogram());

        assert!(positive.is_positive());
        assert!(!negative.is_positive());
        assert!(!zero.is_positive());
    }

    #[test]
    fn test_is_negative() {
        let positive = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let negative = Quantity::new(dec!(-100), UnitOfMeasure::kilogram()).unwrap();

        assert!(!positive.is_negative());
        assert!(negative.is_negative());
    }

    #[test]
    fn test_negate() {
        let qty = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let negated = qty.negate();

        assert_eq!(negated.value(), dec!(-100));
    }

    #[test]
    fn test_abs() {
        let negative = Quantity::new(dec!(-100), UnitOfMeasure::kilogram()).unwrap();
        let absolute = negative.abs();

        assert_eq!(absolute.value(), dec!(100));
    }

    #[test]
    fn test_add_same_unit() {
        let a = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let b = Quantity::new(dec!(50), UnitOfMeasure::kilogram()).unwrap();
        let sum = a.add(&b).unwrap();

        assert_eq!(sum.value(), dec!(150));
        assert_eq!(sum.unit().code(), "KG");
    }

    #[test]
    fn test_add_different_unit_same_dimension() {
        let kg = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
        let g = Quantity::new(dec!(500), UnitOfMeasure::gram()).unwrap();
        let sum = kg.add(&g).unwrap();

        assert_eq!(sum.value(), dec!(1.5)); // 1 KG + 500 G = 1.5 KG
        assert_eq!(sum.unit().code(), "KG");
    }

    #[test]
    fn test_add_incompatible_units() {
        let kg = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let m = Quantity::new(dec!(100), UnitOfMeasure::meter()).unwrap();

        let result = kg.add(&m);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract() {
        let a = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let b = Quantity::new(dec!(30), UnitOfMeasure::kilogram()).unwrap();
        let diff = a.subtract(&b).unwrap();

        assert_eq!(diff.value(), dec!(70));
    }

    #[test]
    fn test_multiply() {
        let qty = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = qty.multiply(dec!(2.5));

        assert_eq!(result.value(), dec!(250));
    }

    #[test]
    fn test_divide() {
        let qty = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = qty.divide(dec!(4)).unwrap();

        assert_eq!(result.value(), dec!(25));
    }

    #[test]
    fn test_divide_by_zero() {
        let qty = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = qty.divide(dec!(0));

        assert!(result.is_err());
    }

    #[test]
    fn test_convert_kg_to_g() {
        let kg = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
        let g = kg.convert_to(&UnitOfMeasure::gram()).unwrap();

        assert_eq!(g.value(), dec!(1000));
        assert_eq!(g.unit().code(), "G");
    }

    #[test]
    fn test_convert_g_to_kg() {
        let g = Quantity::new(dec!(1500), UnitOfMeasure::gram()).unwrap();
        let kg = g.convert_to(&UnitOfMeasure::kilogram()).unwrap();

        assert_eq!(kg.value(), dec!(1.5));
        assert_eq!(kg.unit().code(), "KG");
    }

    #[test]
    fn test_convert_same_unit() {
        let kg = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = kg.convert_to(&UnitOfMeasure::kilogram()).unwrap();

        assert_eq!(result.value(), dec!(100));
    }

    #[test]
    fn test_convert_incompatible() {
        let kg = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = kg.convert_to(&UnitOfMeasure::meter());

        assert!(result.is_err());
    }

    #[test]
    fn test_can_convert_to() {
        let kg = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();

        assert!(kg.can_convert_to(&UnitOfMeasure::gram()));
        assert!(!kg.can_convert_to(&UnitOfMeasure::meter()));
    }

    #[test]
    fn test_round() {
        let qty = Quantity::new(dec!(100.123), UnitOfMeasure::kilogram()).unwrap();
        let rounded = qty.round(1);

        assert_eq!(rounded.value(), dec!(100.1));
    }

    #[test]
    fn test_allocate_equal() {
        let total = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)]).unwrap();

        assert_eq!(parts.len(), 3);
        let sum: Decimal = parts.iter().map(|p| p.value()).sum();
        assert_eq!(sum, dec!(100));
    }

    #[test]
    fn test_allocate_empty() {
        let total = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let result = total.allocate(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_equality_same_unit() {
        let a = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let b = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_equality_different_unit_same_value() {
        let kg = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
        let g = Quantity::new(dec!(1000), UnitOfMeasure::gram()).unwrap();

        assert_eq!(kg, g);
    }

    #[test]
    fn test_ordering() {
        let small = Quantity::new(dec!(50), UnitOfMeasure::kilogram()).unwrap();
        let large = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();

        assert!(small < large);
        assert!(large > small);
    }

    #[test]
    fn test_ordering_different_unit() {
        let kg = Quantity::new(dec!(1), UnitOfMeasure::kilogram()).unwrap();
        let g = Quantity::new(dec!(500), UnitOfMeasure::gram()).unwrap();

        assert!(g < kg); // 500g < 1kg
    }

    #[test]
    fn test_display() {
        let qty = Quantity::new(dec!(100.5), UnitOfMeasure::kilogram()).unwrap();
        assert_eq!(format!("{}", qty), "100.5 KG");
    }

    #[test]
    fn test_serialization() {
        let qty = Quantity::new(dec!(100.5), UnitOfMeasure::kilogram()).unwrap();
        let json = serde_json::to_string(&qty).unwrap();

        assert!(json.contains("\"value\":\"100.5\""));
        assert!(json.contains("\"unit\""));
    }

    #[test]
    fn test_shortcut_kg() {
        let qty = Quantity::kg(dec!(100)).unwrap();
        assert_eq!(qty.unit().code(), "KG");
    }

    #[test]
    fn test_shortcut_pc() {
        let qty = Quantity::pc(dec!(10)).unwrap();
        assert_eq!(qty.unit().code(), "PC");
    }

    #[test]
    fn test_clone() {
        let original = Quantity::new(dec!(100), UnitOfMeasure::kilogram()).unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
