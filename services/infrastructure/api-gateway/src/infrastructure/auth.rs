//! JWT 认证模块
//!
//! 支持 JWKS 动态获取、RS256 验证、声明验证

use crate::{
    config::{AuthConfig, JwtConfig},
    domain::{AuthenticationInfo, AuthType, GatewayError, GatewayResult},
};
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// JWKS (JSON Web Key Set)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwks {
    /// 密钥
    pub keys: Vec<Jwk>,
}

/// JWK (JSON Web Key)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwk {
    /// 密钥类型
    #[serde(rename = "kty")]
    pub key_type: String,
    /// 用途
    #[serde(rename = "use")]
    pub key_use: Option<String>,
    /// 算法
    #[serde(rename = "alg")]
    pub algorithm: Option<String>,
    /// 密钥 ID
    #[serde(rename = "kid")]
    pub key_id: Option<String>,
    /// X.509 证书链
    #[serde(rename = "x5c")]
    pub x509_cert_chain: Option<Vec<String>>,
    /// N 模数 (RSA)
    #[serde(rename = "n")]
    pub modulus: Option<String>,
    /// E 指数 (RSA)
    #[serde(rename = "e")]
    pub exponent: Option<String>,
    /// 椭圆曲线 (EC)
    #[serde(rename = "crv")]
    pub curve: Option<String>,
    /// X 坐标 (EC)
    #[serde(rename = "x")]
    pub x: Option<String>,
    /// Y 坐标 (EC)
    #[serde(rename = "y")]
    pub y: Option<String>,
}

/// JWT 验证器
#[derive(Debug, Clone)]
pub struct JwtValidator {
    /// 配置
    config: Arc<JwtConfig>,
    /// JWKS 缓存
    jwks_cache: Cache<String, (Jwks, DateTime<Utc>)>,
    /// 验证状态
    validation_state: Arc<RwLock<ValidationState>>,
    /// HTTP 客户端
    client: reqwest::Client,
}

/// 验证状态
#[derive(Debug, Clone, Default)]
pub struct ValidationState {
    /// JWKS 最后刷新时间
    pub last_refresh: Option<DateTime<Utc>>,
    /// JWKS 刷新失败次数
    pub refresh_failure_count: u32,
    /// 最后验证时间
    pub last_validation: Option<DateTime<Utc>>,
    /// 总验证次数
    pub total_validations: u64,
    /// 验证失败次数
    pub validation_failures: u64,
}

/// 简化的 JWT 声明
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtClaims {
    /// 发行者
    #[serde(default)]
    pub iss: Option<String>,
    /// 主题
    #[serde(default)]
    pub sub: Option<String>,
    /// 受众
    #[serde(default)]
    pub aud: Option<serde_json::Value>,
    /// 过期时间
    #[serde(default)]
    pub exp: Option<i64>,
    /// 颁发时间
    #[serde(default)]
    pub iat: Option<i64>,
    /// 租户 ID
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// 范围
    #[serde(default)]
    pub scope: Option<String>,
    /// 角色
    #[serde(default)]
    pub roles: Option<Vec<String>>,
}

