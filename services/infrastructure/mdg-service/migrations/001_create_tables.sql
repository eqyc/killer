-- =============================================================================
-- MDG 服务数据库迁移 - 创建表
-- =============================================================================

-- 启用 UUID 扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- 物料表
-- =============================================================================

CREATE TABLE materials (
    material_number VARCHAR(18) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    description VARCHAR(200) NOT NULL,
    material_type VARCHAR(50) NOT NULL,
    base_unit VARCHAR(3) NOT NULL,
    material_group VARCHAR(20),
    gross_weight DECIMAL(15,3),
    net_weight DECIMAL(15,3),
    volume DECIMAL(15,3),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, material_number)
);

CREATE INDEX idx_materials_tenant ON materials(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_materials_type ON materials(tenant_id, material_type) WHERE NOT deleted;
CREATE INDEX idx_materials_group ON materials(tenant_id, material_group) WHERE NOT deleted;
CREATE INDEX idx_materials_description ON materials USING gin(to_tsvector('simple', description));

COMMENT ON TABLE materials IS '物料基本数据 (MARA)';

-- =============================================================================
-- 物料工厂数据表
-- =============================================================================

CREATE TABLE material_plant_data (
    material_number VARCHAR(18) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    plant_code VARCHAR(4) NOT NULL,
    mrp_type VARCHAR(2) NOT NULL,
    procurement_type VARCHAR(20) NOT NULL,
    special_procurement VARCHAR(2),
    planned_delivery_time INTEGER,
    safety_stock DECIMAL(15,3),
    minimum_lot_size DECIMAL(15,3),
    maximum_lot_size DECIMAL(15,3),
    fixed_lot_size DECIMAL(15,3),
    reorder_point DECIMAL(15,3),
    production_supervisor VARCHAR(50),
    abc_indicator VARCHAR(1),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, material_number, plant_code),
    FOREIGN KEY (tenant_id, material_number) REFERENCES materials(tenant_id, material_number)
);

CREATE INDEX idx_material_plant_tenant ON material_plant_data(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_material_plant_plant ON material_plant_data(tenant_id, plant_code) WHERE NOT deleted;

COMMENT ON TABLE material_plant_data IS '物料工厂数据 (MARC)';

-- =============================================================================
-- 业务伙伴表
-- =============================================================================

CREATE TABLE business_partners (
    partner_id VARCHAR(10) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    name VARCHAR(200) NOT NULL,
    partner_type VARCHAR(20) NOT NULL,
    street VARCHAR(200),
    city VARCHAR(50),
    postal_code VARCHAR(20),
    country VARCHAR(3) NOT NULL,
    region VARCHAR(50),
    tax_number VARCHAR(50),
    language VARCHAR(3),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, partner_id)
);

CREATE INDEX idx_business_partners_tenant ON business_partners(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_business_partners_type ON business_partners(tenant_id, partner_type) WHERE NOT deleted;
CREATE INDEX idx_business_partners_name ON business_partners USING gin(to_tsvector('simple', name));
CREATE INDEX idx_business_partners_tax ON business_partners(tenant_id, tax_number) WHERE tax_number IS NOT NULL;

COMMENT ON TABLE business_partners IS '业务伙伴 (BUT000)';

-- =============================================================================
-- 客户角色表
-- =============================================================================

CREATE TABLE customer_roles (
    partner_id VARCHAR(10) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    sales_org VARCHAR(4) NOT NULL,
    payment_terms VARCHAR(20) NOT NULL,
    credit_limit_amount DECIMAL(15,2) NOT NULL,
    credit_limit_currency VARCHAR(3) NOT NULL,
    block_status VARCHAR(20) NOT NULL DEFAULT 'unblocked',
    customer_group VARCHAR(20),
    price_group VARCHAR(20),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, partner_id, sales_org),
    FOREIGN KEY (tenant_id, partner_id) REFERENCES business_partners(tenant_id, partner_id)
);

CREATE INDEX idx_customer_roles_tenant ON customer_roles(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_customer_roles_sales_org ON customer_roles(tenant_id, sales_org) WHERE NOT deleted;

COMMENT ON TABLE customer_roles IS '客户角色 (KNA1)';

-- =============================================================================
-- 供应商角色表
-- =============================================================================

CREATE TABLE supplier_roles (
    partner_id VARCHAR(10) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    purchasing_org VARCHAR(4) NOT NULL,
    reconciliation_account VARCHAR(10) NOT NULL,
    payment_terms VARCHAR(20) NOT NULL,
    supplier_group VARCHAR(20),
    block_status VARCHAR(20) NOT NULL DEFAULT 'unblocked',
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, partner_id, purchasing_org),
    FOREIGN KEY (tenant_id, partner_id) REFERENCES business_partners(tenant_id, partner_id)
);

