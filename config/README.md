# KILLER ERP 配置管理指南

本文档说明 KILLER ERP 系统的配置管理策略、配置文件结构和最佳实践。

---

## 目录

1. [配置文件概览](#配置文件概览)
2. [配置加载优先级](#配置加载优先级)
3. [环境切换](#环境切换)
4. [环境差异对比](#环境差异对比)
5. [敏感信息管理](#敏感信息管理)
6. [配置热重载](#配置热重载)
7. [配置验证](#配置验证)
8. [最佳实践](#最佳实践)

---

## 配置文件概览

```
killer/
├── .env.example              # 环境变量模板（复制为 .env 使用）
├── .env                      # 本地环境变量（⚠️ 不提交到 Git）
└── config/
    ├── README.md             # 本文档
    ├── dev.toml              # 开发环境配置
    ├── staging.toml          # 预发布环境配置
    └── production.toml       # 生产环境配置
```

### 配置节说明

| 配置节 | 说明 | 必需 |
|--------|------|------|
| `[environment]` | 环境标识和调试开关 | ✓ |
| `[server]` | HTTP/gRPC 服务器配置 | ✓ |
| `[database]` | PostgreSQL 数据库配置 | ✓ |
| `[redis]` | Redis 缓存配置 | ✓ |
| `[kafka]` | Kafka 消息队列配置 | ✓ |
| `[clickhouse]` | ClickHouse 分析数据库配置 | ○ |
| `[logging]` | 日志配置 | ✓ |
| `[observability]` | 可观测性（追踪、指标） | ○ |
| `[security]` | 安全配置（JWT、CORS、Vault） | ✓ |
| `[service_discovery]` | 服务发现配置 | ✓ |
| `[rate_limit]` | 限流配置 | ○ |
| `[circuit_breaker]` | 熔断器配置 | ○ |
| `[features]` | 功能开关 | ○ |

---

## 配置加载优先级

配置按以下优先级加载，**后者覆盖前者**：

```
┌─────────────────────────────────────────────────────────────┐
│  优先级 4（最高）: 命令行参数                                  │
│  --port 9090 --log-level debug                              │
├─────────────────────────────────────────────────────────────┤
│  优先级 3: 环境变量                                          │
│  DATABASE_URL=... SERVER_PORT=8080                          │
├─────────────────────────────────────────────────────────────┤
│  优先级 2: 配置文件 (config/{env}.toml)                      │
│  根据 RUST_ENV 加载对应文件                                  │
├─────────────────────────────────────────────────────────────┤
│  优先级 1（最低）: 代码默认值                                  │
│  Default::default() 或硬编码值                               │
└─────────────────────────────────────────────────────────────┘
```

### 示例

```toml
# config/dev.toml
[server]
port = 8080
```

```bash
# 环境变量覆盖配置文件
export SERVER_PORT=9090

# 命令行参数覆盖环境变量
cargo run -- --port 3000
```

最终生效的端口是 `3000`。

---

## 环境切换

### 使用 RUST_ENV 环境变量

```bash
# 开发环境（默认）
export RUST_ENV=development
cargo run -p api-gateway

# 预发布环境
export RUST_ENV=staging
cargo run -p api-gateway

# 生产环境
export RUST_ENV=production
cargo run -p api-gateway
```

### Docker 环境

```bash
docker run -e RUST_ENV=production killer/api-gateway
```

### Kubernetes 环境

```yaml
spec:
  containers:
    - name: api-gateway
      env:
        - name: RUST_ENV
          value: "production"
```

---

## 环境差异对比

### 总览表

| 配置项 | Development | Staging | Production |
|--------|-------------|---------|------------|
| **调试模式** | ✅ 启用 | ❌ 禁用 | ❌ 禁用 |
| **日志级别** | `debug` | `info` | `warn` |
| **日志格式** | `pretty` | `json` | `json` |
| **TLS** | ❌ 禁用 | ✅ 启用 | ✅ 强制 |
| **数据库 SSL** | `disable` | `require` | `verify-full` |
| **数据库连接池** | 5 | 15 | 30 |
| **Redis 连接池** | 3 | 10 | 20 |
| **追踪采样率** | 100% | 10% | 1% |
| **限流** | ❌ 禁用 | ✅ 启用 | ✅ 启用 |
| **熔断器** | ❌ 禁用 | ✅ 启用 | ✅ 启用 |
| **JWT 过期** | 1 小时 | 30 分钟 | 15 分钟 |
| **服务发现** | 静态配置 | Kubernetes | Kubernetes |
| **Vault** | ❌ 禁用 | ✅ 启用 | ✅ 启用 |
| **热重载** | ✅ 启用 | ❌ 禁用 | ❌ 禁用 |
| **性能分析** | ✅ 启用 | ❌ 禁用 | ❌ 禁用 |

### Development 环境特点

```toml
[environment]
name = "development"
debug = true                    # 启用调试模式

[logging]
level = "debug"                 # 详细日志
format = "pretty"               # 人类可读格式
include_location = true         # 包含代码位置

[server.tls]
enabled = false                 # 禁用 TLS，简化开发

[database]
ssl_mode = "disable"            # 禁用数据库 SSL
log_sql = true                  # 打印 SQL 语句

[rate_limit]
enabled = false                 # 禁用限流

[circuit_breaker]
enabled = false                 # 禁用熔断

[features]
hot_reload = true               # 配置热重载
profiling = true                # 性能分析
```

### Staging 环境特点

```toml
[environment]
name = "staging"
debug = false                   # 禁用调试

[logging]
level = "info"                  # 中等日志级别
format = "json"                 # 结构化日志

[server.tls]
enabled = true                  # 启用 TLS

[database]
ssl_mode = "require"            # 要求 SSL 连接
pool_size = 15                  # 中等连接池

[observability.tracing]
sampler_ratio = 0.1             # 10% 采样

[rate_limit]
enabled = true
requests_per_second = 500       # 中等限流

[security.vault]
enabled = true                  # 启用 Vault
```

### Production 环境特点

```toml
[environment]
name = "production"
debug = false                   # 禁用调试

[logging]
level = "warn"                  # 最小日志（仅警告和错误）
format = "json"                 # 结构化日志
redact_sensitive = true         # 脱敏敏感数据

[server.tls]
enabled = true                  # 强制 TLS
min_version = "1.2"             # 最低 TLS 1.2
cipher_suites = [...]           # 安全密码套件

[database]
ssl_mode = "verify-full"        # 完整证书验证
pool_size = 30                  # 大连接池

[observability.tracing]
sampler_ratio = 0.01            # 1% 采样（降低开销）

[rate_limit]
enabled = true
requests_per_second = 2000      # 高限流阈值

[security]
jwt_access_token_expiration = 900   # 15 分钟短过期
encryption_key_rotation = true      # 密钥轮换

[security.vault]
enabled = true
auth_method = "kubernetes"      # K8s 认证

[security.headers]
strict_transport_security = "max-age=31536000; includeSubDomains"
content_security_policy = "default-src 'self'"
```

---

## 敏感信息管理

### ⚠️ 禁止提交的信息

以下信息**绝对不能**提交到版本控制：

- 数据库密码
- JWT 密钥
- API 密钥
- 加密密钥
- Vault Token
- 任何凭证信息

### 开发环境：使用 .env 文件

```bash
# 1. 复制模板
cp .env.example .env

# 2. 生成安全密钥
echo "JWT_SECRET=$(openssl rand -base64 64)" >> .env
echo "ENCRYPTION_KEY=$(openssl rand -base64 32)" >> .env

# 3. 确保 .env 在 .gitignore 中
echo ".env" >> .gitignore
```

### 生产环境：使用 HashiCorp Vault

```toml
# config/production.toml
[security.vault]
enabled = true
addr = "https://vault.killer.internal:8200"
auth_method = "kubernetes"
k8s_role = "killer-prod"
secret_path = "secret/data/killer/prod"
cache_ttl = 300                 # 缓存 5 分钟
```

```bash
# 存储密钥到 Vault
vault kv put secret/killer/prod \
  jwt_secret="$(openssl rand -base64 64)" \
  db_password="secure-password" \
  encryption_key="$(openssl rand -base64 32)"
```

### 生产环境：使用 Kubernetes Secrets

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: killer-secrets
  namespace: killer-prod
type: Opaque
stringData:
  JWT_SECRET: "your-secure-jwt-secret"
  DATABASE_PASSWORD: "your-db-password"
  ENCRYPTION_KEY: "your-encryption-key"
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
        - name: api-gateway
          envFrom:
            - secretRef:
                name: killer-secrets
```

### 密钥轮换

```toml
# 生产环境启用密钥轮换
[security.encryption]
key_rotation_enabled = true
key_rotation_interval = 86400   # 每 24 小时轮换
previous_keys = [               # 保留旧密钥用于解密
  "${ENCRYPTION_KEY_PREV_1}",
  "${ENCRYPTION_KEY_PREV_2}"
]
```

---

## 配置热重载

### 支持情况

| 配置类型 | 热重载支持 | 说明 |
|----------|-----------|------|
| 日志级别 | ✅ 支持 | 无需重启 |
| 功能开关 | ✅ 支持 | 无需重启 |
| 限流阈值 | ✅ 支持 | 无需重启 |
| 数据库连接 | ❌ 不支持 | 需要重启 |
| TLS 证书 | ❌ 不支持 | 需要重启 |
| 服务端口 | ❌ 不支持 | 需要重启 |

### 启用热重载（仅开发环境）

```toml
# config/dev.toml
[features]
hot_reload = true
hot_reload_interval = 5         # 每 5 秒检查配置变更
```

### 手动触发重载

```bash
# 发送 SIGHUP 信号
kill -HUP $(pgrep api-gateway)

# 或通过管理 API
curl -X POST http://localhost:8080/admin/reload-config
```

---

## 配置验证

### 启动时验证

服务启动时自动验证配置的完整性和正确性：

```bash
# 验证配置
cargo run -p killer-cli -- config validate --env production

# 输出示例
[2024-01-15T10:30:00Z INFO] 验证配置文件: config/production.toml
[2024-01-15T10:30:00Z INFO] ✓ 配置文件语法正确
[2024-01-15T10:30:00Z INFO] ✓ 必需配置项已设置
[2024-01-15T10:30:01Z INFO] ✓ 数据库连接可达
[2024-01-15T10:30:01Z INFO] ✓ Redis 连接可达
[2024-01-15T10:30:01Z INFO] ✓ Kafka broker 可达
[2024-01-15T10:30:01Z INFO] ✓ JWT 密钥长度符合要求
[2024-01-15T10:30:01Z INFO] ✓ TLS 证书有效
[2024-01-15T10:30:01Z INFO] 配置验证通过！
```

### 验证规则

| 检查项 | 规则 | 环境 |
|--------|------|------|
| 数据库 URL | 必须是有效的 PostgreSQL URL | 全部 |
| 连接池大小 | 1 ≤ size ≤ 100 | 全部 |
| JWT 密钥长度 | ≥ 32 字节 | staging/prod |
| JWT 密钥长度 | ≥ 64 字节 | prod |
| TLS 证书 | 文件存在且可读 | staging/prod |
| TLS 证书 | 未过期 | staging/prod |
| Kafka brokers | 至少 1 个 | 全部 |
| Kafka brokers | 至少 3 个 | prod |
| 日志级别 | 有效值 (trace/debug/info/warn/error) | 全部 |

### 验证失败处理

```bash
# 验证失败时服务拒绝启动
[2024-01-15T10:30:00Z ERROR] 配置验证失败:
  - JWT_SECRET 未设置
  - DATABASE_URL 格式无效
  - TLS 证书文件不存在: /etc/killer/tls/tls.crt

[2024-01-15T10:30:00Z ERROR] 服务启动中止，请修复配置后重试
```

---

## 最佳实践

### 1. 分离敏感信息

```toml
# ❌ 错误：直接写入密码
[database]
url = "postgresql://user:password123@localhost:5432/erp"

# ✅ 正确：使用环境变量引用
[database]
url = "postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:5432/erp_${SERVICE_DB}"
```

### 2. 使用合理的默认值

```toml
# ✅ 提供合理的默认值，避免生产事故
[database]
pool_size = 10              # 中等规模默认值
pool_min = 2                # 保持最小连接
connection_timeout = 30     # 合理超时
idle_timeout = 600          # 空闲连接回收
```

### 3. 环境隔离

```toml
# ✅ 不同环境使用不同的命名空间
[kafka.topics]
domain_events = "killer.${ENV}.domain-events"

[redis.cache]
key_prefix = "killer:${ENV}:"

[database]
# 开发：erp_financial_dev
# 生产：erp_financial
database = "erp_${SERVICE_NAME}_${ENV_SUFFIX}"
```

### 4. 渐进式功能发布

```toml
# ✅ 使用功能开关控制新功能
[features]
new_pricing_engine = false      # 默认关闭

[features.gradual_rollout]
new_pricing_engine = 0.1        # 10% 流量
new_inventory_algo = 0.05       # 5% 流量
```

### 5. 配置文档化

```toml
# ✅ 为每个配置项添加注释
[server]
# 服务监听端口
# 范围：1024-65535
# 默认：8080
port = 8080

# 工作线程数
# 0 表示使用 CPU 核心数
# 建议：CPU 核心数 * 2
worker_threads = 0
```

### 6. 定期审计配置

```bash
# 定期检查配置差异
diff config/staging.toml config/production.toml

# 检查敏感信息泄露
grep -r "password\|secret\|key" config/ --include="*.toml"
```

---

## 故障排查

### 常见问题

**1. 配置文件未加载**

```bash
# 检查 RUST_ENV 设置
echo $RUST_ENV

# 检查配置文件是否存在
ls -la config/

# 检查文件权限
stat config/production.toml
```

**2. 环境变量未生效**

```bash
# 确认 .env 文件存在
cat .env

# 检查变量是否导出
env | grep DATABASE_

# 检查变量名是否正确（区分大小写）
```

**3. 数据库连接失败**

```bash
# 测试数据库连接
psql $DATABASE_URL -c "SELECT 1"

# 检查网络连通性
nc -zv localhost 5432

# 检查 SSL 配置
openssl s_client -connect db.killer.internal:5432
```

**4. 配置值类型错误**

```bash
# TOML 语法检查
cargo run -p killer-cli -- config lint

# 常见错误：
# - 字符串未加引号
# - 数组格式错误
# - 布尔值大小写（应为 true/false）
```

---

## 相关链接

- [ADR-0004: 使用 PostgreSQL 和 ClickHouse](../docs/adr/0004-use-postgresql-and-clickhouse.md)
- [开发环境搭建指南](../docs/DEVELOPMENT.md)
- [部署指南](../docs/guides/deployment.md)
- [HashiCorp Vault 文档](https://www.vaultproject.io/docs)

---

*最后更新: 2024-01*
