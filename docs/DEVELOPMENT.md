# KILLER ERP 开发者手册

本文档为 KILLER ERP 项目的开发者指南，涵盖环境搭建、开发流程、测试策略和最佳实践。

---

## 1. 开发环境搭建

### 前置要求

| 工具 | 最低版本 | 用途 |
|------|----------|------|
| Rust | 1.85+ | 编译工具链 |
| Docker | 24.0+ | 容器化运行环境 |
| Buf CLI | 1.30+ | Proto 管理和代码生成 |
| SQLx CLI | 0.7+ | 数据库迁移 |
| just | 1.25+ | 任务运行器 (可选) |

### macOS 安装

```bash
# 安装 Homebrew (如未安装)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
rustup component add clippy rustfmt

# 安装 Docker Desktop
brew install --cask docker

# 安装开发工具
brew install bufbuild/buf/buf
brew install just
cargo install sqlx-cli --features postgres
cargo install cargo-watch cargo-nextest cargo-llvm-cov
```

### Linux (Ubuntu/Debian) 安装

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装 Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# 安装 Buf CLI
BUF_VERSION="1.30.0"
curl -sSL "https://github.com/bufbuild/buf/releases/download/v${BUF_VERSION}/buf-Linux-x86_64" -o /usr/local/bin/buf
chmod +x /usr/local/bin/buf

# 安装 SQLx CLI
sudo apt-get install -y libssl-dev pkg-config
cargo install sqlx-cli --features postgres
```

### Windows 安装

```powershell
# 安装 Rust (使用 rustup-init.exe)
winget install Rustlang.Rustup

# 安装 Docker Desktop
winget install Docker.DockerDesktop

# 安装 Buf CLI
scoop install buf

# 安装 SQLx CLI
cargo install sqlx-cli --features postgres
```

### IDE 配置

#### VS Code

安装推荐扩展：

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension tamasfe.even-better-toml
code --install-extension vadimcn.vscode-lldb
code --install-extension zxh404.vscode-proto3
```

创建 `.vscode/settings.json`：

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.procMacro.enable": true,
  "editor.formatOnSave": true
}
```

#### IntelliJ IDEA

1. 安装 Rust 插件：`Settings → Plugins → Marketplace → Rust`
2. 配置工具链：`Settings → Languages & Frameworks → Rust → Toolchain`
3. 启用 Clippy：`Settings → Languages & Frameworks → Rust → External Linters`

---

## 2. 项目结构导览

### 目录结构

```
killer/
├── libs/                    # 共享库
│   ├── common/              # 通用模块
│   │   ├── domain-primitives/   # 领域原语 (Money, Quantity)
│   │   ├── types/               # 公共类型定义
│   │   └── utils/               # 工具函数
│   ├── frameworks/          # 框架支持
│   │   ├── cqrs/                # CQRS 抽象
│   │   ├── event-sourcing/      # 事件溯源
│   │   └── saga-orchestration/  # Saga 编排
│   ├── infrastructure/      # 基础设施
│   │   ├── messaging/           # Kafka 客户端
│   │   ├── persistence/         # 数据库抽象
│   │   └── observability/       # 可观测性
│   └── master-data/         # 主数据客户端
├── services/                # 微服务
│   ├── finance/             # 财务域
│   ├── commercial/          # 商业域
│   └── ...
├── proto/                   # Protocol Buffers
├── infrastructure/          # 部署配置
└── docs/                    # 文档
```

### 共享库使用

在服务的 `Cargo.toml` 中引用共享库：

```toml
[dependencies]
domain-primitives = { path = "../../libs/common/domain-primitives" }
cqrs = { path = "../../libs/frameworks/cqrs" }
messaging = { path = "../../libs/infrastructure/messaging" }
```

> **Note**: 共享库通过 workspace 依赖管理，版本号由根 `Cargo.toml` 统一控制。

---

## 3. 本地开发流程

### 启动基础设施

```bash
# 进入项目根目录
cd killer

# 启动核心服务 (Postgres, Redis, Kafka)
docker compose -f infrastructure/docker/docker-compose.core.yml up -d

# 验证服务状态
docker compose -f infrastructure/docker/docker-compose.core.yml ps

# 查看日志
docker compose -f infrastructure/docker/docker-compose.core.yml logs -f kafka
```

### 运行单个微服务

```bash
# 设置环境变量
export DATABASE_URL="postgres://killer:killer_dev@localhost:5432/killer_financial"
export REDIS_URL="redis://localhost:6379"
export KAFKA_BROKERS="localhost:9092"
export RUST_LOG="info,financial_service=debug"

# 运行服务
cargo run -p financial-service

