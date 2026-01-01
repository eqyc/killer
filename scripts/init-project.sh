#!/usr/bin/env bash
# =============================================================================
# KILLER ERP - 项目初始化脚本
# =============================================================================
# 用途: 初始化开发环境，检查依赖，启动基础设施
# 兼容: macOS, Linux
# 用法: ./scripts/init-project.sh [--skip-docker] [--skip-db] [--seed-data]
# =============================================================================

set -euo pipefail

# =============================================================================
# 全局变量
# =============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/logs"
LOG_FILE="$LOG_DIR/init.log"
REQUIRED_RUST_VERSION="1.92"
SKIP_DOCKER=false
SKIP_DB=false
SEED_DATA=false

# =============================================================================
# 颜色定义
# =============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# =============================================================================
# 工具函数
# =============================================================================

log() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $*" >> "$LOG_FILE"
}

print_header() {
    echo -e "\n${MAGENTA}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${MAGENTA}${BOLD}  $1${NC}"
    echo -e "${MAGENTA}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
    log "=== $1 ==="
}

print_step() {
    echo -e "${CYAN}▶${NC} $1"
    log "STEP: $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
    log "SUCCESS: $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    log "WARNING: $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    log "ERROR: $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
    log "INFO: $1"
}

ask_continue() {
    local prompt="${1:-是否继续?}"
    echo -e -n "${YELLOW}?${NC} ${prompt} [Y/n]: "
    read -r response
    case "$response" in
        [nN][oO]|[nN]) return 1 ;;
        *) return 0 ;;
    esac
}

command_exists() {
    command -v "$1" &> /dev/null
}

version_ge() {
    # 比较版本号 $1 >= $2
    printf '%s\n%s\n' "$2" "$1" | sort -V -C
}

# =============================================================================
# 解析命令行参数
# =============================================================================
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-docker) SKIP_DOCKER=true; shift ;;
            --skip-db) SKIP_DB=true; shift ;;
            --seed-data) SEED_DATA=true; shift ;;
            -h|--help) show_help; exit 0 ;;
            *) print_error "未知参数: $1"; show_help; exit 1 ;;
        esac
    done
}

show_help() {
    echo -e "${BOLD}用法:${NC} $0 [选项]"
    echo ""
    echo -e "${BOLD}选项:${NC}"
    echo "  --skip-docker    跳过 Docker 服务启动"
    echo "  --skip-db        跳过数据库迁移"
    echo "  --seed-data      填充测试数据"
    echo "  -h, --help       显示帮助信息"
}

# =============================================================================
# 初始化日志
# =============================================================================
init_logging() {
    mkdir -p "$LOG_DIR"
    echo "=== KILLER ERP 初始化日志 ===" > "$LOG_FILE"
    echo "开始时间: $(date)" >> "$LOG_FILE"
    echo "操作系统: $(uname -s) $(uname -r)" >> "$LOG_FILE"
    echo "" >> "$LOG_FILE"
}

