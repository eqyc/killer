-- KILLER ERP Financial Service Database Schema
-- Version: 0.1.0
-- Description: Initial database schema for financial service

-- =============================================================================
-- Extensions
-- =============================================================================

-- Enable necessary PostgreSQL extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "btree_gin";
CREATE EXTENSION IF NOT EXISTS "btree_gist";

-- =============================================================================
-- Enum Types
-- =============================================================================

-- Journal entry status
DO $$ BEGIN
    CREATE TYPE journal_entry_status AS ENUM ('DRAFT', 'POSTED', 'REVERSED', 'DELETED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Fiscal period status
DO $$ BEGIN
    CREATE TYPE fiscal_period_status AS ENUM ('OPEN', 'CLOSED', 'ARCHIVED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Outbox message status
DO $$ BEGIN
    CREATE TYPE outbox_status AS ENUM ('Pending', 'Sent', 'Failed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Audit action type
DO $$ BEGIN
    CREATE TYPE audit_action AS ENUM ('CREATE', 'UPDATE', 'DELETE', 'POST', 'REVERSE');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- =============================================================================
-- Journal Entries Table (凭证抬头)
-- =============================================================================

CREATE TABLE IF NOT EXISTS journal_entries (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Tenant isolation
    tenant_id UUID NOT NULL,

    -- Business key
    company_code VARCHAR(10) NOT NULL,
    fiscal_year INTEGER NOT NULL,
    document_number VARCHAR(16) NOT NULL,

    -- Document dates
    posting_date DATE NOT NULL,
    document_date DATE NOT NULL,

    -- Currency and status
    currency_code VARCHAR(3) NOT NULL DEFAULT 'CNY',
    status journal_entry_status NOT NULL DEFAULT 'DRAFT',

    -- Text fields
    header_text VARCHAR(200),
    reference_document VARCHAR(20),

    -- Amounts
    total_debit DECIMAL(18,2) NOT NULL DEFAULT 0,
    total_credit DECIMAL(18,2) NOT NULL DEFAULT 0,

    -- Concurrency control
    version INTEGER NOT NULL DEFAULT 1,

    -- Extensions (JSONB for custom attributes)
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Audit fields
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    posted_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID,

    -- Constraints
    CONSTRAINT journal_entries_pk PRIMARY KEY (id),
    CONSTRAINT journal_entries_tenant_doc_uq UNIQUE (tenant_id, company_code, fiscal_year, document_number),
    CONSTRAINT journal_entries_amounts_chk CHECK (
        (status = 'DRAFT' AND total_debit = total_credit)
        OR (status <> 'DRAFT')
    )
);

-- Indexes
CREATE INDEX IF NOT EXISTS journal_entries_tenant_idx ON journal_entries(tenant_id);
CREATE INDEX IF NOT EXISTS journal_entries_company_idx ON journal_entries(tenant_id, company_code);
CREATE INDEX IF NOT EXISTS journal_entries_posting_date_idx ON journal_entries(tenant_id, posting_date);
CREATE INDEX IF NOT EXISTS journal_entries_status_idx ON journal_entries(tenant_id, status);
CREATE INDEX IF NOT EXISTS journal_entries_fiscal_year_idx ON journal_entries(tenant_id, fiscal_year);
CREATE INDEX IF NOT EXISTS journal_entries_created_at_idx ON journal_entries(created_at DESC);
CREATE INDEX IF NOT EXISTS journal_entries_deleted_idx ON journal_entries(tenant_id) WHERE deleted_at IS NULL;

-- GIN index for extensions JSONB queries
CREATE INDEX IF NOT EXISTS journal_entries_extensions_idx ON journal_entries USING GIN (extensions);

-- =============================================================================
-- Journal Entry Lines Table (凭证行项目)
-- =============================================================================

CREATE TABLE IF NOT EXISTS journal_entry_lines (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- References
    tenant_id UUID NOT NULL,
    entry_id UUID NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,

    -- Line number
    line_number INTEGER NOT NULL,

    -- Account and amount
    account_code VARCHAR(10) NOT NULL,
    amount DECIMAL(18,2) NOT NULL,
    debit_credit CHAR(1) NOT NULL CHECK (debit_credit IN ('D', 'C')),

    -- Assignment fields
    cost_center VARCHAR(10),
    profit_center VARCHAR(10),
    functional_area VARCHAR(4),
    business_area VARCHAR(4),
    order_number VARCHAR(12),

    -- Text
    text VARCHAR(200),

    -- Tax information
    tax_code VARCHAR(10),
    tax_amount DECIMAL(18,2),

    -- Extensions
    extensions JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT journal_entry_lines_pk PRIMARY KEY (id),
    CONSTRAINT journal_entry_lines_entry_uq UNIQUE (entry_id, line_number),
    CONSTRAINT journal_entry_lines_amount_chk CHECK (amount >= 0)
);

-- Indexes
CREATE INDEX IF NOT EXISTS journal_entry_lines_entry_idx ON journal_entry_lines(entry_id);
CREATE INDEX IF NOT EXISTS journal_entry_lines_tenant_idx ON journal_entry_lines(tenant_id);
CREATE INDEX IF NOT EXISTS journal_entry_lines_account_idx ON journal_entry_lines(account_code);
CREATE INDEX IF NOT EXISTS journal_entry_lines_cost_center_idx ON journal_entry_lines(cost_center);
CREATE INDEX IF NOT EXISTS journal_entry_lines_profit_center_idx ON journal_entry_lines(profit_center);
CREATE INDEX IF NOT EXISTS journal_entry_lines_order_idx ON journal_entry_lines(order_number);

-- GIN index for extensions
CREATE INDEX IF NOT EXISTS journal_entry_lines_extensions_idx ON journal_entry_lines USING GIN (extensions);

-- =============================================================================
-- Fiscal Periods Table (会计期间)
-- =============================================================================

CREATE TABLE IF NOT EXISTS fiscal_periods (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Tenant isolation
    tenant_id UUID NOT NULL,

    -- Period key
    company_code VARCHAR(10) NOT NULL,
    fiscal_year INTEGER NOT NULL,
    period INTEGER NOT NULL CHECK (period BETWEEN 1 AND 16),

    -- Status and validity
    status fiscal_period_status NOT NULL DEFAULT 'OPEN',
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,

    -- Concurrency
    version INTEGER NOT NULL DEFAULT 1,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT fiscal_periods_pk PRIMARY KEY (id),
    CONSTRAINT fiscal_periods_uq UNIQUE (tenant_id, company_code, fiscal_year, period),
    CONSTRAINT fiscal_periods_date_chk CHECK (valid_from <= valid_to)
);

-- Indexes
CREATE INDEX IF NOT EXISTS fiscal_periods_tenant_idx ON fiscal_periods(tenant_id);
CREATE INDEX IF NOT EXISTS fiscal_periods_company_idx ON fiscal_periods(tenant_id, company_code);
CREATE INDEX IF NOT EXISTS fiscal_periods_status_idx ON fiscal_periods(tenant_id, status);

-- GIST index for date range queries
CREATE INDEX IF NOT EXISTS fiscal_periods_valid_range_idx ON fiscal_periods USING GIST (daterange(valid_from, valid_to));

-- =============================================================================
-- Outbox Messages Table (事务发件箱)
-- =============================================================================

CREATE TABLE IF NOT EXISTS outbox_messages (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Tenant isolation
    tenant_id UUID NOT NULL,

    -- Event metadata
    aggregate_type VARCHAR(50) NOT NULL,
    aggregate_id VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,

    -- Payload
    payload JSONB NOT NULL,
    schema_version INTEGER NOT NULL DEFAULT 1,

    -- Timing
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Processing status
    status outbox_status NOT NULL DEFAULT 'Pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    sent_at TIMESTAMPTZ,

    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Constraints
    CONSTRAINT outbox_messages_pk PRIMARY KEY (id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS outbox_messages_status_idx ON outbox_messages(status);
CREATE INDEX IF NOT EXISTS outbox_messages_tenant_idx ON outbox_messages(tenant_id);
CREATE INDEX IF NOT EXISTS outbox_messages_occurred_at_idx ON outbox_messages(occurred_at);
CREATE INDEX IF NOT EXISTS outbox_messages_pending_idx ON outbox_messages(status, occurred_at)
    WHERE status = 'Pending';
CREATE INDEX IF NOT EXISTS outbox_messages_aggregate_idx ON outbox_messages(aggregate_type, aggregate_id);

-- Partial index for failed messages retry
CREATE INDEX IF NOT EXISTS outbox_messages_failed_idx ON outbox_messages(attempts, occurred_at)
    WHERE status = 'Failed' AND attempts < 5;

-- =============================================================================
-- Audit Log Table (审计日志)
-- =============================================================================

CREATE TABLE IF NOT EXISTS journal_audit_log (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Tenant isolation
    tenant_id UUID NOT NULL,

    -- Change information
    table_name VARCHAR(50) NOT NULL,
    record_id UUID NOT NULL,
    action audit_action NOT NULL,

    -- Before/After values
    old_value JSONB,
    new_value JSONB,

    -- Who/When/Where
    changed_by UUID,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    client_ip INET,
    request_id UUID,

    -- Additional context
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Constraints
    CONSTRAINT journal_audit_log_pk PRIMARY KEY (id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS journal_audit_log_tenant_idx ON journal_audit_log(tenant_id);
CREATE INDEX IF NOT EXISTS journal_audit_log_table_record_idx ON journal_audit_log(table_name, record_id);
CREATE INDEX IF NOT EXISTS journal_audit_log_changed_at_idx ON journal_audit_log(changed_at DESC);
CREATE INDEX IF NOT EXISTS journal_audit_log_changed_by_idx ON journal_audit_log(changed_by);
CREATE INDEX IF NOT EXISTS journal_audit_log_request_idx ON journal_audit_log(request_id);

-- GIN index for JSONB values
CREATE INDEX IF NOT EXISTS journal_audit_log_old_value_idx ON journal_audit_log USING GIN (old_value);
CREATE INDEX IF NOT EXISTS journal_audit_log_new_value_idx ON journal_audit_log USING GIN (new_value);

-- =============================================================================
-- Processed Events Table (投影幂等性)
-- =============================================================================

CREATE TABLE IF NOT EXISTS processed_events (
    -- Primary key
    event_id UUID PRIMARY KEY,

    -- Event information
    event_type VARCHAR(100) NOT NULL,
    aggregate_id VARCHAR(100) NOT NULL,

    -- Processing
    processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT processed_events_pk PRIMARY KEY (event_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS processed_events_aggregate_idx ON processed_events(event_type, aggregate_id);
CREATE INDEX IF NOT EXISTS processed_events_processed_at_idx ON processed_events(processed_at DESC);

-- =============================================================================
-- Comments
-- =============================================================================

COMMENT ON TABLE journal_entries IS '会计凭证抬头表';
COMMENT ON TABLE journal_entry_lines IS '会计凭证行项目表';
COMMENT ON TABLE fiscal_periods IS '会计期间表';
COMMENT ON TABLE outbox_messages IS '事务发件箱，用于可靠事件发布';
COMMENT ON TABLE journal_audit_log IS '审计日志表，记录所有数据变更';
COMMENT ON TABLE processed_events IS '已处理事件表，用于投影幂等性';

COMMENT ON COLUMN journal_entries.tenant_id IS '租户ID，多租户隔离标识';
COMMENT ON COLUMN journal_entries.version IS '乐观锁版本号';
COMMENT ON COLUMN journal_entries.extensions IS '扩展属性，JSONB格式存储自定义字段';
COMMENT ON COLUMN outbox_messages.schema_version IS '事件Schema版本，支持演进';
