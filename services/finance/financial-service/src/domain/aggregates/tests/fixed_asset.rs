//! FixedAsset 单元测试

use crate::domain::aggregates::FixedAsset;
use killer_domain_primitives::{CompanyCode, Money};
use rust_decimal_macros::dec;
use chrono::{NaiveDate, Utc};

#[test]
fn test_new_fixed_asset() {
    let company_code = CompanyCode::new("1000").unwrap();

    let asset = FixedAsset::new(
        company_code.clone(),
        "LAND".to_string(),  // 土地
        "LAND".to_string(),  // 土地估值类
        "公司办公楼用地".to_string(),
    );

    assert_eq!(asset.company_code(), &company_code);
    assert_eq!(asset.asset_class(), "LAND");
    assert_eq!(asset.valuation_class(), "LAND");
    assert_eq!(asset.description(), "公司办公楼用地");
    assert_eq!(asset.status(), crate::domain::aggregates::AssetStatus::New);
}

#[test]
fn test_fixed_asset_set_sub_number() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code.clone(),
        "BUILD".to_string(),
        "BUILD".to_string(),
        "办公楼",
    );

    asset.set_sub_number("0001".to_string());
    assert_eq!(asset.sub_number(), "0001");
}

#[test]
fn test_fixed_asset_set_cost_center() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    asset.set_cost_center("C10000001".to_string());
    assert_eq!(asset.cost_center(), Some(&"C10000001".to_string()));
}

#[test]
fn test_fixed_asset_set_profit_center() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    asset.set_profit_center("P10000001".to_string());
    assert_eq!(asset.profit_center(), Some(&"P10000001".to_string()));
}

#[test]
fn test_fixed_asset_set_location() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    asset.set_location("BJ001".to_string());
    assert_eq!(asset.location(), Some(&"BJ001".to_string()));
}

#[test]
fn test_fixed_asset_capitalize() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    let acquisition_value = Money::new(dec!(100000.00), &"CNY").unwrap();
    let capitalization_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

    asset.capitalize(capitalization_date, acquisition_value);

    assert_eq!(asset.status(), crate::domain::aggregates::AssetStatus::Capitalized);
    assert_eq!(*asset.acquisition_value(), dec!(100000.00));
    assert_eq!(asset.capitalization_date(), Some(capitalization_date));
}

#[test]
fn test_fixed_asset_depreciate() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    // 先资本化
    let acquisition_value = Money::new(dec!(100000.00), &"CNY").unwrap();
    let capitalization_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    asset.capitalize(capitalization_date, acquisition_value);

    // 折旧
    let depreciation_amount = Money::new(dec!(2000.00), &"CNY").unwrap();
    asset.depreciate(depreciation_amount);

    assert_eq!(*asset.accumulated_depreciation(), dec!(2000.00));
    assert_eq!(asset.current_depreciation(), Money::new(dec!(2000.00), &"CNY").unwrap());
    assert_eq!(asset.net_book_value(), Money::new(dec!(98000.00), &"CNY").unwrap());
}

#[test]
fn test_fixed_asset_transfer() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code.clone(),
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    asset.set_cost_center("C10000001".to_string());
    asset.set_profit_center("P10000001".to_string());

    asset.transfer(
        Some("C10000002".to_string()),
        Some("P10000002".to_string()),
        Some("BJ01".to_string()),
    );

    assert_eq!(asset.cost_center(), Some(&"C10000002".to_string()));
    assert_eq!(asset.profit_center(), Some(&"P10000002".to_string()));
    assert_eq!(asset.business_area(), Some(&"BJ01".to_string()));
}

#[test]
fn test_fixed_asset_retire() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    // 先资本化
    let acquisition_value = Money::new(dec!(100000.00), &"CNY").unwrap();
    let capitalization_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    asset.capitalize(capitalization_date, acquisition_value);

    // 报废
    let retirement_value = Money::new(dec!(5000.00), &"CNY").unwrap();
    asset.retire(retirement_value).unwrap();

    assert_eq!(asset.status(), crate::domain::aggregates::AssetStatus::Retired);
    assert_eq!(asset.retirement_value().unwrap().amount(), dec!(5000.00));
    assert!(asset.decommissioning_date().is_some());
}

#[test]
fn test_fixed_asset_block() {
    let company_code = CompanyCode::new("1000").unwrap();

    let mut asset = FixedAsset::new(
        company_code,
        "MACH".to_string(),
        "MACH".to_string(),
        "生产设备",
    );

    assert_eq!(asset.status(), crate::domain::aggregates::AssetStatus::New);

    asset.block();
    assert_eq!(asset.status(), crate::domain::aggregates::AssetStatus::Blocked);
}

#[test]
fn test_fixed_asset_status_conversion() {
    use crate::domain::aggregates::AssetStatus;

    assert_eq!(AssetStatus::New, AssetStatus::try_from(1).unwrap());
    assert_eq!(AssetStatus::Capitalized, AssetStatus::try_from(2).unwrap());
    assert_eq!(AssetStatus::Retired, AssetStatus::try_from(3).unwrap());
    assert_eq!(AssetStatus::Blocked, AssetStatus::try_from(4).unwrap());

    assert!(AssetStatus::try_from(99).is_err());
}
