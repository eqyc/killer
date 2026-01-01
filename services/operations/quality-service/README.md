# Quality Service (QM)

质量管理服务，负责产品质量控制和质量保证。包括质检计划、检验执行、不合格品处理和质量分析。

## 服务职责

| 模块 | 职责 |
|------|------|
| 质检计划 | 检验方案、抽样规则 |
| 检验执行 | 来料检、过程检、成品检 |
| 不合格处理 | NCR、让步接收、退货 |
| 质量分析 | SPC、质量报表 |
| 供应商质量 | 供应商质量评估 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `InspectionLot` | 检验批 |
| `InspectionPlan` | 检验计划 |
| `QualityNotification` | 质量通知 |
| `InspectionResult` | 检验结果 |
| `UsageDecision` | 使用决策 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `InspectionLotCreated` | 检验批创建 |
| `InspectionCompleted` | 检验完成 |
| `UsageDecisionMade` | 使用决策 |
| `QualityNotificationCreated` | 质量通知创建 |
| `DefectRecorded` | 缺陷记录 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Material` | material | 物料 |
| `Supplier` | business-partner | 供应商 |
| `Plant` | organizational-units | 工厂 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
