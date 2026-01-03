//! JournalEntry 单元测试

use crate::domain::aggregates::JournalEntry;
use crate::domain::entities::{JournalEntryItem, DebitCreditIndicator, DocumentStatus};
use killer_domain_primitives::{AccountCode, CompanyCode, DocumentNumber, DocumentType, Money, CurrencyCode};
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[test]
fn test_new_journal_entry() {
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number.clone(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    assert_eq!(entry.document_type(), DocumentType::GeneralLedger);
    assert_eq!(entry.document_number().number(), "1000000001");
    assert_eq!(entry.fiscal_year(), 2024);
    assert_eq!(entry.status(), DocumentStatus::Created);
}

#[test]
fn test_journal_entry_add_item() {
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let mut entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();

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
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let mut entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 添加借方
    let debit_account = AccountCode::new("1001000001", "KA01").unwrap();
    let debit_amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();
    let debit_item = JournalEntryItem::new(
        1,
        debit_account,
        DebitCreditIndicator::Debit,
        debit_amount.clone(),
        debit_amount,
    ).unwrap();
    entry.add_item(debit_item).unwrap();

    // 添加贷方
    let credit_account = AccountCode::new("2001000001", "KA01").unwrap();
    let credit_amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();
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
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let mut entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 只添加借方，不添加贷方
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();
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
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let mut entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 添加平衡的分录
    let debit_account = AccountCode::new("1001000001", "KA01").unwrap();
    let credit_account = AccountCode::new("2001000001", "KA01").unwrap();
    let amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();

    let debit_item = JournalEntryItem::new(
        1,
        debit_account,
        DebitCreditIndicator::Debit,
        amount.clone(),
        amount,
    ).unwrap();
    entry.add_item(debit_item).unwrap();

    let credit_amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();
    let credit_item = JournalEntryItem::new(
        2,
        credit_account,
        DebitCreditIndicator::Credit,
        credit_amount.clone(),
        credit_amount,
    ).unwrap();
    entry.add_item(credit_item).unwrap();

    entry.post().unwrap();
    assert_eq!(entry.status(), DocumentStatus::Posted);
}

#[test]
fn test_journal_entry_reverse() {
    let doc_number = DocumentNumber::new("1000000001", 2024, "1000").unwrap();

    let mut entry = JournalEntry::new(
        DocumentType::GeneralLedger,
        doc_number,
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        "CNY",
        "TEST_USER",
    );

    // 添加平衡的分录
    let debit_account = AccountCode::new("1001000001", "KA01").unwrap();
    let credit_account = AccountCode::new("2001000001", "KA01").unwrap();
    let amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();

    let debit_item = JournalEntryItem::new(
        1,
        debit_account,
        DebitCreditIndicator::Debit,
        amount.clone(),
        amount,
    ).unwrap();
    entry.add_item(debit_item).unwrap();

    let credit_amount = Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap();
    let credit_item = JournalEntryItem::new(
        2,
        credit_account,
        DebitCreditIndicator::Credit,
        credit_amount.clone(),
        credit_amount,
    ).unwrap();
    entry.add_item(credit_item).unwrap();

    entry.post().unwrap();

    let reversal = entry.reverse(
        NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
        "01",  // 错误更正
    ).unwrap();

    assert_eq!(entry.status(), DocumentStatus::Reversed);
    assert!(!reversal.number().is_empty());
}
