//! 仓储接口模块
//!
//! 定义各聚合根的仓储接口

use async_trait::async_trait;
use killer_domain_primitives::{AccountCode, CompanyCode, DocumentNumber, Money};
use crate::domain::aggregates::{
    gl_account::GLAccount,
    journal_entry::JournalEntry,
    customer::Customer,
    vendor::Vendor,
    fixed_asset::FixedAsset,
    bank_account::BankAccount,
};

/// 总账科目仓储接口
#[async_trait]
pub trait GLAccountRepository: Send + Sync {
    async fn find_by_id(&self, company_code: &CompanyCode, account_code: &AccountCode) -> Option<GLAccount>;
    async fn find_all(&self, company_code: &CompanyCode) -> Vec<GLAccount>;
    async fn save(&self, account: &GLAccount) -> Result<(), String>;
    async fn delete(&self, company_code: &CompanyCode, account_code: &AccountCode) -> Result<(), String>;
}

/// 会计凭证仓储接口
#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    async fn find_by_id(&self, company_code: &CompanyCode, document_number: &DocumentNumber, fiscal_year: &str) -> Option<JournalEntry>;
    async fn find_all(&self, company_code: &CompanyCode, fiscal_year: &str) -> Vec<JournalEntry>;
    async fn save(&self, entry: &JournalEntry) -> Result<(), String>;
    async fn delete(&self, company_code: &CompanyCode, document_number: &DocumentNumber, fiscal_year: &str) -> Result<(), String>;
}

/// 客户主数据仓储接口
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_id(&self, company_code: &CompanyCode, customer_id: &str) -> Option<Customer>;
    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Customer>;
    async fn save(&self, customer: &Customer) -> Result<(), String>;
    async fn delete(&self, company_code: &CompanyCode, customer_id: &str) -> Result<(), String>;
}

/// 供应商主数据仓储接口
#[async_trait]
pub trait VendorRepository: Send + Sync {
    async fn find_by_id(&self, company_code: &CompanyCode, vendor_id: &str) -> Option<Vendor>;
    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Vendor>;
    async fn save(&self, vendor: &Vendor) -> Result<(), String>;
    async fn delete(&self, company_code: &CompanyCode, vendor_id: &str) -> Result<(), String>;
}

/// 固定资产仓储接口
#[async_trait]
pub trait FixedAssetRepository: Send + Sync {
    async fn find_by_id(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Option<FixedAsset>;
    async fn find_all(&self, company_code: &CompanyCode) -> Vec<FixedAsset>;
    async fn find_by_status(&self, company_code: &CompanyCode, status: i32) -> Vec<FixedAsset>;
    async fn save(&self, asset: &FixedAsset) -> Result<(), String>;
    async fn delete(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Result<(), String>;
}

/// 银行账户仓储接口
#[async_trait]
pub trait BankAccountRepository: Send + Sync {
    async fn find_by_id(&self, bank_key: &str, bank_account: &str) -> Option<BankAccount>;
    async fn find_all(&self) -> Vec<BankAccount>;
    async fn find_by_country(&self, country_code: &str) -> Vec<BankAccount>;
    async fn find_by_swift(&self, swift_code: &str) -> Option<BankAccount>;
    async fn save(&self, account: &BankAccount) -> Result<(), String>;
    async fn delete(&self, bank_key: &str, bank_account: &str) -> Result<(), String>;
    async fn update_balance(&self, bank_key: &str, bank_account: &str, new_balance: Money) -> Result<(), String>;
}
