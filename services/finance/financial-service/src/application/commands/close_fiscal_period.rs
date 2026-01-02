//! 关闭会计期间命令处理器
//!
//! 处理 CloseFiscalPeriodCommand，关闭会计期间

use crate::application::commands::*;
use crate::application::dto::*;
use crate::application::mapper::*;
use crate::application::repositories::*;
use crate::domain::*;
use killer_cqrs::{Command, CommandHandler, Result as CqrsResult};
use std::sync::Arc;
use tracing::{debug, info, Span};

// =============================================================================
// 命令定义
// =============================================================================

/// 关闭会计期间命令
#[derive(Debug, Clone)]
pub struct CloseFiscalPeriodCommand {
    /// 命令上下文
    pub context: CommandContext,
    /// 关闭请求
    pub request: CloseFiscalPeriodRequest,
}

impl CloseFiscalPeriodCommand {
    pub fn new(tenant_id: Uuid, user_id: Uuid, request: CloseFiscalPeriodRequest) -> Self {
        Self {
            context: CommandContext::new(tenant_id, user_id),
            request,
        }
    }
}

// =============================================================================
// 命令处理器
// =============================================================================

/// 关闭会计期间命令处理器
#[derive(Clone)]
pub struct CloseFiscalPeriodHandler<JR, FPR, UOW, EB>
where
    JR: JournalEntryRepository,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
{
    /// 凭证仓储（用于检查未过账凭证）
    journal_entry_repo: Arc<JR>,
    /// 会计期间仓储
    fiscal_period_repo: Arc<FPR>,
    /// 工作单元
    uow: Arc<UOW>,
    /// 事件总线
    event_bus: Arc<EB>,
}

impl<JR, FPR, UOW, EB> CloseFiscalPeriodHandler<JR, FPR, UOW, EB>
where
    JR: JournalEntryRepository,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
{
    pub fn new(
        journal_entry_repo: Arc<JR>,
        fiscal_period_repo: Arc<FPR>,
        uow: Arc<UOW>,
        event_bus: Arc<EB>,
    ) -> Self {
        Self {
            journal_entry_repo,
            fiscal_period_repo,
            uow,
            event_bus,
        }
    }
}

#[async_trait::async_trait]
impl<JR, FPR, UOW, EB> CommandHandler<CloseFiscalPeriodCommand>
    for CloseFiscalPeriodHandler<JR, FPR, UOW, EB>
where
    JR: JournalEntryRepository + Send + Sync,
    FPR: FiscalPeriodRepository + Send + Sync,
    UOW: UnitOfWork + Send + Sync,
    EB: EventBus + Send + Sync,
{
    async fn handle(
        &self,
        command: CloseFiscalPeriodCommand,
    ) -> Result<CloseFiscalPeriodResponse, crate::application::error::ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = command.context.tenant_id;
            let correlation_id = command.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing CloseFiscalPeriodCommand");

            // 1. 验证请求
            command.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证会计年度
            validate_fiscal_year(command.request.fiscal_year)?;

            // 3. 加载会计期间聚合根
            let fiscal_period = self
                .fiscal_period_repo
                .find_by_company_code_and_year_and_period(
                    &tenant_id,
                    &command.request.company_code,
                    command.request.fiscal_year,
                    command.request.period,
                )
                .await
                .map_err(map_domain_error)?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "FiscalPeriod",
                        format!(
                            "Fiscal period {}/{} not found for company {}",
                            command.request.period,
                            command.request.fiscal_year,
                            command.request.company_code
                        ),
                    )
                })?;

            // 4. 验证租户访问
            validate_tenant_access(tenant_id, fiscal_period.tenant_id())?;

            // 5. 检查是否已经关闭
            if !fiscal_period.is_open() {
                return Err(ApplicationError::conflict(
                    "Fiscal period is already closed".to_string(),
                ));
            }

            // 6. 如果不是强制关闭，检查是否有未过账凭证
            let force = command.request.force.unwrap_or(false);
            if !force {
                let unposted_count = self
                    .journal_entry_repo
                    .count_unposted_by_company_and_period(
                        &tenant_id,
                        &command.request.company_code,
                        command.request.fiscal_year,
                        command.request.period,
                    )
                    .await
                    .map_err(map_domain_error)?;

                if unposted_count > 0 {
                    return Err(ApplicationError::business_rule_violation(
                        "UNPOSTED_ENTRIES_EXIST",
                        format!(
                            "Cannot close period with {} unposted journal entries. Use force=true to override.",
                            unposted_count
                        ),
                    ));
                }
            }

            // 7. 开启工作单元
            self.uow.begin().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to begin transaction: {:?}", e))
            })?;

            // 8. 执行关闭操作
            fiscal_period.close().map_err(map_domain_error)?;

            // 9. 保存聚合根
            if let Err(e) = self.fiscal_period_repo.save(&fiscal_period).await {
                self.uow.rollback().await.ok();
                return Err(map_domain_error(e));
            }

            // 10. 发布领域事件
            let events = fiscal_period.take_events();
            for event in events {
                if let Err(e) = self.event_bus.publish(event).await {
                    self.uow.rollback().await.ok();
                    return Err(ApplicationError::infrastructure_error(format!("Failed to publish event: {:?}", e)));
                }
            }

            // 11. 提交事务
            self.uow.commit().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to commit transaction: {:?}", e))
            })?;

            // 12. 返回响应
            let response = CloseFiscalPeriodResponse {
                company_code: command.request.company_code,
                fiscal_year: command.request.fiscal_year,
                period: command.request.period,
                status: "CLOSED".to_string(),
                valid_from: fiscal_period.valid_from(),
                valid_to: fiscal_period.valid_to(),
            };

            info!(%tenant_id, %correlation_id, company = %response.company_code, year = %response.fiscal_year, period = %response.period, "Fiscal period closed successfully");

            Ok(response)
        }
        .instrument(span)
        .await
    }
}
