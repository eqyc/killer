# Grafana 可视化

KILLER ERP 的 Grafana 仪表盘和数据源配置。

## 文件说明

| 文件/目录 | 用途 |
|-----------|------|
| `datasources.yml` | 数据源自动配置 |
| `dashboards/` | 仪表盘 JSON 文件 |

## 数据源

| 数据源 | 类型 | 用途 |
|--------|------|------|
| Prometheus | 指标 | 服务指标、资源监控 |
| Loki | 日志 | 日志聚合查询 |
| Tempo | 追踪 | 分布式追踪 |
| PostgreSQL | SQL | 业务数据查询 |
| ClickHouse | SQL | 分析数据查询 |

## 仪表盘

### 预置仪表盘

| 仪表盘 | 说明 |
|--------|------|
| `overview.json` | 系统总览 |
| `services.json` | 微服务监控 |
| `database.json` | 数据库监控 |
| `kafka.json` | Kafka 监控 |
| `business.json` | 业务指标 |

### 仪表盘目录结构

```
dashboards/
├── overview/           # 总览仪表盘
├── infrastructure/     # 基础设施
├── services/           # 微服务
├── business/           # 业务指标
└── provisioning.yml    # 仪表盘配置
```

## 访问方式

### 本地开发

```bash
# 启动 Grafana
docker run -d \
  -p 3000:3000 \
  -v $(pwd)/datasources.yml:/etc/grafana/provisioning/datasources/datasources.yml \
  -v $(pwd)/dashboards:/etc/grafana/provisioning/dashboards \
  -e GF_SECURITY_ADMIN_PASSWORD=admin \
  grafana/grafana

# 访问: http://localhost:3000
# 用户名: admin
# 密码: admin
```

### Kubernetes 部署

```bash
helm install grafana grafana/grafana \
  --set adminPassword=changeme \
  --set datasources."datasources\.yaml".apiVersion=1 \
  --set-file datasources."datasources\.yaml".datasources=datasources.yml
```

## 创建仪表盘

### 导出仪表盘

1. 在 Grafana UI 中创建仪表盘
2. 点击 Share -> Export -> Save to file
3. 将 JSON 文件保存到 `dashboards/` 目录

### 仪表盘变量

推荐使用以下变量:

```
$namespace - Kubernetes 命名空间
$service - 服务名称
$instance - 实例
$interval - 时间间隔
```

## 告警配置

### Grafana Alerting

```yaml
# 在仪表盘中配置告警
alert:
  name: High Error Rate
  conditions:
    - evaluator:
        type: gt
        params: [0.05]
      query:
        params: [A, 5m, now]
      reducer:
        type: avg
  frequency: 1m
  handler: 1
  notifications:
    - uid: slack-channel
```

### 通知渠道

支持的通知方式:

- Slack
- Email
- PagerDuty
- Webhook
- Microsoft Teams

## 最佳实践

1. **仪表盘组织**: 按域/服务分类
2. **变量使用**: 使用变量提高复用性
3. **时间范围**: 设置合理的默认时间范围
4. **刷新间隔**: 根据数据特性设置刷新间隔
5. **权限控制**: 使用文件夹管理权限
