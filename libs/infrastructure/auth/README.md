# Auth

认证授权基础设施库，提供 JWT 令牌管理、OAuth2 集成和基于角色的访问控制（RBAC）。支持多租户和细粒度权限控制。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `JwtEncoder` | JWT 令牌编码器 |
| `JwtDecoder` | JWT 令牌解码器 |
| `TokenClaims` | 令牌声明 |
| `PasswordHasher` | 密码哈希工具 |
| `OAuth2Client` | OAuth2 客户端 |
| `Permission` | 权限定义 |
| `Role` | 角色定义 |
| `RbacEnforcer` | RBAC 执行器 |
| `AuthContext` | 认证上下文 |
| `TenantId` | 租户标识 |

## 使用示例

```text
// JWT 令牌生成
let claims = TokenClaims {
    sub: user_id,
    roles: vec!["admin", "finance"],
    tenant_id: tenant,
    exp: now + Duration::hours(24),
};
let token = jwt_encoder.encode(&claims)?;

// JWT 令牌验证
let claims = jwt_decoder.decode(&token)?;

// 密码哈希
let hash = PasswordHasher::hash("password123")?;
let valid = PasswordHasher::verify("password123", &hash)?;

// RBAC 权限检查
if rbac.check(&user, Permission::CreateOrder)? {
    // 允许操作
}
```

## 权限模型

```text
User -> Roles -> Permissions
           |
           v
       Tenant (多租户隔离)
```
