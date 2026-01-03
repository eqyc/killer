//! 业务伙伴主数据域
//!
//! 提供客户、供应商、业务伙伴的统一定义。
//! 采用"一库多角色"模式，BusinessPartner 为核心实体，CustomerRole 和 SupplierRole 为角色扩展。
//!
//! # 核心实体
//!
//! - [`BusinessPartner`] - 业务伙伴 (BUT000)
//! - [`CustomerRole`] - 客户角色 (KNA1)
//! - [`SupplierRole`] - 供应商角色 (LFA1)
//!
//! # 示例
//!
//! ```rust
//! use killer_master_data_business_partner::*;
//!
//! let partner = BusinessPartner::new(
//!     "tenant-001",
//!     "BP-001",
//!     "示例公司",
//!     PartnerType::Organization,
//! ).expect("Failed to create business partner");
//!
//! let customer_role = CustomerRole::new(
//!     "tenant-001",
//!     "BP-001",
//!     "1000",  // sales_org
//!     "NET30",
//!     Money::new(100000.0, "CNY").unwrap(),
//! ).expect("Failed to create customer role");
//! ```

#![warn(missing_docs, unreachable_pub)]
#![cfg_attr(feature = "prost", allow(dead_code))]

use chrono::{DateTime, Utc};
use derive_more::{Display, From};
use killer_domain_primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

/// 验证结果类型
pub type ValidationResult<T> = Result<T, validator::ValidationErrors>;

/// 业务伙伴结果类型
pub type BusinessPartnerResult<T> = Result<T, BusinessPartnerError>;

// =============================================================================
// 错误类型
// =============================================================================

/// 业务伙伴域错误
#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessPartnerError {
    #[error("业务伙伴不存在: {id}")]
    PartnerNotFound { id: String },

    #[error("客户角色不存在: {partner_id}")]
    CustomerRoleNotFound { partner_id: String },

    #[error("供应商角色不存在: {partner_id}")]
    SupplierRoleNotFound { partner_id: String },

    #[error("无效的业务伙伴ID: {id}")]
    InvalidPartnerId { id: String },

    #[error("验证失败: {message}")]
    ValidationFailed { message: String },

    #[error("角色冲突: {message}")]
    RoleConflict { message: String },
}

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
// 业务伙伴类型
// =============================================================================

/// 业务伙伴类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerType {
    /// 个人
    Person,
    /// 组织
    Organization,
}

/// 地址信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct Address {
    /// 街道地址
    #[validate(length(max = 200))]
    #[serde(default)]
    pub street: Option<String>,

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
    pub country: String,

    /// 地区/州
    #[serde(default)]
    pub region: Option<String>,
}

impl Address {
    /// 创建新地址
    pub fn new(country: impl Into<String>) -> Self {
        Self {
            street: None,
            city: None,
            postal_code: None,
            country: country.into(),
            region: None,
        }
    }
}

// =============================================================================
// 业务伙伴 (BUT000)
// =============================================================================

/// 业务伙伴
///
/// SAP 表 BUT000，代表客户、供应商或其他业务伙伴的核心实体。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct BusinessPartner {
    /// 业务伙伴ID
    #[validate(length(min = 1))]
    pub id: String,

    /// 租户ID
    #[validate(length(min = 1))]
    pub tenant_id: String,

    /// 业务伙伴名称
    #[validate(length(min = 1, max = 200))]
    pub name: String,

    /// 业务伙伴类型
    pub partner_type: PartnerType,

    /// 地址
    pub address: Address,

    /// 税号
    #[validate(length(max = 50))]
    #[serde(default)]
    pub tax_number: Option<String>,

    /// 语言代码 (ISO 639)
    #[validate(length(min = 2, max = 3))]
    #[serde(default)]
    pub language: Option<String>,

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

