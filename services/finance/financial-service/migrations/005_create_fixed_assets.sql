-- ============================================
-- 固定资产主数据表 (Fixed Assets)
-- 参考 SAP ANLA, ANLB, ANLC, ANLH 表结构
-- ============================================

-- 固定资产主数据
CREATE TABLE IF NOT EXISTS fixed_assets (
    company_code VARCHAR(4) NOT NULL,
    asset_number VARCHAR(12) NOT NULL,
    sub_number VARCHAR(4) NOT NULL DEFAULT '0',

    -- 分类信息
    asset_class VARCHAR(8) NOT NULL,
    asset_sub_class VARCHAR(8),
    asset_group VARCHAR(8),

    -- 描述
    description VARCHAR(50) NOT NULL DEFAULT '',
    description_2 VARCHAR(50),
    serial_number VARCHAR(18),
    model_number VARCHAR(20),

    -- 组织信息
    cost_center VARCHAR(10),
    profit_center VARCHAR(10),
    business_area VARCHAR(4),
    plant VARCHAR(4),
    location VARCHAR(10),
    room VARCHAR(10),
    floor VARCHAR(3),
    building VARCHAR(10),

    -- 计量单位
    quantity INTEGER,
    base_unit VARCHAR(3),

    -- 供应商
    vendor_account_number VARCHAR(10),
    manufacturer VARCHAR(20),
    manufacturer_part_number VARCHAR(20),

    -- 采购信息
    acquisition_date DATE,
    capitalization_date DATE,
    decommissioning_date DATE,
    useful_life_years INTEGER,
    useful_life_periods INTEGER,

    -- 状态
    status VARCHAR(1) NOT NULL DEFAULT '1',  -- 1=新建, 2=已资本化, 3=已报废, 4=冻结

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (company_code, asset_number, sub_number)
);

-- 资产价值表
CREATE TABLE IF NOT EXISTS fixed_asset_values (
    company_code VARCHAR(4) NOT NULL,
    asset_number VARCHAR(12) NOT NULL,
    sub_number VARCHAR(4) NOT NULL DEFAULT '0',
    fiscal_year VARCHAR(4) NOT NULL,

    -- 价值信息
    acquisition_value DECIMAL(18, 2) NOT NULL DEFAULT 0,
    accumulated_depreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    current_year_depreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    ordinary_depreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    special_depreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    unplanned_depreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    appreciation DECIMAL(18, 2) NOT NULL DEFAULT 0,
    net_book_value DECIMAL(18, 2) NOT NULL DEFAULT 0,
    depreciation_base DECIMAL(18, 2) NOT NULL DEFAULT 0,

    -- 残值
    scrap_value DECIMAL(18, 2) NOT NULL DEFAULT 0,

    -- 折旧码
    depreciation_key VARCHAR(4),

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (company_code, asset_number, sub_number, fiscal_year),
    FOREIGN KEY (company_code, asset_number, sub_number)
        REFERENCES fixed_assets(company_code, asset_number, sub_number)
        ON DELETE CASCADE
);

-- 资产转移历史
CREATE TABLE IF NOT EXISTS fixed_asset_transfers (
    company_code VARCHAR(4) NOT NULL,
    asset_number VARCHAR(12) NOT NULL,
    sub_number VARCHAR(4) NOT NULL DEFAULT '0',
    transfer_sequence SERIAL NOT NULL,

    -- 转移信息
    transfer_date DATE NOT NULL,
    old_cost_center VARCHAR(10),
    new_cost_center VARCHAR(10),
    old_profit_center VARCHAR(10),
    new_profit_center VARCHAR(10),
    old_business_area VARCHAR(4),
    new_business_area VARCHAR(4),
    old_plant VARCHAR(4),
    new_plant VARCHAR(4),
    old_location VARCHAR(10),
    new_location VARCHAR(10),

    -- 参考
    document_number VARCHAR(10),
    document_date DATE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (company_code, asset_number, sub_number, transfer_sequence),
    FOREIGN KEY (company_code, asset_number, sub_number)
        REFERENCES fixed_assets(company_code, asset_number, sub_number)
        ON DELETE CASCADE
);

-- 资产报废记录
CREATE TABLE IF NOT EXISTS fixed_asset_retirements (
    company_code VARCHAR(4) NOT NULL,
    asset_number VARCHAR(12) NOT NULL,
    sub_number VARCHAR(4) NOT NULL DEFAULT '0',
    retirement_sequence SERIAL NOT NULL,

    -- 报废信息
    retirement_date DATE NOT NULL,
    retirement_type VARCHAR(1) NOT NULL,  -- X=完全报废, P=部分报废
    retirement_value DECIMAL(18, 2) NOT NULL DEFAULT 0,
    revenue_from_retirement DECIMAL(18, 2) NOT NULL DEFAULT 0,
    cost_of_removal DECIMAL(18, 2) NOT NULL DEFAULT 0,

    -- 参考
    document_number VARCHAR(10),
    document_date DATE,

    -- 审计字段
    created_by VARCHAR(12) NOT NULL DEFAULT 'SYSTEM',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (company_code, asset_number, sub_number, retirement_sequence),
    FOREIGN KEY (company_code, asset_number, sub_number)
        REFERENCES fixed_assets(company_code, asset_number, sub_number)
        ON DELETE CASCADE
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_fixed_assets_company ON fixed_assets(company_code);
CREATE INDEX IF NOT EXISTS idx_fixed_assets_class ON fixed_assets(asset_class);
CREATE INDEX IF NOT EXISTS idx_fixed_assets_cost_center ON fixed_assets(cost_center);
CREATE INDEX IF NOT EXISTS idx_fixed_assets_profit_center ON fixed_assets(profit_center);
CREATE INDEX IF NOT EXISTS idx_fixed_assets_status ON fixed_assets(status);
CREATE INDEX IF NOT EXISTS idx_fixed_assets_capitalization_date ON fixed_assets(capitalization_date);

CREATE INDEX IF NOT EXISTS idx_fixed_asset_values_year ON fixed_asset_values(fiscal_year);

COMMENT ON TABLE fixed_assets IS '固定资产主数据';
COMMENT ON TABLE fixed_asset_values IS '固定资产价值表';
COMMENT ON TABLE fixed_asset_transfers IS '固定资产转移历史';
COMMENT ON TABLE fixed_asset_retirements IS '固定资产报废记录';
COMMENT ON COLUMN fixed_assets.status IS '资产状态: 1=新建, 2=已资本化, 3=已报废, 4=冻结';
