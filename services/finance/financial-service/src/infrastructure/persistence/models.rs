//! 数据库模型定义
//!
//! 提供领域对象 ↔ 数据库记录的映射
//! 使用 sqlx::FromRow 实现自动映射

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::types::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// 凭证抬头模型
// =============================================================================

/// 会计凭证数据库模型
///
/// # 表结构
///
/// ```sql
/// CREATE TABLE journal_entries (
///     id UUID PRIMARY KEY,
///     tenant_id UUID NOT NULL,
///     company_code VARCHAR(10) NOT NULL,
///     fiscal_year INTEGER NOT NULL,
///     document_number VARCHAR(16) NOT NULL,
///     posting_date DATE NOT NULL,
///     document_date DATE NOT NULL,
///     currency_code VARCHAR(3) NOT NULL,
///     status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
///     header_text VARCHAR(200),
///     reference_document VARCHAR(20),
///     total_debit DECIMAL(18,2) NOT NULL DEFAULT 0,
///     total_credit DECIMAL(18,2) NOT NULL DEFAULT 0,
///     version INTEGER NOT NULL DEFAULT 1,
///     extensions JSONB DEFAULT '{}',
///     created_by UUID,
///     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     posted_at TIMESTAMPTZ,
///     deleted_at TIMESTAMPTZ,
///     deleted_by UUID,
///     CONSTRAINT journal_entries_pk PRIMARY KEY (id),
///     CONSTRAINT journal_entries_tenant_doc_uq UNIQUE (tenant_id, company_code, fiscal_year, document_number)
/// );
///
/// CREATE INDEX journal_entries_tenant_idx ON journal_entries(tenant_id);
/// CREATE INDEX journal_entries_posting_date_idx ON journal_entries(posting_date);
/// CREATE INDEX journal_entries_status_idx ON journal_entries(status);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryEntity {
    /// 主键 ID
    pub id: Uuid,

    /// 租户 ID（多租户隔离）
    pub tenant_id: Uuid,

    /// 公司代码
    pub company_code: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 凭证号
    pub document_number: String,

    /// 过账日期
    pub posting_date: NaiveDate,

    /// 凭证日期
    pub document_date: NaiveDate,

    /// 币种代码
    pub currency_code: String,

    /// 凭证状态
    pub status: String,

    /// 抬头文本
    pub header_text: Option<String>,

    /// 参考凭证号
    pub reference_document: Option<String>,

    /// 借方总额
    pub total_debit: Decimal,

    /// 贷方总额
    pub total_credit: Decimal,

    /// 版本号（乐观锁）
    pub version: i32,

    /// 扩展属性（JSONB）
    pub extensions: Option<serde_json::Value>,

    /// 创建人
    pub created_by: Option<Uuid>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 过账时间
    pub posted_at: Option<DateTime<Utc>>,

    /// 删除时间（软删除）
    pub deleted_at: Option<DateTime<Utc>>,

    /// 删除人
    pub deleted_by: Option<Uuid>,
}

impl JournalEntryEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            company_code: row.try_get("company_code")?,
            fiscal_year: row.try_get("fiscal_year")?,
            document_number: row.try_get("document_number")?,
            posting_date: row.try_get("posting_date")?,
            document_date: row.try_get("document_date")?,
            currency_code: row.try_get("currency_code")?,
            status: row.try_get("status")?,
            header_text: row.try_get("header_text")?,
            reference_document: row.try_get("reference_document")?,
            total_debit: row.try_get("total_debit")?,
            total_credit: row.try_get("total_credit")?,
            version: row.try_get("version")?,
            extensions: row.try_get("extensions")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            posted_at: row.try_get("posted_at")?,
            deleted_at: row.try_get("deleted_at")?,
            deleted_by: row.try_get("deleted_by")?,
        })
    }
}

// =============================================================================
// 凭证行项目模型
// =============================================================================

