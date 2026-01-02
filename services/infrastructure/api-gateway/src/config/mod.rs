//! 配置管理模块
//!
//! 负责加载和管理 gateway.yaml 配置文件，支持多环境配置覆盖

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, time::Duration};
use url::Url;

/// 网关配置根结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// 服务器配置
    pub server: ServerConfig,
    /// 服务发现配置
    pub discovery: DiscoveryConfig,
    /// 路由规则配置
    pub routes: Vec<RouteConfig>,
    /// 认证配置
    pub authentication: AuthConfig,
    /// 租户配置
    pub tenant: TenantConfig,
    /// 权限配置
    pub authorization: AuthorizationConfig,
    /// 限流配置
    pub rate_limiting: RateLimitingConfig,
    /// 熔断器配置
    pub circuit_breaker: CircuitBreakerConfig,
    /// 超时配置
    pub timeouts: TimeoutConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 可观测性配置
    pub observability: ObservabilityConfig,
    /// 缓存配置
    pub cache: CacheConfig,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            discovery: DiscoveryConfig::default(),
            routes: Vec::new(),
            authentication: AuthConfig::default(),
            tenant: TenantConfig::default(),
            authorization: AuthorizationConfig::default(),
            rate_limiting: RateLimitingConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
            timeouts: TimeoutConfig::default(),
            security: SecurityConfig::default(),
            observability: ObservabilityConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// HTTP 服务监听地址
    pub http_addr: String,
    /// HTTP 服务监听端口
    pub http_port: u16,
    /// HTTPS 服务监听地址
    pub https_addr: Option<String>,
    /// HTTPS 服务监听端口
    pub https_port: Option<u16>,
    /// TLS 证书路径
    pub tls_cert_path: Option<PathBuf>,
    /// TLS 私钥路径
    pub tls_key_path: Option<PathBuf>,
    /// 最大请求体大小 (字节)
    pub max_request_size: u64,
    /// 优雅停机等待时间
    pub graceful_shutdown_timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: "0.0.0.0".to_string(),
            http_port: 8080,
            https_addr: None,
            https_port: None,
            tls_cert_path: None,
            tls_key_path: None,
            max_request_size: 2 * 1024 * 1024, // 2MB
            graceful_shutdown_timeout: 30,
        }
    }
}

/// 服务发现配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// 发现模式: "kubernetes", "static", "consul", "dns"
    pub mode: String,
    /// Kubernetes 配置
    #[cfg(feature = "kube-discovery")]
    pub kubernetes: KubernetesDiscoveryConfig,
    /// 静态服务配置
    pub static_services: HashMap<String, StaticServiceConfig>,
    /// 配置热加载间隔
    pub reload_interval: u64,
    /// 缓存 TTL
    pub cache_ttl: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            mode: "kubernetes".to_string(),
            #[cfg(feature = "kube-discovery")]
            kubernetes: KubernetesDiscoveryConfig::default(),
            static_services: HashMap::new(),
            reload_interval: 30,
            cache_ttl: 60,
        }
    }
}

/// Kubernetes 服务发现配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "kube-discovery")]
pub struct KubernetesDiscoveryConfig {
    /// Kubernetes 命名空间
    pub namespace: String,
    /// 标签选择器
    pub label_selector: String,
    /// Endpoints 刷新间隔
    pub refresh_interval: u64,
    /// 是否启用 TLS
    pub use_tls: bool,
    /// KubeConfig 路径
    pub kubeconfig_path: Option<PathBuf>,
}

