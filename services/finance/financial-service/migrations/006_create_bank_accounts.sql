-- ============================================
-- 银行主数据表 (Bank Accounts)
-- 参考 SAP BNKA, BUT0B, BUT0K 表结构
-- ============================================

-- 银行主数据
CREATE TABLE IF NOT EXISTS banks (
    bank_country_code VARCHAR(3) NOT NULL,
    bank_key VARCHAR(15) NOT NULL,

    -- 基本信息
    bank_name VARCHAR(60) NOT NULL,
    bank_name_2 VARCHAR(40),
    street_address VARCHAR(40),
    city VARCHAR(35),
    postal_code VARCHAR(10),
    region VARCHAR(3),
    country VARCHAR(3),

    -- 联系信息
    telephone_number_1 VARCHAR(30),
    telephone_number_2 VARCHAR(30),
    fax_number VARCHAR(30),
    email_address VARCHAR(130),
    website VARCHAR(130),

    -- SWIFT/BIC
    swift_code VARCHAR(11),

    -- 状态
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (bank_country_code, bank_key)
);

-- 银行账户
CREATE TABLE IF NOT EXISTS bank_accounts (
    bank_country_code VARCHAR(3) NOT NULL,
    bank_key VARCHAR(15) NOT NULL,
    bank_account_number VARCHAR(18) NOT NULL,

    -- 账户信息
    bank_account_holder_name VARCHAR(60) NOT NULL,
    account_type VARCHAR(2) NOT NULL DEFAULT '01',  -- 01=往来账户, 02=定期账户, 03=储蓄账户
    account_sub_type VARCHAR(2),
    account_currency VARCHAR(3) NOT NULL DEFAULT 'CNY',

    -- IBAN
    iban VARCHAR(34),

    -- 状态
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    block_reason VARCHAR(4),

    -- 余额信息
    current_balance DECIMAL(18, 2) NOT NULL DEFAULT 0,
    available_balance DECIMAL(18, 2) NOT NULL DEFAULT 0,
    booked_balance DECIMAL(18, 2) NOT NULL DEFAULT 0,
    last_balance_update TIMESTAMP WITH TIME ZONE,

    -- 账户用途
    usage_type VARCHAR(2),  -- OP=运营账户, TL=资金归集账户, CL=清算账户
    company_code VARCHAR(4),

    -- 开户信息
    account_opening_date DATE,
    account_closing_date DATE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (bank_country_code, bank_key, bank_account_number),
    FOREIGN KEY (bank_country_code, bank_key) REFERENCES banks(bank_country_code, bank_key) ON DELETE CASCADE
);

-- 银行账户交易
CREATE TABLE IF NOT EXISTS bank_transactions (
    bank_country_code VARCHAR(3) NOT NULL,
    bank_key VARCHAR(15) NOT NULL,
    bank_account_number VARCHAR(18) NOT NULL,
    transaction_sequence SERIAL NOT NULL,

    -- 交易信息
    transaction_date TIMESTAMP WITH TIME ZONE NOT NULL,
    value_date DATE NOT NULL,
    transaction_type VARCHAR(4) NOT NULL,  -- C=存款, D=取款, T=转账, F=费用, I=利息
    transaction_amount DECIMAL(18, 2) NOT NULL,
    transaction_currency VARCHAR(3) NOT NULL,

    -- 对方信息
    counterparty_bank_key VARCHAR(15),
    counterparty_account_number VARCHAR(34),
    counterparty_name VARCHAR(60),

    -- 参考
    reference VARCHAR(30),
    bank_reference VARCHAR(20),
    customer_reference VARCHAR(20),

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=待处理, 2=已处理, 3=已取消

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (bank_country_code, bank_key, bank_account_number, transaction_sequence),
    FOREIGN KEY (bank_country_code, bank_key, bank_account_number)
        REFERENCES bank_accounts(bank_country_code, bank_key, bank_account_number)
        ON DELETE CASCADE
);

-- 银行对账
CREATE TABLE IF NOT EXISTS bank_reconciliations (
    bank_country_code VARCHAR(3) NOT NULL,
    bank_key VARCHAR(15) NOT NULL,
    bank_account_number VARCHAR(18) NOT NULL,
    reconciliation_date DATE NOT NULL,

    -- 对账期间
    statement_start_date DATE NOT NULL,
    statement_end_date DATE NOT NULL,

    -- 余额信息
    opening_balance DECIMAL(18, 2) NOT NULL,
    closing_balance DECIMAL(18, 2) NOT NULL,
    statement_balance DECIMAL(18, 2) NOT NULL,

    -- 差异
    difference_amount DECIMAL(18, 2) NOT NULL DEFAULT 0,
    difference_reason VARCHAR(255),

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=待对账, 2=已对账, 3=差异待处理

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reconciled_by VARCHAR(12),
    reconciled_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (bank_country_code, bank_key, bank_account_number, reconciliation_date),
    FOREIGN KEY (bank_country_code, bank_key, bank_account_number)
        REFERENCES bank_accounts(bank_country_code, bank_key, bank_account_number)
        ON DELETE CASCADE
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_banks_country ON banks(bank_country_code);
CREATE INDEX IF NOT EXISTS idx_banks_swift ON banks(swift_code);

CREATE INDEX IF NOT EXISTS idx_bank_accounts_company ON bank_accounts(company_code);
CREATE INDEX IF NOT EXISTS idx_bank_accounts_currency ON bank_accounts(account_currency);
CREATE INDEX IF NOT EXISTS idx_bank_accounts_status ON bank_accounts(is_active);

CREATE INDEX IF NOT EXISTS idx_bank_transactions_date ON bank_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_bank_transactions_type ON bank_transactions(transaction_type);

COMMENT ON TABLE banks IS '银行主数据';
COMMENT ON TABLE bank_accounts IS '银行账户';
COMMENT ON TABLE bank_transactions IS '银行账户交易';
COMMENT ON TABLE bank_reconciliations IS '银行对账';
COMMENT ON COLUMN bank_accounts.account_type IS '账户类型: 01=往来账户, 02=定期账户, 03=储蓄账户';
