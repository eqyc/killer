//! 熔断器模块
//!
//! 实现基于状态机的熔断器，保护上游服务

use crate::{
    config::{CircuitBreakerConfig, CircuitBreakerOverride},
    domain::{CircuitBreakerMetrics, CircuitBreakerState, GatewayError, GatewayResult},
};
use chrono::{DateTime, Utc};
use futures::Future;
use serde::{Deserialize, Serialize};
use std::{
    boxed::Box,
    pin::Pin,
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// 熔断器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerPolicy {
    /// 失败阈值 (连续失败次数)
    pub failure_threshold: u32,
    /// 成功阈值 (半开状态需要成功次数)
    pub success_threshold: u32,
    /// 半开超时时间
    pub half_open_timeout: Duration,
    /// 请求量阈值 (最小请求数才触发)
    pub volume_threshold: u32,
    /// 失败率阈值 (百分比)
    pub failure_rate_threshold: f64,
    /// 恢复时间
    pub recovery_timeout: Duration,
}

impl Default for CircuitBreakerPolicy {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            half_open_timeout: Duration::from_secs(30),
            volume_threshold: 10,
            failure_rate_threshold: 50.0,
            recovery_timeout: Duration::from_secs(60),
        }
    }
}

/// 熔断器状态
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum State {
    /// 关闭状态 - 正常请求
    Closed,
    /// 打开状态 - 快速失败
    Open,
    /// 半开状态 - 探测恢复
    HalfOpen,
}

/// 熔断器条目
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    /// 名称
    name: String,
    /// 状态
    state: Arc<RwLock<State>>,
    /// 策略
    policy: CircuitBreakerPolicy,
    /// 失败计数 (连续)
    consecutive_failure: Arc<AtomicU32>,
    /// 成功计数 (半开状态)
    consecutive_success: Arc<AtomicU32>,
    /// 总请求数
    total_requests: Arc<AtomicU64>,
    /// 失败请求数
    failed_requests: Arc<AtomicU64>,
    /// 最后失败时间
    last_failure: Arc<RwLock<Option<Instant>>>,
    /// 最后成功时间
    last_success: Arc<RwLock<Option<Instant>>>,
    /// 半开状态开始时间
    half_open_started: Arc<RwLock<Option<Instant>>>,
    /// 打开状态开始时间
    open_started: Arc<RwLock<Option<Instant>>>,
}

impl CircuitBreaker {
    /// 创建新的熔断器
    pub fn new(name: String, policy: Option<CircuitBreakerPolicy>) -> Self {
        Self {
            name,
            state: Arc::new(RwLock::new(State::Closed)),
            policy: policy.unwrap_or_default(),
            consecutive_failure: Arc::new(AtomicU32::new(0)),
            consecutive_success: Arc::new(AtomicU32::new(0)),
            total_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            last_failure: Arc::new(RwLock::new(None)),
            last_success: Arc::new(RwLock::new(None)),
            half_open_started: Arc::new(RwLock::new(None)),
            open_started: Arc::new(RwLock::new(None)),
        }
    }

    /// 从配置创建
    pub fn from_config(name: String, config: &CircuitBreakerConfig, overrides: Option<&CircuitBreakerOverride>) -> Self {
        let policy = if let Some(override_) = overrides {
            CircuitBreakerPolicy {
                failure_threshold: override_.failure_threshold,
                success_threshold: override_.success_threshold,
                half_open_timeout: Duration::from_secs(override_.half_open_timeout),
                volume_threshold: config.default_volume_threshold,
                ..Default::default()
            }
        } else {
            CircuitBreakerPolicy {
                failure_threshold: config.default_failure_threshold,
                success_threshold: config.default_success_threshold,
                half_open_timeout: Duration::from_secs(config.default_half_open_timeout),
                volume_threshold: config.default_volume_threshold,
                ..Default::default()
            }
        };

        Self::new(name, Some(policy))
    }

    /// 获取当前状态
    pub async fn state(&self) -> State {
        let state = self.state.read().await;
        let now = Instant::now();

        match *state {
            State::HalfOpen => {
                // 检查半开超时
                if let Some(started) = *self.half_open_started.read().await {
                    if now.duration_since(started) > self.policy.half_open_timeout {
                        // 超时，切换到打开
                        drop(state);
                        *self.state.write().await = State::Open;
                        *self.open_started.write().await = Some(now);
                        warn!("[{}] Circuit breaker half-open timeout, opening", self.name);
                    }
                }
                *state
            }
            State::Open => {
                // 检查恢复超时
                if let Some(started) = *self.open_started.read().await {
                    if now.duration_since(started) > self.policy.recovery_timeout {
                        // 切换到半开
                        drop(state);
                        *self.state.write().await = State::HalfOpen;
                        *self.half_open_started.write().await = Some(now);
                        self.consecutive_success.store(0, Ordering::SeqCst);
                        info!("[{}] Circuit breaker trying to recover, half-open", self.name);
                    }
                }
                *state
            }
            State::Closed => *state,
        }
    }

