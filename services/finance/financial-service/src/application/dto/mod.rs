//! 数据传输对象模块

use crate::domain::aggregates::{gl_account::GLAccount, journal_entry::JournalEntry, customer::Customer, vendor::Vendor, fixed_asset::FixedAsset, bank_account::BankAccount};
use killer_domain_primitives::CompanyCode;

/// 创建总账科目 DTO
#[derive(Debug)]
pub struct CreateGLAccountDto {
    pub chart_of_accounts: String,
    pub account_code: String,
    pub company_code: CompanyCode,
    pub account_type: String,
    pub balance_sheet_indicator: String,
    pub currency: String,
    pub description: String,
    pub cost_control_area: Option<String>,
    pub account_group: Option<String>,
}

/// 创建会计凭证 DTO
#[derive(Debug)]
pub struct CreateJournalEntryDto {
    pub company_code: CompanyCode,
    pub document_type: String,
    pub document_date: chrono::NaiveDate,
    pub posting_date: chrono::NaiveDate,
    pub currency: String,
    pub reference_document: Option<String>,
    pub header_text: Option<String>,
    pub items: Vec<JournalEntryItemDto>,
}

/// 会计凭证行项目 DTO
#[derive(Debug)]
pub struct JournalEntryItemDto {
    pub gl_account: String,
    pub debit_credit: i32,  // 1: Debit, 2: Credit
    pub amount: f64,
    pub currency: String,
    pub customer_id: Option<String>,
    pub vendor_id: Option<String>,
    pub cost_center: Option<String>,
    pub profit_center: Option<String>,
    pub tax_code: Option<String>,
    pub line_text: Option<String>,
    pub assignment: Option<String>,
}

/// 创建客户 DTO
#[derive(Debug)]
pub struct CreateCustomerDto {
    pub company_code: CompanyCode,
    pub account_group: String,
    pub name_1: String,
    pub name_2: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub tax_number: Option<String>,
    pub currency: String,
    pub reconciliation_account: String,
    pub payment_terms: String,
    pub payment_methods: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
}

/// 更新客户 DTO
#[derive(Debug)]
pub struct UpdateCustomerDto {
    pub company_code: CompanyCode,
    pub customer_id: String,
    pub name_1: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub payment_terms: Option<String>,
}

/// 创建供应商 DTO
#[derive(Debug)]
pub struct CreateVendorDto {
    pub company_code: CompanyCode,
    pub account_group: String,
    pub name_1: String,
    pub name_2: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub tax_number: Option<String>,
    pub currency: String,
    pub reconciliation_account: String,
    pub payment_terms: String,
    pub payment_methods: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
}

/// 更新供应商 DTO
#[derive(Debug)]
pub struct UpdateVendorDto {
    pub company_code: CompanyCode,
    pub vendor_id: String,
    pub name_1: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub payment_terms: Option<String>,
}

/// 过账凭证 DTO
#[derive(Debug)]
pub struct PostJournalEntryDto {
    pub company_code: CompanyCode,
    pub document_number: String,
    pub fiscal_year: String,
}

/// 冲销凭证 DTO
#[derive(Debug)]
pub struct ReverseJournalEntryDto {
    pub company_code: CompanyCode,
    pub document_number: String,
    pub fiscal_year: String,
    pub reversal_date: chrono::NaiveDate,
    pub reversal_reason: String,
}

/// 查询总账科目 DTO
#[derive(Debug)]
pub struct QueryGLAccountsDto {
    pub company_code: CompanyCode,
    pub chart_of_accounts: String,
    pub account_type: Option<String>,
    pub balance_sheet_indicator: Option<String>,
    pub page_size: u32,
    pub page_token: Option<String>,
}

/// 查询会计凭证 DTO
#[derive(Debug)]
pub struct QueryJournalEntriesDto {
    pub company_code: CompanyCode,
    pub fiscal_year: String,
    pub period_from: Option<u32>,
    pub period_to: Option<u32>,
    pub document_type: Option<String>,
    pub gl_account: Option<String>,
    pub customer_id: Option<String>,
    pub vendor_id: Option<String>,
    pub page_size: u32,
    pub page_token: Option<String>,
}

/// 客户清账 DTO
#[derive(Debug)]
pub struct ClearCustomerDto {
    pub company_code: CompanyCode,
    pub customer_id: String,
    pub document_numbers: Vec<String>,
    pub clearing_date: chrono::NaiveDate,
}

/// 供应商清账 DTO
#[derive(Debug)]
pub struct ClearVendorDto {
    pub company_code: CompanyCode,
    pub vendor_id: String,
    pub document_numbers: Vec<String>,
    pub clearing_date: chrono::NaiveDate,
}

// =============================================================================
// 固定资产 DTOs
// =============================================================================

/// 创建固定资产 DTO
#[derive(Debug)]
pub struct CreateFixedAssetDto {
    pub company_code: CompanyCode,
    pub asset_class: String,
    pub valuation_class: String,
    pub description: String,
    pub cost_center: Option<String>,
    pub profit_center: Option<String>,
    pub location: Option<String>,
    pub acquisition_value: f64,
    pub currency: String,
    pub capitalization_date: chrono::NaiveDate,
}

/// 固定资产资本化 DTO
#[derive(Debug)]
pub struct CapitalizeFixedAssetDto {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub sub_number: String,
    pub acquisition_value: f64,
    pub currency: String,
    pub capitalization_date: chrono::NaiveDate,
}

/// 固定资产折旧 DTO
#[derive(Debug)]
pub struct DepreciateFixedAssetDto {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub sub_number: String,
    pub depreciation_amount: f64,
    pub currency: String,
}

/// 固定资产转移 DTO
#[derive(Debug)]
pub struct TransferFixedAssetDto {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub sub_number: String,
    pub new_cost_center: Option<String>,
    pub new_profit_center: Option<String>,
    pub new_business_area: Option<String>,
}

/// 固定资产报废 DTO
#[derive(Debug)]
pub struct RetireFixedAssetDto {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub sub_number: String,
    pub retirement_value: f64,
}

// =============================================================================
// 银行账户 DTOs
// =============================================================================

/// 创建银行账户 DTO
#[derive(Debug)]
pub struct CreateBankAccountDto {
    pub bank_country_code: String,
    pub bank_key: String,
    pub bank_name: String,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub swift_code: Option<String>,
    pub iban: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_type: Option<String>,
}

/// 银行账户存款 DTO
#[derive(Debug)]
pub struct DepositBankAccountDto {
    pub bank_key: String,
    pub bank_account: String,
    pub amount: f64,
    pub currency: String,
}

/// 银行账户取款 DTO
#[derive(Debug)]
pub struct WithdrawBankAccountDto {
    pub bank_key: String,
    pub bank_account: String,
    pub amount: f64,
    pub currency: String,
}

/// 银行账户更新余额 DTO
#[derive(Debug)]
pub struct UpdateBankAccountBalanceDto {
    pub bank_key: String,
    pub bank_account: String,
    pub new_balance: f64,
    pub currency: String,
}
