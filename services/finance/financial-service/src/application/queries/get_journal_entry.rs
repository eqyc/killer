//! 获取凭证详情查询处理器
//!
//! 处理 GetJournalEntryQuery，从读模型获取凭证详情

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

/// 获取凭证详情查询
#[derive(Debug, Clone)]
pub struct GetJournalEntryQuery {
    /// 查询上下文
    pub context: QueryContext,
    /// 查询请求
    pub request: GetJournalEntryRequest,
}

impl GetJournalEntryQuery {
    pub fn new(tenant_id: Uuid, request: GetJournalEntryRequest) -> Self {
        Self {
            context: QueryContext::new(tenant_id),
            request,
        }
    }
}

// =============================================================================
// 查询处理器
// =============================================================================

/// 获取凭证详情查询处理器
#[derive(Clone)]
pub struct GetJournalEntryHandler<JRM>
where
    JRM: JournalEntryReadModel,
{
    /// 凭证读模型
    journal_entry_read_model: Arc<JRM>,
}

impl<JRM> GetJournalEntryHandler<JRM>
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
impl<JRM> QueryHandler<GetJournalEntryQuery> for GetJournalEntryHandler<JRM>
where
    JRM: JournalEntryReadModel + Send + Sync,
{
    async fn handle(
        &self,
        _ctx: &CommandContext,
        query: GetJournalEntryQuery,
    ) -> Result<JournalEntryDetail, ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = query.context.tenant_id;
            let correlation_id = query.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing GetJournalEntryQuery");

            // 1. 验证请求
            query.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 从读模型获取凭证详情
            let detail = self
                .journal_entry_read_model
                .find_detail(
                    &tenant_id,
                    &query.request.company_code,
                    query.request.fiscal_year,
                    &query.request.document_number,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Read model error: {:?}", e)))?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "JournalEntry",
                        format!(
                            "Journal entry {} not found",
                            query.request.document_number
                        ),
                    )
                })?;

            // 3. 记录查询指标
            record_query_metrics("GetJournalEntry", true, start_time.elapsed());

            debug!(%tenant_id, %correlation_id, document_number = %detail.document_number, "Journal entry retrieved successfully");

            Ok(detail)
        }
        .instrument(span)
        .await
    }
}
