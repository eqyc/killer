//! 会计科目代码（Account Code）
//!
//! 本模块实现会计科目代码值对象。
//!
//! # SAP 参考
//! - 表: SKA1（总账科目主数据）
//! - 表: SKB1（公司代码级科目数据）
//! - 字段: SAKNR（总账科目编号，10 位）
//! - 字段: KTOPL（科目表，4 位）
//! - 字段: XBILK（资产负债表科目标识）
//!
//! # 科目类型
//! SAP 将科目分为以下类型：
//! - 资产类（Assets）
//! - 负债类（Liabilities）
//! - 所有者权益类（Equity）
//! - 收入类（Revenue）
//! - 费用类（Expenses）
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::AccountCode;
//!
//! // 创建科目代码
//! let account = AccountCode::new("1001000000", "YCOA").unwrap();
//!
//! // 带前导零的科目
//! let account2 = AccountCode::with_leading_zeros("1001", "YCOA", 10).unwrap();
//! assert_eq!(account2.code(), "0000001001");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 科目代码最大长度
pub const ACCOUNT_CODE_MAX_LENGTH: usize = 10;

/// 科目表代码长度
pub const CHART_OF_ACCOUNTS_LENGTH: usize = 4;

/// 科目类型
///
/// 定义会计科目的分类
///
/// # SAP 参考
/// 对应 SKA1.XBILK 和科目编号范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// 资产类科目
    Asset,
    /// 负债类科目
    Liability,
    /// 所有者权益类科目
    Equity,
    /// 收入类科目
    Revenue,
    /// 费用类科目
    Expense,
    /// 成本类科目
    Cost,
    /// 其他科目
    Other,
}

impl AccountType {
    /// 获取科目类型的中文描述
    pub fn description_zh(&self) -> &'static str {
        match self {
            AccountType::Asset => "资产",
            AccountType::Liability => "负债",
            AccountType::Equity => "所有者权益",
            AccountType::Revenue => "收入",
            AccountType::Expense => "费用",
            AccountType::Cost => "成本",
            AccountType::Other => "其他",
        }
    }

    /// 判断是否为资产负债表科目
    ///
    /// # SAP 参考
    /// 对应 SKA1.XBILK = 'X'
    pub fn is_balance_sheet(&self) -> bool {
        matches!(
            self,
            AccountType::Asset | AccountType::Liability | AccountType::Equity
        )
    }

    /// 判断是否为损益表科目
    pub fn is_profit_loss(&self) -> bool {
        matches!(
            self,
            AccountType::Revenue | AccountType::Expense | AccountType::Cost
        )
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description_zh())
    }
}

/// 会计科目代码
///
/// 表示一个会计科目的编码，是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: SKA1（科目主数据）
/// - 字段: SAKNR（10 位字符）
/// - 字段: KTOPL（科目表，4 位）
///
/// # 编码规则
/// - 长度: 最多 10 位
/// - 字符: 数字和字母
/// - 支持前导零
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCode {
    /// 科目代码
    /// 对应 SKA1.SAKNR
    code: String,

    /// 科目表
    /// 对应 SKA1.KTOPL
    chart_of_accounts: String,

    /// 科目类型（可选）
    account_type: Option<AccountType>,
}

impl AccountCode {
    /// 创建新的科目代码
    ///
    /// # 参数
    /// - `code`: 科目代码（最多 10 位）
    /// - `chart_of_accounts`: 科目表代码（4 位）
    ///
    /// # 错误
    /// - 科目代码为空
    /// - 科目代码超过 10 位
    /// - 科目代码包含非法字符
    /// - 科目表代码不是 4 位
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::AccountCode;
    ///
    /// let account = AccountCode::new("1001000000", "YCOA").unwrap();
    /// assert_eq!(account.code(), "1001000000");
    /// ```
    pub fn new(code: impl Into<String>, chart_of_accounts: impl Into<String>) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();
        let chart_of_accounts = chart_of_accounts.into().trim().to_uppercase();

        // 验证科目代码
        if code.is_empty() {
            return Err(DomainError::account_code_invalid(
                code,
                "科目代码不能为空",
            ));
        }

