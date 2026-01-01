# KILLER

> 🦀 基于 Rust 微服务架构的新一代企业资源计划（ERP）系统

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/eqyc/killer/ci.yml?branch=main&label=CI)](https://github.com/eqyc/killer/actions)
[![Milestone](https://img.shields.io/badge/Milestone-v0.1.0-green)](https://github.com/eqyc/killer/milestones)

---

## ⚡ 快速启动

```bash
# 1. 启动基础设施（PostgreSQL, Redis, Kafka, ClickHouse）
docker compose -f infrastructure/docker/docker-compose.yml up -d

# 2. 初始化数据库
cargo run -p killer-cli -- db migrate

# 3. 启动网关服务
cargo run -p killer-gateway

# 4. 验证服务状态
curl http://localhost:8080/health
```

---

## 📚 文档导航

| 文档 | 说明 |
|------|------|
| [架构设计](docs/architecture/README.md) | 系统架构、领域模型、技术选型 |
| [API 文档](docs/api/README.md) | REST API 与 gRPC 接口规范 |
| [开发指南](docs/guides/README.md) | 快速开始、开发规范、最佳实践 |
| [架构决策](docs/adr/README.md) | ADR 架构决策记录 |

---

## 🛠 技术栈

| 类别 | 技术 |
|------|------|
| **语言/运行时** | Rust 2024 Edition, Tokio |
| **Web/RPC** | Axum, Tonic (gRPC) |
| **数据库** | PostgreSQL (OLTP), ClickHouse (OLAP) |
| **缓存/消息** | Redis, Apache Kafka |
| **可观测性** | OpenTelemetry, Prometheus, Grafana |

---

## 📁 项目结构

```
killer/
├── libs/                   # 共享库
│   ├── common/             # 通用工具（错误、日志、配置）
│   ├── frameworks/         # 框架封装（Web、gRPC）
│   ├── infrastructure/     # 基础设施（DB、缓存、MQ）
│   ├── master-data/        # 主数据定义
│   └── integration/        # 外部系统集成
├── services/               # 微服务（按业务域分组）
│   ├── infrastructure/     # 认证、权限、审计
│   ├── finance/            # 财务管理
│   ├── procurement-ops/    # 采购运营
│   ├── operations/         # 生产运营
│   ├── logistics/          # 仓储物流
│   ├── commercial/         # 销售客户
│   ├── project-rd/         # 项目研发
│   └── human-capital/      # 人力资源
├── proto/                  # Protocol Buffers 定义
├── infrastructure/         # DevOps 配置
├── tools/                  # 开发工具
├── scripts/                # 构建部署脚本
└── docs/                   # 项目文档
```

---

## 📄 License

[Apache License 2.0](LICENSE) © KILLER Team
