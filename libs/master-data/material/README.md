# Material

物料主数据定义，包括物料基本数据、工厂级数据和库存地点级数据。遵循 SAP 风格的物料主数据视图模型，支持多组织架构。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Material` | 物料主数据（客户端级） |
| `MaterialId` | 物料 ID |
| `MaterialNumber` | 物料编号 |
| `MaterialType` | 物料类型（原材料、半成品、成品等） |
| `MaterialPlantData` | 物料工厂数据 |
| `MaterialStorageData` | 物料库存地点数据 |
| `MaterialSalesData` | 物料销售数据 |
| `MaterialPurchasingData` | 物料采购数据 |
| `UnitOfMeasure` | 计量单位 |
| `MaterialGroup` | 物料组 |

## 使用示例

```text
// 创建物料主数据
let material = Material::builder()
    .number(MaterialNumber::new("MAT-001234")?)
    .description("高强度钢板")
    .material_type(MaterialType::RawMaterial)
    .base_unit(UnitOfMeasure::KG)
    .build()?;

// 添加工厂数据
material.add_plant_data(PlantCode::new("1000")?, MaterialPlantData {
    mrp_type: MrpType::PD,
    procurement_type: ProcurementType::External,
    ...
})?;

// 添加库存地点数据
material.add_storage_data(plant, storage_location, MaterialStorageData { ... })?;
```

## 数据视图

| 视图 | 说明 |
|------|------|
| 基本数据 | 物料描述、类型、计量单位 |
| 工厂数据 | MRP、采购、生产相关配置 |
| 库存数据 | 库存地点级配置 |
| 销售数据 | 销售组织级配置 |
| 采购数据 | 采购组织级配置 |
