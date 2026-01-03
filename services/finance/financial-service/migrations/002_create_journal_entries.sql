-- ============================================
-- 会计凭证表 (Journal Entries)
-- 参考 SAP BKPF, BSEG 表结构
-- ============================================

-- 凭证头表
CREATE TABLE IF NOT EXISTS journal_entries (
    -- 复合主键
    company_code VARCHAR(4) NOT NULL,
    document_number VARCHAR(10) NOT NULL,
    fiscal_year VARCHAR(4) NOT NULL,

    -- 基本字段
    document_type VARCHAR(2) NOT NULL DEFAULT 'SA',
    document_date DATE NOT NULL,
    posting_date DATE NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'CNY',
    exchange_rate DECIMAL(10, 5),

    -- 参考信息
    reference_document VARCHAR(16),
    header_text VARCHAR(25),

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=预制, 2=已过账, 3=已冲销, 4=冻结, 5=删除

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- 冲销信息
    reversal_document_number VARCHAR(10),
    reversal_date DATE,
    reversal_reason VARCHAR(2),

    PRIMARY KEY (company_code, document_number, fiscal_year)
);

-- 凭证行项目表
CREATE TABLE IF NOT EXISTS journal_entry_items (
    -- 复合主键
    company_code VARCHAR(4) NOT NULL,
    document_number VARCHAR(10) NOT NULL,
    fiscal_year VARCHAR(4) NOT NULL,
    line_number INTEGER NOT NULL,

    -- 基本字段
    gl_account VARCHAR(10) NOT NULL,
    debit_credit VARCHAR(1) NOT NULL DEFAULT 'S',  -- S=借方, H=贷方
    document_currency_amount DECIMAL(18, 2) NOT NULL DEFAULT 0,
    document_currency VARCHAR(3) NOT NULL DEFAULT 'CNY',
    local_currency_amount DECIMAL(18, 2) NOT NULL DEFAULT 0,
    local_currency VARCHAR(3) NOT NULL DEFAULT 'CNY',

    -- 业务伙伴
    customer_id VARCHAR(10),
    vendor_id VARCHAR(10),

    -- 成本对象
    cost_center VARCHAR(10),
    profit_center VARCHAR(10),
    internal_order VARCHAR(12),
    business_area VARCHAR(4),

    -- 税务
    tax_code VARCHAR(2),
    tax_amount DECIMAL(18, 2),
    tax_country VARCHAR(3),

    -- 行项目文本
    line_text VARCHAR(50),
    assignment VARCHAR(18),

    -- 参考字段
    reference_key_1 VARCHAR(10),
    reference_key_2 VARCHAR(10),
    reference_key_3 VARCHAR(20),

    -- 清账信息
    clearing_document_number VARCHAR(10),
    clearing_date DATE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (company_code, document_number, fiscal_year, line_number),
    FOREIGN KEY (company_code, document_number, fiscal_year)
        REFERENCES journal_entries(company_code, document_number, fiscal_year)
        ON DELETE CASCADE
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_journal_entries_company ON journal_entries(company_code);
CREATE INDEX IF NOT EXISTS idx_journal_entries_status ON journal_entries(status);
CREATE INDEX IF NOT EXISTS idx_journal_entries_posting_date ON journal_entries(posting_date);
CREATE INDEX IF NOT EXISTS idx_journal_entries_fiscal_year ON journal_entries(fiscal_year);

CREATE INDEX IF NOT EXISTS idx_journal_entry_items_document ON journal_entry_items(company_code, document_number, fiscal_year);
CREATE INDEX IF NOT EXISTS idx_journal_entry_items_gl_account ON journal_entry_items(gl_account);
CREATE INDEX IF NOT EXISTS idx_journal_entry_items_customer ON journal_entry_items(customer_id);
CREATE INDEX IF NOT EXISTS idx_journal_entry_items_vendor ON journal_entry_items(vendor_id);
CREATE INDEX IF NOT EXISTS idx_journal_entry_items_cost_center ON journal_entry_items(cost_center);

COMMENT ON TABLE journal_entries IS '会计凭证头表';
COMMENT ON TABLE journal_entry_items IS '会计凭证行项目表';
COMMENT ON COLUMN journal_entries.document_type IS '凭证类型: SA=总账凭证, KR=供应商发票, KZ=供应商付款, RV=客户发票, DZ=客户付款, AB=清算凭证, AA=资产凭证';
COMMENT ON COLUMN journal_entries.status IS '凭证状态: 1=预制, 2=已过账, 3=已冲销, 4=冻结, 5=删除';
