# KILLER ERP - 主数据契约模块

全局主数据契约定义，提供跨服务共享的领域模型和事件定义。

## 📦 模块结构

```
libs/master-data/
├── organizational-units/    # 组织单元主数据
│   ├── CompanyCode         # 公司代码 (T001)
│   ├── Plant               # 工厂 (T001W)
│   ├── StorageLocation     # 库存地点 (T001L)
│   ├── PurchasingOrganization  # 采购组织 (T024E)
│   └── SalesOrganization   # 销售组织 (TVKO)
│
├── business-partner/        # 业务伙伴主数据
│   ├── BusinessPartner     # 业务伙伴 (BUT000)
│   ├── CustomerRole        # 客户角色 (KNA1)
│   └── SupplierRole        # 供应商角色 (LFA1)
│
├── material/                # 物料主数据
│   ├── Material            # 物料基本数据 (MARA)
│   ├── MaterialPlantData   # 物料工厂数据 (MARC)
│   └── MaterialStorageData # 物料库存地点数据 (MARD)
│
└── cost-center/             # 成本中心主数据
    ├── CostCenter          # 成本中心 (CSKS)
    ├── ProfitCenter        # 利润中心 (CEPC)
    └── CostElement         # 成本要素 (CSKB)
```

## 🎯 设计原则

### 1. 多租户支持
所有实体包含 `tenant_id` 字段，支持 SaaS 多租户隔离。

```rust
pub struct CompanyCode {
    pub code: CompanyCodeValue,
    pub tenant_id: String,  // 租户隔离
    // ...
}
```

### 2. 时间有效性
支持时间依赖的主数据，使用 `ValidityRange` 表示有效期。

```rust
pub struct ValidityRange {
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,  // None = 无限期
}

// 检查有效性
if plant.is_valid_at(Utc::now()) {
    // 使用工厂数据
}
```

### 3. 审计与事件溯源
所有变更通过事件记录，支持 Delta 追踪和完整快照。

```rust
pub struct CompanyCodeChangedEvent {
    pub header: ChangeEventHeader,
    pub code: CompanyCodeValue,
    pub changes: Vec<FieldDelta>,      // 字段级变更
    pub snapshot: Option<CompanyCode>, // 完整快照
}
```

### 4. 扩展字段
使用 `Extensions` HashMap 支持自定义字段，避免频繁修改 Protobuf。

```rust
let mut company = CompanyCode::new(...)?;
company.extensions.set(
    "sap_bukrs".to_string(),
    json!("1000")
);
```

### 5. 软删除
使用 `deleted` 标记代替物理删除，保留历史数据。

```rust
company.mark_deleted();  // 软删除
assert!(company.deleted);
```

## 🚀 快速开始

### 添加依赖

```toml
[dependencies]
killer-master-data-organizational-units = { path = "libs/master-data/organizational-units" }
killer-master-data-business-partner = { path = "libs/master-data/business-partner" }
killer-master-data-material = { path = "libs/master-data/material" }
killer-master-data-cost-center = { path = "libs/master-data/cost-center" }
```

### 基本使用

```rust
use killer_master_data_organizational_units::*;
use killer_master_data_business_partner::*;
use killer_master_data_material::*;

// 创建公司代码
let company = CompanyCode::new(
    "tenant-001",
    "1000",
    "示例公司",
    "CN",
    "CNY",
)?;

// 创建工厂
let plant = Plant::new(
    "tenant-001",
    "1000",
    "SH01",
    "上海工厂",
    "Shanghai",
    "CN",
    Some(Utc::now()),
    None,
)?;

// 创建业务伙伴
let partner = BusinessPartner::new(
    "tenant-001",
    "BP-001",
    "客户A",
    PartnerType::Organization,
)?;

// 创建客户角色
let customer = CustomerRole::new(
    "tenant-001",
    "BP-001",
    "1000",
    "NET30",
    Money::new(100000.0, "CNY")?,
)?;

// 创建物料
let material = Material::new(
    "tenant-001",
    "MAT-001",
    "示例物料",
    MaterialType::FinishedProduct,
    "EA",
)?;
```

