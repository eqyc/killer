# Protocol Buffers 定义

存放 KILLER ERP 系统的 gRPC 服务接口定义。使用 [Buf](https://buf.build/) 进行 Proto 管理和代码生成。

## 目录结构

```
proto/
├── common/v1/              # 公共类型
│   ├── types.proto         # 通用类型 (Money, Address, etc.)
│   ├── errors.proto        # 错误定义
│   ├── pagination.proto    # 分页定义
│   ├── api_gateway.proto   # API网关
│   ├── identity.proto      # 身份认证
│   └── master_data.proto   # 主数据管理
├── finance/                # 财务域
│   ├── financial/v1/       # 核心财务会计
│   ├── controlling/v1/     # 管理会计
│   └── treasury/v1/        # 资金管理
├── procurement/            # 采购域
│   ├── scm/v1/             # 供应链管理
│   └── purchasing/v1/      # 采购执行
├── operations/             # 运营域
│   ├── production/v1/      # 生产计划
│   ├── quality/v1/         # 质量管理
│   └── maintenance/v1/     # 设备维护
├── logistics/              # 物流域
│   ├── materials/v1/       # 物料管理
│   ├── warehouse/v1/       # 仓储管理
│   └── shipping/v1/        # 运输管理
├── commercial/             # 商业域
│   ├── sales/v1/           # 销售订单
│   ├── crm/v1/             # 客户关系
│   └── field-service/v1/   # 现场服务
├── project-rd/             # 项目研发域
│   ├── project/v1/         # 项目管理
│   └── plm/v1/             # 产品生命周期
├── human-capital/          # 人力资本域
│   ├── hr/v1/              # 核心人事
│   └── payroll/v1/         # 薪资核算
├── buf.yaml                # Buf 工作区配置
└── buf.gen.yaml            # 代码生成配置
```

## 核心聚合根与数据表映射

| 业务域 | 聚合根 | 核心数据表 | 行项目 |
|--------|--------|-----------|--------|
| 财务会计 | JournalEntry | 通用日记账 | JournalEntryItem |
| 管理会计 | CostDocument | 成本核算凭证 | CostDocumentItem |
| 资金管理 | CashFlow | 现金流明细 | CashFlowItem |
| 物料管理 | MaterialDocument | 物料凭证 | MaterialDocumentItem |
| 仓储管理 | WarehouseTask | 仓库任务 | WarehouseTaskItem |
| 发运管理 | Delivery | 交货单 | DeliveryItem |
| 生产管理 | ProductionOrder | 生产订单 | ProductionOrderItem |
| 设备维护 | MaintenanceOrder | 维护订单 | MaintenanceOrderItem |
| 质量管理 | InspectionLot | 质检批 | InspectionLotItem |
| 采购管理 | PurchaseOrder | 采购订单 | PurchaseOrderItem |
| 供应链 | SupplyPlan | 供应计划 | SupplyPlanItem |
| 销售管理 | SalesOrder | 销售订单 | SalesOrderItem |
| CRM | CRMOrder | 客户订单 | CRMOrderItem |
| 现场服务 | ServiceOrder | 服务订单 | ServiceOrderItem |
| 人事管理 | Employee | 员工主数据 | EmployeeInfoRecord |
| 薪酬管理 | PayrollRecord | 薪资记录 | PayrollItem |
| 项目管理 | Project | 项目定义 | WBSElement |
| PLM | BillOfMaterial | 物料清单 | BOMItem |

## 版本管理策略

### API 版本号

采用 `v1`, `v2` 等主版本号进行管理：

| 版本 | 说明 |
|------|------|
| `v1` | 当前稳定版本 |
| `v2` | 下一个主版本（存在不兼容变更时） |
| `v1beta1` | Beta 版本，可能变更 |
| `v1alpha1` | Alpha 版本，不稳定 |

### 版本升级原则

1. **向后兼容变更**（不需要升级版本）：
   - 添加新字段（使用新的 field number）
   - 添加新方法
   - 添加新枚举值
   - 废弃字段（添加 `deprecated = true`）

2. **不兼容变更**（需要升级主版本）：
   - 删除或重命名字段
   - 修改字段类型
   - 修改方法签名
   - 删除方法

3. **版本共存**：
   - 新版本发布后，旧版本至少维护 6 个月
   - 服务端同时支持 v1 和 v2
   - 客户端逐步迁移到新版本

## 字段命名规范

本项目Proto消息字段采用**大写命名规范**，与企业ERP系统核心数据表字段保持一致：

| 类型 | 格式 | 示例 |
|------|------|------|
| 主键字段 | `UPPER_CASE` | `BELNR`, `VBELN`, `AUFNR` |
| 组织字段 | `UPPER_CASE` | `BUKRS`, `WERKS`, `VKORG` |
| 日期字段 | `UPPER_CASE` | `BUDAT`, `BLDAT`, `ERDAT` |
| 金额字段 | `UPPER_CASE` | `WRBTR`, `DMBTR`, `NETWR` |
| 数量字段 | `UPPER_CASE` | `MENGE`, `GAMNG`, `LFIMG` |
| 控制字段 | `UPPER_CASE` | `LOEKZ`, `SHKZG`, `WAERS` |

## 代码生成

### 环境准备

```bash
# 安装 Buf CLI
brew install bufbuild/buf/buf

# 安装 Rust 插件
cargo install protoc-gen-prost
cargo install protoc-gen-prost-serde
cargo install protoc-gen-tonic
```

### 生成 Rust 代码

```bash
# 进入 proto 目录
cd proto

# Lint 检查
buf lint

# 格式化
buf format -w

# 生成代码
buf generate

# 生成代码到指定目录
buf generate --output ../libs/integration/api-contracts/src/gen
```

### 生成其他语言代码

```bash
# 生成 Go 代码（需要配置 buf.gen.yaml）
buf generate --template buf.gen.go.yaml

# 生成 TypeScript 代码
buf generate --template buf.gen.ts.yaml
```

### 检测破坏性变更

```bash
# 与 Git 主分支比较
buf breaking --against '.git#branch=main'

# 与远程模块比较
buf breaking --against 'buf.build/killer-erp/killer'
```

## 添加新的服务定义

### 步骤 1: 创建目录结构

```bash
mkdir -p proto/<domain>/<service>/v1
```

### 步骤 2: 创建 service.proto

```protobuf
syntax = "proto3";

package killer.<domain>.<service>.v1;

option java_multiple_files = true;
option java_package = "com.killer.<domain>.<service>.v1";

import "common/v1/types.proto";
import "common/v1/pagination.proto";

// 服务定义
service <Service>Service {
  // 创建
  rpc Create<Entity>(Create<Entity>Request) returns (Create<Entity>Response);
  // 获取
  rpc Get<Entity>(Get<Entity>Request) returns (Get<Entity>Response);
  // 列表
  rpc List<Entity>s(List<Entity>sRequest) returns (List<Entity>sResponse);
}

// 聚合根消息
message <Entity> {
  // 主键字段
  string PRIMARY_KEY = 1;
  // 组织层级字段
  string ORG_FIELD = 2;
  // ... 其他核心字段
  // 审计信息
  killer.common.v1.AuditInfo audit_info = N;
  // 行项目
  repeated <Entity>Item items = M;
}

// 行项目消息
message <Entity>Item {
  // 行号
  int32 LINE_NUMBER = 1;
  // ... 行级字段
}
```

### 步骤 3: 验证和生成

```bash
# Lint 检查
buf lint

# 生成代码
buf generate
```

## 最佳实践

### 字段编号

- 1-15：常用字段（1 字节编码）
- 16-2047：一般字段（2 字节编码）
- 19000-19999：保留字段（protobuf 内部使用）

### 通用类型使用

| 类型 | Proto类型 | 说明 |
|------|-----------|------|
| 金额 | `killer.common.v1.Money` | 包含金额和货币代码 |
| 数量 | `killer.common.v1.Quantity` | 包含数值和单位 |
| 时间 | `google.protobuf.Timestamp` | UTC时间戳 |
| 地址 | `killer.common.v1.Address` | 结构化地址 |
| 审计 | `killer.common.v1.AuditInfo` | 创建/修改信息 |

### 枚举默认值

枚举第一个值必须是 `UNSPECIFIED = 0`：

```protobuf
enum OrderStatus {
  ORDER_STATUS_UNSPECIFIED = 0;
  ORDER_STATUS_DRAFT = 1;
  ORDER_STATUS_CONFIRMED = 2;
}
```

## 相关链接

- [Buf 官方文档](https://buf.build/docs/)
- [Protocol Buffers 语言指南](https://developers.google.com/protocol-buffers/docs/proto3)
- [gRPC 官方文档](https://grpc.io/docs/)
- [Google API 设计指南](https://cloud.google.com/apis/design)
