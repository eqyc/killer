//! 会计凭证聚合根

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::{
    account_code::AccountCode,
    document_number::DocumentNumber,
    posting_date::PostingDate,
};
use crate::domain::entities::{
    journal_entry_item::{JournalEntryItem, DebitCreditIndicator},
    document::{Document, DocumentType, DocumentStatus},
};
use crate::domain::events::{
    JournalEntryCreated,
    JournalEntryPosted,
    JournalEntryReversed,
};
use killer_domain_primitives::{CompanyCode, Money};

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
        fiscal_year: String,
        company_code: CompanyCode,
        document_date: chrono::NaiveDate,
        posting_date: chrono::NaiveDate,
        currency: impl Into<String>,
        created_by: impl Into<String>,
    ) -> Self {
        Self {
            document: Document::new(
                document_type,
                document_number,
                fiscal_year,
                company_code,
                document_date,
                posting_date,
                currency,
                created_by,
            ),
            items: Vec::new(),
            total_debit: Money::zero(),
            total_credit: Money::zero(),
        }
    }

    // Getters
    pub fn document_number(&self) -> &str {
        self.document.document_number()
    }

    pub fn fiscal_year(&self) -> &str {
        self.document.fiscal_year()
    }

    pub fn company_code(&self) -> &CompanyCode {
        &self.document.company_code().clone()
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

    pub fn currency(&self) -> &str {
        self.document.currency()
    }

    pub fn status(&self) -> DocumentStatus {
        self.document.status()
    }

    pub fn header_text(&self) -> Option<&str> {
        self.document.header_text()
    }

    pub fn reference_document(&self) -> Option<&str> {
        self.document.reference_document()
    }

    pub fn items(&self) -> &[JournalEntryItem] {
        &self.items
    }

    pub fn total_debit(&self) -> Money {
        self.total_debit
    }

    pub fn total_credit(&self) -> Money {
        self.total_credit
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debit == self.total_credit
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
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
            self.total_debit = self.total_debit.add(amount);
        } else {
            self.total_credit = self.total_credit.add(amount);
        }

        // 设置行号
        let line_number = (self.items.len() + 1) as u32;
        // 注意：实际项目中需要克隆item，这里简化处理
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
    pub fn set_reference(&mut self, reference: impl Into<String>) {
        self.document.set_reference(reference);
    }

    /// 过账凭证
    pub fn post(&mut self) -> Result<(), JournalEntryError> {
        // 验证状态
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

        // 验证至少有一行
        if self.items.is_empty() {
            return Err(JournalEntryError::NoItems);
        }

        self.document.update_status(DocumentStatus::Posted);
        Ok(())
    }

    /// 冲销凭证
    pub fn reverse(&mut self, reversal_date: chrono::NaiveDate, reason: &str) -> Result<DocumentNumber, JournalEntryError> {
        // 验证状态
        if !self.document.can_reverse() {
            return Err(JournalEntryError::InvalidStatus(self.document.status()));
        }

        self.document.update_status(DocumentStatus::Reversed);

        // 返回冲销凭证号（实际业务中需要生成新的凭证号）
        Ok(DocumentNumber::from_str(&format!("REV{}", self.document_number())).unwrap())
    }

    /// 冻结凭证
    pub fn block(&mut self) {
        self.document.update_status(DocumentStatus::Blocked);
    }

    /// 解冻凭证
    pub fn unblock(&mut self) {
        self.document.update_status(DocumentStatus::Created);
    }

    /// 删除凭证
    pub fn delete(&mut self) {
        self.document.update_status(DocumentStatus::Deleted);
    }

    // Events

    /// 生成创建事件
    pub fn into_created_event(self) -> JournalEntryCreated {
        JournalEntryCreated {
            company_code: self.document.company_code().clone(),
            document_number: DocumentNumber::from_str(self.document_number()).unwrap(),
            fiscal_year: self.fiscal_year().to_string(),
            total_debit: self.total_debit,
            total_credit: self.total_credit,
            created_at: self.document.created_at(),
        }
    }

    /// 生成过账事件
    pub fn into_posted_event(self) -> JournalEntryPosted {
        JournalEntryPosted {
            company_code: self.document.company_code().clone(),
            document_number: DocumentNumber::from_str(self.document_number()).unwrap(),
            fiscal_year: self.fiscal_year().to_string(),
            posted_at: Utc::now(),
        }
    }

    /// 生成冲销事件
    pub fn into_reversed_event(self, reversal_document: DocumentNumber) -> JournalEntryReversed {
        JournalEntryReversed {
            company_code: self.document.company_code().clone(),
            original_document: DocumentNumber::from_str(self.document_number()).unwrap(),
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
    #[error("借贷不平衡: 借方={0:?}, 贷方={1:?}")]
    NotBalanced { debit: Money, credit: Money },
    #[error("凭证没有行项目")]
    NoItems,
    #[error("行项目添加失败: {0}")]
    ItemError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_journal_entry() {
        let entry = JournalEntry::new(
            DocumentType::StandardDocument,
            DocumentNumber::new("0000001000").unwrap(),
            "2024".to_string(),
            CompanyCode::new("1000").unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "CNY",
            "TESTUSER",
        );

        assert_eq!(entry.document_number(), "0000001000");
        assert_eq!(entry.fiscal_year(), "2024");
        assert_eq!(entry.status(), DocumentStatus::Created);
    }

    #[test]
    fn test_add_items() {
        let mut entry = JournalEntry::new(
            DocumentType::StandardDocument,
            DocumentNumber::new("0000001000").unwrap(),
            "2024".to_string(),
            CompanyCode::new("1000").unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "CNY",
            "TESTUSER",
        );

        // 添加借方行
        entry.add_item(JournalEntryItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            DebitCreditIndicator::Debit,
            Money::new(1000, "CNY").unwrap(),
            Money::new(1000, "CNY").unwrap(),
        ).unwrap()).unwrap();

        // 添加贷方行
        entry.add_item(JournalEntryItem::new(
            2,
            AccountCode::new("2201").unwrap(),
            DebitCreditIndicator::Credit,
            Money::new(1000, "CNY").unwrap(),
            Money::new(1000, "CNY").unwrap(),
        ).unwrap()).unwrap();

        assert_eq!(entry.item_count(), 2);
        assert!(entry.is_balanced());
    }

    #[test]
    fn test_post_entry() {
        let mut entry = JournalEntry::new(
            DocumentType::StandardDocument,
            DocumentNumber::new("0000001000").unwrap(),
            "2024".to_string(),
            CompanyCode::new("1000").unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "CNY",
            "TESTUSER",
        );

        entry.add_item(JournalEntryItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            DebitCreditIndicator::Debit,
            Money::new(1000, "CNY").unwrap(),
            Money::new(1000, "CNY").unwrap(),
        ).unwrap()).unwrap();

        entry.add_item(JournalEntryItem::new(
            2,
            AccountCode::new("2201").unwrap(),
            DebitCreditIndicator::Credit,
            Money::new(1000, "CNY").unwrap(),
            Money::new(1000, "CNY").unwrap(),
        ).unwrap()).unwrap();

        entry.post().unwrap();
        assert_eq!(entry.status(), DocumentStatus::Posted);
    }

    #[test]
    fn test_unbalanced_entry() {
        let mut entry = JournalEntry::new(
            DocumentType::StandardDocument,
            DocumentNumber::new("0000001000").unwrap(),
            "2024".to_string(),
            CompanyCode::new("1000").unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "CNY",
            "TESTUSER",
        );

        entry.add_item(JournalEntryItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            DebitCreditIndicator::Debit,
            Money::new(1000, "CNY").unwrap(),
            Money::new(1000, "CNY").unwrap(),
        ).unwrap()).unwrap();

        entry.add_item(JournalEntryItem::new(
            2,
            AccountCode::new("2201").unwrap(),
            DebitCreditIndicator::Credit,
            Money::new(500, "CNY").unwrap(),
            Money::new(500, "CNY").unwrap(),
        ).unwrap()).unwrap();

        assert!(entry.post().is_err());
    }
}
