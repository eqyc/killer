//! 主数据服务客户端
//!
//! 实现 MasterDataValidator 接口
//! 使用 gRPC 调用 mdg-service，Redis 缓存加速

use crate::domain::ProfitCenter;
use crate::infrastructure::adapters::{AdapterMetrics, MasterDataValidator};
use crate::infrastructure::adapters::redis_cache::RedisCache;
use crate::infrastructure::adapters::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use async_trait::async_trait;
use killer_domain_primitives::{CompanyCode, CostCenter, MaterialNumber, Plant};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, error, info, span, Level, warn};
use uuid::Uuid;

// =============================================================================
// gRPC 客户端配置
// =============================================================================

/// MDG 服务 gRPC 客户端配置
#[derive(Debug, Clone)]
pub struct MasterDataClientConfig {
    /// gRPC 服务器地址
    pub grpc_address: String,
    /// 连接超时
    pub connection_timeout: Duration,
    /// 请求超时
    pub request_timeout: Duration,
    /// 缓存 TTL
    pub cache_ttl: Duration,
    /// 熔断器配置
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

// =============================================================================
// gRPC 客户端实现
// =============================================================================

/// 主数据服务客户端实现
#[derive(Clone)]
pub struct MasterDataClientImpl {
    /// gRPC 客户端（未来使用 tonic 生成）
    /// 目前使用 HTTP REST 客户端模拟
    http_client: reqwest::Client,
    /// 服务地址
    service_address: String,
    /// 缓存
    cache: Arc<RedisCache>,
    /// 指标
    metrics: Arc<AdapterMetrics>,
    /// 熔断器
    circuit_breaker: Arc<CircuitBreaker>,
    /// 配置
    config: MasterDataClientConfig,
}

impl MasterDataClientImpl {
    /// 创建新的客户端
    pub fn new(
        config: MasterDataClientConfig,
        cache: Arc<RedisCache>,
        metrics: Arc<AdapterMetrics>,
    ) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(config.request_timeout)
            .build()
            .expect("Failed to create HTTP client");

        let breaker = Arc::new(CircuitBreaker::new(
            "mdg-service",
            config.circuit_breaker.clone(),
        ));

        Self {
            http_client,
            service_address: config.grpc_address,
            cache,
            metrics,
            circuit_breaker: breaker,
            config,
        }
    }

    /// 获取缓存键
    fn cache_key(&self, prefix: &str, tenant_id: &Uuid, value: &str) -> String {
        format!("mdg:{}:{}:{}", prefix, tenant_id, value)
    }

    /// 调用 MDG 服务 API
    async fn call_api(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.service_address, endpoint);

        let start = std::time::Instant::now();
        let result = async {
            let mut request = self.http_client.request(
                match method {
                    "GET" => reqwest::Method::GET,
                    "POST" => reqwest::Method::POST,
                    _ => reqwest::Method::GET,
                },
                &url,
            );

            if let Some(body) = body {
                request = request.json(&body);
            }

            request.send().await?.json().await
        };

        match timeout(self.config.request_timeout, result).await {
            Ok(Ok(response)) => {
                let duration = start.elapsed();
                self.metrics.record_call("mdg-service", endpoint, true, duration);
                Ok(response)
            }
            Ok(Err(e)) => {
                let duration = start.elapsed();
                self.metrics.record_call("mdg-service", endpoint, false, duration);
                self.metrics.record_error("mdg-service", "request_error");
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
            Err(_) => {
                let duration = start.elapsed();
                self.metrics.record_call("mdg-service", endpoint, false, duration);
                self.metrics.record_error("mdg-service", "timeout");
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Request timeout",
                )))
            }
        }
    }
}

#[async_trait]
impl MasterDataValidator for MasterDataClientImpl {
    /// 验证公司代码是否存在
    async fn validate_company_code(
        &self,
        tenant_id: &Uuid,
        company_code: &CompanyCode,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "ValidateCompanyCode", %tenant_id, %company_code);
        let _guard = span.enter();

        let cache_key = self.cache_key("company_code", tenant_id, company_code.as_str());

