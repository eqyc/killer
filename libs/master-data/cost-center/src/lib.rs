//! 成本中心主数据域
//!
//! 提供成本中心、利润中心、成本要素的定义。
//! 支持时间有效性、层级结构和成本分配。
//!
//! # 核心实体
//!
//! - [`CostCenter`] - 成本中心 (CSKS)
//! - [`ProfitCenter`] - 利润中心 (CEPC)
//! - [`CostElement`] - 成本要素 (CSKB)
//!
//! # 示例
//!
//! ```rust
//! use killer_master_data_cost_center::*;
//!
//! let cost_center = CostCenter::new(
//!     "tenant-001",
//!     "CC-001",
//!     "1000",  // controlling_area
//!     "研发部门",
//!     CostCenterCategory::Production,
//! ).expect("Failed to create cost center");
//!
//! let profit_center = ProfitCenter::new(
//!     "tenant-001",
//!     "PC-001",
//!     "1000",  // controlling_area
//!     "华东区",
//! ).expect("Failed to create profit center");
//! ```

#![warn(missing_docs, unreachable_pub)]
#![cfg_attr(feature = "prost", allow(dead_code))]

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, From};
use killer_domain_primitives::*;
use killer_types::{CurrencyCode, ValidationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// 错误类型
// =============================================================================

/// 成本中心域错误
#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostCenterError {
    #[error("成本中心不存在: {code}")]
    CostCenterNotFound { code: String },

    #[error("利润中心不存在: {code}")]
    ProfitCenterNotFound { code: String },

    #[error("成本要素不存在: {code}")]
    CostElementNotFound { code: String },

    #[error("无效的成本中心代码: {code}")]
    InvalidCostCenterCode { code: String },

    #[error("验证失败: {message}")]
    ValidationFailed { message: String },

    #[error("时间有效性冲突: {message}")]
    ValidityConflict { message: String },
}

/// 成本中心结果类型
pub type CostCenterResult<T> = Result<T, CostCenterError>;

// =============================================================================
// 扩展字段支持
// =============================================================================

/// 扩展字段容器
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
// 有效性时间范围
// =============================================================================

/// 有效性时间范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidityRange {
    /// 生效日期
    pub valid_from: DateTime<Utc>,
    /// 失效日期 (None 表示无结束日期)
    #[serde(default)]
    pub valid_to: Option<DateTime<Utc>>,
}

impl ValidityRange {
    /// 创建新的有效性范围
    pub fn new(valid_from: DateTime<Utc>, valid_to: Option<DateTime<Utc>>) -> Self {
        Self { valid_from, valid_to }
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: DateTime<Utc>) -> bool {
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
        self.is_valid_at(Utc::now())
    }
}

impl Default for ValidityRange {
    fn default() -> Self {
        Self {
            valid_from: Utc::now(),
            valid_to: None,
        }
    }
}

// =============================================================================
// 成本中心类别
// =============================================================================

/// 成本中心类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostCenterCategory {
    /// 生产成本中心
    Production,
    /// 管理成本中心
    Administration,
    /// 销售成本中心
    Sales,
    /// 研发成本中心
    Research,
    /// 服务成本中心
    Service,
}

// =============================================================================
// 成本中心 (CSKS)
// =============================================================================

/// 成本中心
///
/// SAP 表 CSKS，代表成本控制的基本单位。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct CostCenter {
    /// 成本中心代码
    #[validate(non_empty)]
    pub code: CostCenterValue,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 控制范围
    #[validate(length(min = 4, max = 4))]
    pub controlling_area: String,

    /// 成本中心名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 成本中心类别
    pub category: CostCenterCategory,

    /// 负责人
    #[validate(length(max = 50))]
    #[serde(default)]
    pub responsible_person: Option<String>,

    /// 部门
    #[validate(length(max = 50))]
    #[serde(default)]
    pub department: Option<String>,

    /// 公司代码
    #[validate(length(min = 4, max = 4))]
    #[serde(default)]
    pub company_code: Option<CompanyCodeValue>,

    /// 利润中心
    #[serde(default)]
    pub profit_center: Option<String>,

    /// 有效性范围
    pub validity: ValidityRange,

    /// 是否锁定
    #[serde(default)]
    pub locked: bool,

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

impl CostCenter {
    /// 创建新的成本中心
    pub fn new(
        tenant_id: impl Into<String>,
        code: impl Into<CostCenterValue>,
        controlling_area: impl Into<String>,
        name: impl Into<String>,
        category: CostCenterCategory,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let cost_center = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            controlling_area: controlling_area.into(),
            name: name.into(),
            category,
            responsible_person: None,
            department: None,
            company_code: None,
            profit_center: None,
            validity: ValidityRange::default(),
            locked: false,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        cost_center.validate()?;
        Ok(cost_center)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 锁定成本中心
    pub fn lock(&mut self) {
        self.locked = true;
        self.updated_at = Utc::now();
    }

    /// 解锁成本中心
    pub fn unlock(&mut self) {
        self.locked = false;
        self.updated_at = Utc::now();
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: DateTime<Utc>) -> bool {
        !self.deleted && !self.locked && self.validity.is_valid_at(date)
    }

    /// 检查当前是否有效
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid_at(Utc::now())
    }

    /// 更新有效期
    pub fn update_validity(&mut self, valid_from: DateTime<Utc>, valid_to: Option<DateTime<Utc>>) {
        self.validity = ValidityRange::new(valid_from, valid_to);
        self.updated_at = Utc::now();
    }

