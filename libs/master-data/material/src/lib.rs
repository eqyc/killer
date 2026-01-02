//! 物料主数据域
//!
//! 提供物料基本数据、工厂数据、库存地点数据的定义。
//! 支持多层级数据结构：Material -> MaterialPlantData -> MaterialStorageData
//!
//! # 核心实体
//!
//! - [`Material`] - 物料基本数据 (MARA)
//! - [`MaterialPlantData`] - 物料工厂数据 (MARC)
//! - [`MaterialStorageData`] - 物料库存地点数据 (MARD)
//!
//! # 示例
//!
//! ```rust
//! use killer_master_data_material::*;
//!
//! let material = Material::new(
//!     "tenant-001",
//!     "MAT-001",
//!     "示例物料",
//!     MaterialType::FinishedProduct,
//!     "EA",
//! ).expect("Failed to create material");
//!
//! let plant_data = MaterialPlantData::new(
//!     "tenant-001",
//!     "MAT-001",
//!     "1000",  // plant_code
//!     MrpType::PD,
//!     ProcurementType::InHouse,
//! ).expect("Failed to create plant data");
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

/// 物料域错误
#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialError {
    #[error("物料不存在: {material_number}")]
    MaterialNotFound { material_number: String },

    #[error("物料工厂数据不存在: {material_number}/{plant_code}")]
    PlantDataNotFound {
        material_number: String,
        plant_code: String,
    },

    #[error("物料库存地点数据不存在: {material_number}/{plant_code}/{storage_location}")]
    StorageDataNotFound {
        material_number: String,
        plant_code: String,
        storage_location: String,
    },

    #[error("无效的物料编号: {material_number}")]
    InvalidMaterialNumber { material_number: String },

    #[error("验证失败: {message}")]
    ValidationFailed { message: String },

    #[error("库存不足: 可用 {available}, 需要 {required}")]
    InsufficientStock { available: f64, required: f64 },
}

/// 物料结果类型
pub type MaterialResult<T> = Result<T, MaterialError>;

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
// 物料类型
// =============================================================================

/// 物料类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaterialType {
    /// 原材料
    RawMaterial,
    /// 半成品
    SemiFinished,
    /// 成品
    FinishedProduct,
    /// 贸易货物
    TradingGoods,
    /// 备件
    SparePart,
    /// 服务
    Service,
    /// 包装材料
    PackagingMaterial,
}

/// MRP 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MrpType {
    /// 无计划
    #[serde(rename = "ND")]
    None,
    /// 计划独立需求
    #[serde(rename = "PD")]
    PD,
    /// MRP
    #[serde(rename = "VB")]
    VB,
    /// 重订货点计划
    #[serde(rename = "VM")]
    VM,
}

/// 采购类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcurementType {
    /// 自制
    InHouse,
    /// 外购
    External,
    /// 两者皆可
    Both,
}

/// 特殊采购类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialProcurementType {
    /// 无
    #[serde(rename = "")]
    None,
    /// 委外加工
    #[serde(rename = "30")]
    Subcontracting,
    /// 第三方
    #[serde(rename = "50")]
    ThirdParty,
}

impl Default for SpecialProcurementType {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// 物料基本数据 (MARA)
// =============================================================================

/// 物料基本数据
///
/// SAP 表 MARA，代表物料的基本信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct Material {
    /// 物料编号
    #[validate(non_empty)]
    pub material_number: MaterialNumber,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 物料描述
    #[validate(length(min = 1, max = 200))]
    pub description: String,

    /// 物料类型
    pub material_type: MaterialType,

    /// 基本计量单位
    #[validate(non_empty)]
    pub base_unit: UnitOfMeasure,

    /// 物料组
    #[validate(length(max = 20))]
    #[serde(default)]
    pub material_group: Option<String>,

    /// 毛重
    #[serde(default)]
    pub gross_weight: Option<Quantity>,

    /// 净重
    #[serde(default)]
    pub net_weight: Option<Quantity>,

    /// 体积
    #[serde(default)]
    pub volume: Option<Quantity>,

    /// 尺寸单位
    #[serde(default)]
    pub size_unit: Option<UnitOfMeasure>,

