//! 币种代码（Currency Code）
//!
//! 本模块实现 ISO 4217 标准的币种代码值对象。
//!
//! # SAP 参考
//! - 表: TCURC（币种主数据）
//! - 字段: WAERS（币种代码，5位字符）
//! - 表: TCURX（币种小数位配置）
//!
//! # 示例
//! ```
//! use killer_domain_primitives::CurrencyCode;
//!
//! let cny = CurrencyCode::CNY;
//! let usd = CurrencyCode::try_from("USD").unwrap();
//! assert_eq!(usd.as_str(), "USD");
//! assert_eq!(usd.decimal_places(), 2);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

use crate::error::{DomainError, DomainResult};

/// 币种代码
///
/// 符合 ISO 4217 标准的三位字母币种代码。
/// 预定义了常用币种，同时支持自定义币种。
///
/// # SAP 对应
/// - TCURC.WAERS: 币种代码
/// - TCURX.CURRKEY: 币种小数位数
///
/// # 不可变性
/// 币种代码一旦创建不可修改，符合 DDD 值对象原则。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyCode {
    /// 内部存储的币种代码（3位大写字母）
    code: [u8; 3],
}

impl CurrencyCode {
    // =========================================================================
    // 常用币种常量
    // =========================================================================

    /// 人民币
    pub const CNY: Self = Self::from_static(b"CNY");
    /// 美元
    pub const USD: Self = Self::from_static(b"USD");
    /// 欧元
    pub const EUR: Self = Self::from_static(b"EUR");
    /// 日元
    pub const JPY: Self = Self::from_static(b"JPY");
    /// 英镑
    pub const GBP: Self = Self::from_static(b"GBP");
    /// 港币
    pub const HKD: Self = Self::from_static(b"HKD");
    /// 澳元
    pub const AUD: Self = Self::from_static(b"AUD");
    /// 加元
    pub const CAD: Self = Self::from_static(b"CAD");
    /// 瑞士法郎
    pub const CHF: Self = Self::from_static(b"CHF");
    /// 新加坡元
    pub const SGD: Self = Self::from_static(b"SGD");
    /// 韩元
    pub const KRW: Self = Self::from_static(b"KRW");
    /// 台币
    pub const TWD: Self = Self::from_static(b"TWD");
    /// 印度卢比
    pub const INR: Self = Self::from_static(b"INR");
    /// 俄罗斯卢布
    pub const RUB: Self = Self::from_static(b"RUB");

    /// 从静态字节数组创建（编译时常量）
    const fn from_static(bytes: &[u8; 3]) -> Self {
        Self { code: *bytes }
    }

    /// 创建新的币种代码
    ///
    /// # 参数
    /// - `code`: 3位字母的币种代码
    ///
    /// # 错误
    /// - 如果代码不是3位字母，返回 `InvalidCurrencyCode` 错误
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// let cny = CurrencyCode::new("CNY").unwrap();
    /// assert_eq!(cny.as_str(), "CNY");
    ///
    /// // 小写会自动转换为大写
    /// let usd = CurrencyCode::new("usd").unwrap();
    /// assert_eq!(usd.as_str(), "USD");
    /// ```
    pub fn new(code: &str) -> DomainResult<Self> {
        Self::validate_and_create(code)
    }

    /// 验证并创建币种代码
    fn validate_and_create(code: &str) -> DomainResult<Self> {
        let code = code.trim();

        // 检查长度
        if code.len() != 3 {
            return Err(DomainError::InvalidCurrencyCode(code.to_string()));
        }

        // 检查是否全为字母并转换为大写
        let bytes = code.as_bytes();
        let mut result = [0u8; 3];

        for (i, &b) in bytes.iter().enumerate() {
            if b.is_ascii_alphabetic() {
                result[i] = b.to_ascii_uppercase();
            } else {
                return Err(DomainError::InvalidCurrencyCode(code.to_string()));
            }
        }

        Ok(Self { code: result })
    }

