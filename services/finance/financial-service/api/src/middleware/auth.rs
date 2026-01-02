//! 认证授权中间件
//!
//! 从 JWT token 或 HTTP header 提取用户身份和租户信息

use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Code, Request, Status};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// JWT Claims 结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// 用户 ID (sub)
    pub sub: String,

    /// 租户 ID
    pub tenant_id: String,

    /// 用户角色列表
    #[serde(default)]
    pub roles: Vec<String>,

    /// Token 发行者
    pub iss: Option<String>,

    /// Token 受众
    pub aud: Option<String>,

    /// 过期时间
    pub exp: Option<i64>,

    /// 签发时间
    pub iat: Option<i64>,
}

/// 认证上下文 - 存储请求的身份信息
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// 用户 ID
    pub user_id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 用户角色
    pub roles: Vec<String>,

    /// JWT 原始 claims（可选）
    pub claims: Option<JwtClaims>,

    /// 请求时间
    pub timestamp: DateTime<Utc>,
}

impl AuthContext {
    /// 创建新的认证上下文
    pub fn new(user_id: Uuid, tenant_id: Uuid, roles: Vec<String>) -> Self {
        Self {
            user_id,
            tenant_id,
            roles,
            claims: None,
            timestamp: Utc::now(),
        }
    }

    /// 检查用户是否有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// 检查用户是否有任一指定角色
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }

    /// 检查用户是否有所有指定角色
    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|role| self.has_role(role))
    }

    /// 获取用户显示名称
    pub fn display_name(&self) -> String {
        format!("user:{}@tenant:{}", self.user_id, self.tenant_id)
    }
}

/// 认证拦截器 - 用于 gRPC 服务
#[derive(Clone)]
pub struct AuthInterceptor {
    /// JWT 解码密钥
    decoding_key: Arc<DecodingKey>,

    /// JWT 验证配置
    validation: Validation,

    /// 允许的签发者
    allowed_issuers: Vec<String>,

    /// 允许的受众
    allowed_audiences: Vec<String>,
}

impl AuthInterceptor {
    /// 创建新的认证拦截器
    pub fn new(
        jwt_secret: &str,
        allowed_issuers: Vec<String>,
        allowed_audiences: Vec<String>,
    ) -> Self {
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let validation = Validation {
            iss: None, // We'll validate manually
            aud: None, // We'll validate manually
            ..Validation::default()
        };

        Self {
            decoding_key: Arc::new(decoding_key),
            validation,
            allowed_issuers,
            allowed_audiences,
        }
    }

    /// 验证 JWT token
    fn validate_token(&self, token: &str) -> Result<JwtClaims, JwtError> {
        let claims: JwtClaims = decode(
            token,
            &self.decoding_key,
            &self.validation,
        )?;

        // Validate issuer
        if let Some(iss) = &claims.iss {
            if !self.allowed_issuers.is_empty() && !self.allowed_issuers.contains(iss) {
                return Err(JwtError::InvalidIssuer);
            }
        }

        // Validate audience
        if let Some(aud) = &claims.aud {
            if !self.allowed_audiences.is_empty() && !self.allowed_audiences.contains(aud) {
                return Err(JwtError::InvalidAudience);
            }
        }

        Ok(claims)
    }

    /// 从 metadata 提取并验证认证信息
    pub fn authenticate(&self, request: &Request<()>) -> Result<AuthContext, Status> {
        // 1. 尝试从 Authorization header 提取 JWT
        let auth_header = request.metadata().get("authorization");

        if let Some(auth_value) = auth_header {
            let auth_str = auth_value.to_str().map_err(|_| {
                Status::unauthenticated("Invalid authorization header encoding")
            })?;

            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return self.validate_jwt(token).map_err(|e| {
                    Status::unauthenticated(format!("Invalid token: {}", e))
                });
            }
        }

        // 2. 尝试从 X-User-Id header 提取（由 api-gateway 注入）
        let user_id_header = request.metadata().get("x-user-id");
        let tenant_id_header = request.metadata().get("x-tenant-id");
        let roles_header = request.metadata().get("x-roles");

        if let (Some(user_id_val), Some(tenant_id_val)) = (user_id_header, tenant_id_header) {
            let user_id = user_id_val.to_str()
                .ok()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| Status::invalid_argument("Invalid x-user-id"))?;

            let tenant_id = tenant_id_val.to_str()
                .ok()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| Status::invalid_argument("Invalid x-tenant-id"))?;

            let roles = roles_header
                .and_then(|v| v.to_str().ok())
                .map(|s| s.split(',').map(|r| r.trim().to_string()).collect())
                .unwrap_or_default();

            debug!(%user_id, %tenant_id, roles=?roles, "Authenticated via headers");

            return Ok(AuthContext::new(user_id, tenant_id, roles));
        }

        // 3. 如果都不存在，返回未认证错误
        Err(Status::unauthenticated("Missing authentication"))
    }

    /// 验证 JWT 并返回认证上下文
    fn validate_jwt(&self, token: &str) -> Result<AuthContext, JwtError> {
        let claims = self.validate_token(token)?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| JwtError::InvalidSignature)?; // Use InvalidSignature as fallback

        let tenant_id = Uuid::parse_str(&claims.tenant_id)
            .map_err(|_| JwtError::InvalidSignature)?;

        debug!(%user_id, %tenant_id, roles=?claims.roles, "Authenticated via JWT");

        Ok(AuthContext {
            user_id,
            tenant_id,
            roles: claims.roles.clone(),
            claims: Some(claims),
            timestamp: Utc::now(),
        })
    }
}

/// 从请求中提取认证上下文的辅助函数
pub fn extract_auth_context(request: &Request<()>) -> Result<AuthContext, Status> {
    // 从请求扩展中获取（如果已由拦截器设置）
    if let Some(ctx) = request.extensions().get::<AuthContext>() {
        return Ok(ctx.clone());
    }

    Err(Status::unauthenticated("Authentication context not found"))
}

/// 权限检查中间件
pub struct PermissionChecker {
    /// 必需的角色
    required_roles: Vec<String>,
}

impl PermissionChecker {
    pub fn new(required_roles: Vec<String>) -> Self {
        Self { required_roles }
    }

    pub fn check(&self, auth: &AuthContext) -> Result<(), Status> {
        if self.required_roles.is_empty() {
            return Ok(());
        }

        let has_permission = self.required_roles.iter().any(|role| {
            auth.roles.iter().any(|r| r == role || r == format!("finance:{}", role))
        });

        if !has_permission {
            return Err(Status::permission_denied(format!(
                "Missing required permissions: {:?}",
                self.required_roles
            )));
        }

        Ok(())
    }
}

/// 认证装饰器 - 用于包装服务处理函数
pub fn with_auth<T, F>(
    auth: &AuthContext,
    required_roles: Vec<String>,
    f: F,
) -> Result<T, Status>
where
    F: FnOnce() -> Result<T, Status>,
{
    // 检查权限
    let checker = PermissionChecker::new(required_roles);
    checker.check(auth)?;

    f()
}
