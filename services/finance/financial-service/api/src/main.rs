//! API 服务主入口
//!
//! 启动 gRPC 和 HTTP 服务器

use killer_financial_service::api::{ApiConfig, ApiServer};
use killer_financial_service::middleware::auth::AuthInterceptor;
use killer_financial_service::middleware::idempotency::IdempotencyMiddleware;
use killer_financial_service::middleware::audit::AuditMiddleware;
use prometheus::Registry;
use std::sync::Arc;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 初始化日志
    let env_filter = EnvFilter::from_default_env();
    let subscriber = Subscriber::builder()
        .with_env_filter(env_filter)
        .with_ansi(false)
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // 加载配置
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.yaml".to_string());

    let config = Arc::new(
        ApiConfig::from_config_file(&config_path)
            .unwrap_or_else(|_| Arc::new(ApiConfig::default())),
    );

    info!(?config, "Loaded configuration");

    // 创建 Prometheus 注册表
    let registry = Registry::new();

    // 创建认证拦截器
    let auth_interceptor = AuthInterceptor::new(
        &config.auth.jwt_secret,
        config.auth.allowed_issuers.clone(),
        config.auth.allowed_audiences.clone(),
    );

    // 创建幂等性中间件（如果有 Redis 配置）
    let idempotency = if config.idempotency.enabled {
        // TODO: 创建 Redis 连接并初始化 IdempotencyMiddleware
        None
    } else {
        None
    };

    // 创建审计中间件
    let audit = if config.audit.enabled {
        // TODO: 创建审计存储并初始化 AuditMiddleware
        None
    } else {
        None
    };

    // TODO: 创建应用服务
    // let journal_entry_service = Arc::new(JournalEntryApplicationService::new(...));
    // let reporting_service = Arc::new(ReportingService::new(...));

    // 创建 API 服务器
    // let mut server = ApiServer::new(
    //     config,
    //     journal_entry_service,
    //     reporting_service,
    //     auth_interceptor,
    //     idempotency,
    //     audit,
    //     registry,
    // )?;

    // 启动服务器
    // server.serve().await?;

    info!("API service started");
    Ok(())
}
