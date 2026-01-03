//! 会计凭证聚合根

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::posting_date::PostingDate;
use crate::domain::entities::{
    journal_entry_item::{JournalEntryItem, DebitCreditIndicator},
    document::{Document, DocumentStatus},
};
use crate::domain::events::{
    JournalEntryCreated,
    JournalEntryPosted,
    JournalEntryReversed,
};
use killer_domain_primitives::{AccountCode, CompanyCode, CurrencyCode, DocumentNumber, DocumentType, Money};

/// 会计凭证聚合根
///
/// 代表完整的会计凭证，包含凭证头和行项目
#[derive(Debug, Clone)]
pub struct JournalEntry {
    /// 凭证基本信息
    document: Document,
    /// 行项目列表
    items: Vec<JournalEntryItem>,
    /// 借方总金额
    total_debit: Money,
    /// 贷方总金额
    total_credit: Money,
}

impl JournalEntry {
    /// 创建新的会计凭证
    pub fn new(
        document_type: DocumentType,
        document_number: DocumentNumber,
        document_date: chrono::NaiveDate,
        posting_date: chrono::NaiveDate,
        currency: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        let currency_str = currency.into();
        let currency_code = CurrencyCode::new(&currency_str).unwrap_or(CurrencyCode::CNY);
        Self {
            document: Document::new(
                document_type,
                document_number,
                document_date,
                posting_date,
                currency_str.clone(),
                created_by,
            ),
            items: Vec::new(),
            total_debit: Money::zero(currency_code),
            total_credit: Money::zero(currency_code),
        }
    }

    // Getters
    pub fn document_number(&self) -> &DocumentNumber {
        self.document.document_number()
    }

    pub fn fiscal_year(&self) -> i32 {
        self.document.fiscal_year()
    }

    pub fn company_code(&self) -> &str {
        self.document.company_code()
    }

    pub fn document_type(&self) -> DocumentType {
        self.document.document_type()
    }

    pub fn document_date(&self) -> chrono::NaiveDate {
        self.document.document_date()
    }

    pub fn posting_date(&self) -> chrono::NaiveDate {
        self.document.posting_date()
    }

    pub fn status(&self) -> DocumentStatus {
        self.document.status()
    }

    pub fn currency(&self) -> &str {
        self.document.currency()
    }

    pub fn header_text(&self) -> Option<&str> {
        self.document.header_text()
    }

    pub fn items(&self) -> &[JournalEntryItem] {
        &self.items
    }

    pub fn total_debit(&self) -> &Money {
        &self.total_debit
    }

    pub fn total_credit(&self) -> &Money {
        &self.total_credit
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debit == self.total_credit
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.document.created_at()
    }

    pub fn created_by(&self) -> &str {
        self.document.created_by()
    }

    // Commands

    /// 添加行项目
    pub fn add_item(&mut self, item: JournalEntryItem) -> Result<(), JournalEntryError> {
        // 验证凭证状态
        if !self.document.can_post() {
            return Err(JournalEntryError::InvalidStatus(self.document.status()));
        }

        // 验证借贷平衡
        let amount = item.document_currency_amount();
        if item.is_debit() {
            self.total_debit = self.total_debit.add(amount).unwrap();
        } else {
            self.total_credit = self.total_credit.add(amount).unwrap();
        }

        self.items.push(item);

        Ok(())
    }

    /// 批量添加行项目
    pub fn add_items(&mut self, items: Vec<JournalEntryItem>) -> Result<(), JournalEntryError> {
        for item in items {
            self.add_item(item)?;
        }
        Ok(())
    }

    /// 设置抬头文本
    pub fn set_header_text(&mut self, text: impl Into<String>) {
        self.document.set_header_text(text);
    }

    /// 设置参考凭证号
    pub fn set_reference_document(&mut self, reference: impl Into<String>) {
        self.document.set_reference_document(reference);
    }

    /// 过账
    pub fn post(&mut self) -> Result<(), JournalEntryError> {
        // 验证凭证状态
        if !self.document.can_post() {
            return Err(JournalEntryError::InvalidStatus(self.document.status()));
        }

        // 验证借贷平衡
        if !self.is_balanced() {
            return Err(JournalEntryError::NotBalanced {
                debit: self.total_debit.clone(),
                credit: self.total_credit.clone(),
            });
        }

        // 验证行项目
        if self.items.is_empty() {
            return Err(JournalEntryError::NoItems);
        }

        // 过账
        self.document.post();

        Ok(())
    }

    /// 冲销
    pub fn reverse(&mut self, reversal_date: chrono::NaiveDate, reason: &str) -> Result<DocumentNumber, JournalEntryError> {
        // 验证凭证状态
        if self.document.is_reversed() {
            return Err(JournalEntryError::AlreadyReversed);
        }

        if !matches!(self.document.status(), DocumentStatus::Posted) {
            return Err(JournalEntryError::InvalidStatus(self.document.status()));
        }

        // 验证借贷平衡
        if !self.is_balanced() {
            return Err(JournalEntryError::NotBalanced {
                debit: self.total_debit.clone(),
                credit: self.total_credit.clone(),
            });
        }

        // 生成冲销凭证号
        let doc_number = self.document_number();
        let reversal_doc_number = doc_number.next().map_err(|_| JournalEntryError::DocumentNumberOverflow)?;

        // 冲销
        self.document.reverse();

        Ok(reversal_doc_number)
    }

    // Events

    /// 生成创建事件
    pub fn into_created_event(self) -> JournalEntryCreated {
        JournalEntryCreated {
            company_code: self.document.company_code_value(),
            document_number: self.document_number().clone(),
            fiscal_year: self.fiscal_year().to_string(),
            total_debit: self.total_debit,
            total_credit: self.total_credit,
            created_at: self.document.created_at(),
        }
    }

    /// 生成过账事件
    pub fn into_posted_event(self) -> JournalEntryPosted {
        JournalEntryPosted {
            company_code: self.document.company_code_value(),
            document_number: self.document_number().clone(),
            fiscal_year: self.fiscal_year().to_string(),
            posted_at: Utc::now(),
        }
    }

    /// 生成冲销事件
    pub fn into_reversed_event(self, reversal_document: DocumentNumber) -> JournalEntryReversed {
        JournalEntryReversed {
            company_code: self.document.company_code_value(),
            original_document: self.document_number().clone(),
            reversal_document,
            fiscal_year: self.fiscal_year().to_string(),
            reversed_at: Utc::now(),
        }
    }
}

/// 凭证错误
#[derive(Debug, thiserror::Error)]
pub enum JournalEntryError {
    #[error("无效的凭证状态: {0:?}")]
    InvalidStatus(DocumentStatus),
    #[error("借贷不平衡: 借方={debit:?}, 贷方={credit:?}")]
    NotBalanced { debit: Money, credit: Money },
    #[error("凭证没有行项目")]
    NoItems,
    #[error("凭证已冲销，不能再次冲销")]
    AlreadyReversed,
    #[error("凭证编号溢出")]
    DocumentNumberOverflow,
    #[error("行项目添加失败: {0}")]
    ItemError(String),
}