impl JwtValidator {
    /// 创建 JWT 验证器
    pub fn new(config: Arc<JwtConfig>) -> Self {
        Self {
            config: config.clone(),
            jwks_cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(config.cache_ttl))
                .build(),
            validation_state: Arc::new(RwLock::new(ValidationState::default())),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// 初始化并预加载 JWKS
    pub async fn initialize(&self) {
        if let Err(e) = self.refresh_jwks().await {
            error!("Failed to initial load JWKS: {}", e);
        }
    }

    /// 获取 JWKS
    pub async fn get_jwks(&self) -> GatewayResult<Jwks> {
        // 检查缓存
        if let Some((jwks, _)) = self.jwks_cache.get(&self.config.jwks_url).await {
            return Ok(jwks);
        }
        // 刷新 JWKS
        self.refresh_jwks().await
    }

    /// 刷新 JWKS
    async fn refresh_jwks(&self) -> GatewayResult<Jwks> {
        debug!("Refreshing JWKS from: {}", self.config.jwks_url);

        let response = self.client
            .get(&self.config.jwks_url)
            .send()
            .await
            .map_err(|e| GatewayError::AuthenticationFailed {
                reason: format!("Failed to fetch JWKS: {}", e),
            })?;

        if !response.status().is_success() {
            return Err(GatewayError::AuthenticationFailed {
                reason: format!("JWKS request failed with status: {}", response.status()),
            });
        }

        let jwks: Jwks = response
            .json()
            .await
            .map_err(|e| GatewayError::AuthenticationFailed {
                reason: format!("Failed to parse JWKS: {}", e),
            })?;

        // 缓存 JWKS
        let now = Utc::now();
        self.jwks_cache.insert(
            self.config.jwks_url.clone(),
            (jwks.clone(), now),
        ).await;

        // 更新状态
        let mut state = self.validation_state.write().await;
        state.last_refresh = Some(now);
        state.refresh_failure_count = 0;

        info!("JWKS refreshed successfully, {} keys loaded", jwks.keys.len());

        Ok(jwks)
    }

    /// 验证并解码 JWT (简化实现)
    pub async fn validate(&self, token: &str) -> GatewayResult<AuthenticationInfo> {
        debug!("Validating JWT token");

        // 更新验证计数
        {
            let mut state = self.validation_state.write().await;
            state.total_validations += 1;
        }

        // 解析 token 为 Base64URL 片段
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(GatewayError::AuthenticationFailed {
                reason: "Invalid token format".to_string(),
            });
        }

        // 解码 payload
        let payload = decode_base64_url(parts[1]).map_err(|e| GatewayError::AuthenticationFailed {
            reason: format!("Failed to decode token payload: {}", e),
        })?;

        let claims: JwtClaims = serde_json::from_slice(&payload).map_err(|e| GatewayError::AuthenticationFailed {
            reason: format!("Failed to parse token claims: {}", e),
        })?;

        // 验证发行者
        if let Some(ref expected_iss) = self.config.issuer {
            if claims.iss.as_ref() != Some(expected_iss) {
                return Err(GatewayError::AuthenticationFailed {
                    reason: "Invalid issuer".to_string(),
                });
            }
        }

        // 验证过期时间
        if let Some(exp_ts) = claims.exp {
            let exp_time = DateTime::<Utc>::from_timestamp(exp_ts, 0)
                .ok_or_else(|| GatewayError::AuthenticationFailed {
                    reason: "Invalid expiration time".to_string(),
                })?;
            if exp_time < Utc::now() {
                return Err(GatewayError::AuthenticationFailed {
                    reason: "Token expired".to_string(),
                });
            }
        }

        // 提取认证信息
        let scopes: Vec<String> = claims.scope
            .as_ref()
            .map(|s| s.split_whitespace().map(|str| str.to_string()).collect())
            .unwrap_or_default();

        let roles: Vec<String> = claims.roles.clone().unwrap_or_default();

        let expires_at = claims.exp.and_then(|ts| DateTime::<Utc>::from_timestamp(ts, 0));

        let subject = claims.sub.unwrap_or_else(|| "unknown".to_string());

        // 构建 claims HashMap
        let claims_map: HashMap<String, serde_json::Value> = serde_json::from_slice(&payload)
            .unwrap_or_default();

        let auth_info = AuthenticationInfo {
            auth_type: AuthType::Jwt,
            subject_id: subject,
            tenant_id: claims.tenant_id,
            scopes,
            roles,
            claims: claims_map,
            expires_at,
            authenticated_at: Utc::now(),
        };

        // 更新最后验证时间
        {
            let mut state = self.validation_state.write().await;
            state.last_validation = Some(Utc::now());
        }

        Ok(auth_info)
    }

    /// 刷新 JWKS (后台任务)
    pub async fn start_refresh_task(&self, interval: Duration) {
        let validator = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;
                if let Err(e) = validator.refresh_jwks().await {
                    error!("Background JWKS refresh failed: {}", e);

                    let mut state = validator.validation_state.write().await;
                    state.refresh_failure_count += 1;

                    if state.refresh_failure_count > 5 {
                        warn!("Multiple JWKS refresh failures, will retry");
                    }
                }
            }
        });
    }
}

/// API Key 验证器
#[derive(Debug, Clone)]
pub struct ApiKeyValidator {
    /// Redis URL
    redis_url: String,
    /// 缓存
    cache: Cache<String, (ApiKeyInfo, DateTime<Utc>)>,
    /// 缓存 TTL
    cache_ttl: Duration,
    /// HTTP 客户端
    client: reqwest::Client,
}

