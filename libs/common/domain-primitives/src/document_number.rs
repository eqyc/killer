//! 凭证编号（Document Number）
//!
//! 本模块实现凭证编号值对象。
//!
//! # SAP 参考
//! - 表: BKPF（会计凭证抬头）
//! - 字段: BELNR（凭证编号，10 位）
//! - 字段: GJAHR（会计年度，4 位）
//! - 字段: BUKRS（公司代码，4 位）
//! - 字段: BLART（凭证类型，2 位）
//!
//! # 凭证唯一性
//! SAP 中凭证的唯一键是：凭证编号 + 公司代码 + 会计年度
//!
//! # 示例
//! ```rust
//! use killer_domain_primitives::DocumentNumber;
//!
//! // 创建凭证编号
//! let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
//!
//! // 获取完整标识
//! assert_eq!(doc.full_key(), "1000000001-1000-2024");
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::{DomainError, DomainResult};

/// 凭证编号长度
pub const DOCUMENT_NUMBER_LENGTH: usize = 10;

/// 公司代码长度
pub const COMPANY_CODE_LENGTH: usize = 4;

/// 凭证类型
///
/// 定义常见的凭证类型
///
/// # SAP 参考
/// 对应 BKPF.BLART
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    /// 总账凭证 (SA)
    GeneralLedger,
    /// 供应商发票 (RE)
    VendorInvoice,
    /// 供应商付款 (KZ)
    VendorPayment,
    /// 客户发票 (RV)
    CustomerInvoice,
    /// 客户收款 (DZ)
    CustomerPayment,
    /// 转账过账 (AB)
    ClearingDocument,
    /// 物料凭证 (WE)
    MaterialDocument,
    /// 资产凭证 (AA)
    AssetDocument,
    /// 其他
    Other,
}

impl DocumentType {
    /// 获取 SAP 凭证类型代码
    pub fn sap_code(&self) -> &'static str {
        match self {
            DocumentType::GeneralLedger => "SA",
            DocumentType::VendorInvoice => "RE",
            DocumentType::VendorPayment => "KZ",
            DocumentType::CustomerInvoice => "RV",
            DocumentType::CustomerPayment => "DZ",
            DocumentType::ClearingDocument => "AB",
            DocumentType::MaterialDocument => "WE",
            DocumentType::AssetDocument => "AA",
            DocumentType::Other => "XX",
        }
    }

    /// 获取凭证类型的中文描述
    pub fn description_zh(&self) -> &'static str {
        match self {
            DocumentType::GeneralLedger => "总账凭证",
            DocumentType::VendorInvoice => "供应商发票",
            DocumentType::VendorPayment => "供应商付款",
            DocumentType::CustomerInvoice => "客户发票",
            DocumentType::CustomerPayment => "客户收款",
            DocumentType::ClearingDocument => "清账凭证",
            DocumentType::MaterialDocument => "物料凭证",
            DocumentType::AssetDocument => "资产凭证",
            DocumentType::Other => "其他凭证",
        }
    }
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sap_code())
    }
}

/// 凭证编号
///
/// 表示一个会计凭证的唯一标识，是一个不可变的值对象。
///
/// # SAP 参考
/// - 表: BKPF
/// - 唯一键: BELNR + BUKRS + GJAHR
///
/// # 编码规则
/// - 凭证编号: 10 位数字
/// - 公司代码: 4 位字母数字
/// - 会计年度: 4 位数字
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNumber {
    /// 凭证编号
    /// 对应 BKPF.BELNR
    number: String,

    /// 会计年度
    /// 对应 BKPF.GJAHR
    fiscal_year: i32,

    /// 公司代码
    /// 对应 BKPF.BUKRS
    company_code: String,

    /// 凭证类型（可选）
    /// 对应 BKPF.BLART
    document_type: Option<DocumentType>,
}

