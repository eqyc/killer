# API Contracts

API 契约定义库，包含 gRPC 服务接口定义、REST API DTO 和 OpenAPI 规范。作为服务间通信的契约层，确保 API 一致性。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `*ServiceClient` | gRPC 服务客户端（自动生成） |
| `*ServiceServer` | gRPC 服务端 trait（自动生成） |
| `*Request` | 请求 DTO |
| `*Response` | 响应 DTO |
| `PageRequest` | 分页请求 |
| `PageResponse<T>` | 分页响应 |
| `ApiError` | API 错误响应 |
| `ApiResult<T>` | API 结果封装 |

## 使用示例

```text
// gRPC 客户端调用
let client = OrderServiceClient::connect("http://order-service:50051").await?;
let response = client.create_order(CreateOrderRequest { ... }).await?;

// gRPC 服务端实现
impl OrderService for OrderServiceImpl {
    async fn create_order(&self, request: Request<CreateOrderRequest>)
        -> Result<Response<CreateOrderResponse>, Status> {
        // 实现逻辑
    }
}

// REST DTO
#[derive(Serialize, Deserialize)]
struct CreateOrderDto {
    customer_id: String,
    items: Vec<OrderItemDto>,
}
```

## Proto 文件组织

```text
proto/
├── common/
│   ├── pagination.proto
│   └── error.proto
└── services/
    ├── order/
    │   └── order_service.proto
    └── inventory/
        └── inventory_service.proto
```
