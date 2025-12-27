#!/usr/bin/env bash
# 快速启动可观测性组件脚本

set -e

echo "=========================================="
echo "启动 ERP 系统可观测性组件"
echo "=========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 切换到项目根目录
cd "$(dirname "$0")/.."

echo -e "${YELLOW}→${NC} 启动可观测性栈..."
docker-compose -f docker/docker-compose.observability.yml up -d

echo ""
echo -e "${YELLOW}→${NC} 等待服务启动..."
sleep 10

echo ""
echo "=========================================="
echo -e "${GREEN}✓${NC} 可观测性组件已启动！"
echo "=========================================="
echo ""
echo "访问以下服务："
echo ""
echo "  📊 Grafana:    http://localhost:3000"
echo "     账号: admin / admin"
echo ""
echo "  📈 Prometheus: http://localhost:9090"
echo ""
echo "  🔍 Tempo:      http://localhost:3200"
echo ""
echo "  📝 Loki:       http://localhost:3100"
echo ""
echo "=========================================="
echo ""
echo "查看日志："
echo "  docker-compose -f docker/docker-compose.observability.yml logs -f"
echo ""
echo "停止服务："
echo "  docker-compose -f docker/docker-compose.observability.yml down"
echo ""
