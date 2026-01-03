//! 文档实体

use chrono::{DateTime, Utc};
use killer_domain_primitives::{CompanyCode, DocumentNumber, DocumentType as PrimitivesDocumentType, Money};

/// 文档状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentStatus {
    Created = 1,   // 新建
    Posted = 2,    // 已过账
    Reversed = 3,  // 已冲销
}

impl TryFrom<i32> for DocumentStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Created),
            2 => Ok(Self::Posted),
            3 => Ok(Self::Reversed),
            _ => Err(()),
        }
    }
}

/// 通用文档实体
///
/// 为会计凭证提供通用的文档标识和状态管理
#[derive(Debug, Clone)]
pub struct Document {
    /// 凭证类型
    document_type: PrimitivesDocumentType,
    /// 凭证编号
    document_number: DocumentNumber,
    /// 凭证日期
    document_date: chrono::NaiveDate,
    /// 过账日期
    posting_date: chrono::NaiveDate,
    /// 凭证状态
    status: DocumentStatus,
    /// 凭证货币
    currency: String,
    /// 参考凭证号
    reference_document: Option<String>,
    /// 凭证抬头文本
    header_text: Option<String>,
    /// 来源系统
    source_system: Option<String>,
    /// 创建信息
    created_at: DateTime<Utc>,
    created_by: String,
    /// 修改信息
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<String>,
}

impl Document {
    /// 创建新文档
    pub fn new(
        document_type: PrimitivesDocumentType,
        document_number: DocumentNumber,
        document_date: chrono::NaiveDate,
        posting_date: chrono::NaiveDate,
        currency: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        Self {
            document_type,
            document_number,
            document_date,
            posting_date,
            status: DocumentStatus::Created,
            currency: currency.into(),
            reference_document: None,
            header_text: None,
            source_system: None,
            created_at: Utc::now(),
            created_by: created_by.into(),
            updated_at: None,
            updated_by: None,
        }
    }

    // Getters
    pub fn document_type(&self) -> PrimitivesDocumentType {
        self.document_type
    }

    pub fn document_number(&self) -> &DocumentNumber {
        &self.document_number
    }

    pub fn fiscal_year(&self) -> i32 {
        self.document_number.fiscal_year()
    }

    pub fn company_code(&self) -> &str {
        self.document_number.company_code()
    }

    pub fn company_code_value(&self) -> CompanyCode {
        CompanyCode::new(self.document_number.company_code()).unwrap()
    }

    pub fn document_date(&self) -> chrono::NaiveDate {
        self.document_date
    }

    pub fn posting_date(&self) -> chrono::NaiveDate {
        self.posting_date
    }

    pub fn status(&self) -> DocumentStatus {
        self.status
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn reference_document(&self) -> Option<&str> {
        self.reference_document.as_deref()
    }

    pub fn header_text(&self) -> Option<&str> {
        self.header_text.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    // Commands

    /// 设置凭证状态
    pub fn set_status(&mut self, status: DocumentStatus) {
        self.status = status;
    }

    /// 检查是否可以过账
    pub fn can_post(&self) -> bool {
        matches!(self.status, DocumentStatus::Created)
    }

    /// 检查是否已冲销
    pub fn is_reversed(&self) -> bool {
        matches!(self.status, DocumentStatus::Reversed)
    }

    /// 过账
    pub fn post(&mut self) {
        self.status = DocumentStatus::Posted;
    }

    /// 冲销
    pub fn reverse(&mut self) {
        self.status = DocumentStatus::Reversed;
    }

    /// 设置参考凭证号
    pub fn set_reference_document(&mut self, reference: impl Into<String>) {
        self.reference_document = Some(reference.into());
    }

    /// 设置抬头文本
    pub fn set_header_text(&mut self, text: impl Into<String>) {
        self.header_text = Some(text.into());
    }
}