    /// 检查是否允许请求
    pub async fn can_proceed(&self) -> bool {
        self.state().await != State::Open
    }

    /// 记录成功
    pub async fn record_success(&self) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);

        *self.last_success.write().await = Some(Instant::now());

        let state = self.state.read().await;

        match *state {
            State::HalfOpen => {
                let success = self.consecutive_success.fetch_add(1, Ordering::SeqCst) + 1;
                if success >= self.policy.success_threshold {
                    // 成功阈值达到，切换到关闭
                    drop(state);
                    *self.state.write().await = State::Closed;
                    self.consecutive_failure.store(0, Ordering::SeqCst);
                    info!("[{}] Circuit breaker recovered, closed", self.name);
                }
            }
            State::Closed => {
                // 成功时重置连续失败计数
                self.consecutive_failure.store(0, Ordering::SeqCst);
            }
            State::Open => {}
        }
    }

    /// 记录失败
    pub async fn record_failure(&self) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);
        self.failed_requests.fetch_add(1, Ordering::SeqCst);

        *self.last_failure.write().await = Some(Instant::now());

        let mut state = self.state.write().await;

        match *state {
            State::Closed => {
                let failure = self.consecutive_failure.fetch_add(1, Ordering::SeqCst) + 1;
                let total = self.total_requests.load(Ordering::SeqCst);
                let failed = self.failed_requests.load(Ordering::SeqCst);

                // 检查失败率
                let failure_rate = if total > 0 {
                    (failed as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                if failure >= self.policy.failure_threshold
                    || (total >= self.policy.volume_threshold && failure_rate >= self.policy.failure_rate_threshold)
                {
                    *state = State::Open;
                    *self.open_started.write().await = Some(Instant::now());
                    warn!(
                        "[{}] Circuit breaker opened: consecutive_failures={}, failure_rate={:.1}%",
                        self.name, failure, failure_rate
                    );
                }
            }
            State::HalfOpen => {
                // 半开状态任何失败都切换到打开
                *state = State::Open;
                *self.open_started.write().await = Some(Instant::now());
                self.consecutive_success.store(0, Ordering::SeqCst);
                warn!("[{}] Circuit breaker half-open failed, opening", self.name);
            }
            State::Open => {}
        }
    }

    /// 获取指标
    pub async fn metrics(&self) -> CircuitBreakerMetrics {
        let state = self.state().await;
        let last_failure = *self.last_failure.read().await;
        let last_success = *self.last_success.read().await;

        CircuitBreakerMetrics {
            state: match state {
                State::Closed => CircuitBreakerState::Closed,
                State::Open => CircuitBreakerState::Open,
                State::HalfOpen => CircuitBreakerState::HalfOpen,
            },
            failure_count: self.consecutive_failure.load(Ordering::SeqCst),
            success_count: self.consecutive_success.load(Ordering::SeqCst),
            rejected_count: self.failed_requests.load(Ordering::SeqCst),
            last_failure_time: last_failure.map(|t| Utc::now()),
            last_success_time: last_success.map(|t| Utc::now()),
        }
    }

    /// 强制打开
    pub async fn force_open(&self) {
        let mut state = self.state.write().await;
        *state = State::Open;
        *self.open_started.write().await = Some(Instant::now());
        info!("[{}] Circuit breaker forced open", self.name);
    }

    /// 强制关闭
    pub async fn force_close(&self) {
        let mut state = self.state.write().await;
        *state = State::Closed;
        self.consecutive_failure.store(0, Ordering::SeqCst);
        self.consecutive_success.store(0, Ordering::SeqCst);
        info!("[{}] Circuit breaker forced close", self.name);
    }

    /// 重置
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        *state = State::Closed;
        self.consecutive_failure.store(0, Ordering::SeqCst);
        self.consecutive_success.store(0, Ordering::SeqCst);
        self.total_requests.store(0, Ordering::SeqCst);
        self.failed_requests.store(0, Ordering::SeqCst);
        *self.last_failure.write().await = None;
        *self.last_success.write().await = None;
        *self.half_open_started.write().await = None;
        *self.open_started.write().await = None;
        info!("[{}] Circuit breaker reset", self.name);
    }
}

