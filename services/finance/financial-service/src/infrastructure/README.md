# Financial Service Infrastructure Layer

基础设施层文档，包含持久化、消息、适配器和投影的详细说明。

## 架构概览

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      Infrastructure Layer                               │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Persistence │  │  Messaging  │  │   Adapters  │  │  Projection │    │
│  ├─────────────┤  ├─────────────┤  ├─────────────┤  ├─────────────┤    │
│  │ - PostgreSQL│  │ - Kafka     │  │ - MDG gRPC  │  │ - ClickHouse│    │
│  │ - Repositories│ │ - Outbox   │  │ - Materials │  │ - Worker    │    │
│  │ - UoW       │  │ - Producer  │  │ - Cache     │  │ - Metrics   │    │
│  │ - Audit     │  │ - Consumer  │  │ - Circuit   │  │ - Lag Mon   │    │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
```

## 模块结构

### persistence - 持久化模块

提供 PostgreSQL 持久化支持，实现领域仓储接口。

#### Repository 实现

| Repository | 文件 | 职责 |
|------------|------|------|
| `JournalEntryRepository` | `persistence/journal_entry_repository.rs` | 凭证聚合根持久化 |
| `FiscalPeriodRepository` | `persistence/fiscal_period_repository.rs` | 会计期间持久化 |
| `OutboxRepository` | `persistence/outbox_repository.rs` | 发件箱持久化 |

#### 数据库表

| 表名 | 职责 |
|------|------|
| `journal_entries` | 凭证抬头（含 JSONB 扩展属性） |
| `journal_entry_lines` | 凭证行项目 |
| `fiscal_periods` | 会计期间（含有效性范围） |
| `outbox_messages` | 事务发件箱 |
| `journal_audit_log` | 审计日志（软删除） |
| `processed_events` | 已处理事件（投影幂等） |

#### 配置示例 (YAML)

```yaml
infrastructure:
  postgres:
    url: "postgres://user:pass@localhost:5432/financial"
    max_connections: 20
    connection_timeout: 30
    idle_timeout: 600
    max_lifetime: 3600
    ssl_mode: require
```

### messaging - 消息模块

提供 Kafka 生产者和消费者支持。

#### 组件

| 组件 | 文件 | 职责 |
|------|------|------|
| `KafkaEventProducer` | `messaging/kafka_producer.rs` | 事件发布到 Kafka |
| `OutboxPublisherWorker` | `messaging/outbox_publisher.rs` | 后台发件箱处理器 |
| `KafkaEventConsumer` | `messaging/kafka_consumer.rs` | 事件消费 |
| `EventSerializer` | `messaging/event_serializer.rs` | 事件序列化 |

#### Kafka 主题

| 主题 | 分区键 | 职责 |
|------|--------|------|
| `killer.finance.events` | tenant_id | 财务领域事件 |
| `killer.logistics.events` | tenant_id | 物流领域事件 |

#### 配置示例 (YAML)

```yaml
infrastructure:
  kafka:
    brokers:
      - "kafka-0:9092"
      - "kafka-1:9092"
      - "kafka-2:9092"
    client_id: "killer-financial-service"
    consumer_group_id: "finance-service-group"
    sasl:
      mechanism: SCRAM-SHA-256
      username: "${KAFKA_USER}"
      password: "${KAFKA_PASS}"
    producer:
      acks: all
      retries: 5
      batch_size: 16384
      linger_ms: 5
    consumer:
      enable_auto_commit: false
      session_timeout_ms: 30000
      max_poll_records: 100
```

### adapters - 适配器模块

提供外部服务集成。

#### 适配器

| 适配器 | 文件 | 依赖 |
|--------|------|------|
| `MasterDataClientImpl` | `adapters/master_data_client.rs` | Redis 缓存、熔断器 |
| `MaterialsEventSubscriber` | `adapters/materials_event_subscriber.rs` | Kafka Consumer |
| `RedisCache` | `adapters/redis_cache.rs` | Redis 连接池 |
| `CircuitBreaker` | `adapters/circuit_breaker.rs` | 熔断器模式 |

#### 配置示例 (YAML)

```yaml
infrastructure:
  redis:
    url: "redis://localhost:6379"
    max_connections: 50
    command_timeout: 5000

  mdg_service:
    grpc_address: "mdg-service:50051"
    connection_timeout: 5000
    request_timeout: 10000
    cache_ttl: 3600
    circuit_breaker:
      failure_threshold: 5
      success_threshold: 2
      timeout: 60000
