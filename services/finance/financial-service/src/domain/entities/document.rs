//! 文档实体

use chrono::{DateTime, Utc};

/// 通用文档实体
///
/// 为会计凭证提供通用的文档标识和状态管理
#[derive(Debug, Clone)]
pub struct Document {
    /// 凭证类型
    document_type: DocumentType,
    /// 凭证号
    document_number: String,
    /// 会计年度
    fiscal_year: String,
    /// 公司代码
    company_code: String,
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
        document_type: DocumentType,
        document_number: impl Into<String>,
        fiscal_year: impl Into<String>,
        company_code: impl Into<String>,
        document_date: chrono::NaiveDate,
        posting_date: chrono::NaiveDate,
        currency: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        Self {
            document_type,
            document_number: document_number.into(),
            fiscal_year: fiscal_year.into(),
            company_code: company_code.into(),
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
    pub fn document_type(&self) -> DocumentType {
        self.document_type
    }

    pub fn document_number(&self) -> &str {
        &self.document_number
    }

    pub fn fiscal_year(&self) -> &str {
        &self.fiscal_year
    }

    pub fn company_code(&self) -> &str {
        &self.company_code
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

    pub fn source_system(&self) -> Option<&str> {
        self.source_system.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn created_by(&self) -> &str {
        &self.created_by
    }

    /// 更新状态
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        self.status = new_status;
        self.updated_at = Some(Utc::now());
    }

    /// 设置参考凭证号
    pub fn set_reference(&mut self, reference: impl Into<String>) {
        self.reference_document = Some(reference.into());
        self.updated_at = Some(Utc::now());
    }

    /// 设置抬头文本
    pub fn set_header_text(&mut self, text: impl Into<String>) {
        self.header_text = Some(text.into());
        self.updated_at = Some(Utc::now());
    }

    /// 判断是否可以过账
    pub fn can_post(&self) -> bool {
        self.status == DocumentStatus::Created
    }

    /// 判断是否可以冲销
    pub fn can_reverse(&self) -> bool {
        self.status == DocumentStatus::Posted
    }

    /// 判断是否已删除
    pub fn is_deleted(&self) -> bool {
        self.status == DocumentStatus::Deleted
    }
}

/// 凭证类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DocumentType {
    /// 预制凭证
    PreDocument,
    /// 标准凭证
    StandardDocument,
    /// 收票凭证
    InvoiceReceipt,
    /// 付款凭证
    PaymentDocument,
    /// 调整凭证
    AdjustmentDocument,
    /// 冲销凭证
    ReversalDocument,
    /// 年度结转凭证
    YearEndClosing,
}

/// 凭证状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentStatus {
    Created = 1,   // 已创建
    Posted = 2,    // 已过账
    Reversed = 3,  // 已冲销
    Blocked = 4,   // 已冻结
    Deleted = 5,   // 已删除
}

impl TryFrom<i32> for DocumentStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Created),
            2 => Ok(Self::Posted),
            3 => Ok(Self::Reversed),
            4 => Ok(Self::Blocked),
            5 => Ok(Self::Deleted),
            _ => Err(()),
        }
    }
}
