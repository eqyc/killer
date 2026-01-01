# Prometheus 监控

KILLER ERP 的 Prometheus 指标收集和告警配置。

## 文件说明

| 文件 | 用途 |
|------|------|
| `prometheus.yml` | Prometheus 主配置文件 |
| `alerts.yml` | 告警规则定义 |

## 指标收集

### 服务发现

使用 Kubernetes 服务发现自动发现 Pod:

```yaml
kubernetes_sd_configs:
  - role: pod
    namespaces:
      names:
        - killer-system
```

### 抓取目标

| 目标 | 端口 | 路径 |
|------|------|------|
| 微服务 | 9090 | /metrics |
| PostgreSQL | 9187 | /metrics |
| Redis | 9121 | /metrics |
| Kafka | 9308 | /metrics |

### 服务注解

为 Pod 添加以下注解启用指标抓取:

```yaml
annotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "9090"
  prometheus.io/path: "/metrics"
```

## 告警规则

### 告警级别

| 级别 | 说明 | 响应时间 |
|------|------|----------|
| critical | 严重告警，需立即处理 | < 5 分钟 |
| warning | 警告，需关注 | < 30 分钟 |
| info | 信息，供参考 | - |

### 告警分类

1. **服务可用性**: 服务宕机、高错误率、高延迟
2. **资源告警**: CPU、内存、磁盘
3. **数据库告警**: 连接数、慢查询、复制延迟
4. **消息队列告警**: 消费延迟、Broker 状态
5. **业务告警**: 订单处理、支付超时

## 使用方式

### 本地开发

```bash
# 启动 Prometheus
docker run -d \
  -p 9090:9090 \
  -v $(pwd)/prometheus.yml:/etc/prometheus/prometheus.yml \
  -v $(pwd)/alerts.yml:/etc/prometheus/alerts.yml \
  prom/prometheus
```

### Kubernetes 部署

```bash
# 使用 Helm
helm install prometheus prometheus-community/prometheus \
  -f values-prometheus.yaml

# 使用 Kustomize
kubectl apply -k monitoring/prometheus/
```

## 常用 PromQL

### 服务健康

```promql
# 服务可用性
up{job=~"killer-.*"}

# 请求速率
sum(rate(http_requests_total[5m])) by (service)

# 错误率
sum(rate(http_requests_total{status=~"5.."}[5m])) by (service)
/
sum(rate(http_requests_total[5m])) by (service)

# P95 延迟
histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service))
```

### 资源使用

```promql
# CPU 使用率
sum(rate(container_cpu_usage_seconds_total[5m])) by (pod)

# 内存使用
container_memory_usage_bytes / container_spec_memory_limit_bytes
```

## 最佳实践

1. **标签规范**: 使用统一的标签命名
2. **告警分级**: 合理设置告警级别
3. **告警抑制**: 避免告警风暴
4. **记录规则**: 预计算常用查询
5. **数据保留**: 根据需求设置保留时间
