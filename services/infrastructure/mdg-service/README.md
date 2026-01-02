# KILLER ERP - 主数据治理服务 (MDG Service)

主数据治理服务是 KILLER ERP 系统的核心基础设施服务，负责集中管理、治理和分发所有主数据。

## 🎯 核心职责

- **单一写入源**: 所有主数据 CRUD 操作的唯一入口
- **数据治理**: 验证、重复检测、数据质量评分
- **事件分发**: 通过 Kafka 向下游服务分发主数据变更
- **多租户隔离**: 强制租户级数据隔离和访问控制
- **审计合规**: 完整的变更历史和审计追踪

## 📦 支持的主数据实体

### 1. 物料主数据 (Material)
- Material (物料基本数据 - MARA)
- MaterialPlantData (物料工厂数据 - MARC)
- MaterialStorageData (物料库存地点数据 - MARD)

### 2. 业务伙伴 (Business Partner)
- BusinessPartner (业务伙伴 - BUT000)
- CustomerRole (客户角色 - KNA1)
- SupplierRole (供应商角色 - LFA1)

### 3. 组织单元 (Organizational Units)
- CompanyCode (公司代码 - T001)
- Plant (工厂 - T001W)
- StorageLocation (库存地点 - T001L)
- PurchasingOrganization (采购组织 - T024E)
- SalesOrganization (销售组织 - TVKO)

### 4. 成本对象 (Cost Objects)
- CostCenter (成本中心 - CSKS)
- ProfitCenter (利润中心 - CEPC)
- CostElement (成本要素 - CSKB)

## 🏗️ 架构设计

### DDD 四层架构

```
src/
├── domain/                  # 领域层
│   ├── aggregates/         # 聚合根 (扩展 libs/master-data/)
│   ├── events/             # 领域事件
│   └── repositories/       # 仓储接口
├── application/            # 应用层
│   ├── commands/           # 命令处理器 (CQRS)
│   ├── queries/            # 查询处理器
│   ├── workflows/          # 审批工作流
│   └── services/           # 应用服务
├── infrastructure/         # 基础设施层
│   ├── persistence/        # PostgreSQL 实现
│   ├── messaging/          # Kafka 生产者
│   ├── grpc/              # gRPC 服务实现
│   ├── cache/             # Redis 缓存
│   └── observability/     # 追踪和指标
└── config/                # 配置管理
```

### 技术栈

- **gRPC**: tonic (服务接口)
- **数据库**: sqlx + PostgreSQL (主存储)
- **消息队列**: rdkafka (事件发布)
- **缓存**: redis (查询加速)
- **追踪**: tracing + OpenTelemetry
- **验证**: validator + 自定义规则引擎

## 🚀 快速开始

### 前置条件

```bash
# PostgreSQL
docker run -d --name mdg-postgres \
  -e POSTGRES_DB=killer_mdg \
  -e POSTGRES_USER=mdg \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 postgres:15

# Kafka
docker run -d --name mdg-kafka \
  -p 9092:9092 \
  -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 \
  confluentinc/cp-kafka:latest

# Redis
docker run -d --name mdg-redis \
  -p 6379:6379 redis:7-alpine
```

### 数据库迁移

```bash
# 运行迁移
sqlx migrate run --database-url postgres://mdg:password@localhost:5432/killer_mdg

# 创建表结构
psql -U mdg -d killer_mdg -f migrations/001_create_tables.sql
```

### 启动服务

```bash
# 开发模式
cargo run --bin mdg-service

# 生产模式
cargo build --release
./target/release/mdg-service --config config/mdg.yaml
```

### 健康检查

```bash
# gRPC 健康检查
grpcurl -plaintext localhost:50051 grpc.health.v1.Health/Check

# HTTP 健康检查
curl http://localhost:8080/health
```

## 📡 gRPC API 示例

### 创建物料

```bash
grpcurl -plaintext \
  -d '{
    "metadata": {
      "tenant_id": "tenant-001",
      "actor_id": "user-123"
    },
    "material_number": "MAT-001",
    "description": "示例物料",
    "material_type": "FINISHED_PRODUCT",
    "base_unit": "EA"
  }' \
  localhost:50051 killer.mdg.v1.MaterialService/CreateMaterial
```

### 获取物料

```bash
grpcurl -plaintext \
  -d '{
    "metadata": {
      "tenant_id": "tenant-001",
      "actor_id": "user-123"
    },
    "id": "MAT-001"
  }' \
  localhost:50051 killer.mdg.v1.MaterialService/GetMaterial
```

### 创建业务伙伴

