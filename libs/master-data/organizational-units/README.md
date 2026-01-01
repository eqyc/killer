# Organizational Units

组织单元主数据定义，包括公司代码、工厂、库存地点、销售组织、采购组织等。这些组织单元构成 ERP 系统的组织架构基础。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `CompanyCode` | 公司代码（法人实体） |
| `Plant` | 工厂 |
| `StorageLocation` | 库存地点 |
| `SalesOrganization` | 销售组织 |
| `DistributionChannel` | 分销渠道 |
| `Division` | 产品组 |
| `PurchasingOrganization` | 采购组织 |
| `PurchasingGroup` | 采购组 |
| `BusinessArea` | 业务范围 |
| `Client` | 集团（最高级别） |

## 使用示例

```text
// 定义组织架构
let client = Client::new("100", "KILLER 集团");

let company = CompanyCode::builder()
    .code("1000")
    .name("KILLER 中国有限公司")
    .currency(CurrencyCode::CNY)
    .country(CountryCode::CN)
    .build()?;

let plant = Plant::builder()
    .code("1001")
    .name("上海工厂")
    .company_code(&company)
    .build()?;

let storage = StorageLocation::builder()
    .code("0001")
    .name("原材料仓库")
    .plant(&plant)
    .build()?;

// 组织架构查询
let plants = organization.plants_by_company(&company);
let storages = organization.storage_locations_by_plant(&plant);
```

## 组织层次

```text
Client (集团)
└── CompanyCode (公司代码)
    ├── Plant (工厂)
    │   └── StorageLocation (库存地点)
    ├── SalesOrganization (销售组织)
    │   └── DistributionChannel (分销渠道)
    │       └── Division (产品组)
    └── PurchasingOrganization (采购组织)
        └── PurchasingGroup (采购组)
```