impl DocumentNumber {
    /// 创建新的凭证编号
    ///
    /// # 参数
    /// - `number`: 凭证编号（10 位）
    /// - `fiscal_year`: 会计年度
    /// - `company_code`: 公司代码（4 位）
    ///
    /// # 错误
    /// - 凭证编号不是 10 位数字
    /// - 会计年度不合法
    /// - 公司代码不是 4 位
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::DocumentNumber;
    ///
    /// let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
    /// ```
    pub fn new(
        number: impl Into<String>,
        fiscal_year: i32,
        company_code: impl Into<String>,
    ) -> DomainResult<Self> {
        let number = number.into().trim().to_string();
        let company_code = company_code.into().trim().to_uppercase();

        // 验证凭证编号
        if number.len() != DOCUMENT_NUMBER_LENGTH {
            return Err(DomainError::document_number_invalid(
                number,
                format!("凭证编号必须是 {} 位", DOCUMENT_NUMBER_LENGTH),
            ));
        }

        if !number.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::document_number_invalid(
                number,
                "凭证编号必须是纯数字",
            ));
        }

        // 验证会计年度
        if fiscal_year < 1900 || fiscal_year > 2100 {
            return Err(DomainError::document_number_invalid(
                number,
                format!("会计年度 {} 不合法", fiscal_year),
            ));
        }

        // 验证公司代码
        if company_code.len() != COMPANY_CODE_LENGTH {
            return Err(DomainError::document_number_invalid(
                number,
                format!("公司代码必须是 {} 位", COMPANY_CODE_LENGTH),
            ));
        }

        if !company_code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::document_number_invalid(
                number,
                "公司代码只能包含字母和数字",
            ));
        }

        Ok(Self {
            number,
            fiscal_year,
            company_code,
            document_type: None,
        })
    }

    /// 从数字创建凭证编号（自动补零）
    ///
    /// # 参数
    /// - `number`: 凭证编号数字
    /// - `fiscal_year`: 会计年度
    /// - `company_code`: 公司代码
    ///
    /// # 示例
    /// ```rust
    /// use killer_domain_primitives::DocumentNumber;
    ///
    /// let doc = DocumentNumber::from_number(1, 2024, "1000").unwrap();
    /// assert_eq!(doc.number(), "0000000001");
    /// ```
    pub fn from_number(
        number: u64,
        fiscal_year: i32,
        company_code: impl Into<String>,
    ) -> DomainResult<Self> {
        let number_str = format!("{:0>width$}", number, width = DOCUMENT_NUMBER_LENGTH);
        Self::new(number_str, fiscal_year, company_code)
    }

    /// 设置凭证类型
    pub fn with_document_type(mut self, document_type: DocumentType) -> Self {
        self.document_type = Some(document_type);
        self
    }

    /// 获取凭证编号
    pub fn number(&self) -> &str {
        &self.number
    }

    /// 获取会计年度
    pub fn fiscal_year(&self) -> i32 {
        self.fiscal_year
    }

    /// 获取公司代码
    pub fn company_code(&self) -> &str {
        &self.company_code
    }

    /// 获取凭证类型
    pub fn document_type(&self) -> Option<DocumentType> {
        self.document_type
    }

    /// 获取凭证编号的数字值
    pub fn to_number(&self) -> u64 {
        self.number.parse().unwrap_or(0)
    }

    /// 获取完整的唯一键
    ///
    /// 格式: {凭证编号}-{公司代码}-{会计年度}
    pub fn full_key(&self) -> String {
        format!("{}-{}-{}", self.number, self.company_code, self.fiscal_year)
    }

    /// 获取下一个凭证编号
    ///
    /// # 错误
    /// - 编号溢出
    pub fn next(&self) -> DomainResult<Self> {
        let current = self.to_number();
        let next = current + 1;

        if next > 9999999999 {
            return Err(DomainError::document_number_invalid(
                self.number.clone(),
                "凭证编号已达到最大值",
            ));
        }

        let mut doc = Self::from_number(next, self.fiscal_year, &self.company_code)?;
        doc.document_type = self.document_type;
        Ok(doc)
    }
}

impl PartialEq for DocumentNumber {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
            && self.fiscal_year == other.fiscal_year
            && self.company_code == other.company_code
    }
}

impl Eq for DocumentNumber {}