        // 1. 先检查缓存
        if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
            self.metrics.record_cache_hit("mdg-service");
            debug!(%cache_key, "Cache hit");
            return Ok(cached == "true");
        }

        self.metrics.record_cache_miss("mdg-service");

        // 2. 调用熔断器保护的服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "GET",
                    &format!(
                        "api/v1/master-data/company-codes/{}/exists",
                        company_code.as_str()
                    ),
                    None,
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists = response
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 3. 写入缓存
                if let Err(e) = self
                    .cache
                    .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                    .await
                {
                    warn!(error = %e, "Failed to cache company code validation");
                }

                debug!(%company_code, exists, "Company code validated");
                Ok(exists)
            }
            Err(e) => {
                error!(error = %e, "Failed to validate company code");
                Err(e)
            }
        }
    }

    /// 验证成本中心是否存在
    async fn validate_cost_center(
        &self,
        tenant_id: &Uuid,
        cost_center: &CostCenter,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "ValidateCostCenter", %tenant_id, %cost_center);
        let _guard = span.enter();

        let cache_key = self.cache_key("cost_center", tenant_id, cost_center.as_str());

        // 检查缓存
        if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
            self.metrics.record_cache_hit("mdg-service");
            return Ok(cached == "true");
        }

        self.metrics.record_cache_miss("mdg-service");

        // 调用服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "GET",
                    &format!(
                        "api/v1/master-data/cost-centers/{}/exists",
                        cost_center.as_str()
                    ),
                    None,
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists = response
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 写入缓存
                let _ = self
                    .cache
                    .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                    .await;

                Ok(exists)
            }
            Err(e) => Err(e),
        }
    }

    /// 验证利润中心是否存在
    async fn validate_profit_center(
        &self,
        tenant_id: &Uuid,
        profit_center: &ProfitCenter,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "ValidateProfitCenter", %tenant_id, %profit_center);
        let _guard = span.enter();

        let cache_key = self.cache_key("profit_center", tenant_id, profit_center.as_str());

        // 检查缓存
        if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
            self.metrics.record_cache_hit("mdg-service");
            return Ok(cached == "true");
        }

        self.metrics.record_cache_miss("mdg-service");

        // 调用服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "GET",
                    &format!(
                        "api/v1/master-data/profit-centers/{}/exists",
                        profit_center.as_str()
                    ),
                    None,
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists = response
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 写入缓存
                let _ = self
                    .cache
                    .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                    .await;

                Ok(exists)
            }
            Err(e) => Err(e),
        }
    }

    /// 验证工厂是否存在
    async fn validate_plant(
        &self,
        tenant_id: &Uuid,
        plant: &Plant,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "ValidatePlant", %tenant_id, %plant);
        let _guard = span.enter();

        let cache_key = self.cache_key("plant", tenant_id, plant.as_str());

        // 检查缓存
        if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
            self.metrics.record_cache_hit("mdg-service");
            return Ok(cached == "true");
        }

        self.metrics.record_cache_miss("mdg-service");

        // 调用服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "GET",
                    &format!("api/v1/master-data/plants/{}/exists", plant.as_str()),
                    None,
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists = response
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 写入缓存
                let _ = self
                    .cache
                    .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                    .await;

                Ok(exists)
            }
            Err(e) => Err(e),
        }
    }

    /// 验证物料是否存在
    async fn validate_material(
        &self,
        tenant_id: &Uuid,
        material: &MaterialNumber,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "ValidateMaterial", %tenant_id, %material);
        let _guard = span.enter();

        let cache_key = self.cache_key("material", tenant_id, material.as_str());

        // 检查缓存
        if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
            self.metrics.record_cache_hit("mdg-service");
            return Ok(cached == "true");
        }

        self.metrics.record_cache_miss("mdg-service");

        // 调用服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "GET",
                    &format!(
                        "api/v1/master-data/materials/{}/exists",
                        material.as_str()
                    ),
                    None,
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists = response
                    .get("exists")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // 写入缓存
                let _ = self
                    .cache
                    .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                    .await;

                Ok(exists)
            }
            Err(e) => Err(e),
        }
    }

    /// 批量验证公司代码
    async fn batch_validate_company_codes(
        &self,
        tenant_id: &Uuid,
        company_codes: &[CompanyCode],
    ) -> Result<Vec<(CompanyCode, bool)>, Box<dyn std::error::Error>> {
        let span = span!(Level::DEBUG, "BatchValidateCompanyCodes", %tenant_id, count = company_codes.len());
        let _guard = span.enter();

        let mut results = Vec::new();
        let mut uncached = Vec::new();

        // 1. 先检查缓存
        for code in company_codes {
            let cache_key = self.cache_key("company_code", tenant_id, code.as_str());
            if let Ok(Some(cached)) = self.cache.get(&cache_key).await {
                self.metrics.record_cache_hit("mdg-service");
                results.push((code.clone(), cached == "true"));
            } else {
                self.metrics.record_cache_miss("mdg-service");
                uncached.push(code.clone());
            }
        }

        if uncached.is_empty() {
            return Ok(results);
        }

        // 2. 批量调用服务
        let result = self
            .circuit_breaker
            .call(|| async {
                self.call_api(
                    "POST",
                    "api/v1/master-data/company-codes/batch-exists",
                    Some(serde_json::json!({
                        "company_codes": uncached.iter().map(|c| c.as_str().to_string()).collect::<Vec<_>>()
                    })),
                )
                .await
            })
            .await;

        match result {
            Ok(response) => {
                let exists_map: std::collections::HashMap<String, bool> = response
                    .get("results")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();

                // 3. 处理结果并更新缓存
                for code in uncached {
                    let exists = exists_map.get(code.as_str()).copied().unwrap_or(false);
                    results.push((code.clone(), exists));

                    let cache_key = self.cache_key("company_code", tenant_id, code.as_str());
                    let _ = self
                        .cache
                        .set(&cache_key, if exists { "true" } else { "false" }, self.config.cache_ttl)
                        .await;
                }

                Ok(results)
            }
            Err(e) => Err(e),
        }
    }
}
