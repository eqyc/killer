# 项目目录结构 (Directory Structure)

本文档详细描述了 `killer` 项目的目录结构及其功能模块。

## 概览 (Overview)

项目采用 Monorepo 架构，所有服务、库和配置文件都集中在单一代码库中，便于版本控制和依赖管理。

```text
/Users/x/killer
├── .env.example            # 环境变量示例文件
├── .github/                # GitHub Action 自动化工作流
├── Cargo.toml              # Rust 项目配置文件 (包含核心依赖定义)
├── Cargo.lock              # Rust 依赖版本锁定文件
├── Makefile                # 常用编译、运行和部署命令
├── README.md               # 项目总体介绍
├── LICENSE                 # 项目许可证
├── config/                 # 全局配置文件
├── docs/                   # 项目文档目录
├── infrastructure/         # 基础设施定义
│   ├── clickhouse/         # Clickhouse 数据库模式和迁移
│   ├── docker/             # Docker 镜像构建相关文件
│   ├── helm/               # Helm Charts (Kubernetes 部署)
│   ├── k8s/                # Kubernetes 资源定义
│   ├── monitoring/         # 监控相关配置 (Prometheus, Grafana)
│   └── terraform/          # 云基础设施即代码 (IaC)
├── libs/                   # 内部共享库
│   ├── common/             # 基础工具类、错误定义、常用宏
│   ├── frameworks/         # 框架级封装 (如 CQRS 基础实现)
│   ├── infrastructure/     # 基础设施连接/抽象层 (数据库客户端、缓存等)
│   ├── integration/        # 第三方系统集成适配器
│   └── master-data/        # 跨服务共享的主数据领域模型
├── proto/                  # Protobuf 接口定义文件 (gRPC 契约)
│   ├── commercial/         # 商务模块接口
│   ├── common/             # 通用类型定义
│   ├── finance/            # 财务模块接口
│   └── ... (按业务线划分)
├── services/               # 核心业务微服务
│   ├── commercial/         # 商务相关服务 (销售、CRM)
│   ├── finance/            # 财务/金融相关服务 (总账、成本、资金)
│   ├── human-capital/      # 人力资源相关服务
│   ├── infrastructure/     # 基础设施支持服务 (网关、身份认证)
│   ├── logistics/          # 物流相关服务 (物料管理、仓储)
│   ├── operations/         # 运营相关服务
│   ├── procurement-ops/    # 采购运营服务
│   └── project-rd/         # 项目研发管理服务
├── scripts/                # 运维、测试和构建脚本
└── tools/                  # 开发测试辅助工具
```

## 核心设计规范 (Core Design Principles)

1.  **Monorepo**: 所有模块共享同一个 Rust Workspace。
2.  **Schema-First**: 使用 `proto/` 下的 Protobuf 定义服务间契约。
3.  **Clean Architecture**: 多层布局，通常包含 `Domain`, `Application`, `Infrastructure`, `API` 层。
4.  **云原生**: `infrastructure/` 中包含完整的容器化和 K8s 部署配置。

---
最后更新时间: 2026-01-02
