//! 试算平衡表查询处理器
//!
//! 处理 GetTrialBalanceQuery，获取试算平衡表

use crate::application::dto::*;
use crate::application::error::ApplicationError;
use crate::application::queries::*;
use crate::domain::repositories::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, Span};

// =============================================================================
// 查询定义
// =============================================================================

/// 试算平衡表查询
#[derive(Debug, Clone)]
pub struct GetTrialBalanceQuery {
    /// 查询上下文
    pub context: QueryContext,
    /// 查询请求
    pub request: GetTrialBalanceRequest,
}

impl GetTrialBalanceQuery {
    pub fn new(tenant_id: Uuid, request: GetTrialBalanceRequest) -> Self {
        Self {
            context: QueryContext::new(tenant_id),
            request,
        }
    }
}

// =============================================================================
// 查询处理器
// =============================================================================

/// 试算平衡表查询处理器
#[derive(Clone)]
pub struct GetTrialBalanceHandler<TBRM>
where
    TBRM: TrialBalanceReadModel,
{
    /// 试算平衡表读模型
    trial_balance_read_model: Arc<TBRM>,
}

impl<TBRM> GetTrialBalanceHandler<TBRM>
where
    TBRM: TrialBalanceReadModel,
{
    pub fn new(trial_balance_read_model: Arc<TBRM>) -> Self {
        Self {
            trial_balance_read_model,
        }
    }
}

#[async_trait::async_trait]
impl<TBRM> QueryHandler<GetTrialBalanceQuery> for GetTrialBalanceHandler<TBRM>
where
    TBRM: TrialBalanceReadModel + Send + Sync,
{
    async fn handle(
        &self,
        _ctx: &CommandContext,
        query: GetTrialBalanceQuery,
    ) -> Result<TrialBalanceSummary, ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = query.context.tenant_id;
            let correlation_id = query.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing GetTrialBalanceQuery");

            // 1. 验证请求
            query.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证会计年度
            validate_fiscal_year(query.request.fiscal_year)?;

            // 3. 从读模型获取试算平衡表
            let summary = self
                .trial_balance_read_model
                .find_trial_balance(
                    &tenant_id,
                    &query.request.company_code,
                    query.request.fiscal_year,
                    query.request.period,
                    query.request.expand_hierarchy.unwrap_or(false),
                    query.request.hide_zero_balance.unwrap_or(false),
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Read model error: {:?}", e)))?;

            // 4. 计算差额和平衡状态
            let difference = summary.total_debit - summary.total_credit;
            let is_balanced = difference.abs() < 0.01; // 允许微小误差

            let result = TrialBalanceSummary {
                company_code: summary.company_code,
                fiscal_year: summary.fiscal_year,
                period: summary.period,
                total_debit: summary.total_debit,
                total_credit: summary.total_credit,
                difference,
                is_balanced,
                lines: summary.lines,
            };

            // 5. 记录查询指标
            record_query_metrics("GetTrialBalance", true, start_time.elapsed());

            debug!(%tenant_id, %correlation_id, is_balanced = %result.is_balanced, "Trial balance retrieved successfully");

            Ok(result)
        }
        .instrument(span)
        .await
    }
}
