# PLM Service

产品生命周期管理服务，负责产品从概念到退市的全生命周期管理。包括产品设计、工程BOM、变更管理和版本控制。

## 服务职责

| 模块 | 职责 |
|------|------|
| 产品设计 | 产品结构、CAD 集成 |
| BOM 管理 | 工程 BOM、制造 BOM |
| 变更管理 | ECR、ECN 流程 |
| 版本管理 | 产品版本、修订 |
| 文档管理 | 技术文档、图纸 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Product` | 产品 |
| `EngineeringBOM` | 工程 BOM |
| `ChangeRequest` | 变更请求 |
| `ChangeNotice` | 变更通知 |
| `ProductVersion` | 产品版本 |
| `Document` | 技术文档 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `ProductCreated` | 产品创建 |
| `BOMReleased` | BOM 发布 |
| `ChangeRequestCreated` | ECR 创建 |
| `ChangeNoticeApproved` | ECN 审批 |
| `VersionReleased` | 版本发布 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Material` | material | 物料/零件 |
| `Plant` | organizational-units | 工厂 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
