//! 会计期间（Fiscal Period）
//!
//! 本模块实现会计期间值对象。
//!
//! # SAP 参考
//! - 表: T009（会计年度变式）
//! - 表: T009B（会计期间定义）
//! - 字段: GJAHR（会计年度）
//! - 字段: MONAT（会计期间，1-16）
//!
//! # 特殊期间
//! SAP 支持 16 个会计期间：
//! - 期间 1-12: 正常期间（对应自然月）
//! - 期间 13-16: 特殊期间（年结/调整）
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::FiscalPeriod;
//!
//! // 创建会计期间
//! let period = FiscalPeriod::new(2024, 3).unwrap();
//!
//! // 判断是否为特殊期间
//! assert!(!period.is_special_period());
//!
//! // 获取下一期间
//! let next = period.next().unwrap();
//! assert_eq!(next.period(), 4);
//! ```

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 最小会计期间
pub const MIN_PERIOD: i32 = 1;

/// 正常期间最大值
pub const MAX_NORMAL_PERIOD: i32 = 12;

/// 特殊期间最大值（SAP 支持 13-16 为特殊期间）
pub const MAX_SPECIAL_PERIOD: i32 = 16;

/// 会计期间
///
/// 表示一个会计期间（年度 + 期间），是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: T009B
/// - 字段: GJAHR（会计年度）, MONAT（期间）
///
/// # 期间规则
/// - 期间 1-12: 正常期间
/// - 期间 13-16: 特殊期间（年结调整）
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FiscalPeriod {
    /// 会计年度
    /// 对应 T009B.GJAHR
    year: i32,

    /// 会计期间（1-16）
    /// 对应 T009B.MONAT
    period: i32,
}

impl FiscalPeriod {
    /// 创建新的会计期间
    ///
    /// # 参数
    /// - `year`: 会计年度
    /// - `period`: 会计期间（1-16）
    ///
    /// # 错误
    /// - 年度不合法（1900-2100）
    /// - 期间不合法（1-16）
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::FiscalPeriod;
    ///
    /// let period = FiscalPeriod::new(2024, 3).unwrap();
    /// assert_eq!(period.year(), 2024);
    /// assert_eq!(period.period(), 3);
    /// ```
    pub fn new(year: i32, period: i32) -> DomainResult<Self> {
        // 验证年度
        if year < 1900 || year > 2100 {
            return Err(DomainError::fiscal_period_invalid(
                year,
                period,
                format!("会计年度 {} 不合法（有效范围: 1900-2100）", year),
            ));
        }

        // 验证期间
        if period < MIN_PERIOD || period > MAX_SPECIAL_PERIOD {
            return Err(DomainError::fiscal_period_invalid(
                year,
                period,
                format!(
                    "会计期间 {} 不合法（有效范围: {}-{}）",
                    period, MIN_PERIOD, MAX_SPECIAL_PERIOD
                ),
            ));
        }

        Ok(Self { year, period })
    }

    /// 从年月创建会计期间
    ///
    /// 假设会计年度与自然年度一致，期间对应自然月
    ///
    /// # 参数
    /// - `year`: 年份
    /// - `month`: 月份（1-12）
    pub fn from_year_month(year: i32, month: i32) -> DomainResult<Self> {
        if month < 1 || month > 12 {
            return Err(DomainError::fiscal_period_invalid(
                year,
                month,
                format!("月份 {} 不合法", month),
            ));
        }
        Self::new(year, month)
    }

    /// 创建特殊期间
    ///
    /// # 参数
    /// - `year`: 会计年度
    /// - `special_period`: 特殊期间号（1-4，对应期间 13-16）
    pub fn special_period(year: i32, special_period: i32) -> DomainResult<Self> {
        if special_period < 1 || special_period > 4 {
            return Err(DomainError::fiscal_period_invalid(
                year,
                special_period,
                "特殊期间号必须是 1-4",
            ));
        }
        Self::new(year, MAX_NORMAL_PERIOD + special_period)
    }

    /// 获取会计年度
    pub fn year(&self) -> i32 {
        self.year
    }

    /// 获取会计期间
    pub fn period(&self) -> i32 {
        self.period
    }

    /// 判断是否为特殊期间（13-16）
    pub fn is_special_period(&self) -> bool {
        self.period > MAX_NORMAL_PERIOD
    }

    /// 判断是否为正常期间（1-12）
    pub fn is_normal_period(&self) -> bool {
        self.period <= MAX_NORMAL_PERIOD
    }

    /// 获取特殊期间号（1-4）
    ///
    /// 仅对特殊期间有效
    pub fn special_period_number(&self) -> Option<i32> {
        if self.is_special_period() {
            Some(self.period - MAX_NORMAL_PERIOD)
        } else {
            None
        }
    }