#[cfg(feature = "kube-discovery")]
impl Default for KubernetesDiscoveryConfig {
    fn default() -> Self {
        Self {
            namespace: "default".to_string(),
            label_selector: "app.kubernetes.io/part-of=gateway".to_string(),
            refresh_interval: 30,
            use_tls: true,
            kubeconfig_path: None,
        }
    }
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

/// 路由配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// 路由 ID
    pub id: String,
    /// 匹配路径前缀
    pub path_prefix: String,
    /// 路径重写模板
    pub path_rewrite: Option<String>,
    /// 目标服务名称
    pub target_service: String,
    /// 目标路径前缀
    pub target_prefix: Option<String>,
    /// HTTP 方法限制
    pub methods: Vec<String>,
    /// 目标协议: "http", "grpc"
    pub target_protocol: String,
    /// 负载均衡策略
    pub load_balancer: String,
    /// 超时覆盖
    pub timeout: Option<u64>,
    /// 熔断器配置覆盖
    pub circuit_breaker: Option<CircuitBreakerOverride>,
    /// 是否启用
    pub enabled: bool,
    /// 优先级 (数值越大优先级越高)
    pub priority: u32,
    /// 匹配条件
    pub match_conditions: Vec<MatchCondition>,
    /// 响应字段遮蔽规则
    pub response_masking: Option<ResponseMaskingConfig>,
}

impl Default for RouteConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            path_prefix: String::new(),
            path_rewrite: None,
            target_service: String::new(),
            target_prefix: None,
            methods: Vec::new(),
            target_protocol: "http".to_string(),
            load_balancer: "round_robin".to_string(),
            timeout: None,
            circuit_breaker: None,
            enabled: true,
            priority: 0,
            match_conditions: Vec::new(),
            response_masking: None,
        }
    }
}

/// 匹配条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCondition {
    /// 条件类型: "header", "query", "cookie", "claim"
    pub type_: String,
    /// 字段名
    pub field: String,
    /// 匹配操作符: "equals", "contains", "regex", "exists"
    pub operator: String,
    /// 匹配值
    pub value: String,
}

/// 熔断器覆盖配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerOverride {
    /// 失败阈值 (连续失败次数)
    pub failure_threshold: u32,
    /// 成功阈值 (半开状态下需要的成功次数)
    pub success_threshold: u32,
    /// 半开状态最长持续时间
    pub half_open_timeout: u64,
    /// 请求量阈值
    pub volume_threshold: u32,
}

/// 响应字段遮蔽配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMaskingConfig {
    /// 遮蔽规则
    pub rules: Vec<MaskingRule>,
}

/// 遮蔽规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskingRule {
    /// JSON 路径 (e.g., "$.user.ssn")
    pub path: String,
    /// 遮蔽字符
    pub replacement: String,
    /// 角色列表 (仅对这些角色遮蔽)
    pub for_roles: Vec<String>,
}

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// 是否启用认证
    pub enabled: bool,
    /// JWT 配置
    pub jwt: JwtConfig,
    /// API Key 配置
    pub api_key: ApiKeyConfig,
    /// 旁路路径 (无需认证)
    pub bypass_paths: Vec<String>,
}

/// JWT 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtConfig {
    /// JWKS URL
    pub jwks_url: String,
    /// 发行者
    pub issuer: String,
    /// 受众
    pub audience: String,
    /// 缓存 TTL (秒)
    pub cache_ttl: u64,
    /// 刷新间隔 (秒)
    pub refresh_interval: u64,
    /// 允许的算法
    pub allowed_algorithms: Vec<String>,
    /// 声明验证规则
    pub claims_validation: ClaimsValidationConfig,
}

/// 声明验证配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaimsValidationConfig {
    /// 是否验证过期时间
    pub validate_exp: bool,
    /// 是否验证颁发时间
    pub validate_iat: bool,
    /// 是否验证颁发者
    pub validate_iss: bool,
    /// 是否验证受众
    pub validate_aud: bool,
    /// 时钟偏移容差 (秒)
    pub clock_skew_tolerance: u64,
}

/// API Key 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeyConfig {
    /// 是否启用
    pub enabled: bool,
    /// Redis 连接字符串
    pub redis_url: String,
    /// API Key 前缀
    pub key_prefix: String,
    /// 缓存 TTL (秒)
    pub cache_ttl: u64,
}

