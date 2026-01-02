//! 会计凭证仓储 PostgreSQL 实现
//!
//! 实现 domain::JournalEntryRepository 接口
//! 使用 sqlx 进行数据库操作

use crate::domain::aggregates::JournalEntry;
use crate::domain::error::DomainError;
use crate::domain::repositories::JournalEntryRepository;
use crate::domain::value_objects::{CompanyCode, DebitCredit, FiscalPeriodId, JournalEntryId};
use crate::infrastructure::persistence::{
    execute_with_metrics, soft_delete, DbMetrics, JournalEntryDb, JournalEntryLineDb,
};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use killer_domain_primitives::{AccountCode, CurrencyCode};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use std::sync::Arc;
use tracing::{debug, error, info, span, Level};
use uuid::Uuid;

// =============================================================================
// PostgreSQL 仓储实现
// =============================================================================

/// PostgreSQL 会计凭证仓储
#[derive(Clone)]
pub struct PgJournalEntryRepository {
    /// 数据库连接池
    pool: Arc<PgPool>,
    /// 指标
    metrics: Arc<DbMetrics>,
    /// 表名
    table_name: &'static str,
    /// 行项目表名
    lines_table_name: &'static str,
}

impl PgJournalEntryRepository {
    /// 创建新的仓储实例
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            metrics: Arc::new(DbMetrics::new().unwrap_or_default()),
            table_name: "journal_entries",
            lines_table_name: "journal_entry_lines",
        }
    }

    /// 将数据库行转换为聚合根
    async fn aggregate_from_db(
        &self,
        entry: JournalEntryDb,
        lines: Vec<JournalEntryLineDb>,
    ) -> Result<JournalEntry, DomainError> {
        // TODO: 这里需要根据数据库模型重建聚合根
        // 由于领域聚合根的构建涉及复杂的业务逻辑，
        // 这里需要从应用层传入的工厂函数或使用事件溯源
        // 暂时返回错误，实际实现需要结合事件溯源或从数据库完整读取

        Err(DomainError::InfrastructureError(
            "Reconstruction of aggregate from DB requires event sourcing or complete snapshot".to_string(),
        ))
    }

    /// 保存行项目（批量）
    async fn save_lines(
        &self,
        tenant_id: Uuid,
        entry_id: Uuid,
        lines: &[JournalEntryLineDb],
    ) -> Result<(), sqlx::Error> {
        if lines.is_empty() {
            return Ok(());
        }

        let mut query = format!(
            "INSERT INTO {} (id, tenant_id, entry_id, line_number, account_code, amount, debit_credit, cost_center, profit_center, text, functional_area, business_area, order_number) VALUES ",
            self.lines_table_name
        );

        let mut placeholders = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let offset = i * 14;
            placeholders.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${})",
                1 + offset,
                2 + offset,
                3 + offset,
                4 + offset,
                5 + offset,
                6 + offset,
                7 + offset,
                8 + offset,
                9 + offset,
                10 + offset,
                11 + offset,
                12 + offset,
                13 + offset,
            ));
        }

        query.push_str(&placeholders.join(", "));
        query.push_str(" ON CONFLICT (id) DO NOTHING");

        let mut q = sqlx::query(&query);

        for line in lines {
            q = q
                .bind(Uuid::new_v4())
                .bind(tenant_id)
                .bind(entry_id)
                .bind(line.line_number as i32)
                .bind(&line.account_code)
                .bind(&line.amount)
                .bind(&line.debit_credit)
                .bind(&line.cost_center)
                .bind(&line.profit_center)
                .bind(&line.text)
                .bind(&line.functional_area)
                .bind(&line.business_area)
                .bind(&line.order_number);
        }

        q.execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 查询行项目
    async fn find_lines(&self, entry_id: Uuid) -> Result<Vec<JournalEntryLineDb>, sqlx::Error> {
        let query = format!(
            "SELECT line_number, account_code, amount, debit_credit, cost_center, profit_center, text, functional_area, business_area, order_number
             FROM {} WHERE entry_id = $1 ORDER BY line_number",
            self.lines_table_name
        );

        sqlx::query_as(&query)
            .bind(entry_id)
            .fetch_all(self.pool.as_ref())
            .await
    }
}

#[async_trait]
impl JournalEntryRepository for PgJournalEntryRepository {
    /// 保存会计凭证
    async fn save(&self, entry: &JournalEntry) -> Result<(), DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.save", document_number = %entry.document_number());
        let _guard = span.enter();

        let tenant_id = entry.tenant_id();
        let id = entry.id().clone();
        let version = entry.version();

        debug!(%tenant_id, %id, version, "Saving journal entry");

        // TODO: 实现完整的保存逻辑
        // 需要将聚合根转换为数据库模型并保存
        // 由于聚合根的内部结构复杂，这里需要事件溯源或快照

