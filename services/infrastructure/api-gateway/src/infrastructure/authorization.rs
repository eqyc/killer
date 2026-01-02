//! 权限控制模块
//!
//! 实现 RBAC (基于角色的访问控制) 和 ABAC (基于属性的访问控制)

use crate::{
    config::{AuthorizationConfig, RbacRule},
    domain::{AuthenticationInfo, GatewayError, GatewayResult},
};
use async_trait::async_trait;
use moka::future::Cache;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc, time::Duration};
use tracing::debug;

/// 权限决策
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationDecision {
    /// 允许
    Allow,
    /// 拒绝
    Deny,
    /// 不适用 (无匹配规则)
    NotApplicable,
}

/// 授权请求
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    /// 认证信息
    pub authentication: AuthenticationInfo,
    /// 请求路径
    pub path: String,
    /// HTTP 方法
    pub method: String,
    /// 资源类型
    pub resource_type: String,
    /// 资源 ID
    pub resource_id: Option<String>,
    /// 请求头
    pub headers: HashSet<String>,
    /// 查询参数
    pub query_params: HashSet<String>,
    /// 租户 ID
    pub tenant_id: Option<String>,
}

/// 授权上下文
#[derive(Debug, Clone, Default)]
pub struct AuthorizationContext {
    /// 角色
    pub roles: Vec<String>,
    /// 权限范围
    pub scopes: Vec<String>,
    /// 资源访问
    pub resource_access: HashSet<String>,
    /// 租户访问
    pub tenant_access: HashSet<String>,
    /// 自定义属性
    pub attributes: HashSet<String>,
}

/// 权限检查器 Trait
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// 授权决策
    async fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;
}

/// RBAC 授权器
#[derive(Debug, Clone)]
pub struct RbacAuthorizer {
    /// 配置
    config: Arc<AuthorizationConfig>,
    /// 权限缓存
    permission_cache: Cache<String, Vec<String>>,
    /// 编译的路径模式
    path_patterns: Vec<(Regex, RbacRule)>,
}

impl RbacAuthorizer {
    /// 创建 RBAC 授权器
    pub fn new(config: Arc<AuthorizationConfig>) -> Self {
        let mut path_patterns = Vec::new();

        // 预编译路径模式
        for rule in &config.rbac_rules {
            if let Ok(regex) = Regex::new(&Self::path_to_regex(&rule.path_pattern)) {
                path_patterns.push((regex, rule.clone()));
            }
        }

        let permission_cache = Cache::builder()
            .max_capacity(10000)
            .time_to_live(Duration::from_secs(300))
            .build();

        Self {
            config,
            permission_cache,
            path_patterns,
        }
    }

    /// 将路径模式转换为正则
    fn path_to_regex(pattern: &str) -> String {
        // 将通配符 * 转换为正则
        let mut regex = pattern
            .replace(".", "\\.")
            .replace("**", ".*")
            .replace("*", "[^/]*");

        // 添加锚定
        format!("^{}$", regex)
    }

    /// 提取权限
    fn extract_permission(&self, method: &str, path: &str) -> String {
        format!("{}:{}", method.to_uppercase(), path)
    }

    /// 检查角色是否匹配
    fn role_matches(&self, user_role: &str, required_role: &str) -> bool {
        if required_role.ends_with("*") {
            // 前缀匹配
            let prefix = &required_role[..required_role.len() - 1];
            user_role.starts_with(prefix) || user_role == &required_role[..required_role.len() - 1]
        } else {
            user_role == required_role
        }
    }

    /// 获取用户权限
    async fn get_user_permissions(&self, auth: &AuthenticationInfo) -> Vec<String> {
        let cache_key = format!("{}:{}",
            auth.subject_id,
            auth.tenant_id.clone().unwrap_or_default()
        );

        if let Some(permissions) = self.permission_cache.get(&cache_key).await {
            return permissions;
        }

        // 计算权限
        let mut permissions = Vec::new();

        // 从角色推导权限
        for role in &auth.roles {
            // 添加角色本身
            permissions.push(format!("role:{}", role));

            // 查找匹配的 RBAC 规则
            for (_, rule) in &self.path_patterns {
                if self.role_matches(role, &rule.role) {
                    for action in &rule.actions {
                        permissions.push(format!("{}:{}", action, rule.path_pattern));
                    }
                }
            }
        }

        // 添加直接权限范围
        for scope in &auth.scopes {
            permissions.push(format!("scope:{}", scope));
        }

        // 缓存
        self.permission_cache.insert(cache_key, permissions.clone()).await;

        permissions
    }

