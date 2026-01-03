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
