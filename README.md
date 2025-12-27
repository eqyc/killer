# KILLER ERP System

<!-- Line 1: Identity & Health (Rust -> License -> CI/Security -> Milestone) -->
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/eqyc/killer/actions/workflows/ci.yml/badge.svg)](https://github.com/eqyc/killer/actions/workflows/ci.yml)
[![Security Scan](https://github.com/eqyc/killer/actions/workflows/security-scan.yml/badge.svg)](https://github.com/eqyc/killer/actions/workflows/security-scan.yml)
[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](.github/dependabot.yml)
[![Milestone](https://img.shields.io/badge/Milestone-Wave%200--Platform-blue)](../../../Desktop/doc/Rust-BOM.md)

<!-- Line 2: Social & Activity (Stars -> Activity -> Issues -> Size) -->
[![Stars](https://img.shields.io/github/stars/eqyc/killer?style=social)](https://github.com/eqyc/killer/stargazers)
[![Forks](https://img.shields.io/github/forks/eqyc/killer?style=social)](https://github.com/eqyc/killer/network/members)
[![Commit Activity](https://img.shields.io/github/commit-activity/m/eqyc/killer)](https://github.com/eqyc/killer/graphs/commit-activity)
[![Last Commit](https://img.shields.io/github/last-commit/eqyc/killer)](https://github.com/eqyc/killer/commits/main)
[![Open Issues](https://img.shields.io/github/issues/eqyc/killer)](https://github.com/eqyc/killer/issues)
[![Open PRs](https://img.shields.io/github/issues-pr/eqyc/killer)](https://github.com/eqyc/killer/pulls)
[![Repo Size](https://img.shields.io/github/repo-size/eqyc/killer)](https://github.com/eqyc/killer)

KILLER 是一套基于 Rust 的下一代云原生 ERP 平台，采用微服务 + DDD + 事件驱动架构。本项目旨在提供一个高度可扩展、类型安全且高性能的企业资源计划系统核心。

---

## 🚀 5 分钟快速启动

### 1. 启动基础设施
```bash
# 启动 PostgreSQL, Redis, ClickHouse, Kafka 等
make dev
```

### 2. 初始化数据库
```bash
# 运行数据库迁移
make db-migrate
```

### 3. 启动应用
```bash
# 启动 API Gateway (入口)
make run-gateway
```

### 4. 验证
```bash
# 在新终端查看运行中的服务
make docker-ps
```

---

## 📚 文档导航

我们信奉**精简文档**原则。核心文档仅保留以下三份：

*   **[📖 开发者手册 (docs/DEVELOPMENT.md)](docs/DEVELOPMENT.md)**
    *   这是你最需要的文件。包含：详细的安装指南、测试方法、数据库迁移教程、服务列表和故障排查。
*   **[🏛️ 架构设计 (docs/ARCHITECTURE.md)](docs/ARCHITECTURE.md)**
    *   包含：系统全景图、限界上下文划分、技术选型理由和目录结构说明。
*   **[🤝 贡献指南 (CONTRIBUTING.md)](CONTRIBUTING.md)**
    *   包含：分支管理、提交规范和代码风格要求。

---

## 🛠️ 技术栈核心

*   **语言**: Rust 1.92 (Edition 2024)
*   **Web 框架**: Axum
*   **RPC**: Tonic (gRPC)
*   **数据库**: PostgreSQL (OLTP), ClickHouse (OLAP), Redis (Cache)
*   **消息队列**: Kafka
*   **可观测性**: OpenTelemetry, Prometheus, Grafana

## 📦 项目结构

```
killer/
├── services/           # 16+ 个微服务 (CRM, Sales, Finance...)
├── infrastructure/     # 基础设施配置 (Docker, K8s)
├── docs/               # 核心文档
├── shared/             # Rust 共享库 (Common Utils)
├── tools/              # 开发工具脚本
└── Makefile            # 任务自动化入口
```

---
*License: Apache-2.0*
