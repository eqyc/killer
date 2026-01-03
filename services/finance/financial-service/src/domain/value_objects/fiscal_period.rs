//! 会计期间值对象

use std::fmt;
use std::str::FromStr;
use crate::domain::value_objects::fiscal_period::FiscalPeriodError;

/// 会计期间
///
/// 包含会计年度和期间两个部分
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FiscalPeriod {
    fiscal_year: u32,
    period: u32,
}

impl FiscalPeriod {
    /// 创建会计期间
    ///
    /// # Arguments
    /// * `fiscal_year` - 会计年度（4位年份）
    /// * `period` - 会计期间（1-16，16为年度结算期间）
    pub fn new(fiscal_year: u32, period: u32) -> Result<Self, FiscalPeriodError> {
        if !(1990..=9999).contains(&fiscal_year) {
            return Err(FiscalPeriodError::InvalidYear(fiscal_year));
        }
        if !(1..=16).contains(&period) {
            return Err(FiscalPeriodError::InvalidPeriod(period));
        }
        Ok(Self { fiscal_year, period })
    }

    /// 从字符串解析（格式：YYYYMM）
    pub fn from_ymd(ymd: &str) -> Result<Self, FiscalPeriodError> {
        if ymd.len() != 6 {
            return Err(FiscalPeriodError::InvalidFormat(ymd.to_string()));
        }
        let year: u32 = ymd[0..4].parse().map_err(|_| FiscalPeriodError::InvalidFormat(ymd.to_string()))?;
        let period: u32 = ymd[4..6].parse().map_err(|_| FiscalPeriodError::InvalidFormat(ymd.to_string()))?;
        Self::new(year, period)
    }

    /// 获取会计年度
    pub fn fiscal_year(&self) -> u32 {
        self.fiscal_year
    }

    /// 获取期间
    pub fn period(&self) -> u32 {
        self.period
    }

    /// 判断是否为年度结算期间
    pub fn is_year_closing(&self) -> bool {
        self.period == 16
    }

    /// 下一期间
    pub fn next_period(&self) -> Self {
        if self.period == 16 {
            Self::new(self.fiscal_year + 1, 1).unwrap()
        } else {
            Self::new(self.fiscal_year, self.period + 1).unwrap()
        }
    }

    /// 上一期间
    pub fn previous_period(&self) -> Self {
        if self.period == 1 {
            Self::new(self.fiscal_year - 1, 16).unwrap()
        } else {
            Self::new(self.fiscal_year, self.period - 1).unwrap()
        }
    }
}

impl fmt::Display for FiscalPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:02}", self.fiscal_year, self.period)
    }
}

impl FromStr for FiscalPeriod {
    type Err = FiscalPeriodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ymd(s)
    }
}

/// 会计期间错误
#[derive(Debug, thiserror::Error)]
pub enum FiscalPeriodError {
    #[error("无效的会计年度: {0}")]
    InvalidYear(u32),
    #[error("无效的会计期间: {0} (有效范围: 1-16)")]
    InvalidPeriod(u32),
    #[error("格式无效，应为 YYYYMM 格式: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_period() {
        let period = FiscalPeriod::new(2024, 1).unwrap();
        assert_eq!(period.fiscal_year(), 2024);
        assert_eq!(period.period(), 1);
    }

    #[test]
    fn test_year_closing_period() {
        let period = FiscalPeriod::new(2024, 16).unwrap();
        assert!(period.is_year_closing());
    }

    #[test]
    fn test_from_ymd() {
        let period = FiscalPeriod::from_ymd("202403").unwrap();
        assert_eq!(period.fiscal_year(), 2024);
        assert_eq!(period.period(), 3);
    }

    #[test]
    fn test_display() {
        let period = FiscalPeriod::new(2024, 3).unwrap();
        assert_eq!(format!("{}", period), "202403");
    }

    #[test]
    fn test_invalid_period() {
        assert!(FiscalPeriod::new(2024, 0).is_err());
        assert!(FiscalPeriod::new(2024, 17).is_err());
    }
}
