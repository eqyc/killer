//! 熔断器
//!
//! 实现熔断器模式，保护外部服务调用
//! 支持半开状态自动恢复

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

// =============================================================================
// 熔断器状态
// =============================================================================

/// 熔断器状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    /// 关闭状态，正常请求
    Closed,
    /// 半开状态，尝试恢复
    HalfOpen,
    /// 打开状态，阻止请求
    Open,
}

impl CircuitBreakerState {
    /// 是否允许请求
    pub fn allows_requests(&self) -> bool {
        match self {
            CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => true,
            CircuitBreakerState::Open => false,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            CircuitBreakerState::Closed => "closed",
            CircuitBreakerState::HalfOpen => "half_open",
            CircuitBreakerState::Open => "open",
        }
    }
}

// =============================================================================
// 熔断器配置
// =============================================================================

/// 熔断器配置
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// 失败阈值（连续失败次数）
    pub failure_threshold: u64,
    /// 成功阈值（半开状态下的成功次数）
    pub success_threshold: u64,
    /// 超时时间（进入半开状态前的等待时间）
    pub timeout: Duration,
    /// 半开状态下允许的请求数
    pub half_open_max_requests: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            half_open_max_requests: 3,
        }
    }
}

// =============================================================================
// 熔断器
// =============================================================================

/// 熔断器
#[derive(Debug)]
pub struct CircuitBreaker {
    /// 服务名称
    name: String,
    /// 配置
    config: CircuitBreakerConfig,
    /// 当前状态
    state: Arc<Mutex<CircuitBreakerState>>,
    /// 状态变更时间
    state_changed_at: Arc<AtomicU64>,
    /// 连续失败计数
    consecutive_failures: Arc<AtomicU64>,
    /// 半开状态下的请求计数
    half_open_requests: Arc<AtomicU64>,
    /// 半开状态下的成功计数
    half_open_successes: Arc<AtomicU64>,
}

impl CircuitBreaker {
    /// 创建新的熔断器
    pub fn new(name: &str, config: Option<CircuitBreakerConfig>) -> Self {
        Self {
            name: name.to_string(),
            config: config.unwrap_or_default(),
            state: Arc::new(Mutex::new(CircuitBreakerState::Closed)),
            state_changed_at: Arc::new(AtomicU64::new(0)),
            consecutive_failures: Arc::new(AtomicU64::new(0)),
            half_open_requests: Arc::new(AtomicU64::new(0)),
            half_open_successes: Arc::new(AtomicU64::new(0)),
        }
    }

    /// 获取当前状态
    pub async fn state(&self) -> CircuitBreakerState {
        let state = *self.state.lock().await;

        // 检查是否需要从 Open 转换为 HalfOpen
        if state == CircuitBreakerState::Open {
            let state_changed_at = self.state_changed_at.load(Ordering::Relaxed);
            if state_changed_at > 0 {
                let elapsed = Duration::from_secs(state_changed_at..now_secs());
                if elapsed >= self.config.timeout {
                    self.move_to_half_open().await;
                    return *self.state.lock().await;
                }
            }
        }

        state
    }

    /// 尝试执行操作
    pub async fn call<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::error::Error,
    {
        // 检查状态
        let state = self.state().await;
        if !state.allows_requests() {
            return Err(E::from(std::fmt::Error));
        }

        // 半开状态下限制请求数
        if state == CircuitBreakerState::HalfOpen {
            let requests = self.half_open_requests.fetch_add(1, Ordering::SeqCst);
            if requests >= self.config.half_open_max_requests {
                return Err(E::from(std::fmt::Error));
            }
        }

        // 执行操作
        match operation() {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }

    /// 成功回调
    async fn on_success(&self) {
        let state = *self.state.lock().await;

        match state {
            CircuitBreakerState::Closed => {
                // 重置失败计数
                self.consecutive_failures.store(0, Ordering::SeqCst);
            }
            CircuitBreakerState::HalfOpen => {
                // 增加成功计数
                let successes = self.half_open_successes.fetch_add(1, Ordering::SeqCst) + 1;
                if successes >= self.config.success_threshold {
                    self.move_to_closed().await;
                }
            }
            CircuitBreakerState::Open => {
                // 不应该发生
            }
        }
    }

    /// 失败回调
    async fn on_failure(&self) {
        let state = *self.state.lock().await;

        match state {
            CircuitBreakerState::Closed => {
                // 增加失败计数
                let failures = self.consecutive_failures.fetch_add(1, Ordering::SeqCst) + 1;
                if failures >= self.config.failure_threshold {
                    self.move_to_open().await;
                }
            }
            CircuitBreakerState::HalfOpen => {
                // 任何失败都回到 Open 状态
                self.move_to_open().await;
            }
            CircuitBreakerState::Open => {
                // 不应该发生
            }
        }
    }

    /// 移动到关闭状态
    async fn move_to_closed(&self) {
        let mut state = self.state.lock().await;
        if *state != CircuitBreakerState::Closed {
            *state = CircuitBreakerState::Closed;
            self.state_changed_at.store(0, Ordering::SeqCst);
            self.consecutive_failures.store(0, Ordering::SeqCst);
            self.half_open_requests.store(0, Ordering::SeqCst);
            self.half_open_successes.store(0, Ordering::SeqCst);
            info!(service = %self.name, "Circuit breaker closed");
        }
    }

    /// 移动到半开状态
    async fn move_to_half_open(&self) {
        let mut state = self.state.lock().await;
        if *state == CircuitBreakerState::Open {
            *state = CircuitBreakerState::HalfOpen;
            self.state_changed_at.store(now_secs(), Ordering::SeqCst);
            self.half_open_requests.store(0, Ordering::SeqCst);
            self.half_open_successes.store(0, Ordering::SeqCst);
            info!(service = %self.name, "Circuit breaker half-open");
        }
    }

    /// 移动到打开状态
    async fn move_to_open(&self) {
        let mut state = self.state.lock().await;
        if *state != CircuitBreakerState::Open {
            *state = CircuitBreakerState::Open;
            self.state_changed_at.store(now_secs(), Ordering::SeqCst);
            info!(service = %self.name, "Circuit breaker opened");
        }
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// =============================================================================
// 带熔断器的客户端
// =============================================================================

/// 带熔断器保护的客户端
#[derive(Clone)]
pub struct CircuitBreakerClient<C> {
    /// 内部客户端
    client: C,
    /// 熔断器
    breaker: Arc<CircuitBreaker>,
}

impl<C> CircuitBreakerClient<C>
where
    C: Clone,
{
    /// 创建新的受保护客户端
    pub fn new(client: C, breaker: Arc<CircuitBreaker>) -> Self {
        Self { client, breaker }
    }

    /// 获取熔断器
    pub fn circuit_breaker(&self) -> &Arc<CircuitBreaker> {
        &self.breaker
    }
}

// =============================================================================
// 宏：快速创建带熔断器的操作
// =============================================================================

#[macro_export]
macro_rules! with_circuit_breaker {
    ($breaker:expr, $operation:expr) => {
        $breaker.call(|| $operation).await
    };
}
