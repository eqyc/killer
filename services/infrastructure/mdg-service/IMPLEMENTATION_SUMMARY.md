# MDG 服务实现总结

## ✅ 已完成的工作

### 1. 项目结构 ✓
```
services/infrastructure/mdg-service/
├── src/
│   ├── main.rs                      # 服务入口，支持优雅关闭
│   ├── error.rs                     # 统一错误类型和 gRPC 转换
│   ├── config.rs                    # 配置管理（YAML + 环境变量）
│   ├── health.rs                    # 健康检查
│   ├── domain/                      # 领域层（模块占位）
│   │   ├── aggregates/
│   │   ├── events/
│   │   └── repositories/
│   ├── application/                 # 应用层（模块占位）
│   │   ├── commands/
│   │   ├── queries/
│   │   ├── workflows/
│   │   └── services/
│   └── infrastructure/              # 基础设施层（模块占位）
│       ├── persistence/
│       ├── messaging/
│       ├── grpc/
│       ├── cache/
│       ├── external/
│       └── observability/
├── proto/
│   ├── mdg.proto                    # gRPC 服务定义
│   └── events.proto                 # 事件定义
├── config/
│   └── mdg.yaml                     # 服务配置
├── migrations/
│   └── 001_create_tables.sql       # 数据库表结构
├── Cargo.toml                       # 依赖配置
├── build.rs                         # Proto 编译
├── Dockerfile                       # 容器镜像
├── docker-compose.yaml              # 本地开发环境
├── docker-compose.test.yaml         # 测试环境
├── .env.example                     # 环境变量示例
└── README.md                        # 完整文档
```

### 2. Proto 定义 ✓

#### 服务接口 (mdg.proto)
- **MaterialService**: 物料 CRUD + 工厂数据 + 批量导出
- **BusinessPartnerService**: 业务伙伴 CRUD + 角色管理 + 重复检测
- **OrganizationService**: 公司代码/工厂 CRUD + 层级验证
- **CostObjectService**: 成本中心/利润中心 CRUD

#### 通用消息
- RequestMetadata: 租户ID、操作者ID、关联ID
- PageRequest/PageInfo: 分页支持
- Filter: 灵活的过滤条件
- DataQualityScore: 数据质量评分

#### 事件定义 (events.proto)
- EventEnvelope: 统一事件信封
- FieldChange: 字段级变更追踪
- 实体事件: Material/BusinessPartner/CompanyCode/Plant/CostCenter

### 3. 配置管理 ✓

**config/mdg.yaml** 包含:
- 服务器配置 (gRPC + HTTP 端口)
- 数据库配置 (PostgreSQL 连接池)
- Kafka 配置 (生产者 + 主题)
- Redis 配置 (缓存 TTL)
- 治理配置 (验证/重复检测/质量评分)
- 可观测性配置 (追踪/指标/日志)
- 安全配置 (JWT + RBAC)

### 4. 错误处理 ✓

**error.rs** 定义:
- MdgError: 统一错误枚举
  - EntityNotFound
  - EntityAlreadyExists
  - VersionConflict (乐观锁)
  - ValidationError
  - TenantMismatch
  - HierarchyValidationFailed
  - DatabaseError/KafkaError/RedisError
- 自动转换为 tonic::Status

### 5. 数据库设计 ✓

**migrations/001_create_tables.sql** 包含:

#### 主数据表
- materials (物料基本数据)
- material_plant_data (物料工厂数据)
- business_partners (业务伙伴)
- customer_roles (客户角色)
- supplier_roles (供应商角色)
- company_codes (公司代码)
- plants (工厂)
- cost_centers (成本中心)

#### 支持表
- change_history (变更历史审计)
- data_quality_scores (数据质量评分)

#### 特性
- 多租户隔离 (tenant_id + 复合主键)
- 乐观锁 (version 字段)
- 软删除 (deleted 标记)
- 扩展字段 (JSONB extensions)
- 全文搜索索引 (GIN)
- 时间有效性索引

### 6. 部署配置 ✓

#### Dockerfile
- 多阶段构建 (builder + runtime)
- 非 root 用户运行
- 健康检查
- 最小化镜像大小

#### docker-compose.yaml
- PostgreSQL (持久化存储)
- Kafka + Zookeeper (事件流)
- Redis (缓存)
- MDG Service (主服务)
- 健康检查和依赖管理

#### docker-compose.test.yaml
- 测试环境 (tmpfs 存储)
- 独立端口避免冲突

### 7. 文档 ✓

**README.md** 包含:
- 核心职责说明
- 支持的主数据实体
- DDD 四层架构
- 技术栈
- 快速开始指南
- gRPC API 示例
- 数据治理功能
- 事件驱动集成
- 数据库设计
- 安全与权限
- 可观测性
- 测试指南
- Kubernetes 部署

## 🎯 核心特性

### 1. 单一写入源
- 所有主数据 CRUD 操作集中管理
- 强制租户隔离
- 乐观锁防止并发冲突

### 2. 数据治理
- **验证规则引擎**: 可配置的 YAML 规则
- **重复检测**: 基于字符串相似度 (strsim)
- **数据质量评分**: 完整性 + 一致性 + 准确性

### 3. 事件驱动
- 所有变更发布到 Kafka
- 支持 Delta 追踪
- 完整快照
- 下游服务异步订阅

### 4. 多租户
- tenant_id 强制过滤
- 复合主键隔离
- JWT 租户验证

### 5. 审计合规
- 变更历史表
- 字段级 Delta
- 操作者追踪
- 关联ID追踪

## 🔧 技术栈

