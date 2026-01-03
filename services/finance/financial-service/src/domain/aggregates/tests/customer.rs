//! Customer 单元测试

use crate::domain::aggregates::{Customer, CustomerStatus};
use killer_domain_primitives::CompanyCode;

#[test]
fn test_new_customer() {
    let company_code = CompanyCode::new("1000").unwrap();

    let customer = Customer::new(
        "C0000100001".to_string(),
        company_code.clone(),
        "REBU".to_string(),  // 应收账款客户
        "测试公司".to_string(),
        "CN".to_string(),
        "CNY".to_string(),
    );

    assert_eq!(customer.customer_id(), "C0000100001");
    assert_eq!(customer.company_code(), &company_code);
    assert_eq!(customer.account_group(), "REBU");
    assert_eq!(customer.name_1(), "测试公司");
    assert_eq!(customer.country(), "CN");
    assert_eq!(customer.currency(), "CNY");
    assert_eq!(customer.status(), CustomerStatus::Active);
}

#[test]
fn test_customer_set_tax_number() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    customer.set_tax_number("91310000781789823A".to_string(), "TEST_USER");
    assert_eq!(customer.tax_number(), Some("91310000781789823A"));
}

#[test]
fn test_customer_update_basic_info() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "旧公司名",
        "CN",
        "CNY",
    );

    customer.update_basic_info(
        "新公司名".to_string(),
        Some("北京市".to_string()),
        Some("朝阳区".to_string()),
        Some("100000".to_string()),
        Some("CN".to_string()),
        "TEST_USER",
    );

    assert_eq!(customer.name_1(), "新公司名");
    assert_eq!(customer.street(), Some("北京市"));
    assert_eq!(customer.city(), Some("朝阳区"));
    assert_eq!(customer.postal_code(), Some("100000"));
}

#[test]
fn test_customer_update_financial_info() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    customer.update_financial_info(
        None,  // 信用限额
        "1402000000".to_string(),  // 应收账款科目
        "0001".to_string(),  // 付款条件
        Some("T".to_string()),  // 付款方式
        "TEST_USER",
    );

    assert_eq!(customer.reconciliation_account(), "1402000000");
    assert_eq!(customer.payment_terms(), "0001");
    assert_eq!(customer.payment_methods(), Some("T"));
}

#[test]
fn test_customer_set_contact() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    customer.set_contact(
        Some("010-12345678".to_string()),
        Some("test@example.com".to_string()),
        "TEST_USER",
    );

    assert_eq!(customer.phone_number(), Some("010-12345678"));
    assert_eq!(customer.email_address(), Some("test@example.com"));
}

#[test]
fn test_customer_block() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    assert_eq!(customer.status(), CustomerStatus::Active);

    customer.block("TEST_USER");
    assert_eq!(customer.status(), CustomerStatus::Blocked);
}

#[test]
fn test_customer_unblock() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    customer.block("TEST_USER");
    assert_eq!(customer.status(), CustomerStatus::Blocked);

    customer.unblock("TEST_USER");
    assert_eq!(customer.status(), CustomerStatus::Active);
}

#[test]
fn test_customer_delete() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut customer = Customer::new(
        "C0000100001".to_string(),
        company_code,
        "REBU",
        "测试公司",
        "CN",
        "CNY",
    );

    customer.delete("TEST_USER");
    assert_eq!(customer.status(), CustomerStatus::Deleted);
}

#[test]
fn test_customer_status_conversion() {
    use crate::domain::aggregates::CustomerStatus;

    assert_eq!(CustomerStatus::Active, CustomerStatus::try_from(1).unwrap());
    assert_eq!(CustomerStatus::Blocked, CustomerStatus::try_from(2).unwrap());
    assert_eq!(CustomerStatus::Deleted, CustomerStatus::try_from(3).unwrap());

    assert!(CustomerStatus::try_from(99).is_err());
}
