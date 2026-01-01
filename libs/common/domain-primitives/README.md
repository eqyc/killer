# Domain Primitives

领域原语类型库，提供 ERP 系统中的基础值对象定义。这些类型封装了业务规则验证，确保在编译时和运行时的类型安全。所有原语类型都是不可变的，支持序列化和相等性比较。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Money` | 货币金额，支持多币种和精确计算 |
| `Quantity` | 数量，带单位的数值类型 |
| `UnitOfMeasure` | 计量单位（EA, KG, M, L 等） |
| `AccountCode` | 会计科目代码 |
| `MaterialNumber` | 物料编号 |
| `DocumentNumber` | 单据编号 |
| `CurrencyCode` | ISO 4217 货币代码 |
| `CountryCode` | ISO 3166 国家代码 |
| `Percentage` | 百分比（0-100%） |
| `TaxRate` | 税率 |

## 使用示例

```text
// 创建货币金额
let amount = Money::new(1000.50, CurrencyCode::CNY)?;

// 数量计算
let qty = Quantity::new(100.0, UnitOfMeasure::KG);
let doubled = qty.multiply(2.0);

// 物料编号验证
let material = MaterialNumber::try_from("MAT-001234")?;
```

## 设计原则

- **不可变性**：所有类型创建后不可修改
- **验证前置**：构造时验证，使用时无需再验证
- **类型安全**：编译时防止类型混用
- **序列化友好**：支持 JSON/数据库序列化