    /// 判断是否为年度第一期
    pub fn is_first_period(&self) -> bool {
        self.period == MIN_PERIOD
    }

    /// 判断是否为年度最后正常期间
    pub fn is_last_normal_period(&self) -> bool {
        self.period == MAX_NORMAL_PERIOD
    }

    /// 判断是否为年度最后期间（包括特殊期间）
    pub fn is_last_period(&self) -> bool {
        self.period == MAX_SPECIAL_PERIOD
    }

    /// 获取下一个期间
    ///
    /// 如果当前是第 12 期，下一期是下一年度第 1 期
    /// 特殊期间（13-16）不参与正常期间流转
    ///
    /// # 错误
    /// - 当前是特殊期间
    pub fn next(&self) -> DomainResult<Self> {
        if self.is_special_period() {
            return Err(DomainError::fiscal_period_invalid(
                self.year,
                self.period,
                "特殊期间不支持获取下一期间",
            ));
        }

        if self.period == MAX_NORMAL_PERIOD {
            Self::new(self.year + 1, MIN_PERIOD)
        } else {
            Self::new(self.year, self.period + 1)
        }
    }

    /// 获取上一个期间
    ///
    /// 如果当前是第 1 期，上一期是上一年度第 12 期
    ///
    /// # 错误
    /// - 当前是特殊期间
    pub fn previous(&self) -> DomainResult<Self> {
        if self.is_special_period() {
            return Err(DomainError::fiscal_period_invalid(
                self.year,
                self.period,
                "特殊期间不支持获取上一期间",
            ));
        }

        if self.period == MIN_PERIOD {
            Self::new(self.year - 1, MAX_NORMAL_PERIOD)
        } else {
            Self::new(self.year, self.period - 1)
        }
    }

    /// 获取年度第一期
    pub fn first_period_of_year(&self) -> Self {
        Self {
            year: self.year,
            period: MIN_PERIOD,
        }
    }

    /// 获取年度最后正常期间
    pub fn last_normal_period_of_year(&self) -> Self {
        Self {
            year: self.year,
            period: MAX_NORMAL_PERIOD,
        }
    }

    /// 计算与另一期间的差值（期间数）
    ///
    /// 正数表示 self 在 other 之后，负数表示 self 在 other 之前
    ///
    /// # 注意
    /// 仅对正常期间有效，特殊期间返回 None
    pub fn periods_between(&self, other: &FiscalPeriod) -> Option<i32> {
        if self.is_special_period() || other.is_special_period() {
            return None;
        }

        let self_months = self.year * 12 + self.period;
        let other_months = other.year * 12 + other.period;
        Some(self_months - other_months)
    }

    /// 获取格式化的期间字符串
    ///
    /// 格式: YYYY/PP（如 2024/03）
    pub fn to_string_formatted(&self) -> String {
        format!("{}/{:02}", self.year, self.period)
    }
}

impl PartialEq for FiscalPeriod {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.period == other.period
    }
}

impl Eq for FiscalPeriod {}

impl Hash for FiscalPeriod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.year.hash(state);
        self.period.hash(state);
    }
}

impl PartialOrd for FiscalPeriod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiscalPeriod {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => self.period.cmp(&other.period),
            other => other,
        }
    }
}

