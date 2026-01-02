//! 凭证冲销事件处理器
//!
//! 处理 JournalEntryReversedEvent，更新读模型

use crate::application::error::ApplicationError;
use crate::application::events::*;
use crate::domain::events::*;
use crate::domain::repositories::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, span, Level};

// =============================================================================
// 事件处理器
// =============================================================================

/// 凭证冲销事件处理器
#[derive(Clone)]
pub struct JournalEntryReversedHandler<JRM>
where
    JRM: JournalEntryReadModel,
{
    /// 凭证读模型
    journal_entry_read_model: Arc<JRM>,
    /// 重试配置
    retry_config: RetryConfig,
}

impl<JRM> JournalEntryReversedHandler<JRM>
where
    JRM: JournalEntryReadModel,
{
    pub fn new(journal_entry_read_model: Arc<JRM>) -> Self {
        Self {
            journal_entry_read_model,
            retry_config: RetryConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl<JRM> EventHandler<JournalEntryReversedEvent> for JournalEntryReversedHandler<JRM>
where
    JRM: JournalEntryReadModel + Send + Sync,
{
    async fn handle(
        &self,
        envelope: &EventEnvelope<JournalEntryReversedEvent>,
    ) -> EventResult {
        let start_time = std::time::Instant::now();
        let event = &envelope.payload;
        let span = span!(Level::DEBUG, "JournalEntryReversedHandler", tenant_id = %event.tenant_id, document_number = %event.document_number);
        let _guard = span.enter();

        async move {
            let tenant_id = event.tenant_id;
            let document_number = event.document_number.clone();
            let reversal_document_number = event.reversal_document_number.clone();

            debug!(%tenant_id, %document_number, reversal = %reversal_document_number, "Processing JournalEntryReversedEvent");

            // 1. 更新原凭证状态为已冲销
            self.journal_entry_read_model
                .update_reversed_status(
                    &tenant_id,
                    &event.company_code,
                    event.fiscal_year,
                    &document_number,
                    event.reversed_at,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to update reversed status: {:?}",
                    e
                )))?;

            // 2. 更新冲销凭证到读模型
            self.journal_entry_read_model
                .upsert_journal_entry(
                    &tenant_id,
                    &event.company_code,
                    event.fiscal_year,
                    &reversal_document_number,
                    &event.reversal_line_items,
                    event.reversed_at,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to upsert reversal entry: {:?}",
                    e
                )))?;

            // 3. 更新科目余额（冲销金额）
            self.journal_entry_read_model
                .update_account_balances(
                    &tenant_id,
                    &event.company_code,
                    event.fiscal_year,
                    event.period,
                    &event.reversal_line_items,
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to update account balances for reversal: {:?}",
                    e
                )))?;

            debug!(%tenant_id, %document_number, "JournalEntryReversedEvent processed successfully");

            Ok(())
        }
        .await
    }
}
