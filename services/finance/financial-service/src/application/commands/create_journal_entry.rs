//! 创建会计凭证命令处理器
//!
//! 处理 CreateJournalEntryCommand，验证数据并创建凭证聚合根

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

/// 创建会计凭证命令
#[derive(Debug, Clone)]
pub struct CreateJournalEntryCommand {
    /// 命令上下文
    pub context: CommandContext,
    /// 创建请求
    pub request: CreateJournalEntryRequest,
}

impl CreateJournalEntryCommand {
    pub fn new(tenant_id: Uuid, user_id: Uuid, request: CreateJournalEntryRequest) -> Self {
        Self {
            context: CommandContext::new(tenant_id, user_id),
            request,
        }
    }
}

// =============================================================================
// 命令处理器
// =============================================================================

/// 创建凭证命令处理器
#[derive(Clone)]
pub struct CreateJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
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

impl<JR, JE, FPR, UOW, EB, DOC> CreateJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
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
impl<JR, JE, FPR, UOW, EB, DOC> CommandHandler<CreateJournalEntryCommand>
    for CreateJournalEntryHandler<JR, JE, FPR, UOW, EB, DOC>
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
        command: CreateJournalEntryCommand,
    ) -> Result<CreateJournalEntryResponse, crate::application::error::ApplicationError> {
        let start_time = std::time::Instant::now();
        let span = Span::current();

        async move {
            let tenant_id = command.context.tenant_id;
            let correlation_id = command.context.correlation_id;

            debug!(%tenant_id, %correlation_id, "Processing CreateJournalEntryCommand");

            // 1. 验证请求
            command.request.validate().map_err(|e| {
                ApplicationError::validation_failed(format!("Validation failed: {:?}", e))
            })?;

            // 2. 验证会计年度
            validate_fiscal_year(command.request.fiscal_year)?;

            // 3. 验证会计期间是否开放
            let fiscal_period = self
                .fiscal_period_repo
                .find_by_company_code_and_year_and_period(
                    &tenant_id,
                    &command.request.company_code,
                    command.request.fiscal_year,
                    get_period_from_date(command.request.posting_date.unwrap_or_default())?,
                )
                .await
                .map_err(map_domain_error)?
                .ok_or_else(|| {
                    ApplicationError::not_found(
                        "FiscalPeriod",
                        format!(
                            "Period not found for company {} year {}",
                            command.request.company_code, command.request.fiscal_year
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

            // 4. 生成凭证号
            let document_number = self
                .document_number_gen
                .generate(&tenant_id, &command.request.company_code)
                .await
                .map_err(|e| ApplicationError::infrastructure_error(format!("Document number generation failed: {:?}", e)))?;

            // 5. 创建聚合根
            let aggregate = journal_entry_from_dto(command.request, tenant_id, document_number.clone())
                .map_err(map_domain_error)?;

            // 6. 验证租户访问
            validate_tenant_access(tenant_id, aggregate.tenant_id())?;

            // 7. 开启工作单元
            self.uow.begin().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to begin transaction: {:?}", e))
            })?;

            // 8. 保存聚合根
            if let Err(e) = self.journal_entry_repo.save(&aggregate).await {
                self.uow.rollback().await.ok();
                return Err(map_domain_error(e));
            }

            // 9. 发布领域事件
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

            // 10. 提交事务
            self.uow.commit().await.map_err(|e| {
                ApplicationError::infrastructure_error(format!("Failed to commit transaction: {:?}", e))
            })?;

            // 11. 返回响应
            let response = CreateJournalEntryResponse {
                document_number,
                status: "DRAFT".to_string(),
                created_at: chrono::Utc::now(),
            };

            info!(%tenant_id, %correlation_id, %document_number, "Journal entry created successfully");

            Ok(response)
        }
        .instrument(span)
        .await
    }
}

/// 根据日期获取会计期间
fn get_period_from_date(date: chrono::NaiveDate) -> Result<u8, ApplicationError> {
    let month = date.month();
    // 简化处理：假设期间等于月份
    // 实际实现需要根据公司代码的会计日历来确定
    if month >= 1 && month <= 12 {
        Ok(month as u8)
    } else {
        Err(ApplicationError::validation_failed(
            "Invalid posting date".to_string(),
        ))
    }
}
