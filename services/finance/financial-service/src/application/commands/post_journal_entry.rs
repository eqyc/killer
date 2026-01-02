//! 过账会计凭证命令处理器
//!
//! 处理 PostJournalEntryCommand，验证并执行凭证过账

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

/// 过账会计凭证命令
#[derive(Debug, Clone)]
pub struct PostJournalEntryCommand {
    /// 命令上下文
    pub context: CommandContext,
    /// 过账请求
    pub request: PostJournalEntryRequest,
}

impl PostJournalEntryCommand {
    pub fn new(tenant_id: Uuid, user_id: Uuid, request: PostJournalEntryRequest) -> Self {
        Self {
            context: CommandContext::new(tenant_id, user_id),
            request,
        }
    }
}

// =============================================================================
// 命令处理器
// =============================================================================

/// 过账凭证命令处理器
#[derive(Clone)]
pub struct PostJournalEntryHandler<JR, JE, FPR, UOW, EB>
where
    JR: JournalEntryRepository,
    JE: JournalEntryEventStore,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
{
    /// 凭证仓储
    journal_entry_repo: Arc<JR>,
    /// 凭证事件存储
    journal_entry_event_store: Arc<JE>,
    /// 会计期间仓储
    fiscal_period_repo: Arc<FPR>,
    /// 工作单元
    uow: Arc<UOW>,
    /// 事件总线
    event_bus: Arc<EB>,
}

impl<JR, JE, FPR, UOW, EB> PostJournalEntryHandler<JR, JE, FPR, UOW, EB>
where
    JR: JournalEntryRepository,
    JE: JournalEntryEventStore,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
{
    pub fn new(
        journal_entry_repo: Arc<JR>,
        journal_entry_event_store: Arc<JE>,
        fiscal_period_repo: Arc<FPR>,
        uow: Arc<UOW>,
        event_bus: Arc<EB>,
    ) -> Self {
        Self {
            journal_entry_repo,
            journal_entry_event_store,
            fiscal_period_repo,
            uow,
            event_bus,
        }
    }
}

#[async_trait::async_trait]
impl<JR, JE, FPR, UOW, EB> CommandHandler<PostJournalEntryCommand>
    for PostJournalEntryHandler<JR, JE, FPR, UOW, EB>
where
    JR: JournalEntryRepository + Send + Sync,
    JE: JournalEntryEventStore + Send + Sync,
    FPR: FiscalPeriodRepository + Send + Sync,
    UOW: UnitOfWork + Send + Sync,
    EB: EventBus + Send + Sync,
{
    async fn handle(
        &self,
        command: PostJournalEntryCommand,
    ) -> Result<PostJournalEntryResponse, crate::application::error::ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = command.context.tenant_id;
            let correlation_id = command.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing PostJournalEntryCommand");

            // 1. 验证请求
            command.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证凭证号格式
            validate_document_number(&command.request.document_number)?;

            // 3. 加载聚合根
            let aggregate_id = JournalEntryId::new(
                tenant_id,
                command.request.company_code.clone(),
                command.request.fiscal_year,
                command.request.document_number.clone(),
            );

            let aggregate = self
                .journal_entry_repo
                .find_by_id(&aggregate_id)
                .await
                .map_err(map_domain_error)?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "JournalEntry",
                        format!(
                            "Journal entry {} not found",
                            command.request.document_number
                        ),
                    )
                })?;

            // 4. 验证租户访问
            validate_tenant_access(tenant_id, aggregate.tenant_id())?;

            // 5. 验证凭证状态
            if aggregate.is_posted() {
                return Err(ApplicationError::conflict(
                    "Journal entry is already posted".to_string(),
                ));
            }

            // 6. 确定过账日期
            let posting_date = command
                .request
                .posting_date
                .unwrap_or_else(|| aggregate.posting_date());

            // 7. 验证会计期间是否开放
            let posting_period = get_period_from_date(posting_date)?;
            let fiscal_period = self
                .fiscal_period_repo
                .find_by_company_code_and_year_and_period(
                    &tenant_id,
                    &command.request.company_code,
                    command.request.fiscal_year,
                    posting_period,
                )
                .await
                .map_err(map_domain_error)?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "FiscalPeriod",
                        format!(
                            "Period not found for company {} year {} period {}",
                            command.request.company_code,
                            command.request.fiscal_year,
                            posting_period
                        ),
                    )
                })?;

            if !fiscal_period.is_open() {
                return Err(ApplicationError::business_rule_violation(
                    "FISCAL_PERIOD_CLOSED",
                    format!(
                        "Fiscal period {} of {} is closed",
                        fiscal_period.period(), fiscal_period.fiscal_year()
                    ),
                ));
            }

            // 8. 开启工作单元
            self.uow.begin().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to begin transaction: {:?}", e))
            })?;

            // 9. 执行过账操作
            aggregate.post(posting_date).map_err(map_domain_error)?;

            // 10. 保存聚合根
            if let Err(e) = self.journal_entry_repo.save(&aggregate).await {
                self.uow.rollback().await.ok();
                return Err(map_domain_error(e));
            }

            // 11. 发布领域事件
            let events = aggregate.take_events();
            for event in events {
                if let Err(e) = self.journal_entry_event_store
                    .append(&tenant_id, &aggregate.id(), event.clone())
                    .await
                {
                    self.uow.rollback().await.ok();
                    return Err(ApplicationError::infrastructure_error(format!("Failed to store event: {:?}", e)));
                }

                if let Err(e) = self.event_bus.publish(event).await {
                    self.uow.rollback().await.ok();
                    return Err(ApplicationError::infrastructure_error(format!("Failed to publish event: {:?}", e)));
                }
            }

            // 12. 提交事务
            self.uow.commit().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to commit transaction: {:?}", e))
            })?;

            // 13. 返回响应
            let (total_debit, total_credit) = aggregate.totals();
            let response = PostJournalEntryResponse {
                document_number: command.request.document_number,
                status: "POSTED".to_string(),
                posting_date,
                total_debit: total_debit.as_f64(),
                total_credit: total_credit.as_f64(),
            };

            info!(%tenant_id, %correlation_id, %response.document_number, "Journal entry posted successfully");

            Ok(response)
        }
        .instrument(span)
        .await
    }
}

/// 根据日期获取会计期间
fn get_period_from_date(date: chrono::NaiveDate) -> Result<u8, ApplicationError> {
    let month = date.month();
    if month >= 1 && month <= 12 {
        Ok(month as u8)
    } else {
        Err(ApplicationError::validation_failed(
            "Invalid posting date".to_string(),
        ))
    }
}