# =============================================================================
# 1. 环境检查
# =============================================================================
check_environment() {
    print_header "1. 环境检查"
    local has_error=false

    # 检查 Rust
    print_step "检查 Rust 版本..."
    if command_exists rustc; then
        local rust_version
        rust_version=$(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        if version_ge "$rust_version" "$REQUIRED_RUST_VERSION"; then
            print_success "Rust $rust_version (>= $REQUIRED_RUST_VERSION)"
        else
            print_error "Rust 版本过低: $rust_version (需要 >= $REQUIRED_RUST_VERSION)"
            print_info "运行 'rustup update' 更新 Rust"
            has_error=true
        fi
    else
        print_error "未安装 Rust"
        print_info "访问 https://rustup.rs 安装 Rust"
        has_error=true
    fi

    # 检查 Cargo
    print_step "检查 Cargo..."
    if command_exists cargo; then
        print_success "Cargo $(cargo --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        print_error "未安装 Cargo"
        has_error=true
    fi

    # 检查 Docker
    print_step "检查 Docker..."
    if command_exists docker; then
        if docker info &> /dev/null; then
            print_success "Docker $(docker --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
        else
            print_warning "Docker 未运行，请启动 Docker Desktop"
        fi
    else
        print_error "未安装 Docker"
        print_info "访问 https://docs.docker.com/get-docker/ 安装 Docker"
        has_error=true
    fi

    # 检查 Docker Compose
    print_step "检查 Docker Compose..."
    if docker compose version &> /dev/null; then
        print_success "Docker Compose $(docker compose version --short)"
    elif command_exists docker-compose; then
        print_success "docker-compose $(docker-compose --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        print_warning "未安装 Docker Compose 插件"
    fi

    # 检查 Buf CLI
    print_step "检查 Buf CLI..."
    if command_exists buf; then
        print_success "Buf $(buf --version 2>&1 | head -1)"
    else
        print_warning "未安装 Buf CLI (用于 Protocol Buffers)"
    fi

    # 检查 SQLx CLI
    print_step "检查 SQLx CLI..."
    if command_exists sqlx; then
        print_success "SQLx CLI $(sqlx --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        print_warning "未安装 SQLx CLI (用于数据库迁移)"
    fi

    # 检查 Make
    print_step "检查 Make..."
    if command_exists make; then
        print_success "Make $(make --version | head -1 | grep -oE '[0-9]+\.[0-9]+' | head -1)"
    else
        print_warning "未安装 Make"
    fi

    if [[ "$has_error" == true ]]; then
        print_error "环境检查失败，请安装缺失的依赖"
        exit 1
    fi
}

# =============================================================================
# 2. 目录结构验证
# =============================================================================
verify_directory_structure() {
    print_header "2. 目录结构验证"
    local missing_dirs=()
    local missing_files=()

    # 关键目录
    local required_dirs=(
        "libs"
        "services"
        "proto"
        "config"
        "infrastructure"
        "infrastructure/docker"
        "scripts"
        "tools"
        "docs"
    )

    # 关键文件
    local required_files=(
        "Cargo.toml"
        "Makefile"
        ".env.example"
        "infrastructure/docker/docker-compose.core.yml"
    )

    print_step "检查目录结构..."
    for dir in "${required_dirs[@]}"; do
        if [[ -d "$PROJECT_ROOT/$dir" ]]; then
            print_success "目录存在: $dir"
        else
            print_warning "目录缺失: $dir"
            missing_dirs+=("$dir")
        fi
    done

    print_step "检查关键文件..."
    for file in "${required_files[@]}"; do
        if [[ -f "$PROJECT_ROOT/$file" ]]; then
            print_success "文件存在: $file"
        else
            print_warning "文件缺失: $file"
            missing_files+=("$file")
        fi
    done

    # 输出缺失清单
    if [[ ${#missing_dirs[@]} -gt 0 ]] || [[ ${#missing_files[@]} -gt 0 ]]; then
        echo ""
        print_warning "缺失项目清单:"
        for dir in "${missing_dirs[@]}"; do
            echo -e "  ${YELLOW}目录:${NC} $dir"
        done
        for file in "${missing_files[@]}"; do
            echo -e "  ${YELLOW}文件:${NC} $file"
        done

        if ! ask_continue "是否继续初始化?"; then
            exit 1
        fi
    fi
}

# =============================================================================
# 3. 依赖安装
# =============================================================================
install_dependencies() {
    print_header "3. 依赖安装"

    # 安装 Rust 工具
    print_step "安装 Rust 开发工具..."

    if ! command_exists sqlx; then
        print_info "安装 sqlx-cli..."
        cargo install sqlx-cli --no-default-features --features native-tls,postgres 2>&1 | tee -a "$LOG_FILE"
        print_success "sqlx-cli 安装完成"
    else
        print_success "sqlx-cli 已安装"
    fi

    if ! command_exists cargo-watch; then
        print_info "安装 cargo-watch..."
        cargo install cargo-watch 2>&1 | tee -a "$LOG_FILE"
        print_success "cargo-watch 安装完成"
    else
        print_success "cargo-watch 已安装"
    fi

    if ! command_exists cargo-audit; then
        print_info "安装 cargo-audit..."
        cargo install cargo-audit 2>&1 | tee -a "$LOG_FILE"
        print_success "cargo-audit 安装完成"
    else
        print_success "cargo-audit 已安装"
    fi

    # 安装 Buf CLI
    if ! command_exists buf; then
        print_step "安装 Buf CLI..."
        if [[ "$(uname -s)" == "Darwin" ]]; then
            if command_exists brew; then
                brew install bufbuild/buf/buf 2>&1 | tee -a "$LOG_FILE"
            else
                print_warning "请手动安装 Buf: https://buf.build/docs/installation"
            fi
        elif [[ "$(uname -s)" == "Linux" ]]; then
            curl -sSL "https://github.com/bufbuild/buf/releases/latest/download/buf-$(uname -s)-$(uname -m)" \
                -o /usr/local/bin/buf && chmod +x /usr/local/bin/buf 2>&1 | tee -a "$LOG_FILE" || \
                print_warning "Buf 安装失败，请手动安装"
        fi
    else
        print_success "Buf CLI 已安装"
    fi

    print_success "依赖安装完成"
}

# =============================================================================
# 4. 配置文件初始化
# =============================================================================
init_config_files() {
    print_header "4. 配置文件初始化"

    # 复制 .env.example 到 .env
    print_step "初始化环境变量文件..."
    if [[ -f "$PROJECT_ROOT/.env" ]]; then
        print_warning ".env 文件已存在"
        if ask_continue "是否覆盖现有 .env 文件?"; then
            cp "$PROJECT_ROOT/.env.example" "$PROJECT_ROOT/.env"
            print_success ".env 文件已更新"
        fi
    else
        if [[ -f "$PROJECT_ROOT/.env.example" ]]; then
            cp "$PROJECT_ROOT/.env.example" "$PROJECT_ROOT/.env"
            print_success ".env 文件已创建"
        else
            print_error ".env.example 文件不存在"
        fi
    fi

    # 验证 TOML 配置文件
    print_step "验证配置文件格式..."
    local config_files=()
    while IFS= read -r -d '' file; do
        config_files+=("$file")
    done < <(find "$PROJECT_ROOT/config" -name "*.toml" -print0 2>/dev/null)

    if [[ ${#config_files[@]} -gt 0 ]]; then
        for config_file in "${config_files[@]}"; do
            if cargo run --quiet --bin toml-check -- "$config_file" 2>/dev/null || \
               python3 -c "import toml; toml.load('$config_file')" 2>/dev/null || \
               command_exists tomlq && tomlq . "$config_file" > /dev/null 2>&1; then
                print_success "TOML 有效: $(basename "$config_file")"
            else
                # 简单的 TOML 语法检查
                if grep -qE '^\s*\[.*\]\s*$|^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*=' "$config_file"; then
                    print_success "TOML 格式基本正确: $(basename "$config_file")"
                else
                    print_warning "无法验证 TOML: $(basename "$config_file")"
                fi
            fi
        done
    else
        print_info "未找到 TOML 配置文件"
    fi

    print_info "请编辑 .env 文件配置您的环境变量"
}

# =============================================================================
# 5. Git 初始化
# =============================================================================
init_git() {
    print_header "5. Git 初始化"

    cd "$PROJECT_ROOT"

    # 检查是否已初始化
    if [[ -d ".git" ]]; then
        print_success "Git 仓库已初始化"
        return
    fi

    print_step "初始化 Git 仓库..."
    git init
    print_success "Git 仓库初始化完成"

    # 创建 .gitignore
    print_step "创建 .gitignore..."
    if [[ ! -f ".gitignore" ]]; then
        cat > .gitignore << 'EOF'
# 编译输出
/target/
**/*.rs.bk

# 环境变量 (包含敏感信息)
.env
.env.local
.env.*.local

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# 日志
logs/
*.log

# 操作系统
.DS_Store
Thumbs.db

# 依赖锁定文件 (可选)
# Cargo.lock

# 测试覆盖率
coverage/
*.profraw
*.profdata

# 文档生成
/doc/

# 临时文件
tmp/
temp/
*.tmp
EOF
        print_success ".gitignore 已创建"
    else
        print_success ".gitignore 已存在"
    fi

    # 首次提交
    if ! git rev-parse HEAD &>/dev/null; then
        print_step "执行首次提交..."
        git add .
        git commit -m "chore: initial commit

KILLER ERP 项目初始化

- 项目结构搭建
- 基础配置文件
- 开发环境脚本"
        print_success "首次提交完成"
    fi
}

# =============================================================================
# 6. 基础设施启动
# =============================================================================
start_infrastructure() {
    if [[ "$SKIP_DOCKER" == true ]]; then
        print_header "6. 基础设施启动 (已跳过)"
        return
    fi

    print_header "6. 基础设施启动"

    # 检查 Docker 是否运行
    if ! docker info &> /dev/null; then
        print_error "Docker 未运行，请先启动 Docker"
        if ! ask_continue "是否跳过基础设施启动?"; then
            exit 1
        fi
        return
    fi

    local compose_file="$PROJECT_ROOT/infrastructure/docker/docker-compose.core.yml"

    if [[ ! -f "$compose_file" ]]; then
        print_warning "docker-compose.core.yml 不存在，跳过服务启动"
        return
    fi

    print_step "启动核心基础设施服务..."
    cd "$PROJECT_ROOT/infrastructure/docker"

    # 拉取镜像
    print_info "拉取 Docker 镜像..."
    docker compose -f docker-compose.core.yml pull 2>&1 | tee -a "$LOG_FILE" || true

    # 启动服务
    docker compose -f docker-compose.core.yml up -d 2>&1 | tee -a "$LOG_FILE"
    print_success "Docker 服务启动命令已执行"

    # 等待服务就绪
    print_step "等待服务健康检查..."
    local max_wait=60
    local waited=0
    local services_ready=false

    while [[ $waited -lt $max_wait ]]; do
        if docker compose -f docker-compose.core.yml ps 2>/dev/null | grep -q "healthy\|running"; then
            services_ready=true
            break
        fi
        echo -n "."
        sleep 2
        waited=$((waited + 2))
    done
    echo ""

    if [[ "$services_ready" == true ]]; then
        print_success "服务已就绪"
    else
        print_warning "部分服务可能仍在启动中"
    fi

    # 输出服务地址
    echo ""
    print_info "服务访问地址:"
    echo -e "  ${CYAN}PostgreSQL:${NC}  localhost:5432"
    echo -e "  ${CYAN}Redis:${NC}       localhost:6379"
    echo -e "  ${CYAN}Kafka:${NC}       localhost:9092"
    echo -e "  ${CYAN}Kafka UI:${NC}    http://localhost:8080"
    echo -e "  ${CYAN}MinIO API:${NC}   localhost:9000"
    echo -e "  ${CYAN}MinIO Console:${NC} http://localhost:9001"

    cd "$PROJECT_ROOT"
}

# =============================================================================
# 7. 数据库初始化
# =============================================================================
init_database() {
    if [[ "$SKIP_DB" == true ]]; then
        print_header "7. 数据库初始化 (已跳过)"
        return
    fi

    print_header "7. 数据库初始化"

    if ! command_exists sqlx; then
        print_warning "SQLx CLI 未安装，跳过数据库迁移"
        return
    fi

    # 检查数据库连接
    print_step "检查数据库连接..."
    if [[ -f "$PROJECT_ROOT/.env" ]]; then
        source "$PROJECT_ROOT/.env" 2>/dev/null || true
    fi

    local db_url="${DATABASE_URL:-postgresql://killer:killer_dev@localhost:5432/erp_financial}"

    if pg_isready -h localhost -p 5432 &>/dev/null || \
       docker exec killer-postgres pg_isready &>/dev/null 2>&1; then
        print_success "数据库连接正常"
    else
        print_warning "无法连接到数据库，跳过迁移"
        return
    fi

    # 运行迁移
    print_step "运行数据库迁移..."
    local migration_dirs=()
    while IFS= read -r -d '' dir; do
        migration_dirs+=("$dir")
    done < <(find "$PROJECT_ROOT/services" -type d -name "migrations" -print0 2>/dev/null)

    if [[ ${#migration_dirs[@]} -gt 0 ]]; then
        for migration_dir in "${migration_dirs[@]}"; do
            local service_name
            service_name=$(basename "$(dirname "$migration_dir")")
            print_info "迁移服务: $service_name"
            cd "$(dirname "$migration_dir")"
            sqlx migrate run 2>&1 | tee -a "$LOG_FILE" || print_warning "$service_name 迁移失败"
        done
        print_success "数据库迁移完成"
    else
        print_info "未找到迁移文件"
    fi

    # 填充测试数据
    if [[ "$SEED_DATA" == true ]]; then
        print_step "填充测试数据..."
        local seed_script="$PROJECT_ROOT/scripts/seed-data.sh"
        if [[ -f "$seed_script" ]]; then
            bash "$seed_script" 2>&1 | tee -a "$LOG_FILE"
            print_success "测试数据填充完成"
        else
            print_warning "未找到 seed-data.sh 脚本"
        fi
    fi

    cd "$PROJECT_ROOT"
}

# =============================================================================
# 8. 验证
# =============================================================================
run_verification() {
    print_header "8. 项目验证"

    cd "$PROJECT_ROOT"

    # Cargo check
    print_step "运行 cargo check..."
    if cargo check 2>&1 | tee -a "$LOG_FILE"; then
        print_success "cargo check 通过"
    else
        print_warning "cargo check 失败，请检查编译错误"
    fi

    # Buf lint
    print_step "运行 buf lint..."
    if command_exists buf; then
        if [[ -d "$PROJECT_ROOT/proto" ]]; then
            cd "$PROJECT_ROOT/proto"
            if buf lint 2>&1 | tee -a "$LOG_FILE"; then
                print_success "buf lint 通过"
            else
                print_warning "buf lint 发现问题"
            fi
            cd "$PROJECT_ROOT"
        else
            print_info "proto 目录不存在，跳过 buf lint"
        fi
    else
        print_info "Buf CLI 未安装，跳过 lint"
    fi
}

# =============================================================================
# 9. 输出总结
# =============================================================================
print_summary() {
    print_header "9. 初始化完成"

    echo -e "${GREEN}${BOLD}"
    echo "  ██╗  ██╗██╗██╗     ██╗     ███████╗██████╗ "
    echo "  ██║ ██╔╝██║██║     ██║     ██╔════╝██╔══██╗"
    echo "  █████╔╝ ██║██║     ██║     █████╗  ██████╔╝"
    echo "  ██╔═██╗ ██║██║     ██║     ██╔══╝  ██╔══██╗"
    echo "  ██║  ██╗██║███████╗███████╗███████╗██║  ██║"
    echo "  ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝"
    echo -e "${NC}"

    echo -e "${BOLD}项目信息:${NC}"
    echo -e "  ${CYAN}项目路径:${NC}    $PROJECT_ROOT"
    echo -e "  ${CYAN}Rust 版本:${NC}   $(rustc --version 2>/dev/null || echo '未知')"
    echo -e "  ${CYAN}初始化日志:${NC}  $LOG_FILE"
    echo ""

    echo -e "${BOLD}下一步操作:${NC}"
    echo -e "  ${WHITE}1.${NC} 编辑 ${CYAN}.env${NC} 文件配置环境变量"
    echo -e "  ${WHITE}2.${NC} 运行 ${CYAN}make dev${NC} 启动开发环境"
    echo -e "  ${WHITE}3.${NC} 运行 ${CYAN}make test${NC} 执行测试"
    echo -e "  ${WHITE}4.${NC} 查看 ${CYAN}docs/${NC} 目录了解更多"
    echo ""

    echo -e "${BOLD}常用命令:${NC}"
    echo -e "  ${CYAN}make build${NC}        构建项目"
    echo -e "  ${CYAN}make dev${NC}          启动开发服务器 (热重载)"
    echo -e "  ${CYAN}make test${NC}         运行测试"
    echo -e "  ${CYAN}make lint${NC}         代码检查"
    echo -e "  ${CYAN}make fmt${NC}          格式化代码"
    echo -e "  ${CYAN}make db-migrate${NC}   数据库迁移"
    echo -e "  ${CYAN}make proto${NC}        生成 protobuf 代码"
    echo -e "  ${CYAN}make docker-up${NC}    启动 Docker 服务"
    echo -e "  ${CYAN}make docker-down${NC}  停止 Docker 服务"
    echo ""

    echo -e "${GREEN}${BOLD}项目初始化成功!${NC}"
    log "初始化完成"
}

# =============================================================================
# 主函数
# =============================================================================
main() {
    parse_args "$@"
    init_logging

    echo -e "${BOLD}"
    echo "  ╔═══════════════════════════════════════════════════════════════════╗"
    echo "  ║           KILLER ERP - 项目初始化脚本                              ║"
    echo "  ╚═══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""

    check_environment
    verify_directory_structure
    install_dependencies
    init_config_files
    init_git
    start_infrastructure
    init_database
    run_verification
    print_summary
}

# 执行主函数
main "$@"