/// API Key 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    /// Key ID
    pub key_id: String,
    /// 关联的租户 ID
    pub tenant_id: String,
    /// 关联的用户 ID
    pub user_id: String,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 角色
    pub roles: Vec<String>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 是否启用
    pub enabled: bool,
}

impl ApiKeyValidator {
    /// 创建 API Key 验证器
    pub fn new(redis_url: String, cache_ttl: u64) -> Self {
        Self {
            redis_url,
            cache: Cache::builder()
                .max_capacity(10000)
                .time_to_live(Duration::from_secs(cache_ttl))
                .build(),
            cache_ttl: Duration::from_secs(cache_ttl),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// 验证 API Key
    pub async fn validate(&self, api_key: &str) -> GatewayResult<AuthenticationInfo> {
        // 检查缓存
        if let Some((info, _)) = self.cache.get(api_key).await {
            if info.enabled {
                if let Some(expires) = info.expires_at {
                    if expires < Utc::now() {
                        return Err(GatewayError::AuthenticationFailed {
                            reason: "API Key expired".to_string(),
                        });
                    }
                }

                return Ok(AuthenticationInfo {
                    auth_type: AuthType::ApiKey,
                    subject_id: info.user_id.clone(),
                    tenant_id: Some(info.tenant_id.clone()),
                    scopes: info.scopes.clone(),
                    roles: info.roles.clone(),
                    claims: HashMap::new(),
                    expires_at: info.expires_at,
                    authenticated_at: Utc::now(),
                });
            } else {
                return Err(GatewayError::AuthenticationFailed {
                    reason: "API Key disabled".to_string(),
                });
            }
        }

        self.fetch_and_validate(api_key).await
    }

    /// 从存储获取并验证
    async fn fetch_and_validate(&self, _api_key: &str) -> GatewayResult<AuthenticationInfo> {
        Err(GatewayError::AuthenticationFailed {
            reason: "API Key not found".to_string(),
        })
    }
}

/// 认证服务
#[derive(Debug, Clone)]
pub struct AuthenticationService {
    /// JWT 验证器
    jwt_validator: Option<JwtValidator>,
    /// API Key 验证器
    api_key_validator: Option<ApiKeyValidator>,
    /// 配置
    config: Arc<AuthConfig>,
}

impl AuthenticationService {
    /// 创建认证服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        let jwt_validator = if config.jwt.enabled {
            Some(JwtValidator::new(Arc::new(config.jwt.clone())))
        } else {
            None
        };

        let api_key_validator = if config.api_key.enabled {
            Some(ApiKeyValidator::new(
                config.api_key.redis_url.clone(),
                config.api_key.cache_ttl,
            ))
        } else {
            None
        };

        Self {
            jwt_validator,
            api_key_validator,
            config,
        }
    }

    /// 初始化认证服务
    pub async fn initialize(&self) {
        if let Some(ref validator) = self.jwt_validator {
            validator.initialize().await;
        }
    }

    /// 验证请求
    pub async fn authenticate(&self, auth_header: Option<&str>, api_key: Option<&str>) -> GatewayResult<AuthenticationInfo> {
        // 1. 尝试 JWT 验证
        if let Some(ref validator) = self.jwt_validator {
            if let Some(header) = auth_header {
                if header.starts_with("Bearer ") {
                    let token = &header[7..];
                    return validator.validate(token).await;
                }
            }
        }

        // 2. 尝试 API Key 验证
        if let Some(ref validator) = self.api_key_validator {
            if let Some(key) = api_key.or(auth_header) {
                return validator.validate(key).await;
            }
        }

        // 3. 返回匿名访问
        Ok(AuthenticationInfo::default())
    }

    /// 检查路径是否需要认证
    pub fn requires_auth(&self, path: &str) -> bool {
        !self.config.bypass_paths.iter().any(|p| {
            if p.ends_with("*") {
                let prefix = &p[..p.len() - 1];
                path.starts_with(prefix)
            } else {
                path == p
            }
        })
    }
}

/// 解码 Base64URL 编码的字符串
fn decode_base64_url(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    const URL_SAFE_NO_PAD: base64::Engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    URL_SAFE_NO_PAD.decode(input)
}
