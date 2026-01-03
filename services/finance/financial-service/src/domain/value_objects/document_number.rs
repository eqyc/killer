//! 凭证号值对象

use std::fmt;
use std::str::FromStr;

/// 会计凭证号
///
/// 代表 SAP 风格的凭证号（10位数字）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentNumber(String);

impl DocumentNumber {
    /// 创建新的凭证号
    pub fn new(number: impl Into<String>) -> Result<Self, DocumentNumberError> {
        let number = number.into();
        Self::validate(&number)?;
        Ok(Self(number))
    }

    /// 从数字创建凭证号（自动补零）
    pub fn from_number(number: u32) -> Self {
        Self(format!("{:010}", number))
    }

    /// 验证凭证号格式
    fn validate(number: &str) -> Result<(), DocumentNumberError> {
        if number.is_empty() {
            return Err(DocumentNumberError::Empty);
        }
        if number.len() > 10 {
            return Err(DocumentNumberError::TooLong(number.len()));
        }
        if !number.chars().all(|c| c.is_ascii_digit()) {
            return Err(DocumentNumberError::InvalidFormat(number.clone()));
        }
        Ok(())
    }

    /// 获取内部字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 转换为 u32
    pub fn to_u32(&self) -> Option<u32> {
        self.0.parse().ok()
    }

    /// 获取长度
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for DocumentNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DocumentNumber {
    type Err = DocumentNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<DocumentNumber> for String {
    fn from(val: DocumentNumber) -> Self {
        val.0
    }
}

impl AsRef<str> for DocumentNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// 凭证号错误
#[derive(Debug, thiserror::Error)]
pub enum DocumentNumberError {
    #[error("凭证号不能为空")]
    Empty,
    #[error("凭证号长度不能超过10个字符 (实际: {0})")]
    TooLong(usize),
    #[error("凭证号格式无效 (必须为数字): {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_document_number() {
        let num = DocumentNumber::new("0000001234").unwrap();
        assert_eq!(num.as_str(), "0000001234");
        assert_eq!(num.to_u32(), Some(1234));
    }

    #[test]
    fn test_from_number() {
        let num = DocumentNumber::from_number(1234);
        assert_eq!(num.as_str(), "0000001234");
    }

    #[test]
    fn test_invalid_format() {
        assert!(DocumentNumber::new("1234ABC").is_err());
    }

    #[test]
    fn test_empty_number() {
        assert!(DocumentNumber::new("").is_err());
    }
}
