//! Vendor 单元测试

use crate::domain::aggregates::{Vendor, VendorStatus};
use killer_domain_primitives::CompanyCode;

#[test]
fn test_new_vendor() {
    let company_code = CompanyCode::new("1000").unwrap();

    let vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code.clone(),
        "VEND".to_string(),  // 供应商
        "测试供应商".to_string(),
        "CN".to_string(),
        "CNY".to_string(),
    );

    assert_eq!(vendor.vendor_id(), "V0000100001");
    assert_eq!(vendor.company_code(), &company_code);
    assert_eq!(vendor.account_group(), "VEND");
    assert_eq!(vendor.name_1(), "测试供应商");
    assert_eq!(vendor.country(), "CN");
    assert_eq!(vendor.currency(), "CNY");
    assert_eq!(vendor.status(), VendorStatus::Active);
}

#[test]
fn test_vendor_set_tax_number() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    vendor.set_tax_number("91310000781789823B".to_string(), "TEST_USER");
    assert_eq!(vendor.tax_number(), Some("91310000781789823B"));
}

#[test]
fn test_vendor_update_basic_info() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "旧供应商名",
        "CN",
        "CNY",
    );

    vendor.update_basic_info(
        "新供应商名".to_string(),
        Some("上海市".to_string()),
        Some("浦东新区".to_string()),
        Some("200000".to_string()),
        Some("CN".to_string()),
        "TEST_USER",
    );

    assert_eq!(vendor.name_1(), "新供应商名");
    assert_eq!(vendor.street(), Some("上海市"));
    assert_eq!(vendor.city(), Some("浦东新区"));
    assert_eq!(vendor.postal_code(), Some("200000"));
}

#[test]
fn test_vendor_update_financial_info() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    vendor.update_financial_info(
        None,  // 信用限额
        "2202000000".to_string(),  // 应付账款科目
        "0001".to_string(),  // 付款条件
        Some("T".to_string()),  // 付款方式
        "TEST_USER",
    );

    assert_eq!(vendor.reconciliation_account(), "2202000000");
    assert_eq!(vendor.payment_terms(), "0001");
    assert_eq!(vendor.payment_methods(), Some("T"));
}

#[test]
fn test_vendor_set_contact() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    vendor.set_contact(
        Some("021-87654321".to_string()),
        Some("vendor@example.com".to_string()),
        "TEST_USER",
    );

    assert_eq!(vendor.phone_number(), Some("021-87654321"));
    assert_eq!(vendor.email_address(), Some("vendor@example.com"));
}

#[test]
fn test_vendor_block() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    assert_eq!(vendor.status(), VendorStatus::Active);

    vendor.block("TEST_USER");
    assert_eq!(vendor.status(), VendorStatus::Blocked);
}

#[test]
fn test_vendor_unblock() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    vendor.block("TEST_USER");
    assert_eq!(vendor.status(), VendorStatus::Blocked);

    vendor.unblock("TEST_USER");
    assert_eq!(vendor.status(), VendorStatus::Active);
}

#[test]
fn test_vendor_delete() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut vendor = Vendor::new(
        "V0000100001".to_string(),
        company_code,
        "VEND",
        "测试供应商",
        "CN",
        "CNY",
    );

    vendor.delete("TEST_USER");
    assert_eq!(vendor.status(), VendorStatus::Deleted);
}

#[test]
fn test_vendor_status_conversion() {
    use crate::domain::aggregates::VendorStatus;

    assert_eq!(VendorStatus::Active, VendorStatus::try_from(1).unwrap());
    assert_eq!(VendorStatus::Blocked, VendorStatus::try_from(2).unwrap());
    assert_eq!(VendorStatus::Deleted, VendorStatus::try_from(3).unwrap());

    assert!(VendorStatus::try_from(99).is_err());
}
