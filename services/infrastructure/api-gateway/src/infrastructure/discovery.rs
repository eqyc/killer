//! 服务发现模块
//!
//! 支持 Kubernetes Endpoints 和静态配置两种服务发现方式

use crate::domain::{GatewayError, GatewayResult, ServiceEndpoint, ServiceInstance, LoadBalancerStrategy};
use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use url::Url;

/// 服务发现 Trait
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    /// 获取服务终端节点
    async fn get_endpoint(&self, service_name: &str) -> GatewayResult<ServiceEndpoint>;
    /// 列出所有服务
    async fn list_services(&self) -> GatewayResult<Vec<String>>;
    /// 健康检查
    async fn health_check(&self) -> bool;
}

/// 静态服务配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StaticServiceConfig {
    /// 服务名称
    pub name: String,
    /// 服务 URL
    pub url: Url,
    /// 服务协议: "http", "https", "grpc", "grpcs"
    pub protocol: String,
    /// 权重 (用于负载均衡)
    pub weight: u32,
    /// 健康检查 URL
    pub health_check_url: Option<Url>,
    /// 超时时间
    pub timeout: Option<u64>,
}

/// 静态服务发现 (基于配置文件)
#[derive(Debug, Clone)]
pub struct StaticServiceDiscovery {
    /// 服务配置
    services: Arc<DashMap<String, StaticServiceConfig>>,
    /// 缓存的实例
    instances: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    /// 缓存 TTL
    cache_ttl: Duration,
}

impl StaticServiceDiscovery {
    /// 创建静态服务发现
    pub fn new(services: HashMap<String, StaticServiceConfig>, cache_ttl: u64) -> Self {
        let map = DashMap::new();
        for (name, config) in services {
            map.insert(name, config);
        }

        Self {
            services: Arc::new(map),
            instances: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(cache_ttl),
        }
    }

    /// 获取服务配置
    pub fn get_service(&self, name: &str) -> Option<StaticServiceConfig> {
        self.services.get(name).map(|c| c.clone())
    }

    /// 添加或更新服务
    pub fn add_service(&self, name: String, config: StaticServiceConfig) {
        self.services.insert(name, config);
    }
}

#[async_trait]
impl ServiceDiscovery for StaticServiceDiscovery {
    async fn get_endpoint(&self, service_name: &str) -> GatewayResult<ServiceEndpoint> {
        // 检查缓存
        if let Some(instances) = self.instances.read().await.get(service_name) {
            if let Some(instance) = instances.first() {
                return Ok(ServiceEndpoint {
                    service_name: service_name.to_string(),
                    instance: instance.clone(),
                    load_balancer: LoadBalancerStrategy::RoundRobin,
                });
            }
        }

        // 获取服务配置
        let config = self.get_service(service_name)
            .ok_or_else(|| GatewayError::ServiceUnavailable {
                service: service_name.to_string(),
            })?;

        // 创建实例
        let instance = ServiceInstance {
            id: format!("{}-0", service_name),
            service_name: service_name.to_string(),
            address: config.url.host_str().unwrap_or("").to_string(),
            port: config.url.port().unwrap_or(80),
            protocol: config.protocol.clone(),
            weight: config.weight,
            healthy: true,
            last_health_check: Some(Utc::now()),
            metadata: HashMap::new(),
        };

        // 缓存实例
        self.instances.write().await.insert(
            service_name.to_string(),
            vec![instance.clone()],
        );

        Ok(ServiceEndpoint {
            service_name: service_name.to_string(),
            instance,
            load_balancer: LoadBalancerStrategy::RoundRobin,
        })
    }

    async fn list_services(&self) -> GatewayResult<Vec<String>> {
        Ok(self.services.iter().map(|r| r.key().clone()).collect())
    }

    async fn health_check(&self) -> bool {
        true
    }
}

/// 简化的服务发现管理器
#[derive(Debug, Clone)]
pub struct ServiceDiscoveryManager {
    /// 静态发现
    static_discovery: Option<StaticServiceDiscovery>,
}

impl ServiceDiscoveryManager {
    /// 创建服务发现管理器
    pub async fn new(static_services: Option<HashMap<String, StaticServiceConfig>>, cache_ttl: u64) -> Self {
        let static_discovery = static_services.map(|s| StaticServiceDiscovery::new(s, cache_ttl));

        Self { static_discovery }
    }

    /// 获取服务终端节点
    pub async fn get_endpoint(&self, service_name: &str) -> GatewayResult<ServiceEndpoint> {
        if let Some(ref discovery) = self.static_discovery {
            return discovery.get_endpoint(service_name).await;
        }

        Err(GatewayError::ServiceUnavailable {
            service: service_name.to_string(),
        })
    }

    /// 列出所有服务
    pub async fn list_services(&self) -> GatewayResult<Vec<String>> {
        if let Some(ref discovery) = self.static_discovery {
            return discovery.list_services().await;
        }
        Ok(Vec::new())
    }
}
