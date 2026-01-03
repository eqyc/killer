//! 过账日期值对象

use std::fmt;
use chrono::{NaiveDate, Datelike};

/// 过账日期
///
/// 确保过账日期在有效的会计期间内
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostingDate(NaiveDate);

impl PostingDate {
    /// 创建过账日期
    pub fn new(date: NaiveDate) -> Result<Self, PostingDateError> {
        // 验证日期有效（不为空日期）
        if date.year() < 1990 || date.year() > 9999 {
            return Err(PostingDateError::InvalidYear(date.year()));
        }
        Ok(Self(date))
    }

    /// 从字符串解析（格式：YYYY-MM-DD）
    pub fn from_str(date_str: &str) -> Result<Self, PostingDateError> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| PostingDateError::InvalidFormat(date_str.to_string()))?;
        Self::new(date)
    }

    /// 获取内部日期
    pub fn inner(&self) -> NaiveDate {
        self.0
    }

    /// 获取年份
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// 获取月份
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// 获取日
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// 获取星期几（1-7，周一到周日）
    pub fn weekday(&self) -> u32 {
        self.0.weekday().num_days_from_monday() + 1
    }

    /// 判断是否为周末
    pub fn is_weekend(&self) -> bool {
        let wd = self.weekday();
        wd == 6 || wd == 7
    }

    /// 获取所在月份的最后一天
    pub fn month_end(&self) -> NaiveDate {
        let next_month = if self.0.month() == 12 {
            NaiveDate::from_ymd_opt(self.0.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(self.0.year(), self.0.month() + 1, 1).unwrap()
        };
        next_month.pred_opt().unwrap()
    }
}

impl fmt::Display for PostingDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

/// 过账日期错误
#[derive(Debug, thiserror::Error)]
pub enum PostingDateError {
    #[error("无效的年份: {0}")]
    InvalidYear(i32),
    #[error("日期格式无效，应为 YYYY-MM-DD: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let date = PostingDate::new(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()).unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_from_str() {
        let date = PostingDate::from_str("2024-03-15").unwrap();
        assert_eq!(date.year(), 2024);
    }

    #[test]
    fn test_weekend() {
        // 2024-03-16 是周六
        let date = PostingDate::new(NaiveDate::from_ymd_opt(2024, 3, 16).unwrap()).unwrap();
        assert!(date.is_weekend());
    }

    #[test]
    fn test_month_end() {
        let date = PostingDate::new(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()).unwrap();
        assert_eq!(date.month_end(), NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
    }

    #[test]
    fn test_display() {
        let date = PostingDate::from_str("2024-03-15").unwrap();
        assert_eq!(format!("{}", date), "2024-03-15");
    }
}
