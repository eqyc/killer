//! Handler 注册表
//!
//! 提供 Handler 的注册和查找功能

use crate::{
    command::{Command, CommandHandler},
    event::{DomainEvent, EventHandler},
    query::{Query, QueryHandler},
};
use std::sync::Arc;

/// Command Handler 注册表
#[derive(Default)]
pub struct CommandHandlerRegistry {
    handlers: std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>>,
}

impl CommandHandlerRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    /// 注册 Command Handler
    pub fn register<C: Command + 'static>(&mut self, handler: Arc<dyn CommandHandler<C>>) {
        self.handlers.insert(C::command_name(), Box::new(handler));
    }

    /// 获取 Command Handler
    pub fn get<C: Command + 'static>(&self) -> Option<Arc<dyn CommandHandler<C>>> {
        self.handlers
            .get(C::command_name())
            .and_then(|h| h.downcast_ref::<Arc<dyn CommandHandler<C>>>())
            .cloned()
    }

    /// 获取所有已注册的 Command 名称
    pub fn command_names(&self) -> Vec<&'static str> {
        self.handlers.keys().copied().collect()
    }

    /// 检查是否已注册
    pub fn is_registered<C: Command + 'static>(&self) -> bool {
        self.handlers.contains_key(C::command_name())
    }
}

/// Event Handler 注册表
#[derive(Default)]
pub struct EventHandlerRegistry {
    handlers: std::collections::HashMap<&'static str, Vec<Box<dyn std::any::Any + Send + Sync>>>,
}

impl EventHandlerRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    /// 注册 Event Handler
    pub fn register<E: DomainEvent + 'static>(
        &mut self,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.handlers
            .entry(E::event_name())
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }

    /// 获取 Event Handlers
    pub fn get<E: DomainEvent + 'static>(&self) -> Option<Vec<Arc<dyn EventHandler<E>>>> {
        self.handlers
            .get(E::event_name())
            .map(|handlers| {
                handlers
                    .iter()
                    .filter_map(|h| h.downcast_ref::<Arc<dyn EventHandler<E>>>())
                    .cloned()
                    .collect()
            })
    }

    /// 获取所有已注册的事件名称
    pub fn event_names(&self) -> Vec<&'static str> {
        self.handlers.keys().copied().collect()
    }

    /// 检查是否已注册
    pub fn is_registered<E: DomainEvent + 'static>(&self) -> bool {
        self.handlers.contains_key(E::event_name())
    }
}

/// Query Handler 注册表
#[derive(Default)]
pub struct QueryHandlerRegistry {
    handlers: std::collections::HashMap<&'static str, Box<dyn std::any::Any + Send + Sync>>,
}

impl QueryHandlerRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    /// 注册 Query Handler
    pub fn register<Q: Query + 'static>(&mut self, handler: Arc<dyn QueryHandler<Q>>) {
        self.handlers.insert(Q::query_name(), Box::new(handler));
    }

    /// 获取 Query Handler
    pub fn get<Q: Query + 'static>(&self) -> Option<Arc<dyn QueryHandler<Q>>> {
        self.handlers
            .get(Q::query_name())
            .and_then(|h| h.downcast_ref::<Arc<dyn QueryHandler<Q>>>())
            .cloned()
    }

    /// 获取所有已注册的查询名称
    pub fn query_names(&self) -> Vec<&'static str> {
        self.handlers.keys().copied().collect()
    }

    /// 检查是否已注册
    pub fn is_registered<Q: Query + 'static>(&self) -> bool {
        self.handlers.contains_key(Q::query_name())
    }
}

/// CQRS 应用程序
///
/// 整合所有注册表
#[derive(Default)]
pub struct CqrsApplication {
    command_registry: CommandHandlerRegistry,
    event_registry: EventHandlerRegistry,
    query_registry: QueryHandlerRegistry,
}

impl CqrsApplication {
    /// 创建新的 CQRS 应用程序
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册 Command Handler
    pub fn register_command_handler<C: Command + 'static>(
        &mut self,
        handler: Arc<dyn CommandHandler<C>>,
    ) {
        self.command_registry.register(handler);
    }

    /// 注册 Event Handler
    pub fn register_event_handler<E: DomainEvent + 'static>(
        &mut self,
        handler: Arc<dyn EventHandler<E>>,
    ) {
        self.event_registry.register(handler);
    }

    /// 注册 Query Handler
    pub fn register_query_handler<Q: Query + 'static>(
        &mut self,
        handler: Arc<dyn QueryHandler<Q>>,
    ) {
        self.query_registry.register(handler);
    }

    /// 获取 Command Handler
    pub fn get_command_handler<C: Command + 'static>(
        &self,
    ) -> Option<Arc<dyn CommandHandler<C>>> {
        self.command_registry.get()
    }

    /// 获取 Event Handlers
    pub fn get_event_handlers<E: DomainEvent + 'static>(
        &self,
    ) -> Option<Vec<Arc<dyn EventHandler<E>>>> {
        self.event_registry.get()
    }

    /// 获取 Query Handler
    pub fn get_query_handler<Q: Query + 'static>(
        &self,
    ) -> Option<Arc<dyn QueryHandler<Q>>> {
        self.query_registry.get()
    }

    /// 打印所有已注册的处理器
    pub fn print_registered(&self) {
        tracing::info!("Registered Command Handlers: {:?}", self.command_registry.command_names());
        tracing::info!("Registered Event Handlers: {:?}", self.event_registry.event_names());
        tracing::info!("Registered Query Handlers: {:?}", self.query_registry.query_names());
    }
}

/// CQRS 应用程序构建器
#[derive(Default)]
pub struct CqrsApplicationBuilder;

impl CqrsApplicationBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 构建应用程序
    pub fn build(self) -> CqrsApplication {
        CqrsApplication::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{command::Command, context::CommandContext, error::AppError, query::Query};
    use async_trait::async_trait;
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

    struct TestCommandHandler;

    #[async_trait::async_trait]
    impl CommandHandler<TestCommand> for TestCommandHandler {
        async fn handle(&self, _ctx: &CommandContext, cmd: TestCommand) -> Result<String, AppError> {
            Ok(format!("Processed: {}", cmd.value))
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

    struct TestQueryHandler;

    #[async_trait::async_trait]
    impl QueryHandler<TestQuery> for TestQueryHandler {
        async fn handle(&self, _ctx: &CommandContext, query: TestQuery) -> Result<String, AppError> {
            Ok(format!("Query: {}", query.id))
        }
    }

    #[test]
    fn test_command_handler_registry() {
        let mut registry = CommandHandlerRegistry::new();
        registry.register::<TestCommand>(Arc::new(TestCommandHandler));

        let handler = registry.get::<TestCommand>();
        assert!(handler.is_some());
    }

    #[test]
    fn test_query_handler_registry() {
        let mut registry = QueryHandlerRegistry::new();
        registry.register::<TestQuery>(Arc::new(TestQueryHandler));

        let handler = registry.get::<TestQuery>();
        assert!(handler.is_some());
    }

    #[test]
    fn test_cqrs_application() {
        let mut app = CqrsApplicationBuilder::new().build();

        app.register_command_handler::<TestCommand>(Arc::new(TestCommandHandler));
        app.register_query_handler::<TestQuery>(Arc::new(TestQueryHandler));

        let cmd_handler = app.get_command_handler::<TestCommand>();
        let query_handler = app.get_query_handler::<TestQuery>();

        assert!(cmd_handler.is_some());
        assert!(query_handler.is_some());
    }
}