## 📡 事件集成

### 发布事件 (在 MDG 服务中)

```rust
use killer_master_data_organizational_units::*;

// 创建变更事件
let event = CompanyCodeChangedEvent {
    header: ChangeEventHeader::new(
        "tenant-001",
        "user-123",
        ChangeEventType::Created,
    ),
    code: company.code.clone(),
    changes: vec![],
    snapshot: Some(company.clone()),
};

// 发布到 Kafka
kafka_producer.send(
    "master-data.company-code.events",
    &event.header.event_id.to_string(),
    &serde_json::to_vec(&event)?,
).await?;
```

### 订阅事件 (在业务服务中)

```rust
use killer_master_data_organizational_units::*;

// Kafka 消费者
let mut consumer = kafka_consumer.subscribe(&[
    "master-data.company-code.events",
    "master-data.plant.events",
])?;

while let Some(message) = consumer.recv().await {
    let event: CompanyCodeChangedEvent = serde_json::from_slice(&message.payload)?;
    
    match event.header.event_type {
        ChangeEventType::Created => {
            // 同步到本地缓存
            local_cache.insert(event.code.clone(), event.snapshot.unwrap());
        }
        ChangeEventType::Updated => {
            // 更新本地缓存
            local_cache.update(event.code.clone(), event.snapshot.unwrap());
        }
        ChangeEventType::Deleted => {
            // 从本地缓存删除
            local_cache.remove(&event.code);
        }
        _ => {}
    }
}
```

## 🔄 SAP 集成映射

### 组织单元映射

| KILLER 实体 | SAP 表 | 说明 |
|------------|--------|------|
| CompanyCode | T001 | 公司代码 |
| Plant | T001W | 工厂 |
| StorageLocation | T001L | 库存地点 |
| PurchasingOrganization | T024E | 采购组织 |
| SalesOrganization | TVKO | 销售组织 |

### 业务伙伴映射

| KILLER 实体 | SAP 表 | 说明 |
|------------|--------|------|
| BusinessPartner | BUT000 | 业务伙伴 |
| CustomerRole | KNA1 | 客户主数据 |
| SupplierRole | LFA1 | 供应商主数据 |

### 物料映射

| KILLER 实体 | SAP 表 | 说明 |
|------------|--------|------|
| Material | MARA | 物料基本数据 |
| MaterialPlantData | MARC | 物料工厂数据 |
| MaterialStorageData | MARD | 物料库存地点数据 |

### 成本中心映射

| KILLER 实体 | SAP 表 | 说明 |
|------------|--------|------|
| CostCenter | CSKS | 成本中心 |
| ProfitCenter | CEPC | 利润中心 |
| CostElement | CSKB | 成本要素 |

## 🏗️ 架构模式

### 1. 主数据治理服务 (MDG Service)

```rust
// services/infrastructure/mdg-service/src/application/company_code_service.rs

pub struct CompanyCodeService {
    repository: Arc<dyn CompanyCodeRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CompanyCodeService {
    pub async fn create_company_code(
        &self,
        cmd: CreateCompanyCodeCommand,
    ) -> Result<CompanyCode, DomainError> {
        // 1. 创建实体
        let company = CompanyCode::new(
            &cmd.tenant_id,
            &cmd.code,
            &cmd.name,
            &cmd.country,
            &cmd.currency_code,
        )?;
        
        // 2. 持久化
        self.repository.save(&company).await?;
        
        // 3. 发布事件
        let event = CompanyCodeChangedEvent {
            header: ChangeEventHeader::new(
                &cmd.tenant_id,
                &cmd.actor_id,
                ChangeEventType::Created,
            ),
            code: company.code.clone(),
            changes: vec![],
            snapshot: Some(company.clone()),
        };
        self.event_bus.publish("master-data.company-code.events", event).await?;
        
        Ok(company)
    }
}
```

