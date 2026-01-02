//! 凭证过账事件处理器
//!
//! 处理 JournalEntryPostedEvent，更新读模型并触发 FI-MM 集成

use crate::application::error::ApplicationError;
use crate::application::events::*;
use crate::domain::events::*;
use crate::domain::repositories::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, info, span, Level};

// =============================================================================
// 事件处理器
// =============================================================================

/// 凭证过账事件处理器
#[derive(Clone)]
pub struct JournalEntryPostedHandler<JRM, EB>
where
    JRM: JournalEntryReadModel,
    EB: EventBus,
{
    /// 凭证读模型
    journal_entry_read_model: Arc<JRM>,
    /// 事件总线（用于发布跨服务事件）
    event_bus: Arc<EB>,
    /// 重试配置
    retry_config: RetryConfig,
}

impl<JRM, EB> JournalEntryPostedHandler<JRM, EB>
where
    JRM: JournalEntryReadModel,
    EB: EventBus,
{
    pub fn new(
        journal_entry_read_model: Arc<JRM>,
        event_bus: Arc<EB>,
    ) -> Self {
        Self {
            journal_entry_read_model,
            event_bus,
            retry_config: RetryConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl<JRM, EB> EventHandler<JournalEntryPostedEvent> for JournalEntryPostedHandler<JRM, EB>
where
    JRM: JournalEntryReadModel + Send + Sync,
    EB: EventBus + Send + Sync,
{
    async fn handle(
        &self,
        envelope: &EventEnvelope<JournalEntryPostedEvent>,
    ) -> EventResult {
        let start_time = std::time::Instant::now();
        let event = &envelope.payload;
        let span = span!(Level::DEBUG, "JournalEntryPostedHandler", tenant_id = %event.tenant_id, document_number = %event.document_number);
        let _guard = span.enter();

        async move {
            let tenant_id = event.tenant_id;
            let document_number = event.document_number.clone();

            debug!(%tenant_id, %document_number, "Processing JournalEntryPostedEvent");

            // 1. 更新凭证读模型
            self.journal_entry_read_model
                .update_posted_status(
                    &tenant_id,
                    &event.company_code,
                    event.fiscal_year,
                    &document_number,
                    event.posted_at,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to update read model: {:?}",
                    e
                )))?;

            // 2. 更新科目余额读模型
            self.journal_entry_read_model
                .update_account_balances(
                    &tenant_id,
                    &event.company_code,
                    event.fiscal_year,
                    event.period,
                    &event.line_items,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to update account balances: {:?}",
                    e
                )))?;

            debug!(%tenant_id, %document_number, "JournalEntryPostedEvent processed successfully");

            Ok(())
        }
        .await
    }
}
