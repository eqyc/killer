//! 冲销会计凭证命令处理器
//!
//! 处理 ReverseJournalEntryCommand，创建冲销凭证

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

/// 冲销会计凭证命令
#[derive(Debug, Clone)]
pub struct ReverseJournalEntryCommand {
    /// 命令上下文
    pub context: CommandContext,
    /// 冲销请求
    pub request: ReverseJournalEntryRequest,
}

impl ReverseJournalEntryCommand {
    pub fn new(tenant_id: Uuid, user_id: Uuid, request: ReverseJournalEntryRequest) -> Self {
        Self {
            context: CommandContext::new(tenant_id, user_id),
            request,
        }
    }
}

// =============================================================================
// 命令处理器
// =============================================================================

/// 冲销凭证命令处理器
#[derive(Clone)]
pub struct ReverseJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
where
    JR: JournalEntryRepository,
    JE: JournalEntryEventStore,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
    DOC: DocumentNumberGenerator,
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
    /// 凭证号生成器
    document_number_gen: Arc<DOC>,
}

impl<JR, JE, FPR, UOW, EB, DOC> ReverseJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
where
    JR: JournalEntryRepository,
    JE: JournalEntryEventStore,
    FPR: FiscalPeriodRepository,
    UOW: UnitOfWork,
    EB: EventBus,
    DOC: DocumentNumberGenerator,
{
    pub fn new(
        journal_entry_repo: Arc<JR>,
        journal_entry_event_store: Arc<JE>,
        fiscal_period_repo: Arc<FPR>,
        uow: Arc<UOW>,
        event_bus: Arc<EB>,
        document_number_gen: Arc<DOC>,
    ) -> Self {
        Self {
            journal_entry_repo,
            journal_entry_event_store,
            fiscal_period_repo,
            uow,
            event_bus,
            document_number_gen,
        }
    }
}

#[async_trait::async_trait]
impl<JR, JE, FPR, UOW, EB, DOC> CommandHandler<ReverseJournalEntryCommand>
    for ReverseJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
where
    JR: JournalEntryRepository + Send + Sync,
    JE: JournalEntryEventStore + Send + Sync,
    FPR: FiscalPeriodRepository + Send + Sync,
    UOW: UnitOfWork + Send + Sync,
    EB: EventBus + Send + Sync,
    DOC: DocumentNumberGenerator + Send + Sync,
{
    async fn handle(
        &self,
        command: ReverseJournalEntryCommand,
    ) -> Result<ReverseJournalEntryResponse, crate::application::error::ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = command.context.tenant_id;
            let correlation_id = command.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing ReverseJournalEntryCommand");

            // 1. 验证请求
            command.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证原凭证号格式
            validate_document_number(&command.request.original_document_number)?;

            // 3. 加载原凭证聚合根
            let original_aggregate_id = JournalEntryId::new(
                tenant_id,
                command.request.company_code.clone(),
                command.request.fiscal_year,
                command.request.original_document_number.clone(),
            );

            let original_aggregate = self
                .journal_entry_repo
                .find_by_id(&original_aggregate_id)
                .await
                .map_err(map_domain_error)?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "JournalEntry",
                        format!(
                            "Original journal entry {} not found",
                            command.request.original_document_number
                        ),
                    )
                })?;

            // 4. 验证租户访问
            validate_tenant_access(tenant_id, original_aggregate.tenant_id())?;

            // 5. 验证原凭证状态
            if !original_aggregate.is_posted() {
                return Err(ApplicationError::business_rule_violation(
                    "ENTRY_NOT_POSTED",
                    "Cannot reverse a journal entry that is not posted".to_string(),
                ));
            }

            if original_aggregate.is_reversed() {
                return Err(ApplicationError::conflict(
                    "Journal entry is already reversed".to_string(),
                ));
            }

            // 6. 确定冲销日期
            let reversal_date = command
                .request
                .reversal_date
                .ok_or_else(|| ApplicationError::validation_failed(
                    "Reversal date is required".to_string(),
                ))?;

            // 7. 验证会计期间是否开放
            let reversal_period = get_period_from_date(reversal_date)?;
            let fiscal_period = self
                .fiscal_period_repo
                .find_by_company_code_and_year_and_period(
                    &tenant_id,
                    &command.request.company_code,
                    command.request.fiscal_year,
                    reversal_period,
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
                            reversal_period
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

            // 8. 生成冲销凭证号
            let reversal_document_number = self
                .document_number_gen
                .generate(&tenant_id, &command.request.company_code)
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Document number generation failed: {:?}", e)))?;

            // 9. 创建冲销凭证
            let reversal_reason = command.request.reversal_reason.unwrap_or(1);
            let reversal_aggregate = original_aggregate.reverse(
                reversal_document_number.clone(),
                reversal_date,
                reversal_reason,
            ).map_err(map_domain_error)?;

            // 10. 开启工作单元
            self.uow.begin().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to begin transaction: {:?}", e))
            })?;

            // 11. 保存原凭证（标记为已冲销）
            if let Err(e) = self.journal_entry_repo.save(&original_aggregate).await {
                self.uow.rollback().await.ok();
                return Err(map_domain_error(e));
            }

            // 12. 保存冲销凭证
            if let Err(e) = self.journal_entry_repo.save(&reversal_aggregate).await {
                self.uow.rollback().await.ok();
                return Err(map_domain_error(e));
            }

            // 13. 发布领域事件
            let original_events = original_aggregate.take_events();
            let reversal_events = reversal_aggregate.take_events();

            for event in original_events.into_iter().chain(reversal_events) {
                if let Err(e) = self.journal_entry_event_store
                    .append(&tenant_id, &original_aggregate_id, event.clone())
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

            // 14. 提交事务
            self.uow.commit().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to commit transaction: {:?}", e))
            })?;

            // 15. 返回响应
            let response = ReverseJournalEntryResponse {
                original_document_number: command.request.original_document_number,
                reversal_document_number,
                reversal_date,
                status: "POSTED".to_string(),
            };

            info!(%tenant_id, %correlation_id, original = %response.original_document_number, reversal = %response.reversal_document_number, "Journal entry reversed successfully");

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
            "Invalid date".to_string(),
        ))
    }
}
