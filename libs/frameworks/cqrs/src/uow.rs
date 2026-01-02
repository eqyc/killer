//! Unit of Work 模式
//!
//! 确保命令处理和事件存储在同一事务中

use crate::{
    command::{Command, CommandHandler},
    context::CommandContext,
    error::Result,
    event::{DomainEvent, EventEnvelope, OutboxEvent, OutboxRepository},
};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 带事件收集的 Unit of Work
///
/// 管理事务边界，确保命令处理和事件存储的原子性
pub struct UnitOfWork<R: OutboxRepository> {
    outbox: Arc<R>,
    events: Arc<Mutex<Vec<OutboxEvent>>>,
}

impl<R: OutboxRepository> UnitOfWork<R> {
    /// 创建新的 UoW
    pub fn new(outbox: Arc<R>) -> Self {
        Self {
            outbox,
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加事件到 UoW（自动转换为 OutboxEvent）
    pub async fn add_event<E: DomainEvent>(&self, envelope: EventEnvelope<E>) -> Result<()> {
        let outbox_event = OutboxEvent::from_envelope(&envelope)?;
        let mut events = self.events.lock().await;
        events.push(outbox_event);
        Ok(())
    }

    /// 添加预转换的 OutboxEvent
    pub async fn add_outbox_event(&self, event: OutboxEvent) {
        let mut events = self.events.lock().await;
        events.push(event);
    }

    /// 提交 UoW（保存所有事件到 Outbox）
    pub async fn commit(&self) -> Result<()> {
        let events = self.events.lock().await;

        // 批量保存到 Outbox
        if !events.is_empty() {
            self.outbox.save_batch(events.clone()).await?;
        }

        Ok(())
    }

    /// 回滚 UoW（清空事件）
    pub async fn rollback(&self) {
        let mut events = self.events.lock().await;
        events.clear();
    }

    /// 获取事件数量
    pub async fn event_count(&self) -> usize {
        self.events.lock().await.len()
    }
}

/// UoW 装饰器
///
/// 自动管理事务边界
pub struct UnitOfWorkDecorator<R: OutboxRepository> {
    outbox: Arc<R>,
}

impl<R: OutboxRepository> UnitOfWorkDecorator<R> {
    pub fn new(outbox: Arc<R>) -> Self {
        Self { outbox }
    }

    /// 创建新的 UoW
    pub fn create_uow(&self) -> UnitOfWork<R> {
        UnitOfWork::new(self.outbox.clone())
    }
}

#[async_trait]
impl<C, R> crate::command::CommandHandlerDecorator<C> for UnitOfWorkDecorator<R>
where
    C: Command,
    R: OutboxRepository + 'static,
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
        let uow = self.create_uow();

        // 执行命令
        let result = handler.handle(ctx, cmd).await;

        match result {
            Ok(output) => {
                // 提交事务
                uow.commit().await?;
                Ok(output)
            }
            Err(e) => {
                // 回滚事务
                uow.rollback().await;
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{OutboxEvent, OutboxRepository};
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    struct MockOutboxRepository;

    #[async_trait]
    impl OutboxRepository for MockOutboxRepository {
        async fn save(&self, _event: OutboxEvent) -> Result<()> {
            Ok(())
        }

        async fn save_batch(&self, _events: Vec<OutboxEvent>) -> Result<()> {
            Ok(())
        }

        async fn get_unpublished(&self, _limit: usize) -> Result<Vec<OutboxEvent>> {
            Ok(Vec::new())
        }

        async fn mark_published(&self, _event_id: Uuid) -> Result<()> {
            Ok(())
        }

        async fn mark_failed(&self, _event_id: Uuid, _error: String) -> Result<()> {
            Ok(())
        }

        async fn delete_published_before(&self, _before: DateTime<Utc>) -> Result<u64> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_uow_creation() {
        let outbox = Arc::new(MockOutboxRepository);
        let uow = UnitOfWork::new(outbox);

        // UoW 应该被正确创建
        assert_eq!(uow.event_count().await, 0);
    }

    #[tokio::test]
    async fn test_uow_commit() {
        let outbox = Arc::new(MockOutboxRepository);
        let uow = UnitOfWork::new(outbox);

        // 提交应该成功
        assert!(uow.commit().await.is_ok());
    }

    #[tokio::test]
    async fn test_uow_rollback() {
        let outbox = Arc::new(MockOutboxRepository);
        let uow = UnitOfWork::new(outbox);

        uow.rollback().await;

        // 事件应该被清空
        assert_eq!(uow.event_count().await, 0);
    }
}
