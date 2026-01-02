//! 组织单元主数据域
//!
//! 提供公司代码、工厂、库存地点、采购组织和销售组织等组织结构的定义。
//! 支持层级建模、时间有效性、审计追踪和变更事件。
//!
//! # 核心实体
//!
//! - [`CompanyCode`] - 公司代码 (T001)
//! - [`Plant`] - 工厂 (T001W)
//! - [`StorageLocation`] - 库存地点 (T001L)
//! - [`PurchasingOrganization`] - 采购组织 (T024E)
//! - [`SalesOrganization`] - 销售组织 (TVKO)
//! - [`ControllingArea`] - 控制范围 (T000)
//!
//! # 层级关系
//!
//! ```text
//! CompanyCode (公司代码)
//!   ├── Plant (工厂)
//!   │     └── StorageLocation (库存地点)
//!   ├── PurchasingOrganization (采购组织)
//!   └── SalesOrganization (销售组织)
//! ```
//!
//! # 示例
//!
//! ```rust
//! use killer_master_data_organizational_units::*;
//!
//! let company = CompanyCode::new(
//!     "tenant-001",
//!     "1000",
//!     "示例公司",
//!     "CN",
//!     "CNY",
//! ).expect("Failed to create company code");
//!
//! let plant = Plant::new(
//!     "tenant-001",
//!     "1000",  // company_code
//!     "SH01",
//!     "上海工厂",
//!     "Shanghai",
//!     "CN",
//!     Some(chrono::Local::now().date_naive()),
//!     None,
//! ).expect("Failed to create plant");
//! ```

#![warn(missing_docs, unreachable_pub)]
#![cfg_attr(feature = "prost", allow(dead_code))]

use chrono::{Date, DateTime, NaiveDate, Utc};
use derive_more::{Display, Error, From};
use killer_domain_primitives::*;
use killer_types::{CurrencyCode, ValidationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::Hash;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// 有效性时间范围
// =============================================================================

/// 有效性时间范围
///
/// 用于支持时间依赖的主数据，如工厂的有效期、成本中心的分配有效期等。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, From,
)]
#[display(
    fmt = "valid from {} to {}",
    "valid_from",
    "valid_to.as_ref().map_or(\"unlimited\", |d| d.format(\"%Y-%m-%d\").to_string())"
)]
pub struct ValidityRange {
    /// 生效日期
    pub valid_from: Date<Utc>,
    /// 失效日期 (None 表示无结束日期)
    #[serde(default)]
    pub valid_to: Option<Date<Utc>>,
}

impl ValidityRange {
    /// 创建新的有效性范围
    pub fn new(valid_from: Date<Utc>, valid_to: Option<Date<Utc>>) -> Self {
        Self { valid_from, valid_to }
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: Date<Utc>) -> bool {
        if date < self.valid_from {
            return false;
        }
        if let Some(valid_to) = self.valid_to {
            if date > valid_to {
                return false;
            }
        }
        true
    }

    /// 检查当前是否有效
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid_at(Utc::now().date())
    }

    /// 获取与另一个范围的交集
    pub fn intersection(&self, other: &ValidityRange) -> Option<ValidityRange> {
        let valid_from = std::cmp::max(self.valid_from, other.valid_from);

        let valid_to = match (self.valid_to, other.valid_to) {
            (Some(t1), Some(t2)) => Some(std::cmp::min(t1, t2)),
            (Some(t), None) => Some(t),
            (None, Some(t)) => Some(t),
            (None, None) => None,
        };

        if valid_from <= valid_to.unwrap_or(valid_from) {
            Some(ValidityRange::new(valid_from, valid_to))
        } else {
            None
        }
    }
}

impl Default for ValidityRange {
    fn default() -> Self {
        Self {
            valid_from: Utc::now().date(),
            valid_to: None,
        }
    }
}

// =============================================================================
// 错误类型
// =============================================================================

