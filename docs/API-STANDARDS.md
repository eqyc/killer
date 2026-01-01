# KILLER ERP API 设计规范

本文档定义 KILLER ERP 系统的 API 设计标准，涵盖 gRPC 和 REST API 的设计原则、命名约定、错误处理等规范。

---

## 1. API 设计原则

### 1.1 RESTful 原则

| 原则 | 说明 | 示例 |
|------|------|------|
| 资源导向 | URL 表示资源，非操作 | `/orders` 而非 `/getOrders` |
| HTTP 动词语义化 | 动词表示操作类型 | GET 读取, POST 创建 |
| 无状态 | 请求包含完整信息 | 携带认证 Token |
| 统一接口 | 一致的响应格式 | 统一错误结构 |

### 1.2 gRPC 原则

| 原则 | 说明 |
|------|------|
| 强类型契约 | Protocol Buffers 定义明确的消息结构 |
| 服务聚合 | 按业务域组织服务，避免过细粒度 |
| 幂等设计 | 重试安全，使用幂等键 |
| 流式优先 | 大数据量传输使用 Server Streaming |

### 1.3 版本管理策略

| 策略 | 适用场景 | 示例 |
|------|----------|------|
| URL 版本（推荐） | REST API | `/api/v1/orders` |
| Package 版本 | gRPC | `package killer.sales.v1;` |
| Header 版本 | 细粒度控制 | `Api-Version: 2024-01-01` |

**版本演进规则**：
- 新增字段：保持当前版本，可选字段向后兼容
- 删除/重命名字段：发布新版本 (v1 → v2)
- 语义变更：发布新版本

---

## 2. gRPC 服务设计规范

### 2.1 服务命名约定

```protobuf
// 文件路径：proto/killer/finance/v1/financial_service.proto

syntax = "proto3";
package killer.finance.v1;

// 服务命名：{Domain}Service
service FinancialService {
    // RPC 方法命名：{动词}{资源}
    rpc GetJournalEntry(GetJournalEntryRequest) returns (GetJournalEntryResponse);
    rpc ListJournalEntries(ListJournalEntriesRequest) returns (ListJournalEntriesResponse);
    rpc CreateJournalEntry(CreateJournalEntryRequest) returns (CreateJournalEntryResponse);
    rpc PostJournalEntry(PostJournalEntryRequest) returns (PostJournalEntryResponse);
    rpc ReverseJournalEntry(ReverseJournalEntryRequest) returns (ReverseJournalEntryResponse);
}
```

**RPC 方法命名规范**：

| 操作类型 | 前缀 | 示例 |
|----------|------|------|
| 单个查询 | Get | `GetJournalEntry` |
| 列表查询 | List | `ListJournalEntries` |
| 创建 | Create | `CreatePurchaseOrder` |
| 更新 | Update | `UpdateCustomer` |
| 删除 | Delete | `DeleteDraftEntry` |
| 业务动作 | 动词 | `PostJournalEntry`, `ApproveOrder` |

### 2.2 请求/响应消息设计

```protobuf
// 请求消息：{RPC名}Request
message CreateJournalEntryRequest {
    string company_code = 1;
    string ledger_id = 2;
    google.type.Date document_date = 3;
    string description = 4;
    repeated LineItemInput line_items = 5;

    // 幂等键（可选）
    string idempotency_key = 10;
}

message LineItemInput {
    string account_code = 1;
    string dc_indicator = 2;  // "D" 或 "C"
    Money amount = 3;
    string cost_center = 4;
    string line_text = 5;
}

// 响应消息：{RPC名}Response
message CreateJournalEntryResponse {
    string entry_id = 1;
    string document_number = 2;
    string status = 3;
    google.protobuf.Timestamp created_at = 4;
}

// 共享值对象
message Money {
    string amount = 1;      // Decimal as string
    string currency = 2;    // ISO 4217
}
```

### 2.3 分页设计

