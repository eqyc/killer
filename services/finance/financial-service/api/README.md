# KILLER ERP Financial Service API Layer

## 概述

API 层作为财务服务的外部统一入口，提供 gRPC 和 REST 两种访问方式，负责协议转换、认证授权、幂等性控制、审计追踪和错误标准化。

## 架构

```
+------------------+     +------------------+     +-------------------+
| External Client  | --> | API Layer        | --> | Application Layer |
| (gRPC/REST)      |     | - Auth/Authz     |     | - CQRS Commands   |
| - Mobile App     |     | - Idempotency    |     | - CQRS Queries    |
| - Web Frontend   |     | - Audit          |     |                   |
| - API Gateway    |     | - Error Mapping  |     |                   |
+------------------+     +------------------+     +-------------------+
```

## gRPC 服务定义

### JournalEntryService

| RPC | 描述 | 权限要求 |
|-----|------|----------|
| `CreateJournalEntry` | 创建会计凭证 | 无 |
| `PostJournalEntry` | 过账会计凭证 | `finance:post` |
| `ReverseJournalEntry` | 冲销会计凭证 | `finance:reverse` |
| `GetJournalEntry` | 获取凭证详情 | 无 |
| `ListJournalEntries` | 列出凭证（流式） | 无 |
| `GetAccountBalance` | 获取科目余额 | 无 |
| `GetTrialBalance` | 获取试算平衡表 | 无 |

### 消息类型

#### 请求消息

```protobuf
message PostJournalEntryRequest {
  string id = 1;                          // 凭证ID
  google.protobuf.Timestamp posting_date = 2;  // 过账日期
  string idempotency_key = 3;             // 幂等键（可选）
}

message ReverseJournalEntryRequest {
  string id = 1;                          // 原凭证ID
  google.protobuf.Timestamp reversal_date = 2;  // 冲销日期
  string reversal_reason = 3;             // 冲销原因
  string reference_document = 4;          // 参考（可选）
  string idempotency_key = 5;             // 幂等键（可选）
}
```

#### 响应消息

```protobuf
message PostJournalEntryResponse {
  string id = 1;
  string document_number = 2;
  string status = 3;                      // "POSTED"
  google.protobuf.Timestamp posting_date = 4;
  double total_debit = 5;
  double total_credit = 6;
  google.protobuf.Timestamp posted_at = 7;
}
```

## REST API

### 路径映射

| gRPC | REST | 方法 | 描述 |
|------|------|------|------|
| CreateJournalEntry | `/api/v1/journal-entries` | POST | 创建凭证 |
| PostJournalEntry | `/api/v1/journal-entries/{id}/post` | POST | 过账凭证 |
| ReverseJournalEntry | `/api/v1/journal-entries/{id}/reverse` | POST | 冲销凭证 |
| GetJournalEntry | `/api/v1/journal-entries/{id}` | GET | 获取凭证 |
| ListJournalEntries | `/api/v1/journal-entries` | GET | 列表查询 |
| GetAccountBalance | `/api/v1/account-balances` | GET | 科目余额 |
| GetTrialBalance | `/api/v1/trial-balance` | GET | 试算平衡 |

### 查询参数

```http
GET /api/v1/journal-entries?page_size=50&page_token=xxx&company_code=1000&fiscal_year=2024&status=POSTED&sort_by=posting_date&sort_order=desc
```

### 请求示例

```bash
# 创建凭证
curl -X POST http://localhost:8080/api/v1/journal-entries \
  -H "Content-Type: application/json" \
  -H "X-Tenant-Id: 550e8400-e29b-41d4-a716-446655440000" \
  -H "X-User-Id: 550e8400-e29b-41d4-a716-446655440001" \
  -d '{
    "company_code": "1000",
    "fiscal_year": 2024,
    "posting_date": "2024-01-15",
    "document_date": "2024-01-15",
    "currency_code": "CNY",
    "header_text": "Test Entry",
    "line_items": [
      {"account_code": "10010001", "amount": 1000, "debit_credit": "D", "cost_center": "CC001"},
      {"account_code": "20010001", "amount": 1000, "debit_credit": "C"}
    ]
  }'

# 过账凭证
curl -X POST http://localhost:8080/api/v1/journal-entries/{id}/post \
  -H "X-Tenant-Id: 550e8400-e29b-41d4-a716-446655440000" \
  -H "X-User-Id: 550e8400-e29b-41d4-a716-446655440001" \
  -H "Authorization: Bearer <token>"
```

## 认证与授权

### 认证方式

1. **JWT Bearer Token**（推荐）
```http
Authorization: Bearer <JWT token>
```

2. **API Gateway 注入**（内部调用）
```http
X-Tenant-Id: 550e8400-e29b-41d4-a716-446655440000
X-User-Id: 550e8400-e29b-41d4-a716-446655440001
X-Roles: finance:post,finance:read,accountant
```

### JWT Claims

```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440001",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "roles": ["finance:post", "finance:read", "accountant"],
  "iss": "killer-erp",
  "aud": "financial-service",
  "exp": 1704067200
}
```

### 角色权限

| 角色 | 权限 |
|------|------|
| `finance:post` | 过账凭证 |
| `finance:read` | 读取凭证、余额、试算表 |
| `finance:reverse` | 冲销凭证 |
| `finance:admin` | 所有财务操作 |
| `accountant` | 会计操作（过账、冲销） |

