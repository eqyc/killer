# ============================================================
# ERP 系统 Makefile
# ============================================================

.PHONY: help
.DEFAULT_GOAL := help

# 颜色定义
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m

# ============================================================
# 帮助信息
# ============================================================

help: ## 显示此帮助信息
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"
	@echo "$(BLUE)  ERP 系统开发工具$(NC)"
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"
	@echo ""
	@echo "$(YELLOW)使用方法:$(NC) make <目标>"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"

# ============================================================
# Docker 服务管理
# ============================================================

.PHONY: docker-start docker-stop docker-restart docker-clean docker-status docker-logs

docker-start: ## 启动 Docker 服务
	@./scripts/docker-start.sh

docker-start-all: ## 启动所有 Docker 服务（包括监控）
	@./scripts/docker-start.sh all

docker-stop: ## 停止 Docker 服务
	@./scripts/docker-manage.sh stop

docker-restart: ## 重启 Docker 服务
	@./scripts/docker-manage.sh restart

docker-clean: ## 清理 Docker 服务和数据
	@./scripts/docker-manage.sh clean

docker-status: ## 查看 Docker 服务状态
	@./scripts/docker-manage.sh status

docker-logs: ## 查看 Docker 日志
	@./scripts/docker-manage.sh logs

docker-ps: ## 查看运行中的容器
	@docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# ============================================================
# 开发环境
# ============================================================

.PHONY: dev dev-all setup env

dev: docker-start ## 启动开发环境（Docker + 服务）
	@echo "$(GREEN)✓ 开发环境已启动$(NC)"
	@echo "$(YELLOW)💡 提示: Kafka 暂时不可用，其他服务正常运行$(NC)"

dev-all: docker-start-all ## 启动完整开发环境（包括监控）
	@echo "$(GREEN)✓ 完整开发环境已启动$(NC)"

setup: ## 初始化项目（首次使用）
	@echo "$(BLUE)正在初始化项目...$(NC)"
	@cp -n .env.example .env || true
	@./scripts/docker-start.sh
	@echo "$(GREEN)✓ 项目初始化完成$(NC)"
	@echo ""
	@echo "$(YELLOW)下一步:$(NC)"
	@echo "  1. 编辑 .env 文件配置环境变量"
	@echo "  2. 运行 'make migrate' 初始化数据库"
	@echo "  3. 运行 'make test' 验证环境"

env: ## 创建环境配置文件
	@cp .env.example .env
	@echo "$(GREEN)✓ 已创建 .env 文件$(NC)"
	@echo "$(YELLOW)请编辑 .env 文件配置环境变量$(NC)"

# ============================================================
# Rust 开发
# ============================================================

.PHONY: build build-release test test-all lint fmt check clean-rust

build: ## 构建项目（Debug）
	@cargo build

build-release: ## 构建项目（Release）
	@cargo build --release

test: ## 运行测试
	@cargo test

test-all: ## 运行所有测试（包括集成测试）
	@cargo test --workspace --all-features

lint: ## 运行 Clippy 检查
	@cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt: ## 格式化代码
	@cargo fmt --all

fmt-check: ## 检查代码格式
	@cargo fmt --all -- --check

check: ## 快速检查（不生成代码）
	@cargo check --workspace

clean-rust: ## 清理 Rust 构建产物
	@cargo clean

# ============================================================
# 数据库管理
# ============================================================

.PHONY: db-migrate db-shell db-backup db-restore db-reset

db-migrate: ## 运行数据库迁移
	@echo "$(YELLOW)运行数据库迁移...$(NC)"
	@cargo run --bin data-migration
	@echo "$(GREEN)✓ 迁移完成$(NC)"

db-shell: ## 进入 PostgreSQL Shell
	@./scripts/docker-manage.sh shell postgres

db-backup: ## 备份数据库
	@./scripts/docker-manage.sh backup

db-restore: ## 恢复数据库（需要指定文件: make db-restore FILE=xxx.sql）
	@./scripts/docker-manage.sh restore $(FILE)

db-reset: ## 重置数据库（危险！）
	@./scripts/docker-manage.sh reset

# ============================================================
# CI/CD
# ============================================================

.PHONY: ci ci-local verify security-scan

ci: lint test ## 运行 CI 检查（本地）
	@echo "$(GREEN)✓ CI 检查通过$(NC)"

ci-local: ## 完整 CI 流程（本地）
	@echo "$(BLUE)运行完整 CI 流程...$(NC)"
	@make fmt-check
	@make lint
	@make test-all
	@make build-release
	@echo "$(GREEN)✓ 完整 CI 检查通过$(NC)"

verify: ## 验证版本配置
	@./scripts/upgrade-verification.sh

security-scan: ## 运行安全扫描
	@cargo audit
	@cargo deny check

# ============================================================
# 代码质量
# ============================================================

.PHONY: coverage doc bench

coverage: ## 生成代码覆盖率报告
	@cargo tarpaulin --workspace --out Html --output-dir coverage

doc: ## 生成并打开文档
	@cargo doc --workspace --no-deps --open

bench: ## 运行基准测试
	@cargo bench

# ============================================================
# 清理
# ============================================================

.PHONY: clean clean-all

clean: clean-rust ## 清理构建产物
	@echo "$(GREEN)✓ 清理完成$(NC)"

