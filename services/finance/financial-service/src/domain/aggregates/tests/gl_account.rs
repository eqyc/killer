//! GLAccount 单元测试

use crate::domain::aggregates::GLAccount;
use killer_domain_primitives::{AccountCode, CompanyCode, Money};
use rust_decimal::Decimal;

#[test]
fn test_new_gl_account() {
    let company_code = CompanyCode::new("1000").unwrap();
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();

    let account = GLAccount::new(
        "KA01",
        account_code.clone(),
        company_code.clone(),
        "A",  // 资产类
        "X",  // 资产负债表科目
        "CNY",
        "库存现金",
    );

    assert_eq!(account.chart_of_accounts(), "KA01");
    assert_eq!(account.account_code(), &account_code);
    assert_eq!(account.company_code(), &company_code);
    assert_eq!(account.account_type(), "A");
    assert_eq!(account.balance_sheet_indicator(), "X");
    assert_eq!(account.currency(), "CNY");
    assert_eq!(account.description(), "库存现金");
    assert!(!account.is_deleted());
}

#[test]
fn test_gl_account_setters() {
    let company_code = CompanyCode::new("1000").unwrap();
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();

    let mut account = GLAccount::new(
        "KA01",
        account_code,
        company_code,
        "A",
        "X",
        "CNY",
        "库存现金",
    );

    // 设置成本控制范围
    account.set_cost_control_area("C001", "TEST_USER");
    assert_eq!(account.cost_control_area(), "C001");

    // 设置科目组
    account.set_account_group("Cash", "TEST_USER");
    assert_eq!(account.account_group(), "Cash");
}

#[test]
fn test_gl_account_soft_delete() {
    let company_code = CompanyCode::new("1000").unwrap();
    let account_code = AccountCode::new("1001000001", "KA01").unwrap();

    let mut account = GLAccount::new(
        "KA01",
        account_code,
        company_code,
        "A",
        "X",
        "CNY",
        "库存现金",
    );

    assert!(!account.is_deleted());

    account.mark_deleted("TEST_USER");
    assert!(account.is_deleted());
}
