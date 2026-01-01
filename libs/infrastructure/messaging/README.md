# Messaging

消息基础设施库，提供 Kafka 生产者/消费者封装以及事件总线抽象。支持消息序列化、分区策略、消费者组管理和死信队列。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `EventBus` | 事件总线 trait |
| `EventPublisher` | 事件发布者 trait |
| `EventSubscriber` | 事件订阅者 trait |
| `KafkaProducer` | Kafka 生产者封装 |
| `KafkaConsumer` | Kafka 消费者封装 |
| `Message<T>` | 消息封装（含元数据） |
| `MessageHandler<M>` | 消息处理器 trait |
| `DeadLetterQueue` | 死信队列处理 |
| `ConsumerGroup` | 消费者组配置 |

## 使用示例

```text
// 发布事件
let publisher = KafkaProducer::new(config).await?;
publisher.publish("order-events", OrderCreatedEvent { ... }).await?;

// 订阅事件
let consumer = KafkaConsumer::builder()
    .topic("order-events")
    .group_id("inventory-service")
    .handler(|event: OrderCreatedEvent| async {
        // 处理事件
    })
    .build()
    .await?;

consumer.start().await?;

// 使用事件总线
event_bus.publish(order_created_event).await?;
event_bus.subscribe::<OrderCreatedEvent, _>(handler).await?;
```

## 配置选项

| 配置 | 说明 | 默认值 |
|------|------|--------|
| `bootstrap_servers` | Kafka 服务器地址 | - |
| `acks` | 确认级别 | `all` |
| `retries` | 重试次数 | `3` |
| `batch_size` | 批量大小 | `16KB` |
| `linger_ms` | 等待时间 | `5ms` |
