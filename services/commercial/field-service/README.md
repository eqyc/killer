# Field Service

售后服务管理，负责产品售后的安装、维修和保养服务。包括服务工单、工程师调度、备件管理和 SLA 监控。

## 服务职责

| 模块 | 职责 |
|------|------|
| 服务工单 | 工单创建、派工、关闭 |
| 工程师调度 | 资源调度、路线优化 |
| 备件管理 | 备件申请、消耗 |
| SLA 管理 | 响应时间、解决时间 |
| 知识库 | 故障诊断、解决方案 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `ServiceOrder` | 服务工单 |
| `Technician` | 工程师 |
| `InstalledBase` | 装机基础 |
| `ServiceContract` | 服务合同 |
| `SparePart` | 备件 |
| `SLA` | 服务级别协议 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `ServiceOrderCreated` | 工单创建 |
| `TechnicianAssigned` | 工程师分配 |
| `ServiceStarted` | 服务开始 |
| `ServiceCompleted` | 服务完成 |
| `SLABreached` | SLA 违约 |
| `SparePartUsed` | 备件使用 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Customer` | business-partner | 客户 |
| `Material` | material | 产品/备件 |
| `InstalledBase` | 本服务 | 装机信息 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
