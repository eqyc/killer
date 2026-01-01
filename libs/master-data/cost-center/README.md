# Cost Center

成本中心主数据定义，包括成本中心（CostCenter）和利润中心（ProfitCenter）。用于管理会计中的成本归集和利润分析。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `CostCenter` | 成本中心 |
| `CostCenterId` | 成本中心 ID |
| `CostCenterCode` | 成本中心代码 |
| `CostCenterCategory` | 成本中心类别 |
| `CostCenterHierarchy` | 成本中心层次结构 |
| `ProfitCenter` | 利润中心 |
| `ProfitCenterId` | 利润中心 ID |
| `ProfitCenterCode` | 利润中心代码 |
| `ControllingArea` | 控制范围 |
| `ValidityPeriod` | 有效期 |

## 使用示例

```text
// 创建成本中心
let cost_center = CostCenter::builder()
    .code(CostCenterCode::new("CC-1001")?)
    .name("生产部门")
    .category(CostCenterCategory::Production)
    .controlling_area(controlling_area)
    .valid_from(start_date)
    .valid_to(end_date)
    .build()?;

// 创建利润中心
let profit_center = ProfitCenter::builder()
    .code(ProfitCenterCode::new("PC-2001")?)
    .name("华东区销售")
    .controlling_area(controlling_area)
    .build()?;

// 层次结构
let hierarchy = CostCenterHierarchy::new(root_cost_center);
hierarchy.add_child(parent_id, child_cost_center)?;
```

## 层次结构

```text
控制范围 (Controlling Area)
└── 成本中心组 (Cost Center Group)
    ├── 成本中心 1
    ├── 成本中心 2
    └── 成本中心组（子）
        └── 成本中心 3
```