    /// 检查路径是否在规则中
    fn find_matching_rules(&self, path: &str, method: &str) -> Vec<&RbacRule> {
        self.path_patterns.iter()
            .filter(|(regex, rule)| {
                regex.is_match(path) && rule.methods.iter().any(|m| m == "*" || m == method)
            })
            .map(|(_, rule)| rule)
            .collect()
    }
}

#[async_trait]
impl Authorizer for RbacAuthorizer {
    async fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        // 1. 检查是否启用
        if !self.config.enabled {
            return AuthorizationDecision::Allow;
        }

        // 2. 获取用户权限
        let user_permissions = self.get_user_permissions(&request.authentication).await;

        // 3. 查找匹配的规则
        let matching_rules = self.find_matching_rules(&request.path, &request.method);

        if matching_rules.is_empty() {
            // 无匹配规则，检查是否需要授权
            if self.config.default_permissions.is_empty() {
                return AuthorizationDecision::Deny;
            }
            return AuthorizationDecision::Allow;
        }

        // 4. 检查权限
        for rule in matching_rules {
            // 检查角色
            let has_role = request.authentication.roles.iter().any(|user_role| {
                self.role_matches(user_role, &rule.role)
            });

            if !has_role {
                continue;
            }

            // 检查动作
            let has_permission = request.authentication.scopes.iter().any(|scope| {
                rule.actions.iter().any(|action| {
                    format!("{}:{}", action, request.path).starts_with(scope)
                        || rule.actions.contains(&"*".to_string())
                })
            });

            if has_role && has_permission {
                return AuthorizationDecision::Allow;
            }
        }

        // 5. 拒绝访问
        debug!(
            "Access denied: user={}, path={}, method={}",
            request.authentication.subject_id,
            request.path,
            request.method
        );

        AuthorizationDecision::Deny
    }
}

/// ABAC 授权器 (基于属性的访问控制)
#[derive(Debug, Clone)]
pub struct AbacAuthorizer {
    /// 策略
    policies: Vec<AbacPolicy>,
    /// 条件评估器
    condition_evaluator: ConditionEvaluator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacPolicy {
    /// 策略 ID
    pub id: String,
    /// 策略描述
    pub description: String,
    /// 目标条件
    pub target: AbacTarget,
    /// 条件规则
    pub conditions: Vec<AbacCondition>,
    /// 决策
    pub effect: AuthorizationDecision,
}

/// ABAC 目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacTarget {
    /// 操作
    pub actions: Vec<String>,
    /// 资源类型
    pub resource_types: Vec<String>,
    /// 路径模式
    pub path_patterns: Vec<String>,
}

/// ABAC 条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacCondition {
    /// 属性
    pub attribute: String,
    /// 操作符
    pub operator: ConditionOperator,
    /// 值
    pub value: serde_json::Value,
}

/// 条件操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    /// 等于
    Equals,
    /// 不等于
    NotEquals,
    /// 包含
    Contains,
    /// 不包含
    NotContains,
    /// 匹配
    Matches,
    /// 存在
    Exists,
    /// 不存在
    NotExists,
    /// 大于
    GreaterThan,
    /// 小于
    LessThan,
    /// 在列表中
    In,
    /// 不在列表中
    NotIn,
}

#[derive(Debug, Clone)]
pub struct ConditionEvaluator;

