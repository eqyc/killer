//! 凭证应用服务
//!
//! 提供凭证相关的高级业务服务
//! 协调多个命令和查询操作

use crate::application::commands::*;
use crate::application::dto::*;
use crate::application::error::ApplicationError;
use crate::application::queries::*;
use crate::application::repositories::*;
use killer_cqrs::prelude::*;
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

// =============================================================================
// 应用服务
// =============================================================================

/// 凭证应用服务
#[derive(Clone)]
pub struct JournalEntryApplicationService<CMD, QRY>
where
    CMD: CommandBus,
    QRY: QueryBus,
{
    /// 命令总线
    command_bus: Arc<CMD>,
    /// 查询总线
    query_bus: Arc<QRY>,
}

impl<CMD, QRY> JournalEntryApplicationService<CMD, QRY>
where
    CMD: CommandBus + Send + Sync,
    QRY: QueryBus + Send + Sync,
{
    pub fn new(command_bus: Arc<CMD>, query_bus: Arc<QRY>) -> Self {
        Self {
            command_bus,
            query_bus,
        }
    }

    /// 创建并过账凭证（单事务）
    pub async fn create_and_post(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        request: CreateJournalEntryRequest,
    ) -> ServiceResult<PostJournalEntryResponse> {
        let start_time = std::time::Instant::now();

        // 1. 创建凭证
        let create_command = CreateJournalEntryCommand::new(tenant_id, user_id, request.clone());
        let create_response = self
            .command_bus
            .execute(create_command)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to create journal entry: {:?}", e)))?;

        // 2. 过账凭证
        let post_request = PostJournalEntryRequest {
            company_code: request.company_code,
            fiscal_year: request.fiscal_year,
            document_number: create_response.document_number.clone(),
            posting_date: request.posting_date,
        };
        let post_command = PostJournalEntryCommand::new(tenant_id, user_id, post_request);
        let post_response = self
            .command_bus
            .execute(post_command)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to post journal entry: {:?}", e)))?;

        info!(%tenant_id, document_number = %post_response.document_number, duration_ms = %start_time.elapsed().as_millis(), "Journal entry created and posted successfully");

        Ok(post_response)
    }

    /// 批量创建凭证
    pub async fn batch_create(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        requests: Vec<CreateJournalEntryRequest>,
    ) -> ServiceResult<Vec<CreateJournalEntryResponse>> {
        let mut responses = Vec::new();

        for request in requests {
            let command = CreateJournalEntryCommand::new(tenant_id, user_id, request);
            let response = self
                .command_bus
                .execute(command)
                .await
                .map_err(|e| ApplicationError::from(std::format!("Failed to create journal entry: {:?}", e)))?;
            responses.push(response);
        }

        Ok(responses)
    }

    /// 获取凭证详情（带缓存）
    pub async fn get_entry_detail(
        &self,
        tenant_id: Uuid,
        request: GetJournalEntryRequest,
    ) -> ServiceResult<JournalEntryDetail> {
        let query = GetJournalEntryQuery::new(tenant_id, request);
        self.query_bus
            .execute(query)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to get journal entry: {:?}", e)))
    }

    /// 搜索凭证
    pub async fn search_entries(
        &self,
        tenant_id: Uuid,
        request: ListJournalEntriesRequest,
    ) -> ServiceResult<PagedResult<JournalEntrySummary>> {
        let query = ListJournalEntriesQuery::new(tenant_id, request);
        self.query_bus
            .execute(query)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to search journal entries: {:?}", e)))
    }

    /// 冲销凭证（带验证）
    pub async fn reverse_with_validation(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        request: ReverseJournalEntryRequest,
    ) -> ServiceResult<ReverseJournalEntryResponse> {
        // 1. 获取原凭证详情
        let get_request = GetJournalEntryRequest {
            company_code: request.company_code.clone(),
            fiscal_year: request.fiscal_year,
            document_number: request.original_document_number.clone(),
        };

        let original_entry = self.get_entry_detail(tenant_id, get_request).await?;

        // 2. 验证是否可以冲销
        if original_entry.status != "POSTED" {
            return Err(ApplicationError::business_rule_violation(
                "NOT_POSTED",
                "Can only reverse posted entries".to_string(),
            ));
        }

        // 3. 执行冲销
        let command = ReverseJournalEntryCommand::new(tenant_id, user_id, request);
        self.command_bus
            .execute(command)
            .await
            .map_err(|e| ApplicationError::from(std::format!("Failed to reverse journal entry: {:?}", e)))
    }
}
