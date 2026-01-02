//! 公司代码（Company Code）
//!
//! 本模块实现公司代码值对象。
//!
//! # SAP 参考
//! - 表: T001（公司代码主数据）
//! - 字段: BUKRS（公司代码，4 位）
//! - 字段: BUTXT（公司名称）
//! - 字段: WAERS（本位币）
//! - 字段: KTOPL（科目表）
//!
//! # 组织架构
//! 公司代码是 SAP 财务组织的核心单元：
//! - 所有财务凭证必须关联公司代码
//! - 每个公司代码有独立的本位币和科目表
//! - 公司代码是法人实体的财务视图
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::CompanyCode;
//!
//! // 创建公司代码
//! let company = CompanyCode::new("1000").unwrap();
//! assert_eq!(company.code(), "1000");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 公司代码长度
pub const COMPANY_CODE_LENGTH: usize = 4;

/// 公司代码
///
/// 表示一个公司代码，是 SAP 财务组织的核心单元。
/// 这是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: T001
/// - 字段: BUKRS（4 位字母数字）
///
/// # 编码规则
/// - 长度: 4 位
/// - 字符: 字母和数字
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyCode {
    /// 公司代码
    /// 对应 T001.BUKRS
    code: String,

    /// 公司名称（可选）
    /// 对应 T001.BUTXT
    name: Option<String>,

    /// 本位币代码（可选）
    /// 对应 T001.WAERS
    currency_code: Option<String>,

    /// 科目表代码（可选）
    /// 对应 T001.KTOPL
    chart_of_accounts: Option<String>,
}

impl CompanyCode {
    /// 创建新的公司代码
    ///
    /// # 参数
    /// - `code`: 公司代码（4 位）
    ///
    /// # 错误
    /// - 代码不是 4 位
    /// - 代码包含非法字符
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::CompanyCode;
    ///
    /// let company = CompanyCode::new("1000").unwrap();
    /// assert_eq!(company.code(), "1000");
    /// ```
    pub fn new(code: impl Into<String>) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();

        // 验证长度
        if code.len() != COMPANY_CODE_LENGTH {
            return Err(DomainError::company_code_invalid(
                code,
                format!("公司代码必须是 {} 位", COMPANY_CODE_LENGTH),
            ));
        }

        // 验证字符
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::company_code_invalid(
                code,
                "公司代码只能包含字母和数字",
            ));
        }

        Ok(Self {
            code,
            name: None,
            currency_code: None,
            chart_of_accounts: None,
        })
    }

    /// 设置公司名称
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置本位币代码
    pub fn with_currency_code(mut self, currency_code: impl Into<String>) -> Self {
        self.currency_code = Some(currency_code.into().to_uppercase());
        self
    }

    /// 设置科目表代码
    pub fn with_chart_of_accounts(mut self, chart_of_accounts: impl Into<String>) -> Self {
        self.chart_of_accounts = Some(chart_of_accounts.into().to_uppercase());
        self
    }

    /// 获取公司代码
    pub fn code(&self) -> &str {
        &self.code
    }

    /// 获取公司名称
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// 获取本位币代码
    pub fn currency_code(&self) -> Option<&str> {
        self.currency_code.as_deref()
    }

    /// 获取科目表代码
    pub fn chart_of_accounts(&self) -> Option<&str> {
        self.chart_of_accounts.as_deref()
    }
}

impl PartialEq for CompanyCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for CompanyCode {}

impl Hash for CompanyCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl fmt::Display for CompanyCode {
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
    fn test_create_company_code() {
        let company = CompanyCode::new("1000").unwrap();
        assert_eq!(company.code(), "1000");
    }

    #[test]
    fn test_create_with_letters() {
        let company = CompanyCode::new("AB01").unwrap();
        assert_eq!(company.code(), "AB01");
    }

    #[test]
    fn test_create_with_lowercase() {
        let company = CompanyCode::new("ab01").unwrap();
        assert_eq!(company.code(), "AB01");
    }

    #[test]
    fn test_create_with_whitespace() {
        let company = CompanyCode::new("  1000  ").unwrap();
        assert_eq!(company.code(), "1000");
    }

    #[test]
    fn test_invalid_length_short() {
        let result = CompanyCode::new("100");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_length_long() {
        let result = CompanyCode::new("10000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_special_chars() {
        let result = CompanyCode::new("10-0");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_name() {
        let company = CompanyCode::new("1000")
            .unwrap()
            .with_name("测试公司");

        assert_eq!(company.name(), Some("测试公司"));
    }

    #[test]
    fn test_with_currency_code() {
        let company = CompanyCode::new("1000")
            .unwrap()
            .with_currency_code("cny");

        assert_eq!(company.currency_code(), Some("CNY"));
    }

    #[test]
    fn test_with_chart_of_accounts() {
        let company = CompanyCode::new("1000")
            .unwrap()
            .with_chart_of_accounts("ycoa");

        assert_eq!(company.chart_of_accounts(), Some("YCOA"));
    }

    #[test]
    fn test_full_builder() {
        let company = CompanyCode::new("1000")
            .unwrap()
            .with_name("测试公司")
            .with_currency_code("CNY")
            .with_chart_of_accounts("YCOA");

        assert_eq!(company.code(), "1000");
        assert_eq!(company.name(), Some("测试公司"));
        assert_eq!(company.currency_code(), Some("CNY"));
        assert_eq!(company.chart_of_accounts(), Some("YCOA"));
    }

    #[test]
    fn test_equality() {
        let a = CompanyCode::new("1000").unwrap();
        let b = CompanyCode::new("1000").unwrap().with_name("不同名称");
        let c = CompanyCode::new("2000").unwrap();

        // 相等性只比较代码
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CompanyCode::new("1000").unwrap());
        set.insert(CompanyCode::new("1000").unwrap().with_name("名称"));
        set.insert(CompanyCode::new("2000").unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let company = CompanyCode::new("1000").unwrap();
        assert_eq!(format!("{}", company), "1000");
    }

    #[test]
    fn test_serialization() {
        let company = CompanyCode::new("1000")
            .unwrap()
            .with_name("测试公司");
        let json = serde_json::to_string(&company).unwrap();

        assert!(json.contains("\"code\":\"1000\""));
        assert!(json.contains("\"name\":\"测试公司\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"code":"1000","name":"测试公司","currency_code":null,"chart_of_accounts":null}"#;
        let company: CompanyCode = serde_json::from_str(json).unwrap();

        assert_eq!(company.code(), "1000");
        assert_eq!(company.name(), Some("测试公司"));
    }

    #[test]
    fn test_clone() {
        let original = CompanyCode::new("1000").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
