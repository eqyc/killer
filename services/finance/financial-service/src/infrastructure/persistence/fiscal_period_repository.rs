//! 会计期间仓储 PostgreSQL 实现
//!
//! 实现 domain::FiscalPeriodRepository 接口

use crate::domain::aggregates::FiscalPeriod;
use crate::domain::error::DomainError;
use crate::domain::repositories::FiscalPeriodRepository;
use crate::domain::value_objects::{CompanyCode, FiscalPeriodId, PeriodStatus, ValidityRange};
use crate::infrastructure::persistence::{execute_with_metrics, soft_delete, DbMetrics, FiscalPeriodDb};
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::sync::Arc;
use tracing::{debug, error, span, Level};
use uuid::Uuid;

// =============================================================================
// PostgreSQL 仓储实现
// =============================================================================

/// PostgreSQL 会计期间仓储
#[derive(Clone)]
pub struct PgFiscalPeriodRepository {
    /// 数据库连接池
    pool: Arc<PgPool>,
    /// 指标
    metrics: Arc<DbMetrics>,
    /// 表名
    table_name: &'static str,
}

impl PgFiscalPeriodRepository {
    /// 创建新的仓储实例
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            metrics: Arc::new(DbMetrics::new().unwrap_or_default()),
            table_name: "fiscal_periods",
        }
    }

    /// 将数据库行转换为聚合根
    async fn aggregate_from_db(&self, period: FiscalPeriodDb) -> Result<FiscalPeriod, DomainError> {
        // TODO: 从数据库模型重建聚合根
        // 由于 FiscalPeriod 聚合根的构建涉及复杂的业务逻辑，
        // 这里需要事件溯源或快照
        Err(DomainError::InfrastructureError(
            "Reconstruction of FiscalPeriod aggregate from DB requires event sourcing".to_string(),
        ))
    }
}

#[async_trait]
impl FiscalPeriodRepository for PgFiscalPeriodRepository {
    /// 保存会计期间
    async fn save(&self, period: &FiscalPeriod) -> Result<(), DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.save", period = %period.id());
        let _guard = span.enter();

        let tenant_id = period.tenant_id();
        let id = period.id().clone();
        let version = period.version();

        debug!(%tenant_id, %id, version, "Saving fiscal period");

        // TODO: 实现完整的保存逻辑
        // 需要将聚合根转换为数据库模型并保存

        Err(DomainError::InfrastructureError(
            "Full save requires event sourcing implementation".to_string(),
        ))
    }

    /// 根据 ID 查找会计期间
    async fn find_by_id(&self, id: &FiscalPeriodId) -> Result<Option<FiscalPeriod>, DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.find_by_id", %id);
        let _guard = span.enter();

        let query = format!(
            "SELECT * FROM {} WHERE id = $1 AND deleted_at IS NULL",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Option<FiscalPeriodDb>, sqlx::Error> = execute_with_metrics(
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
                        let period: FiscalPeriodDb = FiscalPeriodDb::try_from_row(&row)?;
                        Ok(Some(period))
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
            Ok(Some(period)) => self.aggregate_from_db(period).await.map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 根据日期查找开放的会计期间
    async fn find_open_period_by_date(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        date: NaiveDate,
    ) -> Result<Option<FiscalPeriod>, DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.find_open_period_by_date");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT * FROM {} WHERE tenant_id = $1 AND company_code = $2 AND status = 'OPEN' AND valid_from <= $3 AND valid_to >= $3 AND deleted_at IS NULL LIMIT 1",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Option<FiscalPeriodDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_open_period_by_date",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(date)
                    .fetch_optional(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_open_period_by_date", self.table_name, result.is_ok(), duration);

        match result {
            Ok(Some(period)) => self.aggregate_from_db(period).await.map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(DomainError::InfrastructureError(format!(
                "Database error: {:?}",
                e
            ))),
        }
    }

    /// 查询会计年度的所有期间
    async fn find_by_fiscal_year(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
    ) -> Result<Vec<FiscalPeriod>, DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.find_by_fiscal_year");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT * FROM {} WHERE tenant_id = $1 AND company_code = $2 AND fiscal_year = $3 AND deleted_at IS NULL ORDER BY period",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<FiscalPeriodDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_by_fiscal_year",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_by_fiscal_year", self.table_name, result.is_ok(), duration);

        match result {
            Ok(periods) => {
                let mut aggregates = Vec::new();
                for period in periods {
                    if let Ok(agg) = self.aggregate_from_db(period).await {
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

    /// 查询后续期间
    async fn find_subsequent_periods(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        period: u8,
    ) -> Result<Vec<FiscalPeriod>, DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.find_subsequent_periods");
        let _guard = span.enter();

        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT * FROM {} WHERE tenant_id = $1 AND company_code = $2 AND fiscal_year = $3 AND period > $4 AND deleted_at IS NULL ORDER BY period",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Vec<FiscalPeriodDb>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "find_subsequent_periods",
            self.table_name,
            |pool| async move {
                sqlx::query_as(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .bind(period as i32)
                    .fetch_all(pool)
                    .await
            },
        )
        .await;

        let duration = start.elapsed();
        self.metrics
            .record_query("find_subsequent_periods", self.table_name, result.is_ok(), duration);

        match result {
            Ok(periods) => {
                let mut aggregates = Vec::new();
                for period in periods {
                    if let Ok(agg) = self.aggregate_from_db(period).await {
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

    /// 删除会计期间（软删除）
    async fn delete(&self, id: &FiscalPeriodId) -> Result<(), DomainError> {
        let span = span!(Level::DEBUG, "FiscalPeriodRepository.delete", %id);
        let _guard = span.enter();

        let tenant_id = Uuid::parse_str(id.tenant_id()).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        soft_delete(self.pool.as_ref(), self.table_name, id.to_uuid(), tenant_id)
            .await
            .map_err(|e| DomainError::InfrastructureError(format!("Failed to delete: {:?}", e)))?;

        info!(%id, "Fiscal period soft deleted");
        Ok(())
    }

    /// 检查期间是否存在
    async fn exists(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        period: u8,
    ) -> Result<bool, DomainError> {
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|_| {
            DomainError::InfrastructureError("Invalid tenant_id format".to_string())
        })?;

        let query = format!(
            "SELECT 1 FROM {} WHERE tenant_id = $1 AND company_code = $2 AND fiscal_year = $3 AND period = $4 AND deleted_at IS NULL LIMIT 1",
            self.table_name
        );

        let start = std::time::Instant::now();

        let result: Result<Option<sqlx::postgres::PgRow>, sqlx::Error> = execute_with_metrics(
            self.pool.as_ref(),
            self.metrics.as_ref(),
            "exists",
            self.table_name,
            |pool| async move {
                sqlx::query(&query)
                    .bind(tenant_uuid)
                    .bind(company_code.as_str())
                    .bind(fiscal_year)
                    .bind(period as i32)
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

impl FiscalPeriodId {
    /// 获取 UUID
    pub fn to_uuid(&self) -> Uuid {
        let input = format!(
            "{}/{}/{}/{}",
            self.tenant_id(),
            self.company_code().as_str(),
            self.fiscal_year(),
            self.period()
        );
        let hash = md5::compute(input);
        Uuid::from_bytes(hash.0)
    }
}
