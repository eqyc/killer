# Killer Master Data - Organizational Units

组织单元主数据域，提供公司代码、工厂、库存地点等组织结构的定义和管理。

## 目录结构

```
organizational-units/
├── Cargo.toml
├── src/
│   └── lib.rs           # 领域模型 + 验证 + 事件
├── proto/
│   ├── domain.proto     # 实体定义
│   └── events.proto     # 变更事件
├── tests/
│   └── lib.rs           # 单元测试
└── README.md
```

## SAP 映射

| 类型 | SAP 表 | 字段 | 用途 | 示例 |
|------|--------|------|------|------|
| CompanyCode | T001 | BUKRS | 法务实体/公司 | 1000 |
| Plant | T001W | WERKS | 工厂/生产基地 | SH01 |
| StorageLocation | T001L | LGORT | 库存地点 | 0001 |
| PurchasingOrganization | T024E | EKORG | 采购组织 | PU01 |
| SalesOrganization | TVKO | VKORG | 销售组织 | SV01 |
| ControllingArea | T000 | KOKRS | 控制范围 | 1000 |

## 层级关系

```text
CompanyCode (公司代码)
├── Plant (工厂)
│   └── StorageLocation (库存地点)
├── PurchasingOrganization (采购组织)
└── SalesOrganization (销售组织)
```

## 快速开始

### 创建公司代码

```rust
use killer_master_data_organizational_units::*;

let company = CompanyCode::new(
    "tenant-001",
    "1000",
    "示例公司",
    "CN",
    "CNY",
).expect("Failed to create company code");

println!("Created company: {}", company.code);
```

### 创建工厂

```rust
let plant = Plant::new(
    "tenant-001",
    "1000",  // company_code
    "SH01",
    "上海工厂",
    "Shanghai",
    "CN",
    Some(chrono::Local::now().date_naive()),
    None,
).expect("Failed to create plant");

// 检查有效性
assert!(plant.is_currently_valid());
```

### 层级验证

```rust
let storage = StorageLocation::new(
    "tenant-001",
    "SH01",  // plant_code
    "0001",
    "主仓库",
).expect("Failed to create storage location");

// 验证 Plant 是否存在
let plant = Plant::new(
    "tenant-001",
    "1000",
    "SH01",
    "上海工厂",
    "Shanghai",
    "CN",
    None,
    None,
).expect("Failed to create plant");
```

### 变更事件

```rust
let event = CompanyCodeChangedEvent {
    header: ChangeEventHeader::new(
        "tenant-001",
        "user-123",
        ChangeEventType::Created,
    ),
    code: "1000".into(),
    changes: vec![
        FieldDelta {
            field_name: "name".to_string(),
            old_value: None,
            new_value: Some(json!("示例公司")),
        },
    ],
    snapshot: Some(company),
};
```

## 事件集成

### 在 mdg-service 发布事件

```rust
use killer_frameworks_cqrs::EventBus;
use killer_master_data_organizational_units::*;

// 创建事件
let event = CompanyCodeChangedEvent {
    header: ChangeEventHeader::new(
        tenant_id,
        actor_id,
        ChangeEventType::Created,
    ),
    code: company_code,
    changes: vec![],
    snapshot: Some(company),
};

// 发布到 Kafka
event_bus.publish("company-code-events", &event).await?;
```

### 在 finance-service 订阅事件

```rust
use killer_frameworks_cqrs::EventHandler;

struct CompanyCodeEventHandler;

#[async_trait::async_trait]
impl EventHandler for CompanyCodeEventHandler {
    type Event = CompanyCodeChangedEvent;

    async fn handle(&self, event: &Self::Event) {
        match event.header.event_type {
            ChangeEventType::Created => {
                // 同步到本地缓存
            }
            ChangeEventType::Updated => {
                // 更新本地数据
            }
            ChangeEventType::Deleted => {
                // 标记为已删除
            }
            _ => {}
        }
    }
}

// 订阅 Kafka topic
event_bus.subscribe::<CompanyCodeEventHandler>("company-code-events").await?;
```

## ValidityRange 使用

```rust
use chrono::{Duration, Utc};

// 创建有效期范围
let validity = ValidityRange::new(
    Utc::now().date(),
    Some(Utc::now().date() + Duration::days(365)),
);

// 检查指定日期是否有效
assert!(validity.is_valid_at(Utc::now().date()));

// 检查当前是否有效
assert!(validity.is_currently_valid());
```

## 扩展字段

```rust
let mut company = CompanyCode::new(
    "tenant-001",
    "1000",
    "示例公司",
    "CN",
    "CNY",
).expect("Failed to create company code");

// 设置扩展字段
company.extensions.set(
    "sap_client".to_string(),
    serde_json::json!(100),
);
company.extensions.set(
    "industry".to_string(),
    serde_json::json!("Manufacturing"),
);

// 获取扩展字段
let industry = company.extensions.get("industry");
```

## 验证

```rust
use validator::Validate;

// 创建无效的公司代码 (代码不是4位)
let result = CompanyCode::new(
    "tenant-001",
    "100",  // 无效：只有3位
    "示例公司",
    "CN",
    "CNY",
);

assert!(result.is_err());
```

## 依赖

```toml
[dependencies]
killer-domain-primitives = { path = "../../common/domain-primitives" }
killer-types = { path = "../../common/types" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
validator = { version = "0.16", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
prost = { version = "0.12", optional = true }
tonic = { version = "0.10", optional = true }
```

## 许可证

MIT
