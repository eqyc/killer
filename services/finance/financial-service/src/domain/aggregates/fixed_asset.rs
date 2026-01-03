//! 固定资产聚合根

use chrono::{DateTime, Utc};
use crate::domain::events::{FixedAssetAcquired, FixedAssetDepreciated, FixedAssetRetired};
use crate::domain::aggregates::fixed_asset::AssetStatus;
use killer_domain_primitives::{CompanyCode, Money, AuditInfo};

/// 固定资产聚合根
///
/// 代表 SAP 风格的固定资产主数据
#[derive(Debug, Clone)]
pub struct FixedAsset {
    /// 公司代码
    company_code: CompanyCode,
    /// 资产编号
    asset_number: String,
    /// 资产子编号
    sub_number: String,
    /// 资产类别
    asset_class: String,
    /// 评估类
    valuation_class: String,
    /// 成本中心
    cost_center: Option<String>,
    /// 利润中心
    profit_center: Option<String>,
    /// 业务范围
    business_area: Option<String>,
    /// 位置
    location: Option<String>,
    /// 投资订单
    investment_order: Option<String>,
    /// 描述
    description: String,
    /// 描述长文本
    long_description: Option<String>,
    /// 资本化日期
    capitalization_date: Option<chrono::NaiveDate>,
    /// 资产原值
    acquisition_value: Money,
    /// 累计折旧
    accumulated_depreciation: Money,
    /// 计划外折旧
    unplanned_depreciation: Money,
    /// 资产类型（用于重估）
    asset_type: Option<Money>,
    /// 状态
    status: AssetStatus,
    /// 审计信息
    audit_info: AuditInfo,
}

