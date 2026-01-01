# Identity & IAM Service

身份与访问管理服务，负责用户身份管理、认证授权、角色权限控制和多租户管理。是所有服务的认证授权中心。

## 服务职责

| 模块 | 职责 |
|------|------|
| 用户管理 | 用户 CRUD、密码管理 |
| 认证 | 登录、登出、Token 管理 |
| 授权 | RBAC、权限检查 |
| 角色管理 | 角色定义、权限分配 |
| 租户管理 | 多租户隔离 |
| 审计日志 | 登录日志、操作日志 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `User` | 用户 |
| `Role` | 角色 |
| `Permission` | 权限 |
| `Tenant` | 租户 |
| `Session` | 会话 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `UserCreated` | 用户创建 |
| `UserLoggedIn` | 用户登录 |
| `UserLoggedOut` | 用户登出 |
| `PasswordChanged` | 密码修改 |
| `RoleAssigned` | 角色分配 |
| `PermissionGranted` | 权限授予 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `CompanyCode` | organizational-units | 公司关联 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
