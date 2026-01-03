//! 查询处理器模块

use async_trait::async_trait;
use crate::domain::repositories::{
    GLAccountRepository,
    JournalEntryRepository,
    CustomerRepository,
    VendorRepository,
    FixedAssetRepository,
    BankAccountRepository,
};
use crate::application::dto::*;

/// 查询总账科目处理器
#[async_trait]
pub struct GetGLAccountHandler<R: GLAccountRepository> {
    repository: R,
}

impl<R: GLAccountRepository> GetGLAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, account_code: &str) -> Option<GLAccount> {
        let code = account_code.parse().ok()?;
        self.repository.find_by_id(company_code, &code).await
    }
}

/// 查询总账科目列表处理器
#[async_trait]
pub struct ListGLAccountsHandler<R: GLAccountRepository> {
    repository: R,
}

impl<R: GLAccountRepository> ListGLAccountsHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode) -> Vec<GLAccount> {
        self.repository.find_all(company_code).await
    }
}

/// 查询会计凭证处理器
#[async_trait]
pub struct GetJournalEntryHandler<R: JournalEntryRepository> {
    repository: R,
}

impl<R: JournalEntryRepository> GetJournalEntryHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        company_code: &CompanyCode,
        document_number: &str,
        fiscal_year: &str,
    ) -> Option<JournalEntry> {
        let doc_number = document_number.parse().ok()?;
        self.repository.find_by_id(company_code, &doc_number, fiscal_year).await
    }
}

/// 查询会计凭证列表处理器
#[async_trait]
pub struct ListJournalEntriesHandler<R: JournalEntryRepository> {
    repository: R,
}

impl<R: JournalEntryRepository> ListJournalEntriesHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, fiscal_year: &str) -> Vec<JournalEntry> {
        self.repository.find_all(company_code, fiscal_year).await
    }
}

/// 查询客户处理器
#[async_trait]
pub struct GetCustomerHandler<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> GetCustomerHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, customer_id: &str) -> Option<Customer> {
        self.repository.find_by_id(company_code, customer_id).await
    }
}

/// 查询客户列表处理器
#[async_trait]
pub struct ListCustomersHandler<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> ListCustomersHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode) -> Vec<Customer> {
        self.repository.find_all(company_code).await
    }
}

/// 查询供应商处理器
#[async_trait]
pub struct GetVendorHandler<R: VendorRepository> {
    repository: R,
}

impl<R: VendorRepository> GetVendorHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, vendor_id: &str) -> Option<Vendor> {
        self.repository.find_by_id(company_code, vendor_id).await
    }
}

/// 查询供应商列表处理器
#[async_trait]
pub struct ListVendorsHandler<R: VendorRepository> {
    repository: R,
}

impl<R: VendorRepository> ListVendorsHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode) -> Vec<Vendor> {
        self.repository.find_all(company_code).await
    }
}

// =============================================================================
// 固定资产查询处理器
// =============================================================================

/// 查询固定资产处理器
#[async_trait]
pub struct GetFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> GetFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Option<FixedAsset> {
        self.repository.find_by_id(company_code, asset_number, sub_number).await
    }
}

/// 查询固定资产列表处理器
#[async_trait]
pub struct ListFixedAssetsHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> ListFixedAssetsHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode) -> Vec<FixedAsset> {
        self.repository.find_all(company_code).await
    }
}

/// 按状态查询固定资产处理器
#[async_trait]
pub struct ListFixedAssetsByStatusHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> ListFixedAssetsByStatusHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, status: i32) -> Vec<FixedAsset> {
        self.repository.find_by_status(company_code, status).await
    }
}

// =============================================================================
// 银行账户查询处理器
// =============================================================================

/// 查询银行账户处理器
#[async_trait]
pub struct GetBankAccountHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> GetBankAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, bank_key: &str, bank_account: &str) -> Option<BankAccount> {
        self.repository.find_by_id(bank_key, bank_account).await
    }
}

/// 查询所有银行账户处理器
#[async_trait]
pub struct ListBankAccountsHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> ListBankAccountsHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self) -> Vec<BankAccount> {
        self.repository.find_all().await
    }
}

/// 按国家查询银行账户处理器
#[async_trait]
pub struct ListBankAccountsByCountryHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> ListBankAccountsByCountryHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, country_code: &str) -> Vec<BankAccount> {
        self.repository.find_by_country(country_code).await
    }
}

/// 按 SWIFT 代码查询银行账户处理器
#[async_trait]
pub struct GetBankAccountBySwiftHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> GetBankAccountBySwiftHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, swift_code: &str) -> Option<BankAccount> {
        self.repository.find_by_swift(swift_code).await
    }
}
