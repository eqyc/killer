//! 领域核心类型定义
//!
//! 定义网关运行时的核心数据结构

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc};
use uuid::Uuid;

/// 认证类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthType {
    /// JWT 认证
    Jwt,
    /// API Key 认证
    ApiKey,
    /// 匿名访问
    Anonymous,
}

/// 认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationInfo {
    /// 认证类型
    pub auth_type: AuthType,
    /// 主体 ID
    pub subject_id: String,
    /// 租户 ID
    pub tenant_id: Option<String>,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 角色
    pub roles: Vec<String>,
    /// 声明
    pub claims: HashMap<String, serde_json::Value>,
    /// 令牌过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 认证时间
    pub authenticated_at: DateTime<Utc>,
}

impl Default for AuthenticationInfo {
    fn default() -> Self {
        Self {
            auth_type: AuthType::Anonymous,
            subject_id: "anonymous".to_string(),
            tenant_id: None,
            scopes: Vec::new(),
            roles: Vec::new(),
            claims: HashMap::new(),
            expires_at: None,
            authenticated_at: Utc::now(),
        }
    }
}

/// 租户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantInfo {
    /// 租户 ID
    pub id: String,
    /// 租户名称
    pub name: String,
    /// 租户类型
    pub tenant_type: String,
    /// 配额配置
    pub quota: TenantQuota,
    /// 状态
    pub status: TenantStatus,
}

/// 租户配额
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantQuota {
    /// API 调用配额 (每小时)
    pub api_calls_per_hour: u64,
    /// 带宽限制 (MB)
    pub bandwidth_limit_mb: u64,
    /// 并发请求限制
    pub concurrent_requests: u32,
}

/// 租户状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TenantStatus {
    Active,
    Suspended,
    Trial,
    Expired,
}

/// 路由匹配结果
#[derive(Debug, Clone)]
pub struct RouteMatch {
    /// 路由配置
    pub route: Arc<crate::config::RouteConfig>,
    /// 提取的参数
    pub params: HashMap<String, String>,
    /// 匹配的分量
    pub matched_segments: Vec<String>,
}

/// 目标服务实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// 实例 ID
    pub id: String,
    /// 服务名称
    pub service_name: String,
    /// 实例地址
    pub address: String,
    /// 端口
    pub port: u16,
    /// 协议
    pub protocol: String,
    /// 权重
    pub weight: u32,
    /// 健康状态
    pub healthy: bool,
    /// 最后健康检查时间
    pub last_health_check: Option<DateTime<Utc>>,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

impl ServiceInstance {
    /// 获取完整 URL
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.address, self.port)
    }

    /// 获取健康得分 (用于负载均衡)
    pub fn health_score(&self) -> u32 {
        if !self.healthy {
            return 0;
        }
        // 基础分数 100，减去延迟权重
        100
    }
}

/// 服务端点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// 服务名称
    pub name: String,
    /// 协议
    pub protocol: String,
    /// 实例列表
    pub instances: Vec<ServiceInstance>,
    /// 负载均衡策略
    pub load_balancer: LoadBalancerStrategy,
    /// 最后更新
    pub updated_at: DateTime<Utc>,
}

/// 负载均衡策略
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadBalancerStrategy {
    /// 轮询
    RoundRobin,
    /// 加权轮询
    WeightedRoundRobin,
    /// 最少连接
    LeastConnections,
    /// 随机
    Random,
    /// IP 哈希
    IpHash,
    /// 一致性哈希
    ConsistentHash(String),
}

impl Default for LoadBalancerStrategy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

/// 熔断器状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    /// 关闭状态，正常请求
    Closed,
    /// 打开状态，快速失败
    Open,
    /// 半开状态，探测恢复
    HalfOpen,
}

/// 熔断器指标
#[derive(Debug, Clone)]
pub struct CircuitBreakerMetrics {
    /// 当前状态
    pub state: CircuitBreakerState,
    /// 失败请求数
    pub failure_count: u32,
    /// 成功请求数
    pub success_count: u32,
    /// 请求拒绝数
    pub rejected_count: u64,
    /// 最后失败时间
    pub last_failure_time: Option<DateTime<Utc>>,
    /// 最后成功时间
    pub last_success_time: Option<DateTime<Utc>>,
}

/// 限流决策
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RateLimitDecision {
    /// 允许
    Allowed,
    /// 拒绝 (超出限流)
    RateLimited,
    /// 拒绝 (配额耗尽)
    QuotaExceeded,
}

