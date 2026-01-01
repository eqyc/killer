#!/usr/bin/env bash
# =============================================================================
# KILLER ERP - 项目设置验证脚本
# =============================================================================
# 用途: 验证项目设置是否正确，检查所有依赖和服务状态
# 兼容: macOS, Linux
# 用法: ./scripts/verify-setup.sh [--verbose] [--fix]
# =============================================================================

set -euo pipefail

# =============================================================================
# 全局变量
# =============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERBOSE=false
FIX_ISSUES=false
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# =============================================================================
# 颜色定义
# =============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'
BOLD='\033[1m'

# =============================================================================
# 工具函数
# =============================================================================

print_header() {
    echo -e "\n${CYAN}${BOLD}═══ $1 ═══${NC}\n"
}

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED_CHECKS++))
    ((TOTAL_CHECKS++))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED_CHECKS++))
    ((TOTAL_CHECKS++))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNING_CHECKS++))
    ((TOTAL_CHECKS++))
}

check_info() {
    if [[ "$VERBOSE" == true ]]; then
        echo -e "${BLUE}ℹ${NC} $1"
    fi
}

command_exists() {
    command -v "$1" &> /dev/null
}

version_ge() {
    printf '%s\n%s\n' "$2" "$1" | sort -V -C
}

# =============================================================================
# 解析命令行参数
# =============================================================================
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose) VERBOSE=true; shift ;;
            --fix) FIX_ISSUES=true; shift ;;
            -h|--help) show_help; exit 0 ;;
            *) echo "未知参数: $1"; show_help; exit 1 ;;
        esac
    done
}

show_help() {
    echo -e "${BOLD}用法:${NC} $0 [选项]"
    echo ""
    echo -e "${BOLD}选项:${NC}"
    echo "  -v, --verbose    显示详细信息"
    echo "  --fix            尝试自动修复问题"
    echo "  -h, --help       显示帮助信息"
}

