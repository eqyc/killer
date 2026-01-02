//! 会计期间聚合根
//!
//! 管理会计期间的开放、关闭和重新开放

use crate::domain::error::{DomainError, DomainResult};
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{FiscalPeriodId, PeriodStatus, ValidityRange};
use chrono::NaiveDate;
use killer_domain_primitives::CompanyCode;
use serde::{Deserialize, Serialize};

// =============================================================================
// 会计期间聚合根
// =============================================================================

/// 会计期间聚合根
///
/// 管理会计期间的生命周期，控制凭证过账的时间窗口
///
/// # 不变式
/// - 期间号必须在 1-16 之间（SAP 标准：12 个常规期间 + 4 个特殊期间）
/// - 有效期范围必须合理（开始日期 < 结束日期）
/// - 只有开放状态的期间允许过账
/// - 关闭期间前必须确保所有凭证已过账
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalPeriod {
    /// 租户ID
    tenant_id: String,

    /// 公司代码
    company_code: CompanyCode,

    /// 会计年度
    fiscal_year: i32,

    /// 期间号 (1-16)
    /// 1-12: 常规期间
    /// 13-16: 特殊期间（用于年末调整）
    period: u8,

    /// 期间状态
    status: PeriodStatus,

    /// 有效期范围
    validity_range: ValidityRange,

    /// 期间描述
    description: Option<String>,

    /// 版本号（乐观锁）
    version: u64,
}

impl FiscalPeriod {
    /// 创建新的会计期间（默认开放状态）
    ///
    /// # 参数
    /// - `tenant_id`: 租户ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `period`: 期间号 (1-16)
    /// - `valid_from`: 开始日期
    /// - `valid_to`: 结束日期
    ///
    /// # 不变式验证
    /// - 期间号在 1-16 范围内
    /// - 开始日期 < 结束日期
    pub fn create(
        tenant_id: impl Into<String>,
        company_code: CompanyCode,
        fiscal_year: i32,
        period: u8,
        valid_from: NaiveDate,
        valid_to: NaiveDate,
    ) -> DomainResult<Self> {
        // 验证期间号
        if !(1..=16).contains(&period) {
            return Err(DomainError::ValidationError {
                message: format!("期间号必须在 1-16 之间，实际: {}", period),
            });
        }

        // 验证日期范围
        if valid_from >= valid_to {
            return Err(DomainError::ValidationError {
                message: format!(
                    "开始日期 {} 必须早于结束日期 {}",
                    valid_from, valid_to
                ),
            });
        }

        Ok(Self {
            tenant_id: tenant_id.into(),
            company_code,
            fiscal_year,
            period,
            status: PeriodStatus::Open,
            validity_range: ValidityRange::new(valid_from, valid_to),
            description: None,
            version: 1,
        })
    }

    /// 开放期间
    ///
    /// # 业务规则
    /// - 只有已关闭的期间可以重新开放
    ///
    /// # 返回
    /// - 成功返回 FiscalPeriodOpened 事件
    pub fn open(&mut self) -> DomainResult<Vec<DomainEvent>> {
        if !self.status.can_reopen() {
            return Err(DomainError::InvalidPeriodStatus {
                current_status: format!("{}", self.status),
                operation: "开放".to_string(),
            });
        }

        self.status = PeriodStatus::Open;
        self.version += 1;

        let event = DomainEvent::FiscalPeriodOpened {
            tenant_id: self.tenant_id.clone(),
            company_code: self.company_code.clone(),
            fiscal_year: self.fiscal_year,
            period: self.period,
        };

        Ok(vec![event])
    }

    /// 关闭期间
    ///
    /// # 业务规则
    /// - 只有开放或结账中的期间可以关闭
    ///
    /// # 返回
    /// - 成功返回 FiscalPeriodClosed 事件
    pub fn close(&mut self) -> DomainResult<Vec<DomainEvent>> {
        if !self.status.can_close() {
            return Err(DomainError::InvalidPeriodStatus {
                current_status: format!("{}", self.status),
                operation: "关闭".to_string(),
            });
        }

        self.status = PeriodStatus::Closed;
        self.version += 1;

        let event = DomainEvent::FiscalPeriodClosed {
            tenant_id: self.tenant_id.clone(),
            company_code: self.company_code.clone(),
            fiscal_year: self.fiscal_year,
            period: self.period,
        };

        Ok(vec![event])
    }

