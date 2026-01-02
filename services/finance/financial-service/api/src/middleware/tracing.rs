//! 追踪中间件
//!
//! 提供 OpenTelemetry 追踪集成

use tracing::{Event, Id, Level, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// 设置追踪上下文
pub fn set_trace_context(span: &Span) {
    let otel_context = span.context();
    tracing_opentelemetry::set_parent_context(otel_context);
}

/// 从请求中提取追踪信息
pub fn extract_trace_id<T>(request: &tonic::Request<T>) -> Option<String> {
    request
        .metadata()
        .get("x-trace-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// 创建子 span
pub fn child_span<'a>(parent: &'a Span, name: &'a str) -> tracing::Span {
    tracing::span!(
        parent: parent.clone(),
        Level::INFO,
        "{}", name
    )
}

/// 记录事件带追踪信息
pub fn event_with_trace(event: &Event, span: &Span) {
    let trace_id = span.context().span().span_context().trace_id();
    let span_id = span.context().span().span_context().span_id();

    tracing::info!(
        trace_id = format!("{:032x}", trace_id),
        span_id = format!("{:016x}", span_id),
        message = %event.metadata().name(),
        "Event recorded"
    );
}