```protobuf
// 分页请求
message PageRequest {
    int32 page_size = 1;        // 每页数量，默认 20，最大 100
    string page_token = 2;      // 分页令牌（游标分页）
}

// 分页响应
message PageResponse {
    string next_page_token = 1; // 下一页令牌，空则无更多
    int32 total_count = 2;      // 总记录数（可选）
}

// 使用示例
message ListJournalEntriesRequest {
    string company_code = 1;
    string fiscal_period = 2;
    string status_filter = 3;
    PageRequest page = 10;
}

message ListJournalEntriesResponse {
    repeated JournalEntrySummary entries = 1;
    PageResponse page = 10;
}
```

### 2.4 gRPC 错误处理

```protobuf
// 错误详情扩展
message ErrorDetail {
    string code = 1;            // 业务错误码
    string message = 2;         // 用户可读消息
    string field = 3;           // 关联字段（验证错误）
    map<string, string> metadata = 4;
}
```

**gRPC Status Code 映射表**：

| gRPC Code | HTTP 等价 | 使用场景 |
|-----------|----------|----------|
| OK (0) | 200 | 成功 |
| INVALID_ARGUMENT (3) | 400 | 参数验证失败 |
| NOT_FOUND (5) | 404 | 资源不存在 |
| ALREADY_EXISTS (6) | 409 | 资源已存在 |
| PERMISSION_DENIED (7) | 403 | 无权限 |
| UNAUTHENTICATED (16) | 401 | 未认证 |
| RESOURCE_EXHAUSTED (8) | 429 | 限流触发 |
| FAILED_PRECONDITION (9) | 400 | 前置条件不满足 |
| INTERNAL (13) | 500 | 服务内部错误 |
| UNAVAILABLE (14) | 503 | 服务不可用 |

### 2.5 流式 RPC 使用场景

| 流式类型 | 场景 | 示例 |
|----------|------|------|
| Server Streaming | 大量数据导出 | `ExportJournalEntries` |
| Client Streaming | 批量数据上传 | `BatchImportMaterials` |
| Bidirectional | 实时同步 | `SyncInventoryChanges` |

```protobuf
// Server Streaming 示例
rpc ExportJournalEntries(ExportRequest) returns (stream JournalEntryRow);

// Client Streaming 示例
rpc BatchImportMaterials(stream MaterialInput) returns (BatchImportResponse);
```

---

## 3. REST API 设计规范

### 3.1 URL 路径设计

```
URL 结构：/api/{version}/{domain}/{resource}[/{id}][/{sub-resource}]

示例：
GET    /api/v1/finance/journal-entries              # 列表
POST   /api/v1/finance/journal-entries              # 创建
GET    /api/v1/finance/journal-entries/{id}         # 详情
PUT    /api/v1/finance/journal-entries/{id}         # 全量更新
PATCH  /api/v1/finance/journal-entries/{id}         # 部分更新
DELETE /api/v1/finance/journal-entries/{id}         # 删除
POST   /api/v1/finance/journal-entries/{id}/post    # 业务动作
GET    /api/v1/finance/journal-entries/{id}/lines   # 子资源
```

**命名规范**：

| 规则 | 正例 | 反例 |
|------|------|------|
| 复数名词 | `/orders` | `/order` |
| 小写连字符 | `/journal-entries` | `/journalEntries` |
| 避免动词 | `/orders` | `/getOrders` |
| 动作用 POST | `POST /orders/{id}/cancel` | `DELETE /orders/{id}/cancel` |

### 3.2 HTTP 动词使用

| 动词 | 语义 | 幂等 | 请求体 | 响应码 |
|------|------|------|--------|--------|
| GET | 读取资源 | 是 | 无 | 200 |
| POST | 创建资源 | 否* | 有 | 201 |
| PUT | 全量替换 | 是 | 有 | 200 |
| PATCH | 部分更新 | 否 | 有 | 200 |
| DELETE | 删除资源 | 是 | 无 | 204 |

*POST 配合幂等键可实现幂等

### 3.3 查询参数约定

```
GET /api/v1/sales/orders?filter[status]=pending&filter[customer_id]=C001
    &sort=-created_at,order_number
    &page=1&limit=20
    &fields=id,order_number,total_amount
```

