# Maintenance Service (PM)

设备维护服务，负责企业设备资产的维护管理。包括预防性维护计划、故障报修、维护执行和备件管理。

## 服务职责

| 模块 | 职责 |
|------|------|
| 设备管理 | 设备台账、技术参数 |
| 预防维护 | 维护计划、定期保养 |
| 故障维修 | 报修、派工、修复 |
| 备件管理 | 备件库存、消耗分析 |
| 维护分析 | MTBF、MTTR、OEE |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Equipment` | 设备 |
| `MaintenanceOrder` | 维护工单 |
| `MaintenancePlan` | 维护计划 |
| `Notification` | 维护通知 |
| `SparePart` | 备件 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `MaintenanceOrderCreated` | 工单创建 |
| `MaintenanceCompleted` | 维护完成 |
| `EquipmentBreakdown` | 设备故障 |
| `SparePartConsumed` | 备件消耗 |
| `PreventiveMaintenanceDue` | 预防维护到期 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Equipment` | 本服务 | 设备信息 |
| `Material` | material | 备件物料 |
| `Plant` | organizational-units | 工厂 |
| `CostCenter` | cost-center | 成本中心 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
