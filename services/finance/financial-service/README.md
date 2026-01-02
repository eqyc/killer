# KILLER ERP Financial Service

## 概述

财务服务是 KILLER ERP 的核心业务微服务，提供完整的会计凭证管理、会计期间控制、科目余额查询和试算平衡功能。

## 架构设计

```
financial-service/
├── Cargo.toml                    # 服务配置 + 依赖
├── src/
│   ├── main.rs                   # 程序入口
│   ├── lib.rs                    # 库入口，导出所有模块
│   ├── domain/                   # 领域层（DDD）
│   │   ├── mod.rs
│   │   ├── aggregates/           # 聚合根
│   │   │   ├── mod.rs
│   │   │   ├── journal_entry.rs  # 会计凭证
│   │   │   └── fiscal_period.rs  # 会计期间
│   │   ├── entities/
│   │   │   └── journal_entry_line_item.rs
│   │   ├── value_objects.rs
│   │   ├── events.rs
│   │   ├── services.rs
│   │   ├── repositories.rs
│   │   └── error.rs
│   ├── application/              # 应用层（CQRS）
│   │   ├── mod.rs
│   │   ├── commands/             # 命令处理器（写模型）
│   │   ├── queries/              # 查询处理器（读模型）
│   │   ├── events/               # 事件处理器
│   │   ├── dto/                  # 数据传输对象
│   │   ├── mapper/               # DTO 映射器
│   │   ├── services/             # 应用服务
│   │   ├── repositories/         # 读模型仓储接口
│   │   └── error.rs
│   ├── infrastructure/           # 基础设施层
│   │   ├── mod.rs
│   │   ├── persistence/          # 数据库适配器
│   │   ├── messaging/            # Kafka 适配器
│   │   └── observability/        # 日志/指标
│   └── interfaces/               # 接口层
│       ├── mod.rs
│       ├── grpc/                 # gRPC 接口
│       └── rest/                 # REST API
├── config/                       # 配置文件
├── migrations/                   # 数据库迁移
├── Dockerfile
└── README.md
```

## 模块职责

| 层级 | 职责 | 示例 |
|------|------|------|
| **Domain** | 业务规则、不变式、领域逻辑 | 借贷平衡、凭证过账验证 |
| **Application** | 编排、事务、CQRS 处理器 | CommandBus、EventHandler |
| **Infrastructure** | 数据库、缓存、消息队列 | SQLx、Redis、Kafka |
| **Interface** | API、协议转换 | gRPC、REST、事件订阅 |

## CQRS 流程

### 命令流（写模型）

```
API → CommandBus → CommandHandler → Aggregate → Repository + EventBus
```

### 查询流（读模型）

```
API → QueryBus → QueryHandler → ReadModel/Cache → DTO
```

## 命令

| 命令 | 描述 |
|------|------|
| `CreateJournalEntry` | 创建会计凭证（草稿） |
| `PostJournalEntry` | 过账会计凭证 |
| `ReverseJournalEntry` | 冲销会计凭证 |
| `CloseFiscalPeriod` | 关闭会计期间 |

## 查询

| 查询 | 描述 |
|------|------|
| `GetJournalEntry` | 获取凭证详情 |
| `ListJournalEntries` | 分页列表查询 |
| `GetAccountBalance` | 科目余额查询 |
| `GetTrialBalance` | 试算平衡表 |

## 运行

```bash
# 开发模式
cargo run

# 生产模式（需要配置）
cargo run --features full

# 运行测试
cargo test
```