```bash
grpcurl -plaintext \
  -d '{
    "metadata": {
      "tenant_id": "tenant-001",
      "actor_id": "user-123"
    },
    "partner_id": "BP-001",
    "name": "示例客户",
    "partner_type": "organization",
    "address": {
      "city": "上海",
      "country": "CN"
    }
  }' \
  localhost:50051 killer.mdg.v1.BusinessPartnerService/CreateBusinessPartner
```

### 重复检测

```bash
grpcurl -plaintext \
  -d '{
    "metadata": {
      "tenant_id": "tenant-001",
      "actor_id": "user-123"
    },
    "name": "示例客户",
    "threshold": 0.85
  }' \
  localhost:50051 killer.mdg.v1.BusinessPartnerService/DetectDuplicates
```

## 📊 数据治理功能

### 1. 验证规则引擎

```yaml
# config/validation_rules.yaml
rules:
  material:
    - field: material_number
      type: regex
      pattern: "^MAT-[0-9]{6}$"
      message: "物料编号格式错误"
    
    - field: description
      type: length
      min: 1
      max: 200
      message: "描述长度必须在1-200之间"
    
    - field: plant_code
      type: reference
      entity: plant
      message: "工厂代码不存在"

  business_partner:
    - field: tax_number
      type: unique
      scope: tenant
      message: "税号已存在"
```

### 2. 数据质量评分

评分维度:
- **完整性** (40%): 必填字段填写率
- **一致性** (30%): 层级关系正确性
- **准确性** (30%): 格式和业务规则校验

```rust
// 示例响应
{
  "quality_score": {
    "overall_score": 85.5,
    "completeness": 90.0,
    "consistency": 85.0,
    "accuracy": 80.0,
    "issues": [
      "缺少物料组",
      "未设置安全库存"
    ]
  }
}
```

### 3. 重复检测

使用字符串相似度算法 (Levenshtein Distance):

```rust
// 检测逻辑
let similarity = strsim::jaro_winkler(name1, name2);
if similarity >= threshold {
    // 标记为潜在重复
}
```

## 🔄 事件驱动集成

### 事件发布

所有主数据变更自动发布到 Kafka:

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "created",
  "entity_type": "material",
  "tenant_id": "tenant-001",
  "actor_id": "user-123",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": 1,
  "payload": {
    "material": {
      "material_number": "MAT-001",
      "created": {
        "description": "示例物料",
        "material_type": "FINISHED_PRODUCT",
        "snapshot": { ... }
      }
    }
  }
}
```

### Kafka 主题

- `killer.mdg.events` - 所有事件
- `killer.mdg.material.events` - 物料事件
- `killer.mdg.business_partner.events` - 业务伙伴事件
- `killer.mdg.organization.events` - 组织单元事件
- `killer.mdg.cost_object.events` - 成本对象事件

### 下游服务订阅

```rust
// 在业务服务中订阅事件
use rdkafka::consumer::{Consumer, StreamConsumer};

let consumer: StreamConsumer = ClientConfig::new()
    .set("group.id", "finance-service")
    .set("bootstrap.servers", "localhost:9092")
    .create()?;

consumer.subscribe(&["killer.mdg.material.events"])?;

while let Some(message) = consumer.recv().await {
    let event: EventEnvelope = serde_json::from_slice(message.payload())?;
    
    // 更新本地缓存
    match event.event_type.as_str() {
        "created" | "updated" => {
            cache.upsert(event.entity_id, event.payload);
        }
        "deleted" => {
            cache.remove(event.entity_id);
        }
        _ => {}
    }
}
```

## 🗄️ 数据库设计

### 物料表

```sql
CREATE TABLE materials (
    material_number VARCHAR(18) PRIMARY KEY,
    tenant_id VARCHAR(50) NOT NULL,
    description VARCHAR(200) NOT NULL,
    material_type VARCHAR(50) NOT NULL,
    base_unit VARCHAR(3) NOT NULL,
    material_group VARCHAR(20),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    CONSTRAINT materials_tenant_id_idx UNIQUE (tenant_id, material_number)
);

