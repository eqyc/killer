//! 命令系统
//!
//! 定义 Command trait 和 CommandHandler trait

use crate::{context::CommandContext, error::Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use validator::Validate;

/// 命令标记 trait
///
/// 所有命令必须实现此 trait
pub trait Command: Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static {
    /// 命令的输出类型
    type Output: Debug + Send + Sync + 'static;

    /// 命令名称（用于日志和追踪）
    fn command_name() -> &'static str;
}

/// 命令处理器
///
/// 负责执行命令并返回结果
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// 处理命令
    ///
    /// # 参数
    /// - `ctx`: 命令上下文
    /// - `cmd`: 要执行的命令
    ///
    /// # 返回
    /// 命令执行结果
    async fn handle(&self, ctx: &CommandContext, cmd: C) -> Result<C::Output>;
}

/// 命令处理器装饰器
///
/// 用于实现横切关注点（日志、验证、事务等）
#[async_trait]
pub trait CommandHandlerDecorator<C: Command>: Send + Sync {
    /// 装饰处理器
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        cmd: C,
    ) -> Result<C::Output>
    where
        H: CommandHandler<C> + Send + Sync;
}

/// 验证装饰器
///
/// 在执行命令前自动验证
pub struct ValidationDecorator;

impl ValidationDecorator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ValidationDecorator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<C> CommandHandlerDecorator<C> for ValidationDecorator
where
    C: Command + Validate,
{
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        cmd: C,
    ) -> Result<C::Output>
    where
        H: CommandHandler<C> + Send + Sync,
    {
        // 执行验证
        cmd.validate()
            .map_err(|e| crate::error::AppError::validation_failed(e.to_string()))?;

        // 调用实际处理器
        handler.handle(ctx, cmd).await
    }
}

/// 日志装饰器
///
/// 自动记录命令执行日志
pub struct LoggingDecorator;

impl LoggingDecorator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoggingDecorator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<C> CommandHandlerDecorator<C> for LoggingDecorator
where
    C: Command,
{
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        cmd: C,
    ) -> Result<C::Output>
    where
        H: CommandHandler<C> + Send + Sync,
    {
        let span = ctx.create_span(C::command_name());
        let _enter = span.enter();

        tracing::info!(
            command = C::command_name(),
            tenant_id = %ctx.tenant_id,
            user_id = %ctx.user_id,
            "Executing command"
        );

        let start = std::time::Instant::now();
        let result = handler.handle(ctx, cmd).await;
        let duration = start.elapsed();

        match &result {
            Ok(_) => {
                tracing::info!(
                    command = C::command_name(),
                    duration_ms = duration.as_millis(),
                    "Command executed successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    command = C::command_name(),
                    duration_ms = duration.as_millis(),
                    error = %e,
                    error_code = e.error_code(),
                    "Command execution failed"
                );
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestCommand {
        value: String,
    }

    impl Command for TestCommand {
        type Output = String;

        fn command_name() -> &'static str {
            "TestCommand"
        }
    }

    struct TestHandler;

    #[async_trait]
    impl CommandHandler<TestCommand> for TestHandler {
        async fn handle(&self, _ctx: &CommandContext, cmd: TestCommand) -> Result<String> {
            Ok(format!("Processed: {}", cmd.value))
        }
    }

    #[tokio::test]
    async fn test_command_handler() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let cmd = TestCommand {
            value: "test".to_string(),
        };
        let handler = TestHandler;

        let result = handler.handle(&ctx, cmd).await.unwrap();
        assert_eq!(result, "Processed: test");
    }

    #[tokio::test]
    async fn test_logging_decorator() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let cmd = TestCommand {
            value: "test".to_string(),
        };
        let handler = TestHandler;
        let decorator = LoggingDecorator::new();

        let result = decorator.decorate(&handler, &ctx, cmd).await.unwrap();
        assert_eq!(result, "Processed: test");
    }
}
