```
  KILLER ERP 开发顺序与提示词指南

  开发顺序总览（SAP 架构视角）

  基于 SAP 实施方法论和 DDD 最佳实践，推荐以下开发顺序：

  阶段 1: 基础设施层 (Foundation)
      ↓
  阶段 2: 主数据治理 (Master Data Governance)
      ↓
  阶段 3: 核心财务 (Finance Core)
      ↓
  阶段 4: 供应链基础 (Supply Chain Foundation)
      ↓
  阶段 5: 商业流程 (Commercial Processes)
      ↓
  阶段 6: 运营执行 (Operations Execution)
      ↓
  阶段 7: 项目与人力 (Projects & HR)
      ↓
  阶段 8: 分析与集成 (Analytics & Integration)

  ---
  阶段 1: 基础设施层

  开发顺序

  1. libs/common/ - 通用原语
  2. libs/infrastructure/ - 基础设施抽象
  3. libs/frameworks/ - 应用框架
  4. services/infrastructure/api-gateway/ - API 网关
  5. services/infrastructure/identity-iam/ - 身份认证

  提示词 1.1: 实现通用领域原语

  请为 KILLER ERP 项目实现 libs/common/domain-primitives/ 模块。

  需要实现的核心值对象：
  1. Money - 金额类型
     - 属性: amount (Decimal), currency_code (String)
     - 行为: add, subtract, multiply, divide, round
     - 约束: 金额精度 4 位小数，币种 ISO 4217

  2. Quantity - 数量类型
     - 属性: value (Decimal), unit (UnitOfMeasure)
     - 行为: add, subtract, convert_to
     - 约束: 支持单位换算

  3. AccountCode - 会计科目代码
     - 属性: code (String), chart_of_accounts (String)
     - 验证: 科目代码格式校验

  4. MaterialNumber - 物料编号
     - 属性: number (String)
     - 验证: 18 位字母数字

  5. DocumentNumber - 单据编号
     - 属性: number (String), fiscal_year (i32)
     - 生成: 支持自动编号

  技术要求：
  - 使用 rust_decimal 处理精度
  - 实现 serde Serialize/Deserialize
  - 实现 PartialEq, Eq, Hash
  - 编写单元测试覆盖边界情况
  - 遵循 DDD 值对象不可变性原则

  参考 SAP 数据类型：CURR, QUAN, CHAR

  提示词 1.2: 实现 CQRS 框架

  请为 KILLER ERP 项目实现 libs/frameworks/cqrs/ 模块。

  需要实现的核心组件：

  1. Command 抽象
     - Command trait: 定义命令接口
     - CommandHandler trait: 命令处理器
     - CommandBus: 命令分发器

  2. Query 抽象
     - Query trait: 定义查询接口
     - QueryHandler trait: 查询处理器
     - QueryBus: 查询分发器

  3. 中间件支持
     - 日志中间件
     - 验证中间件
     - 事务中间件
     - 追踪中间件

  4. 错误处理
     - CommandError 枚举
     - QueryError 枚举
     - 统一错误转换

  技术要求：
  - 使用 async_trait 支持异步
  - 支持泛型命令/查询
  - 集成 OpenTelemetry 追踪
  - 提供宏简化 Handler 注册
  - 编写集成测试

  设计参考：
  - Axon Framework (Java)
  - MediatR (.NET)

  提示词 1.3: 实现 API Gateway

  请为 KILLER ERP 项目实现 services/infrastructure/api-gateway/ 服务。

  核心功能：

  1. 路由管理
     - 基于路径的路由 (/api/v1/finance/*, /api/v1/sales/*)
     - 服务发现集成 (支持静态配置和动态发现)
     - 负载均衡 (Round Robin)

  2. 认证授权
     - JWT Token 验证
     - 从 identity-iam 服务获取公钥
     - 权限检查 (RBAC)
     - API Key 支持 (外部系统)

  3. 流量控制
     - 限流 (Token Bucket 算法)
     - 熔断 (Circuit Breaker)
     - 超时控制
     - 重试策略

  4. 可观测性
     - 请求日志 (结构化 JSON)
     - 分布式追踪 (OpenTelemetry)
     - 指标暴露 (Prometheus)
     - 健康检查端点

  5. 协议转换
     - HTTP REST → gRPC 转发
     - 请求/响应转换

  技术栈：
  - Axum 作为 HTTP 框架
  - Tower 中间件
  - Tonic 作为 gRPC 客户端
  - Redis 存储限流状态

  目录结构遵循 DDD 四层架构。

  ---
  阶段 2: 主数据治理

  开发顺序

  1. libs/master-data/ - 主数据契约定义
  2. services/infrastructure/mdg-service/ - 主数据治理服务

  提示词 2.1: 实现主数据契约

  请为 KILLER ERP 项目实现 libs/master-data/ 模块，定义全局主数据契约。

  需要实现的主数据模块：

  1. business-partner/ - 业务伙伴
     - Customer: 客户主数据 (对应 SAP KNA1)
     - Supplier: 供应商主数据 (对应 SAP LFA1)
     - BusinessPartner: 统一业务伙伴 (对应 SAP BUT000)

  2. material/ - 物料主数据
     - Material: 物料基础数据 (对应 SAP MARA)
     - MaterialPlantData: 工厂层数据 (对应 SAP MARC)
     - MaterialStorageData: 库存地点层数据 (对应 SAP MARD)

  3. organizational-units/ - 组织单元
     - CompanyCode: 公司代码
     - Plant: 工厂
     - StorageLocation: 库存地点
     - PurchasingOrganization: 采购组织
     - SalesOrganization: 销售组织

  4. cost-center/ - 成本对象
     - CostCenter: 成本中心
     - ProfitCenter: 利润中心
     - CostElement: 成本要素

  每个主数据需要：
  - 定义 Rust struct (领域模型)
  - 定义 Proto message (gRPC 契约)
  - 定义验证规则
  - 定义变更事件 (XxxCreated, XxxUpdated, XxxDeleted)

  设计原则：
  - 主数据只在 mdg-service 写入
  - 其他服务通过 gRPC 或事件获取只读副本
  - 支持多租户 (tenant_id)

  提示词 2.2: 实现 MDG 服务

  请为 KILLER ERP 项目实现 services/infrastructure/mdg-service/ 主数据治理服务。

  核心功能：

  1. 主数据 CRUD
     - 物料主数据管理 (Material)
     - 业务伙伴管理 (Customer, Supplier)
     - 组织单元管理 (CompanyCode, Plant)
     - 成本对象管理 (CostCenter, ProfitCenter)

  2. 数据治理
     - 数据验证规则引擎
     - 重复检测
     - 数据质量评分
     - 变更审批工作流

  3. 数据分发
     - 变更事件发布 (Kafka)
     - gRPC 查询接口
     - 批量导出 API

  4. 数据同步
     - 外部系统导入 (CSV, JSON)
     - SAP IDoc 适配器 (可选)
     - 增量同步支持

  数据库设计：
  - PostgreSQL 存储主数据
  - 使用 JSONB 存储扩展属性
  - 支持软删除和版本历史

  gRPC 服务定义：
  - MaterialService: GetMaterial, ListMaterials, CreateMaterial, UpdateMaterial
  - BusinessPartnerService: GetCustomer, GetSupplier, ...
  - OrganizationService: GetCompanyCode, GetPlant, ...

  事件发布：
  - MaterialCreated, MaterialUpdated
  - CustomerCreated, SupplierCreated
  - 使用 Kafka 主题: killer.mdg.events

  ---
  阶段 3: 核心财务

  开发顺序

  1. services/finance/financial-service/ - 核心财务 (FI)
  2. services/finance/controlling-service/ - 管理会计 (CO)
  3. services/finance/treasury-service/ - 资金管理 (TR)

  提示词 3.1: 实现财务服务 - 领域层

  请为 KILLER ERP 项目实现 services/finance/financial-service/ 的领域层。

  核心聚合根：

  1. JournalEntry (会计凭证) - 对应 SAP ACDOCA
     - 属性:
       - document_number: DocumentNumber
       - fiscal_year: i32
       - company_code: CompanyCode
       - posting_date: NaiveDate
       - document_date: NaiveDate
       - currency: CurrencyCode
       - status: JournalEntryStatus
       - line_items: Vec<JournalEntryLineItem>

     - 行为:
       - post(): 过账凭证
       - reverse(): 冲销凭证
       - validate(): 验证借贷平衡

     - 不变性规则:
       - 借方合计 = 贷方合计
       - 至少 2 行项目
       - 过账日期在开放期间内

  2. JournalEntryLineItem (凭证行项目)
     - 属性:
       - line_number: i32
       - account_code: AccountCode
       - amount: Money
       - debit_credit: DebitCredit
       - cost_center: Option<CostCenterId>
       - profit_center: Option<ProfitCenterId>
       - text: String

  3. FiscalPeriod (会计期间)
     - 属性: year, period, status (Open/Closed)
     - 行为: open(), close()

  领域事件：
  - JournalEntryPosted
  - JournalEntryReversed
  - FiscalPeriodClosed

  领域服务：
  - BalanceCalculator: 计算科目余额
  - PeriodCloseService: 期末结账

  Repository 接口：
  - JournalEntryRepository trait
  - FiscalPeriodRepository trait

  遵循 DDD 原则，领域层不依赖任何基础设施。

  提示词 3.2: 实现财务服务 - 应用层

  请为 KILLER ERP 项目实现 services/finance/financial-service/ 的应用层 (CQRS)。

  Commands (写操作):

  1. PostJournalEntryCommand
     - 输入: 凭证抬头 + 行项目列表
     - 处理: 验证 → 创建聚合根 → 持久化 → 发布事件
     - 输出: 凭证编号

  2. ReverseJournalEntryCommand
     - 输入: 原凭证编号, 冲销日期
     - 处理: 加载原凭证 → 生成冲销凭证 → 持久化
     - 输出: 冲销凭证编号

  3. CloseFiscalPeriodCommand
     - 输入: 公司代码, 会计年度, 期间
     - 处理: 验证未清项 → 关闭期间 → 发布事件

  Queries (读操作):

  1. GetJournalEntryQuery
     - 输入: 凭证编号, 会计年度
     - 输出: JournalEntryDto

  2. ListJournalEntriesQuery
     - 输入: 过滤条件 (日期范围, 科目, 成本中心)
     - 输出: PagedResult<JournalEntrySummaryDto>

  3. GetAccountBalanceQuery
     - 输入: 科目代码, 期间
     - 输出: AccountBalanceDto

  4. GetTrialBalanceQuery
     - 输入: 公司代码, 期间
     - 输出: Vec<TrialBalanceLineDto>

  Event Handlers:

  1. JournalEntryPostedHandler
     - 更新科目余额缓存
     - 同步到 ClickHouse (读模型)

  2. MaterialDocumentPostedHandler (来自 materials-service)
     - 自动生成物料凭证的会计分录

  应用服务：
  - JournalEntryApplicationService: 协调命令处理
  - ReportingService: 生成财务报表

  使用 libs/frameworks/cqrs 框架。

  每个财务子服务内部建议开发顺序（DDD 四层 + 交付物）：
  1) 领域层 (Domain) → 2) 应用层 (Application/CQRS) → 3) 数据库迁移 (Migrations)
  → 4) 基础设施层 (Infrastructure) → 5) API 层 (API) → 6) 测试 (Tests)

  提示词 3.3: 实现财务服务 - 基础设施层 (Infrastructure)

  请为 KILLER ERP 项目实现 services/finance/financial-service/ 的基础设施层。

  目标：让应用层可以通过 Repository/Adapter 完成持久化、事件发布与外部集成，但不污染领域层。

  需要实现：

  1. 持久化实现 (PostgreSQL)
     - 使用 sqlx 实现 JournalEntryRepository/FiscalPeriodRepository
     - 领域对象 ↔ 数据表模型映射（包含行项目）
     - 事务边界：命令处理器一次写入聚合根 + Outbox，保证原子性
     - 乐观并发控制（version 字段或 updated_at + check）

  2. Outbox/事件发布
     - outbox_messages 表：event_id, aggregate_type, aggregate_id, event_type, payload_json, occurred_at, status
     - 后台发布器：从 Outbox 拉取 → 发布到 Kafka → 标记已发送（支持重试与幂等）
     - 事件序列化：serde_json，事件版本号 (schema_version)

  3. 外部服务适配器
     - mdg-service 查询：公司代码/成本中心/利润中心存在性校验（只读）
     - materials-service 事件订阅：MaterialDocumentPosted → 触发自动会计分录（通过应用层命令）

  4. 读模型投影（可选但推荐）
     - 将 JournalEntryPosted 投影到 ClickHouse（用于试算平衡表/明细账快速查询）
     - 投影器可作为独立任务/worker（与 API 解耦）

  技术要求：
  - 基础设施层通过 trait 实现注入到应用层（不要反向依赖领域）
  - 日志、追踪、指标与重试策略（与阶段 1 可观测性一致）

  提示词 3.4: 实现财务服务 - API 层 (API)

  请为 KILLER ERP 项目实现 services/finance/financial-service/ 的 API 层。

  需要提供：

  1. gRPC API（优先）
     - JournalEntryService:
       - PostJournalEntry
       - ReverseJournalEntry
       - GetJournalEntry
       - ListJournalEntries
       - GetTrialBalance
       - GetAccountBalance
     - 错误码映射：验证失败/期间关闭/不存在/并发冲突/权限不足

  2. HTTP REST API（如需要给前端）
     - 通过 api-gateway 暴露 /api/v1/finance/fi/*
     - OpenAPI 文档输出（可选）
     - 支持分页、过滤、排序

  3. 通用 API 约束
     - 身份与租户：从 JWT/网关头部解析 tenant_id、user_id、roles
     - 幂等：写接口支持 Idempotency-Key（防止重复提交）
     - 审计：记录 who/when/what（用于合规追踪）

  提示词 3.5: 实现财务服务 - 测试 (Tests)

  请为 KILLER ERP 项目补齐 services/finance/financial-service/ 的测试体系。

  需要覆盖：
  1. 领域单元测试：借贷平衡、期间开关、冲销规则、最少行项目、币种与精度
  2. 应用层测试：命令处理流程（验证→持久化→Outbox 事件）、幂等键行为、并发冲突处理
  3. 基础设施集成测试：Repository + 真实 PostgreSQL（建议 testcontainers 或 sqlx test）
  4. API 契约测试：gRPC 请求/响应与错误码（可用 snapshot/Golden）

  提示词 3.6: 实现财务服务 - 数据库迁移 (Migrations)

  请为 KILLER ERP 项目补齐 services/finance/financial-service/ 的数据库迁移。

  PostgreSQL 需要创建（示例，可按实现调整）：
  - journal_entries（抬头：document_number, fiscal_year, company_code, posting_date, document_date, currency, status, version）
  - journal_entry_line_items（行项目：line_number, account_code, amount, debit_credit, cost_center_id, profit_center_id, text）
  - fiscal_periods（year, period, company_code, status）
  - outbox_messages（事件外盒）

  约束与索引要求：
  - journal_entries(company_code, fiscal_year, document_number) 唯一
  - 行项目按 (entry_id, line_number) 唯一
  - 借贷标识使用枚举/约束（D/C）
  - 常用查询字段建立索引（posting_date, account_code, cost_center_id）

  读模型（如使用 ClickHouse）：
  - 提供建表 DDL 与物化视图/分区策略建议（按 company_code + posting_month）

  提示词 3.7: 实现管理会计服务 - 领域层 (CO Domain)

  请为 KILLER ERP 项目实现 services/finance/controlling-service/ 的领域层。

  核心聚合根：

  1. CostCenterPosting (成本中心过账)
     - 属性:
       - posting_id
       - cost_center_id
       - cost_element_id
       - amount: Money
       - posting_date
       - source_document (关联财务凭证)

  2. InternalOrder (内部订单)
     - 属性:
       - order_id
       - order_type
       - responsible_cost_center
       - budget: Money
       - actual_cost: Money
       - status

  3. ProfitCenterPosting (利润中心过账)
     - 属性: 类似成本中心过账

  核心功能：

  1. 成本分摊
     - 按比例分摊
     - 按作业量分摊
     - 周期性分摊任务

  2. 预算管理
     - 预算编制
     - 预算检查 (过账时)
     - 预算超支预警

  3. 获利能力分析
     - 按利润中心汇总
     - 按产品线汇总
     - 边际贡献计算

  集成点：
  - 订阅 JournalEntryPosted 事件
  - 订阅 ProductionOrderCompleted 事件
  - 订阅 SalesOrderInvoiced 事件

  读模型 (ClickHouse):
  - cost_center_monthly_summary
  - profit_center_monthly_summary
  - product_profitability

  领域事件（建议）：
  - CostCenterPosted
  - InternalOrderCreated
  - BudgetAssigned
  - AllocationExecuted

  Repository 接口（建议）：
  - CostCenterPostingRepository trait
  - InternalOrderRepository trait
  - AllocationRuleRepository trait

  提示词 3.8: 实现管理会计服务 - 应用层 (CO Application/CQRS)

  请为 KILLER ERP 项目实现 services/finance/controlling-service/ 的应用层 (CQRS)。

  Commands（写）：
  1. PostCostCenterPostingCommand（从 FI/物流/销售事件或手工产生 CO 过账）
  2. CreateInternalOrderCommand / CloseInternalOrderCommand
  3. AssignBudgetCommand（预算编制/调整）
  4. ExecuteAllocationCommand（周期性分摊：按规则生成过账）

  Queries（读）：
  1. GetCostCenterActualsQuery（期间实际发生）
  2. GetBudgetVsActualQuery（预算对比）
  3. GetProfitabilitySummaryQuery（获利能力汇总）

  Event Handlers：
  - JournalEntryPostedHandler：从 FI 事件提取成本对象维度，生成 CO 读模型/过账
  - ProductionOrderCompletedHandler / SalesOrderInvoicedHandler：生成分摊或获利能力数据

  提示词 3.9: 实现管理会计服务 - 基础设施层 (CO Infrastructure)

  请为 KILLER ERP 项目实现 services/finance/controlling-service/ 的基础设施层。

  需要实现：
  - PostgreSQL Repository（sqlx）
  - Kafka 订阅与 Outbox 发布（与 FI 一致的幂等与重试）
  - 周期任务执行器（周期性分摊）：支持按期间/组织范围批处理与可重跑
  - ClickHouse 投影器：按月汇总表与产品获利能力表

  提示词 3.10: 实现管理会计服务 - API 层 (CO API)

  请为 KILLER ERP 项目实现 services/finance/controlling-service/ 的 API 层。

  建议 gRPC：
  - CostCenterPostingService: Post, Get, List
  - InternalOrderService: Create, Get, Close, List
  - BudgetService: Assign, GetVsActual
  - AllocationService: Execute, GetRunStatus

  提示词 3.11: 实现管理会计服务 - 测试 (CO Tests)

  请为 KILLER ERP 项目补齐 services/finance/controlling-service/ 的测试。

  需要覆盖：
  - 领域规则：预算检查、分摊规则正确性、状态机
  - 应用层：命令处理、事件消费幂等、周期任务可重跑
  - 基础设施：Repository + 真实 PostgreSQL、投影到 ClickHouse（可用 mock 或可选集成）

  提示词 3.12: 实现管理会计服务 - 数据库迁移 (CO Migrations)

  请为 KILLER ERP 项目补齐 services/finance/controlling-service/ 的数据库迁移。

  PostgreSQL 建议表：
  - cost_center_postings
  - internal_orders
  - budgets（或 internal_order_budgets / cost_center_budgets）
  - allocation_rules / allocation_runs
  - outbox_messages

  提示词 3.13: 实现资金管理服务 - 领域层 (TR Domain)

  请为 KILLER ERP 项目实现 services/finance/treasury-service/ 的领域层。

  核心聚合根（建议）：
  1. BankAccount（银行账户）
     - 属性：account_id, bank_name, iban/account_no, currency, status
     - 行为：activate(), deactivate()

  2. Payment（收付/付款指令）
     - 属性：payment_id, direction(In/Out), debtor_account, creditor_account, amount: Money, requested_date, status
     - 行为：initiate(), approve(), execute(), cancel(), fail(reason)
     - 不变性：审批后不可修改关键字段；同一幂等键只创建一次

  3. BankStatement（银行对账单）
     - 属性：statement_id, bank_account_id, period, lines
     - 行为：import(), reconcile(payments)

  领域事件：
  - PaymentInitiated / PaymentApproved / PaymentExecuted / PaymentFailed
  - BankStatementImported / ReconciliationCompleted

  Repository 接口（建议）：
  - BankAccountRepository trait
  - PaymentRepository trait
  - BankStatementRepository trait

  提示词 3.14: 实现资金管理服务 - 应用层 (TR Application/CQRS)

  请为 KILLER ERP 项目实现 services/finance/treasury-service/ 的应用层 (CQRS)。

  Commands（写）：
  - InitiatePaymentCommand（创建付款指令，支持幂等键）
  - ApprovePaymentCommand（多级审批可扩展）
  - ExecutePaymentCommand（调用银行适配器/模拟执行）
  - ImportBankStatementCommand（导入 camt.053/CSV）
  - ReconcileBankStatementCommand（对账与差异处理）

  Queries（读）：
  - GetPaymentQuery / ListPaymentsQuery
  - GetBankAccountBalanceQuery
  - GetCashPositionQuery（现金头寸）

  集成点：
  - 与 financial-service：付款执行后生成银行科目会计分录（可通过事件/命令）
  - 与 identity-iam：审批权限与职责分离（SoD）

  提示词 3.15: 实现资金管理服务 - 基础设施层 (TR Infrastructure)

  请为 KILLER ERP 项目实现 services/finance/treasury-service/ 的基础设施层。

  需要实现：
  - PostgreSQL Repository（sqlx）与 Outbox（同 FI/CO）
  - 银行通道适配器接口（BankingAdapter trait）
    - 默认实现：MockBankingAdapter（用于开发/测试）
    - 可选实现：ISO20022 pain.001 生成、回单/camt.053 解析
  - 敏感数据处理：账户号脱敏、加密存储（至少做到日志不泄露）

  提示词 3.16: 实现资金管理服务 - API 层 (TR API)

  请为 KILLER ERP 项目实现 services/finance/treasury-service/ 的 API 层。

  建议 gRPC：
  - BankAccountService: Create, Get, List, Activate/Deactivate
  - PaymentService: Initiate, Approve, Execute, Get, List
  - BankStatementService: Import, Reconcile, GetStatus

  通用约束：
  - 权限：发起/审批/执行分离
  - 审计：全链路记录审批流

  提示词 3.17: 实现资金管理服务 - 测试 (TR Tests)

  请为 KILLER ERP 项目补齐 services/finance/treasury-service/ 的测试。

  需要覆盖：
  - 支付状态机与审批规则
  - 幂等键与重复回调处理
  - 对账匹配（完全匹配/部分匹配/无法匹配）与差异处理
  - 银行适配器契约测试（mock）

  提示词 3.18: 实现资金管理服务 - 数据库迁移 (TR Migrations)

  请为 KILLER ERP 项目补齐 services/finance/treasury-service/ 的数据库迁移。

  PostgreSQL 建议表：
  - bank_accounts
  - payments / payment_approvals（审批记录）
  - bank_statements / bank_statement_lines
  - reconciliations（对账结果与差异）
  - outbox_messages

  ---
  阶段 4: 供应链基础

  开发顺序

  1. services/logistics/materials-service/ - 物料管理
  2. services/logistics/warehouse-service/ - 仓储管理
  3. services/procurement-ops/purchasing-service/ - 采购执行

  提示词 4.1: 实现物料管理服务

  请为 KILLER ERP 项目实现 services/logistics/materials-service/ 物料管理服务。

  核心聚合根：

  1. MaterialDocument (物料凭证) - 对应 SAP MATDOC
     - 属性:
       - document_number
       - document_date
       - movement_type: MovementType (收货/发货/转储)
       - plant_id
       - storage_location_id
       - items: Vec<MaterialDocumentItem>

     - 行为:
       - post(): 过账物料凭证
       - reverse(): 冲销

  2. MaterialDocumentItem
     - 属性:
       - material_id
       - quantity: Quantity
       - batch: Option<BatchNumber>
       - serial_numbers: Vec<SerialNumber>
       - valuation_type
       - amount: Money (库存价值)

  3. StockBalance (库存余额) - 读模型
     - 属性:
       - material_id
       - plant_id
       - storage_location_id
       - unrestricted_stock: Quantity
       - blocked_stock: Quantity
       - quality_inspection_stock: Quantity

  移动类型 (MovementType):
  - 101: 采购收货
  - 102: 采购收货冲销
  - 201: 生产发料
  - 261: 生产消耗
  - 301: 工厂间转储
  - 501: 无采购订单收货

  领域事件：
  - MaterialDocumentPosted
  - StockLevelChanged
  - StockBelowSafetyLevel (预警)

  集成点：
  - 发布 MaterialDocumentPosted → financial-service (自动生成会计凭证)
  - 订阅 PurchaseOrderGoodsReceived → 创建 101 收货凭证
  - 订阅 ProductionOrderIssued → 创建 261 发料凭证

  库存计算策略：
  - 使用事件溯源计算实时库存
  - 定期生成库存快照 (每日)

  提示词 4.2: 实现采购服务

  请为 KILLER ERP 项目实现 services/procurement-ops/purchasing-service/ 采购服务。

  核心聚合根：

  1. PurchaseOrder (采购订单) - 对应 SAP EKKO/EKPO
     - 属性:
       - order_number
       - supplier_id (引用 mdg-service)
       - purchasing_org_id
       - order_date
       - status: PurchaseOrderStatus
       - items: Vec<PurchaseOrderItem>

     - 行为:
       - submit(): 提交审批
       - approve(): 审批通过
       - release(): 下达
       - receive_goods(item_id, quantity): 收货确认
       - complete(): 完成订单

  2. PurchaseOrderItem
     - 属性:
       - item_number
       - material_id
       - quantity: Quantity
       - unit_price: Money
       - delivery_date
       - plant_id
       - received_quantity: Quantity
       - invoiced_quantity: Quantity

  3. PurchaseRequisition (采购申请) - 对应 SAP EBAN
     - 属性:
       - requisition_number
       - requester_id
       - items: Vec<PurchaseRequisitionItem>
       - status

  业务流程：
  1. 采购申请 → 审批 → 转采购订单
  2. 采购订单 → 下达 → 收货 → 发票校验 → 完成

  三单匹配 (3-Way Match):
  - 采购订单数量
  - 收货数量
  - 发票数量
  - 容差检查

  领域事件：
  - PurchaseOrderCreated
  - PurchaseOrderReleased
  - GoodsReceived
  - InvoiceVerified
  - PurchaseOrderCompleted

  集成点：
  - 调用 mdg-service 获取供应商信息
  - 发布 GoodsReceived → materials-service (创建收货凭证)
  - 发布 InvoiceVerified → financial-service (创建应付凭证)

  ---
  阶段 5: 商业流程

  开发顺序

  1. services/commercial/sales-service/ - 销售订单
  2. services/commercial/crm-service/ - 客户关系
  3. services/logistics/shipping-service/ - 运输管理

  提示词 5.1: 实现销售服务

  请为 KILLER ERP 项目实现 services/commercial/sales-service/ 销售服务。

  核心聚合根：

  1. SalesOrder (销售订单) - 对应 SAP VBAK/VBAP
     - 属性:
       - order_number
       - customer_id (引用 mdg-service)
       - sales_org_id
       - order_date
       - requested_delivery_date
       - status: SalesOrderStatus
       - items: Vec<SalesOrderItem>
       - pricing: PricingResult

     - 行为:
       - confirm(): 确认订单
       - schedule_delivery(): 安排交货
       - create_delivery(): 创建交货单
       - invoice(): 开票

  2. SalesOrderItem
     - 属性:
       - item_number
       - material_id
       - quantity: Quantity
       - unit_price: Money
       - discount: Money
       - net_value: Money
       - plant_id
       - delivered_quantity
       - invoiced_quantity

  3. BillingDocument (开票单据) - 对应 SAP VBRK/VBRP
     - 属性:
       - billing_number
       - billing_date
       - customer_id
       - items: Vec<BillingDocumentItem>
       - total_amount: Money
       - tax_amount: Money

  4. Delivery (交货单)
     - 属性:
       - delivery_number
       - sales_order_ref
       - ship_to_address
       - items: Vec<DeliveryItem>
       - status

  定价引擎：
  - 基础价格 (物料价格表)
  - 客户折扣
  - 数量折扣
  - 促销价格
  - 税费计算

  领域事件：
  - SalesOrderCreated
  - SalesOrderConfirmed
  - DeliveryCreated
  - GoodsIssued
  - BillingDocumentCreated

  集成点：
  - 调用 mdg-service 获取客户信息
  - 调用 materials-service 检查库存
  - 发布 GoodsIssued → materials-service (创建发货凭证)
  - 发布 BillingDocumentCreated → financial-service (创建应收凭证)

  ---
  阶段 6: 运营执行

  开发顺序

  1. services/operations/production-service/ - 生产计划
  2. services/operations/quality-service/ - 质量管理
  3. services/operations/maintenance-service/ - 设备维护

  提示词 6.1: 实现生产服务

  请为 KILLER ERP 项目实现 services/operations/production-service/ 生产服务。

  核心聚合根：

  1. ProductionOrder (生产订单) - 对应 SAP AFKO/AFPO
     - 属性:
       - order_number
       - material_id (成品物料)
       - quantity: Quantity
       - plant_id
       - planned_start_date
       - planned_end_date
       - actual_start_date
       - actual_end_date
       - status: ProductionOrderStatus
       - operations: Vec<ProductionOperation>
       - components: Vec<ProductionComponent>

     - 行为:
       - release(): 下达生产
       - start_operation(op_id): 开始工序
       - confirm_operation(op_id, quantity): 报工
       - issue_components(): 发料
       - receive_finished_goods(): 成品入库
       - complete(): 完成订单

  2. ProductionOperation (生产工序)
     - 属性:
       - operation_number
       - work_center_id
       - planned_duration
       - actual_duration
       - confirmed_quantity
       - status

  3. ProductionComponent (生产组件/BOM 展开)
     - 属性:
       - component_material_id
       - required_quantity
       - issued_quantity
       - storage_location_id

  4. PlannedOrder (计划订单) - 对应 SAP PLAF
     - 属性:
       - order_number
       - material_id
       - quantity
       - planned_date
       - source: MRP/Manual

     - 行为:
       - convert_to_production_order(): 转生产订单

  MRP 运算 (简化版):
  - 需求计算 (销售订单 + 安全库存)
  - 供给计算 (库存 + 在途 + 计划订单)
  - 净需求 = 需求 - 供给
  - 生成计划订单

  领域事件：
  - ProductionOrderReleased
  - OperationStarted
  - OperationConfirmed
  - ComponentsIssued
  - FinishedGoodsReceived
  - ProductionOrderCompleted

  集成点：
  - 调用 plm-service 获取 BOM
  - 调用 materials-service 检查组件库存
  - 发布 ComponentsIssued → materials-service (创建发料凭证)
  - 发布 FinishedGoodsReceived → materials-service (创建入库凭证)
  - 发布 ProductionOrderCompleted → controlling-service (成本结算)

  ---
  阶段 7: 项目与人力

  开发顺序

  1. services/project-rd/project-service/ - 项目管理
  2. services/project-rd/plm-service/ - 产品生命周期
  3. services/human-capital/hr-service/ - 人事管理
  4. services/human-capital/payroll-service/ - 薪资核算

  提示词 7.1: 实现 PLM 服务

  请为 KILLER ERP 项目实现 services/project-rd/plm-service/ 产品生命周期管理服务。

  核心聚合根：

  1. BillOfMaterials (物料清单) - 对应 SAP STKO/STPO
     - 属性:
       - bom_id
       - material_id (父物料)
       - plant_id
       - usage: BOMUsage (生产/工程/销售)
       - valid_from
       - valid_to
       - status: BOMStatus
       - components: Vec<BOMComponent>

     - 行为:
       - add_component()
       - remove_component()
       - update_quantity()
       - create_new_version(): 版本管理
       - activate()
       - deactivate()

  2. BOMComponent
     - 属性:
       - item_number
       - component_material_id
       - quantity: Quantity
       - unit_of_measure
       - scrap_percentage
       - is_phantom: bool (虚拟件)

  3. Routing (工艺路线) - 对应 SAP PLKO/PLPO
     - 属性:
       - routing_id
       - material_id
       - plant_id
       - operations: Vec<RoutingOperation>

  4. RoutingOperation
     - 属性:
       - operation_number
       - work_center_id
       - setup_time
       - machine_time
       - labor_time
       - description

  5. EngineeringChangeOrder (工程变更单)
     - 属性:
       - eco_number
       - affected_boms: Vec<BOMId>
       - affected_routings: Vec<RoutingId>
       - effective_date
       - status
       - approvals: Vec<Approval>

  版本管理：
  - BOM 支持多版本
  - 使用事件溯源记录变更历史
  - 支持按日期查询有效版本

  领域事件：
  - BOMCreated
  - BOMVersionCreated
  - BOMActivated
  - RoutingCreated
  - ECOApproved

  集成点：
  - 被 production-service 调用获取 BOM 展开
  - 被 sales-service 调用获取销售 BOM

  ---
  阶段 8: 分析与集成

  开发顺序

  1. ClickHouse 读模型设计
  2. 报表服务实现
  3. 外部系统集成

  提示词 8.1: 实现 ClickHouse 读模型

  请为 KILLER ERP 项目设计 ClickHouse 读模型架构。

  需要创建的分析表：

  1. 财务分析
     - journal_entries_fact: 凭证明细事实表
     - account_balances_daily: 科目日余额
     - trial_balance_monthly: 月度试算平衡
     - cash_flow_analysis: 现金流分析

  2. 销售分析
     - sales_orders_fact: 销售订单事实表
     - sales_by_customer: 客户销售汇总
     - sales_by_product: 产品销售汇总
     - sales_trend_daily: 日销售趋势

  3. 采购分析
     - purchase_orders_fact: 采购订单事实表
     - supplier_performance: 供应商绩效
     - procurement_spend: 采购支出分析

  4. 库存分析
     - inventory_movements_fact: 库存移动事实表
     - stock_levels_daily: 日库存水平
     - inventory_turnover: 库存周转率
     - slow_moving_items: 呆滞物料

  5. 生产分析
     - production_orders_fact: 生产订单事实表
     - production_efficiency: 生产效率
     - quality_metrics: 质量指标

       ---
  提示词 8.1 (续): ClickHouse 读模型

  数据同步策略：

  1. CDC 实时同步 (Debezium)
     - PostgreSQL → Kafka → ClickHouse
     - 延迟 < 5 秒
     - 适用于事实表

  2. 批量同步 (定时任务)
     - 每日凌晨执行
     - 适用于汇总表、维度表
     - 使用 ClickHouse 物化视图

  表设计原则：
  - 使用 MergeTree 引擎
  - 按日期分区 (toYYYYMM)
  - 合理设置排序键 (ORDER BY)
  - 使用 LowCardinality 优化字符串
  - 预聚合常用查询 (物化视图)

  示例表结构：

  CREATE TABLE journal_entries_fact (
      document_number String,
      fiscal_year UInt16,
      company_code LowCardinality(String),
      posting_date Date,
      account_code LowCardinality(String),
      cost_center LowCardinality(Nullable(String)),
      profit_center LowCardinality(Nullable(String)),
      amount Decimal(18, 4),
      currency LowCardinality(String),
      debit_credit Enum8('D' = 1, 'C' = 2),
      created_at DateTime
  ) ENGINE = MergeTree()
  PARTITION BY toYYYYMM(posting_date)
  ORDER BY (company_code, posting_date, document_number);

  查询接口：
  - 提供 gRPC 查询服务
  - 支持动态过滤条件
  - 支持分页和排序
  - 返回 Arrow 格式 (高效传输)

  提示词 8.2: 实现事件驱动集成

  请为 KILLER ERP 项目实现跨服务事件驱动集成架构。

  Kafka 主题设计：

  1. 领域事件主题 (按服务)
     - killer.finance.events
     - killer.sales.events
     - killer.materials.events
     - killer.production.events
     - killer.purchasing.events

  2. 集成事件主题 (跨服务)
     - killer.integration.events

  3. 死信队列
     - killer.deadletter

  事件格式 (CloudEvents 规范):
  {
    "specversion": "1.0",
    "type": "com.killer.finance.JournalEntryPosted",
    "source": "/services/financial-service",
    "id": "uuid",
    "time": "2026-01-02T10:00:00Z",
    "datacontenttype": "application/json",
    "data": { ... }
  }

  关键集成流程：

  1. 采购到付款 (P2P)
     PurchaseOrderReleased
     → GoodsReceived (purchasing-service)
     → MaterialDocumentPosted (materials-service)
     → JournalEntryPosted (financial-service)
     → InvoiceVerified (purchasing-service)
     → PaymentCreated (treasury-service)

  2. 订单到收款 (O2C)
     SalesOrderConfirmed
     → DeliveryCreated (sales-service)
     → GoodsIssued (materials-service)
     → MaterialDocumentPosted (materials-service)
     → BillingDocumentCreated (sales-service)
     → JournalEntryPosted (financial-service)
     → PaymentReceived (treasury-service)

  3. 计划到生产 (P2M)
     PlannedOrderCreated
     → ProductionOrderReleased (production-service)
     → ComponentsIssued (production-service)
     → MaterialDocumentPosted (materials-service)
     → OperationConfirmed (production-service)
     → FinishedGoodsReceived (production-service)
     → ProductionOrderCompleted (production-service)
     → CostSettled (controlling-service)

  Saga 编排 (关键业务):
  - 使用 libs/frameworks/saga-orchestration
  - 补偿事务处理
  - 幂等性保证
  - 超时处理

  实现要求：
  - 消费者使用 Consumer Group
  - 支持消息重试 (指数退避)
  - 死信队列处理
  - 消息追踪 (correlation_id)

  ---
  开发优先级矩阵

  根据业务价值和技术依赖，推荐以下优先级：

  | 优先级 | 服务                 | 理由             | 预计工期 |
  |--------|----------------------|------------------|----------|
  | P0     | libs/common          | 所有服务依赖     | 1 周     |
  | P0     | libs/infrastructure  | 基础设施抽象     | 1 周     |
  | P0     | libs/frameworks/cqrs | CQRS 核心        | 1 周     |
  | P0     | mdg-service          | 主数据是一切基础 | 2 周     |
  | P1     | api-gateway          | 统一入口         | 1 周     |
  | P1     | identity-iam         | 认证授权         | 1 周     |
  | P1     | financial-service    | 财务是核心       | 3 周     |
  | P1     | materials-service    | 库存是基础       | 2 周     |
  | P2     | purchasing-service   | 采购流程         | 2 周     |
  | P2     | sales-service        | 销售流程         | 2 周     |
  | P2     | controlling-service  | 成本核算         | 2 周     |
  | P3     | production-service   | 生产执行         | 2 周     |
  | P3     | warehouse-service    | 仓储管理         | 2 周     |
  | P3     | plm-service          | BOM/工艺         | 2 周     |
  | P4     | 其他服务             | 按需开发         | -        |

  ---
  每个服务的标准开发提示词模板

  请为 KILLER ERP 项目实现 services/{domain}/{service-name}/ 服务。

  【第一步：领域层】
  实现 src/domain/ 目录：
  1. 定义聚合根 (aggregates/)
     - 属性、行为、不变性规则
  2. 定义实体 (entities/)
  3. 定义值对象 (value_objects/)
  4. 定义领域事件 (events/)
  5. 定义 Repository 接口 (repositories/)
  6. 定义领域服务 (services/) - 如有跨聚合逻辑

  【第二步：应用层】
  实现 src/application/ 目录：
  1. Commands (commands/)
     - 列出所有写操作命令
     - 实现 CommandHandler
  2. Queries (queries/)
     - 列出所有读操作查询
     - 实现 QueryHandler
  3. Event Handlers (event_handlers/)
     - 处理来自其他服务的集成事件
  4. DTOs (dto/)
     - 请求/响应数据结构

  【第三步：基础设施层】
  实现 src/infrastructure/ 目录：
  1. PostgreSQL Repository 实现 (persistence/)
  2. Kafka 消息发布/订阅 (messaging/)
  3. Redis 缓存 (cache/)
  4. 外部服务客户端 (clients/)

  【第四步：API 层】
  实现 src/api/ 目录：
  1. gRPC 服务实现 (grpc/)
  2. HTTP REST 端点 (http/) - 可选
  3. 请求验证

  【第五步：测试】
  实现 tests/ 目录：
  1. 单元测试 (unit/) - 领域逻辑
  2. 集成测试 (integration/) - Repository + 数据库
  3. API 测试 - gRPC/HTTP 端点

  【第六步：数据库迁移】
  实现 migrations/ 目录：
  - 创建表结构
  - 索引设计
  - 初始数据

  技术要求：
  - 遵循 DDD 四层架构
  - 使用 CQRS 模式
  - 集成 OpenTelemetry
  - 编写文档注释

  ---
  服务间依赖关系图

                      ┌─────────────────┐
                      │   api-gateway   │
                      └────────┬────────┘
                               │
           ┌───────────────────┼───────────────────┐
           │                   │                   │
           ▼                   ▼                   ▼
  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
  │  identity-iam   │ │   mdg-service   │ │ financial-svc   │
  └─────────────────┘ └────────┬────────┘ └────────┬────────┘
                               │                   │
           ┌───────────────────┼───────────────────┤
           │                   │                   │
           ▼                   ▼                   ▼
  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
  │  materials-svc  │◄┤ purchasing-svc  │ │ controlling-svc │
  └────────┬────────┘ └────────┬────────┘ └─────────────────┘
           │                   │
           ▼                   ▼
  ┌─────────────────┐ ┌─────────────────┐
  │  warehouse-svc  │ │   sales-svc     │
  └─────────────────┘ └────────┬────────┘
                               │
                               ▼
                      ┌─────────────────┐
                      │  shipping-svc   │
                      └─────────────────┘

  ┌─────────────────┐ ┌─────────────────┐
  │ production-svc  │◄┤   plm-service   │
  └─────────────────┘ └─────────────────┘

  图例：
  ─── gRPC 同步调用
  ◄── 事件订阅

  ---
  关键技术提示词

  提示词: 实现分布式事务 (Saga)

  请为 KILLER ERP 项目实现 libs/frameworks/saga-orchestration/ Saga 编排框架。

  核心组件：

  1. Saga 定义
     - SagaDefinition trait: 定义 Saga 步骤
     - SagaStep: 单个步骤 (执行 + 补偿)
     - SagaContext: 跨步骤共享数据

  2. Saga 编排器
     - SagaOrchestrator: 执行 Saga
     - 状态持久化 (PostgreSQL)
     - 失败重试机制
     - 补偿事务执行

  3. 示例 Saga: 创建销售订单
     Step 1: 检查库存 (materials-service)
       - 执行: ReserveStock
       - 补偿: ReleaseStock

     Step 2: 检查客户信用 (financial-service)
       - 执行: CheckCredit
       - 补偿: (无需补偿)

     Step 3: 创建订单 (sales-service)
       - 执行: CreateOrder
       - 补偿: CancelOrder

     Step 4: 发送确认 (notification-service)
       - 执行: SendConfirmation
       - 补偿: (无需补偿)

  实现要求：
  - 步骤执行幂等性
  - 补偿执行幂等性
  - 超时处理
  - 状态机持久化
  - 支持异步步骤
  - 集成 OpenTelemetry 追踪

  提示词: 实现批处理框架

  请为 KILLER ERP 项目实现 libs/frameworks/batch-framework/ 批处理框架。

  核心功能：

  1. Job 定义
     - Job trait: 批处理任务接口
     - JobStep: 任务步骤 (Reader → Processor → Writer)
     - JobContext: 任务上下文

  2. 调度器
     - Cron 表达式支持
     - 手动触发
     - 任务依赖链

  3. 执行引擎
     - 分片执行 (大数据量)
     - 断点续传
     - 失败重试
     - 并行处理

  4. 监控
     - 任务状态跟踪
     - 执行日志
     - 指标采集

  典型批处理任务：

  1. 期末结账 (月度)
     - 检查未清凭证
     - 计算期末余额
     - 关闭会计期间
     - 生成结账报告

  2. MRP 运算 (每日)
     - 读取销售订单需求
     - 读取当前库存
     - 计算净需求
     - 生成计划订单

  3. 数据归档 (每月)
     - 读取历史数据 (> 2年)
     - 写入 ClickHouse
     - 删除 PostgreSQL 数据

  4. 对账任务 (每日)
     - 财务凭证 vs 物料凭证
     - 生成差异报告

  实现参考：
  - Spring Batch
  - Apache Airflow

  提示词: 实现规则引擎

  请为 KILLER ERP 项目实现定价规则引擎 (用于 sales-service)。

  核心功能：

  1. 规则定义
     - 条件规则 (客户类型、物料组、数量范围)
     - 计算规则 (折扣率、固定金额、阶梯价)
     - 规则优先级

  2. 规则存储
     - 规则元数据存储 (PostgreSQL)
     - 规则版本管理
     - 规则生效日期

  3. 规则执行
     - 规则匹配
     - 计算执行
     - 结果缓存

  定价过程 (Pricing Procedure):
  1. 基础价格 (PR00) - 物料价格表
  2. 客户折扣 (K004) - 客户特定折扣
  3. 物料组折扣 (K005) - 物料组折扣
  4. 数量折扣 (K007) - 阶梯价格
  5. 促销价格 (K020) - 临时促销
  6. 运费 (KF00) - 按重量/体积
  7. 税费 (MWST) - 增值税

  规则示例：
  {
    "rule_id": "K007",
    "name": "数量折扣",
    "condition": {
      "material_group": "01",
      "quantity_range": [100, 500]
    },
    "action": {
      "type": "percentage_discount",
      "value": 5.0
    },
    "priority": 30
  }

  API:
  - CalculatePrice(order_items) → PricingResult
  - GetApplicableRules(context) → Vec<Rule>
  - SimulatePrice(order_items) → PricingSimulation

  ---
  测试策略提示词

  请为 KILLER ERP 项目制定测试策略。

  测试金字塔：

  1. 单元测试 (70%)
     - 领域逻辑测试
     - 值对象测试
     - 聚合根行为测试
     - 使用 mock 隔离依赖

  2. 集成测试 (20%)
     - Repository + 数据库测试
     - 使用 Testcontainers (PostgreSQL)
     - 消息发布/订阅测试
     - 使用 Testcontainers (Kafka)

  3. 端到端测试 (10%)
     - 完整业务流程测试
     - 多服务协作测试
     - 使用 docker-compose 启动环境

  测试工具：
  - 框架: Rust 内置测试 + tokio-test
  - Mock: mockall crate
  - 容器: testcontainers-rs
  - 属性测试: proptest
  - 基准测试: criterion

  测试数据管理：
  - 使用 Factory 模式生成测试数据
  - 数据库测试使用事务回滚
  - 集成测试使用独立数据库

  CI 集成：
  - PR 触发单元测试 + 集成测试
  - 每日运行端到端测试
  - 覆盖率要求 > 80%

  示例测试结构：
  tests/
  ├── unit/
  │   ├── domain/
  │   │   ├── journal_entry_test.rs
  │   │   └── money_test.rs
  │   └── application/
  │       └── post_journal_entry_handler_test.rs
  ├── integration/
  │   ├── repository/
  │   │   └── journal_entry_repository_test.rs
  │   └── messaging/
  │       └── event_publisher_test.rs
  └── e2e/
      └── posting_flow_test.rs

  ---
  总结

  开发顺序关键原则

  1. 基础先行 - libs 和基础设施服务优先
  2. 主数据优先 - mdg-service 是所有业务服务的基础
  3. 核心财务优先 - 财务是 ERP 的核心
  4. 按业务流程展开 - P2P、O2C、P2M 流程驱动
  5. 增量集成 - 每个服务完成后立即集成测试

  每个服务的开发步骤

  1. 领域建模 → 2. 领域层实现 → 3. 应用层实现
  → 4. 基础设施层实现 → 5. API 层实现
  → 6. 单元测试 → 7. 集成测试 → 8. 文档

  参考 SAP 的关键表映射

  | 服务               | SAP 核心表 | 聚合根           |
  |--------------------|------------|------------------|
  | financial-service  | ACDOCA     | JournalEntry     |
  | purchasing-service | EKKO/EKPO  | PurchaseOrder    |
  | sales-service      | VBAK/VBAP  | SalesOrder       |
  | materials-service  | MATDOC     | MaterialDocument |
  | production-service | AFKO/AFPO  | ProductionOrder  |
  | plm-service        | STKO/STPO  | BillOfMaterials  |

```
