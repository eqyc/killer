-- ============================================
-- 供应商主数据表 (Vendors)
-- 参考 SAP LFA1, LFB1, LFM1 表结构
-- ============================================

-- 供应商一般数据
CREATE TABLE IF NOT EXISTS vendors (
    vendor_id VARCHAR(10) NOT NULL,

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
    vat_registration_number VARCHAR(20),

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=激活, 2=冻结, 3=删除

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (vendor_id)
);

-- 供应商公司代码数据
CREATE TABLE IF NOT EXISTS vendor_company_data (
    vendor_id VARCHAR(10) NOT NULL,
    company_code VARCHAR(4) NOT NULL,

    -- 财务信息
    reconciliation_account VARCHAR(10) NOT NULL,
    payment_terms VARCHAR(4) NOT NULL DEFAULT '0001',
    payment_methods VARCHAR(10),
    payment_block VARCHAR(1) NOT NULL DEFAULT '',

    -- 银行信息
    house_bank VARCHAR(4),
    account_holder_name VARCHAR(60),
    bank_account VARCHAR(18),
    bank_key VARCHAR(15),
    iban VARCHAR(34),
    swift_code VARCHAR(11),

    -- 采购组织
    purchasing_organization VARCHAR(4),
    vendor_account_group VARCHAR(4),

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (vendor_id, company_code),
    FOREIGN KEY (vendor_id) REFERENCES vendors(vendor_id) ON DELETE CASCADE
);

-- 供应商采购数据
CREATE TABLE IF NOT EXISTS vendor_purchasing_data (
    vendor_id VARCHAR(10) NOT NULL,
    purchasing_organization VARCHAR(4) NOT NULL,
    company_code VARCHAR(4) NOT NULL,

    -- 采购相关
    currency VARCHAR(3) NOT NULL DEFAULT 'CNY',
    price_group VARCHAR(2),
    ordering_currency VARCHAR(3),
    invoice_currency VARCHAR(3),

    -- 国际贸易
    international_commercial_terms VARCHAR(3),
    port_of_loading VARCHAR(4),
    port_of_discharge VARCHAR(4),
    shipping_type VARCHAR(2),

    -- 付款
    payment_terms VARCHAR(4),
    planned_delivery_time INTEGER,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (vendor_id, purchasing_organization, company_code),
    FOREIGN KEY (vendor_id) REFERENCES vendors(vendor_id) ON DELETE CASCADE
);

-- 供应商联系人
CREATE TABLE IF NOT EXISTS vendor_contacts (
    vendor_id VARCHAR(10) NOT NULL,
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

    PRIMARY KEY (vendor_id, contact_number),
    FOREIGN KEY (vendor_id) REFERENCES vendors(vendor_id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_vendors_company ON vendors(company_code);
CREATE INDEX IF NOT EXISTS idx_vendors_country ON vendors(country);
CREATE INDEX IF NOT EXISTS idx_vendors_status ON vendors(status);
CREATE INDEX IF NOT EXISTS idx_vendors_account_group ON vendors(account_group);

CREATE INDEX IF NOT EXISTS idx_vendor_company_data_company ON vendor_company_data(company_code);
CREATE INDEX IF NOT EXISTS idx_vendor_company_data_reconciliation ON vendor_company_data(reconciliation_account);

COMMENT ON TABLE vendors IS '供应商一般主数据';
COMMENT ON TABLE vendor_company_data IS '供应商公司代码数据';
COMMENT ON TABLE vendor_purchasing_data IS '供应商采购数据';
COMMENT ON COLUMN vendors.status IS '供应商状态: 1=激活, 2=冻结, 3=删除';
