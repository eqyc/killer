# 🏛️ 系统架构设计 (System Architecture)

Killer ERP 是一个基于 Rust 的高性能微服务系统，采用领域驱动设计 (DDD) 和事件驱动架构 (EDA)。

---

## 1. 核心设计原则

*   **微服务**: 业务边界清晰，独立部署。
*   **事件驱动**: 使用 Kafka 进行服务间异步通信，解耦业务逻辑。
*   **CQRS**: 命令查询职责分离，使用 Postgres 写入，ClickHouse/OpenSearch 读取分析。
*   **Clean Architecture**: 严格分层（Domain, Infrastructure, API）。

---

## 2. 系统全景图

### 接入层
*   **API Gateway**: 统一流量入口，负责鉴权、限流、路由。
*   **Web/Mobile Client**: 前端应用（React/Flutter）。

### 核心服务层 (Bounded Contexts)
*   **CRM**: 客户管理。
*   **Sales**: 订单与销售流程。
*   **Financial**: 财务核算与总账。
*   **SCM**: 供应链与库存。
*   **HR**: 员工与组织架构。

### 基础设施层
*   **PostgreSQL**: 事务型主数据库。
*   **Redis**: 分布式缓存与会话存储。
*   **Kafka**: 消息总线。
*   **ClickHouse**: OLAP 数据仓库。
*   **Vault**: 密钥管理。

---

## 3. 安全设计 (Security)

*   **认证**: JWT (JSON Web Tokens)。
*   **通信**: 全链路 mTLS (内部服务间)。
*   **密钥**: HashiCorp Vault 统一管理 Secrets。

---

## 4. 目录结构规范

```
services/<service-name>/
├── Cargo.toml
├── src/
│   ├── api/          # gRPC/HTTP Handlers
│   ├── domain/       # 业务实体与逻辑 (纯 Rust)
│   ├── infrastructure/ # 数据库与外部适配器
│   └── main.rs
├── proto/            # 接口定义
├── migrations/       # SQL 脚本
└── tests/            # 测试套件
```

---
*更多细节请参考代码实现。*
