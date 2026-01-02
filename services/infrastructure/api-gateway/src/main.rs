//! KILLER ERP API Gateway - 核心库
//!
//! 企业级 API 网关 - 统一流量入口、认证授权、路由转发、限流熔断

#![warn(missing_docs)]

// 模块声明
mod config;
mod domain;
mod infrastructure;
mod application;
mod interface;

// 重新导出主要类型
pub use config::GatewayConfig;
pub use domain::{AuthenticationInfo, GatewayError, GatewayResult, TenantInfo, TenantQuota};
pub use application::GatewayService;
pub use infrastructure::{ServiceDiscovery, LoadBalancer, RateLimitManager, CircuitBreaker};

use std::path::PathBuf;
use tracing::info;

/// 加载配置
pub async fn load_config(config_path: Option<PathBuf>) -> Result<std::sync::Arc<GatewayConfig>, anyhow::Error> {
    let path = config_path.unwrap_or_else(|| PathBuf::from("config/gateway.yaml"));

    if !path.exists() {
        info!("Config file not found, using default configuration");
        return Ok(std::sync::Arc::new(GatewayConfig::default()));
    }

    let content = tokio::fs::read_to_string(&path).await?;
    let config: GatewayConfig = serde_yaml::from_str(&content)?;
    Ok(std::sync::Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_config_loading() {
        let config = load_config(None).await;
        assert!(config.is_ok());
    }

    #[tokio::test]
    async fn test_gateway_creation() {
        let config = load_config(None).await.unwrap();
        let _gateway = GatewayService::new(config).await;
        // GatewayService::new returns Self, not Result
    }
}
