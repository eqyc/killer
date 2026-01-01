# Financial Service (FI)

核心财务会计服务，负责企业财务核算的核心功能。包括总账管理、应收账款、应付账款、资产会计和税务管理。所有财务交易通过此服务记录并生成合规的财务报表。

## 服务职责

| 模块 | 职责 |
|------|------|
| 总账 (GL) | 科目表管理、会计凭证、期间结算 |
| 应收 (AR) | 客户发票、收款、账龄分析 |
| 应付 (AP) | 供应商发票、付款、对账 |
| 资产 (AA) | 固定资产折旧、资产处置 |
| 税务 (TX) | 增值税、所得税计算与申报 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `JournalEntry` | 会计凭证（总账分录） |
| `AccountingDocument` | 原始凭证（发票、收据） |
| `CustomerInvoice` | 客户发票 |
| `VendorInvoice` | 供应商发票 |
| `FixedAsset` | 固定资产 |
| `FiscalPeriod` | 会计期间 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `JournalEntryPosted` | 凭证过账 |
| `JournalEntryReversed` | 凭证冲销 |
| `InvoiceCreated` | 发票创建 |
| `PaymentReceived` | 收款确认 |
| `PaymentSent` | 付款确认 |
| `AssetDepreciated` | 资产折旧 |
| `PeriodClosed` | 期间关闭 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `ChartOfAccounts` | 本服务 | 科目表 |
| `CompanyCode` | organizational-units | 公司代码 |
| `CostCenter` | cost-center | 成本中心 |
| `ProfitCenter` | cost-center | 利润中心 |
| `BusinessPartner` | business-partner | 客户/供应商 |
| `TaxCode` | 本服务 | 税码 |

## 集成接口

| 接口类型 | 端点 | 说明 |
|----------|------|------|
| gRPC | `FinancialService` | 服务间调用 |
| REST | `/api/v1/finance/*` | 外部 API |
| Kafka | `finance.journal-entries` | 凭证事件 |
| Kafka | `finance.payments` | 支付事件 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |

## 启动命令

```bash
# 开发环境
cargo run -p killer-financial-service

# 生产环境
./financial-service --config config/production.toml
```
