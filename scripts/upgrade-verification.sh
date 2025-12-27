#!/usr/bin/env bash
# 版本升级验证脚本
# 用于验证所有组件版本是否正确升级

set -e

echo "=========================================="
echo "ERP 系统版本升级验证脚本"
echo "=========================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查函数
check_version() {
    local component=$1
    local expected=$2
    local actual=$3

    if [[ "$actual" == "$expected" ]]; then
        echo -e "${GREEN}✓${NC} $component: $actual (期望: $expected)"
        return 0
    else
        echo -e "${RED}✗${NC} $component: $actual (期望: $expected)"
        return 1
    fi
}

check_contains() {
    local component=$1
    local expected=$2
    local actual=$3

    if [[ "$actual" == *"$expected"* ]]; then
        echo -e "${GREEN}✓${NC} $component: 包含 '$expected'"
        return 0
    else
        echo -e "${RED}✗${NC} $component: 不包含 '$expected' (实际: $actual)"
        return 1
    fi
}

failed_checks=0

echo "=== 检查 Docker Compose 配置 ==="
echo ""

# 检查 PostgreSQL
pg_version=$(grep -A1 "postgres:" docker/docker-compose.core.yml | grep "image:" | awk '{print $2}')
check_version "PostgreSQL" "postgres:18" "$pg_version" || ((failed_checks++))

# 检查 Kafka
kafka_version=$(grep -A1 "kafka:" docker/docker-compose.core.yml | grep "image:" | awk '{print $2}')
check_version "Kafka" "confluentinc/cp-kafka:7.8.0" "$kafka_version" || ((failed_checks++))

# 检查 Redis
redis_version=$(grep -A1 "redis:" docker/docker-compose.core.yml | grep "image:" | awk '{print $2}')
check_version "Redis" "redis:8" "$redis_version" || ((failed_checks++))

# 检查 ClickHouse
clickhouse_version=$(grep -A1 "clickhouse:" docker/docker-compose.core.yml | grep "image:" | awk '{print $2}')
check_version "ClickHouse" "clickhouse/clickhouse-server:25.12" "$clickhouse_version" || ((failed_checks++))

echo ""
echo "=== 检查 Rust 依赖配置 (Cargo.toml) ==="
echo ""

# 检查 OpenTelemetry 版本
otel_version=$(grep 'opentelemetry = {' Cargo.toml | sed -n 's/.*version = "\([^"]*\)".*/\1/p')
check_version "OpenTelemetry" "0.31" "$otel_version" || ((failed_checks++))

# 检查 buf 是否被删除（应该在注释中）
if grep -q '^buf = ' Cargo.toml; then
    echo -e "${RED}✗${NC} buf 依赖仍然存在（应该被注释或删除）"
    ((failed_checks++))
else
    echo -e "${GREEN}✓${NC} buf 依赖已正确移除"
fi

echo ""
echo "=== 检查可观测性组件配置 ==="
echo ""

# 检查 observability docker-compose 是否存在
if [[ -f "docker/docker-compose.observability.yml" ]]; then
    echo -e "${GREEN}✓${NC} docker-compose.observability.yml 存在"

    # 检查 Prometheus 版本
    prom_version=$(grep "image: prom/prometheus:" docker/docker-compose.observability.yml | sed -n 's/.*:v\([0-9.]*\).*/\1/p')
    check_version "Prometheus" "3.8.1" "$prom_version" || ((failed_checks++))

    # 检查 Grafana 版本
    grafana_version=$(grep "image: grafana/grafana:" docker/docker-compose.observability.yml | sed -n 's/.*:\([0-9.]*\).*/\1/p')
    check_version "Grafana" "12.3.1" "$grafana_version" || ((failed_checks++))

    # 检查 Tempo 版本
    tempo_version=$(grep "image: grafana/tempo:" docker/docker-compose.observability.yml | sed -n 's/.*:\([0-9.]*\).*/\1/p')
    check_contains "Tempo" "2." "$tempo_version" || ((failed_checks++))

    # 检查 Loki 配置
    if [[ -f "infrastructure/monitoring/loki/loki-config.yaml" ]]; then
        echo -e "${GREEN}✓${NC} Loki 配置文件存在"
    else
        echo -e "${RED}✗${NC} Loki 配置文件不存在"
        ((failed_checks++))
    fi
else
    echo -e "${RED}✗${NC} docker-compose.observability.yml 不存在"
    ((failed_checks++))
fi

echo ""
echo "=== 检查 Prometheus 配置 ==="
if [[ -f "infrastructure/monitoring/prometheus/prometheus.yml" ]]; then
    echo -e "${GREEN}✓${NC} Prometheus 配置文件存在"

    # 检查是否配置了所有服务
    services=("api-gateway" "analytics-service" "crm-service" "financial-service" "hr-service")
    for service in "${services[@]}"; do
        if grep -q "job_name: '$service'" infrastructure/monitoring/prometheus/prometheus.yml; then
            echo -e "${GREEN}✓${NC} Prometheus 已配置 $service"
        else
            echo -e "${YELLOW}⚠${NC} Prometheus 未配置 $service"
        fi
    done
else
    echo -e "${RED}✗${NC} Prometheus 配置文件不存在"
    ((failed_checks++))
fi

echo ""
echo "=========================================="
if [[ $failed_checks -eq 0 ]]; then
    echo -e "${GREEN}✓ 所有检查通过！${NC}"
    echo "=========================================="
    exit 0
else
    echo -e "${RED}✗ 发现 $failed_checks 个问题${NC}"
    echo "=========================================="
    exit 1
fi