| 参数 | 格式 | 说明 |
|------|------|------|
| filter | `filter[field]=value` | 过滤条件 |
| sort | `sort=field,-field` | 排序，`-` 表示降序 |
| page | `page=1` | 页码（从 1 开始） |
| limit | `limit=20` | 每页数量 |
| fields | `fields=a,b,c` | 稀疏字段集 |

### 3.4 响应格式

**成功响应**：

```json
// 单个资源
{
    "data": {
        "id": "JE-2024-00001",
        "document_number": "1000000001",
        "company_code": "1000",
        "status": "posted"
    }
}

// 列表资源
{
    "data": [
        { "id": "JE-2024-00001", "status": "posted" },
        { "id": "JE-2024-00002", "status": "draft" }
    ],
    "pagination": {
        "page": 1,
        "limit": 20,
        "total": 156,
        "has_next": true
    }
}
```

**错误响应**：

```json
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "请求参数验证失败",
        "details": [
            {
                "field": "line_items",
                "code": "BALANCE_MISMATCH",
                "message": "借贷金额不平衡：借方 1000.00，贷方 900.00"
            }
        ],
        "request_id": "req-abc123",
        "timestamp": "2024-01-15T10:30:00Z"
    }
}
```

### 3.5 HTTP 状态码使用

| 状态码 | 语义 | 使用场景 |
|--------|------|----------|
| 200 OK | 成功 | GET/PUT/PATCH 成功 |
| 201 Created | 已创建 | POST 创建成功 |
| 204 No Content | 无内容 | DELETE 成功 |
| 400 Bad Request | 请求错误 | 参数验证失败 |
| 401 Unauthorized | 未认证 | Token 缺失/过期 |
| 403 Forbidden | 无权限 | 权限不足 |
| 404 Not Found | 不存在 | 资源不存在 |
| 409 Conflict | 冲突 | 并发冲突/业务规则冲突 |
| 422 Unprocessable | 无法处理 | 业务逻辑错误 |
| 429 Too Many | 限流 | 超出请求配额 |
| 500 Internal Error | 服务错误 | 未捕获异常 |
| 503 Unavailable | 不可用 | 服务维护/过载 |

---

## 4. DTO 设计

### 4.1 命名约定

| 类型 | 命名规则 | 示例 |
|------|----------|------|
| 创建请求 | Create{Resource}Request | `CreateOrderRequest` |
| 更新请求 | Update{Resource}Request | `UpdateOrderRequest` |
| 响应 | {Resource}Response | `OrderResponse` |
| 列表项 | {Resource}Summary | `OrderSummary` |
| 详情 | {Resource}Detail | `OrderDetail` |

### 4.2 输入验证

```
// 验证规则伪代码

CreateJournalEntryRequest {
    company_code: String
        @NotEmpty("公司代码不能为空")
        @Length(min=4, max=4, "公司代码必须4位")

    document_date: Date
        @NotNull("凭证日期不能为空")
        @PastOrPresent("凭证日期不能是未来")

    line_items: List<LineItemInput>
        @NotEmpty("行项目不能为空")
        @Size(min=2, "至少需要两行")
        @Valid  // 递归验证

    description: String
        @Length(max=500, "描述不能超过500字")
}
```

### 4.3 敏感数据脱敏

| 数据类型 | 脱敏规则 | 示例 |
|----------|----------|------|
| 银行账号 | 保留后4位 | `****1234` |
| 手机号 | 中间4位掩码 | `138****5678` |
| 身份证 | 保留首尾各4位 | `1101****1234` |
| 邮箱 | 用户名部分掩码 | `z***g@example.com` |

---

## 5. 认证与授权

### 5.1 JWT Token 格式

```
Header.Payload.Signature

Header: {
    "alg": "RS256",
    "typ": "JWT"
}

Payload: {
    "sub": "user-123",              // 用户ID
    "iss": "killer-iam",            // 签发者
    "aud": "killer-api",            // 受众
    "exp": 1705312200,              // 过期时间
    "iat": 1705308600,              // 签发时间
    "roles": ["finance_user"],      // 角色列表
    "company_code": "1000",         // 租户标识
    "permissions": ["journal:read"] // 权限列表（可选）
}
```

### 5.2 Token 传递

