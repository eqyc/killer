# ============================================================================
# KILLER ERP - Makefile
# 项目自动化任务管理
# ============================================================================

# ----------------------------------------------------------------------------
# 变量定义
# ----------------------------------------------------------------------------

# 项目路径
PROJECT_ROOT := $(shell pwd)
SERVICES_DIR := $(PROJECT_ROOT)/services
PROTO_DIR := $(PROJECT_ROOT)/proto
INFRA_DIR := $(PROJECT_ROOT)/infrastructure

# Docker Compose 文件
COMPOSE_FILE := $(INFRA_DIR)/docker/docker-compose.yml
COMPOSE_DEV := $(INFRA_DIR)/docker/docker-compose.dev.yml

# 容器注册表
REGISTRY ?= ghcr.io/killer
IMAGE_TAG ?= latest

# Cargo 命令
CARGO := cargo
CARGO_FLAGS := --workspace

# 服务列表
SERVICES := api-gateway financial-service sales-service purchasing-service \
            materials-service warehouse-service production-service hr-service

# 颜色定义（用于 help 输出）
BLUE := \033[34m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
CYAN := \033[36m
BOLD := \033[1m
RESET := \033[0m

# ----------------------------------------------------------------------------
# 默认任务
# ----------------------------------------------------------------------------

.DEFAULT_GOAL := help

# ----------------------------------------------------------------------------
# PHONY 声明
# ----------------------------------------------------------------------------

.PHONY: help \
        dev dev-full dev-down dev-clean dev-logs dev-ps \
        db-migrate db-reset db-seed db-shell \
        build build-release check fmt lint test test-unit test-integration test-coverage \
        proto-gen proto-lint proto-format proto-breaking \
        run-gateway run-financial run-sales run-purchasing run-materials \
        docker-build docker-push docker-ps docker-clean \
        logs-gateway logs-financial logs-sales \
        grafana prometheus jaeger \
        install-tools clean version

# ============================================================================
# 开发环境管理
# ============================================================================

## dev: 启动核心开发环境（PostgreSQL, Redis, Kafka, ClickHouse）
dev:
	@echo "$(GREEN)▶ 启动开发环境...$(RESET)"
	docker compose -f $(COMPOSE_FILE) up -d postgres redis kafka clickhouse
	@echo "$(GREEN)✓ 开发环境已启动$(RESET)"
	@echo "  PostgreSQL: localhost:5432"
	@echo "  Redis:      localhost:6379"
	@echo "  Kafka:      localhost:9092"
	@echo "  ClickHouse: localhost:8123"

## dev-full: 启动完整开发环境（包含所有基础设施服务）
dev-full:
	@echo "$(GREEN)▶ 启动完整开发环境...$(RESET)"
	docker compose -f $(COMPOSE_FILE) -f $(COMPOSE_DEV) up -d
	@echo "$(GREEN)✓ 完整开发环境已启动$(RESET)"

## dev-down: 停止开发环境
dev-down:
	@echo "$(YELLOW)▶ 停止开发环境...$(RESET)"
	docker compose -f $(COMPOSE_FILE) down || true
	@echo "$(GREEN)✓ 开发环境已停止$(RESET)"

## dev-clean: 清理开发环境（删除容器、卷和网络）
dev-clean:
	@echo "$(RED)▶ 清理开发环境...$(RESET)"
	docker compose -f $(COMPOSE_FILE) down -v --remove-orphans || true
	docker network prune -f || true
	@echo "$(GREEN)✓ 开发环境已清理$(RESET)"

## dev-logs: 查看所有容器日志
dev-logs:
	docker compose -f $(COMPOSE_FILE) logs -f

## dev-ps: 查看运行中的容器状态
dev-ps:
	docker compose -f $(COMPOSE_FILE) ps

# ============================================================================
# 数据库管理
# ============================================================================

## db-migrate: 运行所有服务的数据库迁移
db-migrate:
	@echo "$(GREEN)▶ 运行数据库迁移...$(RESET)"
	@for service in $(SERVICES); do \
		echo "  迁移: $$service"; \
		$(CARGO) run -p killer-cli -- db migrate --service $$service || true; \
	done
	@echo "$(GREEN)✓ 数据库迁移完成$(RESET)"

## db-reset: 重置数据库（删除并重新创建所有表）
db-reset:
	@echo "$(RED)▶ 重置数据库...$(RESET)"
	@read -p "确认要重置数据库吗？[y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	$(CARGO) run -p killer-cli -- db reset --force
	@$(MAKE) db-migrate
	@echo "$(GREEN)✓ 数据库已重置$(RESET)"

## db-seed: 填充测试数据
db-seed:
	@echo "$(GREEN)▶ 填充测试数据...$(RESET)"
	$(CARGO) run -p killer-cli -- db seed --env development
	@echo "$(GREEN)✓ 测试数据已填充$(RESET)"

## db-shell: 进入 PostgreSQL 交互式终端
db-shell:
	@echo "$(CYAN)▶ 连接到 PostgreSQL...$(RESET)"
	docker compose -f $(COMPOSE_FILE) exec postgres psql -U killer -d killer_dev

# ============================================================================
# 代码构建与检查
# ============================================================================

## build: 编译所有服务（debug 模式）
build:
	@echo "$(GREEN)▶ 编译项目 (debug)...$(RESET)"
	$(CARGO) build $(CARGO_FLAGS)
	@echo "$(GREEN)✓ 编译完成$(RESET)"

## build-release: 编译所有服务（release 模式，启用优化）
build-release:
	@echo "$(GREEN)▶ 编译项目 (release)...$(RESET)"
	$(CARGO) build $(CARGO_FLAGS) --release
	@echo "$(GREEN)✓ Release 编译完成$(RESET)"

## check: 快速语法检查（不生成代码）
check:
	@echo "$(GREEN)▶ 检查代码...$(RESET)"
	$(CARGO) check $(CARGO_FLAGS)
	@echo "$(GREEN)✓ 检查通过$(RESET)"

## fmt: 格式化代码
fmt:
	@echo "$(GREEN)▶ 格式化代码...$(RESET)"
	$(CARGO) fmt --all
	@echo "$(GREEN)✓ 格式化完成$(RESET)"

## fmt-check: 检查代码格式（CI 用）
fmt-check:
	@echo "$(GREEN)▶ 检查代码格式...$(RESET)"
	$(CARGO) fmt --all -- --check
	@echo "$(GREEN)✓ 格式检查通过$(RESET)"

## lint: 运行 Clippy 代码检查
lint:
	@echo "$(GREEN)▶ 运行 Clippy...$(RESET)"
	$(CARGO) clippy $(CARGO_FLAGS) -- -D warnings
	@echo "$(GREEN)✓ Lint 检查通过$(RESET)"

## test: 运行所有测试
test:
	@echo "$(GREEN)▶ 运行测试...$(RESET)"
	$(CARGO) test $(CARGO_FLAGS)
	@echo "$(GREEN)✓ 测试完成$(RESET)"

## test-unit: 运行单元测试
test-unit:
	@echo "$(GREEN)▶ 运行单元测试...$(RESET)"
	$(CARGO) test $(CARGO_FLAGS) --lib
	@echo "$(GREEN)✓ 单元测试完成$(RESET)"

## test-integration: 运行集成测试（需要运行 dev 环境）
test-integration:
	@echo "$(GREEN)▶ 运行集成测试...$(RESET)"
	$(CARGO) test $(CARGO_FLAGS) --test '*' -- --test-threads=1
	@echo "$(GREEN)✓ 集成测试完成$(RESET)"

## test-coverage: 生成测试覆盖率报告
test-coverage:
	@echo "$(GREEN)▶ 生成测试覆盖率报告...$(RESET)"
	$(CARGO) llvm-cov --workspace --html
	@echo "$(GREEN)✓ 覆盖率报告已生成: target/llvm-cov/html/index.html$(RESET)"

# ============================================================================
# Proto 管理
# ============================================================================

## proto-gen: 生成 Protocol Buffers 代码
proto-gen:
	@echo "$(GREEN)▶ 生成 Proto 代码...$(RESET)"
	cd $(PROTO_DIR) && buf generate
	@echo "$(GREEN)✓ Proto 代码生成完成$(RESET)"

## proto-lint: 检查 Proto 文件规范
proto-lint:
	@echo "$(GREEN)▶ 检查 Proto 规范...$(RESET)"
	cd $(PROTO_DIR) && buf lint
	@echo "$(GREEN)✓ Proto 检查通过$(RESET)"

## proto-format: 格式化 Proto 文件
proto-format:
	@echo "$(GREEN)▶ 格式化 Proto 文件...$(RESET)"
	cd $(PROTO_DIR) && buf format -w
	@echo "$(GREEN)✓ Proto 格式化完成$(RESET)"

## proto-breaking: 检查 Proto 向后兼容性
proto-breaking:
	@echo "$(GREEN)▶ 检查 Proto 兼容性...$(RESET)"
	cd $(PROTO_DIR) && buf breaking --against '.git#branch=main'
	@echo "$(GREEN)✓ Proto 兼容性检查通过$(RESET)"

# ============================================================================
# 服务运行
# ============================================================================

## run-gateway: 启动 API Gateway 服务
run-gateway:
	@echo "$(GREEN)▶ 启动 API Gateway...$(RESET)"
	$(CARGO) run -p api-gateway

## run-financial: 启动 Financial Service
run-financial:
	@echo "$(GREEN)▶ 启动 Financial Service...$(RESET)"
	$(CARGO) run -p financial-service

## run-sales: 启动 Sales Service
run-sales:
	@echo "$(GREEN)▶ 启动 Sales Service...$(RESET)"
	$(CARGO) run -p sales-service

## run-purchasing: 启动 Purchasing Service
run-purchasing:
	@echo "$(GREEN)▶ 启动 Purchasing Service...$(RESET)"
	$(CARGO) run -p purchasing-service

## run-materials: 启动 Materials Service
run-materials:
	@echo "$(GREEN)▶ 启动 Materials Service...$(RESET)"
	$(CARGO) run -p materials-service

# ============================================================================
# Docker 管理
# ============================================================================

## docker-build: 构建所有服务的 Docker 镜像
docker-build:
	@echo "$(GREEN)▶ 构建 Docker 镜像...$(RESET)"
	@for service in $(SERVICES); do \
		echo "  构建: $(REGISTRY)/$$service:$(IMAGE_TAG)"; \
		docker build -t $(REGISTRY)/$$service:$(IMAGE_TAG) \
			-f $(INFRA_DIR)/docker/Dockerfile.$$service . || exit 1; \
	done
	@echo "$(GREEN)✓ Docker 镜像构建完成$(RESET)"

## docker-push: 推送所有镜像到 Registry
docker-push:
	@echo "$(GREEN)▶ 推送 Docker 镜像...$(RESET)"
	@for service in $(SERVICES); do \
		echo "  推送: $(REGISTRY)/$$service:$(IMAGE_TAG)"; \
		docker push $(REGISTRY)/$$service:$(IMAGE_TAG) || exit 1; \
	done
	@echo "$(GREEN)✓ Docker 镜像推送完成$(RESET)"

## docker-ps: 查看运行中的容器
docker-ps:
	docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

## docker-clean: 清理未使用的 Docker 资源
docker-clean:
	@echo "$(YELLOW)▶ 清理 Docker 资源...$(RESET)"
	docker system prune -f
	docker volume prune -f
	@echo "$(GREEN)✓ Docker 资源已清理$(RESET)"

# ============================================================================
# 监控与日志
# ============================================================================

## logs-gateway: 查看 API Gateway 日志
logs-gateway:
	docker compose -f $(COMPOSE_FILE) logs -f api-gateway

## logs-financial: 查看 Financial Service 日志
logs-financial:
	docker compose -f $(COMPOSE_FILE) logs -f financial-service

## logs-sales: 查看 Sales Service 日志
logs-sales:
	docker compose -f $(COMPOSE_FILE) logs -f sales-service

## grafana: 打开 Grafana 仪表盘
grafana:
	@echo "$(CYAN)▶ 打开 Grafana: http://localhost:3000$(RESET)"
	@open http://localhost:3000 2>/dev/null || xdg-open http://localhost:3000 2>/dev/null || echo "请手动打开浏览器访问"

## prometheus: 打开 Prometheus UI
prometheus:
	@echo "$(CYAN)▶ 打开 Prometheus: http://localhost:9090$(RESET)"
	@open http://localhost:9090 2>/dev/null || xdg-open http://localhost:9090 2>/dev/null || echo "请手动打开浏览器访问"

## jaeger: 打开 Jaeger UI（分布式追踪）
jaeger:
	@echo "$(CYAN)▶ 打开 Jaeger: http://localhost:16686$(RESET)"
	@open http://localhost:16686 2>/dev/null || xdg-open http://localhost:16686 2>/dev/null || echo "请手动打开浏览器访问"

# ============================================================================
# 工具与实用功能
# ============================================================================

## install-tools: 安装开发工具
install-tools:
	@echo "$(GREEN)▶ 安装���发工具...$(RESET)"
	@echo "  安装 sqlx-cli..."
	cargo install sqlx-cli --no-default-features --features rustls,postgres
	@echo "  安装 cargo-watch..."
	cargo install cargo-watch
	@echo "  安装 cargo-llvm-cov..."
	cargo install cargo-llvm-cov
	@echo "  安装 buf..."
	@which buf > /dev/null || (echo "请手动安装 buf: https://buf.build/docs/installation" && exit 1)
	@echo "$(GREEN)✓ 开发工具安装完成$(RESET)"

## clean: 清理构建产物
clean:
	@echo "$(YELLOW)▶ 清理构建产物...$(RESET)"
	$(CARGO) clean
	rm -rf target/
	@echo "$(GREEN)✓ 构建产物已清理$(RESET)"

## version: 显示版本信息
version:
	@echo "$(CYAN)KILLER ERP$(RESET)"
	@echo "  Rust:    $$(rustc --version)"
	@echo "  Cargo:   $$(cargo --version)"
	@echo "  Docker:  $$(docker --version)"
	@echo "  Buf:     $$(buf --version 2>/dev/null || echo 'not installed')"

# ============================================================================
# 帮助信息
# ============================================================================

## help: 显示所有可用命令
help:
	@echo ""
	@echo "$(BOLD)$(CYAN)KILLER ERP - 可用命令$(RESET)"
	@echo ""
	@echo "$(BOLD)$(YELLOW)开发环境:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev" "启动核心开发环境"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev-full" "启动完整开发环境"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev-down" "停止开发环境"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev-clean" "清理开发环境"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev-logs" "查看容器日志"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "dev-ps" "查看容器状态"
	@echo ""
	@echo "$(BOLD)$(YELLOW)数据库:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "db-migrate" "运行数据库迁移"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "db-reset" "重置数据库"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "db-seed" "填充测试数据"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "db-shell" "进入 PostgreSQL 终端"
	@echo ""
	@echo "$(BOLD)$(YELLOW)构建与测试:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "build" "编译项目 (debug)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "build-release" "编译项目 (release)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "check" "快速语法检查"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "fmt" "格式化代码"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "lint" "运行 Clippy"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "test" "运行所有测试"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "test-unit" "运行单元测试"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "test-integration" "运行集成测试"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "test-coverage" "生成覆盖率报告"
	@echo ""
	@echo "$(BOLD)$(YELLOW)Proto:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "proto-gen" "生成 Proto 代码"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "proto-lint" "检查 Proto 规范"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "proto-format" "格式化 Proto 文件"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "proto-breaking" "检查兼容性"
	@echo ""
	@echo "$(BOLD)$(YELLOW)服务运行:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "run-gateway" "启动 API Gateway"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "run-financial" "启动 Financial Service"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "run-sales" "启动 Sales Service"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "run-purchasing" "启动 Purchasing Service"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "run-materials" "启动 Materials Service"
	@echo ""
	@echo "$(BOLD)$(YELLOW)Docker:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "docker-build" "构建 Docker 镜像"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "docker-push" "推送镜像到 Registry"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "docker-ps" "查看运行中的容器"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "docker-clean" "清理 Docker 资源"
	@echo ""
	@echo "$(BOLD)$(YELLOW)监控:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "grafana" "打开 Grafana 仪表盘"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "prometheus" "打开 Prometheus UI"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "jaeger" "打开 Jaeger UI"
	@echo ""
	@echo "$(BOLD)$(YELLOW)工具:$(RESET)"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "install-tools" "安装开发工具"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "clean" "清理构建产物"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "version" "显示版本信息"
	@printf "  $(GREEN)%-20s$(RESET) %s\n" "help" "显示此帮助信息"
	@echo ""
	@echo "$(BOLD)示例:$(RESET)"
	@echo "  $$ make dev          # 启动开发环境"
	@echo "  $$ make build test   # 编译并运行测试"
	@echo "  $$ make fmt lint     # 格式化并检查代码"
	@echo ""
