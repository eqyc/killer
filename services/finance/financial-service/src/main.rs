//! Financial Service 程序入口
//!
//! HTTP REST API 服务

use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};
use axum::serve;
use tokio::net::TcpListener;
use config::Config;

pub mod domain;
pub mod infrastructure;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub http_address: String,
}

fn load_config() -> Result<AppConfig, config::ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("config/financial-service"))
        .add_source(config::Environment::default().prefix("FINANCIAL_"))
        .build()?
        .try_deserialize()
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
            database_url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            http_address: "0.0.0.0:8080".to_string(),
        }
    });

    tracing::info!("HTTP 服务地址: {}", config.http_address);

    // 初始化数据库连接池
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("数据库连接池已建立");

    // 创建 HTTP 路由
    let pool = std::sync::Arc::new(pool);
    let http_router = infrastructure::http::create_router(pool.clone());

    // 解析地址
    let addr: SocketAddr = config.http_address.parse()?;

    // 启动 HTTP 服务器
    tracing::info!("启动 HTTP 服务器: {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    let server = serve(listener, http_router);
    tracing::info!("Financial Service 运行中，访问 http://{}", addr);
    server.await?;

    tracing::info!("Financial Service 已关闭");

    Ok(())
}
