#!/usr/bin/env bash
# GitHub Secrets 交互式配置脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "=========================================="
echo "GitHub Secrets 配置向导"
echo "=========================================="
echo ""

# 检查 GitHub CLI
if ! command -v gh &> /dev/null; then
    echo -e "${RED}错误: 未找到 GitHub CLI (gh)${NC}"
    echo ""
    echo "请先安装 GitHub CLI:"
    echo "  macOS:  brew install gh"
    echo "  Linux:  https://github.com/cli/cli#installation"
    echo ""
    exit 1
fi

# 检查是否已登录
if ! gh auth status &> /dev/null; then
    echo -e "${YELLOW}请先登录 GitHub CLI:${NC}"
    gh auth login
fi

echo -e "${GREEN}✓${NC} GitHub CLI 已就绪"
echo ""

# 获取当前仓库信息
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
echo -e "当前仓库: ${BLUE}$REPO${NC}"
echo ""

# 显示当前已配置的 Secrets
echo "=========================================="
echo "当前已配置的 Secrets:"
echo "=========================================="
gh secret list
echo ""

# 询问用户想要配置哪些 Secrets
echo "=========================================="
echo "选择要配置的 Secrets:"
echo "=========================================="
echo ""

echo "1. Kubernetes 部署配置"
echo "   - KUBE_CONFIG_STAGING (Staging 环境部署)"
echo "   - KUBE_CONFIG_PRODUCTION (Production 环境部署)"
echo ""

echo "2. Gitleaks 许可证"
echo "   - GITLEAKS_LICENSE (可选，启用 Pro 功能)"
echo ""

echo "3. Docker Hub 凭证"
echo "   - DOCKERHUB_USERNAME"
echo "   - DOCKERHUB_TOKEN"
echo ""

echo "4. 通知 Webhook"
echo "   - SLACK_WEBHOOK_URL"
echo "   - DISCORD_WEBHOOK_URL"
echo ""

# 函数：配置 Secret
configure_secret() {
    local secret_name=$1
    local description=$2
    local is_file=${3:-false}

    echo ""
    echo -e "${YELLOW}配置 $secret_name${NC}"
    echo "说明: $description"
    echo ""

    # 检查是否已存在
    if gh secret list | grep -q "^$secret_name"; then
        echo -e "${YELLOW}⚠ Secret '$secret_name' 已存在${NC}"
        read -p "是否要更新它? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "跳过 $secret_name"
            return
        fi
    fi

    if [ "$is_file" = true ]; then
        # 从文件读取
        read -p "请输入文件路径: " file_path
        if [ ! -f "$file_path" ]; then
            echo -e "${RED}✗ 文件不存在: $file_path${NC}"
            return
        fi

        # Base64 编码（如果是 kubeconfig）
        if [[ $secret_name == KUBE_CONFIG* ]]; then
            echo "正在 base64 编码..."
            cat "$file_path" | base64 | gh secret set "$secret_name"
        else
            gh secret set "$secret_name" < "$file_path"
        fi
    else
        # 交互式输入
        echo "请输入 $secret_name 的值:"
        echo "(输入完成后按 Ctrl+D)"
        gh secret set "$secret_name"
    fi

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Secret '$secret_name' 配置成功"
    else
        echo -e "${RED}✗${NC} Secret '$secret_name' 配置失败"
    fi
}

# 配置向导
while true; do
    echo ""
    echo "选择要配置的选项 (输入数字，或 q 退出):"
    echo "  1) KUBE_CONFIG_STAGING"
    echo "  2) KUBE_CONFIG_PRODUCTION"
    echo "  3) GITLEAKS_LICENSE"
    echo "  4) DOCKERHUB_USERNAME"
    echo "  5) DOCKERHUB_TOKEN"
    echo "  6) SLACK_WEBHOOK_URL"
    echo "  7) DISCORD_WEBHOOK_URL"
    echo "  8) 查看所有 Secrets"
    echo "  9) 删除 Secret"
    echo "  q) 退出"
    echo ""
    read -p "请选择: " choice

    case $choice in
        1)
            configure_secret "KUBE_CONFIG_STAGING" \
                "Staging 环境的 kubeconfig（会自动 base64 编码）" \
                true
            ;;
        2)
            configure_secret "KUBE_CONFIG_PRODUCTION" \
                "Production 环境的 kubeconfig（会自动 base64 编码）" \
                true
            ;;
        3)
            configure_secret "GITLEAKS_LICENSE" \
                "Gitleaks Pro 许可证密钥（可选）" \
                false
            ;;
        4)
            configure_secret "DOCKERHUB_USERNAME" \
                "Docker Hub 用户名" \
                false
            ;;
        5)
            configure_secret "DOCKERHUB_TOKEN" \
                "Docker Hub Access Token" \
                false
            ;;
        6)
            configure_secret "SLACK_WEBHOOK_URL" \
                "Slack Incoming Webhook URL" \
                false
            ;;
        7)
            configure_secret "DISCORD_WEBHOOK_URL" \
                "Discord Webhook URL" \
                false
            ;;
        8)
            echo ""
            echo "=========================================="
            echo "当前所有 Secrets:"
            echo "=========================================="
            gh secret list
            ;;
        9)
            echo ""
            read -p "请输入要删除的 Secret 名称: " secret_name
            if [ -n "$secret_name" ]; then
                gh secret delete "$secret_name"
                if [ $? -eq 0 ]; then
                    echo -e "${GREEN}✓${NC} Secret '$secret_name' 已删除"
                else
                    echo -e "${RED}✗${NC} 删除失败"
                fi
            fi
            ;;
        q|Q)
            break
            ;;
        *)
            echo -e "${RED}无效选项${NC}"
            ;;
    esac
done

echo ""
echo "=========================================="
echo -e "${GREEN}配置完成！${NC}"
echo "=========================================="
echo ""
echo "最终配置的 Secrets:"
gh secret list
echo ""
echo "下一步:"
echo "1. 运行验证脚本: ./scripts/check-github-config.sh"
echo "2. 提交代码触发 CI 测试"
echo "3. 查看文档: docs/GITHUB-SECRETS-SETUP.md"
echo ""
