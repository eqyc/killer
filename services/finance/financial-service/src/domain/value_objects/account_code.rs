//! 总账科目代码值对象

use std::fmt;
use std::str::FromStr;
use crate::domain::value_objects::account_code::AccountCodeError;

/// 总账科目代码
///
/// 代表 SAP 风格的科目代码（10位数字/字母组合）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountCode(String);

impl AccountCode {
    /// 创建新的科目代码
    pub fn new(code: impl Into<String>) -> Result<Self, AccountCodeError> {
        let code = code.into();
        Self::validate(&code)?;
        Ok(Self(code))
    }

    /// 验证科目代码格式
    fn validate(code: &str) -> Result<(), AccountCodeError> {
        if code.is_empty() {
            return Err(AccountCodeError::Empty);
        }
        if code.len() > 10 {
            return Err(AccountCodeError::TooLong(code.len()));
        }
        if !code.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(AccountCodeError::InvalidFormat(code.clone()));
        }
        Ok(())
    }

    /// 获取内部字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 获取长度
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for AccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AccountCode {
    type Err = AccountCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<AccountCode> for String {
    fn from(val: AccountCode) -> Self {
        val.0
    }
}

impl AsRef<str> for AccountCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// 科目代码错误
#[derive(Debug, thiserror::Error)]
pub enum AccountCodeError {
    #[error("科目代码不能为空")]
    Empty,
    #[error("科目代码长度不能超过10个字符 (实际: {0})")]
    TooLong(usize),
    #[error("科目代码格式无效: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_account_code() {
        let code = AccountCode::new("1000010000").unwrap();
        assert_eq!(code.as_str(), "1000010000");
        assert_eq!(code.len(), 10);
    }

    #[test]
    fn test_invalid_empty_code() {
        assert!(AccountCode::new("").is_err());
    }

    #[test]
    fn test_too_long_code() {
        assert!(AccountCode::new("12345678901").is_err());
    }

    #[test]
    fn test_display() {
        let code = AccountCode::new("1000").unwrap();
        assert_eq!(format!("{}", code), "1000");
    }
}