/// 会计凭证行项目数据库模型
///
/// ```sql
/// CREATE TABLE journal_entry_lines (
///     id UUID PRIMARY KEY,
///     tenant_id UUID NOT NULL,
///     entry_id UUID NOT NULL REFERENCES journal_entries(id),
///     line_number INTEGER NOT NULL,
///     account_code VARCHAR(10) NOT NULL,
///     amount DECIMAL(18,2) NOT NULL,
///     debit_credit CHAR(1) NOT NULL CHECK (debit_credit IN ('D', 'C')),
///     cost_center VARCHAR(10),
///     profit_center VARCHAR(10),
///     text VARCHAR(200),
///     functional_area VARCHAR(4),
///     business_area VARCHAR(4),
///     order_number VARCHAR(12),
///     tax_code VARCHAR(10),
///     tax_amount DECIMAL(18,2),
///     extensions JSONB DEFAULT '{}',
///     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     CONSTRAINT journal_entry_lines_pk PRIMARY KEY (id),
///     CONSTRAINT journal_entry_lines_entry_uq UNIQUE (entry_id, line_number)
/// );
///
/// CREATE INDEX journal_entry_lines_entry_idx ON journal_entry_lines(entry_id);
/// CREATE INDEX journal_entry_lines_account_idx ON journal_entry_lines(account_code);
/// CREATE INDEX journal_entry_lines_cost_center_idx ON journal_entry_lines(cost_center);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryLineEntity {
    /// 主键 ID
    pub id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 所属凭证 ID
    pub entry_id: Uuid,

    /// 行号
    pub line_number: i32,

    /// 会计科目代码
    pub account_code: String,

    /// 金额
    pub amount: Decimal,

    /// 借贷方向
    pub debit_credit: String,

    /// 成本中心
    pub cost_center: Option<String>,

    /// 利润中心
    pub profit_center: Option<String>,

    /// 行项目文本
    pub text: Option<String>,

    /// 功能范围
    pub functional_area: Option<String>,

    /// 业务范围
    pub business_area: Option<String>,

    /// 订单号
    pub order_number: Option<String>,

    /// 税码
    pub tax_code: Option<String>,

    /// 税额
    pub tax_amount: Option<Decimal>,

    /// 扩展属性
    pub extensions: Option<serde_json::Value>,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

impl JournalEntryLineEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            entry_id: row.try_get("entry_id")?,
            line_number: row.try_get("line_number")?,
            account_code: row.try_get("account_code")?,
            amount: row.try_get("amount")?,
            debit_credit: row.try_get("debit_credit")?,
            cost_center: row.try_get("cost_center")?,
            profit_center: row.try_get("profit_center")?,
            text: row.try_get("text")?,
            functional_area: row.try_get("functional_area")?,
            business_area: row.try_get("business_area")?,
            order_number: row.try_get("order_number")?,
            tax_code: row.try_get("tax_code")?,
            tax_amount: row.try_get("tax_amount")?,
            extensions: row.try_get("extensions")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

// =============================================================================
// 会计期间模型
// =============================================================================

/// 会计期间数据库模型
///
/// ```sql
/// CREATE TABLE fiscal_periods (
///     id UUID PRIMARY KEY,
///     tenant_id UUID NOT NULL,
///     company_code VARCHAR(10) NOT NULL,
///     fiscal_year INTEGER NOT NULL,
///     period INTEGER NOT NULL CHECK (period BETWEEN 1 AND 16),
///     status VARCHAR(20) NOT NULL DEFAULT 'OPEN',
///     valid_from DATE NOT NULL,
///     valid_to DATE NOT NULL,
///     version INTEGER NOT NULL DEFAULT 1,
///     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     closed_at TIMESTAMPTZ,
///     deleted_at TIMESTAMPTZ,
///     CONSTRAINT fiscal_periods_pk PRIMARY KEY (id),
///     CONSTRAINT fiscal_periods_uq UNIQUE (tenant_id, company_code, fiscal_year, period)
/// );
///
/// CREATE INDEX fiscal_periods_tenant_idx ON fiscal_periods(tenant_id);
/// CREATE INDEX fiscal_periods_valid_range_idx ON fiscal_periods USING GIST (daterange(valid_from, valid_to));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalPeriodEntity {
    /// 主键 ID
    pub id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 公司代码
    pub company_code: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 期间号
    pub period: i32,

    /// 状态
    pub status: String,

    /// 有效开始日期
    pub valid_from: NaiveDate,

    /// 有效结束日期
    pub valid_to: NaiveDate,

    /// 版本号
    pub version: i32,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 关闭时间
    pub closed_at: Option<DateTime<Utc>>,

    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

impl FiscalPeriodEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            company_code: row.try_get("company_code")?,
            fiscal_year: row.try_get("fiscal_year")?,
            period: row.try_get("period")?,
            status: row.try_get("status")?,
            valid_from: row.try_get("valid_from")?,
            valid_to: row.try_get("valid_to")?,
            version: row.try_get("version")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            closed_at: row.try_get("closed_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

// =============================================================================
// 发件箱消息模型
// =============================================================================

/// 发件箱消息数据库模型
///
/// ```sql
/// CREATE TABLE outbox_messages (
///     id UUID PRIMARY KEY,
///     tenant_id UUID NOT NULL,
///     aggregate_type VARCHAR(50) NOT NULL,
///     aggregate_id VARCHAR(100) NOT NULL,
///     event_type VARCHAR(100) NOT NULL,
///     payload JSONB NOT NULL,
///     schema_version INTEGER NOT NULL DEFAULT 1,
///     occurred_at TIMESTAMPTZ NOT NULL,
///     status VARCHAR(20) NOT NULL DEFAULT 'Pending',
///     attempts INTEGER NOT NULL DEFAULT 0,
///     last_error TEXT,
///     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     sent_at TIMESTAMPTZ,
///     metadata JSONB DEFAULT '{}',
///     CONSTRAINT outbox_messages_pk PRIMARY KEY (id)
/// );
///
/// CREATE INDEX outbox_messages_status_idx ON outbox_messages(status);
/// CREATE INDEX outbox_messages_tenant_idx ON outbox_messages(tenant_id);
/// CREATE INDEX outbox_messages_occurred_at_idx ON outbox_messages(occurred_at);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxMessageEntity {
    /// 主键 ID
    pub id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 聚合根类型
    pub aggregate_type: String,

    /// 聚合根 ID
    pub aggregate_id: String,

    /// 事件类型
    pub event_type: String,

    /// 事件负载
    pub payload: serde_json::Value,

    /// Schema 版本
    pub schema_version: i32,

    /// 发生时间
    pub occurred_at: DateTime<Utc>,

    /// 状态
    pub status: String,

    /// 重试次数
    pub attempts: i32,

    /// 最后错误
    pub last_error: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 发送时间
    pub sent_at: Option<DateTime<Utc>>,

    /// 元数据
    pub metadata: Option<serde_json::Value>,
}

