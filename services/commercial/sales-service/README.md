# Sales Service (SD)

销售订单服务，负责企业销售业务的全流程管理。包括报价管理、销售订单、发货处理、开票和信用管理。与物流服务和财务服务紧密集成，实现订单到收款（O2C）流程。

## 服务职责

| 模块 | 职责 |
|------|------|
| 报价 (QT) | 报价创建、审批、转订单 |
| 订单 (SO) | 销售订单管理、变更、取消 |
| 发货 (DL) | 发货单创建、拣配、发运 |
| 开票 (BL) | 开票请求、发票生成 |
| 信用 (CR) | 信用检查、信用额度管理 |
| 定价 (PR) | 价格确定、折扣、促销 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `SalesOrder` | 销售订单 |
| `Quotation` | 销售报价 |
| `Delivery` | 发货单 |
| `BillingDocument` | 开票凭证 |
| `CreditLimit` | 客户信用额度 |
| `PricingCondition` | 定价条件 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `QuotationCreated` | 报价创建 |
| `QuotationApproved` | 报价审批通过 |
| `SalesOrderCreated` | 订单创建 |
| `SalesOrderConfirmed` | 订单确认 |
| `SalesOrderCancelled` | 订单取消 |
| `DeliveryCreated` | 发货单创建 |
| `GoodsIssued` | 发货出库 |
| `InvoiceCreated` | 发票创建 |
| `CreditBlocked` | 信用冻结 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Customer` | business-partner | 客户信息 |
| `Material` | material | 产品信息 |
| `SalesOrganization` | organizational-units | 销售组织 |
| `DistributionChannel` | organizational-units | 分销渠道 |
| `Division` | organizational-units | 产品组 |
| `Plant` | organizational-units | 发货工厂 |

## 集成接口

| 接口类型 | 端点 | 说明 |
|----------|------|------|
| gRPC | `SalesService` | 服务间调用 |
| REST | `/api/v1/sales/*` | 外部 API |
| Kafka | `sales.orders` | 订单事件 |
| Kafka | `sales.deliveries` | 发货事件 |

## Saga 流程

### Order-to-Cash (O2C) Saga

```text
1. CreateSalesOrder
   └── 补偿: CancelSalesOrder
2. CheckCredit
   └── 补偿: ReleaseCredit
3. ReserveInventory (→ materials-service)
   └── 补偿: ReleaseInventory
4. CreateDelivery
   └── 补偿: CancelDelivery
5. CreateInvoice (→ financial-service)
   └── 补偿: CancelInvoice
```

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |

## 启动命令

```bash
# 开发环境
cargo run -p killer-sales-service

# 生产环境
./sales-service --config config/production.toml
```