# =============================================================================
# 1. 工具链检查
# =============================================================================
verify_toolchain() {
    print_header "工具链检查"

    # Rust
    if command_exists rustc; then
        local rust_version
        rust_version=$(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        if version_ge "$rust_version" "1.92"; then
            check_pass "Rust $rust_version"
        else
            check_fail "Rust 版本过低: $rust_version (需要 >= 1.92)"
        fi
    else
        check_fail "Rust 未安装"
    fi

    # Cargo
    if command_exists cargo; then
        check_pass "Cargo $(cargo --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        check_fail "Cargo 未安装"
    fi

    # Docker
    if command_exists docker; then
        if docker info &> /dev/null; then
            check_pass "Docker $(docker --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+') (运行中)"
        else
            check_warn "Docker 已安装但未运行"
        fi
    else
        check_fail "Docker 未安装"
    fi

    # Docker Compose
    if docker compose version &> /dev/null; then
        check_pass "Docker Compose $(docker compose version --short)"
    elif command_exists docker-compose; then
        check_pass "docker-compose $(docker-compose --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        check_warn "Docker Compose 未安装"
    fi

    # Buf CLI
    if command_exists buf; then
        check_pass "Buf CLI $(buf --version 2>&1 | head -1)"
    else
        check_warn "Buf CLI 未安装"
    fi

    # SQLx CLI
    if command_exists sqlx; then
        check_pass "SQLx CLI $(sqlx --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
    else
        check_warn "SQLx CLI 未安装"
    fi

    # cargo-watch
    if command_exists cargo-watch; then
        check_pass "cargo-watch 已安装"
    else
        check_warn "cargo-watch 未安装"
    fi

    # cargo-audit
    if command_exists cargo-audit; then
        check_pass "cargo-audit 已安装"
    else
        check_warn "cargo-audit 未安装"
    fi

    # Make
    if command_exists make; then
        check_pass "Make $(make --version | head -1 | grep -oE '[0-9]+\.[0-9]+' | head -1)"
    else
        check_warn "Make 未安装"
    fi
}

# =============================================================================
# 2. 项目结构检查
# =============================================================================
verify_project_structure() {
    print_header "项目结构检查"

    cd "$PROJECT_ROOT"

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

    for dir in "${required_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            check_pass "目录: $dir"
        else
            check_fail "目录缺失: $dir"
            if [[ "$FIX_ISSUES" == true ]]; then
                mkdir -p "$dir"
                echo -e "  ${GREEN}→${NC} 已创建目录: $dir"
            fi
        fi
    done

    # 关键文件
    local required_files=(
        "Cargo.toml"
        "Makefile"
        ".env.example"
        "README.md"
    )

    for file in "${required_files[@]}"; do
        if [[ -f "$file" ]]; then
            check_pass "文件: $file"
        else
            check_fail "文件缺失: $file"
        fi
    done
}

# =============================================================================
# 3. 配置文件检查
# =============================================================================
verify_config_files() {
    print_header "配置文件检查"

    cd "$PROJECT_ROOT"

    # .env 文件
    if [[ -f ".env" ]]; then
        check_pass ".env 文件存在"

        # 检查关键环境变量
        local required_vars=(
            "DATABASE_URL"
            "REDIS_URL"
            "KAFKA_BROKERS"
        )

        for var in "${required_vars[@]}"; do
            if grep -q "^${var}=" .env 2>/dev/null; then
                check_info "$var 已配置"
            else
                check_warn "$var 未在 .env 中配置"
            fi
        done
    else
        check_warn ".env 文件不存在"
        if [[ "$FIX_ISSUES" == true ]] && [[ -f ".env.example" ]]; then
            cp .env.example .env
            echo -e "  ${GREEN}→${NC} 已从 .env.example 创建 .env"
        fi
    fi

    # Cargo.toml 验证
    if [[ -f "Cargo.toml" ]]; then
        if cargo verify-project &>/dev/null || cargo metadata --no-deps &>/dev/null; then
            check_pass "Cargo.toml 格式有效"
        else
            check_fail "Cargo.toml 格式无效"
        fi
    fi

    # 检查 TOML 配置文件
    local config_count=0
    while IFS= read -r -d '' file; do
        ((config_count++))
        check_info "配置文件: $(basename "$file")"
    done < <(find "$PROJECT_ROOT/config" -name "*.toml" -print0 2>/dev/null)

    if [[ $config_count -gt 0 ]]; then
        check_pass "找到 $config_count 个 TOML 配置文件"
    else
        check_info "未找到 TOML 配置文件"
    fi
}

# =============================================================================
# 4. Git 仓库检查
# =============================================================================
verify_git_repo() {
    print_header "Git 仓库检查"

    cd "$PROJECT_ROOT"

    # Git 初始化
    if [[ -d ".git" ]]; then
        check_pass "Git 仓库已初始化"

        # 检查远程仓库
        if git remote -v | grep -q origin; then
            local remote_url
            remote_url=$(git remote get-url origin 2>/dev/null || echo "未知")
            check_pass "远程仓库: $remote_url"
        else
            check_info "未配置远程仓库"
        fi

        # 检查当前分支
        local current_branch
        current_branch=$(git branch --show-current 2>/dev/null || echo "未知")
        check_pass "当前分支: $current_branch"

        # 检查未提交更改
        if git diff --quiet && git diff --cached --quiet; then
            check_pass "工作区干净"
        else
            check_warn "存在未提交的更改"
        fi
    else
        check_fail "Git 仓库未初始化"
        if [[ "$FIX_ISSUES" == true ]]; then
            git init
            echo -e "  ${GREEN}→${NC} 已初始化 Git 仓库"
        fi
    fi

    # .gitignore
    if [[ -f ".gitignore" ]]; then
        check_pass ".gitignore 文件存在"

        # 检查关键忽略项
        local ignore_patterns=("target/" ".env" "*.log")
        for pattern in "${ignore_patterns[@]}"; do
            if grep -q "$pattern" .gitignore 2>/dev/null; then
                check_info "忽略模式: $pattern"
            else
                check_warn ".gitignore 缺少: $pattern"
            fi
        done
    else
        check_warn ".gitignore 文件不存在"
    fi
}

# =============================================================================
# 5. Docker 服务检查
# =============================================================================
verify_docker_services() {
    print_header "Docker 服务检查"

    if ! docker info &> /dev/null; then
        check_warn "Docker 未运行，跳过服务检查"
        return
    fi

    cd "$PROJECT_ROOT"

    local compose_file="infrastructure/docker/docker-compose.core.yml"

    if [[ ! -f "$compose_file" ]]; then
        check_warn "docker-compose.core.yml 不存在"
        return
    fi

    # 检查服务状态
    local services=("postgres" "redis" "kafka" "minio")

    for service in "${services[@]}"; do
        if docker ps --format '{{.Names}}' | grep -qi "$service"; then
            check_pass "$service 容器运行中"
        else
            check_info "$service 容器未运行"
        fi
    done

    # 检查网络
    if docker network ls --format '{{.Name}}' | grep -q "killer"; then
        check_pass "Docker 网络已创建"
    else
        check_info "Docker 网络未创建"
    fi

    # 检查数据卷
    local volumes=("postgres_data" "redis_data" "kafka_data")
    for volume in "${volumes[@]}"; do
        if docker volume ls --format '{{.Name}}' | grep -q "$volume"; then
            check_info "数据卷存在: $volume"
        fi
    done
}

# =============================================================================
# 6. 数据库连接检查
# =============================================================================
verify_database() {
    print_header "数据库连接检查"

    # 加载环境变量
    if [[ -f "$PROJECT_ROOT/.env" ]]; then
        set -a
        source "$PROJECT_ROOT/.env" 2>/dev/null || true
        set +a
    fi

    local db_host="${DATABASE_HOST:-localhost}"
    local db_port="${DATABASE_PORT:-5432}"

    # PostgreSQL
    if command_exists pg_isready; then
        if pg_isready -h "$db_host" -p "$db_port" &>/dev/null; then
            check_pass "PostgreSQL 连接正常 ($db_host:$db_port)"
        else
            check_warn "PostgreSQL 无法连接 ($db_host:$db_port)"
        fi
    elif docker exec killer-postgres pg_isready &>/dev/null 2>&1; then
        check_pass "PostgreSQL 连接正常 (Docker)"
    else
        check_info "无法验证 PostgreSQL 连接"
    fi

    # Redis
    local redis_host="${REDIS_HOST:-localhost}"
    local redis_port="${REDIS_PORT:-6379}"

    if command_exists redis-cli; then
        if redis-cli -h "$redis_host" -p "$redis_port" ping &>/dev/null; then
            check_pass "Redis 连接正常 ($redis_host:$redis_port)"
        else
            check_warn "Redis 无法连接 ($redis_host:$redis_port)"
        fi
    elif docker exec killer-redis redis-cli ping &>/dev/null 2>&1; then
        check_pass "Redis 连接正常 (Docker)"
    else
        check_info "无法验证 Redis 连接"
    fi
}

# =============================================================================
# 7. 编译检查
# =============================================================================
verify_compilation() {
    print_header "编译检查"

    cd "$PROJECT_ROOT"

    if [[ ! -f "Cargo.toml" ]]; then
        check_warn "Cargo.toml 不存在，跳过编译检查"
        return
    fi

    # cargo check
    echo -e "${BLUE}ℹ${NC} 运行 cargo check (可能需要一些时间)..."
    if cargo check 2>/dev/null; then
        check_pass "cargo check 通过"
    else
        check_fail "cargo check 失败"
    fi

    # cargo fmt 检查
    if command_exists rustfmt; then
        if cargo fmt --check &>/dev/null; then
            check_pass "代码格式正确"
        else
            check_warn "代码格式需要调整 (运行 cargo fmt)"
        fi
    fi

    # cargo clippy
    if cargo clippy --version &>/dev/null; then
        echo -e "${BLUE}ℹ${NC} 运行 cargo clippy..."
        if cargo clippy -- -D warnings 2>/dev/null; then
            check_pass "cargo clippy 通过"
        else
            check_warn "cargo clippy 发现警告"
        fi
    fi
}

# =============================================================================
# 8. Proto 文件检查
# =============================================================================
verify_proto() {
    print_header "Proto 文件检查"

    cd "$PROJECT_ROOT"

    if [[ ! -d "proto" ]]; then
        check_info "proto 目录不存在"
        return
    fi

    # 统计 proto 文件
    local proto_count
    proto_count=$(find proto -name "*.proto" 2>/dev/null | wc -l | tr -d ' ')

    if [[ $proto_count -gt 0 ]]; then
        check_pass "找到 $proto_count 个 proto 文件"
    else
        check_info "未找到 proto 文件"
        return
    fi

    # buf lint
    if command_exists buf; then
        cd proto
        if buf lint 2>/dev/null; then
            check_pass "buf lint 通过"
        else
            check_warn "buf lint 发现问题"
        fi
        cd "$PROJECT_ROOT"
    else
        check_info "Buf CLI 未安装，跳过 lint"
    fi
}

# =============================================================================
# 输出总结
# =============================================================================
print_summary() {
    echo ""
    echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}                           验证结果总结${NC}"
    echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${GREEN}通过:${NC}   $PASSED_CHECKS"
    echo -e "  ${RED}失败:${NC}   $FAILED_CHECKS"
    echo -e "  ${YELLOW}警告:${NC}   $WARNING_CHECKS"
    echo -e "  ${BLUE}总计:${NC}   $TOTAL_CHECKS"
    echo ""

    if [[ $FAILED_CHECKS -eq 0 ]]; then
        echo -e "${GREEN}${BOLD}✓ 项目设置验证通过!${NC}"
        exit 0
    else
        echo -e "${RED}${BOLD}✗ 项目设置存在问题，请检查上述失败项${NC}"
        if [[ "$FIX_ISSUES" == false ]]; then
            echo -e "${YELLOW}提示: 运行 '$0 --fix' 尝试自动修复部分问题${NC}"
        fi
        exit 1
    fi
}

# =============================================================================
# 主函数
# =============================================================================
main() {
    parse_args "$@"

    echo -e "${BOLD}"
    echo "  ╔═══════════════════════════════════════════════════════════════════╗"
    echo "  ║           KILLER ERP - 项目设置验证                                ║"
    echo "  ╚═══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    verify_toolchain
    verify_project_structure
    verify_config_files
    verify_git_repo
    verify_docker_services
    verify_database
    verify_compilation
    verify_proto
    print_summary
}

# 执行主函数
main "$@"
