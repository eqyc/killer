# Docker 配置

KILLER ERP 项目的 Docker 容器化配置。

## 文件说明

| 文件 | 用途 |
|------|------|
| `docker-compose.core.yml` | 核心基础设施服务 (Postgres, Redis, Kafka) |
| `docker-compose.full.yml` | 完整环境，包含所有微服务 |
| `docker-compose.minimal.yml` | 最小化环境，资源占用最低 |
| `docker-compose.observability.yml` | 可观测性栈 (Prometheus, Grafana, Loki, Tempo) |
| `Dockerfile.dev` | 开发环境镜像，支持热重载 |
| `Dockerfile.optimized` | 生产优化镜像，多阶段构建 |

## 快速开始

### 启动核心服务

```bash
# 启动 Postgres, Redis, Kafka
docker compose -f docker-compose.core.yml up -d

# 查看服务状态
docker compose -f docker-compose.core.yml ps
```

### 启动完整环境

```bash
# 启动所有服务
docker compose -f docker-compose.core.yml -f docker-compose.full.yml up -d

# 仅启动指定服务
docker compose -f docker-compose.core.yml -f docker-compose.full.yml up -d api-gateway financial-service
```

### 启动最小化环境

```bash
# 适用于资源受限环境
docker compose -f docker-compose.minimal.yml up -d
```

### 启动可观测性栈

```bash
# 添加监控服务
docker compose -f docker-compose.core.yml -f docker-compose.observability.yml up -d

# 访问 Grafana: http://localhost:3000
# 访问 Prometheus: http://localhost:9090
```

## 开发环境

### 使用开发镜像

```bash
# 构建开发镜像
docker build -f Dockerfile.dev -t killer-dev .

# 热重载开发
docker run -it --rm \
  -v $(pwd):/app \
  -v cargo-cache:/usr/local/cargo/registry \
  -p 8000:8000 \
  killer-dev \
  cargo watch -x 'run -p api-gateway'
```

### 运行测试

```bash
docker run -it --rm \
  -v $(pwd):/app \
  killer-dev \
  cargo nextest run
```

## 生产镜像

### 构建优化镜像

```bash
# 构建单个服务
docker build -f Dockerfile.optimized \
  --build-arg SERVICE_NAME=api-gateway \
  -t killer/api-gateway:latest .

# 构建所有服务 (使用脚本)
./scripts/build-all-images.sh
```

### 镜像大小对比

| 镜像类型 | 大小 | 说明 |
|----------|------|------|
| 开发镜像 | ~2GB | 包含完整工具链 |
| 生产镜像 | ~30-50MB | 仅包含二进制 |

## 环境变量

### 通用配置

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `RUST_LOG` | 日志级别 | `info` |
| `DATABASE_URL` | PostgreSQL 连接 | - |
| `REDIS_URL` | Redis 连接 | `redis://localhost:6379` |
| `KAFKA_BROKERS` | Kafka 地址 | `localhost:9092` |

### 服务端口

| 服务 | HTTP 端口 | gRPC 端口 |
|------|-----------|-----------|
| API Gateway | 8000 | 50051 |
| Financial Service | 8100 | 50052 |
| Sales Service | 8500 | 50053 |

## 常用命令

```bash
# 查看日志
docker compose logs -f api-gateway

# 进入容器
docker compose exec api-gateway sh

# 停止所有服务
docker compose down

# 停止并删除数据
docker compose down -v

# 重建镜像
docker compose build --no-cache

# 清理未使用的资源
docker system prune -a
```

## 故障排查

### 服务无法启动

1. 检查端口占用: `lsof -i :8000`
2. 检查依赖服务状态: `docker compose ps`
3. 查看服务日志: `docker compose logs <service>`

### 数据库连接失败

1. 确认 Postgres 已启动: `docker compose ps postgres`
2. 检查连接字符串: `docker compose exec postgres psql -U killer`

### 内存不足

1. 使用最小化环境: `docker-compose.minimal.yml`
2. 调整资源限制: 修改 `deploy.resources` 配置