## 幂等性控制

### 使用幂等键

对于写操作（POST、Reverse），建议提供幂等键防止重复提交：

```http
Idempotency-Key: <唯一标识符>
```

幂等键格式建议：`{tenant_id}-{user_id}-{operation}-{resource_id}`

示例：
```
Idempotency-Key: 550e8400-550e8401-post-12345678-abc12345
```

幂等结果缓存 24 小时。

## 审计追踪

### 审计事件

每个 API 请求都会生成审计记录，包含：

```json
{
  "id": "uuid",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "550e8400-e29b-41d4-a716-446655440001",
  "action": "POST",
  "resource_type": "journal_entry",
  "resource_id": "12345678-abc1-2345-6789-abc123456789",
  "status": "SUCCESS",
  "trace_id": "abc123",
  "timestamp": "2024-01-15T10:30:00Z",
  "duration_ms": 150
}
```

### 审计存储

审计事件可存储到：
- PostgreSQL `journal_audit_log` 表
- Kafka 主题 `killer.audit.events`

## 错误码

| gRPC Code | 错误码 | 描述 |
|-----------|--------|------|
| `INVALID_ARGUMENT` | `VALIDATION_FAILED` | 参数验证失败 |
| `NOT_FOUND` | `NOT_FOUND` | 资源不存在 |
| `ALREADY_EXISTS` | `CONFLICT` | 资源已存在/已操作 |
| `FAILED_PRECONDITION` | `BUSINESS_RULE_VIOLATION` | 业务规则违反 |
| `ABORTED` | `CONFLICT` | 并发冲突（乐观锁） |
| `PERMISSION_DENIED` | `PERMISSION_DENIED` | 权限不足 |
| `RESOURCE_EXHAUSTED` | `RATE_LIMITED` | 请求限流 |
| `INTERNAL` | `INTERNAL` | 内部错误 |
| `UNAVAILABLE` | `EXTERNAL_SERVICE_ERROR` | 外部服务错误 |

### 错误响应格式

```json
{
  "code": "VALIDATION_FAILED",
  "message": "Validation failed: line_items must have at least 2 items",
  "details": [
    {
      "field": "line_items",
      "type": "error",
      "message": "must have at least 2 items",
      "value": 1
    }
  ],
  "trace_id": "abc12345",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## 健康检查

| 端点 | 描述 |
|------|------|
| `/health/live` | Liveness probe |
| `/health/ready` | Readiness probe |
| `/metrics` | Prometheus metrics |

## 可观测性

### Prometheus Metrics

| 指标 | 类型 | 标签 | 描述 |
|------|------|------|------|
| `grpc_requests_total` | Counter | service, method, status, tenant_id | gRPC 请求总数 |
| `grpc_request_duration_seconds` | Histogram | service, method | gRPC 请求延迟 |
| `http_requests_total` | Counter | method, path, status, tenant_id | HTTP 请求总数 |
| `http_request_duration_seconds` | Histogram | method, path | HTTP 请求延迟 |
| `active_requests` | Gauge | - | 活跃请求数 |
| `errors_total` | Counter | service, method, error_type | 错误总数 |
| `idempotency_hits_total` | Counter | - | 幂等性命中数 |
| `auth_failures_total` | Counter | - | 认证失败数 |

### Tracing

集成 OpenTelemetry，支持：
- 请求链路追踪
- 自定义 span
- 传播追踪上下文

## 与 API Gateway 集成

### Header 透传

API Gateway 应透传以下 header：

```http
# 认证相关
Authorization: Bearer <token>
X-Tenant-Id: <UUID>
X-User-Id: <UUID>
X-Roles: role1,role2

# 追踪相关
X-Trace-Id: <UUID>
X-Span-Id: <UUID>

# 幂等性
Idempotency-Key: <key>

# 其他
X-Request-Id: <UUID>
X-Forwarded-For: <IP>
```

### 配置示例

```yaml
api:
  grpc:
    listen_addr: "0.0.0.0:50051"
    max_concurrent_requests: 1000
    request_timeout: 30s

  http:
    listen_addr: "0.0.0.0:8080"
    enabled: true
    cors:
      allowed_origins:
        - "https://*.killer.com"
      allowed_methods:
        - "GET"
        - "POST"
        - "PUT"
        - "DELETE"

  auth:
    jwt_secret: "${JWT_SECRET}"
    allowed_issuers:
      - "killer-erp"
    allowed_audiences:
      - "financial-service"
    skip_auth: false

  idempotency:
    enabled: true
    ttl_hours: 24
    key_prefix: "killer:finance:api"

  audit:
    enabled: true
    storage_type: "kafka"
    kafka_topic: "killer.audit.events"

  metrics:
    enabled: true
    path: "/metrics"
    port: 9090
```

## 启动服务

```bash
# 开发模式（跳过认证）
cargo run --bin financial-service-api -- --config config.dev.yaml

# 生产模式
cargo run --release --bin financial-service-api -- --config config.yaml
```

## OpenAPI 文档

访问 Swagger UI：`http://localhost:8080/swagger-ui/`

## 测试

```bash
# 运行集成测试
cargo test --test integration

# 运行 gRPC 测试
cargo test --test grpc_tests

# 运行幂等性测试
cargo test --test idempotency_tests
```
