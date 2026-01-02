//! 读模型仓储接口
//!
//! 定义所有读模型操作的接口
//! 查询处理器使用这些接口访问读模型

use crate::application::dto::*;
use crate::domain::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

// =============================================================================
// 凭证读模型接口
// =============================================================================

/// 凭证读模型接口
#[async_trait]
pub trait JournalEntryReadModel: Send + Sync {
    /// 获取凭证详情
    async fn find_detail(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
    ) -> Result<Option<JournalEntryDetail>, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取凭证摘要列表
    async fn find_summaries(
        &self,
        tenant_id: &Uuid,
        company_code: Option<&str>,
        fiscal_year: i32,
        status: Option<&str>,
        posting_date_from: Option<NaiveDate>,
        posting_date_to: Option<NaiveDate>,
        account_code: Option<&str>,
        cost_center: Option<&str>,
        amount_min: Option<f64>,
        amount_max: Option<f64>,
        text_search: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<JournalEntrySummary>, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 更新凭证过账状态
    async fn update_posted_status(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
        posted_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 更新凭证冲销状态
    async fn update_reversed_status(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
        reversed_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入或更新凭证
    async fn upsert_journal_entry(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        document_number: &str,
        line_items: &[crate::domain::JournalEntryLineItem],
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 更新科目余额
    async fn update_account_balances(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        period: u8,
        line_items: &[crate::domain::JournalEntryLineItem],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

// =============================================================================
// 科目余额读模型接口
// =============================================================================

/// 科目余额读模型接口
#[async_trait]
pub trait AccountBalanceReadModel: Send + Sync {
    /// 获取科目余额
    async fn find_balance(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        account_code: &str,
        period: Option<u8>,
    ) -> Result<Option<AccountBalance>, Box<dyn std::error::Error + Send + Sync>>;
}

// =============================================================================
// 试算平衡表读模型接口
// =============================================================================

/// 试算平衡表读模型接口
#[async_trait]
pub trait TrialBalanceReadModel: Send + Sync {
    /// 获取试算平衡表
    async fn find_trial_balance(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        period: Option<u8>,
        expand_hierarchy: bool,
        hide_zero_balance: bool,
    ) -> Result<TrialBalanceData, Box<dyn std::error::Error + Send + Sync>>;
}

/// 试算平衡表数据（内部使用）
#[derive(Debug)]
pub struct TrialBalanceData {
    pub company_code: String,
    pub fiscal_year: i32,
    pub period: Option<u8>,
    pub total_debit: f64,
    pub total_credit: f64,
    pub lines: Vec<TrialBalanceLine>,
}

// =============================================================================
// 会计期间读模型接口
// =============================================================================

/// 会计期间读模型接口
#[async_trait]
pub trait FiscalPeriodReadModel: Send + Sync {
    /// 更新期间状态
    async fn update_period_status(
        &self,
        tenant_id: &Uuid,
        company_code: &str,
        fiscal_year: i32,
        period: u8,
        status: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