```

### projection - 投影模块

提供 ClickHouse 读模型投影。

#### 组件

| 组件 | 文件 | 职责 |
|------|------|------|
| `ClickHouseClient` | `projection/mod.rs` | ClickHouse 客户端 |
| `ProjectionWorker` | `projection/mod.rs` | 投影工作器 |

#### ClickHouse 表

| 表名 | 分区 | 排序键 |
|------|------|--------|
| `journal_lines` | (posting_date, tenant_id) | (tenant_id, company_code, fiscal_year, account_code) |
| `trial_balance` | (fiscal_year, tenant_id) | (tenant_id, company_code, fiscal_year, account_code) |

#### 配置示例 (YAML)

```yaml
infrastructure:
  clickhouse:
    url: "http://clickhouse:8123"
    database: "financial"
    username: "default"
    password: ""
    max_connections: 10
    command_timeout: 30000

  projection:
    batch_size: 100
    batch_timeout: 10
    max_retries: 3
```

## 核心设计模式

### 1. 事务性发件箱 (Transactional Outbox)

```
Command → Save Aggregate → Save Outbox → Commit Transaction
                                     ↓
                              OutboxPublisher
                                     ↓
                              Publish to Kafka
```

```rust
// 事务内原子写入聚合和发件箱
sqlx::transaction(|tx| async move {
    repo.save(&aggregate, &tx).await?;
    outbox_repo.save(&events, &tx).await?;
    tx.commit().await?;
})
```

### 2. 乐观并发控制

```sql
-- 更新时检查版本
UPDATE journal_entries
SET version = version + 1,
    updated_at = NOW()
WHERE id = $1 AND version = $expected_version;
```

### 3. 熔断器保护

```rust
let breaker = Arc::new(CircuitBreaker::new("mdg-service", None));
let result = breaker.call(|| client.validate(...)).await;
```

### 4. 投影幂等性

```sql
-- 检查事件是否已处理
INSERT INTO processed_events (event_id, event_type, aggregate_id)
SELECT $1, $2, $3
WHERE NOT EXISTS (
    SELECT 1 FROM processed_events WHERE event_id = $1
);
```

## 指标监控

### Prometheus 指标

| 指标 | 类型 | 标签 | 描述 |
|------|------|------|------|
| `db_queries_total` | Counter | operation, table, status | 数据库查询总数 |
| `db_query_duration_seconds` | Histogram | operation, table | 查询耗时 |
| `kafka_events_published_total` | Counter | topic, status | 发布到 Kafka 的事件数 |
| `kafka_events_consumed_total` | Counter | topic, status | 从 Kafka 消费的事件数 |
| `outbox_events_processed_total` | Counter | status | 发件箱处理的事件数 |
| `projection_events_projected_total` | Counter | event_type | 投影的事件数 |
| `projection_lag_seconds` | Gauge | consumer_group | 投影延迟 |
| `adapter_external_calls_total` | Counter | service, method, status | 外部服务调用数 |
| `cache_hits_total` | Counter | service | 缓存命中数 |
| `circuit_breaker_state` | Gauge | service | 熔断器状态 |

### 部署建议

#### 独立 Pod 部署

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: financial-service
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: financial-service
        image: killer/financial-service:latest
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: financial-db-secret
              key: url
        - name: KAFKA_BROKERS
          valueFrom:
            configMapKeyRef:
              name: financial-config
              key: kafka_brokers
```

#### HPA 配置

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: financial-service-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: financial-service
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## 性能调优

### PostgreSQL

```yaml
infrastructure:
  postgres:
    max_connections: 50  # 根据 QPS 调整
    # 建议: CPU核心数 * 2 + 备用连接数
```

