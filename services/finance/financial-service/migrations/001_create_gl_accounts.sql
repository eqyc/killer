-- ============================================
-- 总账科目表 (GL Accounts)
-- 参考 SAP SKA1, SKB1 表结构
-- ============================================

-- 主数据表
CREATE TABLE IF NOT EXISTS gl_accounts (
    -- 复合主键
    chart_of_accounts VARCHAR(4) NOT NULL,
    account_code VARCHAR(10) NOT NULL,
    company_code VARCHAR(4) NOT NULL,

    -- 基本字段
    account_type VARCHAR(1) NOT NULL DEFAULT 'P',
    balance_sheet_indicator VARCHAR(1) NOT NULL DEFAULT '',
    cost_control_area VARCHAR(4) NOT NULL DEFAULT '',
    currency VARCHAR(3) NOT NULL DEFAULT 'CNY',
    account_group VARCHAR(4) NOT NULL DEFAULT '',
    account_indicator_group VARCHAR(2) NOT NULL DEFAULT '',

    -- 描述
    description VARCHAR(50) NOT NULL DEFAULT '',
    short_description VARCHAR(20) NOT NULL DEFAULT '',
    long_description VARCHAR(255),

    -- 合并科目
    consolidation_account VARCHAR(10),

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- 删除标识
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (chart_of_accounts, account_code, company_code)
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_gl_accounts_company ON gl_accounts(company_code);
CREATE INDEX IF NOT EXISTS idx_gl_accounts_account_type ON gl_accounts(account_type);
CREATE INDEX IF NOT EXISTS idx_gl_accounts_cost_center ON gl_accounts(cost_control_area);
CREATE INDEX IF NOT EXISTS idx_gl_accounts_chart ON gl_accounts(chart_of_accounts);

COMMENT ON TABLE gl_accounts IS '总账科目主数据表';
COMMENT ON COLUMN gl_accounts.chart_of_accounts IS '科目表代码 (KOART)';
COMMENT ON COLUMN gl_accounts.account_code IS '科目代码 (SAKNR)';
COMMENT ON COLUMN gl_accounts.company_code IS '公司代码 (BUKRS)';
COMMENT ON COLUMN gl_accounts.account_type IS '科目类型: A=资产, D=成本/费用, L=负债, H=权益, R=收入';
COMMENT ON COLUMN gl_accounts.balance_sheet_indicator IS '资产负债表科目标识: X=资产负债表科目';
COMMENT ON COLUMN gl_accounts.cost_control_area IS '成本控制范围 (KOKRS)';
