//! Redis 缓存
//!
//! 提供 Redis 缓存客户端封装
//! 支持 TTL、连接池、分布式锁

use async_trait::async_trait;
use redis::{Cmd, ConnectionLike, RedisResult};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, trace};

// =============================================================================
// 缓存 trait
// =============================================================================

/// 缓存接口
#[async_trait]
pub trait Cache: Send + Sync {
    /// 获取值
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>>;

    /// 设置值
    async fn set(&self, key: &str, value: &str, ttl: Duration) -> Result<(), Box<dyn std::error::Error>>;

    /// 删除值
    async fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// 检查键是否存在
    async fn exists(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>>;

    /// 批量获取
    async fn get_multi(&self, keys: &[String]) -> Result<Vec<(String, Option<String>)>, Box<dyn std::error::Error>>;

    /// 批量设置
    async fn set_multi(&self, items: &[(&str, &str)], ttl: Duration) -> Result<(), Box<dyn std::error::Error>>;
}

// =============================================================================
// Redis 缓存实现
// =============================================================================

/// Redis 缓存实现
#[derive(Clone)]
pub struct RedisCache {
    /// Redis 连接管理
    connection: ConnectionManager,
    /// 指标
    metrics: Arc<super::AdapterMetrics>,
}

impl RedisCache {
    /// 创建新的 Redis 缓存
    pub fn new(connection: ConnectionManager, metrics: Arc<super::AdapterMetrics>) -> Self {
        Self {
            connection,
            metrics,
        }
    }

    /// 从连接 URL 创建
    pub async fn from_url(url: &str, metrics: Arc<super::AdapterMetrics>) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        let connection = client.get_connection_manager().await?;
        Ok(Self::new(connection, metrics))
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let result: RedisResult<Option<String>> = Cmd::get(key).query_async(&mut self.connection.clone()).await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "get", result.is_ok(), duration);

        match result {
            Ok(value) => {
                if value.is_some() {
                    self.metrics.record_cache_hit("redis");
                    debug!(%key, "Cache hit");
                } else {
                    self.metrics.record_cache_miss("redis");
                    debug!(%key, "Cache miss");
                }
                Ok(value)
            }
            Err(e) => {
                self.metrics.record_error("redis", "get_error");
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    async fn set(&self, key: &str, value: &str, ttl: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let result: RedisResult<()> = Cmd::set_ex(key, value, ttl.as_secs() as usize)
            .query_async(&mut self.connection.clone())
            .await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "set", result.is_ok(), duration);

        result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        debug!(%key, ttl_secs = %ttl.as_secs(), "Cache set");
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let result: RedisResult<()> = Cmd::del(key).query_async(&mut self.connection.clone()).await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "del", result.is_ok(), duration);

        result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        debug!(%key, "Cache delete");
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();

        let result: RedisResult<bool> = Cmd::exists(key).query_async(&mut self.connection.clone()).await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "exists", result.is_ok(), duration);

        result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    async fn get_multi(&self, keys: &[String]) -> Result<Vec<(String, Option<String>)>, Box<dyn std::error::Error>> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let start = std::time::Instant::now();

        let mut cmd = redis::cmd("MGET");
        for key in keys {
            cmd.arg(key);
        }

        let result: RedisResult<Vec<Option<String>>> = cmd.query_async(&mut self.connection.clone()).await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "mget", result.is_ok(), duration);

        let values = result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let mut results = Vec::new();
        for (i, value) in values.into_iter().enumerate() {
            results.push((keys[i].clone(), value));
        }

        Ok(results)
    }

    async fn set_multi(&self, items: &[(&str, &str)], ttl: Duration) -> Result<(), Box<dyn std::error::Error>> {
        if items.is_empty() {
            return Ok(());
        }

        let start = std::time::Instant::now();

        // 使用 pipeline 批量设置
        let mut pipe = redis::pipe();
        for (key, value) in items {
            pipe.set_ex(key, *value, ttl.as_secs() as usize);
        }

        let result: RedisResult<()> = pipe.query_async(&mut self.connection.clone()).await;

        let duration = start.elapsed();
        self.metrics.record_call("redis", "mset", result.is_ok(), duration);

        result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        debug!(count = items.len(), ttl_secs = %ttl.as_secs(), "Cache multi set");
        Ok(())
    }
}

// =============================================================================
// 分布式锁
// =============================================================================

/// 分布式锁
#[derive(Clone)]
pub struct DistributedLock {
    cache: Arc<RedisCache>,
    lock_prefix: String,
}

impl DistributedLock {
    /// 创建新的分布式锁
    pub fn new(cache: Arc<RedisCache>, lock_prefix: &str) -> Self {
        Self {
            cache,
            lock_prefix: lock_prefix.to_string(),
        }
    }

    /// 尝试获取锁
    pub async fn try_lock(&self, name: &str, ttl: Duration) -> Result<Option<LockGuard>, Box<dyn std::error::Error>> {
        let lock_key = format!("{}:lock:{}", self.lock_prefix, name);
        let lock_value = uuid::Uuid::new_v4().to_string();

        // 使用 SET NX EX 原子操作
        let result: RedisResult<bool> = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&lock_value)
            .arg("NX")
            .arg("EX")
            .arg(ttl.as_secs())
            .query_async(&mut self.cache.connection.clone())
            .await;

        match result {
            Ok(true) => {
                debug!(%lock_key, "Lock acquired");
                Ok(Some(LockGuard {
                    cache: self.cache.clone(),
                    key: lock_key,
                    value: lock_value,
                }))
            }
            Ok(false) => {
                debug!(%lock_key, "Lock not acquired (already held)");
                Ok(None)
            }
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
        }
    }

    /// 释放锁（仅当持有者才能释放）
    pub async fn unlock(&self, guard: LockGuard) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 Lua 脚本原子检查并删除
        let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;

        let result: RedisResult<i32> = redis::cmd("EVAL")
            .arg(script)
            .arg(1)
            .arg(&guard.key)
            .arg(&guard.value)
            .query_async(&mut self.cache.connection.clone())
            .await;

        match result {
            Ok(1) => {
                debug!(%guard.key, "Lock released");
                Ok(())
            }
            _ => {
                debug!(%guard.key, "Lock not released (not held by this instance)");
                Ok(())
            }
        }
    }
}

/// 锁守护者
pub struct LockGuard {
    cache: Arc<RedisCache>,
    key: String,
    value: String,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // 尝试释放锁（忽略错误）
        let cache = self.cache.clone();
        let key = self.key.clone();
        let value = self.value.clone();

        tokio::spawn(async move {
            let script = r#"
                if redis.call("get", KEYS[1]) == ARGV[1] then
                    return redis.call("del", KEYS[1])
                else
                    return 0
                end
            "#;

            let _ = redis::cmd("EVAL")
                .arg(script)
                .arg(1)
                .arg(&key)
                .arg(&value)
                .query_async(&mut cache.connection.clone())
                .await;
        });
    }
}
