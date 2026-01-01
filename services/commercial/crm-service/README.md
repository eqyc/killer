# CRM Service

客户关系管理服务，负责客户全生命周期管理。包括客户 360 视图、商机管理、营销活动和客户分析。

## 服务职责

| 模块 | 职责 |
|------|------|
| 客户管理 | 客户 360、联系人 |
| 商机管理 | 商机跟踪、销售漏斗 |
| 营销活动 | 活动策划、效果分析 |
| 客户服务 | 服务请求、知识库 |
| 客户分析 | RFM、客户画像 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `CustomerProfile` | 客户档案 |
| `Opportunity` | 商机 |
| `Campaign` | 营销活动 |
| `Lead` | 销售线索 |
| `Activity` | 活动记录 |
| `ServiceTicket` | 服务工单 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `LeadCreated` | 线索创建 |
| `LeadConverted` | 线索转化 |
| `OpportunityCreated` | 商机创建 |
| `OpportunityWon` | 商机赢单 |
| `OpportunityLost` | 商机输单 |
| `CampaignLaunched` | 活动上线 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Customer` | business-partner | 客户信息 |
| `SalesOrganization` | organizational-units | 销售组织 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
