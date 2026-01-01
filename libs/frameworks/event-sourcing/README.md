# Event Sourcing

事件溯源框架，提供聚合根、领域事件、事件存储的抽象与实现。支持事件重放、快照优化、以及事件投影。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `DomainEvent` | 领域事件 trait |
| `Aggregate` | 聚合根 trait |
| `AggregateRoot<E>` | 聚合根基类 |
| `EventStore` | 事件存储 trait |
| `EventStream` | 事件流 |
| `Snapshot<A>` | 聚合快照 |
| `EventEnvelope` | 事件信封（包含元数据） |
| `EventRepository<A>` | 事件溯源仓储 |
| `Projection` | 事件投影 trait |

## 使用示例

```text
// 定义领域事件
enum OrderEvent {
    Created { customer_id: CustomerId, ... },
    ItemAdded { item: OrderItem },
    Confirmed { confirmed_at: DateTime },
}
impl DomainEvent for OrderEvent { ... }

// 聚合根应用事件
impl Aggregate for Order {
    type Event = OrderEvent;

    fn apply(&mut self, event: Self::Event) {
        match event {
            OrderEvent::Created { .. } => { ... }
            OrderEvent::ItemAdded { item } => { ... }
        }
    }
}

// 从事件流重建聚合
let order = event_repository.load(order_id).await?;
```

## 快照策略

- `EveryNEvents(n)` - 每 N 个事件创建快照
- `TimeInterval(duration)` - 按时间间隔创建快照
- `OnDemand` - 手动触发快照