        if code.len() > ACCOUNT_CODE_MAX_LENGTH {
            return Err(DomainError::account_code_invalid(
                code,
                format!("科目代码长度不能超过 {} 位", ACCOUNT_CODE_MAX_LENGTH),
            ));
        }

        // 验证字符（只允许数字和字母）
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::account_code_invalid(
                code,
                "科目代码只能包含数字和字母",
            ));
        }

        // 验证科目表代码
        if chart_of_accounts.len() != CHART_OF_ACCOUNTS_LENGTH {
            return Err(DomainError::account_code_invalid(
                code,
                format!("科目表代码必须是 {} 位", CHART_OF_ACCOUNTS_LENGTH),
            ));
        }

        Ok(Self {
            code,
            chart_of_accounts,
            account_type: None,
        })
    }

    /// 创建带前导零的科目代码
    ///
    /// # 参数
    /// - `code`: 科目代码（不含前导零）
    /// - `chart_of_accounts`: 科目表代码
    /// - `length`: 目标长度
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::AccountCode;
    ///
    /// let account = AccountCode::with_leading_zeros("1001", "YCOA", 10).unwrap();
    /// assert_eq!(account.code(), "0000001001");
    /// ```
    pub fn with_leading_zeros(
        code: impl Into<String>,
        chart_of_accounts: impl Into<String>,
        length: usize,
    ) -> DomainResult<Self> {
        let code = code.into().trim().to_uppercase();
        let padded = format!("{:0>width$}", code, width = length);
        Self::new(padded, chart_of_accounts)
    }

    /// 设置科目类型
    pub fn with_account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = Some(account_type);
        self
    }

    /// 获取科目代码
    pub fn code(&self) -> &str {
        &self.code
    }

    /// 获取科目表代码
    pub fn chart_of_accounts(&self) -> &str {
        &self.chart_of_accounts
    }

    /// 获取科目类型
    pub fn account_type(&self) -> Option<AccountType> {
        self.account_type
    }

    /// 获取不含前导零的科目代码
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::AccountCode;
    ///
    /// let account = AccountCode::new("0000001001", "YCOA").unwrap();
    /// assert_eq!(account.without_leading_zeros(), "1001");
    /// ```
    pub fn without_leading_zeros(&self) -> &str {
        self.code.trim_start_matches('0')
    }

    /// 判断是否为资产负债表科目
    pub fn is_balance_sheet(&self) -> bool {
        self.account_type
            .map(|t| t.is_balance_sheet())
            .unwrap_or(false)
    }

    /// 判断是否为损益表科目
    pub fn is_profit_loss(&self) -> bool {
        self.account_type
            .map(|t| t.is_profit_loss())
            .unwrap_or(false)
    }

    /// 判断两个科目是否属于同一科目表
    pub fn same_chart_of_accounts(&self, other: &AccountCode) -> bool {
        self.chart_of_accounts == other.chart_of_accounts
    }
}

impl PartialEq for AccountCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.chart_of_accounts == other.chart_of_accounts
    }
}

impl Eq for AccountCode {}

impl Hash for AccountCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        self.chart_of_accounts.hash(state);
    }
}

impl fmt::Display for AccountCode {
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
    fn test_create_account_code() {
        let account = AccountCode::new("1001000000", "YCOA").unwrap();
        assert_eq!(account.code(), "1001000000");
        assert_eq!(account.chart_of_accounts(), "YCOA");
    }

    #[test]
    fn test_create_with_lowercase() {
        let account = AccountCode::new("abc123", "ycoa").unwrap();
        assert_eq!(account.code(), "ABC123");
        assert_eq!(account.chart_of_accounts(), "YCOA");
    }

    #[test]
    fn test_create_with_whitespace() {
        let account = AccountCode::new("  1001  ", "  YCOA  ").unwrap();
        assert_eq!(account.code(), "1001");
        assert_eq!(account.chart_of_accounts(), "YCOA");
    }

