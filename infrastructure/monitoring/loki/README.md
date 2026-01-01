# Loki 日志聚合

KILLER ERP 的 Loki 日志收集和查询配置。

## 概述

Loki 是一个水平可扩展、高可用的日志聚合系统，专为 Kubernetes 环境设计。

## 架构

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Promtail   │────▶│    Loki     │◀────│   Grafana   │
│  (采集器)   │     │  (存储/查询) │     │  (可视化)   │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │
       ▼                   ▼
┌─────────────┐     ┌─────────────┐
│  Pod 日志   │     │  对象存储   │
│  /var/log   │     │  (S3/MinIO) │
└─────────────┘     └─────────────┘
```

## 配置说明

### 存储模式

| 模式 | 适用场景 | 说明 |
|------|----------|------|
| filesystem | 开发/测试 | 本地文件系统 |
| s3 | 生产 | AWS S3 或兼容存储 |
| gcs | 生产 | Google Cloud Storage |

### 数据保留

```yaml
limits_config:
  retention_period: 720h  # 30 天
```

## 使用方式

### 本地开发

```bash
docker run -d \
  -p 3100:3100 \
  -v $(pwd)/loki-config.yml:/etc/loki/local-config.yaml \
  grafana/loki:2.9.0 \
  -config.file=/etc/loki/local-config.yaml
```

### Kubernetes 部署

```bash
helm install loki grafana/loki \
  -f values-loki.yaml
```

## LogQL 查询

### 基础查询

```logql
# 查看特定服务日志
{service="api-gateway"}

# 过滤错误日志
{service="api-gateway"} |= "error"

# 正则匹配
{service="api-gateway"} |~ "status=[45].."

# JSON 解析
{service="api-gateway"} | json | level="error"
```

### 聚合查询

```logql
# 错误日志计数
count_over_time({service="api-gateway"} |= "error" [5m])

# 按服务分组
sum by (service) (count_over_time({namespace="killer-system"} |= "error" [1h]))

# 日志速率
rate({service="api-gateway"}[5m])
```

## 日志格式

### 推荐格式

```json
{
  "timestamp": "2024-01-01T00:00:00Z",
  "level": "info",
  "service": "api-gateway",
  "trace_id": "abc123",
  "span_id": "def456",
  "message": "Request processed",
  "duration_ms": 42,
  "status_code": 200
}
```

### 标签策略

| 标签 | 说明 | 示例 |
|------|------|------|
| namespace | K8s 命名空间 | killer-system |
| service | 服务名称 | api-gateway |
| pod | Pod 名称 | api-gateway-xxx |
| container | 容器名称 | api-gateway |

## 最佳实践

1. **标签数量**: 控制标签基数，避免高基数标签
2. **日志格式**: 使用结构化日志 (JSON)
3. **保留策略**: 根据合规要求设置保留时间
4. **查询优化**: 使用标签过滤减少扫描范围
5. **告警规则**: 使用 Ruler 配置日志告警