/// 限流结果
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// 决策
    pub decision: RateLimitDecision,
    /// 剩余配额
    pub remaining: i64,
    /// 重试-after 秒数
    pub retry_after: Option<u64>,
    /// 限流桶名称
    pub limit_name: String,
}

/// 请求上下文
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// 请求 ID (追踪 ID)
    pub request_id: Uuid,
    /// 追踪 ID
    pub trace_id: String,
    /// 跨度 ID
    pub span_id: String,
    /// 客户端 IP
    pub client_ip: String,
    /// 客户端端口
    pub client_port: u16,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 认证信息
    pub authentication: AuthenticationInfo,
    /// 租户信息
    pub tenant: Option<TenantInfo>,
    /// 原始路径
    pub original_path: String,
    /// 原始查询字符串
    pub original_query: Option<String>,
    /// 起始时间
    pub start_time: DateTime<Utc>,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

/// 请求信息 (不可变引用)
#[derive(Debug, Clone)]
pub struct RequestInfo {
    /// HTTP 方法
    pub method: String,
    /// 路径
    pub path: String,
    /// 查询字符串
    pub query_string: Option<String>,
    /// 路径参数
    pub path_params: HashMap<String, String>,
    /// 请求头
    pub headers: HashMap<String, String>,
    /// 请求体大小
    pub body_size: u64,
}

/// 代理响应
#[derive(Debug, Clone)]
pub struct ProxyResponse {
    /// HTTP 状态码
    pub status_code: u16,
    /// 响应头
    pub headers: HashMap<String, String>,
    /// 响应体
    pub body: bytes::Bytes,
    /// 响应体大小
    pub body_size: u64,
    /// 响应时间 (毫秒)
    pub response_time_ms: u64,
    /// 目标实例
    pub target_instance: Option<ServiceInstance>,
    /// 是否来自缓存
    pub from_cache: bool,
    /// 熔断器状态
    pub circuit_breaker_state: CircuitBreakerState,
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// 日志 ID
    pub id: Uuid,
    /// 请求 ID
    pub request_id: Uuid,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 租户 ID
    pub tenant_id: Option<String>,
    /// 用户 ID
    pub user_id: Option<String>,
    /// 操作类型
    pub action: String,
    /// 资源类型
    pub resource_type: String,
    /// 资源 ID
    pub resource_id: Option<String>,
    /// 请求路径
    pub path: String,
    /// HTTP 方法
    pub method: String,
    /// 状态码
    pub status_code: u16,
    /// 持续时间 (毫秒)
    pub duration_ms: u64,
    /// 客户端 IP
    pub client_ip: String,
    /// 更改前的值 (脱敏)
    pub before_value: Option<serde_json::Value>,
    /// 更改后的值 (脱敏)
    pub after_value: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

/// 网关指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMetrics {
    /// 请求总数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 4xx 错误数
    pub client_errors: u64,
    /// 5xx 错误数
    pub server_errors: u64,
    /// 总延迟 (毫秒)
    pub total_latency_ms: u64,
    /// 平均延迟 (毫秒)
    pub avg_latency_ms: u64,
    /// P99 延迟 (毫秒)
    pub p99_latency_ms: u64,
    /// 限流拒绝数
    pub rate_limited_count: u64,
    /// 熔断器拒绝数
    pub circuit_breaker_rejected_count: u64,
    /// 认证失败数
    pub auth_failed_count: u64,
    /// 授权失败数
    pub authz_failed_count: u64,
}

/// 错误类型
#[derive(Debug, Clone, thiserror::Error)]
pub enum GatewayError {
    #[error("路由未找到: {path}")]
    RouteNotFound { path: String },

    #[error("服务不可用: {service}")]
    ServiceUnavailable { service: String },

    #[error("认证失败: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("授权失败: {permission}")]
    AuthorizationFailed { permission: String },

    #[error("租户无效: {tenant_id}")]
    InvalidTenant { tenant_id: String },

    #[error("限流触发: {limit_name}")]
    RateLimited { limit_name: String, retry_after: u64 },

    #[error("熔断器打开")]
    CircuitBreakerOpen,

    #[error("请求超时: {timeout}")]
    RequestTimeout { timeout: u64 },

    #[error("上游服务错误: {status_code}")]
    UpstreamError { status_code: u16 },

    #[error("内部错误: {message}")]
    Internal { message: String },

    #[error("配置错误: {message}")]
    ConfigError { message: String },
}

/// 网关结果类型
pub type GatewayResult<T> = std::result::Result<T, GatewayError>;