impl OutboxMessageEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            aggregate_type: row.try_get("aggregate_type")?,
            aggregate_id: row.try_get("aggregate_id")?,
            event_type: row.try_get("event_type")?,
            payload: row.try_get("payload")?,
            schema_version: row.try_get("schema_version")?,
            occurred_at: row.try_get("occurred_at")?,
            status: row.try_get("status")?,
            attempts: row.try_get("attempts")?,
            last_error: row.try_get("last_error")?,
            created_at: row.try_get("created_at")?,
            sent_at: row.try_get("sent_at")?,
            metadata: row.try_get("metadata")?,
        })
    }
}

// =============================================================================
// 审计日志模型
// =============================================================================

/// 审计日志数据库模型
///
/// ```sql
/// CREATE TABLE journal_audit_log (
///     id UUID PRIMARY KEY,
///     tenant_id UUID NOT NULL,
///     table_name VARCHAR(50) NOT NULL,
///     record_id UUID NOT NULL,
///     action VARCHAR(20) NOT NULL,
///     old_value JSONB,
///     new_value JSONB,
///     changed_by UUID,
///     changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     client_ip VARCHAR(45),
///     request_id UUID,
///     metadata JSONB DEFAULT '{}',
///     CONSTRAINT journal_audit_log_pk PRIMARY KEY (id)
/// );
///
/// CREATE INDEX journal_audit_log_tenant_idx ON journal_audit_log(tenant_id);
/// CREATE INDEX journal_audit_log_record_idx ON journal_audit_log(table_name, record_id);
/// CREATE INDEX journal_audit_log_changed_at_idx ON journal_audit_log(changed_at);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntity {
    /// 主键 ID
    pub id: Uuid,

    /// 租户 ID
    pub tenant_id: Uuid,

    /// 表名
    pub table_name: String,

    /// 记录 ID
    pub record_id: Uuid,

    /// 操作类型
    pub action: String,

    /// 变更前值
    pub old_value: Option<serde_json::Value>,

    /// 变更后值
    pub new_value: Option<serde_json::Value>,

    /// 变更人
    pub changed_by: Option<Uuid>,

    /// 变更时间
    pub changed_at: DateTime<Utc>,

    /// 客户端 IP
    pub client_ip: Option<String>,

    /// 请求 ID
    pub request_id: Option<Uuid>,

    /// 元数据
    pub metadata: Option<serde_json::Value>,
}

impl AuditLogEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            table_name: row.try_get("table_name")?,
            record_id: row.try_get("record_id")?,
            action: row.try_get("action")?,
            old_value: row.try_get("old_value")?,
            new_value: row.try_get("new_value")?,
            changed_by: row.try_get("changed_by")?,
            changed_at: row.try_get("changed_at")?,
            client_ip: row.try_get("client_ip")?,
            request_id: row.try_get("request_id")?,
            metadata: row.try_get("metadata")?,
        })
    }
}

// =============================================================================
// 已处理事件模型（投影幂等性）
// =============================================================================

/// 已处理事件记录（用于投影幂等性）
///
/// ```sql
/// CREATE TABLE processed_events (
///     event_id UUID PRIMARY KEY,
///     event_type VARCHAR(100) NOT NULL,
///     aggregate_id VARCHAR(100) NOT NULL,
///     processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
///     CONSTRAINT processed_events_pk PRIMARY KEY (event_id)
/// );
///
/// CREATE INDEX processed_events_aggregate_idx ON processed_events(event_type, aggregate_id);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedEventEntity {
    /// 事件 ID
    pub event_id: Uuid,

    /// 事件类型
    pub event_type: String,

    /// 聚合根 ID
    pub aggregate_id: String,

    /// 处理时间
    pub processed_at: DateTime<Utc>,
}

impl ProcessedEventEntity {
    /// 从数据库行构建
    pub async fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            event_id: row.try_get("event_id")?,
            event_type: row.try_get("event_type")?,
            aggregate_id: row.try_get("aggregate_id")?,
            processed_at: row.try_get("processed_at")?,
        })
    }
}