/// 组织单元域错误
#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationalUnitsError {
    #[error("公司代码不存在: {code}")]
    CompanyCodeNotFound { code: String },

    #[error("工厂不存在: {code}")]
    PlantNotFound { code: String },

    #[error("库存地点不存在: {code}")]
    StorageLocationNotFound { code: String },

    #[error("采购组织不存在: {code}")]
    PurchasingOrgNotFound { code: String },

    #[error("销售组织不存在: {code}")]
    SalesOrgNotFound { code: String },

    #[error("控制范围不存在: {code}")]
    ControllingAreaNotFound { code: String },

    #[error("层级一致性验证失败: {message}")]
    HierarchyValidationFailed { message: String },

    #[error("时间有效性验证失败: {message}")]
    ValidityValidationFailed { message: String },

    #[error("无效的公司代码格式: {code}")]
    InvalidCompanyCodeFormat { code: String },

    #[error("无效的工厂代码格式: {code}")]
    InvalidPlantFormat { code: String },

    #[error("无效的库存地点代码格式: {code}")]
    InvalidStorageLocationFormat { code: String },
}

/// 组织单元结果类型
pub type OrganizationalUnitsResult<T> = Result<T, OrganizationalUnitsError>;

// =============================================================================
// 扩展字段支持
// =============================================================================

/// 扩展字段容器
///
/// 用于存储自定义字段，避免 Protobuf 频繁修改。
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct Extensions(HashMap<String, serde_json::Value>);

impl Extensions {
    /// 创建新的扩展容器
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// 获取扩展值
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.0.get(key)
    }

    /// 设置扩展值
    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.0.insert(key, value);
    }

    /// 检查是否包含键
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// 删除扩展值
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.0.remove(key)
    }

    /// 获取内部 Map 的只读引用
    pub fn inner(&self) -> &HashMap<String, serde_json::Value> {
        &self.0
    }
}

impl From<HashMap<String, serde_json::Value>> for Extensions {
    fn from(map: HashMap<String, serde_json::Value>) -> Self {
        Self(map)
    }
}

// =============================================================================
// 公司代码 (T001)
// =============================================================================

