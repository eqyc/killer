//! 领域服务模块

use async_trait::async_trait;
use killer_domain_primitives::{CompanyCode, DocumentNumber, Money, FiscalPeriod};
use crate::domain::aggregates::journal_entry::JournalEntry;

/// 试算平衡服务
#[async_trait]
pub trait TrialBalanceService: Send + Sync {
    /// 生成试算平衡表
    async fn generate_trial_balance(
        &self,
        company_code: &CompanyCode,
        fiscal_year: &str,
        period: u32,
        include_zero_balance: bool,
    ) -> Result<TrialBalance, String>;
}

pub struct TrialBalanceLine {
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub debit_balance: Money,
    pub credit_balance: Money,
}

pub struct TrialBalance {
    pub company_code: CompanyCode,
    pub fiscal_year: String,
    pub period: u32,
    pub lines: Vec<TrialBalanceLine>,
    pub total_debit: Money,
    pub total_credit: Money,
    pub is_balanced: bool,
}

/// 科目余额服务
#[async_trait]
pub trait AccountBalanceService: Send + Sync {
    /// 获取科目余额
    async fn get_account_balance(
        &self,
        company_code: &CompanyCode,
        account_code: &str,
        fiscal_year: &str,
        period: u32,
    ) -> Result<AccountBalance, String>;

    /// 获取科目余额清单
    async fn get_account_balances(
        &self,
        company_code: &CompanyCode,
        fiscal_year: &str,
        period: u32,
        account_from: Option<&str>,
        account_to: Option<&str>,
    ) -> Result<Vec<AccountBalanceLine>, String>;
}

pub struct AccountBalance {
    pub account_code: String,
    pub account_name: String,
    pub beginning_balance: Money,
    pub debit_turnover: Money,
    pub credit_turnover: Money,
    pub ending_balance: Money,
}

pub struct AccountBalanceLine {
    pub account_code: String,
    pub account_name: String,
    pub beginning_balance: Money,
    pub debit_turnover: Money,
    pub credit_turnover: Money,
    pub ending_balance: Money,
}

/// 凭证过账服务
#[async_trait]
pub trait PostingService: Send + Sync {
    /// 过账会计凭证
    async fn post_journal_entry(
        &self,
        company_code: &CompanyCode,
        document_number: &DocumentNumber,
        fiscal_year: &str,
    ) -> Result<JournalEntry, String>;

    /// 冲销会计凭证
    async fn reverse_journal_entry(
        &self,
        company_code: &CompanyCode,
        document_number: &DocumentNumber,
        fiscal_year: &str,
        reversal_date: chrono::NaiveDate,
        reason: &str,
    ) -> Result<DocumentNumber, String>;
}

/// 清账服务
#[async_trait]
pub trait ClearingService: Send + Sync {
    /// 客户清账
    async fn clear_customer_open_items(
        &self,
        company_code: &CompanyCode,
        customer_id: &str,
        document_numbers: Vec<DocumentNumber>,
        clearing_date: chrono::NaiveDate,
    ) -> Result<ClearingResult, String>;

    /// 供应商清账
    async fn clear_vendor_open_items(
        &self,
        company_code: &CompanyCode,
        vendor_id: &str,
        document_numbers: Vec<DocumentNumber>,
        clearing_date: chrono::NaiveDate,
    ) -> Result<ClearingResult, String>;
}

pub struct ClearingResult {
    pub clearing_document: DocumentNumber,
    pub cleared_documents: Vec<DocumentNumber>,
    pub clearing_date: chrono::NaiveDate,
}

/// 期间控制服务
#[async_trait]
pub trait PeriodControlService: Send + Sync {
    /// 开启会计期间
    async fn open_fiscal_period(
        &self,
        company_code: &CompanyCode,
        fiscal_year: &str,
        period: u32,
    ) -> Result<bool, String>;

    /// 关闭会计期间
    async fn close_fiscal_period(
        &self,
        company_code: &CompanyCode,
        fiscal_year: &str,
        period: u32,
        closing_activities: Vec<String>,
    ) -> Result<PeriodCloseResult, String>;
}

pub struct PeriodCloseResult {
    pub success: bool,
    pub message: String,
    pub activities: Vec<ActivityResult>,
}

pub struct ActivityResult {
    pub activity: String,
    pub completed: bool,
    pub message: String,
}
