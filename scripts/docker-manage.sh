#!/usr/bin/env bash
# Docker 本地环境管理脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCKER_DIR="$PROJECT_ROOT/docker"

# Docker Compose 文件
COMPOSE_CORE="$DOCKER_DIR/docker-compose.core.yml"
COMPOSE_FULL="$DOCKER_DIR/docker-compose.full.yml"
COMPOSE_OBSERVABILITY="$DOCKER_DIR/docker-compose.observability.yml"
COMPOSE_OVERRIDE="$DOCKER_DIR/docker-compose.override.yml"

# 组合参数定义
# 核心模式: core + override
ARGS_CORE="-f $COMPOSE_CORE -f $COMPOSE_OVERRIDE"
# 全量模式: core + override + observability (解决孤儿容器警告的关键)
ARGS_ALL="-f $COMPOSE_CORE -f $COMPOSE_OVERRIDE -f $COMPOSE_OBSERVABILITY"
# 仅可观测性
ARGS_OBS="-f $COMPOSE_OBSERVABILITY"
# Full 单文件模式
ARGS_FULL="-f $COMPOSE_FULL"

# 打印带颜色的消息
print_info() { echo -e "${BLUE}ℹ${NC} $1"; }
print_success() { echo -e "${GREEN}✓${NC} $1"; }
print_warning() { echo -e "${YELLOW}⚠${NC} $1"; }
print_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo -e "${CYAN}================================================${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}================================================${NC}"
}

# 检查 Docker 是否运行
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker 未运行，请先启动 Docker"
        exit 1
    fi
}

# 启动服务
start_services() {
    local mode=${1:-core}
    check_docker

    case $mode in
        core)
            print_header "启动核心服务"
            docker-compose $ARGS_CORE up -d
            print_success "核心服务已启动"
            ;;
        full)
            print_header "启动所有服务 (Full模式)"
            docker-compose $ARGS_FULL up -d
            print_success "所有服务已启动"
            ;;
        obs|observability)
            print_header "启动可观测性服务"
            docker-compose $ARGS_OBS up -d
            print_success "可观测性服务已启动"
            ;;
        all)
            print_header "启动所有服务 (核心 + 可观测性)"
            # 关键修改：使用合并的参数启动，避免孤儿容器警告
            docker-compose $ARGS_ALL up -d
            print_success "所有服务已启动"
            ;;
        *)
            print_error "未知模式: $mode"
            exit 1
            ;;
    esac

    echo ""
    print_info "等待服务启动..."
    sleep 5
    show_status
}

# 停止服务
stop_services() {
    print_header "停止所有服务"
    # 使用全量参数停止，确保能覆盖到所有可能运行的容器
    docker-compose $ARGS_ALL stop 2>/dev/null || true
    # 额外尝试停止 full 模式，防止残留
    docker-compose $ARGS_FULL stop 2>/dev/null || true
    print_success "所有服务已停止"
}

# 停止并删除容器
down_services() {
    print_header "停止并删除容器"
    # 使用全量参数 Down，一次性清理干净且无警告
    docker-compose $ARGS_ALL down --remove-orphans 2>/dev/null || true
    docker-compose $ARGS_FULL down --remove-orphans 2>/dev/null || true
    print_success "容器已删除"
}

# 清理所有数据
clean_all() {
    print_warning "这将删除所有容器和数据卷！"
    read -p "确认清理? (yes/NO): " -r
    echo
    if [[ $REPLY == "yes" ]]; then
        print_header "清理所有数据"
        docker-compose $ARGS_ALL down -v --remove-orphans 2>/dev/null || true
        docker-compose $ARGS_FULL down -v --remove-orphans 2>/dev/null || true
        print_success "所有数据已清理"
    fi
}

# 重启服务
restart_service() {
    local service=$1
    if [ -z "$service" ]; then
        print_header "重启所有服务"
        # 默认重启全量
        docker-compose $ARGS_ALL restart
    else
        print_header "重启服务: $service"
        # 尝试在全量配置中重启
        docker-compose $ARGS_ALL restart "$service"
    fi
    print_success "重启完成"
}

