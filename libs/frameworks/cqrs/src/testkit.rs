//! 测试工具包
//!
//! 提供测试辅助工具

use crate::{
    command::{Command, CommandHandler},
    context::CommandContext,
    error::{AppError, Result},
    event::{DomainEvent, EventEnvelope, EventHandler, OutboxEvent, OutboxRepository},
    query::{Query, QueryHandler},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

// ============ Mock EventBus ============

/// Mock 事件总线（用于测试）
#[derive(Default, Clone)]
pub struct MockEventBus;

impl MockEventBus {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::event::EventBus for MockEventBus {
    async fn publish_sync<E: DomainEvent>(&self, _envelope: EventEnvelope<E>) -> Result<()> {
        Ok(())
    }

    async fn publish_async<E: DomainEvent>(&self, _envelope: EventEnvelope<E>) -> Result<()> {
        Ok(())
    }

    async fn publish_async_batch<E: DomainEvent>(
        &self,
        _envelopes: Vec<EventEnvelope<E>>,
    ) -> Result<()> {
        Ok(())
    }
}

// ============ Mock Outbox Repository ============

/// Mock Outbox Repository（用于测试）
#[derive(Default)]
pub struct MockOutboxRepository {
    events: Arc<std::sync::Mutex<Vec<OutboxEvent>>>,
}

impl MockOutboxRepository {
    pub fn new() -> Self {
        Self {
            events: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn get_all_events(&self) -> Vec<OutboxEvent> {
        let events = self.events.lock().unwrap();
        events.clone()
    }
}

#[async_trait]
impl OutboxRepository for MockOutboxRepository {
    async fn save(&self, event: OutboxEvent) -> Result<()> {
        let mut events = self.events.lock().unwrap();
        events.push(event);
        Ok(())
    }

    async fn save_batch(&self, events: Vec<OutboxEvent>) -> Result<()> {
        let mut stored = self.events.lock().unwrap();
        stored.extend(events);
        Ok(())
    }

    async fn get_unpublished(&self, limit: usize) -> Result<Vec<OutboxEvent>> {
        let events = self.events.lock().unwrap();
        Ok(events
            .iter()
            .filter(|e| !e.published)
            .take(limit)
            .cloned()
            .collect())
    }

    async fn mark_published(&self, event_id: Uuid) -> Result<()> {
        let mut events = self.events.lock().unwrap();
        for event in events.iter_mut() {
            if event.event_id == event_id {
                event.published = true;
                event.published_at = Some(Utc::now());
                break;
            }
        }
        Ok(())
    }

    async fn mark_failed(&self, event_id: Uuid, error: String) -> Result<()> {
        let mut events = self.events.lock().unwrap();
        for event in events.iter_mut() {
            if event.event_id == event_id {
                event.retry_count += 1;
                event.last_error = Some(error);
                break;
            }
        }
        Ok(())
    }

    async fn delete_published_before(&self, before: DateTime<Utc>) -> Result<u64> {
        let mut events = self.events.lock().unwrap();
        let before_len = events.len();
        events.retain(|e| !e.published || e.published_at.map_or(true, |t| t >= before));
        Ok((before_len - events.len()) as u64)
    }
}

// ============ Mock Command Handler ============

/// Mock Command Handler（用于测试）
#[derive(Clone)]
pub struct MockCommandHandler<C: Command> {
    handler: Arc<dyn Fn(&CommandContext, C) -> Result<C::Output> + Send + Sync>,
}

impl<C: Command> MockCommandHandler<C> {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&CommandContext, C) -> Result<C::Output> + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(handler),
        }
    }
}

#[async_trait]
impl<C: Command> CommandHandler<C> for MockCommandHandler<C> {
    async fn handle(&self, ctx: &CommandContext, cmd: C) -> Result<C::Output> {
        (self.handler)(ctx, cmd)
    }
}

/// Mock Query Handler（用于测试）
#[derive(Clone)]
pub struct MockQueryHandler<Q: Query> {
    handler: Arc<dyn Fn(&CommandContext, Q) -> Result<Q::Output> + Send + Sync>,
}

impl<Q: Query> MockQueryHandler<Q> {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&CommandContext, Q) -> Result<Q::Output> + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(handler),
        }
    }
}

#[async_trait]
impl<Q: Query> QueryHandler<Q> for MockQueryHandler<Q>
where
    Q::Output: Send + Sync + Serialize + 'static,
{
    async fn handle(&self, ctx: &CommandContext, query: Q) -> Result<Q::Output> {
        (self.handler)(ctx, query)
    }
}

/// Mock Event Handler（用于测试）
#[derive(Clone)]
pub struct MockEventHandler<E: DomainEvent> {
    handler: Arc<dyn Fn(&EventEnvelope<E>) -> Result<()> + Send + Sync>,
    pub call_count: Arc<std::sync::Mutex<u32>>,
}

impl<E: DomainEvent> MockEventHandler<E> {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&EventEnvelope<E>) -> Result<()> + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(handler),
            call_count: Arc::new(std::sync::Mutex::new(0)),
        }
    }

    pub fn call_count(&self) -> u32 {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait]
impl<E: DomainEvent> EventHandler<E> for MockEventHandler<E> {
    async fn handle(&self, envelope: &EventEnvelope<E>) -> Result<()> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        (self.handler)(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{command::Command, context::CommandContext, event::EventBus, query::Query};
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        message: String,
    }

    impl crate::event::DomainEvent for TestEvent {
        fn event_name() -> &'static str {
            "TestEvent"
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestQuery {
        id: String,
    }

    impl Query for TestQuery {
        type Output = String;

        fn query_name() -> &'static str {
            "TestQuery"
        }
    }

    #[tokio::test]
    async fn test_mock_event_bus() {
        let bus = MockEventBus::new();
        let event = TestEvent {
            message: "test".to_string(),
        };
        let envelope = EventEnvelope::new("agg-123", "TestAggregate", 1, "tenant-001", event);

        bus.publish_sync(envelope).await.unwrap();
    }

    #[tokio::test]
    async fn test_mock_outbox_repository() {
        let repo = MockOutboxRepository::new();
        let event = OutboxEvent {
            event_id: Uuid::now_v7(),
            event_name: "TestEvent".to_string(),
            aggregate_id: "agg-123".to_string(),
            aggregate_type: "TestAggregate".to_string(),
            tenant_id: "tenant-001".to_string(),
            payload: "{}".to_string(),
            occurred_at: Utc::now(),
            published: false,
            published_at: None,
            retry_count: 0,
            last_error: None,
        };

        repo.save(event).await.unwrap();
        assert_eq!(repo.get_unpublished(10).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_mock_command_handler() {
        let ctx = CommandContext::new("tenant-001", "user-123");
        let handler = MockCommandHandler::<TestCommand>::new(|_ctx, cmd: TestCommand| {
            Ok(format!("processed: {}", cmd.value))
        });

        let result = handler
            .handle(
                &ctx,
                TestCommand {
                    value: "test".to_string(),
                },
            )
            .await
            .unwrap();

        assert_eq!(result, "processed: test");
    }
}
