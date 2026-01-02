//! 接口层
//!
//! 提供 HTTP 和 gRPC 接口适配器

pub mod http;

// Re-exports
pub use http::{HttpHandler, HttpServer, build_cors_layer, build_security_headers_layer};
