//! 金额（Money）
//!
//! 本模块实现金额值对象，用于处理货币金额的精确计算。
//!
//! # SAP 参考
//! - 表: ACDOCA（通用日记账）
//! - 字段: HSL（本位币金额）, TSL（交易币金额）
//! - 字段: RHCUR（本位币币种）, RTCUR（交易币币种）
//! - 字段: DRCRK（借贷标识: S=借方, H=贷方）
//! - 数据类型: CURR（货币金额，4 位小数）
//!
//! # 精度处理
//! 使用 `rust_decimal` 库确保金额计算的精度，避免浮点数误差。
//! SAP 标准金额精度为 4 位小数，本模块遵循此标准。
//!
//! # 借贷方向
//! - 正数表示借方（Debit）
//! - 负数表示贷方（Credit）
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::{Money, CurrencyCode};
//! use rust_decimal_macros::dec;
//!
//! // 创建金额
//! let price = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
//!
//! // 金额计算
//! let tax = price.multiply(dec!(0.13)).unwrap();
//! let total = price.add(&tax).unwrap();
//!
//! // 借贷判断
//! assert!(total.is_debit());
//! ```

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::currency_code::CurrencyCode;
use crate::error::{DomainError, DomainResult};

/// 金额精度：4 位小数
///
/// SAP 标准金额精度，适用于大多数货币计算场景
pub const MONEY_SCALE: u32 = 4;

/// 舍入模式
///
/// 定义金额舍入的方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoundingMode {
    /// 四舍五入（银行家舍入法）
    #[default]
    HalfEven,
    /// 向上舍入（进一法）
    Up,
    /// 向下舍入（去尾法）
    Down,
    /// 向正无穷舍入
    Ceiling,
    /// 向负无穷舍入
    Floor,
    /// 标准四舍五入
    HalfUp,
    /// 五舍六入
    HalfDown,
}

impl RoundingMode {
    /// 转换为 rust_decimal 的 RoundingStrategy
    fn to_strategy(self) -> RoundingStrategy {
        match self {
            RoundingMode::HalfEven => RoundingStrategy::MidpointNearestEven,
            RoundingMode::Up => RoundingStrategy::AwayFromZero,
            RoundingMode::Down => RoundingStrategy::ToZero,
            RoundingMode::Ceiling => RoundingStrategy::ToPositiveInfinity,
            RoundingMode::Floor => RoundingStrategy::ToNegativeInfinity,
            RoundingMode::HalfUp => RoundingStrategy::MidpointAwayFromZero,
            RoundingMode::HalfDown => RoundingStrategy::MidpointTowardZero,
        }
    }
}

/// 金额
///
/// 表示一个带币种的货币金额，是一个不可变的值对象。
/// 所有修改操作都会返回新的 Money 实例。
///
/// # SAP 参考
/// - 数据类型: CURR（货币金额）
/// - 精度: 4 位小数
/// - 表: ACDOCA.HSL, ACDOCA.TSL
///
/// # 不变性
/// - 金额精度不超过 4 位小数
/// - 币种必须是有效的 ISO 4217 代码
/// - 同币种才能进行加减运算
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    /// 金额数值
    /// 正数表示借方，负数表示贷方
    #[serde(with = "rust_decimal::serde::str")]
    amount: Decimal,

    /// 币种代码
    currency: CurrencyCode,
}

