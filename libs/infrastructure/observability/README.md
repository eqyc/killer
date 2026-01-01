# Observability

可观测性基础设施库，提供分布式追踪、指标采集和结构化日志的统一封装。基于 OpenTelemetry 标准，支持导出到多种后端（Jaeger, Prometheus, Loki 等）。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `TelemetryConfig` | 遥测配置 |
| `TracingLayer` | 追踪层配置 |
| `MetricsRecorder` | 指标记录器 |
| `LogConfig` | 日志配置 |
| `SpanExt` | Span 扩展方法 |
| `ContextPropagator` | 上下文传播器 |

## 使用示例

```text
// 初始化可观测性
Telemetry::builder()
    .service_name("order-service")
    .otlp_endpoint("http://otel-collector:4317")
    .prometheus_port(9090)
    .init()?;

// 分布式追踪
#[tracing::instrument]
async fn create_order(cmd: CreateOrderCommand) -> Result<OrderId> {
    tracing::info!(customer_id = %cmd.customer_id, "Creating order");
    // ...
}

// 指标采集
metrics::counter!("orders_created_total", "status" => "success").increment(1);
metrics::histogram!("order_processing_duration_seconds").record(duration);

// 结构化日志
tracing::info!(
    order_id = %order.id,
    amount = %order.total,
    "Order created successfully"
);
```

## 导出后端

| 类型 | 后端 | 端口 |
|------|------|------|
| Traces | Jaeger / Tempo | OTLP 4317 |
| Metrics | Prometheus | 9090 |
| Logs | Loki | OTLP 4317 |
