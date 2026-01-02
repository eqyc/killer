//! 列表查询处理器
//!
//! 处理 ListJournalEntriesQuery，分页获取凭证列表

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

/// 列表查询请求扩展
#[derive(Debug, Clone)]
pub struct ListJournalEntriesQuery {
    /// 查询上下文
    pub context: QueryContext,
    /// 查询请求
    pub request: ListJournalEntriesRequest,
}

impl ListJournalEntriesQuery {
    pub fn new(tenant_id: Uuid, request: ListJournalEntriesRequest) -> Self {
        Self {
            context: QueryContext::new(tenant_id),
            request,
        }
    }
}

// =============================================================================
// 查询处理器
// =============================================================================

/// 列表查询处理器
#[derive(Clone)]
pub struct ListJournalEntriesHandler<JRM>
where
    JRM: JournalEntryReadModel,
{
    /// 凭证读模型
    journal_entry_read_model: Arc<JRM>,
}

impl<JRM> ListJournalEntriesHandler<JRM>
where
    JRM: JournalEntryReadModel,
{
    pub fn new(journal_entry_read_model: Arc<JRM>) -> Self {
        Self {
            journal_entry_read_model,
        }
    }
}

#[async_trait::async_trait]
impl<JRM> QueryHandler<ListJournalEntriesQuery> for ListJournalEntriesHandler<JRM>
where
    JRM: JournalEntryReadModel + Send + Sync,
{
    async fn handle(
        &self,
        _ctx: &CommandContext,
        query: ListJournalEntriesQuery,
    ) -> Result<PagedResult<JournalEntrySummary>, ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = query.context.tenant_id;
            let correlation_id = query.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing ListJournalEntriesQuery");

            // 1. 验证请求
            query.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证分页参数
            let (page, page_size) = validate_pagination_params(
                query.request.page,
                query.request.page_size,
            )?;

            // 3. 验证日期范围
            validate_date_range(
                query.request.posting_date_from,
                query.request.posting_date_to,
            )?;

            // 4. 验证金额范围
            validate_amount_range(
                query.request.amount_min,
                query.request.amount_max,
            )?;

            // 5. 从读模型获取列表
            let (items, total_count) = self
                .journal_entry_read_model
                .find_summaries(
                    &tenant_id,
                    query.request.company_code.as_deref(),
                    query.request.fiscal_year,
                    query.request.status.as_deref(),
                    query.request.posting_date_from,
                    query.request.posting_date_to,
                    query.request.account_code.as_deref(),
                    query.request.cost_center.as_deref(),
                    query.request.amount_min,
                    query.request.amount_max,
                    query.request.text_search.as_deref(),
                    page,
                    page_size,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Read model error: {:?}", e)))?;

            // 6. 计算总页数
            let total_pages = ((total_count as u64 + page_size as u64 - 1) / page_size as u64) as u32;

            // 7. 构建分页结果
            let result = PagedResult {
                items,
                total_count,
                page,
                page_size,
                total_pages,
            };

            // 8. 记录查询指标
            record_query_metrics("ListJournalEntries", true, start_time.elapsed());

            debug!(%tenant_id, %correlation_id, total = %result.total_count, page = %result.page, "Journal entries list retrieved successfully");

            Ok(result)
        }
        .instrument(span)
        .await
    }
}