```http
# Bearer Token（推荐）
Authorization: Bearer eyJhbGciOiJSUzI1NiIs...

# API Key（外部系统）
X-API-Key: ak_live_xxxxxxxxxxxx
```

### 5.3 权限检查模型

```
RBAC 权限结构：
User → Role → Permission

Permission 命名：{resource}:{action}
示例：journal:read, journal:post, order:approve

权限检查伪代码：
@RequirePermission("journal:post")
function postJournalEntry(request) { ... }
```

---

## 6. 限流与熔断

### 6.1 限流策略

| 策略 | 配置 | 适用场景 |
|------|------|----------|
| Token Bucket | rate=100/s, burst=200 | 通用 API |
| Sliding Window | 1000 req/min | 防止突发 |
| 并发限制 | max_concurrent=50 | 重计算 API |

**限流响应头**：

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1705312260
Retry-After: 30
```

### 6.2 熔断器配置

```
CircuitBreaker 配置：
├── failure_threshold: 50%      # 失败率阈值
├── slow_call_threshold: 80%    # 慢调用阈值
├── slow_call_duration: 2s      # 慢调用判定时间
├── wait_duration: 30s          # 熔断恢复等待
└── permitted_calls: 10         # 半开状态允许调用数
```

---

## 7. 文档化

### 7.1 OpenAPI 规范

```yaml
# openapi.yaml 示例片段
openapi: 3.1.0
info:
  title: KILLER Financial API
  version: 1.0.0

paths:
  /api/v1/finance/journal-entries:
    post:
      summary: 创建日记账凭证
      operationId: createJournalEntry
      tags: [Journal Entries]
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateJournalEntryRequest'
      responses:
        '201':
          description: 凭证创建成功
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JournalEntryResponse'
```

### 7.2 gRPC Reflection

```bash
# 启用 gRPC Reflection 后可使用 grpcurl 探索
grpcurl -plaintext localhost:50051 list
grpcurl -plaintext localhost:50051 describe killer.finance.v1.FinancialService
```

---

## 8. 测试规范

### 8.1 集成测试要求

| 测试类型 | 覆盖要求 | 说明 |
|----------|----------|------|
| 正向流程 | 100% | 所有成功路径 |
| 参数验证 | 100% | 每个字段的验证规则 |
| 错误场景 | > 80% | 业务异常、系统异常 |
| 边界条件 | > 80% | 空值、最大值、特殊字符 |

### 8.2 Mock 数据规范

```json
// test/fixtures/journal_entry.json
{
    "valid_create_request": {
        "company_code": "1000",
        "document_date": "2024-01-15",
        "line_items": [
            {"account_code": "100100", "dc_indicator": "D", "amount": {"amount": "1000.00", "currency": "CNY"}},
            {"account_code": "200100", "dc_indicator": "C", "amount": {"amount": "1000.00", "currency": "CNY"}}
        ]
    },
    "invalid_unbalanced_request": {
        "company_code": "1000",
        "line_items": [
            {"account_code": "100100", "dc_indicator": "D", "amount": {"amount": "1000.00", "currency": "CNY"}},
            {"account_code": "200100", "dc_indicator": "C", "amount": {"amount": "900.00", "currency": "CNY"}}
        ]
    }
}
```

---

## 9. 变更管理

### 9.1 兼容性保证

| 变更类型 | 兼容性 | 处理方式 |
|----------|--------|----------|
| 新增可选字段 | 向后兼容 | 直接发布 |
| 新增必填字段 | 破坏性 | 新版本 |
| 删除字段 | 破坏性 | 先废弃，后删除 |
| 字段重命名 | 破坏性 | 新版本 |
| 修改字段类型 | 破坏性 | 新版本 |

### 9.2 废弃流程

```
阶段 1：标记废弃（保持运行 6 个月）
- 添加 @Deprecated 注解
- 响应头添加 Deprecation: true
- 文档标注废弃说明和替代方案

阶段 2：返回警告（保持运行 3 个月）
- 响应中添加 warning 字段
- 日志记录调用方信息

阶段 3：移除
- 返回 410 Gone
- 提供迁移文档链接
```

---

## 10. 示例：创建销售订单 API

### 10.1 gRPC 定义

```protobuf
// proto/killer/sales/v1/sales_service.proto

