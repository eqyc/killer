# Business Partner

业务伙伴主数据定义，包括客户（Customer）、供应商（Supplier）和通用业务伙伴（BusinessPartner）。这些是 ERP 系统中的核心主数据，被多个业务域共享。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `BusinessPartner` | 业务伙伴基类 |
| `BusinessPartnerId` | 业务伙伴 ID |
| `Customer` | 客户（销售视图） |
| `CustomerId` | 客户 ID |
| `Supplier` | 供应商（采购视图） |
| `SupplierId` | 供应商 ID |
| `PartnerRole` | 伙伴角色（客户/供应商/两者） |
| `Address` | 地址信息 |
| `ContactPerson` | 联系人 |
| `BankAccount` | 银行账户 |
| `TaxInfo` | 税务信息 |

## 使用示例

```text
// 创建业务伙伴
let partner = BusinessPartner::builder()
    .name("ACME Corporation")
    .role(PartnerRole::CustomerAndSupplier)
    .address(address)
    .build()?;

// 获取客户视图
let customer: &Customer = partner.as_customer()?;

// 获取供应商视图
let supplier: &Supplier = partner.as_supplier()?;

// 添加联系人
partner.add_contact(ContactPerson { ... })?;
```

## 数据模型

```text
BusinessPartner (1) ----< (N) Address
                  (1) ----< (N) ContactPerson
                  (1) ----< (N) BankAccount
                  (1) ----> (1) Customer [可选]
                  (1) ----> (1) Supplier [可选]
```