    #[test]
    fn test_invalid_empty_code() {
        let result = AccountCode::new("", "YCOA");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_code_too_long() {
        let result = AccountCode::new("12345678901", "YCOA"); // 11 位
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_code_special_chars() {
        let result = AccountCode::new("1001-0000", "YCOA");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_chart_of_accounts_length() {
        let result = AccountCode::new("1001", "YCO"); // 3 位
        assert!(result.is_err());

        let result = AccountCode::new("1001", "YCOAA"); // 5 位
        assert!(result.is_err());
    }

    #[test]
    fn test_with_leading_zeros() {
        let account = AccountCode::with_leading_zeros("1001", "YCOA", 10).unwrap();
        assert_eq!(account.code(), "0000001001");
    }

    #[test]
    fn test_without_leading_zeros() {
        let account = AccountCode::new("0000001001", "YCOA").unwrap();
        assert_eq!(account.without_leading_zeros(), "1001");
    }

    #[test]
    fn test_without_leading_zeros_all_zeros() {
        let account = AccountCode::new("0000000000", "YCOA").unwrap();
        assert_eq!(account.without_leading_zeros(), "");
    }

    #[test]
    fn test_with_account_type() {
        let account = AccountCode::new("1001", "YCOA")
            .unwrap()
            .with_account_type(AccountType::Asset);

        assert_eq!(account.account_type(), Some(AccountType::Asset));
        assert!(account.is_balance_sheet());
        assert!(!account.is_profit_loss());
    }

    #[test]
    fn test_account_type_balance_sheet() {
        assert!(AccountType::Asset.is_balance_sheet());
        assert!(AccountType::Liability.is_balance_sheet());
        assert!(AccountType::Equity.is_balance_sheet());
        assert!(!AccountType::Revenue.is_balance_sheet());
        assert!(!AccountType::Expense.is_balance_sheet());
    }

    #[test]
    fn test_account_type_profit_loss() {
        assert!(!AccountType::Asset.is_profit_loss());
        assert!(AccountType::Revenue.is_profit_loss());
        assert!(AccountType::Expense.is_profit_loss());
        assert!(AccountType::Cost.is_profit_loss());
    }

    #[test]
    fn test_same_chart_of_accounts() {
        let a = AccountCode::new("1001", "YCOA").unwrap();
        let b = AccountCode::new("2001", "YCOA").unwrap();
        let c = AccountCode::new("1001", "ZCOA").unwrap();

        assert!(a.same_chart_of_accounts(&b));
        assert!(!a.same_chart_of_accounts(&c));
    }

    #[test]
    fn test_equality() {
        let a = AccountCode::new("1001", "YCOA").unwrap();
        let b = AccountCode::new("1001", "YCOA").unwrap();
        let c = AccountCode::new("1001", "ZCOA").unwrap();
        let d = AccountCode::new("2001", "YCOA").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AccountCode::new("1001", "YCOA").unwrap());
        set.insert(AccountCode::new("1001", "YCOA").unwrap());
        set.insert(AccountCode::new("2001", "YCOA").unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let account = AccountCode::new("1001000000", "YCOA").unwrap();
        assert_eq!(format!("{}", account), "1001000000");
    }

    #[test]
    fn test_serialization() {
        let account = AccountCode::new("1001", "YCOA")
            .unwrap()
            .with_account_type(AccountType::Asset);
        let json = serde_json::to_string(&account).unwrap();

        assert!(json.contains("\"code\":\"1001\""));
        assert!(json.contains("\"chart_of_accounts\":\"YCOA\""));
        assert!(json.contains("\"account_type\":\"ASSET\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"code":"1001","chart_of_accounts":"YCOA","account_type":"ASSET"}"#;
        let account: AccountCode = serde_json::from_str(json).unwrap();

        assert_eq!(account.code(), "1001");
        assert_eq!(account.chart_of_accounts(), "YCOA");
        assert_eq!(account.account_type(), Some(AccountType::Asset));
    }

    #[test]
    fn test_clone() {
        let original = AccountCode::new("1001", "YCOA").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn test_account_type_description() {
        assert_eq!(AccountType::Asset.description_zh(), "资产");
        assert_eq!(AccountType::Liability.description_zh(), "负债");
        assert_eq!(AccountType::Revenue.description_zh(), "收入");
    }
}