# 或使用 cargo-watch 热重载
cargo watch -x 'run -p financial-service'
```

### 运行测试

```bash
# 运行所有测试
cargo test --workspace

# 使用 nextest 加速测试
cargo nextest run

# 运行特定服务的测试
cargo test -p financial-service

# 运行集成测试
cargo test -p financial-service --test integration

# 生成覆盖率报告
cargo llvm-cov --workspace --html
open target/llvm-cov/html/index.html
```

### Proto 代码生成

```bash
# 进入 proto 目录
cd proto

# Lint 检查
buf lint

# 格式化
buf format -w

# 生成代码
buf generate

# 检测破坏性变更
buf breaking --against '.git#branch=main'
```

> **Warning**: 修改 Proto 文件后必须重新生成代码，否则编译将失败。

---

## 4. 数据库管理

### 创建数据库

```bash
# 为每个服务创建独立数据库
docker exec -it killer-postgres psql -U killer -c "CREATE DATABASE killer_financial;"
docker exec -it killer-postgres psql -U killer -c "CREATE DATABASE killer_sales;"
```

### 迁移脚本管理

```bash
# 创建新迁移
cd services/finance/financial-service
sqlx migrate add create_journal_entries

# 编辑迁移文件
# migrations/20240101000000_create_journal_entries.sql
```

迁移文件命名规范：`<timestamp>_<description>.sql`

### 运行迁移

```bash
# 确保 DATABASE_URL 已设置
export DATABASE_URL="postgres://killer:killer_dev@localhost:5432/killer_financial"

# 运行所有待执行迁移
sqlx migrate run

# 查看迁移状态
sqlx migrate info

# 回滚最后一次迁移
sqlx migrate revert
```

### 离线模式准备

```bash
# 生成编译时查询校验数据
cargo sqlx prepare --workspace

# 提交 .sqlx 目录到版本控制
git add .sqlx
```

> **Note**: CI 环境使用离线模式编译，无需连接真实数据库。

---

## 5. 服务开发指南

### 创建新服务

```bash
# 1. 创建目录结构
mkdir -p services/<domain>/<service-name>/src/{api,application,domain,infrastructure}
cd services/<domain>/<service-name>

# 2. 初始化 Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "<service-name>"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[dependencies]
# 工作区依赖
tokio.workspace = true
axum.workspace = true
tonic.workspace = true
sqlx.workspace = true
tracing.workspace = true

# 共享库
domain-primitives = { path = "../../../libs/common/domain-primitives" }
cqrs = { path = "../../../libs/frameworks/cqrs" }
EOF

# 3. 创建入口文件
touch src/main.rs src/lib.rs
```

### DDD 四层职责

| 层 | 目录 | 职责 | 依赖规则 |
|----|------|------|----------|
| API | `src/api/` | HTTP/gRPC 端点定义、输入验证 | → Application |
| Application | `src/application/` | 用例编排、命令/查询处理 | → Domain, Infrastructure |
| Domain | `src/domain/` | 聚合、实体、值对象、领域事件 | 无外部依赖 |
| Infrastructure | `src/infrastructure/` | 仓储实现、外部服务集成 | → Domain (接口) |

### 聚合根设计原则

1. **一致性边界**：聚合内保证强一致性，聚合间通过事件实现最终一致性
2. **小聚合**：优先设计小聚合，避免加载过多数据
3. **通过 ID 引用**：聚合间通过 ID 引用，不直接持有对象引用
4. **单一聚合根**：每个聚合仅有一个入口点

### 领域事件发布

```bash
# 事件通过 Kafka 发布
# Topic 命名: <domain>.<aggregate>.<event>
# 示例: finance.journal-entry.posted
```

---

## 6. 测试策略

### 测试金字塔

| 类型 | 占比 | 范围 | 运行速度 |
|------|------|------|----------|
| 单元测试 | 70% | 领域逻辑 | 毫秒级 |
| 集成测试 | 20% | API + DB | 秒级 |
| E2E 测试 | 10% | 多服务协作 | 分钟级 |

### 单元测试

```bash
# 运行领域层单元测试
cargo test -p financial-service --lib domain::

# 仅运行文档测试
cargo test -p financial-service --doc
```

### 集成测试

```bash
# 使用 Testcontainers 自动启动依赖
cargo test -p financial-service --test integration -- --test-threads=1

# 指定测试数据库
TEST_DATABASE_URL="postgres://test:test@localhost:5433/test" cargo test
```

### 端到端测试

```bash
# 启动完整环境
docker compose -f infrastructure/docker/docker-compose.full.yml up -d

# 运行 E2E 测试
cargo test -p e2e-tests

