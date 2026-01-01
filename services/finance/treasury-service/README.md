# Treasury Service (TR)

资金管理服务，负责企业资金的集中管理和银行通信。包括现金管理、银行账户管理、付款处理和资金预测。

## 服务职责

| 模块 | 职责 |
|------|------|
| 现金管理 | 现金头寸、流动性分析 |
| 银行通信 | 银企直连、对账 |
| 付款处理 | 付款运行、批量付款 |
| 资金预测 | 现金流预测 |
| 银行账户 | 账户管理、余额查询 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `BankAccount` | 银行账户 |
| `PaymentRun` | 付款运行 |
| `CashPosition` | 现金头寸 |
| `BankStatement` | 银行对账单 |
| `CashForecast` | 现金预测 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `PaymentExecuted` | 付款执行 |
| `BankStatementReceived` | 对账单接收 |
| `ReconciliationCompleted` | 对账完成 |
| `CashPositionUpdated` | 头寸更新 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `HouseBank` | 本服务 | 开户银行 |
| `CompanyCode` | organizational-units | 公司代码 |
| `BusinessPartner` | business-partner | 收付款方 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