impl ConditionEvaluator {
    /// 评估条件
    pub fn evaluate(&self, value: &serde_json::Value, condition: &AbacCondition) -> bool {
        match &condition.operator {
            ConditionOperator::Equals => self.equals(value, &condition.value),
            ConditionOperator::NotEquals => !self.equals(value, &condition.value),
            ConditionOperator::Contains => self.contains(value, &condition.value),
            ConditionOperator::NotContains => !self.contains(value, &condition.value),
            ConditionOperator::Matches => self.matches(value, &condition.value),
            ConditionOperator::Exists => !value.is_null(),
            ConditionOperator::NotExists => value.is_null(),
            _ => false,
        }
    }

    fn equals(&self, value: &serde_json::Value, expected: &serde_json::Value) -> bool {
        value == expected
    }

    fn contains(&self, value: &serde_json::Value, expected: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::String(s) => s.contains(&expected.as_str().unwrap_or("")),
            serde_json::Value::Array(arr) => arr.contains(expected),
            _ => false,
        }
    }

    fn matches(&self, value: &serde_json::Value, pattern: &serde_json::Value) -> bool {
        if let Some(pattern_str) = pattern.as_str() {
            if let Ok(regex) = Regex::new(pattern_str) {
                return regex.is_match(&value.as_str().unwrap_or(""));
            }
        }
        false
    }
}

impl AbacAuthorizer {
    /// 创建 ABAC 授权器
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
            condition_evaluator: ConditionEvaluator,
        }
    }

    /// 添加策略
    pub fn add_policy(&mut self, policy: AbacPolicy) {
        self.policies.push(policy);
    }

    /// 评估策略
    fn evaluate_policy(&self, request: &AuthorizationRequest, policy: &AbacPolicy) -> bool {
        // 检查目标是否匹配
        if !policy.target.actions.is_empty()
            && !policy.target.actions.iter().any(|a| a == "*" || a.eq_ignore_ascii_case(&request.method)) {
            return false;
        }

        if !policy.target.resource_types.is_empty()
            && !policy.target.resource_types.contains(&request.resource_type) {
            return false;
        }

        // 评估所有条件
        for condition in &policy.conditions {
            let value = self.get_attribute_value(request, &condition.attribute);
            if !self.condition_evaluator.evaluate(&value, condition) {
                return false;
            }
        }

        true
    }

    /// 获取属性值
    fn get_attribute_value<'a>(&self, request: &'a AuthorizationRequest, attribute: &str) -> serde_json::Value {
        match attribute {
            "subject.id" => serde_json::Value::String(request.authentication.subject_id.clone()),
            "subject.tenant_id" => {
                serde_json::Value::String(request.tenant_id.clone().unwrap_or_default())
            }
            "subject.roles" => serde_json::Value::Array(
                request.authentication.roles.iter().map(|r| serde_json::Value::String(r.clone())).collect()
            ),
            "subject.scopes" => serde_json::Value::Array(
                request.authentication.scopes.iter().map(|s| serde_json::Value::String(s.clone())).collect()
            ),
            "request.method" => serde_json::Value::String(request.method.clone()),
            "request.path" => serde_json::Value::String(request.path.clone()),
            _ => serde_json::Value::Null,
        }
    }
}

#[async_trait]
impl Authorizer for AbacAuthorizer {
    async fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        for policy in &self.policies {
            if self.evaluate_policy(request, policy) {
                return policy.effect;
            }
        }

        AuthorizationDecision::NotApplicable
    }
}

/// 复合授权器 (结合 RBAC 和 ABAC)
#[derive(Debug, Clone)]
pub struct CompositeAuthorizer {
    /// RBAC 授权器
    rbac: RbacAuthorizer,
    /// ABAC 授权器
    abac: AbacAuthorizer,
    /// 组合策略
    combine_algorithm: CombineAlgorithm,
}

#[derive(Debug, Clone)]
pub enum CombineAlgorithm {
    /// 首先 RBAC，然后 ABAC
    RbacFirst,
    /// 首先 ABAC，然后 RBAC
    AbacFirst,
    /// 仅 RBAC
    RbacOnly,
    /// 仅 ABAC
    AbacOnly,
    /// 两者都必须允许
    DenyUnlessBothAllow,
    /// 任一允许即可
    AllowIfEitherAllows,
}

