//! 代理模块
//!
//! 实现 HTTP 和 gRPC 请求转发，支持协议转换

use crate::domain::{GatewayError, GatewayResult};
use bytes::Bytes;
use http::{header, StatusCode, Uri, Request, Response};
use hyper::body::Body;
use hyper_util::body::to_bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

/// HTTP 代理客户端
#[derive(Debug, Clone)]
pub struct HttpProxyClient {
    /// HTTP 客户端
    client: reqwest::Client,
    /// 连接超时
    connect_timeout: Duration,
    /// 读取超时
    read_timeout: Duration,
    /// 写入超时
    write_timeout: Duration,
}

impl HttpProxyClient {
    /// 创建新的 HTTP 代理客户端
    pub fn new(
        connect_timeout: u64,
        read_timeout: u64,
        write_timeout: u64,
    ) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(read_timeout))
                .connect_timeout(Duration::from_secs(connect_timeout))
                .build()
                .expect("Failed to create HTTP client"),
            connect_timeout: Duration::from_secs(connect_timeout),
            read_timeout: Duration::from_secs(read_timeout),
            write_timeout: Duration::from_secs(write_timeout),
        }
    }

    /// 发送 HTTP 请求
    pub async fn proxy_request(
        &self,
        target_url: &str,
        mut request: Request<Body>,
    ) -> GatewayResult<Response<Body>> {
        // 构建目标 URI
        let target_uri = self.build_uri(&request, target_url)?;

        // 设置目标主机
        request.headers_mut().insert(
            header::HOST,
            target_uri.host().unwrap_or("").parse().unwrap(),
        );

        // 添加 X-Forwarded-* 头
        if let Some(remote_addr) = request.extensions().get::<std::net::SocketAddr>() {
            request.headers_mut().insert(
                header::HeaderName::from_lowercase(b"x-forwarded-for").unwrap(),
                remote_addr.to_string().parse().unwrap(),
            );
        }

        debug!("Proxying request to: {}", target_uri);

        // 发送请求
        let response = self
            .client
            .request(request.method().clone(), target_uri)
            .body(to_bytes(request.into_body()).await.map_err(|e| GatewayError::ProxyError {
                reason: format!("Failed to read request body: {}", e),
            })?)
            .send()
            .await
            .map_err(|e| GatewayError::ProxyError {
                reason: format!("Failed to proxy request: {}", e),
            })?;

        // 转换响应
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await.map_err(|e| GatewayError::ProxyError {
            reason: format!("Failed to read response body: {}", e),
        })?;

        let mut builder = Response::builder().status(status);
        for (key, value) in headers {
            if let Some(key) = key {
                builder = builder.header(key, value);
            }
        }

        Ok(builder.body(Body::from(body)).map_err(|e| GatewayError::ProxyError {
            reason: format!("Failed to build response: {}", e),
        })?)
    }

    /// 构建目标 URI
    fn build_uri(&self, request: &Request<Body>, target_url: &str) -> Result<Uri, GatewayError> {
        let path = request.uri().path();
        let query = request.uri().query();

        let target_uri = if let Some(query) = query {
            format!("{}{}?{}", target_url, path, query)
        } else {
            format!("{}{}", target_url, path)
        };

        target_uri.parse().map_err(|e| GatewayError::ProxyError {
            reason: format!("Failed to parse URI: {}", e),
        })
    }
}

/// gRPC 代理客户端
#[derive(Debug, Clone)]
pub struct GrpcProxyClient {
    /// gRPC 通道缓存
    channels: Arc<RwLock<HashMap<String, tonic::transport::Channel>>>,
    /// 连接超时
    connect_timeout: Duration,
}

impl GrpcProxyClient {
    /// 创建新的 gRPC 代理客户端
    pub fn new(connect_timeout: u64) -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            connect_timeout: Duration::from_secs(connect_timeout),
        }
    }

    /// 获取 gRPC 通道
    pub async fn get_channel(&self, target: &str) -> GatewayResult<tonic::transport::Channel> {
        if let Some(channel) = self.channels.read().await.get(target).cloned() {
            return Ok(channel);
        }

        let channel = tonic::transport::Channel::from_shared(target.to_string())
            .map_err(|e| GatewayError::ProxyError {
                reason: format!("Failed to create gRPC channel: {}", e),
            })?
            .connect_timeout(self.connect_timeout)
            .connect()
            .await
            .map_err(|e| GatewayError::ProxyError {
                reason: format!("Failed to connect to gRPC service: {}", e),
            })?;

        self.channels.write().await.insert(target.to_string(), channel.clone());
        Ok(channel)
    }
}
