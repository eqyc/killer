#!/usr/bin/env bash
# GitHub 配置检查脚本

set -e

echo "=========================================="
echo "GitHub 配置检查脚本"
echo "=========================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_file() {
    local file=$1
    local description=$2

    if [[ -f "$file" ]]; then
        echo -e "${GREEN}✓${NC} $description"
        return 0
    else
        echo -e "${RED}✗${NC} $description (缺失: $file)"
        return 1
    fi
}

failed_checks=0

echo "=== 检查 Workflows ==="
check_file ".github/workflows/ci.yml" "CI Workflow" || ((failed_checks++))
check_file ".github/workflows/release.yml" "Release Workflow" || ((failed_checks++))
check_file ".github/workflows/security-scan.yml" "Security Scan Workflow" || ((failed_checks++))
check_file ".github/workflows/labeler.yml" "Labeler Workflow" || ((failed_checks++))

echo ""
echo "=== 检查 Issue/PR 模板 ==="
check_file ".github/ISSUE_TEMPLATE/bug_report.yml" "Bug Report 模板" || ((failed_checks++))
check_file ".github/ISSUE_TEMPLATE/feature_request.yml" "Feature Request 模板" || ((failed_checks++))
check_file ".github/ISSUE_TEMPLATE/config.yml" "Issue 配置" || ((failed_checks++))
check_file ".github/pull_request_template.md" "PR 模板" || ((failed_checks++))

echo ""
echo "=== 检查其他配置 ==="
check_file ".github/dependabot.yml" "Dependabot 配置" || ((failed_checks++))
check_file ".github/CODEOWNERS" "CODEOWNERS" || ((failed_checks++))
check_file ".github/labeler.yml" "Labeler 规则" || ((failed_checks++))

echo ""
echo "=== 检查 CI Workflow 配置 ==="

# 检查 Rust 版本
rust_version=$(grep 'toolchain:' .github/workflows/ci.yml | head -1 | grep -o '"[0-9.]*"' | tr -d '"')
if [[ "$rust_version" == "1.92.0" ]]; then
    echo -e "${GREEN}✓${NC} Rust 版本: $rust_version"
else
    echo -e "${RED}✗${NC} Rust 版本: $rust_version (期望: 1.92.0)"
    ((failed_checks++))
fi

# 检查是否有数据库服务
if grep -q "postgres:" .github/workflows/ci.yml; then
    echo -e "${GREEN}✓${NC} PostgreSQL 测试环境已配置"
else
    echo -e "${RED}✗${NC} PostgreSQL 测试环境未配置"
    ((failed_checks++))
fi

if grep -q "redis:" .github/workflows/ci.yml; then
    echo -e "${GREEN}✓${NC} Redis 测试环境已配置"
else
    echo -e "${RED}✗${NC} Redis 测试环境未配置"
    ((failed_checks++))
fi

# 检查数据库版本
pg_version=$(grep "image: postgres:" .github/workflows/ci.yml | grep -o 'postgres:[0-9]*' | head -1)
if [[ "$pg_version" == "postgres:18" ]]; then
    echo -e "${GREEN}✓${NC} PostgreSQL 版本: $pg_version"
else
    echo -e "${YELLOW}⚠${NC} PostgreSQL 版本: $pg_version (推荐: postgres:18)"
fi

redis_version=$(grep "image: redis:" .github/workflows/ci.yml | grep -o 'redis:[0-9]*' | head -1)
if [[ "$redis_version" == "redis:8" ]]; then
    echo -e "${GREEN}✓${NC} Redis 版本: $redis_version"
else
    echo -e "${YELLOW}⚠${NC} Redis 版本: $redis_version (推荐: redis:8)"
fi

echo ""
echo "=== 检查 Release Workflow ==="

# 检查是否配置了 Docker 构建
if grep -q "build-docker:" .github/workflows/release.yml; then
    echo -e "${GREEN}✓${NC} Docker 镜像构建已配置"
else
    echo -e "${RED}✗${NC} Docker 镜像构建未配置"
    ((failed_checks++))
fi

# 检查是否配置了 Helm 发布
if grep -q "publish-helm:" .github/workflows/release.yml; then
    echo -e "${GREEN}✓${NC} Helm Charts 发布已配置"
else
    echo -e "${RED}✗${NC} Helm Charts 发布未配置"
    ((failed_checks++))
fi

# 计算需要构建的服务数量
service_count=$(grep -c "- api-gateway\|- analytics-service\|- crm-service" .github/workflows/release.yml || echo "0")
if [[ $service_count -gt 0 ]]; then
    echo -e "${GREEN}✓${NC} 配置了 Docker 镜像构建（找到 $service_count 个服务配置）"
fi

echo ""
echo "=== 检查 Security Scan Workflow ==="

# 统计安全扫描工具数量
scan_tools=0
grep -q "cargo-audit:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Cargo Audit"
grep -q "cargo-deny:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Cargo Deny"
grep -q "semgrep:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Semgrep"
grep -q "trivy-filesystem:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Trivy (Filesystem)"
grep -q "trivy-config:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Trivy (Config)"
grep -q "kubesec:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Kubesec"
grep -q "gitleaks:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} Gitleaks"
grep -q "codeql:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} CodeQL"
grep -q "sbom:" .github/workflows/security-scan.yml && ((scan_tools++)) && echo -e "${GREEN}✓${NC} SBOM"

echo ""
echo -e "${GREEN}安全扫描工具总数: $scan_tools${NC}"

echo ""
echo "=== 检查 Dependabot ==="

# 检查 Dependabot 生态系统
ecosystems=$(grep "package-ecosystem:" .github/dependabot.yml | wc -l | tr -d ' ')
echo -e "${GREEN}✓${NC} 配置了 $ecosystems 个生态系统"

if grep -q "cargo" .github/dependabot.yml; then
    echo -e "${GREEN}✓${NC} Cargo 依赖更新"
fi
if grep -q "github-actions" .github/dependabot.yml; then
    echo -e "${GREEN}✓${NC} GitHub Actions 更新"
fi
if grep -q "docker" .github/dependabot.yml; then
    echo -e "${GREEN}✓${NC} Docker 镜像更新"
fi

echo ""
echo "=========================================="
if [[ $failed_checks -eq 0 ]]; then
    echo -e "${GREEN}✓ 所有检查通过！${NC}"
    echo "=========================================="
    echo ""
    echo "GitHub 配置已完全就绪 🎉"
    echo ""
    echo "下一步:"
    echo "1. 配置 GitHub Secrets (如有需要)"
    echo "2. 设置分支保护规则"
    echo "3. 提交代码触发 CI"
    echo ""
    exit 0
else
    echo -e "${RED}✗ 发现 $failed_checks 个问题${NC}"
    echo "=========================================="
    exit 1
fi
