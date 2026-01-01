# Warehouse Service (EWM)

仓储管理服务，负责仓库内部作业管理。包括库位管理、入库上架、出库拣配、波次管理和 RF 终端作业。

## 服务职责

| 模块 | 职责 |
|------|------|
| 库位管理 | 库位定义、容量管理 |
| 入库作业 | 收货、上架、质检 |
| 出库作业 | 波次、拣配、打包 |
| 任务管理 | 任务分配、优先级 |
| RF 作业 | 手持终端接口 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Bin` | 库位 |
| `WarehouseTask` | 仓库任务 |
| `InboundDelivery` | 入库交货 |
| `OutboundDelivery` | 出库交货 |
| `Wave` | 波次 |
| `HandlingUnit` | 处理单元 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `TaskCreated` | 任务创建 |
| `TaskCompleted` | 任务完成 |
| `PutawayCompleted` | 上架完成 |
| `PickingCompleted` | 拣配完成 |
| `WaveReleased` | 波次释放 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Material` | material | 物料 |
| `Warehouse` | 本服务 | 仓库 |
| `StorageType` | 本服务 | 存储类型 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
