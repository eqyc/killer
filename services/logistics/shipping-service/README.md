# Shipping Service (TM)

运输管理服务，负责货物运输的计划和执行。包括运输计划、承运商管理、车辆调度、运费计算和物流跟踪。

## 服务职责

| 模块 | 职责 |
|------|------|
| 运输计划 | 路线规划、装载优化 |
| 承运商管理 | 承运商选择、合同 |
| 车辆调度 | 派车、排程 |
| 运费管理 | 运费计算、结算 |
| 物流跟踪 | 在途跟踪、签收 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Shipment` | 运输单 |
| `FreightOrder` | 运单 |
| `Carrier` | 承运商 |
| `Vehicle` | 车辆 |
| `Route` | 路线 |
| `FreightSettlement` | 运费结算 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `ShipmentCreated` | 运输单创建 |
| `ShipmentDispatched` | 发运 |
| `InTransit` | 在途更新 |
| `DeliveryCompleted` | 签收 |
| `FreightSettled` | 运费结算 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Carrier` | business-partner | 承运商 |
| `Customer` | business-partner | 收货方 |
| `Plant` | organizational-units | 发货点 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
