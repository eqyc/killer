//! Journal Entry gRPC 服务实现
//!
//! 实现 JournalEntryService 的所有 RPC 方法

use crate::error::{
    map_application_error, map_domain_error, ApiError, ApiResult, ErrorCode,
};
use crate::middleware::audit::{AuditAction, AuditMiddleware, AuditStatus};
use crate::middleware::auth::{extract_auth_context, AuthContext};
use crate::middleware::idempotency::{IdempotencyMiddleware, IdempotencyResult};
use crate::middleware::metrics::ApiMetrics;
use chrono::{DateTime, NaiveDate, Utc};
use killer_financial_service::application::commands::*;
use killer_financial_service::application::dto::*;
use killer_financial_service::application::queries::*;
use killer_financial_service::application::services::*;
use killer_financial_service::domain::*;
use killer_financial_service::application::error::ApplicationError;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};
use tracing::{debug, error, info, span, Instrument, Level};
use uuid::Uuid;

/// Journal Entry gRPC 服务实现
#[derive(Clone)]
pub struct JournalEntryGrpcService {
    /// 应用服务
    journal_entry_app_service: Arc<JournalEntryApplicationService>,

    /// 报表服务
    reporting_service: Arc<ReportingService>,

    /// 认证拦截器
    auth_interceptor: Arc<dyn crate::middleware::auth::AuthInterceptorTrait + Send + Sync>,

    /// 幂等性中间件
    idempotency: Option<Arc<IdempotencyMiddleware>>,

    /// 审计中间件
    audit: Option<Arc<AuditMiddleware>>,

    /// 指标
    metrics: Arc<ApiMetrics>,
}

impl JournalEntryGrpcService {
    /// 创建新的服务
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        journal_entry_app_service: Arc<JournalEntryApplicationService>,
        reporting_service: Arc<ReportingService>,
        auth_interceptor: Arc<dyn crate::middleware::auth::AuthInterceptorTrait + Send + Sync>,
        idempotency: Option<Arc<IdempotencyMiddleware>>,
        audit: Option<Arc<AuditMiddleware>>,
        metrics: Arc<ApiMetrics>,
    ) -> Self {
        Self {
            journal_entry_app_service,
            reporting_service,
            auth_interceptor,
            idempotency,
            audit,
            metrics,
        }
    }

    /// 获取认证上下文
    fn auth_context(&self, request: &Request<()>) -> Result<AuthContext, Status> {
        self.auth_interceptor.authenticate(request)
    }

    /// 创建追踪 span
    fn create_span(&self, name: &str, auth: &AuthContext) -> tracing::Span {
        span!(
            Level::INFO,
            "{}",
            name,
            tenant_id = %auth.tenant_id,
            user_id = %auth.user_id,
        )
    }
}

