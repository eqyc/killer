//! BankAccount 单元测试

use crate::domain::aggregates::BankAccount;
use killer_domain_primitives::{Money};
use rust_decimal_macros::dec;

#[test]
fn test_new_bank_account() {
    let account = BankAccount::new(
        "CN".to_string(),  // 中国
        "ICBC".to_string(),  // 工商银行
        "中国工商银行股份有限公司".to_string(),
    );

    assert_eq!(account.bank_country_code(), "CN");
    assert_eq!(account.bank_key(), "ICBC");
    assert_eq!(account.bank_name(), "中国工商银行股份有限公司");
    assert!(account.is_active());
}

#[test]
fn test_bank_account_update_address() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    account.update_address(
        Some("北京市西城区复兴门内大街55号".to_string()),
        Some("北京市".to_string()),
        Some("100000".to_string()),
        "TEST_USER",
    );

    assert_eq!(account.street_address(), Some(&"北京市西城区复兴门内大街55号".to_string()));
    assert_eq!(account.city(), Some(&"北京市".to_string()));
    assert_eq!(account.postal_code(), Some(&"100000".to_string()));
}

#[test]
fn test_bank_account_set_swift_code() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    account.set_swift_code("ICBKCNBJ".to_string(), "TEST_USER");
    assert_eq!(account.swift_code(), Some(&"ICBKCNBJ".to_string()));
}

#[test]
fn test_bank_account_set_iban() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    account.set_iban("CN6223456789012345678901234".to_string(), "TEST_USER");
    assert_eq!(account.iban(), Some(&"CN6223456789012345678901234".to_string()));
}

#[test]
fn test_bank_account_set_bank_account_number() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    account.set_bank_account_number("123456789012".to_string(), "TEST_USER");
    assert_eq!(account.bank_account_number(), Some(&"123456789012".to_string()));
}

#[test]
fn test_bank_account_set_bank_type() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    account.set_bank_type("01".to_string(), "TEST_USER");
    assert_eq!(account.bank_type(), Some(&"01".to_string()));
}

#[test]
fn test_bank_account_deposit() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    let amount = Money::new(dec!(10000.00), &"CNY").unwrap();
    account.deposit(amount);

    assert_eq!(account.current_balance(), dec!(10000.00));
    assert_eq!(account.available_balance(), dec!(10000.00));
}

#[test]
fn test_bank_account_withdraw() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    // 先存款
    let deposit_amount = Money::new(dec!(10000.00), &"CNY").unwrap();
    account.deposit(deposit_amount);

    // 再取款
    let withdraw_amount = Money::new(dec!(3000.00), &"CNY").unwrap();
    account.withdraw(withdraw_amount).unwrap();

    assert_eq!(account.current_balance(), dec!(7000.00));
    assert_eq!(account.available_balance(), dec!(7000.00));
}

#[test]
fn test_bank_account_withdraw_insufficient_balance() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    // 余额为0，取款应该失败
    let withdraw_amount = Money::new(dec!(1000.00), &"CNY").unwrap();
    let result = account.withdraw(withdraw_amount);

    assert!(result.is_err());
    assert_eq!(account.current_balance(), dec!(0.00));
}

#[test]
fn test_bank_account_multiple_transactions() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    // 连续存款
    account.deposit(Money::new(dec!(5000.00), &"CNY").unwrap());
    account.deposit(Money::new(dec!(3000.00), &"CNY").unwrap());
    account.deposit(Money::new(dec!(2000.00), &"CNY").unwrap());

    assert_eq!(account.current_balance(), dec!(10000.00));

    // 取款
    account.withdraw(Money::new(dec!(2500.00), &"CNY").unwrap()).unwrap();

    assert_eq!(account.current_balance(), dec!(7500.00));
}

#[test]
fn test_bank_account_zero_balance() {
    let account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    assert_eq!(account.current_balance(), dec!(0.00));
    assert_eq!(account.available_balance(), dec!(0.00));
    assert!(account.bank_account_number().is_none());
    assert!(account.swift_code().is_none());
    assert!(account.iban().is_none());
}

#[test]
fn test_bank_account_balance_precision() {
    let mut account = BankAccount::new(
        "CN".to_string(),
        "ICBC".to_string(),
        "中国工商银行股份有限公司".to_string(),
    );

    // 测试高精度金额
    account.deposit(Money::new(dec!(12345.67), &"CNY").unwrap());
    assert_eq!(account.current_balance(), dec!(12345.67));

    account.withdraw(Money::new(dec!(123.45), &"CNY").unwrap()).unwrap();
    assert_eq!(account.current_balance(), dec!(12222.22));
}
