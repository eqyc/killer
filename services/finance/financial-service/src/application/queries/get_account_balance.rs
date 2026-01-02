//! 科目余额查询处理器
//!
//! 处理 GetAccountBalanceQuery，获取科目余额信息

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

/// 科目余额查询
#[derive(Debug, Clone)]
pub struct GetAccountBalanceQuery {
    /// 查询上下文
    pub context: QueryContext,
    /// 查询请求
    pub request: GetAccountBalanceRequest,
}

impl GetAccountBalanceQuery {
    pub fn new(tenant_id: Uuid, request: GetAccountBalanceRequest) -> Self {
        Self {
            context: QueryContext::new(tenant_id),
            request,
        }
    }
}

// =============================================================================
// 查询处理器
// =============================================================================

/// 科目余额查询处理器
#[derive(Clone)]
pub struct GetAccountBalanceHandler<ABRM>
where
    ABRM: AccountBalanceReadModel,
{
    /// 科目余额读模型
    account_balance_read_model: Arc<ABRM>,
}

impl<ABRM> GetAccountBalanceHandler<ABRM>
where
    ABRM: AccountBalanceReadModel,
{
    pub fn new(account_balance_read_model: Arc<ABRM>) -> Self {
        Self {
            account_balance_read_model,
        }
    }
}

#[async_trait::async_trait]
impl<ABRM> QueryHandler<GetAccountBalanceQuery> for GetAccountBalanceHandler<ABRM>
where
    ABRM: AccountBalanceReadModel + Send + Sync,
{
    async fn handle(
        &self,
        _ctx: &CommandContext,
        query: GetAccountBalanceQuery,
    ) -> Result<AccountBalance, ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = query.context.tenant_id;
            let correlation_id = query.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing GetAccountBalanceQuery");

            // 1. 验证请求
            query.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证会计年度
            validate_fiscal_year(query.request.fiscal_year)?;

            // 3. 从读模型获取科目余额
            let balance = self
                .account_balance_read_model
                .find_balance(
                    &tenant_id,
                    &query.request.company_code,
                    query.request.fiscal_year,
                    &query.request.account_code,
                    query.request.period,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Read model error: {:?}", e)))?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "AccountBalance",
                        format!(
                            "Account balance not found for account {} in company {} year {}",
                            query.request.account_code,
                            query.request.company_code,
                            query.request.fiscal_year
                        ),
                    )
                })?;

            // 4. 记录查询指标
            record_query_metrics("GetAccountBalance", true, start_time.elapsed());

            debug!(%tenant_id, %correlation_id, account = %balance.account_code, "Account balance retrieved successfully");

            Ok(balance)
        }
        .instrument(span)
        .await
    }
}
