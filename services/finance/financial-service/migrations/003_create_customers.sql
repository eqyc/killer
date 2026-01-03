-- ============================================
-- 客户主数据表 (Customers)
-- 参考 SAP KNA1, KNB1, KNVV 表结构
-- ============================================

-- 客户一般数据
CREATE TABLE IF NOT EXISTS customers (
    customer_id VARCHAR(10) NOT NULL,

    -- 基本字段
    account_group VARCHAR(4) NOT NULL,
    company_code VARCHAR(4) NOT NULL,
    name_1 VARCHAR(35) NOT NULL,
    name_2 VARCHAR(35),
    street VARCHAR(35),
    city VARCHAR(35),
    postal_code VARCHAR(10),
    country VARCHAR(3) NOT NULL DEFAULT 'CNY',
    language VARCHAR(1) NOT NULL DEFAULT 'Z',

    -- 税务信息
    tax_number_1 VARCHAR(18),
    tax_number_2 VARCHAR(18),
    tax_number_3 VARCHAR(18),
    tax_number_4 VARCHAR(18),
    tax_number_5 VARCHAR(18),
    vat_registration_number VARCHAR(20),

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=激活, 2=冻结, 3=删除

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (customer_id)
);

-- 客户公司代码数据
CREATE TABLE IF NOT EXISTS customer_company_data (
    customer_id VARCHAR(10) NOT NULL,
    company_code VARCHAR(4) NOT NULL,

    -- 财务信息
    reconciliation_account VARCHAR(10) NOT NULL,
    payment_terms VARCHAR(4) NOT NULL DEFAULT '0001',
    payment_methods VARCHAR(10),

    -- 银行信息
    house_bank VARCHAR(4),
    account_holder_name VARCHAR(60),
    bank_account VARCHAR(18),
    bank_key VARCHAR(15),

    -- 信用管理
    credit_limit DECIMAL(18, 2),
    credit_area VARCHAR(3),

    -- 催收
    dunning_area VARCHAR(2),
    dunning_procedure VARCHAR(2),
    last_dunning_date DATE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (customer_id, company_code),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE CASCADE
);

-- 客户销售数据
CREATE TABLE IF NOT EXISTS customer_sales_data (
    customer_id VARCHAR(10) NOT NULL,
    sales_organization VARCHAR(4) NOT NULL,
    distribution_channel VARCHAR(2) NOT NULL,
    division VARCHAR(2) NOT NULL,

    -- 销售相关
    currency VARCHAR(3) NOT NULL DEFAULT 'CNY',
    price_group VARCHAR(2),
    customer_pricing_procedure VARCHAR(2),
    order_combination_allowed BOOLEAN NOT NULL DEFAULT FALSE,
    partial_delivery_allowed BOOLEAN NOT NULL DEFAULT TRUE,

    -- 装运
    shipping_condition VARCHAR(2),
    delivery_plant VARCHAR(4),

    -- 发票
    invoice_dates VARCHAR(2),
    billing_plan VARCHAR(2),

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (customer_id, sales_organization, distribution_channel, division),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE CASCADE
);

-- 客户联系人
CREATE TABLE IF NOT EXISTS customer_contacts (
    customer_id VARCHAR(10) NOT NULL,
    contact_number SERIAL NOT NULL,

    contact_name VARCHAR(40),
    department VARCHAR(40),
    phone_number VARCHAR(30),
    mobile_number VARCHAR(30),
    email_address VARCHAR(130),

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (customer_id, contact_number),
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_customers_company ON customers(company_code);
CREATE INDEX IF NOT EXISTS idx_customers_country ON customers(country);
CREATE INDEX IF NOT EXISTS idx_customers_status ON customers(status);
CREATE INDEX IF NOT EXISTS idx_customers_account_group ON customers(account_group);

CREATE INDEX IF NOT EXISTS idx_customer_company_data_company ON customer_company_data(company_code);
CREATE INDEX IF NOT EXISTS idx_customer_company_data_reconciliation ON customer_company_data(reconciliation_account);

COMMENT ON TABLE customers IS '客户一般主数据';
COMMENT ON TABLE customer_company_data IS '客户公司代码数据';
COMMENT ON TABLE customer_sales_data IS '客户销售数据';
COMMENT ON COLUMN customers.status IS '客户状态: 1=激活, 2=冻结, 3=删除';