CREATE INDEX idx_materials_tenant ON materials(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_materials_type ON materials(material_type) WHERE NOT deleted;
CREATE INDEX idx_materials_group ON materials(material_group) WHERE NOT deleted;
```

### 变更历史表

```sql
CREATE TABLE material_history (
    id BIGSERIAL PRIMARY KEY,
    material_number VARCHAR(18) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    version INTEGER NOT NULL,
    operation VARCHAR(20) NOT NULL,  -- created, updated, deleted
    changes JSONB NOT NULL,
    snapshot JSONB NOT NULL,
    actor_id VARCHAR(50) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (material_number, tenant_id) 
        REFERENCES materials(material_number, tenant_id)
);

CREATE INDEX idx_material_history_lookup 
    ON material_history(material_number, tenant_id, timestamp DESC);
```

## 🔒 安全与权限

### JWT 认证

```rust
// 从 gRPC metadata 提取 JWT
let token = request.metadata()
    .get("authorization")
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.strip_prefix("Bearer "))
    .ok_or(MdgError::PermissionDenied("Missing token".into()))?;

// 验证 JWT
let claims = jsonwebtoken::decode::<Claims>(
    token,
    &decoding_key,
    &validation,
)?;

// 检查租户ID
if claims.tenant_id != request.metadata.tenant_id {
    return Err(MdgError::TenantMismatch("Tenant mismatch".into()));
}
```

### RBAC 权限

```rust
// 权限检查
fn check_permission(claims: &Claims, permission: &str) -> MdgResult<()> {
    if !claims.permissions.contains(&permission.to_string()) {
        return Err(MdgError::PermissionDenied(
            format!("Missing permission: {}", permission)
        ));
    }
    Ok(())
}

// 使用示例
check_permission(&claims, "mdg:write")?;
```

## 📈 可观测性

### Prometheus 指标

```
# 请求总数
mdg_requests_total{service="material",method="create",status="success"} 1234

# 请求延迟
mdg_request_duration_seconds{service="material",method="create",quantile="0.99"} 0.125

# 数据质量评分
mdg_data_quality_score{entity_type="material",tenant_id="tenant-001"} 85.5

# 事件发布
mdg_events_published_total{entity_type="material",event_type="created"} 567

# 缓存命中率
mdg_cache_hit_rate{entity_type="material"} 0.92
```

### 分布式追踪

```rust
use tracing::{info, instrument};

#[instrument(skip(self))]
async fn create_material(&self, request: CreateMaterialRequest) -> MdgResult<Material> {
    info!("Creating material: {}", request.material_number);
    
    // 业务逻辑...
    
    Ok(material)
}
```

## 🧪 测试

### 单元测试

```bash
cargo test --lib
```

### 集成测试

```bash
# 启动测试依赖
docker-compose -f docker-compose.test.yaml up -d

# 运行集成测试
cargo test --test integration_tests

# 清理
docker-compose -f docker-compose.test.yaml down
```

### 测试示例

```rust
#[tokio::test]
async fn test_create_material() {
    let service = MaterialService::new(/* ... */);
    
    let request = CreateMaterialRequest {
        metadata: Some(RequestMetadata {
            tenant_id: "tenant-001".into(),
            actor_id: "user-123".into(),
            correlation_id: "test-001".into(),
        }),
        material_number: "MAT-001".into(),
        description: "Test Material".into(),
        material_type: "FINISHED_PRODUCT".into(),
        base_unit: "EA".into(),
        ..Default::default()
    };
    
    let response = service.create_material(request).await.unwrap();
    
    assert_eq!(response.material_number, "MAT-001");
    assert_eq!(response.tenant_id, "tenant-001");
}
```

## 🚢 部署

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin mdg-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/mdg-service /usr/local/bin/
COPY config /etc/mdg/config
EXPOSE 50051 8080
CMD ["mdg-service", "--config", "/etc/mdg/config/mdg.yaml"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mdg-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: mdg-service
  template:
    metadata:
      labels:
        app: mdg-service
    spec:
      containers:
      - name: mdg-service
        image: killer-erp/mdg-service:latest
        ports:
        - containerPort: 50051
          name: grpc
        - containerPort: 8080
          name: http
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: mdg-secrets
              key: database-url
        - name: KAFKA_BROKERS
          value: "kafka:9092"
        - name: REDIS_URL
          value: "redis://redis:6379"
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: mdg-service
spec:
  selector:
    app: mdg-service
  ports:
  - name: grpc
    port: 50051
    targetPort: 50051
  - name: http
    port: 8080
    targetPort: 8080
```

## 📚 相关文档

- [主数据契约文档](../../../libs/master-data/README.md)
- [CQRS 框架文档](../../../libs/frameworks/cqrs/README.md)
- [API 网关集成](../api-gateway/README.md)

## 🤝 贡献指南

1. 所有 API 必须支持多租户隔离
2. 所有变更必须发布事件
3. 所有操作必须记录审计日志
4. 添加完整的单元测试和集成测试
5. 更新 API 文档和示例

## 📄 许可证

MIT OR Apache-2.0
