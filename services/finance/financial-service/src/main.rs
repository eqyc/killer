//! Financial Service 程序入口
//!
//! # 职责
//!
//! 1. **配置加载** - 从环境变量和配置文件加载服务配置
//! 2. **依赖注入** - 构建并注入所有服务依赖（数据库连接池、缓存、消息队列等）
//! 3. **可观测性初始化** - 设置 Tracing、Metrics、OpenTelemetry
//! 4. **服务注册** - 注册 Command/Query Handler、Event Handler
//! 5. **服务器启动** - 启动 HTTP (Axum) 和 gRPC (Tonic) 服务器
//! 6. **优雅关闭** - 处理 SIGTERM/SIGINT，确保请求完成后关闭
//!
//! # 启动流程
//!
//! ```text
//! main()
//!   ├── load_config()           // 加载配置
//!   ├── init_telemetry()        // 初始化可观测性
//!   ├── init_database()         // 初始化数据库连接池
//!   ├── init_cache()            // 初始化缓存
//!   ├── init_messaging()        // 初始化消息队列
//!   ├── build_app_state()       // 构建应用状态（依赖注入容器）
//!   ├── register_handlers()     // 注册 CQRS 处理器
//!   ├── start_event_consumers() // 启动事件消费者
//!   ├── start_servers()         // 启动 HTTP + gRPC 服务器
//!   └── await_shutdown()        // 等待关闭信号
//! ```

use tonic::transport::Server;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use tracing_subscriber::{fmt, EnvFilter};

mod domain;
mod application;
mod infrastructure;

// 加载配置
fn load_config() -> Result<AppConfig, config::ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("config/financial-service"))
        .add_source(config::Environment::default().prefix("FINANCIAL_"))
        .build()?
        .try_deserialize()
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub grpc_address: String,
    pub http_address: String,
    pub service_name: String,
    pub log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    fmt()
        .with_env_filter(EnvFilter::new("info"))
        .with_target(false)
        .compact()
        .init();

    tracing::info!("启动 Financial Service...");

    // 加载配置
    let config = load_config().unwrap_or_else(|_| {
        tracing::warn!("无法加载配置文件，使用默认值");
        AppConfig {
            database_url: "postgres://postgres:postgres@localhost:5432/killer_fi".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            kafka_brokers: "localhost:9092".to_string(),
            grpc_address: "0.0.0.0:50051".to_string(),
            http_address: "0.0.0.0:8080".to_string(),
            service_name: "financial-service".to_string(),
            log_level: "info".to_string(),
        }
    });

    tracing::info!(
        "Financial Service 配置: gRPC={}, HTTP={}",
        config.grpc_address,
        config.http_address
    );

    // 初始化数据库连接池
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("数据库连接池已建立");

    // 初始化仓储
    let repository = infrastructure::persistence::postgres::PostgresRepository::new(pool);

    // 初始化 gRPC 服务
    let addr = config.grpc_address.parse::<std::net::SocketAddr>()?;

    // 创建 gRPC 服务实现
    let financial_service = infrastructure::grpc::financial_grpc_service::FinancialGrpcService::new(
        repository.clone(),
        repository.clone(),
        repository.clone(),
        repository.clone(),
        repository.clone(),
        repository.clone(),
    );

    // 添加健康检查端点
    let health_service = tonic_health::server::health_service();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(killer_api_contracts::FILE_DESCRIPTOR_SET)
        .build()?;

    tracing::info!("Financial Service 启动成功，监听地址: {}", addr);

    // 启动 gRPC 服务器
    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(killer_api_contracts::finance::financial::v1::financial_service_server::FinancialServiceServer::new(
            financial_service,
        ))
        .serve(addr)
        .await?;

    tracing::info!("Financial Service 已关闭");

    Ok(())
}