clean-all: clean docker-clean ## 清理所有（包括 Docker）
	@rm -rf target/
	@rm -rf coverage/
	@echo "$(GREEN)✓ 完全清理完成$(NC)"

# ============================================================
# 监控和日志
# ============================================================

.PHONY: logs logs-postgres logs-redis logs-kafka grafana prometheus

logs: ## 查看所有日志
	@./scripts/docker-manage.sh logs -f

logs-postgres: ## 查看 PostgreSQL 日志
	@./scripts/docker-manage.sh logs postgres -f

logs-redis: ## 查看 Redis 日志
	@./scripts/docker-manage.sh logs redis -f

logs-kafka: ## 查看 Kafka 日志
	@./scripts/docker-manage.sh logs kafka -f

grafana: ## 打开 Grafana
	@open http://localhost:3000 || xdg-open http://localhost:3000

prometheus: ## 打开 Prometheus
	@open http://localhost:9090 || xdg-open http://localhost:9090

# ============================================================
# 服务运行
# ============================================================

.PHONY: run-gateway run-service run-all-services list-services

run-gateway: ## 运行 API Gateway
	@echo "$(BLUE)启动 API Gateway...$(NC)"
	@cargo run --bin api-gateway

run-service: ## 运行指定服务 (使用: make run-service SERVICE=crm-service)
	@if [ -z "$(SERVICE)" ]; then \
		echo "$(YELLOW)使用方法: make run-service SERVICE=服务名$(NC)"; \
		echo "$(YELLOW)可用服务:$(NC)"; \
		$(MAKE) list-services; \
	else \
		echo "$(BLUE)启动 $(SERVICE)...$(NC)"; \
		cargo run --bin $(SERVICE); \
	fi

list-services: ## 列出所有可用服务
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"
	@echo "$(GREEN)可用的服务:$(NC)"
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"
	@echo ""
	@echo "$(YELLOW)核心服务:$(NC)"
	@echo "  $(GREEN)api-gateway$(NC)         - API 网关（推荐首先启动）"
	@echo ""
	@echo "$(YELLOW)业务服务:$(NC)"
	@echo "  crm-service          - 客户关系管理"
	@echo "  sales-service        - 销售管理"
	@echo "  financial-service    - 财务管理"
	@echo "  hr-service           - 人力资源"
	@echo "  scm-service          - 供应链管理"
	@echo "  warehouse-service    - 仓库管理"
	@echo "  materials-service    - 物料管理"
	@echo "  production-service   - 生产管理"
	@echo "  quality-service      - 质量管理"
	@echo "  shipping-service     - 物流管理"
	@echo "  project-service      - 项目管理"
	@echo "  treasury-service     - 资金管理"
	@echo "  controlling-service  - 控制管理"
	@echo "  maintenance-service  - 维护管理"
	@echo "  mdg-service          - 主数据治理"
	@echo "  analytics-service    - 数据分析"
	@echo ""
	@echo "$(YELLOW)工具:$(NC)"
	@echo "  cli                  - 命令行工具"
	@echo "  data-migration       - 数据迁移"
	@echo "  proto-gen            - Protobuf 生成"
	@echo ""
	@echo "$(BLUE)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(NC)"
	@echo "$(YELLOW)示例:$(NC)"
	@echo "  make run-gateway                    # 启动 API Gateway"
	@echo "  make run-service SERVICE=crm-service  # 启动 CRM 服务"
	@echo "  cargo run --bin sales-service         # 直接运行销售服务"
	@echo ""

# ============================================================
# 实用工具
# ============================================================

.PHONY: watch install-tools stats migrate

watch: ## 监视文件变化并自动重新编译
	@cargo watch -x check -x test

migrate: ## 运行数据迁移
	@echo "$(BLUE)运行数据迁移...$(NC)"
	@cargo run --bin data-migration

install-tools: ## 安装开发工具
	@echo "$(BLUE)安装开发工具...$(NC)"
	@cargo install cargo-watch
	@cargo install cargo-audit
	@cargo install cargo-deny
	@cargo install cargo-tarpaulin
	@cargo install cargo-sbom
	@brew install gh || true
	@echo "$(GREEN)✓ 工具安装完成$(NC)"

stats: ## 显示项目统计
	@echo "$(BLUE)项目统计:$(NC)"
	@echo ""
	@echo "代码行数:"
	@find . -name "*.rs" -not -path "./target/*" | xargs wc -l | tail -1
	@echo ""
	@echo "服务数量:"
	@ls -1 services/ | wc -l
	@echo ""
	@echo "共享库数量:"
	@ls -1 shared/ | wc -l
	@echo ""
	@echo "Docker 镜像:"
	@docker images | grep erp || echo "无"

# ============================================================
# GitHub 配置
# ============================================================

.PHONY: gh-secrets gh-check

gh-secrets: ## 配置 GitHub Secrets
	@./scripts/setup-github-secrets.sh

gh-check: ## 检查 GitHub 配置
	@./scripts/check-github-config.sh

# ============================================================
# 快速命令别名
# ============================================================

.PHONY: up down restart status

up: docker-start ## 别名: 启动服务
down: docker-stop ## 别名: 停止服务
restart: docker-restart ## 别名: 重启服务
status: docker-status ## 别名: 查看状态
