# Purchasing Service (MM)

采购执行服务，负责采购业务的全流程管理。包括采购申请、采购订单、收货和发票校验。实现采购到付款（P2P）流程。

## 服务职责

| 模块 | 职责 |
|------|------|
| 采购申请 | 需求提报、审批 |
| 采购订单 | PO 创建、变更、关闭 |
| 收货 | 入库确认、质检 |
| 发票校验 | 三单匹配 |
| 采购报表 | 采购分析 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `PurchaseRequisition` | 采购申请 |
| `PurchaseOrder` | 采购订单 |
| `GoodsReceipt` | 收货单 |
| `InvoiceVerification` | 发票校验 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `RequisitionCreated` | 申请创建 |
| `RequisitionApproved` | 申请审批 |
| `PurchaseOrderCreated` | PO 创建 |
| `GoodsReceived` | 收货确认 |
| `InvoiceVerified` | 发票校验通过 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Supplier` | business-partner | 供应商 |
| `Material` | material | 物料 |
| `Plant` | organizational-units | 工厂 |
| `PurchasingOrganization` | organizational-units | 采购组织 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
