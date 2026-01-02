//! API 服务器
//!
//! 启动 gRPC 和 HTTP 服务器

use crate::config::{ApiConfig, GrpcConfig, HttpConfig};
use crate::error::{ApiError, ApiResult};
use crate::middleware::auth::{AuthContext, AuthInterceptor};
use crate::middleware::audit::{AuditMiddleware, AuditStorage, DbAuditStorage};
use crate::middleware::idempotency::IdempotencyMiddleware;
use crate::middleware::metrics::ApiMetrics;
use crate::services::health::HealthService;
use crate::services::journal_entry::JournalEntryGrpcService;
use futures::Future;
use killer_financial_service::application::services::*;
use prometheus::Registry;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::body::BoxBody;
use tonic::codegen::*;
use tonic::transport::{NamedService, Server};
use tonic::Request;
use tower::Service;
use tracing::{info, warn};

/// API 服务器
pub struct ApiServer {
    /// 配置
    config: Arc<ApiConfig>,

    /// gRPC 服务
    grpc_service: Option<JournalEntryGrpcService>,

    /// 健康检查服务
    health_service: HealthService,

    /// 认证拦截器
    auth_interceptor: Arc<AuthInterceptor>,

    /// 幂等性中间件
    idempotency: Option<Arc<IdempotencyMiddleware>>,

    /// 审计中间件
    audit: Option<Arc<AuditMiddleware>>,

    /// 指标
    metrics: Arc<ApiMetrics>,

    /// Prometheus 注册表
    registry: Registry,
}

impl ApiServer {
    /// 创建新的服务器
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Arc<ApiConfig>,
        journal_entry_service: Arc<JournalEntryApplicationService>,
        reporting_service: Arc<ReportingService>,
        auth_interceptor: AuthInterceptor,
        idempotency: Option<IdempotencyMiddleware>,
        audit_middleware: Option<AuditMiddleware>,
        registry: Registry,
    ) -> Result<Self, prometheus::Error> {
        let metrics = Arc::new(ApiMetrics::new(&registry)?);

        let auth_interceptor = Arc::new(auth_interceptor);
        let idempotency = idempotency.map(Arc::new);
        let audit = audit_middleware.map(Arc::new);

        let grpc_service = JournalEntryGrpcService::new(
            journal_entry_service,
            reporting_service,
            auth_interceptor.clone(),
            idempotency.clone(),
            audit.clone(),
            metrics.clone(),
        );

        Ok(Self {
            config,
            grpc_service: Some(grpc_service),
            health_service: HealthService::new(),
            auth_interceptor,
            idempotency,
            audit,
            metrics,
            registry,
        })
    }

    /// 获取配置
    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    /// 获取指标注册表
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// 启动服务器
    pub async fn serve(&mut self) -> Result<(), anyhow::Error> {
        let grpc_addr = self.config.grpc_addr();
        let http_addr = self.config.http_addr();

        info!(%grpc_addr, %http_addr, "Starting API servers");

        // 构建 gRPC 服务
        let mut grpc_server = Server::builder()
            .concurrency_limit_per_connection(self.config.grpc.max_concurrent_requests)
            .add_service(self.health_service.clone().write_encoding())
            .add_service(
                tonic_health::make_health_reporter()
                    .set_serving()
                    .write_encoding(),
            );

        if let Some(ref service) = self.grpc_service {
            let descriptor = crate::finance_v1::journal_entry_service_server::server::inventory::service_descriptor();
            grpc_server = grpc_server.add_service(
                crate::finance_v1::journal_entry_service_server::JournalEntryServiceServer::new(service.clone())
                    .max_decoding_message_size(self.config.grpc.max_message_size * 1024 * 1024)
            );
        }

        // 启动 gRPC 服务器
        let grpc_handle = tokio::spawn(async move {
            info!("gRPC server listening on {}", grpc_addr);
            grpc_server
                .serve(grpc_addr)
                .await
                .expect("gRPC server failed");
        });

        // 启动 HTTP 服务器（如果启用）
        if self.config.http.enabled {
            let http_server = self.start_http_server(http_addr).await?;
            info!("HTTP server listening on {}", http_addr);
        }

        // 等待 gRPC 服务器完成
        grpc_handle.await?;

        Ok(())
    }

    /// 启动 HTTP 服务器
    async fn start_http_server(&self, addr: SocketAddr) -> Result<(), anyhow::Error> {
        let app = self.create_http_router().await?;

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// 创建 HTTP 路由
    async fn create_http_router(&self) -> Result<axum::Router, anyhow::Error> {
        use axum::{
            routing::{get, post},
            Json,
        };

        let metrics = self.metrics.clone();
        let registry = self.registry.clone();

        let router = axum::Router::new()
            // 健康检查端点
            .route("/health/live", get(|| async { "OK" }))
            .route("/health/ready", get(|| async {
                Json(serde_json::json!({
                    "status": "ready",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }))
            }))
            // 指标端点
            .route(
                &self.config.metrics.path,
                get(move || async move {
                    use prometheus::Encoder;
                    let encoder = prometheus::TextEncoder::new();
                    let mut buffer = Vec::new();
                    encoder.encode(&registry.gather(), &mut buffer).unwrap();
                    String::from_utf8(buffer).unwrap()
                }),
            )
            // Swagger UI
            .route("/swagger-ui", get(|| async { "Swagger UI: See /swagger-ui/index.html" }))
            .route("/swagger-ui/", get(|| async { "Swagger UI: See /swagger-ui/index.html" }))
            .route("/swagger-ui/*path", get(|| async { "Swagger UI" }));

        // 添加 CORS 中间件
        let router = router.layer(tower_http::cors::CorsLayer::new()
            .allow_origin(self.config.http.cors.allowed_origins.clone())
            .allow_methods(self.config.http.cors.allowed_methods.clone())
            .allow_headers(self.config.http.cors.allowed_headers.clone())
            .expose_headers(self.config.http.cors.expose_headers.clone())
            .max_age(std::time::Duration::from_secs(self.config.http.cors.max_age)));

        // 添加追踪中间件
        let router = router.layer(tower_http::trace::TraceLayer::new_for_http());

        // 添加超时中间件
        let router = router.layer(tower::timeout::Timeout::new(
            self.config.grpc.request_timeout,
        ));

        Ok(router)
    }

    /// 关闭服务器
    pub async fn shutdown(&self) {
        self.health_service.set_status(tonic_health::pb::ServingStatus::NotServing);
        info!("API server shutting down");
    }
}
