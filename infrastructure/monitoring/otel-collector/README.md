# OpenTelemetry Collector

KILLER ERP 的 OpenTelemetry Collector 配置，统一收集和处理遥测数据。

## 概述

OpenTelemetry Collector 是一个与供应商无关的代理，用于接收、处理和导出遥测数据。

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                   OpenTelemetry Collector                    │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │ Receivers│───▶│Processors│───▶│ Exporters│              │
│  └──────────┘    └──────────┘    └──────────┘              │
│       │                                  │                   │
│       ▼                                  ▼                   │
│  ┌─────────┐                      ┌───────────┐             │
│  │  OTLP   │                      │ Prometheus│             │
│  │ Jaeger  │                      │   Tempo   │             │
│  │ Zipkin  │                      │   Loki    │             │
│  └─────────┘                      └───────────┘             │
└─────────────────────────────────────────────────────────────┘
```

## 组件说明

### 接收器 (Receivers)

| 接收器 | 端口 | 数据类型 |
|--------|------|----------|
| OTLP gRPC | 4317 | 追踪、指标、日志 |
| OTLP HTTP | 4318 | 追踪、指标、日志 |
| Prometheus | - | 指标 |
| hostmetrics | - | 主机指标 |

### 处理器 (Processors)

| 处理器 | 用途 |
|--------|------|
| batch | 批处理，提高效率 |
| memory_limiter | 内存限制，防止 OOM |
| attributes | 添加/修改属性 |
| resourcedetection | 自动检测资源属性 |
| tail_sampling | 尾部采样 |

### 导出器 (Exporters)

| 导出器 | 目标 | 数据类型 |
|--------|------|----------|
| prometheus | Prometheus | 指标 |
| otlp/tempo | Tempo | 追踪 |
| loki | Loki | 日志 |

## 使用方式

### 本地开发

```bash
docker run -d \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 8888:8888 \
  -p 8889:8889 \
  -v $(pwd)/otel-config.yml:/etc/otelcol-contrib/config.yaml \
  otel/opentelemetry-collector-contrib:0.96.0 \
  --config=/etc/otelcol-contrib/config.yaml
```

### Kubernetes 部署

```bash
helm install otel-collector open-telemetry/opentelemetry-collector \
  -f values-otel.yaml
```

## 服��� SDK 配置

### 环境变量

```bash
# OTLP 端点
OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317

# 服务信息
OTEL_SERVICE_NAME=api-gateway
OTEL_RESOURCE_ATTRIBUTES=service.namespace=killer,deployment.environment=dev
```

### Rust 配置

```rust
use opentelemetry::global;
use opentelemetry_otlp::WithExportConfig;

// 追踪
let tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap())
    )
    .install_batch(opentelemetry_sdk::runtime::Tokio)?;

// 指标
let meter_provider = opentelemetry_otlp::new_pipeline()
    .metrics(opentelemetry_sdk::runtime::Tokio)
    .with_exporter(
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap())
    )
    .build()?;
```

## 采样策略

### 尾部采样配置

```yaml
tail_sampling:
  policies:
    # 保留所有错误
    - name: errors
      type: status_code
      status_code:
        status_codes: [ERROR]

    # 保留慢请求
    - name: slow-traces
      type: latency
      latency:
        threshold_ms: 1000

    # 概率采样其他请求
    - name: probabilistic
      type: probabilistic
      probabilistic:
        sampling_percentage: 10
```

## 监控端点

| 端点 | 端口 | 用途 |
|------|------|------|
| /metrics | 8888 | Collector 自身指标 |
| /health | 13133 | 健康检查 |
| /debug/pprof | 1888 | 性能分析 |

## 最佳实践

1. **资源限制**: 配置 memory_limiter 防止 OOM
2. **批处理**: 使用 batch 处理器提高效率
3. **采样策略**: 生产环境使用尾部采样
4. **高可用**: 部署多个 Collector 实例
5. **监控**: 监控 Collector 自身的指标