impl Money {
    /// 创建新的金额
    ///
    /// # 参数
    /// - `amount`: 金额数值
    /// - `currency`: 币种代码
    ///
    /// # 返回
    /// 创建的 Money 实例，金额会自动舍入到 4 位小数
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    /// use rust_decimal_macros::dec;
    ///
    /// let money = Money::new(dec!(100.1234), CurrencyCode::cny()).unwrap();
    /// assert_eq!(money.amount(), dec!(100.1234));
    /// ```
    pub fn new(amount: Decimal, currency: CurrencyCode) -> DomainResult<Self> {
        let rounded = amount.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            amount: rounded,
            currency,
        })
    }

    /// 创建零金额
    ///
    /// # 参数
    /// - `currency`: 币种代码
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    ///
    /// let zero = Money::zero(CurrencyCode::cny());
    /// assert!(zero.is_zero());
    /// ```
    pub fn zero(currency: CurrencyCode) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    /// 从字符串解析金额
    ///
    /// # 参数
    /// - `amount_str`: 金额字符串
    /// - `currency`: 币种代码
    ///
    /// # 错误
    /// - 金额字符串格式无效
    pub fn from_str(amount_str: &str, currency: CurrencyCode) -> DomainResult<Self> {
        let amount = Decimal::from_str(amount_str).map_err(|_| {
            DomainError::money_invalid_amount(format!("无效的金额格式: {}", amount_str))
        })?;
        Self::new(amount, currency)
    }

    /// 从整数创建金额（最小单位）
    ///
    /// 例如：100 分 = 1 元
    ///
    /// # 参数
    /// - `minor_units`: 最小货币单位的数量（如分）
    /// - `currency`: 币种代码
    pub fn from_minor_units(minor_units: i64, currency: CurrencyCode) -> Self {
        let decimals = currency.decimal_places();
        let divisor = Decimal::from(10_i64.pow(decimals as u32));
        let amount = Decimal::from(minor_units) / divisor;
        Self { amount, currency }
    }

    /// 获取金额数值
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// 获取币种代码
    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    /// 获取金额的最小单位数量
    ///
    /// 例如：1.00 CNY = 100 分
    pub fn to_minor_units(&self) -> i64 {
        let decimals = self.currency.decimal_places();
        let multiplier = Decimal::from(10_i64.pow(decimals as u32));
        let minor = self.amount * multiplier;
        minor.to_i64().unwrap_or(0)
    }

    /// 判断金额是否为零
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// 判断金额是否为正（借方）
    ///
    /// # SAP 参考
    /// 对应 DRCRK = 'S'（Soll，借方）
    pub fn is_positive(&self) -> bool {
        self.amount > Decimal::ZERO
    }

    /// 判断金额是否为负（贷方）
    ///
    /// # SAP 参考
    /// 对应 DRCRK = 'H'（Haben，贷方）
    pub fn is_negative(&self) -> bool {
        self.amount < Decimal::ZERO
    }

    /// 判断是否为借方金额
    ///
    /// 借方金额为正数或零
    pub fn is_debit(&self) -> bool {
        self.amount >= Decimal::ZERO
    }

    /// 判断是否为贷方金额
    ///
    /// 贷方金额为负数
    pub fn is_credit(&self) -> bool {
        self.amount < Decimal::ZERO
    }

    /// 获取借贷标识
    ///
    /// # 返回
    /// - 'S': 借方（Soll）
    /// - 'H': 贷方（Haben）
    pub fn debit_credit_indicator(&self) -> char {
        if self.amount >= Decimal::ZERO {
            'S'
        } else {
            'H'
        }
    }

    /// 金额取反
    ///
    /// 借方变贷方，贷方变借方
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    /// use rust_decimal_macros::dec;
    ///
    /// let debit = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
    /// let credit = debit.negate();
    /// assert_eq!(credit.amount(), dec!(-100));
    /// ```
    pub fn negate(&self) -> Self {
        Self {
            amount: -self.amount,
            currency: self.currency.clone(),
        }
    }

    /// 获取绝对值
    pub fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency.clone(),
        }
    }

    /// 金额相加
    ///
    /// # 错误
    /// - 币种不一致
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    /// use rust_decimal_macros::dec;
    ///
    /// let a = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
    /// let b = Money::new(dec!(50), CurrencyCode::cny()).unwrap();
    /// let sum = a.add(&b).unwrap();
    /// assert_eq!(sum.amount(), dec!(150));
    /// ```
    pub fn add(&self, other: &Money) -> DomainResult<Self> {
        self.ensure_same_currency(other)?;
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// 金额相减
    ///
    /// # 错误
    /// - 币种不一致
    pub fn subtract(&self, other: &Money) -> DomainResult<Self> {
        self.ensure_same_currency(other)?;
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    /// 金额乘法
    ///
    /// 用于计算税额、折扣等
    ///
    /// # 参数
    /// - `multiplier`: 乘数（如税率 0.13）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    /// use rust_decimal_macros::dec;
    ///
    /// let price = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
    /// let tax = price.multiply(dec!(0.13)).unwrap();
    /// assert_eq!(tax.amount(), dec!(13));
    /// ```
    pub fn multiply(&self, multiplier: Decimal) -> DomainResult<Self> {
        let result = self.amount * multiplier;
        let rounded = result.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            amount: rounded,
            currency: self.currency.clone(),
        })
    }

    /// 金额除法
    ///
    /// # 参数
    /// - `divisor`: 除数
    ///
    /// # 错误
    /// - 除数为零
    pub fn divide(&self, divisor: Decimal) -> DomainResult<Self> {
        if divisor.is_zero() {
            return Err(DomainError::money_invalid_operation("除数不能为零"));
        }
        let result = self.amount / divisor;
        let rounded = result.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::MidpointNearestEven);
        Ok(Self {
            amount: rounded,
            currency: self.currency.clone(),
        })
    }

    /// 按指定模式舍入
    ///
    /// # 参数
    /// - `scale`: 小数位数
    /// - `mode`: 舍入模式
    pub fn round(&self, scale: u32, mode: RoundingMode) -> Self {
        let rounded = self.amount.round_dp_with_strategy(scale, mode.to_strategy());
        Self {
            amount: rounded,
            currency: self.currency.clone(),
        }
    }

    /// 舍入到币种的标准小数位
    ///
    /// 例如：CNY 舍入到 2 位，JPY 舍入到 0 位
    pub fn round_to_currency_precision(&self, mode: RoundingMode) -> Self {
        let scale = self.currency.decimal_places() as u32;
        self.round(scale, mode)
    }

    /// 分配金额
    ///
    /// 将金额按比例分配到多个部分，确保总和等于原金额。
    /// 使用最大余数法处理舍入误差。
    ///
    /// # 参数
    /// - `ratios`: 分配比例（不需要归一化）
    ///
    /// # 错误
    /// - 比例为空
    /// - 所有比例都为零
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::{Money, CurrencyCode};
    /// use rust_decimal_macros::dec;
    ///
    /// let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
    /// let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)]).unwrap();
    /// assert_eq!(parts.len(), 3);
    /// // 33.3334 + 33.3333 + 33.3333 = 100
    /// ```
    pub fn allocate(&self, ratios: &[Decimal]) -> DomainResult<Vec<Self>> {
        if ratios.is_empty() {
            return Err(DomainError::money_invalid_operation("分配比例不能为空"));
        }

        let total_ratio: Decimal = ratios.iter().sum();
        if total_ratio.is_zero() {
            return Err(DomainError::money_invalid_operation("分配比例总和不能为零"));
        }

        // 计算每个部分的金额
        let mut results: Vec<(Decimal, Decimal)> = ratios
            .iter()
            .map(|ratio| {
                let exact = self.amount * (*ratio) / total_ratio;
                let rounded = exact.round_dp_with_strategy(MONEY_SCALE, RoundingStrategy::ToZero);
                let remainder = exact - rounded;
                (rounded, remainder)
            })
            .collect();

        // 计算舍入误差
        let allocated_sum: Decimal = results.iter().map(|(r, _)| *r).sum();
        let mut remaining = self.amount - allocated_sum;

        // 使用最大余数法分配剩余金额
        let unit = Decimal::new(1, MONEY_SCALE);
        while remaining >= unit {
            // 找到余数最大的项
            let max_idx = results
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap();

            results[max_idx].0 += unit;
            results[max_idx].1 = Decimal::ZERO; // 清除余数，避免重复分配
            remaining -= unit;
        }

        Ok(results
            .into_iter()
            .map(|(amount, _)| Self {
                amount,
                currency: self.currency.clone(),
            })
            .collect())
    }

    /// 确保两个金额币种相同
    fn ensure_same_currency(&self, other: &Money) -> DomainResult<()> {
        if self.currency != other.currency {
            return Err(DomainError::money_currency_mismatch(
                self.currency.code().to_string(),
                other.currency.code().to_string(),
            ));
        }
        Ok(())
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount && self.currency == other.currency
    }
}