impl BusinessPartner {
    /// 创建新的业务伙伴
    pub fn new(
        tenant_id: impl Into<String>,
        id: impl Into<String>,
        name: impl Into<String>,
        partner_type: PartnerType,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let partner = Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            partner_type,
            address: Address::new("CN"),
            tax_number: None,
            language: Some("zh".to_string()),
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        // TODO: 启用验证
        // partner.validate()?;
        Ok(partner)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 更新名称
    pub fn update_name(&mut self, name: impl Into<String>) -> ValidationResult<()> {
        self.name = name.into();
        self.updated_at = Utc::now();
        // TODO: 启用验证
        // self.validate()?;
        Ok(())
    }

    /// 更新地址
    pub fn update_address(&mut self, address: Address) -> ValidationResult<()> {
        // TODO: 启用验证
        // address.validate()?;
        self.address = address;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// =============================================================================
// 阻塞状态
// =============================================================================

/// 阻塞状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockStatus {
    /// 未阻塞
    Unblocked,
    /// 订单阻塞
    OrderBlocked,
    /// 交付阻塞
    DeliveryBlocked,
    /// 开票阻塞
    InvoiceBlocked,
    /// 全部阻塞
    FullyBlocked,
}

impl Default for BlockStatus {
    fn default() -> Self {
        Self::Unblocked
    }
}

// =============================================================================
// 客户角色 (KNA1)
// =============================================================================

/// 客户角色
///
/// SAP 表 KNA1，代表业务伙伴的客户角色扩展。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerRole {
    /// 业务伙伴ID (引用)
    pub partner_id: String,

    /// 租户ID
    pub tenant_id: String,

    /// 销售组织
    pub sales_org: String,

    /// 付款条款
    pub payment_terms: String,

    /// 信用额度
    pub credit_limit: Money,

    /// 阻塞状态
    #[serde(default)]
    pub block_status: BlockStatus,

    /// 客户组
    #[serde(default)]
    pub customer_group: Option<String>,

    /// 价格组
    #[serde(default)]
    pub price_group: Option<String>,

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

impl CustomerRole {
    /// 创建新的客户角色
    pub fn new(
        tenant_id: impl Into<String>,
        partner_id: impl Into<String>,
        sales_org: impl Into<String>,
        payment_terms: impl Into<String>,
        credit_limit: Money,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let role = Self {
            partner_id: partner_id.into(),
            tenant_id: tenant_id.into(),
            sales_org: sales_org.into(),
            payment_terms: payment_terms.into(),
            credit_limit,
            block_status: BlockStatus::Unblocked,
            customer_group: None,
            price_group: None,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        // TODO: 启用验证
        // role.validate()?;
        Ok(role)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 更新信用额度
    pub fn update_credit_limit(&mut self, credit_limit: Money) {
        self.credit_limit = credit_limit;
        self.updated_at = Utc::now();
    }

    /// 设置阻塞状态
    pub fn set_block_status(&mut self, status: BlockStatus) {
        self.block_status = status;
        self.updated_at = Utc::now();
    }

    /// 检查是否被阻塞
    pub fn is_blocked(&self) -> bool {
        self.block_status != BlockStatus::Unblocked
    }
}

// =============================================================================
// 银行账户
// =============================================================================

/// 银行账户
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct BankAccount {
    /// 银行代码
    #[validate(length(max = 20))]
    pub bank_code: String,

    /// 银行账号
    #[validate(length(max = 50))]
    pub account_number: String,

    /// 账户持有人
    #[validate(length(max = 100))]
    pub account_holder: String,

    /// IBAN
    #[validate(length(max = 34))]
    #[serde(default)]
    pub iban: Option<String>,

    /// SWIFT/BIC
    #[validate(length(max = 11))]
    #[serde(default)]
    pub swift_code: Option<String>,

    /// 是否为主账户
    #[serde(default)]
    pub is_primary: bool,
}

impl BankAccount {
    /// 创建新的银行账户
    pub fn new(
        bank_code: impl Into<String>,
        account_number: impl Into<String>,
        account_holder: impl Into<String>,
    ) -> ValidationResult<Self> {
        let account = Self {
            bank_code: bank_code.into(),
            account_number: account_number.into(),
            account_holder: account_holder.into(),
            iban: None,
            swift_code: None,
            is_primary: false,
        };

        // TODO: 启用验证
        // account.validate()?;
        Ok(account)
    }
}

// =============================================================================
// 供应商角色 (LFA1)
// =============================================================================

/// 供应商角色
///
/// SAP 表 LFA1，代表业务伙伴的供应商角色扩展。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplierRole {
    /// 业务伙伴ID (引用)
    pub partner_id: String,

    /// 租户ID
    pub tenant_id: String,

    /// 采购组织
    pub purchasing_org: String,

    /// 银行账户列表
    pub bank_accounts: Vec<BankAccount>,

    /// 统驭科目 (会计科目代码)
    pub reconciliation_account: AccountCode,

    /// 付款条款
    pub payment_terms: String,

    /// 供应商组
    #[serde(default)]
    pub supplier_group: Option<String>,

    /// 阻塞状态
    #[serde(default)]
    pub block_status: BlockStatus,

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

impl SupplierRole {
    /// 创建新的供应商角色
    pub fn new(
        tenant_id: impl Into<String>,
        partner_id: impl Into<String>,
        purchasing_org: impl Into<String>,
        reconciliation_account: AccountCode,
        payment_terms: impl Into<String>,
    ) -> ValidationResult<Self> {
        let now = Utc::now();

        let role = Self {
            partner_id: partner_id.into(),
            tenant_id: tenant_id.into(),
            purchasing_org: purchasing_org.into(),
            bank_accounts: Vec::new(),
            reconciliation_account,
            payment_terms: payment_terms.into(),
            supplier_group: None,
            block_status: BlockStatus::Unblocked,
            created_at: now,
            updated_at: now,
            extensions: Extensions::new(),
            deleted: false,
        };

        // TODO: 启用验证
        // role.validate()?;
        Ok(role)
    }

    /// 标记为已删除
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now();
    }

    /// 添加银行账户
    pub fn add_bank_account(&mut self, account: BankAccount) -> ValidationResult<()> {
        // TODO: 启用验证
        // account.validate()?;
        self.bank_accounts.push(account);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 设置主银行账户
    pub fn set_primary_bank_account(&mut self, index: usize) -> BusinessPartnerResult<()> {
        if index >= self.bank_accounts.len() {
            return Err(BusinessPartnerError::ValidationFailed {
                message: "Invalid bank account index".to_string(),
            });
        }

        for (i, account) in self.bank_accounts.iter_mut().enumerate() {
            account.is_primary = i == index;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// 设置阻塞状态
    pub fn set_block_status(&mut self, status: BlockStatus) {
        self.block_status = status;
        self.updated_at = Utc::now();
    }

    /// 检查是否被阻塞
    pub fn is_blocked(&self) -> bool {
        self.block_status != BlockStatus::Unblocked
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

/// 业务伙伴变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessPartnerChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 业务伙伴ID
    pub partner_id: String,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<BusinessPartner>,
}

/// 客户角色变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerRoleChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 业务伙伴ID
    pub partner_id: String,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<CustomerRole>,
}

/// 供应商角色变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierRoleChangedEvent {
    /// 事件头
    pub header: ChangeEventHeader,
    /// 业务伙伴ID
    pub partner_id: String,
    /// 变更列表
    pub changes: Vec<FieldDelta>,
    /// 完整快照
    pub snapshot: Option<SupplierRole>,
}

// =============================================================================
// Protobuf 导出 (条件编译)
// =============================================================================

#[cfg(feature = "prost")]
pub mod proto {
    include!("business_partner_prost.rs");
}

// =============================================================================
// Re-exports
// =============================================================================

// 错误类型已在上面定义，直接使用

