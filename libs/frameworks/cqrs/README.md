# CQRS

命令查询职责分离（CQRS）框架，提供 Command 和 Query 的抽象定义与处理器模式。支持命令验证、中间件管道、以及与事件溯源的集成。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Command` | 命令 trait，表示写操作意图 |
| `Query` | 查询 trait，表示读操作意图 |
| `CommandHandler<C>` | 命令处理器 trait |
| `QueryHandler<Q>` | 查询处理器 trait |
| `CommandBus` | 命令总线，路由命令到处理器 |
| `QueryBus` | 查询总线，路由查询到处理器 |
| `Middleware` | 中间件 trait（日志、验证、事务等） |
| `CommandResult<T>` | 命令执行结果 |

## 使用示例

```text
// 定义命令
struct CreateOrderCommand {
    customer_id: CustomerId,
    items: Vec<OrderItem>,
}
impl Command for CreateOrderCommand { ... }

// 实现处理器
impl CommandHandler<CreateOrderCommand> for OrderCommandHandler {
    async fn handle(&self, cmd: CreateOrderCommand) -> CommandResult<OrderId> {
        // 处理逻辑
    }
}

// 通过总线分发
let order_id = command_bus.dispatch(create_order_cmd).await?;
```

## 中间件支持

- `LoggingMiddleware` - 命令/查询日志记录
- `ValidationMiddleware` - 参数验证
- `TransactionMiddleware` - 事务管理
- `MetricsMiddleware` - 性能指标采集
