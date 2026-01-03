//! DocumentNumber 单元测试

use crate::domain::value_objects::DocumentNumber;
use crate::domain::value_objects::DocumentNumberError;

#[test]
fn test_document_number_valid() {
    let number = DocumentNumber::new("1000000001");
    assert!(number.is_ok());
    assert_eq!(number.unwrap().to_string(), "1000000001");
}

#[test]
fn test_document_number_too_short() {
    let number = DocumentNumber::new("123456789");
    assert!(number.is_err());
    assert_eq!(
        number.unwrap_err(),
        DocumentNumberError::InvalidLength { expected: 10, actual: 9 }
    );
}

#[test]
fn test_document_number_too_long() {
    let number = DocumentNumber::new("12345678901");
    assert!(number.is_err());
    assert_eq!(
        number.unwrap_err(),
        DocumentNumberError::InvalidLength { expected: 10, actual: 11 }
    );
}

#[test]
fn test_document_number_not_numeric() {
    let number = DocumentNumber::new("12345ABCDE");
    assert!(number.is_err());
    assert!(matches!(
        number.unwrap_err(),
        DocumentNumberError::NotNumeric(_)
    ));
}

#[test]
fn test_document_number_from_number() {
    let number = DocumentNumber::from_number(12345);
    assert_eq!(number.to_string(), "00000012345");
}

#[test]
fn test_document_number_from_number_max() {
    let number = DocumentNumber::from_number(9999999999);
    assert_eq!(number.to_string(), "9999999999");
}

#[test]
fn test_document_number_from_number_overflow() {
    let number = DocumentNumber::from_number(10000000000);
    // 应该回绕
    assert_ne!(number.to_string(), "10000000000");
}

#[test]
fn test_document_number_equality() {
    let num1 = DocumentNumber::new("1000000001").unwrap();
    let num2 = DocumentNumber::new("1000000001").unwrap();
    let num3 = DocumentNumber::new("1000000002").unwrap();

    assert_eq!(num1, num2);
    assert_ne!(num1, num3);
}
