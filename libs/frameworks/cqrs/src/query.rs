//! 查询系统
//!
//! 定义 Query trait 和 QueryHandler trait，专注于高性能只读操作

use crate::{context::CommandContext, error::Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// 查询标记 trait
///
/// 所有查询必须实现此 trait
pub trait Query: Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static {
    /// 查询的输出类型
    type Output: Debug + Serialize + Send + Sync + 'static;

    /// 查询名称（用于日志和追踪）
    fn query_name() -> &'static str;

    /// 是否可缓存
    fn cacheable() -> bool {
        false
    }

    /// 缓存 TTL（秒）
    fn cache_ttl() -> Option<u64> {
        None
    }

    /// 缓存键（如果可缓存）
    fn cache_key(&self) -> Option<String> {
        None
    }
}

/// 查询处理器
///
/// 负责执行查询并返回结果
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// 处理查询
    ///
    /// # 参数
    /// - `ctx`: 命令上下文（包含租户、用户等信息）
    /// - `query`: 要执行的查询
    ///
    /// # 返回
    /// 查询结果
    async fn handle(&self, ctx: &CommandContext, query: Q) -> Result<Q::Output>;
}

/// 查询处理器装饰器
///
/// 用于实现横切关注点（缓存、日志等）
#[async_trait]
pub trait QueryHandlerDecorator<Q: Query>: Send + Sync {
    /// 装饰处理器
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        query: Q,
    ) -> Result<Q::Output>
    where
        H: QueryHandler<Q> + Send + Sync;
}

/// 日志装饰器
///
/// 自动记录查询执行日志
pub struct QueryLoggingDecorator;

impl QueryLoggingDecorator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for QueryLoggingDecorator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<Q> QueryHandlerDecorator<Q> for QueryLoggingDecorator
where
    Q: Query,
{
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        query: Q,
    ) -> Result<Q::Output>
    where
        H: QueryHandler<Q> + Send + Sync,
    {
        let span = ctx.create_span(Q::query_name());
        let _enter = span.enter();

        tracing::debug!(
            query = Q::query_name(),
            tenant_id = %ctx.tenant_id,
            user_id = %ctx.user_id,
            "Executing query"
        );

        let start = std::time::Instant::now();
        let result = handler.handle(ctx, query).await;
        let duration = start.elapsed();

        match &result {
            Ok(_) => {
                tracing::debug!(
                    query = Q::query_name(),
                    duration_ms = duration.as_millis(),
                    "Query executed successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    query = Q::query_name(),
                    duration_ms = duration.as_millis(),
                    error = %e,
                    error_code = e.error_code(),
                    "Query execution failed"
                );
            }
        }

        result
    }
}

/// 缓存装饰器接口
///
/// 提供查询结果缓存功能
#[async_trait]
pub trait CacheProvider: Send + Sync {
    /// 获取缓存
    async fn get(&self, key: &str) -> Result<Option<String>>;

    /// 设置缓存
    async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<()>;

    /// 删除缓存
    async fn delete(&self, key: &str) -> Result<()>;
}

/// 缓存装饰器
///
/// 自动缓存查询结果
pub struct CachingDecorator<C: CacheProvider> {
    cache: C,
}

impl<C: CacheProvider> CachingDecorator<C> {
    pub fn new(cache: C) -> Self {
        Self { cache }
    }
}

#[async_trait]
impl<Q, C> QueryHandlerDecorator<Q> for CachingDecorator<C>
where
    Q: Query,
    Q::Output: serde::de::DeserializeOwned,
    C: CacheProvider,
{
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        query: Q,
    ) -> Result<Q::Output>
    where
        H: QueryHandler<Q> + Send + Sync,
    {
        // 如果不可缓存，直接执行
        if !Q::cacheable() {
            return handler.handle(ctx, query).await;
        }

        // 获取缓存键
        let cache_key = match query.cache_key() {
            Some(key) => format!("{}:{}:{}", ctx.tenant_id, Q::query_name(), key),
            None => return handler.handle(ctx, query).await,
        };

        // 尝试从缓存获取
        if let Some(cached) = self.cache.get(&cache_key).await? {
            tracing::debug!(
                query = Q::query_name(),
                cache_key = %cache_key,
                "Cache hit"
            );

            return serde_json::from_str(&cached).map_err(|e| {
                crate::error::AppError::Internal(format!("Failed to deserialize cached result: {}", e))
            });
        }

        // 缓存未命中，执行查询
        tracing::debug!(
            query = Q::query_name(),
            cache_key = %cache_key,
            "Cache miss"
        );

        let result = handler.handle(ctx, query).await?;

        // 缓存结果
        if let Some(ttl) = Q::cache_ttl() {
            let serialized = serde_json::to_string(&result).map_err(|e| {
                crate::error::AppError::Internal(format!("Failed to serialize result: {}", e))
            })?;

            if let Err(e) = self.cache.set(&cache_key, &serialized, ttl).await {
                tracing::warn!(
                    error = %e,
                    "Failed to cache query result"
                );
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestQuery {
        id: String,
    }

    impl Query for TestQuery {
        type Output = String;

        fn query_name() -> &'static str {
            "TestQuery"
        }

        fn cacheable() -> bool {
            true
        }

        fn cache_ttl() -> Option<u64> {
            Some(300)
        }

        fn cache_key(&self) -> Option<String> {
            Some(self.id.clone())
        }
    }

    struct TestQueryHandler;

    #[async_trait]
    impl QueryHandler<TestQuery> for TestQueryHandler {
        async fn handle(&self, _ctx: &CommandContext, query: TestQuery) -> Result<String> {
            Ok(format!("Result for: {}", query.id))
        }
    }

    #[tokio::test]
    async fn test_query_handler() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let query = TestQuery {
            id: "test-id".to_string(),
        };
        let handler = TestQueryHandler;

        let result = handler.handle(&ctx, query).await.unwrap();
        assert_eq!(result, "Result for: test-id");
    }

    #[tokio::test]
    async fn test_query_logging_decorator() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let query = TestQuery {
            id: "test-id".to_string(),
        };
        let handler = TestQueryHandler;
        let decorator = QueryLoggingDecorator::new();

        let result = decorator.decorate(&handler, &ctx, query).await.unwrap();
        assert_eq!(result, "Result for: test-id");
    }
}
