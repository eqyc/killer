# Production Service (PP)

生产计划与执行服务，负责制造业生产管理。包括物料需求计划（MRP）、生产订单管理、车间作业执行和生产报工。

## 服务职责

| 模块 | 职责 |
|------|------|
| 需求计划 | MRP 运行、需求管理 |
| 生产订单 | 订单创建、下达、关闭 |
| 车间执行 | 作业调度、报工 |
| BOM 管理 | 物料清单维护 |
| 工艺路线 | 工序管理 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `ProductionOrder` | 生产订单 |
| `PlannedOrder` | 计划订单 |
| `BillOfMaterial` | 物料清单 |
| `Routing` | 工艺路线 |
| `WorkCenter` | 工作中心 |
| `ShopFloorOrder` | 车间订单 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `MrpCompleted` | MRP 运行完成 |
| `ProductionOrderCreated` | 生产订单创建 |
| `ProductionOrderReleased` | 订单下达 |
| `OperationConfirmed` | 工序确认 |
| `GoodsIssued` | 组件发料 |
| `GoodsReceipt` | 成品入库 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Material` | material | 物料 |
| `Plant` | organizational-units | 工厂 |
| `WorkCenter` | 本服务 | 工作中心 |
| `BOM` | 本服务 | 物料清单 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
