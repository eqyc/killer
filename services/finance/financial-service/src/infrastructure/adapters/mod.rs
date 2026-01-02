//! 外部服务适配器模块
//!
//! 提供对外部服务的集成适配器：
//! - MasterDataClient: 主数据服务 gRPC 客户端
//! - MaterialsEventSubscriber: 物料服务事件订阅器
//!
//! ## 设计原则
//!
//! 1. 所有适配器实现 trait 接口，注入到应用层
//! 2. 外部调用使用熔断器保护
//! 3. 支持缓存以减少外部调用
//! 4. 完整的 metrics 和 tracing

#[cfg(feature = "adapters")]
pub mod master_data_client;

#[cfg(feature = "adapters")]
pub mod materials_event_subscriber;

#[cfg(feature = "adapters")]
pub mod circuit_breaker;

#[cfg(feature = "adapters")]
pub mod redis_cache;

#[cfg(feature = "adapters")]
pub use master_data_client::MasterDataClientImpl;

#[cfg(feature = "adapters")]
pub use materials_event_subscriber::MaterialsEventSubscriber;

// =============================================================================
// 共享类型和 trait 定义
// =============================================================================

use async_trait::async_trait;
use killer_domain_primitives::{CompanyCode, CostCenter, MaterialNumber, Plant, ProfitCenter};
use std::sync::Arc;
use uuid::Uuid;

// =============================================================================
// 主数据验证 trait
// =============================================================================

/// 主数据验证接口
#[async_trait]
pub trait MasterDataValidator: Send + Sync {
    /// 验证公司代码是否存在
    async fn validate_company_code(&self, tenant_id: &Uuid, company_code: &CompanyCode) -> Result<bool, Box<dyn std::error::Error>>;

    /// 验证成本中心是否存在
    async fn validate_cost_center(&self, tenant_id: &Uuid, cost_center: &CostCenter) -> Result<bool, Box<dyn std::error::Error>>;

    /// 验证利润中心是否存在
    async fn validate_profit_center(&self, tenant_id: &Uuid, profit_center: &ProfitCenter) -> Result<bool, Box<dyn std::error::Error>>;

    /// 验证工厂是否存在
    async fn validate_plant(&self, tenant_id: &Uuid, plant: &Plant) -> Result<bool, Box<dyn std::error::Error>>;

    /// 验证物料是否存在
    async fn validate_material(&self, tenant_id: &Uuid, material: &MaterialNumber) -> Result<bool, Box<dyn std::error::Error>>;

    /// 批量验证公司代码
    async fn batch_validate_company_codes(
        &self,
        tenant_id: &Uuid,
        company_codes: &[CompanyCode],
    ) -> Result<Vec<(CompanyCode, bool)>, Box<dyn std::error::Error>>;
}

// =============================================================================
// 物料凭证事件
// =============================================================================

/// 物料凭证事件（从 materials-service 接收）
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MaterialDocumentPostedEvent {
    /// 事件元数据
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub correlation_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// 物料凭证信息
    pub material_document_number: String,
    pub company_code: String,
    pub posting_date: chrono::NaiveDate,
    pub document_date: chrono::NaiveDate,

    /// 行项目
    pub items: Vec<MaterialDocumentItem>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MaterialDocumentItem {
    pub line_number: u32,
    pub material_number: String,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub amount: f64,
    pub currency_code: String,
    pub cost_center: Option<String>,
    pub order_number: Option<String>,
    pub movement_type: String, // "101" for GR, "102" for GR reversal
}

// =============================================================================
// 适配器指标
// =============================================================================

/// 适配器指标
#[derive(Default)]
pub struct AdapterMetrics {
    external_calls_total: prometheus::IntCounterVec,
    external_call_duration: prometheus::HistogramVec,
    cache_hits_total: prometheus::IntCounterVec,
    cache_misses_total: prometheus::IntCounterVec,
    circuit_breaker_state: prometheus::GaugeVec,
    errors_total: prometheus::IntCounterVec,
}

impl AdapterMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            external_calls_total: prometheus::register_int_counter_vec!(
                "adapter_external_calls_total",
                "Total external adapter calls",
                &["service", "method", "status"]
            )?,
            external_call_duration: prometheus::register_histogram_vec!(
                "adapter_external_call_duration_seconds",
                "External adapter call duration",
                &["service", "method"]
            )?,
            cache_hits_total: prometheus::register_int_counter_vec!(
                "adapter_cache_hits_total",
                "Cache hits",
                &["service"]
            )?,
            cache_misses_total: prometheus::register_int_counter_vec!(
                "adapter_cache_misses_total",
                "Cache misses",
                &["service"]
            )?,
            circuit_breaker_state: prometheus::register_gauge_vec!(
                "adapter_circuit_breaker_state",
                "Circuit breaker state (0=closed, 1=half-open, 2=open)",
                &["service"]
            )?,
            errors_total: prometheus::register_int_counter_vec!(
                "adapter_errors_total",
                "Total adapter errors",
                &["service", "error_type"]
            )?,
        })
    }

    pub fn record_call(&self, service: &str, method: &str, success: bool, duration: std::time::Duration) {
        let status = if success { "success" } else { "failure" };
        self.external_calls_total
            .with_label_values(&[service, method, status])
            .inc();
        self.external_call_duration
            .with_label_values(&[service, method])
            .observe(duration.as_secs_f64());
    }

    pub fn record_cache_hit(&self, service: &str) {
        self.cache_hits_total.with_label_values(&[service]).inc();
    }

    pub fn record_cache_miss(&self, service: &str) {
        self.cache_misses_total.with_label_values(&[service]).inc();
    }

    pub fn record_error(&self, service: &str, error_type: &str) {
        self.errors_total
            .with_label_values(&[service, error_type])
            .inc();
    }
}
