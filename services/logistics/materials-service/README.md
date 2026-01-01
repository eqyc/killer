# Materials Service (MM-IM)

物料管理服务，负责库存管理和物料移动。包括库存查询、物料收发、库存调整和周期盘点。

## 服务职责

| 模块 | 职责 |
|------|------|
| 库存管理 | 库存查询、可用量计算 |
| 物料移动 | 收货、发货、调拨 |
| 库存调整 | 差异处理、报废 |
| 周期盘点 | 盘点计划、盘点执行 |
| 批次管理 | 批次追溯 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Stock` | 库存 |
| `MaterialDocument` | 物料凭证 |
| `Reservation` | 预留 |
| `PhysicalInventory` | 实物盘点 |
| `Batch` | 批次 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `GoodsReceived` | 收货 |
| `GoodsIssued` | 发货 |
| `StockTransferred` | 调拨 |
| `InventoryCounted` | 盘点 |
| `StockAdjusted` | 库存调整 |
| `ReservationCreated` | 预留创建 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Material` | material | 物料 |
| `Plant` | organizational-units | 工厂 |
| `StorageLocation` | organizational-units | 库存地点 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
