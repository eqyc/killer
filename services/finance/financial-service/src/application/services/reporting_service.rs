//! 报表服务
//!
//! 提供财务报表相关的高级查询服务

use serde::{Deserialize, Serialize};
use crate::application::dto::*;
use crate::application::error::ApplicationError;
use crate::application::queries::*;
use crate::application::repositories::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, info};

// =============================================================================
// 应用服务
// =============================================================================

/// 报表服务
#[derive(Clone)]
pub struct ReportingService<QRY>
where
    QRY: QueryBus,
{
    /// 查询总线
    query_bus: Arc<QRY>,
}

impl<QRY> ReportingService<QRY>
where
    QRY: QueryBus + Send + Sync,
{
    pub fn new(query_bus: Arc<QRY>) -> Self {
        Self { query_bus }
    }

    /// 获取科目余额
    pub async fn get_account_balance(
        &self,
        tenant_id: Uuid,
        request: GetAccountBalanceRequest,
    ) -> ServiceResult<AccountBalance> {
        let query = GetAccountBalanceQuery::new(tenant_id, request);
        self.query_bus
            .execute(query)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to get account balance: {:?}", e)))
    }

    /// 获取试算平衡表
    pub async fn get_trial_balance(
        &self,
        tenant_id: Uuid,
        request: GetTrialBalanceRequest,
    ) -> ServiceResult<TrialBalanceSummary> {
        let query = GetTrialBalanceQuery::new(tenant_id, request);
        self.query_bus
            .execute(query)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to get trial balance: {:?}", e)))
    }

    /// 获取财务概览
    pub async fn get_financial_overview(
        &self,
        tenant_id: Uuid,
        company_code: &str,
        fiscal_year: i32,
    ) -> ServiceResult<FinancialOverview> {
        // 1. 获取年度试算平衡表
        let tb_request = GetTrialBalanceRequest {
            company_code: company_code.to_string(),
            fiscal_year,
            period: None,
            expand_hierarchy: Some(false),
            hide_zero_balance: Some(true),
        };

        let trial_balance = self.get_trial_balance(tenant_id, tb_request).await?;

        // 2. 计算概览指标
        let overview = FinancialOverview {
            company_code: company_code.to_string(),
            fiscal_year,
            total_assets: trial_balance.total_debit, // 简化计算
            total_liabilities: trial_balance.total_credit, // 简化计算
            net_income: trial_balance.difference,
            is_balanced: trial_balance.is_balanced,
            total_entries: trial_balance.lines.len() as u64,
            generated_at: chrono::Utc::now(),
        };

        Ok(overview)
    }

    /// 导出凭证到文件
    pub async fn export_entries(
        &self,
        tenant_id: Uuid,
        request: ListJournalEntriesRequest,
        format: ExportFormat,
    ) -> ServiceResult<Vec<u8>> {
        // 1. 获取所有匹配的凭证（不分页）
        let mut all_items = Vec::new();
        let mut page = 1;
        let page_size = 100;

        loop {
            let paged_request = ListJournalEntriesRequest {
                page: Some(page),
                page_size: Some(page_size),
                ..request.clone()
            };

            let result = self.search_entries(tenant_id, paged_request).await?;
            all_items.extend(result.items);

            if all_items.len() >= result.total_count as usize {
                break;
            }
            page += 1;
        }

        // 2. 根据格式导出
        match format {
            ExportFormat::Csv => self.export_to_csv(&all_items),
            ExportFormat::Json => self.export_to_json(&all_items),
            ExportFormat::Excel => Err(ApplicationError::not_implemented(
                "Excel export not implemented".to_string(),
            )),
        }
    }

    fn export_to_csv(&self, items: &[JournalEntrySummary]) -> ServiceResult<Vec<u8>> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        // 写入表头
        wtr.write_record(&[
            "Document Number",
            "Fiscal Year",
            "Posting Date",
            "Document Date",
            "Currency",
            "Status",
            "Total Amount",
            "Line Count",
            "Header Text",
        ])?;

        // 写入数据
        for item in items {
            wtr.write_record(&[
                &item.document_number,
                &item.fiscal_year.to_string(),
                &item.posting_date.to_string(),
                &item.document_date.to_string(),
                &item.currency_code,
                &item.status,
                &item.total_amount.to_string(),
                &item.line_count.to_string(),
                item.header_text.as_deref().unwrap_or(""),
            ])?;
        }

        wtr.flush()?;
        Ok(wtr.into_inner()?)
    }

    fn export_to_json(&self, items: &[JournalEntrySummary]) -> ServiceResult<Vec<u8>> {
        let json = serde_json::to_vec_pretty(items)
            .map_err(|e| ApplicationError::infrastructure_error(format!("JSON serialization failed: {:?}", e)))?;
        Ok(json)
    }

    async fn search_entries(
        &self,
        tenant_id: Uuid,
        request: ListJournalEntriesRequest,
    ) -> ServiceResult<PagedResult<JournalEntrySummary>> {
        let query = ListJournalEntriesQuery::new(tenant_id, request);
        self.query_bus
            .execute(query)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to search entries: {:?}", e)))
    }
}

// =============================================================================
// 导出格式
// =============================================================================

/// 导出格式
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Csv,
    Json,
    Excel,
}

// =============================================================================
// 财务概览 DTO
// =============================================================================

/// 财务概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialOverview {
    pub company_code: String,
    pub fiscal_year: i32,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub net_income: f64,
    pub is_balanced: bool,
    pub total_entries: u64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}
