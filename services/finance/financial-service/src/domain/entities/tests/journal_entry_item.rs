//! JournalEntryItem 单元测试

use crate::domain::entities::{JournalEntryItem, DebitCreditIndicator, JournalEntryItemError};
use killer_domain_primitives::{AccountCode, Money, CurrencyCode};
use rust_decimal::Decimal;

fn amount_1000() -> Money {
    Money::new(Decimal::new(1000, 2), CurrencyCode::CNY).unwrap()
}

#[test]
fn test_new_journal_entry_item() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    );

    assert!(item.is_ok());
    let item = item.unwrap();
    assert_eq!(item.line_number(), 1);
    assert_eq!(item.debit_credit(), DebitCreditIndicator::Debit);
}

#[test]
fn test_journal_entry_item_with_customer() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_customer("C0000100001".to_string());

    assert_eq!(item.customer_id(), Some("C0000100001"));
}

#[test]
fn test_journal_entry_item_with_vendor() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Credit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_vendor("V0000100001".to_string());

    assert_eq!(item.vendor_id(), Some("V0000100001"));
}

#[test]
fn test_journal_entry_item_with_cost_center() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_cost_center("C10000001".to_string());

    assert_eq!(item.cost_center(), Some("C10000001"));
}

#[test]
fn test_journal_entry_item_with_profit_center() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_profit_center("P10000001".to_string());

    assert_eq!(item.profit_center(), Some("P10000001"));
}

#[test]
fn test_journal_entry_item_with_line_text() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_line_text("销售商品款".to_string());

    assert_eq!(item.line_text(), Some("销售商品款"));
}

#[test]
fn test_journal_entry_item_with_assignment() {
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();
    let item_amount = amount_1000();

    let item = JournalEntryItem::new(
        1,
        account_code,
        DebitCreditIndicator::Debit,
        item_amount.clone(),
        item_amount,
    ).unwrap()
    .with_assignment("INV2024001".to_string());

    assert_eq!(item.assignment_number(), Some("INV2024001"));
}

#[test]
fn test_debit_credit_indicator_conversion() {
    assert_eq!(DebitCreditIndicator::try_from(1).unwrap(), DebitCreditIndicator::Debit);
    assert_eq!(DebitCreditIndicator::try_from(2).unwrap(), DebitCreditIndicator::Credit);
    assert!(DebitCreditIndicator::try_from(3).is_err());
}

#[test]
fn test_debit_credit_indicator_display() {
    assert_eq!(format!("{}", DebitCreditIndicator::Debit), "S");
    assert_eq!(format!("{}", DebitCreditIndicator::Credit), "H");
}