        Err(DomainError::InfrastructureError(
            "Full save requires event sourcing implementation".to_string(),
        ))
    }

    /// 根据 ID 查找会计凭证
    async fn find_by_id(&self, id: &JournalEntryId) -> Result<Option<JournalEntry>, DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.find_by_id", %id);
        let _guard = span.enter();

        let start = std::time::Instant::now();

        let query = format!(
            "SELECT * FROM {} WHERE id = $1 AND deleted_at IS NULL",
            self.table_name
        );

        let result: Result<Option<JournalEntryDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_by_id",
            self.table_name,
            |pool| async move {
                let row = sqlx::query(&query)
                    .bind(id.to_uuid())
                    .fetch_optional(pool)
                    .await?;

                match row {
                    Some(row) => {
                        let entry: JournalEntryDb = JournalEntryDb::try_from_row(&row)?;
                        Ok(Some(entry))
                    }
                    None => Ok(None),
                }
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_by_id", self.table_name, result.is_ok(), duration);

        match result {
            Ok(Some(entry)) => {
                let lines = self.find_lines(entry.id).await.map_err(|e| {
                    DomainError::InfrastructureError(format!("Failed to fetch lines: {:?}", e))
                })?;

                self.aggregate_from_db(entry, lines).await.map(Some)
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 根据租户和公司代码查询凭证列表
    async fn find_by_company(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<JournalEntry>, DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.find_by_company");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT * FROM {} WHERE tenant_id = $1 AND company_code = $2 AND fiscal_year = $3 AND deleted_at IS NULL ORDER BY created_at DESC LIMIT $4 OFFSET $5",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<JournalEntryDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_by_company",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .bind(limit as i32)
                    .bind(offset as i32)
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_by_company", self.table_name, result.is_ok(), duration);

        match result {
            Ok(entries) => {
                let mut aggregates = Vec::new();
                for entry in entries {
                    let lines = self.find_lines(entry.id).await.unwrap_or_default();
                    if let Ok(agg) = self.aggregate_from_db(entry, lines).await {
                        aggregates.push(agg);
                    }
                }
                Ok(aggregates)
            }
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 根据过账日期范围查询凭证
    async fn find_by_posting_date_range(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Vec<JournalEntry>, DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.find_by_posting_date_range");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT * FROM {} WHERE tenant_id = $1 AND company_code = $2 AND posting_date >= $3 AND posting_date <= $4 AND deleted_at IS NULL ORDER BY posting_date, document_number",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<JournalEntryDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_by_posting_date_range",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(from_date)
                    .bind(to_date)
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_by_posting_date_range", self.table_name, result.is_ok(), duration);

        match result {
            Ok(entries) => {
                let mut aggregates = Vec::new();
                for entry in entries {
                    let lines = self.find_lines(entry.id).await.unwrap_or_default();
                    if let Ok(agg) = self.aggregate_from_db(entry, lines).await {
                        aggregates.push(agg);
                    }
                }
                Ok(aggregates)
            }
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 根据账户代码查询相关凭证
    async fn find_by_account(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        account_code: &AccountCode,
        fiscal_year: i32,
    ) -> Result<Vec<JournalEntry>, DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.find_by_account");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT DISTINCT e.* FROM {} e
             INNER JOIN {} l ON e.id = l.entry_id
             WHERE e.tenant_id = $1 AND e.company_code = $2 AND e.fiscal_year = $3 AND l.account_code = $4 AND e.deleted_at IS NULL
             ORDER BY e.created_at DESC",
            self.table_name, self.lines_table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<JournalEntryDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_by_account",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .bind(account_code.as_str())
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_by_account", self.table_name, result.is_ok(), duration);

        match result {
            Ok(entries) => {
                let mut aggregates = Vec::new();
                for entry in entries {
                    let lines = self.find_lines(entry.id).await.unwrap_or_default();
                    if let Ok(agg) = self.aggregate_from_db(entry, lines).await {
                        aggregates.push(agg);
                    }
                }
                Ok(aggregates)
            }
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 删除会计凭证（软删除）
    async fn delete(&self, id: &JournalEntryId) -> Result<(), DomainError> {
        let span = span!(Level::DEBUG, "JournalEntryRepository.delete", %id);
        let _guard = span.enter();

        let tenant_id = Uuid::parse_str(id.tenant_id()).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        soft_delete(self.pool.as_ref(), self.table_name, id.to_uuid(), tenant_id)
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("Failed to delete: {:?}", e)))?;

        info!(%id, "Journal entry soft deleted");
        Ok(())
    }

    /// 检查凭证号是否存在
    async fn exists(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        document_number: &str,
    ) -> Result<bool, DomainError> {
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT 1 FROM {} WHERE tenant_id = $1 AND company_code = $2 AND fiscal_year = $3 AND document_number = $4 AND deleted_at IS NULL LIMIT 1",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Option<PgRow>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "exists",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .bind(document_number)
                    .fetch_optional(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("exists", self.table_name, result.is_ok(), duration);

        Ok(result.map(|r| r.is_some()).unwrap_or(false))
    }
}

// =============================================================================
// 辅助实现
// =============================================================================

impl JournalEntryId {
    /// 获取 UUID
    pub fn to_uuid(&self) -> Uuid {
        // JournalEntryId 应该包含足够信息生成唯一 ID
        // 这里使用 tenant_id + company_code + fiscal_year + document_number 的哈希
        let input = format!(
            "{}/{}/{}/{}",
            self.tenant_id(),
            self.company_code().as_str(),
            self.fiscal_year(),
            self.document_number().as_str()
        );
        let hash = md5::compute(input);
        Uuid::from_bytes(hash.0)
    }
}