# 清理环境
docker compose -f infrastructure/docker/docker-compose.full.yml down -v
```

> **Note**: E2E 测试应在 CI 的独立阶段运行，避免影响快速反馈。

---

## 7. 调试技巧

### 日志配置

```bash
# 设置日志级别
export RUST_LOG="warn,financial_service=debug,sqlx=info"

# 启用完整 backtrace
export RUST_BACKTRACE=1

# JSON 格式日志 (生产环境)
export LOG_FORMAT="json"
```

日志级别优先级：`error > warn > info > debug > trace`

### 分布式追踪

```bash
# 启动 Jaeger
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  jaegertracing/all-in-one:latest

# 配置 OTLP 端点
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"

# 访问 Jaeger UI
open http://localhost:16686
```

### 性能分析

```bash
# 安装 flamegraph
cargo install flamegraph

# 生成火焰图
cargo flamegraph -p financial-service --bin financial-service

# 使用 perf 分析 (Linux)
perf record -g target/release/financial-service
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

---

## 8. 故障排查

### FAQ

#### Q1: 编译时报 `DATABASE_URL must be set`

```bash
# 解决方案 1: 设置环境变量
export DATABASE_URL="postgres://killer:killer_dev@localhost:5432/killer"

# 解决方案 2: 使用离线模式
cargo sqlx prepare --workspace
```

#### Q2: Docker Compose 启动后服务无法连接

```bash
# 检查网络
docker network ls
docker network inspect killer-network

# 检查容器 IP
docker inspect killer-postgres | grep IPAddress

# 从宿主机连接使用 localhost，从容器内使用服务名
```

#### Q3: Kafka 消费者无法接收消息

```bash
# 检查 Topic 是否存在
docker exec -it killer-kafka kafka-topics.sh --bootstrap-server localhost:9092 --list

# 创建 Topic
docker exec -it killer-kafka kafka-topics.sh --bootstrap-server localhost:9092 \
  --create --topic finance.journal-entry.posted --partitions 3

# 查看消费者组
docker exec -it killer-kafka kafka-consumer-groups.sh --bootstrap-server localhost:9092 --list
```

#### Q4: gRPC 调用返回 `UNAVAILABLE` 错误

```bash
# 检查服务是否启动
grpcurl -plaintext localhost:50051 list

# 检查健康状态
grpcurl -plaintext localhost:50051 grpc.health.v1.Health/Check

# 增加超时时间
export GRPC_TIMEOUT="30s"
```

#### Q5: SQLx 迁移失败 `relation already exists`

```bash
# 检查迁移历史
sqlx migrate info

# 手动标记迁移完成 (谨慎使用)
psql $DATABASE_URL -c "INSERT INTO _sqlx_migrations (version, description, success) VALUES (20240101000000, 'create_tables', true);"

# 重置数据库 (开发环境)
sqlx database drop && sqlx database create && sqlx migrate run
```

> **Warning**: 生产环境禁止使用 `database drop`，迁移问题需人工审核解决。

---

## 9. 代码规范

### 命名约定

| 类型 | 风格 | 示例 |
|------|------|------|
| 模块 | snake_case | `journal_entry`, `sales_order` |
| 类型/结构体 | PascalCase | `JournalEntry`, `SalesOrder` |
| 函数/方法 | snake_case | `create_entry`, `calculate_total` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_LINE_ITEMS`, `DEFAULT_CURRENCY` |
| 枚举变体 | PascalCase | `Status::Pending`, `Status::Completed` |

### 错误处理模式

| 场景 | 库 | 用途 |
|------|-----|------|
| 库/共享代码 | `thiserror` | 定义明确的错误类型 |
| 应用层代码 | `anyhow` | 快速传播错误，附加上下文 |
| API 层 | 自定义 | 转换为 HTTP/gRPC 状态码 |

```bash
# 错误信息应包含上下文
# ✗ "not found"
# ✓ "journal entry not found: id=JE-2024-0001"
```

### 日志记录规范

```bash
# Span 命名: <module>.<operation>
# 示例: financial.create_journal_entry

# 必须记录的字段:
# - request_id: 请求唯一标识
# - trace_id: 分布式追踪 ID
# - user_id: 操作用户 (如有)

# 日志级别选择:
# - ERROR: 需要人工干预的错误
# - WARN: 异常但可自动恢复
# - INFO: 关键业务操作
# - DEBUG: 调试信息
# - TRACE: 详细执行路径
```

---

## 附录：常用命令速查

```bash
# 构建
cargo build --workspace --release

# 检查
cargo clippy --workspace -- -D warnings

# 格式化
cargo fmt --all

# 依赖审计
cargo audit

# 更新依赖
cargo update

# 清理构建产物
cargo clean

# 查看依赖树
cargo tree -p financial-service
```