### Kafka Producer

```yaml
infrastructure:
  kafka:
    producer:
      batch_size: 65536  # 增加批量大小
      linger_ms: 10      # 增加 linger 时间
      compression: lz4   # 使用高效压缩
```

### Outbox Publisher

```yaml
infrastructure:
  outbox:
    poll_interval_ms: 100  # 减少轮询间隔
    batch_size: 100        # 增加批量大小
    max_retries: 5
    initial_backoff_ms: 1000
```

### ClickHouse Projection

```yaml
infrastructure:
  projection:
    batch_size: 500        # 增加批量大小
    batch_timeout: 30      # 增加超时时间
```

## 问题诊断

### 常见错误

| 错误码 | 错误信息 | 可能原因 | 解决方案 |
|--------|----------|----------|----------|
| `DB_CONN_FAILED` | connection refused | 数据库连接池耗尽 | 增加 max_connections |
| `DB_DEADLOCK` | deadlock detected | 并发更新同一行 | 检查业务逻辑 |
| `KAFKA_PUB_TIMEOUT` | publish timeout | Kafka 集群压力 | 增加超时或检查集群 |
| `CIRCUIT_OPEN` | service unavailable | 下游服务故障 | 检查下游服务 |
| `PROJECTION_LAG_HIGH` | lag > threshold | 投影速度跟不上 | 增加 worker |

### 日志示例

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "ERROR",
  "service": "financial-service",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "trace_id": "abc123",
  "component": "persistence",
  "operation": "save",
  "error": "DB_CONN_FAILED",
  "message": "Failed to save journal entry",
  "duration_ms": 5000
}
```

### 监控 Dashboard

推荐 Grafana Dashboard 包含：

1. **请求延迟** - P50/P95/P99
2. **错误率** - 按错误类型分类
3. **吞吐量** - QPS
4. **数据库连接池** - 活跃/空闲连接数
5. **Kafka lag** - 消费者延迟
6. **投影延迟** - 事件到投影的时间差

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::PgJournalEntryRepository;

    // Mock PgPool
    struct MockPool;

    #[tokio::test]
    async fn test_repository_find_by_id() {
        let repo = PgJournalEntryRepository::new(Arc::new(MockPool));
        let result = repo.find_by_id(&id).await;
        assert!(result.is_ok());
    }
}
```

### 集成测试

```rust
#[cfg(test)]
mod integration_tests {
    use testcontainers::clients;
    use testcontainers_modules::postgres;
    use testcontainers_modules::kafka::Kafka;

    #[tokio::test]
    async fn test_transactional_outbox() {
        // 启动 testcontainers
        let docker = clients::Cli::default();
        let postgres_container = docker.run(postgres::default());
        let kafka_container = docker.run(Kafka::default());

        // 执行测试
        // ...
    }
}
```

## 与 CQRS 框架集成

```rust
use cqrs::prelude::*;

let uow = UnitOfWork::new(outbox_repo);

// Command Handler 中
uow.add_event(envelope)?;
uow.commit().await?;

// 应用层注入
let repo = Arc::new(PgJournalEntryRepository::new(pool));
let event_bus = Arc::new(KafkaEventProducer::new(&config).await?);
let cache = Arc::new(RedisCache::new(conn, metrics));
let master_data = Arc::new(MasterDataClientImpl::new(config, cache, metrics));
```

## 安全

### 数据库连接

```yaml
infrastructure:
  postgres:
    url: "postgres://user:pass@host:5432/db?sslmode=require"
    ssl_mode: require
```

### Kafka SASL/SSL

```yaml
infrastructure:
  kafka:
    sasl:
      mechanism: SCRAM-SHA-512
      username: "${KAFKA_USER}"
      password: "${KAFKA_PASS}"
    ssl:
      ca_location: /certs/ca.crt
      certificate_location: /certs/service.crt
      key_location: /certs/service.key
```

### 敏感数据脱敏

```rust
// 日志中脱敏金额
tracing::info!(amount = "[REDACTED]", "Journal entry posted");
```