impl Hash for DocumentNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
        self.fiscal_year.hash(state);
        self.company_code.hash(state);
    }
}

impl fmt::Display for DocumentNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_key())
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document_number() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        assert_eq!(doc.number(), "1000000001");
        assert_eq!(doc.fiscal_year(), 2024);
        assert_eq!(doc.company_code(), "1000");
    }

    #[test]
    fn test_from_number() {
        let doc = DocumentNumber::from_number(1, 2024, "1000").unwrap();
        assert_eq!(doc.number(), "0000000001");
    }

    #[test]
    fn test_from_number_large() {
        let doc = DocumentNumber::from_number(1234567890, 2024, "1000").unwrap();
        assert_eq!(doc.number(), "1234567890");
    }

    #[test]
    fn test_invalid_number_length() {
        let result = DocumentNumber::new("123", 2024, "1000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_non_numeric() {
        let result = DocumentNumber::new("12345678AB", 2024, "1000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_fiscal_year() {
        let result = DocumentNumber::new("1000000001", 1800, "1000");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_company_code_length() {
        let result = DocumentNumber::new("1000000001", 2024, "10");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_document_type() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000")
            .unwrap()
            .with_document_type(DocumentType::VendorInvoice);

        assert_eq!(doc.document_type(), Some(DocumentType::VendorInvoice));
    }

    #[test]
    fn test_to_number() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        assert_eq!(doc.to_number(), 1000000001);
    }

    #[test]
    fn test_full_key() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        assert_eq!(doc.full_key(), "1000000001-1000-2024");
    }

    #[test]
    fn test_next() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        let next = doc.next().unwrap();

        assert_eq!(next.number(), "1000000002");
        assert_eq!(next.fiscal_year(), 2024);
        assert_eq!(next.company_code(), "1000");
    }

    #[test]
    fn test_next_with_document_type() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000")
            .unwrap()
            .with_document_type(DocumentType::VendorInvoice);
        let next = doc.next().unwrap();

        assert_eq!(next.document_type(), Some(DocumentType::VendorInvoice));
    }

    #[test]
    fn test_next_overflow() {
        let doc = DocumentNumber::new("9999999999", 2024, "1000").unwrap();
        let result = doc.next();

        assert!(result.is_err());
    }

    #[test]
    fn test_equality() {
        let a = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        let b = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        let c = DocumentNumber::new("1000000001", 2023, "1000").unwrap();
        let d = DocumentNumber::new("1000000001", 2024, "2000").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DocumentNumber::new("1000000001", 2024, "1000").unwrap());
        set.insert(DocumentNumber::new("1000000001", 2024, "1000").unwrap());
        set.insert(DocumentNumber::new("1000000002", 2024, "1000").unwrap());

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        assert_eq!(format!("{}", doc), "1000000001-1000-2024");
    }

    #[test]
    fn test_document_type_sap_code() {
        assert_eq!(DocumentType::VendorInvoice.sap_code(), "RE");
        assert_eq!(DocumentType::CustomerPayment.sap_code(), "DZ");
    }

    #[test]
    fn test_document_type_description() {
        assert_eq!(DocumentType::VendorInvoice.description_zh(), "供应商发票");
        assert_eq!(DocumentType::GeneralLedger.description_zh(), "总账凭证");
    }

    #[test]
    fn test_serialization() {
        let doc = DocumentNumber::new("1000000001", 2024, "1000")
            .unwrap()
            .with_document_type(DocumentType::VendorInvoice);
        let json = serde_json::to_string(&doc).unwrap();

        assert!(json.contains("\"number\":\"1000000001\""));
        assert!(json.contains("\"fiscal_year\":2024"));
        assert!(json.contains("\"company_code\":\"1000\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"number":"1000000001","fiscal_year":2024,"company_code":"1000","document_type":null}"#;
        let doc: DocumentNumber = serde_json::from_str(json).unwrap();

        assert_eq!(doc.number(), "1000000001");
        assert_eq!(doc.fiscal_year(), 2024);
    }

    #[test]
    fn test_clone() {
        let original = DocumentNumber::new("1000000001", 2024, "1000").unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
