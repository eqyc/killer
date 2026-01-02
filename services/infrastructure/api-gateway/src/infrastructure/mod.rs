//! 基础设施层
//!
//! 提供外部系统集成：认证、授权、限流、熔断

use async_trait::async_trait;
use std::sync::Arc;

pub mod auth;
pub mod authorization;
pub mod rate_limit;
pub mod circuit_breaker;
pub mod discovery;
pub mod observability;
pub mod proxy;

// 简化的类型重导出
pub use auth::{AuthenticationService, JwtValidator};
pub use authorization::{AuthorizationService, AuthorizationDecision};
pub use rate_limit::RateLimitManager;
pub use circuit_breaker::CircuitBreaker;
pub use observability::{MetricsRegistry, TracingService, init_tracing};

/// 服务发现 Trait
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    async fn get_endpoint(&self, _name: &str) -> Result<super::domain::ServiceEndpoint, super::domain::GatewayError> {
        Err(super::domain::GatewayError::ServiceUnavailable { service: _name.to_string() })
    }
}

/// 简化的负载均衡器
#[derive(Debug, Clone, Default)]
pub struct LoadBalancer;

impl LoadBalancer {
    pub fn select<'a>(&self, instances: &'a [super::domain::ServiceInstance]) -> Option<&'a super::domain::ServiceInstance> {
        instances.first()
    }
}

/// 服务发现管理器
#[derive(Debug, Clone)]
pub struct ServiceDiscoveryManager;

impl ServiceDiscoveryManager {
    pub async fn new(_config: Arc<super::config::DiscoveryConfig>) -> Self {
        Self
    }

    pub async fn get_endpoint(&self, _name: &str) -> Result<super::domain::ServiceEndpoint, super::domain::GatewayError> {
        Err(super::domain::GatewayError::ServiceUnavailable { service: _name.to_string() })
    }
}