    /// 分配利润中心
    pub fn assign_profit_center(&mut self, profit_center: impl Into<String>) {
        self.profit_center = Some(profit_center.into());
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 利润中心 (CEPC)
// =============================================================================

/// 利润中心
///
/// SAP 表 CEPC，代表利润责任单位。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct ProfitCenter {
    /// 利润中心代码
    #[validate(length(min = 4, max = 10))]
    pub code: String,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 控制范围
    #[validate(length(min = 4, max = 4))]
    pub controlling_area: String,

    /// 利润中心名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 负责人
    #[validate(length(max = 50))]
    #[serde(default)]
    pub responsible_person: Option<String>,

    /// 上级利润中心
    #[serde(default)]
    pub parent_profit_center: Option<String>,

    /// 公司代码
    #[validate(length(min = 4, max = 4))]
    #[serde(default)]
    pub company_code: Option<CompanyCodeValue>,

    /// 有效性范围
    pub validity: ValidityRange,

    /// 是否锁定
    #[serde(default)]
    pub locked: bool,

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

impl ProfitCenter {
    /// 创建新的利润中心
    pub fn new(
        tenant_id: impl Into<String>,
        code: impl Into<String>,
        controlling_area: impl Into<String>,
        name: impl Into<String>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let profit_center = Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            controlling_area: controlling_area.into(),
            name: name.into(),
            responsible_person: None,
            parent_profit_center: None,
            company_code: None,
            validity: ValidityRange::default(),
            locked: false,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        profit_center.validate()?;
        Ok(profit_center)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 锁定利润中心
    pub fn lock(&mut self) {
        self.locked = true;
        self.updated_at = Utc::now();
    }

    /// 解锁利润中心
    pub fn unlock(&mut self) {
        self.locked = false;
        self.updated_at = Utc::now();
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: DateTime<Utc>) -> bool {
        !self.deleted && !self.locked && self.validity.is_valid_at(date)
    }

    /// 检查当前是否有效
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid_at(Utc::now())
    }

    /// 更新有效期
    pub fn update_validity(&mut self, valid_from: DateTime<Utc>, valid_to: Option<DateTime<Utc>>) {
        self.validity = ValidityRange::new(valid_from, valid_to);
        self.updated_at = Utc::now();
    }

    /// 设置上级利润中心
    pub fn set_parent(&mut self, parent: impl Into<String>) {
        self.parent_profit_center = Some(parent.into());
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 成本要素类别
// =============================================================================

/// 成本要素类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostElementCategory {
    /// 初级成本要素 (对应总账科目)
    #[serde(rename = "1")]
    Primary,
    /// 次级成本要素 (内部分配)
    #[serde(rename = "2")]
    Secondary,
    /// 收入要素
    #[serde(rename = "11")]
    Revenue,
}

// =============================================================================
// 成本要素 (CSKB)
// =============================================================================

/// 成本要素
///
/// SAP 表 CSKB，代表成本核算的基本分类。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct CostElement {
    /// 成本要素代码 (通常与会计科目一致)
    #[validate(non_empty)]
    pub code: AccountCode,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 控制范围
    #[validate(length(min = 4, max = 4))]
    pub controlling_area: String,

    /// 成本要素名称
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// 成本要素类别
    pub category: CostElementCategory,

    /// 对应的总账科目 (仅初级成本要素)
    #[serde(default)]
    pub gl_account: Option<AccountCode>,

    /// 有效性范围
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

impl CostElement {
    /// 创建新的成本要素
    pub fn new(
        tenant_id: impl Into<String>,
        code: AccountCode,
        controlling_area: impl Into<String>,
        name: impl Into<String>,
        category: CostElementCategory,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let cost_element = Self {
            code,
            tenant_id: tenant_id.into(),
            controlling_area: controlling_area.into(),
            name: name.into(),
            category,
            gl_account: None,
            validity: ValidityRange::default(),
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        cost_element.validate()?;
        Ok(cost_element)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 检查在指定日期是否有效
    pub fn is_valid_at(&self, date: DateTime<Utc>) -> bool {
        !self.deleted && self.validity.is_valid_at(date)
    }

    /// 检查当前是否有效
    pub fn is_currently_valid(&self) -> bool {
        self.is_valid_at(Utc::now())
    }

    /// 更新有效期
    pub fn update_validity(&mut self, valid_from: DateTime<Utc>, valid_to: Option<DateTime<Utc>>) {
        self.validity = ValidityRange::new(valid_from, valid_to);
        self.updated_at = Utc::now();
    }

    /// 关联总账科目 (仅初级成本要素)
    pub fn link_gl_account(&mut self, gl_account: AccountCode) -> CostCenterResult<()> {
        if self.category != CostElementCategory::Primary {
            return Err(CostCenterError::ValidationFailed {
                message: "Only primary cost elements can be linked to GL accounts".to_string(),
            });
        }

        self.gl_account = Some(gl_account);
        self.updated_at = Utc::now();
        Ok(())
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
    /// 锁定
    Locked,
    /// 解锁
    Unlocked,
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

/// 成本中心变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCenterChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 成本中心代码
    pub code: CostCenterValue,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 有效性变更
    pub validity_change: Option<ValidityRange>,
    /// 完整快照
    pub snapshot: Option<CostCenter>,
}

/// 利润中心变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitCenterChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 利润中心代码
    pub code: String,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 有效性变更
    pub validity_change: Option<ValidityRange>,
    /// 完整快照
    pub snapshot: Option<ProfitCenter>,
}

/// 成本要素变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostElementChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 成本要素代码
    pub code: AccountCode,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<CostElement>,
}

// =============================================================================
// Protobuf 导出 (条件编译)
// =============================================================================

#[cfg(feature = "prost")]
pub mod proto {
    include!("cost_center_prost.rs");
}

// =============================================================================
// Re-exports
// =============================================================================

pub use self::cost_center_error::CostCenterError;

mod cost_center_error {
    use super::*;

    // 错误类型已在上面定义
}
