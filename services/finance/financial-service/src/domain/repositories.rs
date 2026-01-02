//! 仓储接口
//!
//! 定义聚合根的持久化接口（纯接口，无实现）

use crate::domain::aggregates::{FiscalPeriod, JournalEntry};
use crate::domain::error::DomainResult;
use crate::domain::value_objects::{FiscalPeriodId, JournalEntryId};
use async_trait::async_trait;
use chrono::NaiveDate;
use killer_domain_primitives::{AccountCode, CompanyCode};

// =============================================================================
// 会计凭证仓储
// =============================================================================

/// 会计凭证仓储接口
///
/// 负责会计凭证聚合根的持久化和查询
#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    /// 保存会计凭证
    ///
    /// # 参数
    /// - `entry`: 会计凭证聚合根
    ///
    /// # 返回
    /// - 成功返回 Ok(())
    /// - 如果版本冲突返回 ConcurrencyConflict 错误
    async fn save(&self, entry: &JournalEntry) -> DomainResult<()>;

    /// 根据 ID 查找会计凭证
    ///
    /// # 参数
    /// - `id`: 凭证 ID
    ///
    /// # 返回
    /// - 找到返回 Some(JournalEntry)
    /// - 未找到返回 None
    async fn find_by_id(&self, id: &JournalEntryId) -> DomainResult<Option<JournalEntry>>;

    /// 根据租户和公司代码查询凭证列表
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `limit`: 最大返回数量
    /// - `offset`: 偏移量
    ///
    /// # 返回
    /// - 凭证列表
    async fn find_by_company(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        limit: usize,
        offset: usize,
    ) -> DomainResult<Vec<JournalEntry>>;

    /// 根据过账日期范围查询凭证
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `from_date`: 开始日期
    /// - `to_date`: 结束日期
    ///
    /// # 返回
    /// - 凭证列表
    async fn find_by_posting_date_range(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> DomainResult<Vec<JournalEntry>>;

    /// 根据账户代码查询相关凭证
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `account_code`: 账户代码
    /// - `fiscal_year`: 会计年度
    ///
    /// # 返回
    /// - 包含该账户的凭证列表
    async fn find_by_account(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        account_code: &AccountCode,
        fiscal_year: i32,
    ) -> DomainResult<Vec<JournalEntry>>;

    /// 删除会计凭证（物理删除，仅用于测试）
    ///
    /// # 参数
    /// - `id`: 凭证 ID
    async fn delete(&self, id: &JournalEntryId) -> DomainResult<()>;

    /// 检查凭证号是否存在
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `document_number`: 凭证号
    ///
    /// # 返回
    /// - 存在返回 true，否则返回 false
    async fn exists(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        document_number: &str,
    ) -> DomainResult<bool>;
}

// =============================================================================
// 会计期间仓储
// =============================================================================

/// 会计期间仓储接口
///
/// 负责会计期间聚合根的持久化和查询
#[async_trait]
pub trait FiscalPeriodRepository: Send + Sync {
    /// 保存会计期间
    ///
    /// # 参数
    /// - `period`: 会计期间聚合根
    ///
    /// # 返回
    /// - 成功返回 Ok(())
    /// - 如果版本冲突返回 ConcurrencyConflict 错误
    async fn save(&self, period: &FiscalPeriod) -> DomainResult<()>;

    /// 根据 ID 查找会计期间
    ///
    /// # 参数
    /// - `id`: 期间 ID
    ///
    /// # 返回
    /// - 找到返回 Some(FiscalPeriod)
    /// - 未找到返回 None
    async fn find_by_id(&self, id: &FiscalPeriodId) -> DomainResult<Option<FiscalPeriod>>;

    /// 根据日期查找开放的会计期间
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `date`: 日期
    ///
    /// # 返回
    /// - 找到返回 Some(FiscalPeriod)
    /// - 未找到返回 None
    async fn find_open_period_by_date(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        date: NaiveDate,
    ) -> DomainResult<Option<FiscalPeriod>>;

    /// 查询会计年度的所有期间
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    ///
    /// # 返回
    /// - 期间列表（按期间号排序）
    async fn find_by_fiscal_year(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
    ) -> DomainResult<Vec<FiscalPeriod>>;

    /// 查询后续期间
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `period`: 期间号
    ///
    /// # 返回
    /// - 后续期间列表
    async fn find_subsequent_periods(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        period: u8,
    ) -> DomainResult<Vec<FiscalPeriod>>;

    /// 删除会计期间（物理删除，仅用于测试）
    ///
    /// # 参数
    /// - `id`: 期间 ID
    async fn delete(&self, id: &FiscalPeriodId) -> DomainResult<()>;

    /// 检查期间是否存在
    ///
    /// # 参数
    /// - `tenant_id`: 租户 ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `period`: 期间号
    ///
    /// # 返回
    /// - 存在返回 true，否则返回 false
    async fn exists(
        &self,
        tenant_id: &str,
        company_code: &CompanyCode,
        fiscal_year: i32,
        period: u8,
    ) -> DomainResult<bool>;
}
