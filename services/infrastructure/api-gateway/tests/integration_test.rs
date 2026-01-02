//! API Gateway 集成测试
//!
//! 测试 JWT 验证、限流、熔断器等关键路径

use killer_api_gateway::{
    config::{GatewayConfig, AuthConfig, JwtConfig, RateLimitingConfig, RateLimitTier, AuthorizationConfig, RbacRule},
    domain::{AuthenticationInfo, AuthType, TenantStatus, TenantQuota, ServiceInstance},
    infrastructure::{RateLimitManager, AuthorizationService, CircuitBreaker},
};

use std::sync::Arc;
use chrono::Utc;

// =============================================================================
// 配置加载测试
// =============================================================================

#[tokio::test]
async fn test_default_config() {
    let config = GatewayConfig::default();

    // 验证默认服务器配置
    assert_eq!(config.server.http_port, 8080);
    assert_eq!(config.server.max_request_size, 2 * 1024 * 1024);

    // 验证默认认证配置
    assert!(config.authentication.enabled);

    // 验证默认限流配置
    assert!(config.rate_limiting.enabled);
    assert!(config.rate_limiting.global.enabled);

    // 验证默认熔断器配置
    assert!(config.circuit_breaker.enabled);
}

// =============================================================================
// 认证信息测试
// =============================================================================

#[tokio::test]
async fn test_authentication_info() {
    let auth_info = AuthenticationInfo {
        auth_type: AuthType::Jwt,
        subject_id: "user-123".to_string(),
        tenant_id: Some("tenant-456".to_string()),
        scopes: vec!["read:users".to_string(), "write:users".to_string()],
        roles: vec!["admin".to_string()],
        claims: std::collections::HashMap::new(),
        expires_at: Some(Utc::now()),
        authenticated_at: Utc::now(),
    };

    assert_eq!(auth_info.subject_id, "user-123");
    assert_eq!(auth_info.tenant_id, Some("tenant-456".to_string()));
    assert_eq!(auth_info.scopes.len(), 2);
    assert!(auth_info.roles.contains(&"admin".to_string()));
}

// =============================================================================
// 限流测试
// =============================================================================

#[tokio::test]
async fn test_rate_limiter_allows_within_limit() {
    let rate_config = RateLimitingConfig {
        enabled: true,
        global: RateLimitTier {
            enabled: true,
            capacity: 100,
            refill_rate: 100,
            burst_capacity: 150,
        },
        per_ip: RateLimitTier {
            enabled: true,
            capacity: 100,
            refill_rate: 10,
            burst_capacity: 20,
        },
        per_user: RateLimitTier::default(),
        per_api_key: RateLimitTier::default(),
        per_route: std::collections::HashMap::new(),
    };

    let manager = Arc::new(RateLimitManager::new(Arc::new(rate_config)));

    // 发送 10 个请求，应该全部允许
    for i in 0..10 {
        let result = manager.check_strict(
            "127.0.0.1:1234".parse().ok(),
            Some(&format!("user-{}", i)),
            None,
            "/api/v1/test",
        ).await;

        assert!(result.is_ok());
        let limit_result = result.unwrap();
        assert_eq!(limit_result.decision, killer_api_gateway::domain::RateLimitDecision::Allowed);
    }
}

#[tokio::test]
async fn test_rate_limiter_blocks_excess_requests() {
    let mut rate_config = RateLimitingConfig {
        enabled: true,
        global: RateLimitTier::default(),
        per_ip: RateLimitTier {
            enabled: true,
            capacity: 5,
            refill_rate: 1,
            burst_capacity: 5,
        },
        per_user: RateLimitTier::default(),
        per_api_key: RateLimitTier::default(),
        per_route: std::collections::HashMap::new(),
    };

    let manager = Arc::new(RateLimitManager::new(Arc::new(rate_config)));

    // 前 5 个请求应该允许
    for _ in 0..5 {
        let result = manager.check_strict(
            "127.0.0.1:1234".parse().ok(),
            Some("test-user"),
            None,
            "/api/v1/test",
        ).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().decision, killer_api_gateway::domain::RateLimitDecision::Allowed);
    }

    // 第 6 个请求应该被限流
    let result = manager.check_strict(
        "127.0.0.1:1234".parse().ok(),
        Some("test-user"),
        None,
        "/api/v1/test",
    ).await;

    assert!(result.is_ok());
    let limit_result = result.unwrap();
    assert_eq!(limit_result.decision, killer_api_gateway::domain::RateLimitDecision::RateLimited);
    assert!(limit_result.retry_after.is_some());
}

// =============================================================================
// 熔断器测试
// =============================================================================

#[tokio::test]
async fn test_circuit_breaker_initial_state() {
    let breaker = CircuitBreaker::new(
        "test-service".to_string(),
        Some(killer_api_gateway::infrastructure::CircuitBreakerPolicy {
            failure_threshold: 3,
            success_threshold: 2,
            half_open_timeout: std::time::Duration::from_secs(5),
            volume_threshold: 5,
            failure_rate_threshold: 50.0,
            recovery_timeout: std::time::Duration::from_secs(30),
        }),
    );

    // 初始状态应该是关闭
    let state = breaker.state().await;
    assert!(matches!(state, killer_api_gateway::infrastructure::State::Closed));

    // 应该允许请求
    assert!(breaker.can_proceed().await);
}

