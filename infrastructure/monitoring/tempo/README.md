# Tempo 分布式追踪

KILLER ERP 的 Tempo 分布式追踪配置。

## 概述

Tempo 是 Grafana Labs 开发的高性能分布式追踪后端，支持多种追踪协议。

## 支持的协议

| 协议 | 端口 | 说明 |
|------|------|------|
| OTLP gRPC | 4317 | OpenTelemetry 推荐 |
| OTLP HTTP | 4318 | OpenTelemetry HTTP |
| Jaeger gRPC | 14250 | Jaeger 兼容 |
| Jaeger Thrift | 14268 | Jaeger HTTP |
| Zipkin | 9411 | Zipkin 兼容 |

## 架构

```
┌─────────────┐     ┌─────────────┐     ┌──────────���──┐
│   服务      │────▶│ OTel Collector│───▶│   Tempo     │
│ (SDK 埋点)  │     │  (可选)      │     │  (存储)     │
└─────────────┘     └─────────────┘     └─────────────┘
                                               │
                                               ▼
                                        ┌─────────────┐
                                        │   Grafana   │
                                        │  (可视化)   │
                                        └─────────────┘
```

## 配置说明

### 存储模式

| 模式 | 适用场景 | 保留时间 |
|------|----------|----------|
| local | 开发/测试 | 48h |
| s3 | 生产 | 可配置 |
| gcs | 生产 | 可配置 |

### 采样策略

```yaml
# 尾部采样 (在 OTel Collector 配置)
processors:
  tail_sampling:
    decision_wait: 10s
    policies:
      - name: errors
        type: status_code
        status_code: {status_codes: [ERROR]}
      - name: slow-traces
        type: latency
        latency: {threshold_ms: 1000}
```

## 使用方式

### 本地开发

```bash
docker run -d \
  -p 3200:3200 \
  -p 4317:4317 \
  -p 4318:4318 \
  -v $(pwd)/tempo-config.yml:/etc/tempo/tempo.yaml \
  grafana/tempo:2.4.0 \
  -config.file=/etc/tempo/tempo.yaml
```

### Kubernetes 部署

```bash
helm install tempo grafana/tempo \
  -f values-tempo.yaml
```

## 服务端 SDK 配置

### Rust (OpenTelemetry)

```rust
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::prelude::*;

fn init_tracing() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://tempo:4317")
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .init();
}
```

## Grafana 集成

### 数据源配置

```yaml
datasources:
  - name: Tempo
    type: tempo
    url: http://tempo:3200
    jsonData:
      tracesToLogs:
        datasourceUid: loki
        tags: ['service.name']
      tracesToMetrics:
        datasourceUid: prometheus
      serviceMap:
        datasourceUid: prometheus
```

### TraceQL 查询

```
# 查找特定服务的追踪
{resource.service.name="api-gateway"}

# 查找错误追踪
{status=error}

# 查找慢请求
{duration>1s}

# 组合查询
{resource.service.name="api-gateway" && status=error && duration>500ms}
```

## 最佳实践

1. **采样策略**: 生产环境使用尾部采样
2. **上下文传播**: 确保正确传播 trace context
3. **属性规范**: 遵循 OpenTelemetry 语义约定
4. **关联查询**: 配置 traces-to-logs 和 traces-to-metrics
5. **存储优化**: 根据流量调整保留时间
