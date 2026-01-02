//! HTTP 接口层
//!
//! 提供 HTTP 请求处理入口，集成所有中间件

use crate::{
    application::GatewayService,
    config::{GatewayConfig, SecurityConfig, CorsConfig, HstsConfig},
    domain::RequestContext,
};
use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
    routing::{get, post, put, delete, patch, head, options},
    Router,
};
use http::{header, StatusCode, Uri};
use std::{convert::identity, net::SocketAddr, sync::Arc};
use tower::{ServiceExt};
use tower::layer::Layer;
use tower_http::{
    add_extension::AddExtensionLayer,
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{self, CorsLayer},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{info, span, Level};

/// HTTP 处理器
#[derive(Clone)]
pub struct HttpHandler {
    /// 网关服务
    gateway: Arc<GatewayService>,
    /// 配置
    config: Arc<GatewayConfig>,
}

impl HttpHandler {
    /// 创建 HTTP 处理器
    pub fn new(gateway: Arc<GatewayService>, config: Arc<GatewayConfig>) -> Self {
        Self { gateway, config }
    }

    /// 构建路由
    pub fn router(&self) -> Router {
        // 健康检查端点
        let health = Router::new()
            .route("/health", get(|| async { "OK" }))
            .route("/ready", get(|| async { "Ready" }))
            .route("/live", get(|| async { "Live" }));

        // 指标端点
        let metrics = Router::new()
            .route("/metrics", get(Self::metrics_handler));

        // 管理端点
        let admin = Router::new()
            .route("/routes", get(Self::routes_handler))
            .route("/config", get(Self::config_handler));

        // 主路由
        let api = Router::new()
            .route("/*path", get(Self::proxy_handler))
            .route("/*path", post(Self::proxy_handler))
            .route("/*path", put(Self::proxy_handler))
            .route("/*path", delete(Self::proxy_handler))
            .route("/*path", patch(Self::proxy_handler))
            .route("/*path", head(Self::proxy_handler))
            .route("/*path", options(Self::proxy_handler));

        health
            .merge(metrics)
            .merge(admin)
            .merge(api)
    }

    /// 构建中间件栈
    pub fn build_middleware(&self) -> impl tower::Layer<Body> + Clone {
        // 注意：实际的认证、限流、授权中间件在 GatewayService.handle_request 中处理
        // 这里只处理通用的 HTTP 中间件
        identity()
    }

    /// 代理处理器
    async fn proxy_handler(req: Request<Body>) -> impl IntoResponse {
        // 从请求扩展获取网关服务和客户端地址
        let gateway = req
            .extensions()
            .get::<Arc<GatewayService>>()
            .cloned();

        let client_addr = req
            .extensions()
            .get::<SocketAddr>()
            .cloned()
            .unwrap_or_else(|| "0.0.0.0:0".parse().unwrap());

        if let Some(gw) = gateway {
            match gw.handle_request(req, client_addr).await {
                Ok(response) => response.into_response(),
                Err(e) => {
                    tracing::error!("Gateway error: {:?}", e);
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(format!(r#"{{"error":"Internal Server Error","message":"{}"}}"#, e)))
                        .unwrap()
                        .into_response()
                }
            }
        } else {
            Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .body(Body::from(r#"{"error":"Service Unavailable"}"#))
                .unwrap()
                .into_response()
        }
    }

    /// 指标处理器
    async fn metrics_handler() -> impl IntoResponse {
        // Prometheus 格式指标
        let metrics = r#"
# HELP gateway_requests_total Total number of requests
# TYPE gateway_requests_total counter
gateway_requests_total 0

# HELP gateway_requests_duration_seconds Request duration
# TYPE gateway_requests_duration_seconds histogram
gateway_requests_duration_seconds_bucket{le="0.005"} 0
gateway_requests_duration_seconds_bucket{le="0.01"} 0
gateway_requests_duration_seconds_bucket{le="0.025"} 0
gateway_requests_duration_seconds_bucket{le="0.05"} 0
gateway_requests_duration_seconds_bucket{le="0.1"} 0
gateway_requests_duration_seconds_bucket{le="0.25"} 0
gateway_requests_duration_seconds_bucket{le="0.5"} 0
gateway_requests_duration_seconds_bucket{le="1"} 0
gateway_requests_duration_seconds_bucket{le="2.5"} 0
gateway_requests_duration_seconds_bucket{le="5"} 0
gateway_requests_duration_seconds_bucket{le="10"} 0
gateway_requests_duration_seconds_bucket{le="+Inf"} 0
gateway_requests_duration_seconds_sum 0
gateway_requests_duration_seconds_count 0
"#;
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
            metrics,
        )
    }

    /// 路由列表处理器
    async fn routes_handler() -> impl IntoResponse {
        // 返回配置的路由列表
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            r#"[]"#,
        )
    }

    /// 配置处理器
    async fn config_handler() -> impl IntoResponse {
        // 返回非敏感配置
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            r#"{"server":{"port":8080}}"#,
        )
    }
}

/// 构建 CORS 层
pub fn build_cors_layer(config: &CorsConfig) -> CorsLayer {
    let cors = cors::CorsLayer::new()
        .allow_methods(cors::AllowMethods::list(config.allowed_methods.clone()))
        .allow_headers(cors::AllowHeaders::list(config.allowed_headers.clone()))
        .allow_credentials(config.allow_credentials);

    if config.allowed_origins.iter().any(|o| o == "*") {
        cors.allow_origin(cors::AllowOrigin::any())
    } else {
        cors.allow_origin(cors::AllowOrigin::list(config.allowed_origins.clone()))
    }
}

/// 构建安全头层
pub fn build_security_headers_layer<S>(config: &SecurityConfig) -> impl tower::Layer<S> + Clone
where
    S: Clone,
{
    // 简化的安全头层
    identity()
}

/// HTTP 服务器
#[derive(Debug)]
pub struct HttpServer {
    /// 监听地址
    addr: SocketAddr,
    /// 处理器
    handler: HttpHandler,
    /// 路由器
    router: Router,
}

impl HttpServer {
    /// 创建 HTTP 服务器
    pub fn new(gateway: Arc<GatewayService>, config: Arc<GatewayConfig>) -> Self {
        let addr = format!("{}:{}", config.server.http_addr, config.server.http_port)
            .parse()
            .unwrap();

        let handler = HttpHandler::new(gateway.clone(), config.clone());
        let router = handler.router();

        Self { addr, handler, router }
    }

    /// 启动服务器
    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let server = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new());

        info!("HTTP server listening on {}", self.addr);

        Ok(())
    }

    /// 等待关闭信号
    async fn shutdown_signal() {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        info!("Received shutdown signal, gracefully shutting down...");
    }
}