    /// 行业标准描述
    #[validate(length(max = 100))]
    #[serde(default)]
    pub industry_standard_desc: Option<String>,

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

impl Material {
    /// 创建新的物料
    pub fn new(
        tenant_id: impl Into<String>,
        material_number: impl Into<MaterialNumber>,
        description: impl Into<String>,
        material_type: MaterialType,
        base_unit: impl Into<UnitOfMeasure>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let material = Self {
            material_number: material_number.into(),
            tenant_id: tenant_id.into(),
            description: description.into(),
            material_type,
            base_unit: base_unit.into(),
            material_group: None,
            gross_weight: None,
            net_weight: None,
            volume: None,
            size_unit: None,
            industry_standard_desc: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        material.validate()?;
        Ok(material)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 更新描述
    pub fn update_description(&mut self, description: impl Into<String>) -> ValidationResult<()> {
        self.description = description.into();
        self.updated_at = Utc::now();
        self.validate()?;
        Ok(())
    }

    /// 设置重量
    pub fn set_weight(&mut self, gross_weight: Quantity, net_weight: Quantity) {
        self.gross_weight = Some(gross_weight);
        self.net_weight = Some(net_weight);
        self.updated_at = Utc::now();
    }

    /// 设置体积
    pub fn set_volume(&mut self, volume: Quantity, unit: UnitOfMeasure) {
        self.volume = Some(volume);
        self.size_unit = Some(unit);
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 物料工厂数据 (MARC)
// =============================================================================

/// 物料工厂数据
///
/// SAP 表 MARC，代表物料在特定工厂的数据。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct MaterialPlantData {
    /// 物料编号 (引用)
    #[validate(non_empty)]
    pub material_number: MaterialNumber,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 工厂代码 (引用)
    #[validate(non_empty)]
    pub plant_code: Plant,

    /// MRP 类型
    pub mrp_type: MrpType,

    /// 采购类型
    pub procurement_type: ProcurementType,

    /// 特殊采购类型
    #[serde(default)]
    pub special_procurement: SpecialProcurementType,

    /// 计划交货时间 (天)
    #[serde(default)]
    pub planned_delivery_time: Option<i32>,

    /// 安全库存
    #[serde(default)]
    pub safety_stock: Option<Quantity>,

    /// 最小批量
    #[serde(default)]
    pub minimum_lot_size: Option<Quantity>,

    /// 最大批量
    #[serde(default)]
    pub maximum_lot_size: Option<Quantity>,

    /// 固定批量
    #[serde(default)]
    pub fixed_lot_size: Option<Quantity>,

    /// 重订货点
    #[serde(default)]
    pub reorder_point: Option<Quantity>,

    /// 生产管理员
    #[validate(length(max = 50))]
    #[serde(default)]
    pub production_supervisor: Option<String>,

    /// ABC 指示符
    #[validate(length(max = 1))]
    #[serde(default)]
    pub abc_indicator: Option<String>,

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

impl MaterialPlantData {
    /// 创建新的物料工厂数据
    pub fn new(
        tenant_id: impl Into<String>,
        material_number: impl Into<MaterialNumber>,
        plant_code: impl Into<Plant>,
        mrp_type: MrpType,
        procurement_type: ProcurementType,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let data = Self {
            material_number: material_number.into(),
            tenant_id: tenant_id.into(),
            plant_code: plant_code.into(),
            mrp_type,
            procurement_type,
            special_procurement: SpecialProcurementType::None,
            planned_delivery_time: None,
            safety_stock: None,
            minimum_lot_size: None,
            maximum_lot_size: None,
            fixed_lot_size: None,
            reorder_point: None,
            production_supervisor: None,
            abc_indicator: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        data.validate()?;
        Ok(data)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 设置安全库存
    pub fn set_safety_stock(&mut self, quantity: Quantity) {
        self.safety_stock = Some(quantity);
        self.updated_at = Utc::now();
    }

    /// 设置重订货点
    pub fn set_reorder_point(&mut self, quantity: Quantity) {
        self.reorder_point = Some(quantity);
        self.updated_at = Utc::now();
    }

    /// 设置批量
    pub fn set_lot_size(&mut self, min: Option<Quantity>, max: Option<Quantity>, fixed: Option<Quantity>) {
        self.minimum_lot_size = min;
        self.maximum_lot_size = max;
        self.fixed_lot_size = fixed;
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// 物料库存地点数据 (MARD)
// =============================================================================

/// 物料库存地点数据
///
/// SAP 表 MARD，代表物料在特定库存地点的数据。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[validate(schema(_))]
pub struct MaterialStorageData {
    /// 物料编号 (引用)
    #[validate(non_empty)]
    pub material_number: MaterialNumber,

    /// 租户ID
    #[validate(non_empty)]
    pub tenant_id: String,

    /// 工厂代码 (引用)
    #[validate(non_empty)]
    pub plant_code: Plant,

    /// 库存地点代码 (引用)
    #[validate(non_empty)]
    pub storage_location: StorageLocationValue,

    /// 非限制使用库存
    pub unrestricted_stock: Quantity,

    /// 质检库存
    #[serde(default)]
    pub quality_inspection_stock: Option<Quantity>,

    /// 冻结库存
    #[serde(default)]
    pub blocked_stock: Option<Quantity>,

    /// 在途库存
    #[serde(default)]
    pub in_transit_stock: Option<Quantity>,

    /// 最后盘点日期
    #[serde(default)]
    pub last_count_date: Option<DateTime<Utc>>,

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

impl MaterialStorageData {
    /// 创建新的物料库存地点数据
    pub fn new(
        tenant_id: impl Into<String>,
        material_number: impl Into<MaterialNumber>,
        plant_code: impl Into<Plant>,
        storage_location: impl Into<StorageLocationValue>,
        unrestricted_stock: Quantity,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let data = Self {
            material_number: material_number.into(),
            tenant_id: tenant_id.into(),
            plant_code: plant_code.into(),
            storage_location: storage_location.into(),
            unrestricted_stock,
            quality_inspection_stock: None,
            blocked_stock: None,
            in_transit_stock: None,
            last_count_date: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        data.validate()?;
        Ok(data)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 获取总库存
    pub fn total_stock(&self) -> Quantity {
        let mut total = self.unrestricted_stock.clone();

        if let Some(qi) = &self.quality_inspection_stock {
            total = Quantity::new(
                total.value() + qi.value(),
                total.unit().clone(),
            ).unwrap_or(total);
        }

        if let Some(blocked) = &self.blocked_stock {
            total = Quantity::new(
                total.value() + blocked.value(),
                total.unit().clone(),
            ).unwrap_or(total);
        }

        total
    }

    /// 获取可用库存 (非限制使用库存)
    pub fn available_stock(&self) -> &Quantity {
        &self.unrestricted_stock
    }

    /// 增加库存
    pub fn increase_stock(&mut self, quantity: Quantity) -> MaterialResult<()> {
        self.unrestricted_stock = Quantity::new(
            self.unrestricted_stock.value() + quantity.value(),
            self.unrestricted_stock.unit().clone(),
        )
        .map_err(|e| MaterialError::ValidationFailed {
            message: format!("Failed to increase stock: {}", e),
        })?;

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 减少库存
    pub fn decrease_stock(&mut self, quantity: Quantity) -> MaterialResult<()> {
        let new_value = self.unrestricted_stock.value() - quantity.value();

        if new_value < 0.0 {
            return Err(MaterialError::InsufficientStock {
                available: self.unrestricted_stock.value(),
                required: quantity.value(),
            });
        }

        self.unrestricted_stock = Quantity::new(
            new_value,
            self.unrestricted_stock.unit().clone(),
        )
        .map_err(|e| MaterialError::ValidationFailed {
            message: format!("Failed to decrease stock: {}", e),
        })?;

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 移动到质检库存
    pub fn move_to_quality_inspection(&mut self, quantity: Quantity) -> MaterialResult<()> {
        self.decrease_stock(quantity.clone())?;

        let qi_stock = self.quality_inspection_stock.get_or_insert_with(|| {
            Quantity::new(0.0, quantity.unit().clone()).unwrap()
        });

        *qi_stock = Quantity::new(
            qi_stock.value() + quantity.value(),
            qi_stock.unit().clone(),
        )
        .map_err(|e| MaterialError::ValidationFailed {
            message: format!("Failed to move to QI: ", e),
        })?;

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 从质检库存释放
    pub fn release_from_quality_inspection(&mut self, quantity: Quantity) -> MaterialResult<()> {
        let qi_stock = self.quality_inspection_stock.as_mut()
            .ok_or_else(|| MaterialError::InsufficientStock {
                available: 0.0,
                required: quantity.value(),
            })?;

        let new_qi_value = qi_stock.value() - quantity.value();
        if new_qi_value < 0.0 {
            return Err(MaterialError::InsufficientStock {
                available: qi_stock.value(),
                required: quantity.value(),
            });
        }

        *qi_stock = Quantity::new(new_qi_value, qi_stock.unit().clone())
            .map_err(|e| MaterialError::ValidationFailed {
                message: format!("Failed to release from QI: {}", e),
            })?;

        self.increase_stock(quantity)?;
        Ok(())
    }

    /// 更新盘点日期
    pub fn update_count_date(&mut self) {
        self.last_count_date = Some(Utc::now());
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
    /// 库存变动
    StockChanged,
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

/// 物料变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 物料编号
    pub material_number: MaterialNumber,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<Material>,
}

/// 物料工厂数据变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialPlantDataChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 物料编号
    pub material_number: MaterialNumber,
    /// 工厂代码
    pub plant_code: Plant,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<MaterialPlantData>,
}

/// 库存变动事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 物料编号
    pub material_number: MaterialNumber,
    /// 工厂代码
    pub plant_code: Plant,
    /// 库存地点
    pub storage_location: StorageLocationValue,
    /// 变动数量
    pub quantity_delta: Quantity,
    /// 变动类型
    pub movement_type: String,
    /// 变动后库存
    pub stock_after: Quantity,
}

// =============================================================================
// Protobuf 导出 (条件编译)
// =============================================================================

#[cfg(feature = "prost")]
pub mod proto {
    include!("material_prost.rs");
}

// =============================================================================
// Re-exports
// =============================================================================

pub use self::material_error::MaterialError;

mod material_error {
    use super::*;

    // 错误类型已在上面定义
}