impl fmt::Display for FiscalPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{:02}", self.year, self.period)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fiscal_period() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        assert_eq!(period.year(), 2024);
        assert_eq!(period.period(), 3);
    }

    #[test]
    fn test_create_special_period() {
        let period = FiscalPeriod::new(2024, 13).unwrap();
        assert!(period.is_special_period());
        assert_eq!(period.special_period_number(), Some(1));
    }

    #[test]
    fn test_special_period_factory() {
        let period = FiscalPeriod::special_period(2024, 2).unwrap();
        assert_eq!(period.period(), 14);
        assert!(period.is_special_period());
    }

    #[test]
    fn test_from_year_month() {
        let period = FiscalPeriod::from_year_month(2024, 6).unwrap();
        assert_eq!(period.year(), 2024);
        assert_eq!(period.period(), 6);
    }

    #[test]
    fn test_invalid_year() {
        let result = FiscalPeriod::new(1800, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_period_zero() {
        let result = FiscalPeriod::new(2024, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_period_too_large() {
        let result = FiscalPeriod::new(2024, 17);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_special_period() {
        let normal = FiscalPeriod::new(2024, 12).unwrap();
        let special = FiscalPeriod::new(2024, 13).unwrap();

        assert!(!normal.is_special_period());
        assert!(special.is_special_period());
    }

    #[test]
    fn test_is_normal_period() {
        let normal = FiscalPeriod::new(2024, 12).unwrap();
        let special = FiscalPeriod::new(2024, 13).unwrap();

        assert!(normal.is_normal_period());
        assert!(!special.is_normal_period());
    }

    #[test]
    fn test_is_first_period() {
        let first = FiscalPeriod::new(2024, 1).unwrap();
        let second = FiscalPeriod::new(2024, 2).unwrap();

        assert!(first.is_first_period());
        assert!(!second.is_first_period());
    }

    #[test]
    fn test_is_last_normal_period() {
        let last = FiscalPeriod::new(2024, 12).unwrap();
        let eleventh = FiscalPeriod::new(2024, 11).unwrap();

        assert!(last.is_last_normal_period());
        assert!(!eleventh.is_last_normal_period());
    }

    #[test]
    fn test_next_same_year() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        let next = period.next().unwrap();

        assert_eq!(next.year(), 2024);
        assert_eq!(next.period(), 4);
    }

    #[test]
    fn test_next_year_rollover() {
        let period = FiscalPeriod::new(2024, 12).unwrap();
        let next = period.next().unwrap();

        assert_eq!(next.year(), 2025);
        assert_eq!(next.period(), 1);
    }

    #[test]
    fn test_next_special_period() {
        let period = FiscalPeriod::new(2024, 13).unwrap();
        let result = period.next();

        assert!(result.is_err());
    }

    #[test]
    fn test_previous_same_year() {
        let period = FiscalPeriod::new(2024, 6).unwrap();
        let prev = period.previous().unwrap();

        assert_eq!(prev.year(), 2024);
        assert_eq!(prev.period(), 5);
    }

    #[test]
    fn test_previous_year_rollover() {
        let period = FiscalPeriod::new(2024, 1).unwrap();
        let prev = period.previous().unwrap();

        assert_eq!(prev.year(), 2023);
        assert_eq!(prev.period(), 12);
    }

    #[test]
    fn test_first_period_of_year() {
        let period = FiscalPeriod::new(2024, 6).unwrap();
        let first = period.first_period_of_year();

        assert_eq!(first.year(), 2024);
        assert_eq!(first.period(), 1);
    }

    #[test]
    fn test_last_normal_period_of_year() {
        let period = FiscalPeriod::new(2024, 6).unwrap();
        let last = period.last_normal_period_of_year();

        assert_eq!(last.year(), 2024);
        assert_eq!(last.period(), 12);
    }

    #[test]
    fn test_periods_between_same_year() {
        let a = FiscalPeriod::new(2024, 6).unwrap();
        let b = FiscalPeriod::new(2024, 3).unwrap();

        assert_eq!(a.periods_between(&b), Some(3));
        assert_eq!(b.periods_between(&a), Some(-3));
    }

    #[test]
    fn test_periods_between_different_years() {
        let a = FiscalPeriod::new(2025, 3).unwrap();
        let b = FiscalPeriod::new(2024, 10).unwrap();

        assert_eq!(a.periods_between(&b), Some(5));
    }

    #[test]
    fn test_periods_between_special_period() {
        let a = FiscalPeriod::new(2024, 13).unwrap();
        let b = FiscalPeriod::new(2024, 3).unwrap();

        assert_eq!(a.periods_between(&b), None);
    }

    #[test]
    fn test_equality() {
        let a = FiscalPeriod::new(2024, 6).unwrap();
        let b = FiscalPeriod::new(2024, 6).unwrap();
        let c = FiscalPeriod::new(2024, 7).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_ordering() {
        let earlier = FiscalPeriod::new(2024, 3).unwrap();
        let later = FiscalPeriod::new(2024, 6).unwrap();
        let next_year = FiscalPeriod::new(2025, 1).unwrap();

        assert!(earlier < later);
        assert!(later < next_year);
        assert!(earlier < next_year);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(FiscalPeriod::new(2024, 6).unwrap());
        set.insert(FiscalPeriod::new(2024, 6).unwrap());
        set.insert(FiscalPeriod::new(2024, 7).unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        assert_eq!(format!("{}", period), "2024/03");
    }

    #[test]
    fn test_to_string_formatted() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        assert_eq!(period.to_string_formatted(), "2024/03");
    }

    #[test]
    fn test_serialization() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        let json = serde_json::to_string(&period).unwrap();

        assert!(json.contains("\"year\":2024"));
        assert!(json.contains("\"period\":3"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"year":2024,"period":3}"#;
        let period: FiscalPeriod = serde_json::from_str(json).unwrap();

        assert_eq!(period.year(), 2024);
        assert_eq!(period.period(), 3);
    }

    #[test]
    fn test_clone() {
        let original = FiscalPeriod::new(2024, 6).unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
