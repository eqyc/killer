# Controlling Service (CO)

管理会计服务，负责企业内部成本核算和管理报表。包括成本中心会计、内部订单、产品成本核算和获利分析。

## 服务职责

| 模块 | 职责 |
|------|------|
| 成本中心会计 | 成本归集、分摊、分配 |
| 内部订单 | 项目成本跟踪 |
| 产品成本 | 标准成本、实际成本计算 |
| 获利分析 | 多维度盈利分析 |
| 管理报表 | 成本报表、差异分析 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `CostCenterPosting` | 成本中心过账 |
| `InternalOrder` | 内部订单 |
| `ProductCost` | 产品成本 |
| `ProfitabilitySegment` | 获利段 |
| `AllocationCycle` | 分摊周期 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `CostPosted` | 成本过账 |
| `CostAllocated` | 成本分摊 |
| `OrderSettled` | 订单结算 |
| `VarianceCalculated` | 差异计算完成 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `CostCenter` | cost-center | 成本中心 |
| `ProfitCenter` | cost-center | 利润中心 |
| `CostElement` | 本服务 | 成本要素 |
| `ActivityType` | 本服务 | 作业类型 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
