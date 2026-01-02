//! 幂等性控制中间件
//!
//! 使用 Redis 存储幂等键，确保重复请求返回相同结果

use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tonic::Status;
use tracing::{debug, warn};

/// 幂等键类型
pub type IdempotencyKey = String;

/// 幂等性记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdempotencyRecord {
    /// 幂等键
    pub key: IdempotencyKey,

    /// 响应状态码
    pub status_code: i32,

    /// 响应消息
    pub response_message: String,

    /// 响应数据（JSON）
    pub response_data: Option<String>,

    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// 过期时间
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl IdempotencyRecord {
    /// 创建新的幂等性记录
    pub fn new(
        key: IdempotencyKey,
        status_code: i32,
        response_message: String,
        response_data: Option<String>,
        ttl_seconds: i64,
    ) -> Self {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::seconds(ttl_seconds);

        Self {
            key,
            status_code,
            response_message,
            response_data,
            created_at: now,
            expires_at,
        }
    }

    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        self.expires_at < chrono::Utc::now()
    }
}

/// 幂等性结果
#[derive(Debug, Clone)]
pub enum IdempotencyResult<T> {
    /// 首次请求，正在处理中
    Processing,
    /// 已缓存的结果
    Cached(T),
}

/// 幂等性中间件
#[derive(Clone)]
pub struct IdempotencyMiddleware {
    /// Redis 连接
    redis: Arc<ConnectionManager>,

    /// 幂等键前缀
    key_prefix: String,

    /// TTL（秒）
    ttl_seconds: i64,
}

impl IdempotencyMiddleware {
    /// 创建新的幂等性中间件
    pub fn new(redis: Arc<ConnectionManager>, key_prefix: String, ttl_seconds: i64) -> Self {
        Self {
            redis,
            key_prefix,
            ttl_seconds,
        }
    }

    /// 生成 Redis 键
    fn redis_key(&self, key: &IdempotencyKey) -> String {
        format!("{}:idempotency:{}", self.key_prefix, key)
    }

    /// 获取幂等性记录
    pub async fn get_record(&self, key: &IdempotencyKey) -> RedisResult<Option<IdempotencyRecord>>> {
        let redis_key = self.redis_key(key);
        let data: Option<String> = self.redis.get(&redis_key).await?;

        match data {
            Some(json) => {
                let record: IdempotencyRecord = serde_json::from_str(&json)
                    .map_err(|e| redis::Error::from((redis::ErrorKind::ParseError, e.to_string())))?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    /// 保存幂等性记录
    pub async fn save_record(&self, record: &IdempotencyRecord) -> RedisResult<()> {
        let redis_key = self.redis_key(&record.key);
        let json = serde_json::to_string(record)
            .map_err(|e| redis::Error::from((redis::ErrorKind::ParseError, e.to_string())))?;

        // 设置过期时间
        let ttl = (record.expires_at - chrono::Utc::now())
            .num_seconds()
            .max(1);

        self.redis.set_ex(&redis_key, json, ttl as u64).await?;
        Ok(())
    }

    /// 尝试获取缓存的结果
    pub async fn try_get_cached_response<T>(
        &self,
        key: &IdempotencyKey,
    ) -> Result<Option<T>, Status>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.get_record(key).await {
            Ok(Some(record)) => {
                if record.is_expired() {
                    return Ok(None);
                }

                debug!(%key, "Idempotency key found, returning cached response");

                if let Some(data) = record.response_data {
                    let response: T = serde_json::from_str(&data)
                        .map_err(|e| Status::internal(format!("Failed to deserialize cached response: {}", e)))?;
                    return Ok(Some(response));
                }

                // 如果没有响应数据，根据状态码返回错误
                if record.status_code != 0 {
                    return Err(Status::new(
                        tonic::Code::from_i32(record.status_code),
                        record.response_message,
                    ));
                }

                Ok(None)
            }
            Ok(None) => Ok(None),
            Err(e) => {
                warn!(%key, "Failed to get idempotency record: {}", e);
                Ok(None) // 降级：允许请求继续
            }
        }
    }

    /// 开始处理请求（设置处理中标记）
    pub async fn start_processing(&self, key: &IdempotencyKey) -> Result<(), Status> {
        let redis_key = self.redis_key(key);
        let processing = serde_json::to_string(&IdempotencyRecord::new(
            key.clone(),
            -1, // -1 表示处理中
            "Processing".to_string(),
            None,
            self.ttl_seconds,
        )).map_err(|e| Status::internal(e.to_string()))?;

        // 使用 SET NX (只在不存在时设置)
        let result: bool = self
            .redis
            .set_nx(&redis_key, &processing)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if !result {
            // 键已存在，检查是否有缓存的结果
            match self.try_get_cached_response(key).await {
                Ok(Some(response)) => {
                    // 返回缓存的结果（这不会执行到这里，因为 try_get_cached_response 会直接返回）
                    Ok(())
                }
                Ok(None) => {
                    // 仍在处理中，返回冲突
                    Err(Status::aborted("Request is already being processed"))
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// 完成处理（保存结果）
    pub async fn complete_processing<T>(
        &self,
        key: &IdempotencyKey,
        response: &T,
    ) -> Result<(), Status>
    where
        T: serde::Serialize,
    {
        let response_data = serde_json::to_string(response)
            .map_err(|e| Status::internal(e.to_string()))?;

        let record = IdempotencyRecord::new(
            key.clone(),
            0, // 成功
            "OK".to_string(),
            Some(response_data),
            self.ttl_seconds,
        );

        self.save_record(&record).await
            .map_err(|e| Status::internal(format!("Failed to save idempotency record: {}", e)))
    }

    /// 处理失败（保存错误）
    pub async fn fail_processing(
        &self,
        key: &IdempotencyKey,
        status: &Status,
    ) -> Result<(), Status> {
        let record = IdempotencyRecord::new(
            key.clone(),
            status.code() as i32,
            status.message().to_string(),
            None,
            self.ttl_seconds,
        );

        self.save_record(&record).await
            .map_err(|e| Status::internal(format!("Failed to save idempotency record: {}", e)))
    }
}

/// 从请求中提取幂等键
pub fn extract_idempotency_key(request: &tonic::Request<()>) -> Option<IdempotencyKey> {
    request
        .metadata()
        .get("idempotency-key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// 装饰函数以支持幂等性
pub async fn with_idempotency<T, F, Fut>(
    middleware: &IdempotencyMiddleware,
    request: &tonic::Request<()>,
    f: F,
) -> Result<T, Status>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, Status>>,
{
    let idempotency_key = match extract_idempotency_key(request) {
        Some(key) => key,
        None => {
            // 没有幂等键，不使用幂等性控制
            return f().await;
        }
    };

    // 尝试获取缓存的响应
    if let Some(cached) = middleware.try_get_cached_response(&idempotency_key).await? {
        return Ok(cached);
    }

    // 尝试开始处理
    middleware.start_processing(&idempotency_key).await?;

    // 执行实际的处理
    match f().await {
        Ok(response) => {
            // 保存成功结果
            middleware.complete_processing(&idempotency_key, &response).await?;
            Ok(response)
        }
        Err(status) => {
            // 保存错误结果
            middleware.fail_processing(&idempotency_key, &status).await?;
            Err(status)
        }
    }
}