# 查看服务状态 (PS)
show_ps() {
    print_header "运行中的服务"
    # 使用全量配置查看，这样能在一个视图里看到所有服务
    docker-compose $ARGS_ALL ps
}

# 查看日志
show_logs() {
    local service=$1
    shift
    if [ -z "$service" ]; then
        docker-compose $ARGS_ALL logs "$@"
    else
        docker-compose $ARGS_ALL logs "$service" "$@"
    fi
}

# 显示详细状态
show_status() {
    print_header "服务健康状态"
    
    # 辅助函数：检查服务
    check_svc() {
        local name=$1
        local port_info=$2
        local check_cmd=$3
        
        # 填充空格对齐
        printf "% -14s" "$name"
        
        if docker ps --format '{{.Names}}' | grep -q "$name"; then
            if [ -n "$check_cmd" ]; then
                if eval "$check_cmd" > /dev/null 2>&1; then
                    echo -e "- ${GREEN}运行中${NC} $port_info"
                else
                    echo -e "- ${YELLOW}启动中...${NC}"
                fi
            else
                 echo -e "- ${GREEN}运行中${NC} $port_info"
            fi
        else
            echo -e "- ${RED}未运行${NC}"
        fi
    }

    echo ""
    print_info "核心服务:"
    check_svc "postgres"   "(5432)" "docker exec $(docker ps -qf name=postgres) pg_isready"
    check_svc "redis"      "(6379)" "docker exec $(docker ps -qf name=redis) redis-cli ping"
    check_svc "kafka"      "(9092)" ""
    check_svc "clickhouse" "(8123)" "curl -s http://localhost:8123/ping"
    check_svc "opensearch" "(9200)" "curl -s http://localhost:9200"

    echo ""
    print_info "可观测性服务:"
    check_svc "otel-collector" "(4317/8889)" ""
    check_svc "prometheus"     "(9090)" "wget -q --spider http://localhost:9090/-/healthy"
    check_svc "grafana"        "(3000)" ""
    check_svc "tempo"          "(3200)" ""
    check_svc "loki"           "(3100)" ""
    echo ""
}

# 健康检查 (简化版)
check_health() {
    print_header "健康检查概览"
    # 使用 docker-compose ps 的 health status
    docker-compose $ARGS_ALL ps --format "table {{.Name}}\t{{.Status}}\t{{.Health}}"
}

# 工具函数
enter_shell() {
    local service=$1
    if [ -z "$service" ]; then echo "请指定服务名"; exit 1; fi
    docker-compose $ARGS_ALL exec "$service" /bin/bash || docker-compose $ARGS_ALL exec "$service" /bin/sh
}

exec_command() {
    local service=$1
    shift
    if [ -z "$service" ]; then echo "请指定服务名"; exit 1; fi
    docker-compose $ARGS_ALL exec "$service" "$@"
}

backup_database() {
    local backup_file="backups/pg_$(date +%Y%m%d_%H%M%S).sql"
    mkdir -p backups
    print_info "备份数据库到 $backup_file..."
    docker exec $(docker ps -qf "name=postgres") pg_dumpall -U postgres > "$backup_file"
    print_success "备份完成"
}

# 主入口
main() {
    cd "$PROJECT_ROOT"
    local command=${1:-help}
    shift || true

    case $command in
        start)   start_services "$@" ;;
        stop)    stop_services ;;
        down)    down_services ;;
        clean)   clean_all ;; 
        restart) restart_service "$@" ;; 
        ps)      show_ps ;; 
        logs)    show_logs "$@" ;; 
        status)  show_status ;; 
        health)  check_health ;; 
        shell)   enter_shell "$@" ;; 
        exec)    exec_command "$@" ;; 
        backup)  backup_database ;; 
        *)       echo "用法: $0 {start|stop|down|restart|ps|logs|status|health|shell|backup}"; exit 1 ;; 
    esac
}

main "$@"