syntax = "proto3";
package killer.sales.v1;

import "google/protobuf/timestamp.proto";
import "google/type/date.proto";
import "killer/common/v1/money.proto";

service SalesService {
    rpc CreateSalesOrder(CreateSalesOrderRequest) returns (CreateSalesOrderResponse);
    rpc GetSalesOrder(GetSalesOrderRequest) returns (GetSalesOrderResponse);
}

message CreateSalesOrderRequest {
    string customer_id = 1;
    string company_code = 2;
    google.type.Date order_date = 3;
    string currency = 4;
    repeated OrderItemInput items = 5;
    ShippingAddress shipping_address = 6;
    string idempotency_key = 10;
}

message OrderItemInput {
    string material_id = 1;
    string quantity = 2;
    killer.common.v1.Money unit_price = 3;
}

message ShippingAddress {
    string street = 1;
    string city = 2;
    string postal_code = 3;
    string country = 4;
}

message CreateSalesOrderResponse {
    string order_id = 1;
    string order_number = 2;
    string status = 3;
    killer.common.v1.Money total_amount = 4;
    google.protobuf.Timestamp created_at = 5;
}
```

### 10.2 REST 定义

```
POST /api/v1/sales/orders
Content-Type: application/json
Authorization: Bearer <token>
Idempotency-Key: ord-req-12345
```

### 10.3 请求体示例

```json
{
    "customer_id": "CUST-001",
    "company_code": "1000",
    "order_date": "2024-01-15",
    "currency": "CNY",
    "items": [
        {
            "material_id": "MAT-A001",
            "quantity": "10",
            "unit_price": {
                "amount": "99.00",
                "currency": "CNY"
            }
        },
        {
            "material_id": "MAT-B002",
            "quantity": "5",
            "unit_price": {
                "amount": "199.00",
                "currency": "CNY"
            }
        }
    ],
    "shipping_address": {
        "street": "浦东新区张江高科技园区",
        "city": "上海",
        "postal_code": "201203",
        "country": "CN"
    }
}
```

### 10.4 成功响应示例

```json
{
    "data": {
        "order_id": "ord_2024011500001",
        "order_number": "SO-2024-00001",
        "status": "pending",
        "total_amount": {
            "amount": "1985.00",
            "currency": "CNY"
        },
        "created_at": "2024-01-15T10:30:00Z"
    }
}
```

### 10.5 错误响应示例

```json
{
    "error": {
        "code": "CUSTOMER_NOT_FOUND",
        "message": "客户不存在",
        "details": [
            {
                "field": "customer_id",
                "code": "NOT_FOUND",
                "message": "客户 CUST-001 不存在或已停用"
            }
        ],
        "request_id": "req-xyz789",
        "timestamp": "2024-01-15T10:30:00Z"
    }
}
```

---

## 附录：业务错误码清单

| 错误码 | HTTP | gRPC | 说明 |
|--------|------|------|------|
| VALIDATION_ERROR | 400 | INVALID_ARGUMENT | 参数验证失败 |
| BALANCE_MISMATCH | 422 | FAILED_PRECONDITION | 借贷不平衡 |
| PERIOD_CLOSED | 422 | FAILED_PRECONDITION | 会计期间已关闭 |
| INSUFFICIENT_STOCK | 422 | FAILED_PRECONDITION | 库存不足 |
| CREDIT_EXCEEDED | 422 | FAILED_PRECONDITION | 超出信用额度 |
| DUPLICATE_REQUEST | 409 | ALREADY_EXISTS | 重复请求 |
| RESOURCE_LOCKED | 409 | ABORTED | 资源被锁定 |
| NOT_FOUND | 404 | NOT_FOUND | 资源不存在 |
| UNAUTHORIZED | 401 | UNAUTHENTICATED | 未认证 |
| FORBIDDEN | 403 | PERMISSION_DENIED | 无权限 |
| RATE_LIMITED | 429 | RESOURCE_EXHAUSTED | 限流 |
| INTERNAL_ERROR | 500 | INTERNAL | 内部错误 |

---

*最后更新: 2024-01*
