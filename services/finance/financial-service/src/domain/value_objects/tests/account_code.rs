//! AccountCode 单元测试

use crate::domain::value_objects::AccountCode;
use crate::domain::value_objects::AccountCodeError;

#[test]
fn test_account_code_valid() {
    let code = AccountCode::new("1001000001");
    assert!(code.is_ok());
    assert_eq!(code.unwrap().to_string(), "1001000001");
}

#[test]
fn test_account_code_empty() {
    let code = AccountCode::new("");
    assert!(code.is_err());
    assert!(matches!(code.unwrap_err(), AccountCodeError::Empty));
}

#[test]
fn test_account_code_too_long() {
    let code = AccountCode::new("123456789012345");
    assert!(code.is_err());
    assert!(matches!(code.unwrap_err(), AccountCodeError::TooLong(_)));
}

#[test]
fn test_account_code_contains_invalid_characters() {
    let code = AccountCode::new("1001@00001");
    assert!(code.is_err());
    assert!(matches!(
        code.unwrap_err(),
        AccountCodeError::InvalidFormat(_)
    ));
}

#[test]
fn test_account_code_alphanumeric() {
    let code = AccountCode::new("ABC1000001");
    assert!(code.is_ok());
    assert_eq!(code.unwrap().to_string(), "ABC1000001");
}

#[test]
fn test_account_code_equality() {
    let code1 = AccountCode::new("1001000001").unwrap();
    let code2 = AccountCode::new("1001000001").unwrap();
    let code3 = AccountCode::new("1001000002").unwrap();

    assert_eq!(code1, code2);
    assert_ne!(code1, code3);
}

#[test]
fn test_account_code_display() {
    let code = AccountCode::new("1001000001").unwrap();
    let display = format!("{}", code);
    assert_eq!(display, "1001000001");
}

#[test]
fn test_account_code_from_string() {
    let code: AccountCode = "1001000001".parse().unwrap();
    assert_eq!(code.to_string(), "1001000001");
}

#[test]
fn test_account_code_hash() {
    use std::collections::HashSet;

    let code1 = AccountCode::new("1001000001").unwrap();
    let code2 = AccountCode::new("1001000001").unwrap();
    let code3 = AccountCode::new("1001000002").unwrap();

    let mut set = HashSet::new();
    set.insert(code1.clone());
    set.insert(code2.clone());
    set.insert(code3.clone());

    // 只有两个唯一值
    assert_eq!(set.len(), 2);
}
