//! 接口层
//!
//! 包含 gRPC 和 REST API 接口实现

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "rest")]
pub mod rest;
