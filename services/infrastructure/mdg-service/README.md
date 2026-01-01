# Master Data Governance Service (MDG)

主数据治理中心，负责企业主数据的统一管理、分发和质量控制。作为主数据的单一真实来源，向其他服务提供一致的主数据视图。

## 服务职责

| 模块 | 职责 |
|------|------|
| 数据管理 | 主数据 CRUD、审批工作流 |
| 数据分发 | 主数据变更事件发布 |
| 版本管理 | 主数据版本控制 |
| 数据质量 | 数据校验、重复检测 |
| 数据同步 | 与外部系统同步 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `BusinessPartner` | 业务伙伴 |
| `Material` | 物料 |
| `CostCenter` | 成本中心 |
| `OrganizationalUnit` | 组织单元 |
| `ChangeRequest` | 变更请求 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `MasterDataCreated` | 主数据创建 |
| `MasterDataUpdated` | 主数据更新 |
| `MasterDataDeactivated` | 主数据停用 |
| `ChangeRequestApproved` | 变更审批通过 |

## 集成接口

| 接口类型 | 端点 | 说明 |
|----------|------|------|
| Kafka | `mdg.changes` | 主数据变更事件 |
| REST | `/api/v1/mdg/*` | 外部 API |
| OData | SAP 集成 | 主数据同步 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
