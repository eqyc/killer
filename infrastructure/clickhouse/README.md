# ClickHouse 分析数据库

KILLER ERP 的 ClickHouse OLAP 数据库配置，用于业务分析和报表。

## 概述

ClickHouse 是一个高性能的列式数据库，适用于 OLAP 场景。

## 用途

| 场景 | 说明 |
|------|------|
| 业务报表 | 销售分析、财务报表 |
| 实时分析 | 订单趋势、库存分析 |
| 日志分析 | 审计日志、操作日志 |
| 指标聚合 | 业务指标预聚合 |

## 目录结构

```
clickhouse/
├── docker-compose.yml    # 本地开发环境
├── schemas/              # 表结构定义
│   ├── 001_databases.sql
│   ├── 002_sales_analytics.sql
│   ├── 003_financial_reports.sql
│   └── 004_audit_logs.sql
└── README.md
```

## 快速开始

### 启动服务

```bash
cd infrastructure/clickhouse
docker compose up -d

# 验证服务
curl http://localhost:8123/ping
```

### 连接数据库

```bash
# 使用 clickhouse-client
docker exec -it killer-clickhouse clickhouse-client

# 使用 HTTP 接口
curl 'http://localhost:8123/?query=SELECT%201'
```

## 表结构设计

### 销售分析表

```sql
CREATE TABLE killer_analytics.sales_orders_daily
(
    date Date,
    company_code LowCardinality(String),
    sales_org LowCardinality(String),
    customer_id String,
    material_id String,
    quantity Decimal(18, 3),
    amount Decimal(18, 2),
    currency LowCardinality(String)
)
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (date, company_code, sales_org, customer_id, material_id);
```

### 财务报表表

```sql
CREATE TABLE killer_analytics.gl_balances_monthly
(
    fiscal_year UInt16,
    fiscal_period UInt8,
    company_code LowCardinality(String),
    account_code String,
    cost_center LowCardinality(String),
    debit_amount Decimal(18, 2),
    credit_amount Decimal(18, 2),
    balance Decimal(18, 2),
    currency LowCardinality(String)
)
ENGINE = SummingMergeTree()
PARTITION BY fiscal_year
ORDER BY (fiscal_year, fiscal_period, company_code, account_code);
```

## 数据同步

### 从 PostgreSQL 同步

```sql
-- 使用 PostgreSQL 表引擎
CREATE TABLE killer_analytics.pg_sales_orders
ENGINE = PostgreSQL('postgres:5432', 'killer', 'sales_orders', 'killer', 'password');

-- 定期同步
INSERT INTO killer_analytics.sales_orders_daily
SELECT
    toDate(order_date) as date,
    company_code,
    sales_org,
    customer_id,
    material_id,
    sum(quantity) as quantity,
    sum(amount) as amount,
    currency
FROM killer_analytics.pg_sales_orders
WHERE order_date >= today() - 1
GROUP BY date, company_code, sales_org, customer_id, material_id, currency;
```

### 使用 Kafka 实时同步

```sql
CREATE TABLE killer_analytics.sales_orders_queue
(
    order_id String,
    order_date DateTime,
    customer_id String,
    amount Decimal(18, 2)
)
ENGINE = Kafka()
SETTINGS
    kafka_broker_list = 'kafka:9092',
    kafka_topic_list = 'sales.orders',
    kafka_group_name = 'clickhouse-consumer',
    kafka_format = 'JSONEachRow';
```

## 查询示例

### 销售趋势

```sql
SELECT
    toStartOfMonth(date) as month,
    sum(amount) as total_sales,
    count() as order_count
FROM killer_analytics.sales_orders_daily
WHERE date >= today() - 365
GROUP BY month
ORDER BY month;
```

### 客户分析

```sql
SELECT
    customer_id,
    sum(amount) as total_amount,
    count() as order_count,
    avg(amount) as avg_order_value
FROM killer_analytics.sales_orders_daily
WHERE date >= today() - 30
GROUP BY customer_id
ORDER BY total_amount DESC
LIMIT 100;
```

## 性能优化

### 分区策略

- 按月分区: `PARTITION BY toYYYYMM(date)`
- 按年分区: `PARTITION BY toYear(date)`

### 排序键设计

```sql
-- 常用查询维度放在前面
ORDER BY (date, company_code, customer_id)
```

### 物化视图

```sql
CREATE MATERIALIZED VIEW killer_analytics.sales_daily_mv
ENGINE = SummingMergeTree()
ORDER BY (date, company_code)
AS SELECT
    toDate(order_date) as date,
    company_code,
    sum(amount) as total_amount
FROM killer_analytics.sales_orders
GROUP BY date, company_code;
```

## 监控

### 系统表查询

```sql
-- 查看表大小
SELECT
    database,
    table,
    formatReadableSize(sum(bytes)) as size
FROM system.parts
GROUP BY database, table
ORDER BY sum(bytes) DESC;

-- 查看查询日志
SELECT
    query,
    query_duration_ms,
    read_rows,
    memory_usage
FROM system.query_log
WHERE type = 'QueryFinish'
ORDER BY event_time DESC
LIMIT 10;
```

## 最佳实践

1. **数据类型**: 使用 LowCardinality 优化低基数字符串
2. **分区**: 合理设置分区，避免过多小分区
3. **排序键**: 根据查询模式设计排序键
4. **物化视图**: 预聚合常用查询
5. **TTL**: 设置数据过期策略
