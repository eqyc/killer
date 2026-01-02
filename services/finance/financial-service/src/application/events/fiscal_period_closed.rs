//! 会计期间关闭事件处理器
//!
//! 处理 FiscalPeriodClosedEvent，更新读模型

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

/// 会计期间关闭事件处理器
#[derive(Clone)]
pub struct FiscalPeriodClosedHandler<FPRM>
where
    FPRM: FiscalPeriodReadModel,
{
    /// 会计期间读模型
    fiscal_period_read_model: Arc<FPRM>,
    /// 重试配置
    retry_config: RetryConfig,
}

impl<FPRM> FiscalPeriodClosedHandler<FPRM>
where
    FPRM: FiscalPeriodReadModel,
{
    pub fn new(fiscal_period_read_model: Arc<FPRM>) -> Self {
        Self {
            fiscal_period_read_model,
            retry_config: RetryConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl<FPRM> EventHandler<FiscalPeriodClosedEvent> for FiscalPeriodClosedHandler<FPRM>
where
    FPRM: FiscalPeriodReadModel + Send + Sync,
{
    async fn handle(
        &self,
        envelope: &EventEnvelope<FiscalPeriodClosedEvent>,
    ) -> EventResult {
        let start_time = std::time::Instant::now();
        let event = &envelope.payload;
        let span = span!(Level::DEBUG, "FiscalPeriodClosedHandler", tenant_id = %event.tenant_id, company_code = %event.company_code);
        let _guard = span.enter();

        async move {
            let tenant_id = event.tenant_id;
            let company_code = event.company_code.clone();

            debug!(%tenant_id, %company_code, year = %event.fiscal_year, period = %event.period, "Processing FiscalPeriodClosedEvent");

            // 更新会计期间读模型
            self.fiscal_period_read_model
                .update_period_status(
                    &tenant_id,
                    &company_code,
                    event.fiscal_year,
                    event.period,
                    "CLOSED".to_string(),
                )
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!(
                    "Failed to update fiscal period read model: {:?}",
                    e
                )))?;

            debug!(%tenant_id, %company_code, "FiscalPeriodClosedEvent processed successfully");

            Ok(())
        }
        .await
    }
}
