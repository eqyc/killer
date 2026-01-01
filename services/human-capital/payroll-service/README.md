# Payroll Service (PY)

薪资核算服务，负责员工薪资的计算和发放。包括薪资结构、社保公积金、个人所得税计算和银行发放。

## 服务职责

| 模块 | 职责 |
|------|------|
| 薪资结构 | 工资项、计算规则 |
| 薪资计算 | 月度核算、追溯 |
| 社保公积金 | 缴费基数、缴费计算 |
| 个税计算 | 累计预扣法 |
| 薪资发放 | 银行代发、工资条 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `PayrollRun` | 薪资运行 |
| `PayrollResult` | 薪资结果 |
| `SalaryStructure` | 薪资结构 |
| `SocialInsurance` | 社保 |
| `HousingFund` | 公积金 |
| `TaxDeclaration` | 个税申报 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `PayrollCalculated` | 薪资计算完成 |
| `PayrollApproved` | 薪资审批 |
| `PayrollPaid` | 薪资发放 |
| `TaxDeclared` | 个税申报 |
| `SocialInsurancePaid` | 社保缴纳 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `Employee` | hr-service | 员工信息 |
| `CompanyCode` | organizational-units | 公司 |
| `CostCenter` | cost-center | 成本中心 |
| `BankAccount` | 本服务 | 工资卡 |

## 批处理任务

| 任务 | 调度 | 说明 |
|------|------|------|
| 月度薪资核算 | 每月25日 | 计算当月薪资 |
| 个税累计计算 | 每月26日 | 更新个税累计 |
| 薪资发放 | 每月28日 | 银行代发 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
