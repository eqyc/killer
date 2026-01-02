//! 健康检查模块

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

impl HealthStatus {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            checks: vec![
                HealthCheck {
                    name: "database".to_string(),
                    status: "healthy".to_string(),
                    message: None,
                },
                HealthCheck {
                    name: "kafka".to_string(),
                    status: "healthy".to_string(),
                    message: None,
                },
                HealthCheck {
                    name: "redis".to_string(),
                    status: "healthy".to_string(),
                    message: None,
                },
            ],
        }
    }
}
