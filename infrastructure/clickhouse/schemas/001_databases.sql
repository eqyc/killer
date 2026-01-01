-- =============================================================================
-- KILLER ERP - ClickHouse 数据库初始化
-- =============================================================================
--
-- 创建分析数据库和基础配置。
--
-- =============================================================================

-- 创建分析数据库
CREATE DATABASE IF NOT EXISTS killer_analytics;

-- 创建审计日志数据库
CREATE DATABASE IF NOT EXISTS killer_audit;

-- 创建临时数据库 (用于 ETL)
CREATE DATABASE IF NOT EXISTS killer_staging;