/// 熔断器管理器
#[derive(Debug, Clone)]
pub struct CircuitBreakerManager {
    /// 熔断器映射
    breakers: Arc<RwLock<HashMap<String, Arc<CircuitBreaker>>>>,
    /// 默认配置
    default_config: Arc<CircuitBreakerConfig>,
}

impl CircuitBreakerManager {
    /// 创建熔断器管理器
    pub fn new(default_config: Arc<CircuitBreakerConfig>) -> Self {
        Self {
            breakers: Arc::new(RwLock::new(HashMap::new())),
            default_config,
        }
    }

    /// 获取或创建熔断器
    pub async fn get_or_create(&self, name: &str) -> Arc<CircuitBreaker> {
        {
            let breakers = self.breakers.read().await;
            if let Some(breaker) = breakers.get(name) {
                return breaker.clone();
            }
        }

        let mut breakers = self.breakers.write().await;

        // 双重检查
        if let Some(breaker) = breakers.get(name) {
            return breaker.clone();
        }

        let breaker = Arc::new(CircuitBreaker::from_config(
            name.to_string(),
            &self.default_config,
            self.default_config.route_overrides.get(name),
        ));

        breakers.insert(name.to_string(), breaker.clone());

        breaker
    }

    /// 获取熔断器
    pub async fn get(&self, name: &str) -> Option<Arc<CircuitBreaker>> {
        self.breakers.read().await.get(name).cloned()
    }

    /// 获取所有熔断器
    pub async fn all(&self) -> Vec<(String, Arc<CircuitBreaker>)> {
        self.breakers.read().await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// 获取指标
    pub async fn metrics(&self) -> Vec<(String, CircuitBreakerMetrics)> {
        let mut result = Vec::new();
        for (name, breaker) in self.breakers.read().await.iter() {
            result.push((name.clone(), breaker.metrics().await));
        }
        result
    }
}

/// 熔断器装饰器
#[derive(Debug, Clone)]
pub struct CircuitBreakerDecorator<C> {
    /// 熔断器
    breaker: Arc<CircuitBreaker>,
    /// 内部客户端
    client: C,
}

impl<C> CircuitBreakerDecorator<C>
where
    C: Clone,
{
    /// 创建装饰器
    pub fn new(breaker: Arc<CircuitBreaker>, client: C) -> Self {
        Self { breaker, client }
    }

    /// 执行受保护的调用
    pub async fn execute<F, T, E>(&self, f: impl FnOnce() -> F) -> GatewayResult<T>
    where
        F: Future<Output = std::result::Result<T, E>>,
        E: std::fmt::Display,
    {
        if !self.breaker.can_proceed().await {
            return Err(GatewayError::CircuitBreakerOpen);
        }

        match f().await {
            Ok(result) => {
                self.breaker.record_success().await;
                Ok(result)
            }
            Err(e) => {
                self.breaker.record_failure().await;
                Err(GatewayError::Internal {
                    message: e.to_string(),
                })
            }
        }
    }
}

/// 熔断器中间件
#[derive(Debug, Clone)]
pub struct CircuitBreakerMiddleware<S> {
    /// 熔断器
    breaker: Arc<CircuitBreaker>,
    /// 内部服务
    service: S,
}

impl<S> CircuitBreakerMiddleware<S>
where
    S: Clone,
{
    /// 创建中间件
    pub fn new(breaker: Arc<CircuitBreaker>, service: S) -> Self {
        Self { breaker, service }
    }
}

impl<S> tower::Service<http::Request<S>> for CircuitBreakerMiddleware<S>
where
    S: tower::Service<http::Request<S>> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = http::Response<axum::body::Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<S>) -> Self::Future {
        let breaker = self.breaker.clone();
        let mut service = self.service.clone();

        async move {
            if !breaker.can_proceed().await {
                let mut response = http::Response::new(axum::body::Body::empty());
                *response.status_mut() = http::StatusCode::SERVICE_UNAVAILABLE;
                response.headers_mut().insert(
                    "X-Circuit-Breaker",
                    http::HeaderValue::from_static("open"),
                );
                return Ok(response);
            }

            let result = service.call(req).await;

            match result {
                Ok(response) => {
                    let status = response.status();
                    if status.is_server_error() || status.is_client_error() {
                        breaker.record_failure().await;
                    } else {
                        breaker.record_success().await;
                    }
                    Ok(response)
                }
                Err(e) => {
                    breaker.record_failure().await;
                    Err(e)
                }
            }
        }
    }
}

// 导入 HashMap
use std::collections::HashMap;