### 2. 业务服务本地缓存

```rust
// services/finance/gl-service/src/infrastructure/master_data_cache.rs

pub struct MasterDataCache {
    companies: Arc<RwLock<HashMap<CompanyCodeValue, CompanyCode>>>,
    plants: Arc<RwLock<HashMap<PlantValue, Plant>>>,
}

impl MasterDataCache {
    pub async fn sync_from_events(&self, event: CompanyCodeChangedEvent) {
        match event.header.event_type {
            ChangeEventType::Created | ChangeEventType::Updated => {
                if let Some(snapshot) = event.snapshot {
                    self.companies.write().await.insert(event.code, snapshot);
                }
            }
            ChangeEventType::Deleted => {
                self.companies.write().await.remove(&event.code);
            }
            _ => {}
        }
    }
    
    pub async fn get_company(&self, code: &CompanyCodeValue) -> Option<CompanyCode> {
        self.companies.read().await.get(code).cloned()
    }
}
```

### 3. 层级验证

```rust
// 验证工厂是否属于公司代码
pub async fn validate_plant_hierarchy(
    plant: &Plant,
    company: &CompanyCode,
) -> Result<(), ValidationError> {
    if plant.company_code != company.code {
        return Err(ValidationError::HierarchyMismatch {
            message: format!(
                "Plant {} does not belong to company {}",
                plant.code, company.code
            ),
        });
    }
    
    if plant.tenant_id != company.tenant_id {
        return Err(ValidationError::TenantMismatch);
    }
    
    Ok(())
}
```

## 🧪 测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_company_code_creation() {
        let company = CompanyCode::new(
            "tenant-001",
            "1000",
            "Test Company",
            "US",
            "USD",
        ).unwrap();
        
        assert_eq!(company.code.as_str(), "1000");
        assert_eq!(company.tenant_id, "tenant-001");
        assert!(!company.deleted);
    }
    
    #[test]
    fn test_validity_range() {
        let now = Utc::now();
        let future = now + chrono::Duration::days(30);
        
        let range = ValidityRange::new(now, Some(future));
        
        assert!(range.is_currently_valid());
        assert!(range.is_valid_at(now + chrono::Duration::days(15)));
        assert!(!range.is_valid_at(future + chrono::Duration::days(1)));
    }
    
    #[test]
    fn test_material_stock_operations() {
        let mut storage = MaterialStorageData::new(
            "tenant-001",
            "MAT-001",
            "1000",
            "SL01",
            Quantity::new(100.0, "EA").unwrap(),
        ).unwrap();
        
        // 增加库存
        storage.increase_stock(Quantity::new(50.0, "EA").unwrap()).unwrap();
        assert_eq!(storage.unrestricted_stock.value(), 150.0);
        
        // 减少库存
        storage.decrease_stock(Quantity::new(30.0, "EA").unwrap()).unwrap();
        assert_eq!(storage.unrestricted_stock.value(), 120.0);
        
        // 库存不足
        let result = storage.decrease_stock(Quantity::new(200.0, "EA").unwrap());
        assert!(result.is_err());
    }
}
```

## 📚 相关文档

- [组织单元详细文档](./organizational-units/README.md)
- [业务伙伴详细文档](./business-partner/README.md)
- [物料详细文档](./material/README.md)
- [成本中心详细文档](./cost-center/README.md)

## 🤝 贡献指南

1. 所有主数据实体必须包含 `tenant_id` 字段
2. 时间依赖数据必须使用 `ValidityRange`
3. 所有变更必须发布事件
4. 使用 `Extensions` 支持自定义字段
5. 实现软删除而非物理删除
6. 添加完整的单元测试
7. 更新 SAP 映射文档

## 📄 许可证

MIT OR Apache-2.0
