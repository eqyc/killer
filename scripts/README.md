# 脚本 (scripts)

构建、部署与开发脚本。

## 目录结构

```
scripts/
├── init-project.sh      # 项目初始化脚本
├── verify-setup.sh      # 项目设置验证脚本
├── build/               # 构建脚本
├── deploy/              # 部署脚本
└── dev/                 # 开发环境脚本
```

## 快速开始

### 项目初始化

首次克隆项目后，运行初始化脚本：

```bash
# 添加执行权限
chmod +x scripts/init-project.sh

# 运行初始化
./scripts/init-project.sh
```

### 验证项目设置

检查项目配置是否正确：

```bash
chmod +x scripts/verify-setup.sh
./scripts/verify-setup.sh
```

## 脚本详细说明

### init-project.sh

项目初始化脚本，用于设置开发环境。

#### 功能

1. **环境检查** - 验证必要工具是否安装
   - Rust (>= 1.92)
   - Docker
   - Buf CLI
   - SQLx CLI
   - Make

2. **目录结构验证** - 检查项目结构完整性

3. **依赖安装** - 安装开发工具
   - sqlx-cli
   - cargo-watch
   - cargo-audit
   - Buf CLI

4. **配置文件初始化** - 设置环境变量
   - 复制 `.env.example` 到 `.env`
   - 验证 TOML 配置文件

5. **Git 初始化** - 设置版本控制
   - 初始化仓库
   - 创建 `.gitignore`
   - 执行首次提交

6. **基础设施启动** - 启动 Docker 服务
   - PostgreSQL
   - Redis
   - Kafka
   - MinIO

7. **数据库初始化** - 运行迁移脚本

8. **验证** - 检查项目是否可以编译

#### 用法

```bash
./scripts/init-project.sh [选项]

选项:
  --skip-docker    跳过 Docker 服务启动
  --skip-db        跳过数据库迁移
  --seed-data      填充测试数据
  -h, --help       显示帮助信息
```

#### 示例

```bash
# 完整初始化
./scripts/init-project.sh

# 跳过 Docker（已有外部数据库）
./scripts/init-project.sh --skip-docker

# 初始化并填充测试数据
./scripts/init-project.sh --seed-data

# 仅初始化配置，不启动服务
./scripts/init-project.sh --skip-docker --skip-db
```

#### 日志

初始化日志保存在 `logs/init.log`。

---

### verify-setup.sh

项目设置验证脚本，用于检查开发环境是否正确配置。

#### 功能

1. **工具链检查** - 验证开发工具版本
2. **项目结构检查** - 验证目录和文件
3. **配置文件检查** - 验证环境变量配置
4. **Git 仓库检查** - 验证版本控制状态
5. **Docker 服务检查** - 验证容器运行状态
6. **数据库连接检查** - 验证数据库可访问性
7. **编译检查** - 运行 `cargo check` 和 `cargo clippy`
8. **Proto 文件检查** - 运行 `buf lint`

#### 用法

```bash
./scripts/verify-setup.sh [选项]

选项:
  -v, --verbose    显示详细信息
  --fix            尝试自动修复问题
  -h, --help       显示帮助信息
```

#### 示例

```bash
# 基本验证
./scripts/verify-setup.sh

# 详细输出
./scripts/verify-setup.sh --verbose

# 自动修复问题
./scripts/verify-setup.sh --fix
```

#### 退出码

- `0` - 所有检查通过
- `1` - 存在失败的检查项

---

## 环境要求

### 必需工具

| 工具 | 最低版本 | 安装方式 |
|------|----------|----------|
| Rust | 1.92 | [rustup.rs](https://rustup.rs) |
| Docker | 20.0 | [docker.com](https://docs.docker.com/get-docker/) |
| Make | 3.0 | 系统包管理器 |

### 推荐工具

| 工具 | 用途 | 安装方式 |
|------|------|----------|
| Buf CLI | Protocol Buffers | `brew install bufbuild/buf/buf` |
| SQLx CLI | 数据库迁移 | `cargo install sqlx-cli` |
| cargo-watch | 热重载 | `cargo install cargo-watch` |
| cargo-audit | 安全审计 | `cargo install cargo-audit` |

---

## 常见问题

### Q: 初始化脚本卡在 Docker 服务启动

**A:** 确保 Docker Desktop 已启动。在 macOS 上，检查菜单栏的 Docker 图标。

```bash
# 检查 Docker 状态
docker info

# 如果未运行，启动 Docker Desktop
open -a Docker
```

### Q: Rust 版本过低

**A:** 使用 rustup 更新 Rust：

```bash
rustup update
rustup default stable
```

### Q: 数据库连接失败

**A:** 检查 Docker 容器状态和端口：

```bash
# 查看容器状态
docker ps

# 检查端口占用
lsof -i :5432
```

### Q: cargo check 失败

**A:** 可能是依赖问题，尝试清理并重新构建：

```bash
cargo clean
cargo update
cargo check
```

### Q: 权限被拒绝

**A:** 添加脚本执行权限：

```bash
chmod +x scripts/*.sh
```

---

## 开发工作流

### 日常开发

```bash
# 1. 启动基础设施
make docker-up

# 2. 启动开发服务器（热重载）
make dev

# 3. 运行测试
make test

# 4. 提交前检查
make lint
make fmt
```

### 数据库操作

```bash
# 创建新迁移
sqlx migrate add <migration_name>

# 运行迁移
make db-migrate

# 回滚迁移
sqlx migrate revert
```

### Proto 文件

```bash
# 生成代码
make proto

# 检查格式
buf lint

# 格式化
buf format -w
```

---

## 相关文档

- [项目 README](../README.md)
- [基础设施文档](../infrastructure/README.md)
- [开��指南](../docs/development.md)