    /// 获取币种代码字符串
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// assert_eq!(CurrencyCode::CNY.as_str(), "CNY");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: 我们在构造时已确保只包含 ASCII 字母
        unsafe { std::str::from_utf8_unchecked(&self.code) }
    }

    /// 获取该币种的标准小数位数
    ///
    /// 参考 ISO 4217 标准和 SAP TCURX 表配置。
    ///
    /// # 返回值
    /// - 大多数币种返回 2（如 CNY, USD, EUR）
    /// - 日元、韩元等返回 0
    /// - 科威特第纳尔等返回 3
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// assert_eq!(CurrencyCode::CNY.decimal_places(), 2);
    /// assert_eq!(CurrencyCode::JPY.decimal_places(), 0);
    /// ```
    #[must_use]
    pub const fn decimal_places(&self) -> u8 {
        match &self.code {
            // 0 位小数的币种
            b"JPY" | b"KRW" | b"VND" | b"IDR" | b"CLP" => 0,
            // 3 位小数的币种
            b"KWD" | b"BHD" | b"OMR" => 3,
            // 默认 2 位小数
            _ => 2,
        }
    }

    /// 获取币种符号
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// assert_eq!(CurrencyCode::CNY.symbol(), "¥");
    /// assert_eq!(CurrencyCode::USD.symbol(), "$");
    /// ```
    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        match &self.code {
            b"CNY" => "¥",
            b"USD" => "$",
            b"EUR" => "€",
            b"GBP" => "£",
            b"JPY" => "¥",
            b"HKD" => "HK$",
            b"KRW" => "₩",
            b"INR" => "₹",
            b"RUB" => "₽",
            b"CHF" => "CHF",
            _ => "",
        }
    }

    /// 获取币种中文名称
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// assert_eq!(CurrencyCode::CNY.chinese_name(), "人民币");
    /// assert_eq!(CurrencyCode::USD.chinese_name(), "美元");
    /// ```
    #[must_use]
    pub const fn chinese_name(&self) -> &'static str {
        match &self.code {
            b"CNY" => "人民币",
            b"USD" => "美元",
            b"EUR" => "欧元",
            b"GBP" => "英镑",
            b"JPY" => "日元",
            b"HKD" => "港币",
            b"AUD" => "澳元",
            b"CAD" => "加元",
            b"CHF" => "瑞士法郎",
            b"SGD" => "新加坡元",
            b"KRW" => "韩元",
            b"TWD" => "新台币",
            b"INR" => "印度卢比",
            b"RUB" => "俄罗斯卢布",
            _ => "未知币种",
        }
    }

    /// 检查是否为常用币种
    #[must_use]
    pub const fn is_major_currency(&self) -> bool {
        matches!(
            &self.code,
            b"CNY" | b"USD" | b"EUR" | b"GBP" | b"JPY" | b"HKD"
        )
    }

    /// 获取币种代码字符串（code() 是 as_str() 的别名）
    ///
    /// # 示例
    /// ```
    /// use killer_domain_primitives::CurrencyCode;
    ///
    /// assert_eq!(CurrencyCode::CNY.code(), "CNY");
    /// ```
    #[must_use]
    pub fn code(&self) -> &str {
        self.as_str()
    }

    // =========================================================================
    // 工厂方法
    // =========================================================================

    /// 创建人民币币种代码
    #[must_use]
    pub const fn cny() -> Self {
        Self::CNY
    }

    /// 创建美元币种代码
    #[must_use]
    pub const fn usd() -> Self {
        Self::USD
    }

    /// 创建欧元币种代码
    #[must_use]
    pub const fn eur() -> Self {
        Self::EUR
    }

    /// 创建日元币种代码
    #[must_use]
    pub const fn jpy() -> Self {
        Self::JPY
    }

    /// 创建英镑币种代码
    #[must_use]
    pub const fn gbp() -> Self {
        Self::GBP
    }

    /// 创建港币币种代码
    #[must_use]
    pub const fn hkd() -> Self {
        Self::HKD
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for CurrencyCode {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl From<CurrencyCode> for String {
    fn from(code: CurrencyCode) -> Self {
        code.as_str().to_string()
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for CurrencyCode {
    /// 默认币种为人民币（CNY）
    fn default() -> Self {
        Self::CNY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code_creation() {
        let cny = CurrencyCode::new("CNY").unwrap();
        assert_eq!(cny.as_str(), "CNY");

        // 小写自动转大写
        let usd = CurrencyCode::new("usd").unwrap();
        assert_eq!(usd.as_str(), "USD");

        // 混合大小写
        let eur = CurrencyCode::new("Eur").unwrap();
        assert_eq!(eur.as_str(), "EUR");
    }

    #[test]
    fn test_currency_code_validation() {
        // 长度错误
        assert!(CurrencyCode::new("CN").is_err());
        assert!(CurrencyCode::new("CNYX").is_err());

        // 包含数字
        assert!(CurrencyCode::new("CN1").is_err());

        // 包含特殊字符
        assert!(CurrencyCode::new("CN-").is_err());

        // 空字符串
        assert!(CurrencyCode::new("").is_err());
    }

    #[test]
    fn test_currency_code_constants() {
        assert_eq!(CurrencyCode::CNY.as_str(), "CNY");
        assert_eq!(CurrencyCode::USD.as_str(), "USD");
        assert_eq!(CurrencyCode::EUR.as_str(), "EUR");
        assert_eq!(CurrencyCode::JPY.as_str(), "JPY");
    }

    #[test]
    fn test_decimal_places() {
        assert_eq!(CurrencyCode::CNY.decimal_places(), 2);
        assert_eq!(CurrencyCode::USD.decimal_places(), 2);
        assert_eq!(CurrencyCode::JPY.decimal_places(), 0);
        assert_eq!(CurrencyCode::KRW.decimal_places(), 0);

        // 科威特第纳尔 3 位小数
        let kwd = CurrencyCode::new("KWD").unwrap();
        assert_eq!(kwd.decimal_places(), 3);
    }

    #[test]
    fn test_currency_symbol() {
        assert_eq!(CurrencyCode::CNY.symbol(), "¥");
        assert_eq!(CurrencyCode::USD.symbol(), "$");
        assert_eq!(CurrencyCode::EUR.symbol(), "€");
        assert_eq!(CurrencyCode::GBP.symbol(), "£");
    }

    #[test]
    fn test_chinese_name() {
        assert_eq!(CurrencyCode::CNY.chinese_name(), "人民币");
        assert_eq!(CurrencyCode::USD.chinese_name(), "美元");
        assert_eq!(CurrencyCode::EUR.chinese_name(), "欧元");
    }

    #[test]
    fn test_equality() {
        let cny1 = CurrencyCode::new("CNY").unwrap();
        let cny2 = CurrencyCode::CNY;
        assert_eq!(cny1, cny2);

        let usd = CurrencyCode::USD;
        assert_ne!(cny1, usd);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CurrencyCode::CNY);
        set.insert(CurrencyCode::USD);
        set.insert(CurrencyCode::CNY); // 重复

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_serialization() {
        let cny = CurrencyCode::CNY;
        let json = serde_json::to_string(&cny).unwrap();
        assert_eq!(json, "\"CNY\"");

        let deserialized: CurrencyCode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, cny);
    }

    #[test]
    fn test_deserialization_lowercase() {
        let json = "\"usd\"";
        let usd: CurrencyCode = serde_json::from_str(json).unwrap();
        assert_eq!(usd, CurrencyCode::USD);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", CurrencyCode::CNY), "CNY");
    }

    #[test]
    fn test_from_str() {
        let cny: CurrencyCode = "CNY".parse().unwrap();
        assert_eq!(cny, CurrencyCode::CNY);
    }

    #[test]
    fn test_default() {
        assert_eq!(CurrencyCode::default(), CurrencyCode::CNY);
    }

    #[test]
    fn test_is_major_currency() {
        assert!(CurrencyCode::CNY.is_major_currency());
        assert!(CurrencyCode::USD.is_major_currency());
        assert!(!CurrencyCode::new("THB").unwrap().is_major_currency());
    }
}