CREATE INDEX idx_supplier_roles_tenant ON supplier_roles(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_supplier_roles_purchasing_org ON supplier_roles(tenant_id, purchasing_org) WHERE NOT deleted;

COMMENT ON TABLE supplier_roles IS '供应商角色 (LFA1)';

-- =============================================================================
-- 公司代码表
-- =============================================================================

CREATE TABLE company_codes (
    code VARCHAR(4) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL,
    street_address VARCHAR(200),
    city VARCHAR(50),
    postal_code VARCHAR(20),
    country VARCHAR(3) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    fiscal_year_variant VARCHAR(2),
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, code)
);

CREATE INDEX idx_company_codes_tenant ON company_codes(tenant_id) WHERE NOT deleted;

COMMENT ON TABLE company_codes IS '公司代码 (T001)';

-- =============================================================================
-- 工厂表
-- =============================================================================

CREATE TABLE plants (
    code VARCHAR(4) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    company_code VARCHAR(4) NOT NULL,
    name VARCHAR(100) NOT NULL,
    city VARCHAR(50),
    country VARCHAR(3) NOT NULL,
    region VARCHAR(50),
    valid_from TIMESTAMPTZ NOT NULL,
    valid_to TIMESTAMPTZ,
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, code),
    FOREIGN KEY (tenant_id, company_code) REFERENCES company_codes(tenant_id, code)
);

CREATE INDEX idx_plants_tenant ON plants(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_plants_company ON plants(tenant_id, company_code) WHERE NOT deleted;
CREATE INDEX idx_plants_validity ON plants(tenant_id, valid_from, valid_to) WHERE NOT deleted;

COMMENT ON TABLE plants IS '工厂 (T001W)';

-- =============================================================================
-- 成本中心表
-- =============================================================================

CREATE TABLE cost_centers (
    code VARCHAR(10) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    controlling_area VARCHAR(4) NOT NULL,
    name VARCHAR(100) NOT NULL,
    category VARCHAR(20) NOT NULL,
    responsible_person VARCHAR(50),
    department VARCHAR(50),
    company_code VARCHAR(4),
    profit_center VARCHAR(10),
    valid_from TIMESTAMPTZ NOT NULL,
    valid_to TIMESTAMPTZ,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    extensions JSONB DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(50) NOT NULL,
    updated_by VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (tenant_id, code)
);

CREATE INDEX idx_cost_centers_tenant ON cost_centers(tenant_id) WHERE NOT deleted;
CREATE INDEX idx_cost_centers_controlling_area ON cost_centers(tenant_id, controlling_area) WHERE NOT deleted;
CREATE INDEX idx_cost_centers_validity ON cost_centers(tenant_id, valid_from, valid_to) WHERE NOT deleted;

COMMENT ON TABLE cost_centers IS '成本中心 (CSKS)';

-- =============================================================================
-- 变更历史表 (通用)
-- =============================================================================

CREATE TABLE change_history (
    id BIGSERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL,
    entity_id VARCHAR(100) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    version INTEGER NOT NULL,
    operation VARCHAR(20) NOT NULL,
    changes JSONB NOT NULL,
    snapshot JSONB NOT NULL,
    actor_id VARCHAR(50) NOT NULL,
    correlation_id UUID,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_change_history_entity ON change_history(tenant_id, entity_type, entity_id, timestamp DESC);
CREATE INDEX idx_change_history_actor ON change_history(tenant_id, actor_id, timestamp DESC);
CREATE INDEX idx_change_history_correlation ON change_history(correlation_id) WHERE correlation_id IS NOT NULL;

COMMENT ON TABLE change_history IS '变更历史审计表';

-- =============================================================================
-- 数据质量评分表
-- =============================================================================

CREATE TABLE data_quality_scores (
    entity_type VARCHAR(50) NOT NULL,
    entity_id VARCHAR(100) NOT NULL,
    tenant_id VARCHAR(50) NOT NULL,
    overall_score DECIMAL(5,2) NOT NULL,
    completeness DECIMAL(5,2) NOT NULL,
    consistency DECIMAL(5,2) NOT NULL,
    accuracy DECIMAL(5,2) NOT NULL,
    issues JSONB DEFAULT '[]',
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (tenant_id, entity_type, entity_id)
);

CREATE INDEX idx_data_quality_tenant ON data_quality_scores(tenant_id, entity_type);
CREATE INDEX idx_data_quality_score ON data_quality_scores(tenant_id, overall_score);

COMMENT ON TABLE data_quality_scores IS '数据质量评分';