impl Eq for Money {}

impl Hash for Money {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 使用字符串表示来确保哈希一致性
        self.amount.to_string().hash(state);
        self.currency.hash(state);
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            return None;
        }
        self.amount.partial_cmp(&other.amount)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.code())
    }
}

// ============================================================================
// 常用币种快捷方法
// ============================================================================

impl Money {
    /// 创建人民币金额
    pub fn cny(amount: Decimal) -> DomainResult<Self> {
        Self::new(amount, CurrencyCode::cny())
    }

    /// 创建美元金额
    pub fn usd(amount: Decimal) -> DomainResult<Self> {
        Self::new(amount, CurrencyCode::usd())
    }

    /// 创建欧元金额
    pub fn eur(amount: Decimal) -> DomainResult<Self> {
        Self::new(amount, CurrencyCode::eur())
    }

    /// 创建日元金额
    pub fn jpy(amount: Decimal) -> DomainResult<Self> {
        Self::new(amount, CurrencyCode::jpy())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_money() {
        let money = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
        assert_eq!(money.amount(), dec!(100.50));
        assert_eq!(money.currency().code(), "CNY");
    }

    #[test]
    fn test_money_precision() {
        // 超过 4 位小数会被舍入
        let money = Money::new(dec!(100.123456), CurrencyCode::cny()).unwrap();
        assert_eq!(money.amount(), dec!(100.1235));
    }

    #[test]
    fn test_money_zero() {
        let zero = Money::zero(CurrencyCode::cny());
        assert!(zero.is_zero());
        assert_eq!(zero.amount(), dec!(0));
    }

    #[test]
    fn test_money_from_str() {
        let money = Money::from_str("123.45", CurrencyCode::cny()).unwrap();
        assert_eq!(money.amount(), dec!(123.45));
    }

    #[test]
    fn test_money_from_str_invalid() {
        let result = Money::from_str("abc", CurrencyCode::cny());
        assert!(result.is_err());
    }

    #[test]
    fn test_money_from_minor_units() {
        let money = Money::from_minor_units(10050, CurrencyCode::cny());
        assert_eq!(money.amount(), dec!(100.50));
    }

    #[test]
    fn test_money_to_minor_units() {
        let money = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
        assert_eq!(money.to_minor_units(), 10050);
    }

    #[test]
    fn test_money_jpy_minor_units() {
        // 日元没有小数位
        let money = Money::from_minor_units(100, CurrencyCode::jpy());
        assert_eq!(money.amount(), dec!(100));
        assert_eq!(money.to_minor_units(), 100);
    }

    #[test]
    fn test_is_positive() {
        let positive = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let negative = Money::new(dec!(-100), CurrencyCode::cny()).unwrap();
        let zero = Money::zero(CurrencyCode::cny());

        assert!(positive.is_positive());
        assert!(!negative.is_positive());
        assert!(!zero.is_positive());
    }

    #[test]
    fn test_is_negative() {
        let positive = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let negative = Money::new(dec!(-100), CurrencyCode::cny()).unwrap();

        assert!(!positive.is_negative());
        assert!(negative.is_negative());
    }

    #[test]
    fn test_debit_credit() {
        let debit = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let credit = Money::new(dec!(-100), CurrencyCode::cny()).unwrap();

        assert!(debit.is_debit());
        assert!(!debit.is_credit());
        assert_eq!(debit.debit_credit_indicator(), 'S');

        assert!(!credit.is_debit());
        assert!(credit.is_credit());
        assert_eq!(credit.debit_credit_indicator(), 'H');
    }

    #[test]
    fn test_negate() {
        let debit = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let credit = debit.negate();

        assert_eq!(credit.amount(), dec!(-100));
        assert!(credit.is_credit());
    }

    #[test]
    fn test_abs() {
        let negative = Money::new(dec!(-100), CurrencyCode::cny()).unwrap();
        let absolute = negative.abs();

        assert_eq!(absolute.amount(), dec!(100));
    }

    #[test]
    fn test_add() {
        let a = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let b = Money::new(dec!(50.50), CurrencyCode::cny()).unwrap();
        let sum = a.add(&b).unwrap();

        assert_eq!(sum.amount(), dec!(150.50));
    }

    #[test]
    fn test_add_different_currency() {
        let cny = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let usd = Money::new(dec!(100), CurrencyCode::usd()).unwrap();

        let result = cny.add(&usd);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract() {
        let a = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let b = Money::new(dec!(30), CurrencyCode::cny()).unwrap();
        let diff = a.subtract(&b).unwrap();

        assert_eq!(diff.amount(), dec!(70));
    }

    #[test]
    fn test_multiply() {
        let price = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let tax = price.multiply(dec!(0.13)).unwrap();

        assert_eq!(tax.amount(), dec!(13));
    }

    #[test]
    fn test_multiply_with_rounding() {
        let price = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let result = price.multiply(dec!(0.333333)).unwrap();

        // 应该舍入到 4 位小数
        assert_eq!(result.amount(), dec!(33.3333));
    }

    #[test]
    fn test_divide() {
        let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let half = total.divide(dec!(2)).unwrap();

        assert_eq!(half.amount(), dec!(50));
    }

    #[test]
    fn test_divide_by_zero() {
        let money = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let result = money.divide(dec!(0));

        assert!(result.is_err());
    }

    #[test]
    fn test_round() {
        let money = Money::new(dec!(100.1234), CurrencyCode::cny()).unwrap();

        let rounded = money.round(2, RoundingMode::HalfUp);
        assert_eq!(rounded.amount(), dec!(100.12));

        let up = money.round(2, RoundingMode::Up);
        assert_eq!(up.amount(), dec!(100.13));
    }

    #[test]
    fn test_round_to_currency_precision() {
        let cny = Money::new(dec!(100.1234), CurrencyCode::cny()).unwrap();
        let rounded_cny = cny.round_to_currency_precision(RoundingMode::HalfUp);
        assert_eq!(rounded_cny.amount(), dec!(100.12));

        let jpy = Money::new(dec!(100.5), CurrencyCode::jpy()).unwrap();
        let rounded_jpy = jpy.round_to_currency_precision(RoundingMode::HalfUp);
        assert_eq!(rounded_jpy.amount(), dec!(101));
    }

    #[test]
    fn test_allocate_equal() {
        let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)]).unwrap();

