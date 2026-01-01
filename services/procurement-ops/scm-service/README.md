# SCM Service

战略采购与供应商管理服务，负责供应商关系管理、合同管理和战略寻源。支持供应商评估、认证和绩效管理。

## 服务职责

| 模块 | 职责 |
|------|------|
| 供应商管理 | 供应商注册、认证、分级 |
| 合同管理 | 框架协议、采购合同 |
| 寻源 | RFQ、招投标 |
| 绩效评估 | 供应商评分、KPI |
| 风险管理 | 供应商风险评估 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `SupplierProfile` | 供应商档案 |
| `Contract` | 采购合同 |
| `RFQ` | 询价单 |
| `SupplierEvaluation` | 供应商评估 |
| `SourceList` | 供应来源清单 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `SupplierRegistered` | 供应商注册 |
| `SupplierApproved` | 供应商审批 |
| `ContractCreated` | 合同创建 |
| `ContractExpiring` | 合同即将到期 |
| `EvaluationCompleted` | 评估完成 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Supplier` | business-partner | 供应商信息 |
| `Material` | material | 物料信息 |
| `PurchasingOrganization` | organizational-units | 采购组织 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
