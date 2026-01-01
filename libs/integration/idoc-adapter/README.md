# IDoc Adapter

IDoc 适配器库，用于与 SAP 系统集成。提供 IDoc（Intermediate Document）解析器和 OData 客户端，支持主数据同步和业务单据交换。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `IDocParser` | IDoc 文档解析器 |
| `IDocDocument` | IDoc 文档结构 |
| `IDocSegment` | IDoc 段数据 |
| `IDocType` | IDoc 类型定义 |
| `ODataClient` | OData V4 客户端 |
| `ODataQuery` | OData 查询构建器 |
| `ODataEntity` | OData 实体 trait |
| `SapConnector` | SAP 连接器 |
| `RfcClient` | RFC 调用客户端 |

## 使用示例

```text
// 解析 IDoc 文档
let idoc = IDocParser::parse(xml_content)?;
let header = idoc.control_record();
let segments = idoc.data_records();

// OData 客户端
let client = ODataClient::new("https://sap-server/sap/opu/odata/sap/API_BUSINESS_PARTNER");
let partners = client
    .get::<BusinessPartner>()
    .filter("CustomerClassification eq 'A'")
    .top(100)
    .execute()
    .await?;

// 创建 OData 实体
client.post(&new_business_partner).await?;

// RFC 调用
let result = rfc_client.call("BAPI_MATERIAL_GET_DETAIL", params).await?;
```

## 支持的 IDoc 类型

| IDoc 类型 | 说明 |
|-----------|------|
| `DEBMAS` | 客户主数据 |
| `CREMAS` | 供应商主数据 |
| `MATMAS` | 物料主数据 |
| `ORDERS` | 采购订单 |
| `DESADV` | 发货通知 |
| `INVOIC` | 发票 |
