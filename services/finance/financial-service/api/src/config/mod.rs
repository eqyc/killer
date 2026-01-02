//! API 层配置
//!
//! 从 config.yaml 或环境变量加载配置

use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;

/// API 配置
#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    /// gRPC 服务器配置
    pub grpc: GrpcConfig,

    /// HTTP 服务器配置
    pub http: HttpConfig,

    /// 认证配置
    pub auth: AuthConfig,

    /// 幂等性配置
    pub idempotency: IdempotencyConfig,

    /// 审计配置
    pub audit: AuditConfig,

    /// 指标配置
    pub metrics: MetricsConfig,
}

/// gRPC 服务器配置
#[derive(Debug, Clone, Deserialize)]
pub struct GrpcConfig {
    /// 监听地址
    pub listen_addr: String,

    /// 最大连接数
    pub max_concurrent_requests: usize,

    /// 请求超时
    pub request_timeout: Duration,

    /// 最大消息大小（MB）
    pub max_message_size: usize,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:50051".to_string(),
            max_concurrent_requests: 1000,
            request_timeout: Duration::from_secs(30),
            max_message_size: 4,
        }
    }
}

/// HTTP 服务器配置
#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    /// 监听地址
    pub listen_addr: String,

    /// 是否启用
    pub enabled: bool,

    /// CORS 配置
    pub cors: CorsConfig,

    /// Swagger UI 路径
    pub swagger_path: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:8080".to_string(),
            enabled: true,
            cors: CorsConfig::default(),
            swagger_path: "/swagger-ui".to_string(),
        }
    }
}

/// CORS 配置
#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    /// 允许的来源
    pub allowed_origins: Vec<String>,

    /// 允许的方法
    pub allowed_methods: Vec<String>,

    /// 允许的头
    pub allowed_headers: Vec<String>,

    /// 暴露的头
    pub expose_headers: Vec<String>,

    /// 最大年龄（秒）
    pub max_age: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string(), "X-Tenant-Id".to_string()],
            expose_headers: vec!["X-Trace-Id".to_string()],
            max_age: 3600,
        }
    }
}

/// 认证配置
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// JWT 密钥
    pub jwt_secret: String,

    /// JWT 公开密钥（用于验证 RS256）
    pub jwt_public_key: Option<String>,

    /// 允许的签发者
    pub allowed_issuers: Vec<String>,

    /// 允许的受众
    pub allowed_audiences: Vec<String>,

    /// 令牌过期时间（秒）
    pub token_expiry: u64,

    /// 跳过认证（开发模式）
    pub skip_auth: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "default-secret-change-in-production".to_string(),
            jwt_public_key: None,
            allowed_issuers: vec!["killer-erp".to_string()],
            allowed_audiences: vec!["financial-service".to_string()],
            token_expiry: 3600,
            skip_auth: false,
        }
    }
}

/// 幂等性配置
#[derive(Debug, Clone, Deserialize)]
pub struct IdempotencyConfig {
    /// 是否启用
    pub enabled: bool,

    /// TTL（小时）
    pub ttl_hours: i64,

    /// Redis 前缀
    pub key_prefix: String,
}

impl Default for IdempotencyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_hours: 24,
            key_prefix: "killer:finance:api".to_string(),
        }
    }
}

/// 审计配置
#[derive(Debug, Clone, Deserialize)]
pub struct AuditConfig {
    /// 是否启用
    pub enabled: bool,

    /// 存储类型: "database" | "kafka"
    pub storage_type: String,

    /// Kafka 主题
    pub kafka_topic: String,

    /// 启用审计的操作
    pub enabled_actions: Vec<String>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_type: "database".to_string(),
            kafka_topic: "killer.audit.events".to_string(),
            enabled_actions: vec![
                "create".to_string(),
                "post".to_string(),
                "reverse".to_string(),
                "delete".to_string(),
            ],
        }
    }
}

/// 指标配置
#[derive(Debug, Clone, Deserialize)]
pub struct MetricsConfig {
    /// 是否启用
    pub enabled: bool,

    /// 路径
    pub path: String,

    /// 端口
    pub port: u16,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: "/metrics".to_string(),
            port: 9090,
        }
    }
}

impl ApiConfig {
    /// 从配置文件加载
    pub fn from_config_file(path: &str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }

    /// 获取 gRPC 监听地址
    pub fn grpc_addr(&self) -> SocketAddr {
        self.grpc.listen_addr.parse().unwrap_or_else(|_| {
            SocketAddr::from(([0, 0, 0, 0], 50051))
        })
    }

    /// 获取 HTTP 监听地址
    pub fn http_addr(&self) -> SocketAddr {
        self.http.listen_addr.parse().unwrap_or_else(|_| {
            SocketAddr::from(([0, 0, 0, 0], 8080))
        })
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            grpc: GrpcConfig::default(),
            http: HttpConfig::default(),
            auth: AuthConfig::default(),
            idempotency: IdempotencyConfig::default(),
            audit: AuditConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}
