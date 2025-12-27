# 🛠️ 开发者完全手册 (Developer Handbook)

本文档是 Killer ERP 项目的**唯一**开发指南，涵盖环境搭建、服务启动、测试、迁移和代码规范。

---

## 1. ⚡ 快速开始 (Quick Start)

### 环境要求
*   **Docker & Docker Compose**: 必须安装并运行。
*   **Rust**: 1.75+ (建议使用 `rustup`).
*   **Make**: 自动化任务入口。

### 一键启动
```bash
# 1. 初始化配置 (复制 .env)
make setup

# 2. 启动基础设施 (Postgres, Redis, Clickhouse 等)
make dev

# 3. 运行数据库迁移
make db-migrate

# 4. 运行所有测试
make test

# 5. 启动 API Gateway (入口)
make run-gateway
```

---

## 2. 📦 服务管理 (Services)

所有服务位于 `services/` 目录。

| 命令 | 说明 |
| :--- | :--- |
| `make list-services` | 列出所有可用服务名 |
| `make run-service SERVICE=xxx` | 启动特定服务 (如 `crm-service`) |
| `make logs` | 查看所有 Docker 日志 |

---

## 3. 🗄️ 数据库迁移 (Migrations)

迁移脚本位于各服务的 `migrations/` 目录下。

*   **创建迁移**: `make db-new-migration NAME=create_user_table SERVICE=crm-service` (需确认工具支持)
*   **运行迁移**: `make db-migrate` (应用所有挂起的 SQL)
*   **重置数据**: `make db-reset` (⚠️ 危险：清空数据库)

---

## 4. 🧪 测试策略 (Testing)

我们采用四层测试金字塔：

1.  **单元测试 (Unit)**: `cargo test --bin <service> --lib`
    *   *位置*: `services/<service>/src/` 或 `tests/unit/`
2.  **集成测试 (Integration)**: `cargo test --test integration_tests`
    *   *位置*: `services/<service>/tests/integration/`
3.  **契约测试 (Contract)**: 验证 Protobuf 兼容性。
    *   *位置*: `services/<service>/tests/contract/`
4.  **E2E 测试**: 全链路测试。

**运行所有测试**: `make test-all`

---

## 5. 📡 接口定义 (gRPC & Protobuf)

所有接口定义在 `services/<service>/proto/`。

1.  修改 `.proto` 文件。
2.  运行 `make proto` 或 `cargo run --bin proto-gen` 生成 Rust 代码。
3.  **禁止**修改生成的 `.rs` 文件。

---

## 6. 🚀 部署与运维 (Ops)

*   **Docker**: `docker-compose.yml` 定义了所有基础设施。
*   **CI/CD**: GitHub Actions 配置文件位于 `.github/workflows/`。
*   **监控**: `make grafana` (localhost:3000), `make prometheus` (localhost:9090).

---

## 7. ❌ 常见问题 (Troubleshooting)

*   **Kafka 连接失败**: 检查 `make docker-status`，确认 Kafka 容器 `healthy`。
*   **数据库锁死**: 尝试 `make docker-restart` 重启容器。
*   **端口冲突**: 检查 `.env` 文件中的端口配置。

---
*Last Updated: 2025-12-27*
