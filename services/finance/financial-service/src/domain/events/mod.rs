//! 领域事件模块

use killer_domain_primitives::{AccountCode, CompanyCode, DocumentNumber, Money};
use chrono::Utc;

/// 总账科目相关事件
#[derive(Debug, Clone)]
pub struct GLAccountCreated {
    pub company_code: CompanyCode,
    pub account_code: AccountCode,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GLAccountUpdated {
    pub company_code: CompanyCode,
    pub account_code: AccountCode,
    pub updated_at: chrono::DateTime<Utc>,
}

/// 会计凭证相关事件
#[derive(Debug, Clone)]
pub struct JournalEntryCreated {
    pub company_code: CompanyCode,
    pub document_number: DocumentNumber,
    pub fiscal_year: String,
    pub total_debit: Money,
    pub total_credit: Money,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct JournalEntryPosted {
    pub company_code: CompanyCode,
    pub document_number: DocumentNumber,
    pub fiscal_year: String,
    pub posted_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct JournalEntryReversed {
    pub company_code: CompanyCode,
    pub original_document: DocumentNumber,
    pub reversal_document: DocumentNumber,
    pub fiscal_year: String,
    pub reversed_at: chrono::DateTime<Utc>,
}

/// 客户相关事件
#[derive(Debug, Clone)]
pub struct CustomerCreated {
    pub company_code: CompanyCode,
    pub customer_id: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CustomerPaymentReceived {
    pub company_code: CompanyCode,
    pub customer_id: String,
    pub document_number: DocumentNumber,
    pub amount: Money,
    pub received_at: chrono::DateTime<Utc>,
}

/// 供应商相关事件
#[derive(Debug, Clone)]
pub struct VendorCreated {
    pub company_code: CompanyCode,
    pub vendor_id: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct VendorPaymentMade {
    pub company_code: CompanyCode,
    pub vendor_id: String,
    pub document_number: DocumentNumber,
    pub amount: Money,
    pub paid_at: chrono::DateTime<Utc>,
}

/// 固定资产相关事件
#[derive(Debug, Clone)]
pub struct FixedAssetAcquired {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub acquisition_value: Money,
    pub acquired_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct FixedAssetDepreciated {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub depreciation_amount: Money,
    pub fiscal_year: String,
    pub period: u32,
    pub depreciated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct FixedAssetRetired {
    pub company_code: CompanyCode,
    pub asset_number: String,
    pub retirement_value: Money,
    pub retired_at: chrono::DateTime<Utc>,
}
