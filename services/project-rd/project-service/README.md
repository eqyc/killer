# Project Service (PS)

专业服务项目管理，负责项目型业务的全生命周期管理。包括项目计划、资源管理、成本控制、工时记录和项目计费。

## 服务职责

| 模块 | 职责 |
|------|------|
| 项目管理 | 项目创建、WBS、里程碑 |
| 资源管理 | 资源计划、分配 |
| 成本管理 | 预算、实际成本 |
| 工时管理 | 工时记录、审批 |
| 项目计费 | 计费计划、开票 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Project` | 项目 |
| `WBS` | 工作分解结构 |
| `Resource` | 资源 |
| `TimeSheet` | 工时表 |
| `ProjectBudget` | 项目预算 |
| `BillingMilestone` | 计费里程碑 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `ProjectCreated` | 项目创建 |
| `ProjectReleased` | 项目释放 |
| `MilestoneCompleted` | 里程碑完成 |
| `TimeSheetApproved` | 工时审批 |
| `BillingRequested` | 计费请求 |
| `ProjectClosed` | 项目关闭 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Customer` | business-partner | 客户 |
| `CostCenter` | cost-center | 成本中心 |
| `ProfitCenter` | cost-center | 利润中心 |
| `CompanyCode` | organizational-units | 公司代码 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
