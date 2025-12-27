#!/usr/bin/env bash
# Docker 快速启动脚本

set -e

# 颜色定义
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}🚀 启动本地开发环境${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# 获取脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# 检查 Docker
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker 未运行，请先启动 Docker"
    exit 1
fi

echo "📦 启动核心服务（PostgreSQL, Redis, ClickHouse, OpenSearch）"
echo ""

# 启动服务
cd "$PROJECT_ROOT"
docker-compose -f docker/docker-compose.minimal.yml up -d

echo ""
echo -e "${GREEN}等待服务就绪...${NC}"
sleep 5

echo ""
echo -e "${GREEN}✅ 环境启动完成！${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 服务访问地址:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "核心服务:"
echo "  PostgreSQL:   postgresql://postgres:postgres@localhost:5432/erp_system"
echo "  Redis:        redis://localhost:6379"
echo "  ClickHouse:   http://localhost:8123 (HTTP), tcp://localhost:9000 (Native)"
echo "  OpenSearch:   http://localhost:9200"
echo ""
echo -e "${YELLOW}注意:${NC}"
echo -e "  ${RED}Kafka:        暂时不可用（Docker Hub 网络问题）${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💡 常用命令:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  查看服务状态:   docker ps"
echo "  查看日志:       docker-compose -f docker/docker-compose.minimal.yml logs -f [服务名]"
echo "  停止服务:       docker-compose -f docker/docker-compose.minimal.yml down"
echo "  重启服务:       docker-compose -f docker/docker-compose.minimal.yml restart [服务名]"
echo "  进入容器:       docker exec -it erp-[服务名] bash"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Happy Coding! 🎉"
echo ""