        assert_eq!(parts.len(), 3);
        let sum: Decimal = parts.iter().map(|p| p.amount()).sum();
        assert_eq!(sum, dec!(100));
    }

    #[test]
    fn test_allocate_unequal() {
        let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let parts = total.allocate(&[dec!(1), dec!(2), dec!(2)]).unwrap();

        assert_eq!(parts.len(), 3);
        let sum: Decimal = parts.iter().map(|p| p.amount()).sum();
        assert_eq!(sum, dec!(100));
    }

    #[test]
    fn test_allocate_empty() {
        let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let result = total.allocate(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_allocate_all_zero() {
        let total = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let result = total.allocate(&[dec!(0), dec!(0)]);

        assert!(result.is_err());
    }

    #[test]
    fn test_equality() {
        let a = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let b = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let c = Money::new(dec!(100), CurrencyCode::usd()).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_ordering() {
        let small = Money::new(dec!(50), CurrencyCode::cny()).unwrap();
        let large = Money::new(dec!(100), CurrencyCode::cny()).unwrap();

        assert!(small < large);
        assert!(large > small);
    }

    #[test]
    fn test_ordering_different_currency() {
        let cny = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let usd = Money::new(dec!(100), CurrencyCode::usd()).unwrap();

        assert_eq!(cny.partial_cmp(&usd), None);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Money::new(dec!(100), CurrencyCode::cny()).unwrap());
        set.insert(Money::new(dec!(100), CurrencyCode::cny()).unwrap());
        set.insert(Money::new(dec!(200), CurrencyCode::cny()).unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let money = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
        assert_eq!(format!("{}", money), "100.50 CNY");
    }

    #[test]
    fn test_serialization() {
        let money = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
        let json = serde_json::to_string(&money).unwrap();

        assert!(json.contains("\"amount\":\"100.50\""));
        assert!(json.contains("\"currency\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"amount":"100.50","currency":"CNY"}"#;
        let money: Money = serde_json::from_str(json).unwrap();

        assert_eq!(money.amount(), dec!(100.50));
        assert_eq!(money.currency().code(), "CNY");
    }

    #[test]
    fn test_shortcut_cny() {
        let money = Money::cny(dec!(100)).unwrap();
        assert_eq!(money.currency().code(), "CNY");
    }

    #[test]
    fn test_shortcut_usd() {
        let money = Money::usd(dec!(100)).unwrap();
        assert_eq!(money.currency().code(), "USD");
    }

    #[test]
    fn test_clone() {
        let original = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
