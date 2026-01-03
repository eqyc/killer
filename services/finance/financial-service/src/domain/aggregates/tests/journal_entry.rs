//! JournalEntry 单元测试

use crate::domain::aggregates::JournalEntry;
use crate::domain::entities::{JournalEntryItem, DebitCreditIndicator};
use crate::domain::value_objects::{AccountCode, PostingDate};
use killer_domain_primitives::{CompanyCode, DocumentNumber, Money};
use rust_decimal_macros::dec;
use chrono::NaiveDate;

#[test]
fn test_new_journal_entry() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number.clone(),
        "2024".to_string(),
        company_code.clone(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    assert_eq!(entry.document_type(), &crate::domain::entities::DocumentType::StandardDocument);
    assert_eq!(entry.document_number(), &doc_number);
    assert_eq!(entry.fiscal_year(), "2024");
    assert_eq!(entry.status(), &crate::domain::entities::DocumentStatus::Created);
}

#[test]
fn test_journal_entry_add_item() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let mut entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number,
        "2024",
        company_code,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    let account_code = AccountCode::new("1001000001").unwrap();
    let amount = Money::new(dec!(1000.00), &"CNY").unwrap();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        amount.clone(),
        amount,
    ).unwrap();

    entry.add_item(item).unwrap();
    assert_eq!(entry.items().len(), 1);
}

#[test]
fn test_journal_entry_debit_credit_balance() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let mut entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number,
        "2024",
        company_code,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 添加借方
    let debit_account = AccountCode::new("1001000001").unwrap();
    let debit_amount = Money::new(dec!(1000.00), &"CNY").unwrap();
    let debit_item = JournalEntryItem::new(
        1,
        debit_account,
        DebitCreditIndicator::Debit,
        debit_amount.clone(),
        debit_amount,
    ).unwrap();
    entry.add_item(debit_item).unwrap();

    // 添加贷方
    let credit_account = AccountCode::new("2001000001").unwrap();
    let credit_amount = Money::new(dec!(1000.00), &"CNY").unwrap();
    let credit_item = JournalEntryItem::new(
        2,
        credit_account,
        DebitCreditIndicator::Credit,
        credit_amount.clone(),
        credit_amount,
    ).unwrap();
    entry.add_item(credit_item).unwrap();

    assert!(entry.is_balanced());
}

#[test]
fn test_journal_entry_unbalanced() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let mut entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number,
        "2024",
        company_code,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 只添加借方，不添加贷方
    let account_code = AccountCode::new("1001000001").unwrap();
    let amount = Money::new(dec!(1000.00), &"CNY").unwrap();
    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        amount.clone(),
        amount,
    ).unwrap();
    entry.add_item(item).unwrap();

    assert!(!entry.is_balanced());
}

#[test]
fn test_journal_entry_post() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let mut entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number,
        "2024",
        company_code,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 添加平衡的分录
    let debit_account = AccountCode::new("1001000001").unwrap();
    let credit_account = AccountCode::new("2001000001").unwrap();
    let amount = Money::new(dec!(1000.00), &"CNY").unwrap();

    let debit_item = JournalEntryItem::new(
        1,
        debit_account,
        DebitCreditIndicator::Debit,
        amount.clone(),
        amount,
    ).unwrap();
    entry.add_item(debit_item).unwrap();

    let credit_amount = Money::new(dec!(1000.00), &"CNY").unwrap();
    let credit_item = JournalEntryItem::new(
        2,
        credit_account,
        DebitCreditIndicator::Credit,
        credit_amount.clone(),
        credit_amount,
    ).unwrap();
    entry.add_item(credit_item).unwrap();

    entry.post().unwrap();
    assert_eq!(entry.status(), &crate::domain::entities::DocumentStatus::Posted);
}

#[test]
fn test_journal_entry_reverse() {
    let company_code = CompanyCode::new("1000").unwrap();
    let doc_number = DocumentNumber::new("1000000001").unwrap();

    let mut entry = JournalEntry::new(
        crate::domain::entities::DocumentType::StandardDocument,
        doc_number,
        "2024",
        company_code,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    entry.reverse(
        NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
        "01",  // 错误更正
    ).unwrap();

    assert_eq!(entry.status(), &crate::domain::entities::DocumentStatus::Reversed);
    assert!(entry.reversal_document_number().is_some());
    assert!(entry.reversal_date().is_some());
}