impl CompositeAuthorizer {
    /// 创建复合授权器
    pub fn new(rbac: RbacAuthorizer, abac: AbacAuthorizer, algorithm: CombineAlgorithm) -> Self {
        Self {
            rbac,
            abac,
            combine_algorithm: algorithm,
        }
    }
}

#[async_trait]
impl Authorizer for CompositeAuthorizer {
    async fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        match self.combine_algorithm {
            CombineAlgorithm::RbacFirst => {
                let rbac_decision = self.rbac.authorize(request).await;
                if rbac_decision == AuthorizationDecision::Deny {
                    return AuthorizationDecision::Deny;
                }
                self.abac.authorize(request).await
            }
            CombineAlgorithm::AbacFirst => {
                let abac_decision = self.abac.authorize(request).await;
                if abac_decision == AuthorizationDecision::Deny {
                    return AuthorizationDecision::Deny;
                }
                self.rbac.authorize(request).await
            }
            CombineAlgorithm::RbacOnly => self.rbac.authorize(request).await,
            CombineAlgorithm::AbacOnly => self.abac.authorize(request).await,
            CombineAlgorithm::DenyUnlessBothAllow => {
                let rbac = self.rbac.authorize(request).await;
                let abac = self.abac.authorize(request).await;
                match (rbac, abac) {
                    (AuthorizationDecision::Allow, AuthorizationDecision::Allow) => AuthorizationDecision::Allow,
                    (AuthorizationDecision::Deny, _) | (_, AuthorizationDecision::Deny) => AuthorizationDecision::Deny,
                    _ => AuthorizationDecision::Deny,
                }
            }
            CombineAlgorithm::AllowIfEitherAllows => {
                let rbac = self.rbac.authorize(request).await;
                let abac = self.abac.authorize(request).await;
                match (rbac, abac) {
                    (AuthorizationDecision::Allow, _) | (_, AuthorizationDecision::Allow) => AuthorizationDecision::Allow,
                    _ => AuthorizationDecision::Deny,
                }
            }
        }
    }
}

/// 权限检查服务
#[derive(Debug, Clone)]
pub struct AuthorizationService {
    /// 授权器
    authorizer: Arc<dyn Authorizer>,
    /// 配置
    config: Arc<AuthorizationConfig>,
}

impl AuthorizationService {
    /// 创建权限检查服务
    pub fn new(config: Arc<AuthorizationConfig>) -> Self {
        let rbac = RbacAuthorizer::new(config.clone());
        let abac = AbacAuthorizer::new();
        let authorizer: Arc<dyn Authorizer> = Arc::new(CompositeAuthorizer::new(
            rbac,
            abac,
            CombineAlgorithm::RbacFirst,
        ));

        Self {
            authorizer,
            config,
        }
    }

    /// 检查权限
    pub async fn check_permission(
        &self,
        auth: &AuthenticationInfo,
        path: &str,
        method: &str,
    ) -> GatewayResult<()> {
        let request = AuthorizationRequest {
            authentication: auth.clone(),
            path: path.to_string(),
            method: method.to_string(),
            resource_type: "api".to_string(),
            resource_id: None,
            headers: HashSet::new(),
            query_params: HashSet::new(),
            tenant_id: auth.tenant_id.clone(),
        };

        match self.authorizer.authorize(&request).await {
            AuthorizationDecision::Allow => Ok(()),
            AuthorizationDecision::Deny => Err(GatewayError::AuthorizationFailed {
                permission: format!("{} {}", method, path),
            }),
            AuthorizationDecision::NotApplicable => {
                // 检查是否有默认权限
                if self.config.default_permissions.is_empty() {
                    Err(GatewayError::AuthorizationFailed {
                        permission: format!("{} {}", method, path),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    /// 提取请求的资源 ID
    pub fn extract_resource_id<'a>(&self, path: &'a str, patterns: &[(&str, &str)]) -> Option<&'a str> {
        for (pattern, _name) in patterns {
            if let Some(captures) = Regex::new(pattern).ok().and_then(|r| r.captures(path)) {
                return captures.get(1).map(|m| m.as_str());
            }
        }
        None
    }
}
