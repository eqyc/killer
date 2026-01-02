//! 网关应用服务
//!
//! 组合所有中间件组件，提供统一的请求处理入口

use crate::{
    config::{GatewayConfig, TenantConfig},
    domain::{AuthenticationInfo, GatewayError, RequestContext, RouteMatch, TenantInfo},
};
use async_trait::async_trait;
use chrono::Utc;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use uuid::Uuid;

/// 简化的路由匹配器
#[derive(Debug, Clone)]
pub struct Router {
    routes: Vec<RouterRoute>,
}

#[derive(Debug, Clone)]
struct RouterRoute {
    id: String,
    path_prefix: String,
    priority: u32,
    enabled: bool,
}

impl Router {
    fn new(routes: &[crate::config::RouteConfig]) -> Self {
        let routes: Vec<_> = routes.iter()
            .filter(|r| r.enabled)
            .map(|r| RouterRoute {
                id: r.id.clone(),
                path_prefix: r.path_prefix.clone(),
                priority: r.priority,
                enabled: r.enabled,
            })
            .collect();

        let mut routes = routes;
        routes.sort_by(|a, b| b.priority.cmp(&a.priority));

        Self { routes }
    }

    fn match_route(&self, path: &str) -> Option<RouteMatch> {
        for route in &self.routes {
            if path.starts_with(&route.path_prefix) {
                return Some(RouteMatch {
                    route: Arc::new(crate::config::RouteConfig {
                        id: route.id.clone(),
                        path_prefix: route.path_prefix.clone(),
                        ..Default::default()
                    }),
                    params: HashMap::new(),
                    matched_segments: vec![],
                });
            }
        }
        None
    }
}

/// 简化的负载均衡器
#[derive(Debug, Clone, Default)]
pub struct LoadBalancer {
    current_index: Arc<RwLock<u32>>,
}

impl LoadBalancer {
    fn new() -> Self {
        Self {
            current_index: Arc::new(RwLock::new(0)),
        }
    }
}

/// 网关服务
#[derive(Debug, Clone)]
pub struct GatewayService {
    config: Arc<GatewayConfig>,
    router: Arc<Router>,
    load_balancer: Arc<LoadBalancer>,
}

impl GatewayService {
    pub async fn new(config: Arc<GatewayConfig>) -> Self {
        Self {
            config: config.clone(),
            router: Arc::new(Router::new(&config.routes)),
            load_balancer: Arc::new(LoadBalancer::new()),
        }
    }

    pub fn router(&self) -> &Router {
        &self.router
    }

    pub fn load_balancer(&self) -> &LoadBalancer {
        &self.load_balancer
    }

    /// 处理请求
    pub async fn handle_request(
        &self,
        req: http::Request<()>,
        client_addr: SocketAddr,
    ) -> Result<http::Response<String>, GatewayError> {
        let path = req.uri().path();

        // 1. 路由匹配
        let route = self.router.match_route(path)
            .ok_or_else(|| GatewayError::RouteNotFound { path: path.to_string() })?;

        // 2. 创建上下文
        let ctx = RequestContext {
            request_id: Uuid::new_v4(),
            trace_id: Uuid::new_v4().to_string(),
            span_id: format!("{:x}", rand::random::<u64>()),
            client_ip: client_addr.ip().to_string(),
            client_port: client_addr.port(),
            user_agent: None,
            authentication: AuthenticationInfo::default(),
            tenant: None,
            original_path: path.to_string(),
            original_query: req.uri().query().map(|s| s.to_string()),
            start_time: Utc::now(),
            metadata: HashMap::new(),
        };

        // 3. 租户处理
        let tenant = self.ensure_tenant(&ctx)?;

        // 4. 返回响应
        let response = format!(
            "Gateway Response\nRoute: {}\nTenant: {:?}\nPath: {}",
            route.route.id,
            tenant.map(|t| t.id),
            path
        );

        Ok(http::Response::builder()
            .status(200)
            .body(response)
            .unwrap())
    }

    fn ensure_tenant(&self, ctx: &RequestContext) -> Result<Option<TenantInfo>, GatewayError> {
        let tenant_config = &self.config.tenant;

        if let Some(tenant_id) = &ctx.authentication.tenant_id {
            return Ok(Some(TenantInfo {
                id: tenant_id.clone(),
                name: tenant_id.clone(),
                tenant_type: "default".to_string(),
                quota: crate::domain::TenantQuota::default(),
                status: crate::domain::TenantStatus::Active,
            }));
        }

        if tenant_config.mandatory {
            return Err(GatewayError::InvalidTenant {
                tenant_id: "missing".to_string(),
            });
        }

        Ok(None)
    }
}