/// 租户配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantConfig {
    /// 是否强制要求租户上下文
    pub mandatory: bool,
    /// 租户声明字段名
    pub claim_field: String,
    /// 租户 Header 名称
    pub header_name: String,
    /// 默认租户 ID (用于旁路路径)
    pub default_tenant_id: Option<String>,
    /// 租户白名单
    pub whitelist: Vec<String>,
}

/// 权限配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    /// 是否启用
    pub enabled: bool,
    /// 权限声明字段
    pub claim_field: String,
    /// 权限分隔符
    pub scope_delimiter: String,
    /// 默认权限 (旁路路径)
    pub default_permissions: Vec<String>,
    /// RBAC 规则
    pub rbac_rules: Vec<RbacRule>,
}

/// RBAC 规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacRule {
    /// 角色
    pub role: String,
    /// 路径模式 (支持通配符)
    pub path_pattern: String,
    /// HTTP 方法
    pub methods: Vec<String>,
    /// 允许的操作
    pub actions: Vec<String>,
}

/// 限流配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitingConfig {
    /// 是否启用
    pub enabled: bool,
    /// 全局限流配置
    pub global: RateLimitTier,
    /// IP 级别限流
    pub per_ip: RateLimitTier,
    /// 用户级别限流
    pub per_user: RateLimitTier,
    /// API Key 级别限流
    pub per_api_key: RateLimitTier,
    /// 路由级别限流
    pub per_route: HashMap<String, RateLimitTier>,
}

/// 限流层级配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitTier {
    /// 是否启用
    pub enabled: bool,
    /// 桶容量
    pub capacity: u64,
    /// 填充速率 (每秒)
    pub refill_rate: u64,
    /// 突发容量
    pub burst_capacity: u64,
}

/// 熔断器配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    /// 是否启用
    pub enabled: bool,
    /// 默认失败阈值 (连续失败次数)
    pub default_failure_threshold: u32,
    /// 默认成功阈值 (半开状态成功次数)
    pub default_success_threshold: u32,
    /// 默认半开超时
    pub default_half_open_timeout: u64,
    /// 默认请求量阈值
    pub default_volume_threshold: u32,
    /// 路由级别覆盖
    pub route_overrides: HashMap<String, CircuitBreakerOverride>,
}

/// 超时配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeoutConfig {
    /// 默认连接超时
    pub default_connect_timeout: u64,
    /// 默认读取超时
    pub default_read_timeout: u64,
    /// 默认写入超时
    pub default_write_timeout: u64,
    /// 默认总超时
    pub default_total_timeout: u64,
    /// 重试配置
    pub retry: RetryConfig,
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryConfig {
    /// 是否启用自动重试
    pub enabled: bool,
    /// 最大重试次数
    pub max_retries: u32,
    /// 基础退避时间 (毫秒)
    pub base_backoff_ms: u64,
    /// 最大退避时间 (毫秒)
    pub max_backoff_ms: u64,
    /// 可重试的状态码
    pub retryable_status_codes: Vec<u16>,
    /// 可重试的 HTTP 方法
    pub retryable_methods: Vec<String>,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// CORS 配置
    pub cors: CorsConfig,
    /// HSTS 配置
    pub hsts: HstsConfig,
    /// 安全响应头
    pub headers: SecurityHeadersConfig,
    /// IP 白名单
    pub ip_whitelist: Vec<String>,
    /// IP 黑名单
    pub ip_blacklist: Vec<String>,
}

/// CORS 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorsConfig {
    /// 是否启用
    pub enabled: bool,
    /// 允许的来源
    pub allowed_origins: Vec<String>,
    /// 允许的 HTTP 方法
    pub allowed_methods: Vec<String>,
    /// 允许的 HTTP 头
    pub allowed_headers: Vec<String>,
    /// 暴露的 HTTP 头
    pub exposed_headers: Vec<String>,
    /// 凭证支持
    pub allow_credentials: bool,
    /// 预检请求缓存时间 (秒)
    pub max_age: u64,
}