impl FixedAsset {
    /// 创建新的固定资产
    pub fn new(
        company_code: CompanyCode,
        asset_class: impl Into<String>,
        valuation_class: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            company_code,
            asset_number: String::new(),
            sub_number: String::new(),
            asset_class: asset_class.into(),
            valuation_class: valuation_class.into(),
            cost_center: None,
            profit_center: None,
            business_area: None,
            location: None,
            investment_order: None,
            description: description.into(),
            long_description: None,
            capitalization_date: None,
            acquisition_value: Money::zero(),
            accumulated_depreciation: Money::zero(),
            unplanned_depreciation: Money::zero(),
            asset_type: None,
            status: AssetStatus::Created,
            audit_info: AuditInfo::new("SYSTEM".to_string(), now),
        }
    }

    // Getters
    pub fn company_code(&self) -> &CompanyCode {
        &self.company_code
    }

    pub fn asset_number(&self) -> &str {
        &self.asset_number
    }

    pub fn sub_number(&self) -> &str {
        &self.sub_number
    }

    pub fn asset_class(&self) -> &str {
        &self.asset_class
    }

    pub fn valuation_class(&self) -> &str {
        &self.valuation_class
    }

    pub fn cost_center(&self) -> Option<&str> {
        self.cost_center.as_deref()
    }

    pub fn profit_center(&self) -> Option<&str> {
        self.profit_center.as_deref()
    }

    pub fn business_area(&self) -> Option<&str> {
        self.business_area.as_deref()
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn long_description(&self) -> Option<&str> {
        self.long_description.as_deref()
    }

    pub fn capitalization_date(&self) -> Option<chrono::NaiveDate> {
        self.capitalization_date
    }

    pub fn acquisition_value(&self) -> Money {
        self.acquisition_value
    }

    pub fn accumulated_depreciation(&self) -> Money {
        self.accumulated_depreciation
    }

    pub fn unplanned_depreciation(&self) -> Money {
        self.unplanned_depreciation
    }

    /// 计算账面价值
    pub fn book_value(&self) -> Money {
        self.acquisition_value
            .sub(self.accumulated_depreciation)
            .sub(self.unplanned_depreciation)
    }

    pub fn status(&self) -> AssetStatus {
        self.status
    }

    pub fn is_capitalized(&self) -> bool {
        self.status == AssetStatus::Capitalized
    }

    pub fn is_retired(&self) -> bool {
        self.status == AssetStatus::Retired
    }

    // Commands

    /// 设置资产编号
    pub fn set_asset_number(&mut self, number: impl Into<String>) {
        self.asset_number = number.into();
    }

    /// 设置资产子编号
    pub fn set_sub_number(&mut self, number: impl Into<String>) {
        self.sub_number = number.into();
    }

    /// 设置成本中心
    pub fn set_cost_center(&mut self, cost_center: impl Into<String>) {
        self.cost_center = Some(cost_center.into());
    }

    /// 设置利润中心
    pub fn set_profit_center(&mut self, profit_center: impl Into<String>) {
        self.profit_center = Some(profit_center.into());
    }

    /// 设置位置
    pub fn set_location(&mut self, location: impl Into<String>) {
        self.location = Some(location.into());
    }

    /// 资产采购（资本化）
    pub fn capitalize(
        &mut self,
        capitalization_date: chrono::NaiveDate,
        acquisition_value: Money,
    ) {
        self.capitalization_date = Some(capitalization_date);
        self.acquisition_value = acquisition_value;
        self.status = AssetStatus::Capitalized;
    }

    /// 折旧
    pub fn depreciate(&mut self, amount: Money) {
        self.accumulated_depreciation = self.accumulated_depreciation.add(amount);
    }

    /// 计划外折旧
    pub fn unplanned_depreciation_amount(&mut self, amount: Money) {
        self.unplanned_depreciation = self.unplanned_depreciation.add(amount);
    }

    /// 转移
    pub fn transfer(
        &mut self,
        new_cost_center: Option<String>,
        new_profit_center: Option<String>,
        new_business_area: Option<String>,
    ) {
        if let Some(cc) = new_cost_center {
            self.cost_center = Some(cc);
        }
        if let Some(pc) = new_profit_center {
            self.profit_center = Some(pc);
        }
        if let Some(ba) = new_business_area {
            self.business_area = Some(ba);
        }
    }

    /// 报废
    pub fn retire(&mut self, retirement_value: Money) -> Result<(), String> {
        if self.is_retired() {
            return Err("资产已报废".to_string());
        }
        // 报废金额应该等于账面价值
        self.status = AssetStatus::Retired;
        Ok(())
    }

    /// 冻结
    pub fn block(&mut self) {
        self.status = AssetStatus::Blocked;
    }

    // Events

    pub fn into_acquired_event(self) -> FixedAssetAcquired {
        FixedAssetAcquired {
            company_code: self.company_code,
            asset_number: self.asset_number,
            acquisition_value: self.acquisition_value,
            acquired_at: self.capitalization_date
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc))
                .unwrap_or_else(Utc::now),
        }
    }

    pub fn into_depreciated_event(self, fiscal_year: &str, period: u32) -> FixedAssetDepreciated {
        FixedAssetDepreciated {
            company_code: self.company_code,
            asset_number: self.asset_number,
            depreciation_amount: self.accumulated_depreciation,
            fiscal_year: fiscal_year.to_string(),
            period,
            depreciated_at: Utc::now(),
        }
    }

    pub fn into_retired_event(self) -> FixedAssetRetired {
        FixedAssetRetired {
            company_code: self.company_code,
            asset_number: self.asset_number,
            retirement_value: self.book_value(),
            retired_at: Utc::now(),
        }
    }
}

/// 资产状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetStatus {
    Created = 1,      // 已创建
    Capitalized = 2,  // 已资本化
    Blocked = 3,      // 已冻结
    Retired = 4,      // 已报废
}

impl TryFrom<i32> for AssetStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Created),
            2 => Ok(Self::Capitalized),
            3 => Ok(Self::Blocked),
            4 => Ok(Self::Retired),
            _ => Err(()),
        }
    }
}
