# Kubernetes 配置

KILLER ERP 的 Kubernetes 部署配置，支持 Kustomize 和 Helm 两种部署方式。

## 目录结构

```
k8s/
├── base/                    # Kustomize 基础配置
│   ├── common/              # 公共资源 (ConfigMap, Secret, NetworkPolicy)
│   ├── infrastructure/      # 基础设施服务
│   ├── finance/             # 财务域服务
│   ├── logistics/           # 物流域服务
│   └── kustomization.yaml   # 基础 Kustomization
├── overlays/                # 环境特定配置
│   ├── dev/                 # 开发环境
│   ├── staging/             # 预发布环境
│   └── production/          # 生产环境
└── helm/                    # Helm Chart
    └── killer/
        ├── Chart.yaml
        ├── values.yaml
        ├── charts/          # 子 Chart
        └── templates/       # 模板文件
```

## Kustomize 部署

### 开发环境

```bash
# 预览资源
kubectl kustomize overlays/dev

# 部署
kubectl apply -k overlays/dev

# 查看状态
kubectl get pods -n killer-system-dev
```

### 预发布环境

```bash
kubectl apply -k overlays/staging
```

### 生产环境

```bash
# 生产环境建议使用 GitOps
kubectl apply -k overlays/production
```

## Helm 部署

### 安装

```bash
# 添加依赖仓库
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo add grafana https://grafana.github.io/helm-charts
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update

# 更新依赖
cd helm/killer
helm dependency update

# 安装
helm install killer . -n killer-system --create-namespace
```

### 自定义配置

```bash
# 使用自定义 values
helm install killer . -f values-production.yaml

# 覆盖单个值
helm install killer . --set apiGateway.replicaCount=3
```

### 升级

```bash
helm upgrade killer . -n killer-system
```

### 卸载

```bash
helm uninstall killer -n killer-system
```

## 环境配置对比

| 配置项 | Dev | Staging | Production |
|--------|-----|---------|------------|
| 副本数 | 1 | 2 | 3+ |
| 日志级别 | debug | info | warn |
| 资源限制 | 宽松 | 适中 | 严格 |
| HPA | 禁用 | 可选 | 启用 |
| PDB | 无 | 可选 | 必须 |

## 资源管理

### ConfigMap

- `killer-common-config`: 通用应用配置
- `killer-feature-flags`: 功能开关
- `killer-cors-config`: CORS 配置

### Secret

建议使用外部 Secret 管理方案:

- [External Secrets Operator](https://external-secrets.io/)
- [HashiCorp Vault](https://www.vaultproject.io/)
- [Sealed Secrets](https://sealed-secrets.netlify.app/)

### 网络策略

默认启用以下策略:

1. `default-deny-all`: 拒绝所有入站流量
2. `allow-api-gateway-ingress`: 允许 Ingress 访问 API Gateway
3. `allow-internal-services`: 允许服务间通信
4. `allow-monitoring`: 允许监控系统访问

## 监控集成

### Prometheus 指标

所有服务暴露 `/metrics` 端点，端口 9090。

### 日志收集

使用 Promtail 收集日志，发送到 Loki。

### 分布式追踪

使用 OpenTelemetry SDK，追踪数据发送到 Tempo。

## 故障排查

### Pod 无法启动

```bash
# 查看 Pod 状态
kubectl describe pod <pod-name> -n killer-system

# 查看日志
kubectl logs <pod-name> -n killer-system

# 查看事件
kubectl get events -n killer-system --sort-by='.lastTimestamp'
```

### 服务无法访问

```bash
# 检查 Service
kubectl get svc -n killer-system

# 检查 Endpoints
kubectl get endpoints -n killer-system

# 测试连接
kubectl run test --rm -it --image=busybox -- wget -qO- http://api-gateway:8000/health
```

### 资源不足

```bash
# 查看节点资源
kubectl top nodes

# 查看 Pod 资源
kubectl top pods -n killer-system
```

## 最佳实践

1. **GitOps**: 使用 Argo CD 或 Flux 进行持续部署
2. **Secret 管理**: 不要将敏感信息提交到 Git
3. **资源限制**: 始终设置 requests 和 limits
4. **健康检查**: 配置 liveness 和 readiness 探针
5. **PDB**: 生产环境必须配置 Pod Disruption Budget
6. **网络策略**: 遵循最小权限原则