/// HSTS 配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HstsConfig {
    /// 是否启用
    pub enabled: bool,
    /// 预加载
    pub preload: bool,
    /// 包含子域名
    pub include_subdomains: bool,
    /// 最大有效期 (秒)
    pub max_age: u64,
}

/// 安全响应头配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityHeadersConfig {
    /// X-Content-Type-Options
    pub x_content_type_options: String,
    /// X-Frame-Options
    pub x_frame_options: String,
    /// X-XSS-Protection
    pub x_xss_protection: String,
    /// Referrer-Policy
    pub referrer_policy: String,
    /// Permissions-Policy
    pub permissions_policy: String,
}

/// 可观测性配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObservabilityConfig {
    /// 追踪配置
    pub tracing: TracingConfig,
    /// 指标配置
    pub metrics: MetricsConfig,
    /// 审计日志配置
    pub audit: AuditConfig,
}

/// 追踪配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// 是否启用
    pub enabled: bool,
    /// 采样率 (0.0 - 1.0)
    pub sampling_rate: f64,
    /// OTLP 导出端点
    pub otlp_endpoint: String,
    /// 服务名称
    pub service_name: String,
    /// 是否在 HTTP 头中传递追踪信息
    pub propagate_context: bool,
}

/// 指标配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    /// 是否启用
    pub enabled: bool,
    /// Prometheus 端口
    pub prometheus_port: u16,
    /// 路径
    pub path: String,
    /// 桶配置
    pub buckets: Vec<f64>,
}

/// 审计日志配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditConfig {
    /// 是否启用
    pub enabled: bool,
    /// 脱敏字段
    pub sensitive_fields: Vec<String>,
    /// 排除的路径
    pub excluded_paths: Vec<String>,
    /// 仅记录修改的字段
    pub record_changed_fields_only: bool,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    /// JWKS 缓存 TTL
    pub jwks_ttl: u64,
    /// 服务发现缓存 TTL
    pub discovery_ttl: u64,
    /// 权限缓存 TTL
    pub permissions_ttl: u64,
    /// API Key 缓存 TTL
    pub api_key_ttl: u64,
}

// 配置加载器
pub struct ConfigLoader {
    config_path: PathBuf,
    env_prefix: String,
}

impl ConfigLoader {
    /// 创建配置加载器
    pub fn new(config_path: PathBuf, env_prefix: String) -> Self {
        Self {
            config_path,
            env_prefix,
        }
    }

    /// 加载配置
    pub async fn load(&self) -> Result<GatewayConfig, anyhow::Error> {
        // 1. 加载 YAML 配置文件
        let config_content = tokio::fs::read_to_string(&self.config_path).await?;
        let config: GatewayConfig = serde_yaml::from_str(&config_content)?;

        // 2. 加载环境变量覆盖
        self.apply_env_overrides(&mut config)?;

        Ok(config)
    }

    /// 应用环境变量覆盖
    fn apply_env_overrides(&self, config: &mut GatewayConfig) -> Result<(), anyhow::Error> {
        // 服务器配置覆盖
        if let Ok(addr) = std::env::var(format!("{}_SERVER_HTTP_ADDR", self.env_prefix)) {
            config.server.http_addr = addr;
        }
        if let Ok(port) = std::env::var(format!("{}_SERVER_HTTP_PORT", self.env_prefix)) {
            config.server.http_port = port.parse()?;
        }

        // 其他环境变量覆盖...

        Ok(config)
    }

    /// 监听配置变化
    pub async fn watch(&self) -> Result<impl tokio_stream::Stream<Item = ()>, anyhow::Error> {
        let mut rx = tokio::fs::read_to_string(&self.config_path)?;
        let path = self.config_path.clone();

        Ok(tokio_stream::wrappers::WatchStream::new(async_stream::stream! {
            loop {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    if content != rx {
                        rx = content;
                        yield ();
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }))
    }
}

/// 加载配置文件
pub async fn load_config(config_path: PathBuf) -> Result<GatewayConfig, anyhow::Error> {
    let loader = ConfigLoader::new(config_path, "GATEWAY".to_string());
    loader.load().await
}
