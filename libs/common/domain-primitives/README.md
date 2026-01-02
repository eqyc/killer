# KILLER ERP 领域原语库

> Domain Primitives for KILLER ERP System

本库提供 ERP 系统中的基础值对象定义，参考 SAP S/4 HANA 的数据模型设计。

## 核心类型

### 金额与数量

| 类型 | 描述 | SAP 参考 | 精度 |
|------|------|----------|------|
| `Money` | 金额类型 | ACDOCA.HSL/TSL | 4 位小数 |
| `Quantity` | 数量类型 | EKPO.MENGE | 3 位小数 |
| `Percentage` | 百分比类型 | - | 2 位小数 |

### 计量单位

| 类型 | 描述 | SAP 参考 |
|------|------|----------|
| `UnitOfMeasure` | 计量单位 | T006.MSEHI |
| `Dimension` | 单位维度 | T006.DIMID |

### 币种

| 类型 | 描述 | SAP 参考 |
|------|------|----------|
| `CurrencyCode` | ISO 4217 币种代码 | TCURC.WAERS |

### 组织单元

| 类型 | 描述 | SAP 参考 | 长度 |
|------|------|----------|------|
| `CompanyCode` | 公司代码 | T001.BUKRS | 4 位 |
| `Plant` | 工厂代码 | T001W.WERKS | 4 位 |
| `CostCenter` | 成本中心 | CSKS.KOSTL | 10 位 |

### 业务编码

| 类型 | 描述 | SAP 参考 | 长度 |
|------|------|----------|------|
| `AccountCode` | 会计科目代码 | SKA1.SAKNR | 10 位 |
| `MaterialNumber` | 物料编号 | MARA.MATNR | 18/40 位 |
| `DocumentNumber` | 凭证编号 | BKPF.BELNR | 10 位 |
| `FiscalPeriod` | 会计期间 | T009B.MONAT | 1-16 |

## 使用示例

### 金额计算

```rust
use killer_domain_primitives::{Money, CurrencyCode, Percentage};
use rust_decimal_macros::dec;

// 创建金额
let price = Money::new(dec!(100.50), CurrencyCode::cny()).unwrap();

// 计算税额（13% 增值税）
let tax_rate = Percentage::vat_standard_cn();
let tax = price.multiply(tax_rate.to_decimal()).unwrap();

// 计算含税总价
let total = price.add(&tax).unwrap();

// 金额分配
let parts = total.allocate(&[dec!(1), dec!(1), dec!(1)]).unwrap();
```

### 数量与单位换算

```rust
use killer_domain_primitives::{Quantity, UnitOfMeasure};
use rust_decimal_macros::dec;

// 创建数量
let qty_kg = Quantity::new(dec!(10), UnitOfMeasure::kilogram()).unwrap();

// 单位换算
let qty_g = qty_kg.convert_to(&UnitOfMeasure::gram()).unwrap();
assert_eq!(qty_g.value(), dec!(10000));

// 不同单位相加（自动换算）
let qty_500g = Quantity::new(dec!(500), UnitOfMeasure::gram()).unwrap();
let total = qty_kg.add(&qty_500g).unwrap();
assert_eq!(total.value(), dec!(10.5)); // 10.5 KG
```

### 组织单元

```rust
use killer_domain_primitives::{CompanyCode, Plant, CostCenter};

// 创建公司代码
let company = CompanyCode::new("1000")
    .unwrap()
    .with_name("KILLER 集团")
    .with_currency_code("CNY");

// 创建工厂
let plant = Plant::new("1001")
    .unwrap()
    .with_name("上海工厂")
    .with_company_code("1000");

// 创建成本中心
let cost_center = CostCenter::new("1000100001", "1000")
    .unwrap()
    .with_description("生产部门");
```

### 业务编码

```rust
use killer_domain_primitives::{
    AccountCode, AccountType,
    MaterialNumber,
    DocumentNumber, DocumentType,
    FiscalPeriod,
};

// 会计科目
let account = AccountCode::new("1001000000", "YCOA")
    .unwrap()
    .with_account_type(AccountType::Asset);

// 物料编号（支持前导零）
let material = MaterialNumber::with_leading_zeros("1001", 18).unwrap();
assert_eq!(material.number(), "000000000000001001");

// 凭证编号
let doc = DocumentNumber::new("1000000001", 2024, "1000")
    .unwrap()
    .with_document_type(DocumentType::VendorInvoice);

// 会计期间
let period = FiscalPeriod::new(2024, 3).unwrap();
let next_period = period.next().unwrap();
```

## 设计原则

### 不可变性

所有值对象都是不可变的，任何修改操作都会返回新的实例：

```rust
let money = Money::new(dec!(100), CurrencyCode::cny()).unwrap();
let doubled = money.multiply(dec!(2)).unwrap(); // 返回新实例
// money 仍然是 100
```

### 类型安全

使用 newtype 模式确保类型安全，防止混淆不同业务含义的值：

```rust
// 编译错误：类型不匹配
// let result = account_code + material_number;

// 正确：同类型操作
let qty1 = Quantity::kg(dec!(10)).unwrap();
let qty2 = Quantity::kg(dec!(5)).unwrap();
let sum = qty1.add(&qty2).unwrap();
```

### 精度保证

使用 `rust_decimal` 确保金融计算的精度：

- Money: 4 位小数（SAP CURR 类型）
- Quantity: 3 位小数（SAP QUAN 类型）
- Percentage: 2 位小数

### 验证规则

所有值对象在创建时进行验证：

```rust
// 失败：币种代码格式错误
let result = CurrencyCode::new("INVALID");
assert!(result.is_err());

// 失败：会计期间超出范围
let result = FiscalPeriod::new(2024, 17);
assert!(result.is_err());
```

## SAP 数据类型对照

| SAP 类型 | Rust 类型 | 说明 |
|----------|-----------|------|
| CURR | `Money` | 货币金额，4 位小数 |
| QUAN | `Quantity` | 数量，3 位小数 |
| UNIT | `UnitOfMeasure` | 计量单位 |
| WAERS | `CurrencyCode` | 币种代码 |
| BUKRS | `CompanyCode` | 公司代码 |
| WERKS | `Plant` | 工厂代码 |
| KOSTL | `CostCenter` | 成本中心 |
| SAKNR | `AccountCode` | 科目编号 |
| MATNR | `MaterialNumber` | 物料编号 |
| BELNR | `DocumentNumber` | 凭证编号 |
| GJAHR/MONAT | `FiscalPeriod` | 会计年度/期间 |

## 依赖

```toml
[dependencies]
killer-domain-primitives = { path = "../domain-primitives" }
rust_decimal = "1.36"
rust_decimal_macros = "1.36"
```

## 许可证

MIT License
