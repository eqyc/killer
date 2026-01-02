//! MDG 服务主入口
//!
//! 主数据治理服务 - 集中管理和分发主数据

use anyhow::Result;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod config;
mod domain;
mod application;
mod infrastructure;
mod health;

use error::MdgResult;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    init_tracing()?;

    info!("启动 MDG 服务...");

    // 加载配置
    let config = config::load_config()?;
    info!("配置加载完成");

    // 初始化基础设施
    let infrastructure = infrastructure::Infrastructure::new(&config).await?;
    info!("基础设施初始化完成");

    // 启动 gRPC 服务器
    let grpc_addr: SocketAddr = format!("{}:{}", config.server.grpc_addr, config.server.grpc_port)
        .parse()?;
    
    let grpc_server = tokio::spawn(async move {
        info!("gRPC 服务器监听: {}", grpc_addr);
        // gRPC 服务器实现
        Ok::<_, anyhow::Error>(())
    });

    // 启动 HTTP 服务器 (健康检查和指标)
    let http_addr: SocketAddr = format!("{}:{}", config.server.http_addr, config.server.http_port)
        .parse()?;
    
    let http_server = tokio::spawn(async move {
        info!("HTTP 服务器监听: {}", http_addr);
        // HTTP 服务器实现
        Ok::<_, anyhow::Error>(())
    });

    info!("MDG 服务启动成功");

    // 等待关闭信号
    shutdown_signal().await;

    info!("收到关闭信号，开始优雅关闭...");

    // 等待服务器关闭
    let _ = tokio::join!(grpc_server, http_server);

    info!("MDG 服务已关闭");

    Ok(())
}

/// 初始化追踪
fn init_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    Ok(())
}

/// 等待关闭信号
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("无法安装 Ctrl+C 处理器");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("无法安装 SIGTERM 处理器")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("收到 Ctrl+C 信号");
        },
        _ = terminate => {
            info!("收到 SIGTERM 信号");
        },
    }
}
