//! 中间件装饰器
//!
//! 提供横切关注点的实现

use crate::{
    command::{Command, CommandHandler, CommandHandlerDecorator},
    context::CommandContext,
    error::Result,
};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 审计日志记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// 审计 ID
    pub audit_id: Uuid,

    /// 租户 ID
    pub tenant_id: String,

    /// 用户 ID
    pub user_id: String,

    /// 追踪 ID
    pub trace_id: Uuid,

    /// 命令名称
    pub command_name: String,

    /// 聚合根类型
    pub aggregate_type: Option<String>,

    /// 聚合根 ID
    pub aggregate_id: Option<String>,

    /// 命令负载（JSON）
    pub command_payload: String,

    /// 执行时间
    pub executed_at: chrono::DateTime<Utc>,

    /// 执行结果（成功/失败）
    pub success: bool,

    /// 错误信息（如果失败）
    pub error_message: Option<String>,

    /// 执行耗时（毫秒）
    pub duration_ms: u64,

    /// 元数据
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

impl AuditLog {
    /// 创建新的审计日志
    pub fn new(
        ctx: &CommandContext,
        command_name: impl Into<String>,
        command_payload: impl Into<String>,
    ) -> Self {
        Self {
            audit_id: Uuid::now_v7(),
            tenant_id: ctx.tenant_id.clone(),
            user_id: ctx.user_id.clone(),
            trace_id: ctx.trace_id,
            command_name: command_name.into(),
            aggregate_type: None,
            aggregate_id: None,
            command_payload: command_payload.into(),
            executed_at: Utc::now(),
            success: false,
            error_message: None,
            duration_ms: 0,
            metadata: ctx.metadata.clone(),
        }
    }

    /// 设置聚合根信息
    pub fn with_aggregate(
        mut self,
        aggregate_type: impl Into<String>,
        aggregate_id: impl Into<String>,
    ) -> Self {
        self.aggregate_type = Some(aggregate_type.into());
        self.aggregate_id = Some(aggregate_id.into());
        self
    }

    /// 标记为成功
    pub fn mark_success(mut self, duration_ms: u64) -> Self {
        self.success = true;
        self.duration_ms = duration_ms;
        self
    }

    /// 标记为失败
    pub fn mark_failure(mut self, duration_ms: u64, error: impl Into<String>) -> Self {
        self.success = false;
        self.duration_ms = duration_ms;
        self.error_message = Some(error.into());
        self
    }
}

/// 审计日志仓储接口
#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// 保存审计日志
    async fn save(&self, log: AuditLog) -> Result<()>;

    /// 批量保存审计日志
    async fn save_batch(&self, logs: Vec<AuditLog>) -> Result<()>;
}

/// 审计日志装饰器
///
/// 自动记录命令执行的审计日志
pub struct AuditLogDecorator<R: AuditLogRepository> {
    repository: R,
}

impl<R: AuditLogRepository> AuditLogDecorator<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<C, R> CommandHandlerDecorator<C> for AuditLogDecorator<R>
where
    C: Command,
    R: AuditLogRepository,
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
        // 序列化命令
        let command_payload = serde_json::to_string(&cmd)
            .unwrap_or_else(|_| "Failed to serialize command".to_string());

        // 创建审计日志
        let mut audit_log = AuditLog::new(ctx, C::command_name(), command_payload);

        // 执行命令
        let start = std::time::Instant::now();
        let result = handler.handle(ctx, cmd).await;
        let duration = start.elapsed();

        // 更新审计日志
        audit_log = match &result {
            Ok(_) => audit_log.mark_success(duration.as_millis() as u64),
            Err(e) => audit_log.mark_failure(duration.as_millis() as u64, e.to_string()),
        };

        // 保存审计日志（异步，不影响命令执行结果）
        if let Err(e) = self.repository.save(audit_log).await {
            tracing::error!(
                error = %e,
                command = C::command_name(),
                "Failed to save audit log"
            );
        }

        result
    }
}

/// 幂等性检查器接口
#[async_trait]
pub trait IdempotencyChecker: Send + Sync {
    /// 检查命令是否已执行
    async fn check(&self, command_id: &str, tenant_id: &str) -> Result<bool>;

    /// 记录命令已执行
    async fn record(&self, command_id: &str, tenant_id: &str) -> Result<()>;
}

/// 幂等性装饰器
///
/// 防止重复执行命令
pub struct IdempotencyDecorator<C: IdempotencyChecker> {
    checker: C,
}

impl<C: IdempotencyChecker> IdempotencyDecorator<C> {
    pub fn new(checker: C) -> Self {
        Self { checker }
    }
}

/// 幂等命令 trait
///
/// 需要幂等性检查的命令必须实现此 trait
pub trait IdempotentCommand: Command {
    /// 获取命令 ID（用于幂等性检查）
    fn command_id(&self) -> String;
}

#[async_trait]
impl<Cmd, C> CommandHandlerDecorator<Cmd> for IdempotencyDecorator<C>
where
    Cmd: IdempotentCommand,
    C: IdempotencyChecker,
{
    async fn decorate<H>(
        &self,
        handler: &H,
        ctx: &CommandContext,
        cmd: Cmd,
    ) -> Result<Cmd::Output>
    where
        H: CommandHandler<Cmd> + Send + Sync,
    {
        let command_id = cmd.command_id();

        // 检查是否已执行
        if self.checker.check(&command_id, &ctx.tenant_id).await? {
            tracing::warn!(
                command = Cmd::command_name(),
                command_id = %command_id,
                tenant_id = %ctx.tenant_id,
                "Command already executed (idempotency check)"
            );

            return Err(crate::error::AppError::IdempotencyConflict { command_id });
        }

        // 执行命令
        let result = handler.handle(ctx, cmd).await?;

        // 记录已执行
        self.checker.record(&command_id, &ctx.tenant_id).await?;

        Ok(result)
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

    struct MockAuditLogRepository;

    #[async_trait]
    impl AuditLogRepository for MockAuditLogRepository {
        async fn save(&self, _log: AuditLog) -> Result<()> {
            Ok(())
        }

        async fn save_batch(&self, _logs: Vec<AuditLog>) -> Result<()> {
            Ok(())
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
    async fn test_audit_log_creation() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let log = AuditLog::new(&ctx, "TestCommand", "{}");

        assert_eq!(log.tenant_id, "tenant-001");
        assert_eq!(log.user_id, "user-123");
        assert_eq!(log.command_name, "TestCommand");
        assert!(!log.success);
    }

    #[tokio::test]
    async fn test_audit_log_decorator() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let cmd = TestCommand {
            value: "test".to_string(),
        };
        let handler = TestHandler;
        let decorator = AuditLogDecorator::new(MockAuditLogRepository);

        let result = decorator.decorate(&handler, &ctx, cmd).await.unwrap();
        assert_eq!(result, "Processed: test");
    }
}
