//! 限流模块
//!
//! 使用 governor 实现多维度令牌桶限流

use crate::{
    config::{RateLimitingConfig, RateLimitTier},
    domain::{GatewayError, GatewayResult, RateLimitDecision, RateLimitResult},
};
use chrono::Utc;
use futures::Future;
use governor::state::NotKeyed;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};
use std::{
    boxed::Box,
    collections::HashMap,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing::{debug, warn};

/// 限流维度键
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RateLimitKey {
    /// 全局限流
    Global,
    /// IP 地址
    Ip(SocketAddr),
    /// 用户 ID
    User(String),
    /// API Key
    ApiKey(String),
    /// 租户 ID
    Tenant(String),
    /// 路由
    Route(String),
    /// 复合键
    Composite(String),
}

/// 限流结果
#[derive(Debug, Clone)]
pub struct LimitResult {
    /// 是否允许
    pub allowed: bool,
    /// 剩余配额
    pub remaining: i64,
    /// 重试等待秒数
    pub retry_after: Option<u64>,
    /// 限流名称
    pub limit_name: String,
    /// 限制值
    pub limit: u64,
    /// 当前使用量
    pub current: u64,
    /// 剩余时间
    pub reset_after: Duration,
}

/// 限流器包装 (简化实现)
#[derive(Debug, Clone)]
pub struct RateLimiterWrapper {
    /// 限制名称
    name: String,
    /// 桶容量
    capacity: u64,
    /// 填充速率 (每秒)
    refill_rate: u64,
    /// 突发容量
    burst_capacity: u64,
    /// 最后检查时间
    last_check: Arc<parking_lot::Mutex<std::time::Instant>>,
    /// 当前计数
    current: Arc<parking_lot::Mutex<u64>>,
}

impl RateLimiterWrapper {
    /// 创建新的限流器
    pub fn new(name: String, tier: &RateLimitTier) -> Option<Self> {
        if !tier.enabled || tier.capacity == 0 {
            return None;
        }

        Some(Self {
            name,
            capacity: tier.capacity,
            refill_rate: tier.refill_rate,
            burst_capacity: tier.burst_capacity,
            last_check: Arc::new(parking_lot::Mutex::new(std::time::Instant::now())),
            current: Arc::new(parking_lot::Mutex::new(tier.capacity)),
        })
    }

    /// 检查是否允许请求
    pub fn check(&self, _key: impl std::hash::Hash) -> LimitResult {
        let now = std::time::Instant::now();
        let mut last_check = self.last_check.lock();
        let mut current = self.current.lock();

        // 计算经过的时间
        let elapsed = now.duration_since(*last_check);

        // 填充令牌
        let refill = (elapsed.as_secs() as u64) * self.refill_rate;
        *current = std::cmp::min(*current + refill, self.burst_capacity);

        // 更新最后检查时间
        *last_check = now;

        // 检查是否允许
        let allowed = *current > 0;
        if allowed {
            *current -= 1;
        }

        LimitResult {
            allowed,
            remaining: *current as i64,
            retry_after: if allowed { None } else { Some(1) },
            limit_name: self.name.clone(),
            limit: self.capacity,
            current: self.capacity - *current,
            reset_after: Duration::from_secs(1),
        }
    }
}

/// 限流管理器
#[derive(Debug, Clone)]
pub struct RateLimitManager {
    /// 全局限流器
    global_limiter: Option<RateLimiterWrapper>,
    /// IP 限流器
    ip_limiters: Arc<Mutex<HashMap<SocketAddr, RateLimiterWrapper>>>,
    /// 用户限流器
    user_limiters: Arc<Mutex<HashMap<String, RateLimiterWrapper>>>,
    /// API Key 限流器
    api_key_limiters: Arc<Mutex<HashMap<String, RateLimiterWrapper>>>,
    /// 路由限流器
    route_limiters: Arc<Mutex<HashMap<String, RateLimiterWrapper>>>,
    /// 配置
    config: Arc<RateLimitingConfig>,
}

impl RateLimitManager {
    /// 创建限流管理器
    pub fn new(config: Arc<RateLimitingConfig>) -> Self {
        let global_limiter = RateLimiterWrapper::new("global".to_string(), &config.global);

        Self {
            global_limiter,
            ip_limiters: Arc::new(Mutex::new(HashMap::new())),
            user_limiters: Arc::new(Mutex::new(HashMap::new())),
            api_key_limiters: Arc::new(Mutex::new(HashMap::new())),
            route_limiters: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// 获取或创建 IP 限流器
    async fn get_or_create_ip_limiter(&self, ip: SocketAddr) -> Option<RateLimiterWrapper> {
        let mut limiters = self.ip_limiters.lock().await;
        if let Some(limiter) = limiters.get(&ip) {
            return Some(limiter.clone());
        }

        let limiter = RateLimiterWrapper::new(
            format!("ip:{}", ip),
            &self.config.per_ip,
        );

        if let Some(ref l) = limiter {
            limiters.insert(ip, l.clone());
        }

        limiter
    }

    /// 获取或创建用户限流器
    async fn get_or_create_user_limiter(&self, user_id: &str) -> Option<RateLimiterWrapper> {
        let mut limiters = self.user_limiters.lock().await;
        if let Some(limiter) = limiters.get(user_id) {
            return Some(limiter.clone());
        }

        let limiter = RateLimiterWrapper::new(
            format!("user:{}", user_id),
            &self.config.per_user,
        );

        if let Some(ref l) = limiter {
            limiters.insert(user_id.to_string(), l.clone());
        }

        limiter
    }

    /// 获取或创建 API Key 限流器
    async fn get_or_create_api_key_limiter(&self, api_key: &str) -> Option<RateLimiterWrapper> {
        let mut limiters = self.api_key_limiters.lock().await;
        if let Some(limiter) = limiters.get(api_key) {
            return Some(limiter.clone());
        }

        let limiter = RateLimiterWrapper::new(
            format!("apikey:{}", &api_key[..8.min(api_key.len())]),
            &self.config.per_api_key,
        );

        if let Some(ref l) = limiter {
            limiters.insert(api_key.to_string(), l.clone());
        }

        limiter
    }

    /// 获取或创建路由限流器
    async fn get_or_create_route_limiter(&self, route: &str) -> Option<RateLimiterWrapper> {
        let mut limiters = self.route_limiters.lock().await;
        if let Some(limiter) = limiters.get(route) {
            return Some(limiter.clone());
        }

        let tier = self.config.per_route.get(route)
            .cloned()
            .unwrap_or(RateLimitTier {
                enabled: true,
                capacity: 100,
                refill_rate: 100,
                burst_capacity: 200,
            });

        let limiter = RateLimiterWrapper::new(
            format!("route:{}", route),
            &tier,
        );

        if let Some(ref l) = limiter {
            limiters.insert(route.to_string(), l.clone());
        }

        limiter
    }

    /// 检查所有维度的限流
    pub async fn check(
        &self,
        client_ip: Option<SocketAddr>,
        user_id: Option<&str>,
        api_key: Option<&str>,
        route: &str,
    ) -> Vec<LimitResult> {
        let mut results = Vec::new();

        // 全局限流
        if let Some(ref limiter) = self.global_limiter {
            results.push(limiter.check("global"));
        }

        // IP 限流
        if let Some(ip) = client_ip {
            if let Some(limiter) = self.get_or_create_ip_limiter(ip).await {
                results.push(limiter.check(ip));
            }
        }

        // 用户限流
        if let Some(id) = user_id {
            if let Some(limiter) = self.get_or_create_user_limiter(id).await {
                results.push(limiter.check(id));
            }
        }

        // API Key 限流
        if let Some(key) = api_key {
            if let Some(limiter) = self.get_or_create_api_key_limiter(key).await {
                results.push(limiter.check(key));
            }
        }

        // 路由限流
        if let Some(limiter) = self.get_or_create_route_limiter(route).await {
            results.push(limiter.check(route));
        }

        results
    }

    /// 检查并返回最严格的限流结果
    pub async fn check_strict(
        &self,
        client_ip: Option<SocketAddr>,
        user_id: Option<&str>,
        api_key: Option<&str>,
        route: &str,
    ) -> GatewayResult<RateLimitResult> {
        let results = self.check(client_ip, user_id, api_key, route).await;

        // 找到最严格的限制
        let mut strictest = None;
        for result in &results {
            if !result.allowed {
                if strictest.is_none() || result.remaining < strictest.as_ref().unwrap().remaining {
                    strictest = Some(result);
                }
            }
        }

        if let Some(limit) = strictest {
            return Ok(RateLimitResult {
                decision: RateLimitDecision::RateLimited,
                remaining: limit.remaining,
                retry_after: Some(limit.reset_after.as_secs()),
                limit_name: limit.limit_name.clone(),
            });
        }

        // 允许请求
        let remaining = results.iter()
            .map(|r| r.remaining)
            .min()
            .unwrap_or(i64::MAX);

        Ok(RateLimitResult {
            decision: RateLimitDecision::Allowed,
            remaining,
            retry_after: None,
            limit_name: "none".to_string(),
        })
    }
}

/// 限流中间件工厂
#[derive(Debug, Clone)]
pub struct RateLimitMiddlewareFactory {
    /// 限流管理器
    manager: Arc<RateLimitManager>,
}

impl RateLimitMiddlewareFactory {
    /// 创建工厂
    pub fn new(manager: Arc<RateLimitManager>) -> Self {
        Self { manager }
    }

    /// 创建中间件
    pub fn middleware<S>(
        &self,
        state: S,
    ) -> impl tower::Service<
        http::Request<S>,
        Response = http::Response<axum::body::Body>,
        Error = std::convert::Infallible,
    > + Clone
    where
        S: Clone,
    {
        let manager = self.manager.clone();
        RateLimitMiddleware { manager, state }
    }
}

/// 限流中间件
#[derive(Debug, Clone)]
pub struct RateLimitMiddleware<S> {
    /// 限流管理器
    manager: Arc<RateLimitManager>,
    /// 状态
    state: S,
}

impl<S> tower::Service<http::Request<S>> for RateLimitMiddleware<S>
where
    S: Clone,
{
    type Response = http::Response<axum::body::Body>;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: http::Request<S>) -> Self::Future {
        let manager = self.manager.clone();
        let state = self.state.clone();

        async move {
            // 提取上下文
            let client_ip = req
                .extensions()
                .get::<SocketAddr>()
                .cloned();

            let user_id = req
                .extensions()
                .get::<crate::domain::AuthenticationInfo>()
                .map(|a| a.subject_id.as_str());

            let api_key = req
                .headers()
                .get("X-API-Key")
                .and_then(|h| h.to_str().ok())
                .or(req.headers().get("Authorization")
                    .and_then(|h| h.to_str().ok())
                    .filter(|s| !s.starts_with("Bearer ")));

            let route = req.uri().path();

            // 检查限流
            match manager.check_strict(client_ip, user_id, api_key, route).await {
                Ok(result) => {
                    // 添加限流头
                    let mut response = tower::Service::call(&mut state, req).await?;

                    if result.decision != RateLimitDecision::Allowed {
                        *response.status_mut() = http::StatusCode::TOO_MANY_REQUESTS;
                        response.headers_mut().insert(
                            "X-RateLimit-Remaining",
                            http::HeaderValue::from(result.remaining),
                        );
                        response.headers_mut().insert(
                            "X-RateLimit-Limit",
                            http::HeaderValue::from_str(&result.limit_name).unwrap(),
                        );
                        if let Some(retry_after) = result.retry_after {
                            response.headers_mut().insert(
                                "Retry-After",
                                http::HeaderValue::from(retry_after),
                            );
                        }
                    } else {
                        response.headers_mut().insert(
                            "X-RateLimit-Remaining",
                            http::HeaderValue::from(result.remaining),
                        );
                    }

                    Ok(response)
                }
                Err(e) => Err(e),
            }
        }
    }
}

/// 滑动窗口日志
#[derive(Debug, Clone)]
pub struct SlidingWindowLog {
    /// 请求时间戳
    timestamps: Arc<Mutex<Vec<Instant>>>,
    /// 窗口大小
    window_size: Duration,
    /// 最大请求数
    max_requests: u64,
}

impl SlidingWindowLog {
    /// 创建滑动窗口日志
    pub fn new(window_size: Duration, max_requests: u64) -> Self {
        Self {
            timestamps: Arc::new(Mutex::new(Vec::new())),
            window_size,
            max_requests,
        }
    }

    /// 记录请求
    pub async fn record(&self) -> bool {
        let mut timestamps = self.timestamps.lock().await;
        let now = Instant::now();

        // 移除过期的请求
        timestamps.retain(|t| now.duration_since(*t) < self.window_size);

        // 检查是否超过限制
        if timestamps.len() >= self.max_requests as usize {
            return false;
        }

        timestamps.push(now);
        true
    }

    /// 获取当前请求数
    pub async fn count(&self) -> usize {
        let timestamps = self.timestamps.lock().await;
        let now = Instant::now();
        timestamps.iter()
            .filter(|t| now.duration_since(*t) < self.window_size)
            .count()
    }
}