#[tonic::async_trait]
impl crate::finance_v1::journal_entry_service_server::JournalEntryService
    for JournalEntryGrpcService
{
    /// 创建会计凭证
    async fn create_journal_entry(
        &self,
        request: Request<super::super::super::CreateJournalEntryRequest>,
    ) -> Result<Response<super::super::super::CreateJournalEntryResponse>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("create_journal_entry", &auth);

        // 记录指标
        self.metrics.inc_active_requests();
        let start_time = std::time::Instant::now();

        let tenant_id = auth.tenant_id;
        let user_id = auth.user_id;

        async move {
            // 提取幂等键
            let idempotency_key = request
                .metadata()
                .get("idempotency-key")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // 幂等性检查
            if let Some(ref key) = idempotency_key {
                if let Some(ref middleware) = self.idempotency {
                    match middleware.try_get_cached_response(key).await {
                        Ok(Some(response)) => {
                            self.metrics.record_idempotency_hit();
                            return Ok(Response::new(response));
                        }
                        Ok(None) => {}
                        Err(e) => debug!(%key, "Idempotency check error: {}", e),
                    }
                }
            }

            // 构建命令
            let req = request.into_inner();
            let command = CreateJournalEntryCommand {
                tenant_id: tenant_id.to_string(),
                user_id: user_id.to_string(),
                company_code: req.company_code,
                fiscal_year: req.fiscal_year,
                posting_date: req
                    .posting_date
                    .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0).unwrap())
                    .ok_or(Status::invalid_argument("posting_date is required"))?,
                document_date: req
                    .document_date
                    .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0).unwrap())
                    .ok_or(Status::invalid_argument("document_date is required"))?,
                currency_code: req.currency_code,
                header_text: Some(req.header_text).filter(|s| !s.is_empty()),
                reference_document: Some(req.reference_document).filter(|s| !s.is_empty()),
                line_items: req
                    .line_items
                    .into_iter()
                    .map(|l| JournalEntryLineItemRequest {
                        line_number: l.line_number,
                        account_code: l.account_code,
                        amount: l.amount,
                        debit_credit: l.debit_credit,
                        cost_center: Some(l.cost_center).filter(|s| !s.is_empty()),
                        profit_center: Some(l.profit_center).filter(|s| !s.is_empty()),
                        text: Some(l.text).filter(|s| !s.is_empty()),
                        functional_area: Some(l.functional_area).filter(|s| !s.is_empty()),
                        business_area: Some(l.business_area).filter(|s| !s.is_empty()),
                        order_number: Some(l.order_number).filter(|s| !s.is_empty()),
                        tax_code: Some(l.tax_code).filter(|s| !s.is_empty()),
                        tax_amount: Some(l.tax_amount),
                    })
                    .collect(),
                idempotency_key,
            };

            // 执行命令
            match self
                .journal_entry_app_service
                .create_journal_entry(command)
                .await
            {
                Ok(response) => {
                    // 记录审计
                    if let Some(ref audit_mw) = self.audit {
                        let _ = audit_mw
                            .save_audit_record(
                                &tenant_id,
                                &user_id,
                                AuditAction::Create,
                                "journal_entry",
                                &response.id,
                                AuditStatus::Success,
                                None,
                            )
                            .await;
                    }

                    info!(%tenant_id, document_number = %response.document_number, "Journal entry created");

                    // 返回 proto 响应
                    let proto_response = super::super::super::CreateJournalEntryResponse {
                        id: response.id,
                        document_number: response.document_number,
                        status: response.status,
                        created_at: Some(prost_types::Timestamp {
                            seconds: response.created_at.timestamp(),
                            nanos: 0,
                        }),
                    };

                    Ok(Response::new(proto_response))
                }
                Err(e) => {
                    error!(%tenant_id, "Failed to create journal entry: {:?}", e);

                    let api_err = map_application_error(&e, Uuid::new_v4());
                    self.metrics.record_error("journal_entry", "create", api_err.code.as_str());

                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }

    /// 过账会计凭证
    async fn post_journal_entry(
        &self,
        request: Request<super::super::super::PostJournalEntryRequest>,
    ) -> Result<Response<super::super::super::PostJournalEntryResponse>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("post_journal_entry", &auth);

        self.metrics.inc_active_requests();
        let start_time = std::time::Instant::now();

        let tenant_id = auth.tenant_id;
        let user_id = auth.user_id;

        async move {
            let req = request.into_inner();

            // 幂等性检查
            if let Some(ref key) = req.idempotency_key {
                if let Some(ref middleware) = self.idempotency {
                    match middleware.try_get_cached_response(key).await {
                        Ok(Some(response)) => {
                            self.metrics.record_idempotency_hit();
                            return Ok(Response::new(response));
                        }
                        Ok(None) => {}
                        Err(e) => debug!(%key, "Idempotency check error: {}", e),
                    }
                }
            }

            // 检查权限
            if !auth.has_role("finance:post") && !auth.has_any_role(&["finance:admin", "accountant"]) {
                return Err(Status::permission_denied("Missing finance:post permission"));
            }

            let posting_date = req
                .posting_date
                .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0).unwrap())
                .ok_or(Status::invalid_argument("posting_date is required"))?;

            let command = PostJournalEntryCommand {
                tenant_id: tenant_id.to_string(),
                user_id: user_id.to_string(),
                id: req.id,
                posting_date,
                idempotency_key: req.idempotency_key,
            };

            match self
                .journal_entry_app_service
                .post_journal_entry(command)
                .await
            {
                Ok(response) => {
                    if let Some(ref audit_mw) = self.audit {
                        let _ = audit_mw
                            .save_audit_record(
                                &tenant_id,
                                &user_id,
                                AuditAction::Post,
                                "journal_entry",
                                &response.id,
                                AuditStatus::Success,
                                None,
                            )
                            .await;
                    }

                    info!(%tenant_id, document_number = %response.document_number, "Journal entry posted");

                    let proto_response = super::super::super::PostJournalEntryResponse {
                        id: response.id,
                        document_number: response.document_number,
                        status: response.status,
                        posting_date: Some(prost_types::Timestamp {
                            seconds: response.posting_date.timestamp(),
                            nanos: 0,
                        }),
                        total_debit: response.total_debit,
                        total_credit: response.total_credit,
                        posted_at: Some(prost_types::Timestamp {
                            seconds: response.posted_at.timestamp(),
                            nanos: 0,
                        }),
                    };

                    Ok(Response::new(proto_response))
                }
                Err(e) => {
                    error!(%tenant_id, "Failed to post journal entry: {:?}", e);

                    let api_err = map_application_error(&e, Uuid::new_v4());
                    self.metrics.record_error("journal_entry", "post", api_err.code.as_str());

                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }

    /// 冲销会计凭证
    async fn reverse_journal_entry(
        &self,
        request: Request<super::super::super::ReverseJournalEntryRequest>,
    ) -> Result<Response<super::super::super::ReverseJournalEntryResponse>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("reverse_journal_entry", &auth);

        self.metrics.inc_active_requests();
        let tenant_id = auth.tenant_id;
        let user_id = auth.user_id;

        async move {
            let req = request.into_inner();

            // 检查权限
            if !auth.has_role("finance:reverse") && !auth.has_any_role(&["finance:admin", "accountant"]) {
                return Err(Status::permission_denied("Missing finance:reverse permission"));
            }

            let reversal_date = req
                .reversal_date
                .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0).unwrap())
                .ok_or(Status::invalid_argument("reversal_date is required"))?;

            let command = ReverseJournalEntryCommand {
                tenant_id: tenant_id.to_string(),
                user_id: user_id.to_string(),
                original_id: req.id,
                reversal_date,
                reversal_reason: req.reversal_reason,
                reference_document: Some(req.reference_document).filter(|s| !s.is_empty()),
                idempotency_key: req.idempotency_key,
            };

            match self
                .journal_entry_app_service
                .reverse_journal_entry(command)
                .await
            {
                Ok(response) => {
                    if let Some(ref audit_mw) = self.audit {
                        let _ = audit_mw
                            .save_audit_record(
                                &tenant_id,
                                &user_id,
                                AuditAction::Reverse,
                                "journal_entry",
                                &response.original_id,
                                AuditStatus::Success,
                                None,
                            )
                            .await;
                    }

                    info!(%tenant_id, original = %response.original_document_number, reversal = %response.reversal_document_number, "Journal entry reversed");

                    let proto_response = super::super::super::ReverseJournalEntryResponse {
                        original_id: response.original_id,
                        original_document_number: response.original_document_number,
                        reversal_id: response.reversal_id,
                        reversal_document_number: response.reversal_document_number,
                        reversal_date: Some(prost_types::Timestamp {
                            seconds: response.reversal_date.timestamp(),
                            nanos: 0,
                        }),
                        status: response.status,
                        amount: response.amount,
                    };

                    Ok(Response::new(proto_response))
                }
                Err(e) => {
                    error!(%tenant_id, "Failed to reverse journal entry: {:?}", e);

                    let api_err = map_application_error(&e, Uuid::new_v4());
                    self.metrics.record_error("journal_entry", "reverse", api_err.code.as_str());

                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }

    /// 获取会计凭证详情
    async fn get_journal_entry(
        &self,
        request: Request<super::super::super::GetJournalEntryRequest>,
    ) -> Result<Response<super::super::super::JournalEntry>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("get_journal_entry", &auth);

        self.metrics.inc_active_requests();
        let tenant_id = auth.tenant_id;

        async move {
            let req = request.into_inner();

            // 构建查询
            let query = GetJournalEntryQuery {
                tenant_id: tenant_id.to_string(),
                id: if req.id.is_empty() { None } else { Some(req.id) },
                document_number: if req.document_number.is_empty() { None } else { Some(req.document_number) },
                company_code: if req.company_code.is_empty() { None } else { Some(req.company_code) },
                fiscal_year: if req.fiscal_year == 0 { None } else { Some(req.fiscal_year) },
            };

            match self
                .journal_entry_app_service
                .get_journal_entry(query)
                .await
            {
                Ok(response) => {
                    // 转换为 proto
                    let proto_entry = super::super::super::JournalEntry {
                        id: response.id,
                        company_code: response.company_code,
                        fiscal_year: response.fiscal_year,
                        document_number: response.document_number,
                        posting_date: Some(prost_types::Timestamp {
                            seconds: response.posting_date.timestamp(),
                            nanos: 0,
                        }),
                        document_date: Some(prost_types::Timestamp {
                            seconds: response.document_date.timestamp(),
                            nanos: 0,
                        }),
                        currency_code: response.currency_code,
                        status: response.status,
                        header_text: response.header_text.unwrap_or_default(),
                        reference_document: response.reference_document.unwrap_or_default(),
                        total_debit: response.total_debit,
                        total_credit: response.total_credit,
                        line_items: response
                            .line_items
                            .into_iter()
                            .map(|l| super::super::super::JournalEntryLine {
                                line_number: l.line_number as i32,
                                account_code: l.account_code,
                                amount: l.amount,
                                debit_credit: l.debit_credit,
                                cost_center: l.cost_center.unwrap_or_default(),
                                profit_center: l.profit_center.unwrap_or_default(),
                                text: l.text.unwrap_or_default(),
                                functional_area: l.functional_area.unwrap_or_default(),
                                business_area: l.business_area.unwrap_or_default(),
                                order_number: l.order_number.unwrap_or_default(),
                                tax_code: l.tax_code.unwrap_or_default(),
                                tax_amount: l.tax_amount.unwrap_or(),
                            })
                            .collect(),
                        version: response.version as i32,
                        created_at: Some(prost_types::Timestamp {
                            seconds: response.created_at.timestamp(),
                            nanos: 0,
                        }),
                        updated_at: Some(prost_types::Timestamp {
                            seconds: response.updated_at.timestamp(),
                            nanos: 0,
                        }),
                        posted_at: response
                            .posted_at
                            .map(|t| prost_types::Timestamp {
                                seconds: t.timestamp(),
                                nanos: 0,
                            }),
                        extensions: "{}".to_string(),
                    };

                    Ok(Response::new(proto_entry))
                }
                Err(ApplicationError::NotFound { .. }) => Err(Status::not_found("Journal entry not found")),
                Err(e) => {
                    let api_err = map_application_error(&e, Uuid::new_v4());
                    self.metrics.record_error("journal_entry", "get", api_err.code.as_str());
                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }

    /// 列出会计凭证
    async fn list_journal_entries(
        &self,
        request: Request<super::super::super::ListJournalEntriesRequest>,
    ) -> Result<Response<tonic::Streaming<super::super::super::JournalEntrySummary>>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("list_journal_entries", &auth);

        self.metrics.inc_active_requests();
        let tenant_id = auth.tenant_id;

        async move {
            let req = request.into_inner();

            // 构建查询
            let query = ListJournalEntriesQuery {
                tenant_id: tenant_id.to_string(),
                page_size: if req.page_size == 0 { 50 } else { req.page_size.min(100) } as usize,
                page_token: if req.page_token.is_empty() { None } else { Some(req.page_token) },
                company_code: if req.company_code.is_empty() { None } else { Some(req.company_code) },
                fiscal_year: if req.fiscal_year == 0 { None } else { Some(req.fiscal_year) },
                filter_date_from: req
                    .filter_date_from
                    .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0))
                    .flatten(),
                filter_date_to: req
                    .filter_date_to
                    .map(|t| NaiveDate::from_timestamp_opt(t.seconds, 0))
                    .flatten(),
                account_code: if req.account_code.is_empty() { None } else { Some(req.account_code) },
                cost_center: if req.cost_center.is_empty() { None } else { Some(req.cost_center) },
                status: if req.status.is_empty() { None } else { Some(req.status) },
                sort_by: if req.sort_by.is_empty() { "created_at".to_string() } else { req.sort_by },
                sort_order: if req.sort_order.is_empty() { "desc".to_string() } else { req.sort_order },
            };

            let mut stream = self
                .journal_entry_app_service
                .list_journal_entries(query)
                .await?
                .map_ok(|entry| {
                    super::super::super::JournalEntrySummary {
                        id: entry.id,
                        document_number: entry.document_number,
                        fiscal_year: entry.fiscal_year as i32,
                        company_code: entry.company_code,
                        posting_date: Some(prost_types::Timestamp {
                            seconds: entry.posting_date.timestamp(),
                            nanos: 0,
                        }),
                        document_date: Some(prost_types::Timestamp {
                            seconds: entry.document_date.timestamp(),
                            nanos: 0,
                        }),
                        currency_code: entry.currency_code,
                        status: entry.status,
                        header_text: entry.header_text.unwrap_or_default(),
                        total_amount: entry.total_amount,
                        line_count: entry.line_count as i32,
                        created_at: Some(prost_types::Timestamp {
                            seconds: entry.created_at.timestamp(),
                            nanos: 0,
                        }),
                    }
                })
                .map_err(|e| {
                    let api_err = map_application_error(&e, Uuid::new_v4());
                    Status::from(api_err)
                });

            let output = async_stream::stream! {
                while let Some(result) = stream.next().await {
                    yield result;
                }
            };

            Ok(Response::new(Box::pin(output) as _))
        }
        .instrument(span)
        .await
    }

    /// 获取科目余额
    async fn get_account_balance(
        &self,
        request: Request<super::super::super::GetAccountBalanceRequest>,
    ) -> Result<Response<super::super::super::AccountBalance>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("get_account_balance", &auth);

        self.metrics.inc_active_requests();
        let tenant_id = auth.tenant_id;

        async move {
            let req = request.into_inner();

            let query = AccountBalanceQuery {
                tenant_id: tenant_id.to_string(),
                company_code: req.company_code,
                fiscal_year: req.fiscal_year,
                period: if req.period == 0 { None } else { Some(req.period) },
                account_code: req.account_code,
                cost_center: if req.cost_center.is_empty() { None } else { Some(req.cost_center) },
            };

            match self.reporting_service.get_account_balance(query).await {
                Ok(response) => {
                    let proto_balance = super::super::super::AccountBalance {
                        company_code: response.company_code,
                        fiscal_year: response.fiscal_year as i32,
                        account_code: response.account_code,
                        account_name: response.account_name.unwrap_or_default(),
                        cost_center: response.cost_center.unwrap_or_default(),
                        opening_balance: response.opening_balance,
                        period_debit: response.period_debit,
                        period_credit: response.period_credit,
                        closing_balance: response.closing_balance,
                        balance_direction: response.balance_direction,
                    };
                    Ok(Response::new(proto_balance))
                }
                Err(e) => {
                    let api_err = map_application_error(&e, Uuid::new_v4());
                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }

    /// 获取试算平衡表
    async fn get_trial_balance(
        &self,
        request: Request<super::super::super::GetTrialBalanceRequest>,
    ) -> Result<Response<super::super::super::TrialBalance>, Status> {
        let auth = self.auth_context(&request)?;
        let span = self.create_span("get_trial_balance", &auth);

        self.metrics.inc_active_requests();
        let tenant_id = auth.tenant_id;

        async move {
            let req = request.into_inner();

            let query = TrialBalanceQuery {
                tenant_id: tenant_id.to_string(),
                company_code: req.company_code,
                fiscal_year: req.fiscal_year,
                period: req.period,
                include_open_items: req.include_open_items,
            };

            match self.reporting_service.get_trial_balance(query).await {
                Ok(response) => {
                    let proto_trial_balance = super::super::super::TrialBalance {
                        company_code: response.company_code,
                        fiscal_year: response.fiscal_year as i32,
                        period: response.period as i32,
                        balances: response
                            .balances
                            .into_iter()
                            .map(|b| super::super::super::AccountBalance {
                                company_code: b.company_code,
                                fiscal_year: b.fiscal_year as i32,
                                account_code: b.account_code,
                                account_name: b.account_name.unwrap_or_default(),
                                cost_center: b.cost_center.unwrap_or_default(),
                                opening_balance: b.opening_balance,
                                period_debit: b.period_debit,
                                period_credit: b.period_credit,
                                closing_balance: b.closing_balance,
                                balance_direction: b.balance_direction,
                            })
                            .collect(),
                        total_debit: response.total_debit,
                        total_credit: response.total_credit,
                        difference: response.difference,
                    };
                    Ok(Response::new(proto_trial_balance))
                }
                Err(e) => {
                    let api_err = map_application_error(&e, Uuid::new_v4());
                    Err(Status::from(api_err))
                }
            }
        }
        .instrument(span)
        .await
    }
}