    /// 设置为结账中状态
    ///
    /// # 业务规则
    /// - 只有开放状态可以进入结账中
    pub fn start_closing(&mut self) -> DomainResult<()> {
        if self.status != PeriodStatus::Open {
            return Err(DomainError::InvalidPeriodStatus {
                current_status: format!("{}", self.status),
                operation: "开始结账".to_string(),
            });
        }

        self.status = PeriodStatus::Closing;
        self.version += 1;

        Ok(())
    }

    /// 设置期间描述
    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
        self.version += 1;
    }

    /// 检查日期是否在期间内
    pub fn is_open_at(&self, date: NaiveDate) -> bool {
        self.status.allows_posting() && self.validity_range.contains(date)
    }

    /// 检查期间是否允许过账
    pub fn allows_posting(&self) -> bool {
        self.status.allows_posting()
    }

    /// 检查是否为特殊期间（13-16）
    pub fn is_special_period(&self) -> bool {
        self.period > 12
    }

    // Getters

    /// 获取聚合根 ID
    pub fn id(&self) -> FiscalPeriodId {
        FiscalPeriodId::new(
            self.tenant_id.clone(),
            self.company_code.as_str(),
            self.fiscal_year,
            self.period,
        )
    }

    /// 获取租户ID
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    /// 获取公司代码
    pub fn company_code(&self) -> &CompanyCode {
        &self.company_code
    }

    /// 获取会计年度
    pub fn fiscal_year(&self) -> i32 {
        self.fiscal_year
    }

    /// 获取期间号
    pub fn period(&self) -> u8 {
        self.period
    }

    /// 获取状态
    pub fn status(&self) -> PeriodStatus {
        self.status
    }

    /// 获取有效期范围
    pub fn validity_range(&self) -> &ValidityRange {
        &self.validity_range
    }

    /// 获取开始日期
    pub fn valid_from(&self) -> NaiveDate {
        self.validity_range.valid_from
    }

    /// 获取结束日期
    pub fn valid_to(&self) -> NaiveDate {
        self.validity_range.valid_to
    }

    /// 获取描述
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// 获取版本号
    pub fn version(&self) -> u64 {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fiscal_period() {
        let period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(period.is_ok());
        let period = period.unwrap();
        assert_eq!(period.status(), PeriodStatus::Open);
        assert_eq!(period.period(), 1);
    }

    #[test]
    fn test_invalid_period_number() {
        let result = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            17, // 无效期间号
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_date_range() {
        let result = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // 结束日期早于开始日期
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_close_period() {
        let mut period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let events = period.close().unwrap();

        assert_eq!(period.status(), PeriodStatus::Closed);
        assert_eq!(events.len(), 1);
        assert!(!period.allows_posting());
    }

    #[test]
    fn test_reopen_period() {
        let mut period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        // 先关闭
        period.close().unwrap();

        // 重新开放
        let events = period.open().unwrap();

        assert_eq!(period.status(), PeriodStatus::Open);
        assert_eq!(events.len(), 1);
        assert!(period.allows_posting());
    }

    #[test]
    fn test_is_open_at() {
        let period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_open_at(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(!period.is_open_at(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }

    #[test]
    fn test_special_period() {
        let period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            13, // 特殊期间
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        )
        .unwrap();

        assert!(period.is_special_period());
    }

    #[test]
    fn test_start_closing() {
        let mut period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let result = period.start_closing();
        assert!(result.is_ok());
        assert_eq!(period.status(), PeriodStatus::Closing);
        assert!(!period.allows_posting());
    }

    #[test]
    fn test_cannot_reopen_open_period() {
        let mut period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let result = period.open();
        assert!(result.is_err());
    }
}
