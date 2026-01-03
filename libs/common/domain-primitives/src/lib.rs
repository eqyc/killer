//! # KILLER ERP 领域原语库
//!
//! 本库提供 ERP 系统中的基础值对象定义，参考 SAP S/4 HANA 的数据模型设计。
//!
//! ## 核心类型
//!
//! ### 金额与数量
//! - [`Money`] - 金额类型，支持多币种和精确计算
//! - [`Quantity`] - 数量类型，支持单位换算
//! - [`Percentage`] - 百分比类型，用于税率、折扣等
//!
//! ### 计量单位
//! - [`UnitOfMeasure`] - 计量单位，支持维度和换算
//! - [`Dimension`] - 计量单位维度（质量、长度、体积等）
//!
//! ### 币种
//! - [`CurrencyCode`] - ISO 4217 币种代码
//!
//! ### 组织单元
//! - [`CompanyCode`] - 公司代码（财务组织核心）
//! - [`Plant`] - 工厂代码（物流组织核心）
//! - [`CostCenter`] - 成本中心（管理会计核心）
//!
//! ### 业务编码
//! - [`AccountCode`] - 会计科目代码
//! - [`MaterialNumber`] - 物料编号
//! - [`DocumentNumber`] - 凭证编号
//! - [`FiscalPeriod`] - 会计期间
//!
//! ## SAP 参考
//!
//! | 类型 | SAP 表 | SAP 字段 |
//! |------|--------|----------|
//! | Money | ACDOCA | HSL, TSL |
//! | Quantity | EKPO | MENGE, MEINS |
//! | CurrencyCode | TCURC | WAERS |
//! | UnitOfMeasure | T006 | MSEHI |
//! | CompanyCode | T001 | BUKRS |
//! | Plant | T001W | WERKS |
//! | CostCenter | CSKS | KOSTL |
//! | AccountCode | SKA1 | SAKNR |
//! | MaterialNumber | MARA | MATNR |
//! | DocumentNumber | BKPF | BELNR |
//! | FiscalPeriod | T009B | GJAHR, MONAT |
//!
//! ## 示例
//!
//! ```rust
//! use killer_domain_primitives::{Money, CurrencyCode, Quantity, UnitOfMeasure, Percentage};
//! use rust_decimal_macros::dec;
//!
//! // 创建金额
//! let price = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();
//!
//! // 计算税额
//! let tax_rate = Percentage::vat_standard_cn(); // 13%
//! let tax = price.multiply(tax_rate.to_decimal()).unwrap();
//!
//! // 创建数量
//! let qty = Quantity::new(dec!(10), UnitOfMeasure::kilogram()).unwrap();
//!
//! // 单位换算
//! let qty_g = qty.convert_to(&UnitOfMeasure::gram()).unwrap();
//! assert_eq!(qty_g.value(), dec!(10000));
//! ```

// 模块声明
mod account_code;
mod audit_info;
mod company_code;
mod cost_center;
mod currency_code;
mod document_number;
mod error;
mod fiscal_period;
mod material_number;
mod money;
mod percentage;
mod plant;
mod quantity;
mod unit_of_measure;

// 公开导出
pub use account_code::{AccountCode, AccountType, ACCOUNT_CODE_MAX_LENGTH, CHART_OF_ACCOUNTS_LENGTH};
pub use audit_info::AuditInfo;
pub use company_code::{CompanyCode, COMPANY_CODE_LENGTH};
pub use cost_center::{CostCenter, CONTROLLING_AREA_LENGTH, COST_CENTER_MAX_LENGTH};
pub use currency_code::CurrencyCode;
pub use document_number::{
    DocumentNumber, DocumentType, COMPANY_CODE_LENGTH as DOC_COMPANY_CODE_LENGTH,
    DOCUMENT_NUMBER_LENGTH,
};
pub use error::{DomainError, DomainResult};
pub use fiscal_period::{FiscalPeriod, MAX_NORMAL_PERIOD, MAX_SPECIAL_PERIOD, MIN_PERIOD};
pub use material_number::{
    MaterialNumber, MATERIAL_NUMBER_LENGTH_CLASSIC, MATERIAL_NUMBER_LENGTH_EXTENDED,
};
pub use money::{Money, RoundingMode, MONEY_SCALE};
pub use percentage::{Percentage, PERCENTAGE_SCALE};
pub use plant::{Plant, PLANT_CODE_LENGTH};
pub use quantity::{Quantity, QUANTITY_SCALE};
pub use unit_of_measure::{Dimension, UnitOfMeasure};