/// 公司代码
///
/// SAP 表 T001，代表法务实体或公司。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct CompanyCode {
    /// 公司代码 (4位)
    #[validate(length(min = 4, max = 4, message = "公司代码必须为4位"))]
    pub code: CompanyCodeValue,

    /// 租户ID (多租户支持)
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 公司名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 街道地址
    #[validate(length(max = 200))]
    #[serde(default)]
    pub street_address: Option<String>,

    /// 城市
    #[validate(length(max = 50))]
    #[serde(default)]
    pub city: Option<String>,

    /// 邮政编码
    #[validate(length(max = 20))]
    #[serde(default)]
    pub postal_code: Option<String>,

    /// 国家代码 (ISO 3166)
    #[validate(length(min = 2, max = 3))]
    pub country: CountryCodeValue,

    /// 本位币代码 (ISO 4217)
    pub currency_code: CurrencyCodeValue,

    /// 会计年度变式
    #[serde(default)]
    pub fiscal_year_variant: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl CompanyCode {
    /// 创建新的公司代码
    pub fn new(
        tenant_id: impl Into<String>,
        code: impl Into<CompanyCodeValue>,
        name: impl Into<String>,
        country: impl Into<CountryCodeValue>,
        currency_code: impl Into<CurrencyCodeValue>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();
        let code = code.into();
        let country = country.into();
        let currency_code = currency_code.into();

        let company = Self {
            code,
            tenant_id: tenant_id.into(),
            name: name.into(),
            street_address: None,
            city: None,
            postal_code: None,
            country,
            currency_code,
            fiscal_year_variant: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        company.validate()?;
        Ok(company)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 更新公司名称
    pub fn update_name(&mut self, name: impl Into<String>) -> ValidationResult<()> {
        let name = name.into();
        validator::ValidateVal::validate_val(&name)?;
        self.name = name;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// =============================================================================
// 工厂 (T001W)
// =============================================================================

/// 工厂
///
/// SAP 表 T001W，代表一个工厂或生产基地。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct Plant {
    /// 工厂代码 (4位)
    #[validate(length(min = 4, max = 4, message = "工厂代码必须为4位"))]
    pub code: PlantValue,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 所属公司代码 (引用)
    #[validate(non_empty)]
    pub company_code: CompanyCodeValue,

    /// 工厂名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 城市
    #[validate(length(max = 50))]
    #[serde(default)]
    pub city: Option<String>,

    /// 国家代码
    #[validate(length(min = 2, max = 3))]
    pub country: CountryCodeValue,

    /// 地区/州
    #[serde(default)]
    pub region: Option<String>,

    /// 有效性范围
    #[validate]
    pub validity: ValidityRange,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl Plant {
    /// 创建新的工厂
    pub fn new(
        tenant_id: impl Into<String>,
        company_code: impl Into<CompanyCodeValue>,
        code: impl Into<PlantValue>,
        name: impl Into<String>,
        city: impl Into<String>,
        country: impl Into<CountryCodeValue>,
        valid_from: Option<Date<Utc>>,
        valid_to: Option<Date<Utc>>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();
        let code = code.into();
        let country = country.into();

        let plant = Self {
            code,
            tenant_id: tenant_id.into(),
            company_code: company_code.into(),
            name: name.into(),
            city: Some(city.into()),
            country,
            region: None,
            validity: ValidityRange::new(
                valid_from.unwrap_or(now.date()),
                valid_to,
            ),
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        plant.validate()?;
        Ok(plant)
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: Date<Utc>) -> bool {
        self.validity.is_valid_at(date)
    }

    /// 检查当前是否有效
    pub fn is_currently_valid(&self) -> bool {
        !self.deleted && self.validity.is_currently_valid()
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 更新有效期
    pub fn update_validity(
        &mut self,
        valid_from: Date<Utc>,
        valid_to: Option<Date<Utc>>,
    ) {
        self.validity = ValidityRange::new(valid_from, valid_to);
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 库存地点 (T001L)
// =============================================================================

/// 库存地点
///
/// SAP 表 T001L，代表工厂内的库存存储位置。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct StorageLocation {
    /// 库存地点代码 (4位)
    #[validate(length(min = 4, max = 4))]
    pub code: StorageLocationValue,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 所属工厂 (引用)
    #[validate(non_empty)]
    pub plant_code: PlantValue,

    /// 库存地点名称
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    /// 地址
    #[serde(default)]
    pub address: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl StorageLocation {
    /// 创建新的库存地点
    pub fn new(
        tenant_id: impl Into<String>,
        plant_code: impl Into<PlantValue>,
        code: impl Into<StorageLocationValue>,
        name: impl Into<String>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let storage = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            plant_code: plant_code.into(),
            name: name.into(),
            address: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        storage.validate()?;
        Ok(storage)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 采购组织 (T024E)
// =============================================================================

/// 采购组织
///
/// SAP 表 T024E，负责采购业务。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct PurchasingOrganization {
    /// 采购组织代码 (4位)
    #[validate(length(min = 4, max = 4))]
    pub code: String,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 所属公司代码
    #[validate(non_empty)]
    pub company_code: CompanyCodeValue,

    /// 采购组织名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 是否跨公司采购
    #[serde(default)]
    pub cross_company: bool,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl PurchasingOrganization {
    /// 创建新的采购组织
    pub fn new(
        tenant_id: impl Into<String>,
        company_code: impl Into<CompanyCodeValue>,
        code: impl Into<String>,
        name: impl Into<String>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let org = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            company_code: company_code.into(),
            name: name.into(),
            cross_company: false,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        org.validate()?;
        Ok(org)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 销售组织 (TVKO)
// =============================================================================

/// 销售组织
///
/// SAP 表 TVKO，负责销售业务。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct SalesOrganization {
    /// 销售组织代码 (4位)
    #[validate(length(min = 4, max = 4))]
    pub code: String,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 所属公司代码
    #[validate(non_empty)]
    pub company_code: CompanyCodeValue,

    /// 销售组织名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 销售渠道
    #[serde(default)]
    pub sales_channel: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl SalesOrganization {
    /// 创建新的销售组织
    pub fn new(
        tenant_id: impl Into<String>,
        company_code: impl Into<CompanyCodeValue>,
        code: impl Into<String>,
        name: impl Into<String>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let org = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            company_code: company_code.into(),
            name: name.into(),
            sales_channel: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        org.validate()?;
        Ok(org)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 控制范围 (T000)
// =============================================================================

/// 控制范围
///
/// SAP 表 T000，用于成本控制。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate,
)]
#[validate(schema(_))]
pub struct ControllingArea {
    /// 控制范围代码 (4位)
    #[validate(length(min = 4, max = 4))]
    pub code: String,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 控制范围名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 控制范围本位币
    pub currency_code: CurrencyCodeValue,

    /// 是否激活
    #[serde(default)]
    pub active: bool,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 扩展字段
    #[serde(default)]
    pub extensions: Extensions,

    /// 软删除标记
    #[serde(default)]
    pub deleted: bool,
}

impl ControllingArea {
    /// 创建新的控制范围
    pub fn new(
        tenant_id: impl Into<String>,
        code: impl Into<String>,
        name: impl Into<String>,
        currency_code: impl Into<CurrencyCodeValue>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let area = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            currency_code: currency_code.into(),
            active: true,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        area.validate()?;
        Ok(area)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 停用控制范围
    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }

    /// 激活控制范围
    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 变更事件
// =============================================================================

/// 变更事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeEventType {
    /// 创建
    Created,
    /// 更新
    Updated,
    /// 删除
    Deleted,
    /// 阻塞
    Blocked,
    /// 解锁
    Unblocked,
    /// 有效性变更
    ValidityChanged,
}

/// 字段变更
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDelta {
    /// 字段名
    pub field_name: String,
    /// 旧值
    pub old_value: Option<serde_json::Value>,
    /// 新值
    pub new_value: Option<serde_json::Value>,
}

/// 变更事件头
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeEventHeader {
    /// 事件ID
    pub event_id: Uuid,
    /// 租户ID
    pub tenant_id: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 操作者ID
    pub actor_id: String,
    /// 事件类型
    pub event_type: ChangeEventType,
    /// 事件版本
    pub version: i32,
    /// 关联ID
    pub correlation_id: Option<Uuid>,
}

impl ChangeEventHeader {
    /// 创建新的事件头
    pub fn new(
        tenant_id: impl Into<String>,
        actor_id: impl Into<String>,
        event_type: ChangeEventType,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            tenant_id: tenant_id.into(),
            timestamp: Utc::now(),
            actor_id: actor_id.into(),
            event_type,
            version: 1,
            correlation_id: None,
        }
    }
}

/// 公司代码变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyCodeChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 公司代码
    pub code: CompanyCodeValue,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<CompanyCode>,
}

/// 工厂变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 工厂代码
    pub code: PlantValue,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 有效性变更
    pub validity_change: Option<ValidityRange>,
    /// 完整快照
    pub snapshot: Option<Plant>,
}

/// 库存地点变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocationChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 库存地点代码
    pub code: StorageLocationValue,
    /// 工厂代码
    pub plant_code: PlantValue,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<StorageLocation>,
}

// =============================================================================
// Protobuf 导出 (条件编译)
// =============================================================================

#[cfg(feature = "prost")]
pub mod proto {
    include!("organizational_units_prost.rs");
}

// =============================================================================
// Re-exports
// =============================================================================

pub use self::organizational_units_error::OrganizationalUnitsError;

mod organizational_units_error {
    use super::*;

    // 错误类型已在上面定义
}