- **gRPC**: tonic (服务接口)
- **数据库**: sqlx + PostgreSQL
- **消息队列**: rdkafka (Kafka)
- **缓存**: redis
- **追踪**: tracing + OpenTelemetry
- **验证**: validator
- **配置**: config (YAML + env)

## 📊 支持的主数据

### 物料主数据
- Material (MARA)
- MaterialPlantData (MARC)
- MaterialStorageData (MARD)

### 业务伙伴
- BusinessPartner (BUT000)
- CustomerRole (KNA1)
- SupplierRole (LFA1)

### 组织单元
- CompanyCode (T001)
- Plant (T001W)
- StorageLocation (T001L)
- PurchasingOrganization (T024E)
- SalesOrganization (TVKO)

### 成本对象
- CostCenter (CSKS)
- ProfitCenter (CEPC)
- CostElement (CSKB)

## 🚀 快速启动

```bash
# 启动依赖服务
docker-compose up -d postgres kafka redis

# 运行数据库迁移
psql -U mdg -d killer_mdg -f migrations/001_create_tables.sql

# 启动 MDG 服务
cargo run --bin mdg-service

# 或使用 Docker
docker-compose up mdg-service
```

## 🧪 测试

```bash
# 单元测试
cargo test --lib

# 集成测试
docker-compose -f docker-compose.test.yaml up -d
cargo test --test integration_tests
docker-compose -f docker-compose.test.yaml down
```

## 📡 API 示例

### 创建物料
```bash
grpcurl -plaintext \
  -d '{
    "metadata": {"tenant_id": "tenant-001", "actor_id": "user-123"},
    "material_number": "MAT-001",
    "description": "示例物料",
    "material_type": "FINISHED_PRODUCT",
    "base_unit": "EA"
  }' \
  localhost:50051 killer.mdg.v1.MaterialService/CreateMaterial
```

### 重复检测
```bash
grpcurl -plaintext \
  -d '{
    "metadata": {"tenant_id": "tenant-001", "actor_id": "user-123"},
    "name": "示例客户",
    "threshold": 0.85
  }' \
  localhost:50051 killer.mdg.v1.BusinessPartnerService/DetectDuplicates
```

## 🔄 事件集成

### 发布事件
所有主数据变更自动发布到 Kafka:
- killer.mdg.events (所有事件)
- killer.mdg.material.events (物料事件)
- killer.mdg.business_partner.events (业务伙伴事件)
- killer.mdg.organization.events (组织单元事件)
- killer.mdg.cost_object.events (成本对象事件)

### 订阅事件
下游服务订阅事件更新本地缓存:
```rust
consumer.subscribe(&["killer.mdg.material.events"])?;
while let Some(message) = consumer.recv().await {
    let event: EventEnvelope = serde_json::from_slice(message.payload())?;
    cache.upsert(event.entity_id, event.payload);
}
```

## 📈 可观测性

### Prometheus 指标
- mdg_requests_total
- mdg_request_duration_seconds
- mdg_data_quality_score
- mdg_events_published_total
- mdg_cache_hit_rate

### 分布式追踪
- OpenTelemetry 集成
- Span 追踪
- 关联ID传播

## 🔒 安全

### JWT 认证
- 从 gRPC metadata 提取 token
- 验证签名和过期时间
- 租户ID匹配检查

### RBAC 权限
- mdg:read
- mdg:write
- mdg:delete
- mdg:admin

## 📦 部署

### Docker
```bash
docker build -t killer-erp/mdg-service:latest .
docker run -p 50051:50051 -p 8080:8080 killer-erp/mdg-service:latest
```

### Kubernetes
参考 README.md 中的 Kubernetes 部署配置

## 🎓 下一步

### 需要完整实现的模块

1. **domain/aggregates/** - 聚合根实现
   - MaterialAggregate
   - BusinessPartnerAggregate
   - OrganizationAggregate
   - CostObjectAggregate

2. **domain/repositories/** - 仓储接口
   - MaterialRepository trait
   - BusinessPartnerRepository trait
   - 等

3. **application/commands/** - 命令处理器
   - CreateMaterialCommand
   - UpdateMaterialCommand
   - DeleteMaterialCommand
   - 等

4. **application/queries/** - 查询处理器
   - GetMaterialQuery
   - ListMaterialsQuery
   - 等

5. **application/services/** - 应用服务
   - ValidationService (规则引擎)
   - DuplicateDetectionService (相似度检测)
   - DataQualityService (质量评分)

6. **infrastructure/persistence/** - 持久化实现
   - PostgreSQL repositories
   - sqlx 查询
   - 事务管理

7. **infrastructure/messaging/** - 消息队列
   - Kafka producer
   - 事件序列化
   - 重试机制

8. **infrastructure/grpc/** - gRPC 服务实现
   - MaterialServiceImpl
   - BusinessPartnerServiceImpl
   - OrganizationServiceImpl
   - CostObjectServiceImpl

9. **infrastructure/cache/** - 缓存实现
   - Redis 客户端
   - 缓存策略
   - 失效机制

10. **tests/** - 测试
    - 单元测试
    - 集成测试
    - E2E 测试

## 📝 总结

MDG 服务的完整框架已经搭建完成，包括:
- ✅ 完整的 Proto 定义 (4个服务，20+ RPC 方法)
- ✅ 数据库表结构 (8个主数据表 + 2个支持表)
- ✅ 配置管理 (YAML + 环境变量)
- ✅ 错误处理 (统一错误类型)
- ✅ Docker 部署 (Dockerfile + docker-compose)
- ✅ 完整文档 (README + API 示例)
- ✅ DDD 四层架构 (模块占位)

所有核心设计已经完成，可以基于这个框架继续实现具体的业务逻辑。