#[tokio::test]
async fn test_circuit_breaker_opens_after_failures() {
    let breaker = CircuitBreaker::new(
        "test-service".to_string(),
        Some(killer_api_gateway::infrastructure::CircuitBreakerPolicy {
            failure_threshold: 3,
            success_threshold: 2,
            half_open_timeout: std::time::Duration::from_secs(5),
            volume_threshold: 1,
            failure_rate_threshold: 100.0,
            recovery_timeout: std::time::Duration::from_secs(30),
        }),
    );

    // 记录 3 次失败
    for _ in 0..3 {
        breaker.record_failure().await;
    }

    // 熔断器应该打开
    let state = breaker.state().await;
    assert!(matches!(state, killer_api_gateway::infrastructure::State::Open));

    // 应该不允许请求
    assert!(!breaker.can_proceed().await);
}

#[tokio::test]
async fn test_circuit_breaker_recovers() {
    let breaker = CircuitBreaker::new(
        "test-service".to_string(),
        Some(killer_api_gateway::infrastructure::CircuitBreakerPolicy {
            failure_threshold: 2,
            success_threshold: 2,
            half_open_timeout: std::time::Duration::from_millis(100),
            volume_threshold: 1,
            failure_rate_threshold: 100.0,
            recovery_timeout: std::time::Duration::from_millis(200),
        }),
    );

    // 触发打开
    breaker.record_failure().await;
    breaker.record_failure().await;

    let state = breaker.state().await;
    assert!(matches!(state, killer_api_gateway::infrastructure::State::Open));

    // 等待恢复超时
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // 应该进入半开状态
    let state = breaker.state().await;
    assert!(matches!(state, killer_api_gateway::infrastructure::State::HalfOpen));

    // 记录成功
    breaker.record_success().await;
    breaker.record_success().await;

    // 应该关闭
    let state = breaker.state().await;
    assert!(matches!(state, killer_api_gateway::infrastructure::State::Closed));
}

// =============================================================================
// 负载均衡测试
// =============================================================================

#[tokio::test]
async fn test_load_balancer_selects_instance() {
    use killer_api_gateway::infrastructure::LoadBalancer;

    let instances = vec![
        ServiceInstance {
            id: "inst-1".to_string(),
            service_name: "test-service".to_string(),
            address: "10.0.0.1".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            weight: 100,
            healthy: true,
            last_health_check: Some(Utc::now()),
            metadata: std::collections::HashMap::new(),
        },
        ServiceInstance {
            id: "inst-2".to_string(),
            service_name: "test-service".to_string(),
            address: "10.0.0.2".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            weight: 100,
            healthy: false,
            last_health_check: None,
            metadata: std::collections::HashMap::new(),
        },
    ];

    let lb = LoadBalancer::default();

    // 只应该选择健康的实例
    let selected = lb.select(&instances);
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "inst-1");
}

#[tokio::test]
async fn test_load_balancer_skips_unhealthy() {
    use killer_api_gateway::infrastructure::LoadBalancer;

    let instances = vec![
        ServiceInstance {
            id: "inst-1".to_string(),
            service_name: "test-service".to_string(),
            address: "10.0.0.1".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            weight: 100,
            healthy: true,
            last_health_check: Some(Utc::now()),
            metadata: std::collections::HashMap::new(),
        },
        ServiceInstance {
            id: "inst-2".to_string(),
            service_name: "test-service".to_string(),
            address: "10.0.0.2".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            weight: 100,
            healthy: false,
            last_health_check: None,
            metadata: std::collections::HashMap::new(),
        },
    ];

    let lb = LoadBalancer::default();

    // 只应该选择健康的实例
    let selected = lb.select(&instances);
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "inst-1");
}

// =============================================================================
// 授权测试
// =============================================================================

#[tokio::test]
async fn test_authorization_with_matching_role() {
    let auth_info = AuthenticationInfo {
        auth_type: AuthType::Jwt,
        subject_id: "user-123".to_string(),
        tenant_id: Some("tenant-456".to_string()),
        scopes: vec!["read:users".to_string()],
        roles: vec!["finance:read".to_string()],
        claims: std::collections::HashMap::new(),
        expires_at: None,
        authenticated_at: Utc::now(),
    };

    let authz_config = Arc::new(AuthorizationConfig {
        enabled: true,
        claim_field: "scope".to_string(),
        scope_delimiter: " ".to_string(),
        default_permissions: vec![],
        rbac_rules: vec![
            RbacRule {
                role: "finance:*".to_string(),
                path_pattern: "/api/v1/finance/**".to_string(),
                methods: vec!["GET".to_string()],
                actions: vec!["read".to_string()],
            },
        ],
    });

    let authz_service = AuthorizationService::new(authz_config);

    // 检查权限
    let result = authz_service.check_permission(
        &auth_info,
        "/api/v1/finance/reports",
        "GET",
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_authorization_without_matching_role() {
    let auth_info = AuthenticationInfo {
        auth_type: AuthType::Jwt,
        subject_id: "user-123".to_string(),
        tenant_id: None,
        scopes: vec![],
        roles: vec!["viewer".to_string()],
        claims: std::collections::HashMap::new(),
        expires_at: None,
        authenticated_at: Utc::now(),
    };

    let authz_config = Arc::new(AuthorizationConfig {
        enabled: true,
        claim_field: "scope".to_string(),
        scope_delimiter: " ".to_string(),
        default_permissions: vec![],
        rbac_rules: vec![
            RbacRule {
                role: "viewer".to_string(),
                path_pattern: "/api/v1/**".to_string(),
                methods: vec!["GET".to_string()],
                actions: vec!["read".to_string()],
            },
        ],
    });

    let authz_service = AuthorizationService::new(authz_config);

    // 尝试 DELETE 请求，应该被拒绝
    let result = authz_service.check_permission(
        &auth_info,
        "/api/v1/users/123",
        "DELETE",
    ).await;

    assert!(result.is_err());
}
