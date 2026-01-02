//! 基础设施层
//!
//! 为领域层和应用层提供底层支撑，包括：
//! - 持久化（PostgreSQL）
//! - 消息发布（Kafka）
//! - 外部服务适配器
//! - 读模型投影（ClickHouse）
//!
//! ## 设计原则
//!
//! 1. **基础设施隔离**：仅实现 trait 接口，不依赖领域模型内部逻辑
//! 2. **多租户支持**：所有操作强制包含 tenant_id
//! 3. **最终一致性**：使用 Transactional Outbox 模式
//! 4. **性能与韧性**：连接池、超时、重试、熔断
//! 5. **可观测性**：tracing、metrics、日志

#[cfg(feature = "persistence")]
pub mod persistence;

#[cfg(feature = "messaging")]
pub mod messaging;

#[cfg(feature = "adapters")]
pub mod adapters;

#[cfg(feature = "projection")]
pub mod projection;

// =============================================================================
// 配置
// =============================================================================

/// 基础设施配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct InfrastructureConfig {
    /// PostgreSQL 配置
    #[cfg(feature = "persistence")]
    pub postgres: PostgresConfig,

    /// Kafka 配置
    #[cfg(feature = "messaging")]
    pub kafka: KafkaConfig,

    /// Redis 配置
    pub redis: RedisConfig,

    /// ClickHouse 配置
    #[cfg(feature = "projection")]
    pub clickhouse: ClickHouseConfig,
}

/// PostgreSQL 配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostgresConfig {
    /// 连接字符串
    pub url: String,

    /// 最大连接数
    pub max_connections: u32,

    /// 连接超时（秒）
    pub connection_timeout: u64,

    /// 空闲连接超时（秒）
    pub idle_timeout: u64,

    /// 最大生命周期（秒）
    pub max_lifetime: u64,

    /// SSL 模式
    pub ssl_mode: SslMode,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SslMode {
    Disable,
    Prefer,
    Require,
}

/// Kafka 配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct KafkaConfig {
    /// broker 地址
    pub brokers: Vec<String>,

    /// 客户端 ID
    pub client_id: String,

    /// 消费者组 ID
    pub consumer_group_id: String,

    /// SASL 认证
    pub sasl: Option<KafkaSaslConfig>,

    /// SSL 配置
    pub ssl: Option<KafkaSslConfig>,

    /// 生产者配置
    pub producer: KafkaProducerConfig,

    /// 消费者配置
    pub consumer: KafkaConsumerConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct KafkaSaslConfig {
    pub mechanism: String, // "SCRAM-SHA-256" 或 "SCRAM-SHA-512"
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct KafkaSslConfig {
    pub ca_location: String,
    pub certificate_location: String,
    pub key_location: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct KafkaProducerConfig {
    /// acks 模式
    pub acks: String, // "all", "1", "0"
    /// 重试次数
    pub retries: i32,
    /// 批量大小
    pub batch_size: usize,
    /// linger_ms
    pub linger_ms: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct KafkaConsumerConfig {
    /// 自动提交
    pub enable_auto_commit: bool,
    /// 自动提交间隔
    pub auto_commit_interval_ms: u32,
    /// 会话超时
    pub session_timeout_ms: u32,
    /// 最大轮询记录数
    pub max_poll_records: usize,
}

/// Redis 配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RedisConfig {
    /// 连接字符串
    pub url: String,

    /// 最大连接数
    pub max_connections: u32,

    /// 操作超时（毫秒）
    pub command_timeout: u64,

    /// 读超时（毫秒）
    pub read_timeout: u64,

    /// 写超时（毫秒）
    pub write_timeout: u64,
}

/// ClickHouse 配置
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClickHouseConfig {
    /// 连接字符串
    pub url: String,

    /// 数据库名
    pub database: String,

    /// 用户名
    pub username: String,

    /// 密码
    pub password: String,

    /// 最大连接数
    pub max_connections: u32,

    /// 操作超时（秒）
    pub command_timeout: u64,
}

// =============================================================================
// 工厂函数
// =============================================================================

/// 从配置文件加载配置
pub async fn load_config(config_path: &str) -> Result<InfrastructureConfig, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: InfrastructureConfig = toml::from_str(&config_content)?;
    Ok(config)
}

/// 创建 PostgreSQL 连接池
#[cfg(feature = "persistence")]
pub async fn create_postgres_pool(config: &PostgresConfig) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect_timeout(std::time::Duration::from_secs(config.connection_timeout))
        .idle_timeout(std::time::Duration::from_secs(config.idle_timeout))
        .max_lifetime(std::time::Duration::from_secs(config.max_lifetime))
        .connect(&config.url)
        .await?;

    Ok(pool)
}

/// 创建 Redis 连接池
pub async fn create_redis_pool(config: &RedisConfig) -> Result<redis::aio::ConnectionManager, redis::RedisError> {
    let client = redis::Client::open(config.url.as_str())?;
    let pool = client.get_connection_manager().await?;
    Ok(pool)
}
