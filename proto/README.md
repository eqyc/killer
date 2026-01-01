# Protocol Buffers 定义

存放 KILLER ERP 系统的 gRPC 服务接口定义。使用 [Buf](https://buf.build/) 进行 Proto 管理和代码生成。

## 目录结构

```
proto/
├── common/v1/              # 公共类型
│   ├── types.proto         # 通用类型 (Money, Address, etc.)
│   ├── errors.proto        # 错误定义
│   └── pagination.proto    # 分页定义
├── finance/                # 财务域
│   ├── financial/v1/       # 核心财务 (FI)
│   ├── controlling/v1/     # 管理会计 (CO)
│   └── treasury/v1/        # 资金管理 (TR)
├── procurement/            # 采购域
│   ├── scm/v1/             # 供应链管理
│   └── purchasing/v1/      # 采购执行 (MM)
├── operations/             # 运营域
│   ├── production/v1/      # 生产计划 (PP)
│   ├── quality/v1/         # 质量管理 (QM)
│   └── maintenance/v1/     # 设备维护 (PM)
├── logistics/              # 物流域
│   ├── materials/v1/       # 物料管理 (MM-IM)
│   ├── warehouse/v1/       # 仓储管理 (EWM)
│   └── shipping/v1/        # 运输管理 (TM)
├── commercial/             # 商业域
│   ├── sales/v1/           # 销售订单 (SD)
│   ├── crm/v1/             # 客户关系
│   └── field-service/v1/   # 售后服务
├── project-rd/             # 项目研发域
│   ├── project/v1/         # 项目管理 (PS)
│   └── plm/v1/             # 产品生命周期
├── human-capital/          # 人力资本域
│   ├── hr/v1/              # 核心 HR (PA)
│   └── payroll/v1/         # 薪资核算 (PY)
├── buf.yaml                # Buf 工作区配置
└── buf.gen.yaml            # 代码生成配置
```

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
  // 方法定义
  rpc Create<Entity>(Create<Entity>Request) returns (Create<Entity>Response);
  rpc Get<Entity>(Get<Entity>Request) returns (Get<Entity>Response);
  rpc List<Entity>s(List<Entity>sRequest) returns (List<Entity>sResponse);
  rpc Update<Entity>(Update<Entity>Request) returns (Update<Entity>Response);
  rpc Delete<Entity>(Delete<Entity>Request) returns (Delete<Entity>Response);
}
```

### 步骤 3: 创建 messages.proto

```protobuf
syntax = "proto3";

package killer.<domain>.<service>.v1;

// 实体定义
message <Entity> {
  string id = 1;
  // ... 其他字段
}

// 请求/响应消息
message Create<Entity>Request { ... }
message Create<Entity>Response { ... }
```

### 步骤 4: 验证和生成

```bash
# Lint 检查
buf lint

# 生成代码
buf generate
```

## 命名约定

### Package 命名

```
killer.<domain>.<service>.v<version>
```

示例：
- `killer.finance.financial.v1`
- `killer.commercial.sales.v1`

### 消息命名

| 类型 | 格式 | 示例 |
|------|------|------|
| 实体 | `PascalCase` | `JournalEntry`, `SalesOrder` |
| 请求 | `<Action><Entity>Request` | `CreateOrderRequest` |
| 响应 | `<Action><Entity>Response` | `CreateOrderResponse` |
| 枚举 | `UPPER_SNAKE_CASE` | `ORDER_STATUS_CREATED` |
| 字段 | `snake_case` | `customer_id`, `order_date` |

### RPC 方法命名

| 操作 | 格式 | 示例 |
|------|------|------|
| 创建 | `Create<Entity>` | `CreateOrder` |
| 获取单个 | `Get<Entity>` | `GetOrder` |
| 列表查询 | `List<Entity>s` | `ListOrders` |
| 更新 | `Update<Entity>` | `UpdateOrder` |
| 删除 | `Delete<Entity>` | `DeleteOrder` |
| 批量创建 | `BatchCreate<Entity>s` | `BatchCreateOrders` |
| 自定义动作 | `<Action><Entity>` | `ConfirmOrder`, `CancelOrder` |

## 最佳实践

### 字段编号

- 1-15：常用字段（1 字节编码）
- 16-2047：一般字段（2 字节编码）
- 19000-19999：保留字段（protobuf 内部使用）

### 必填 vs 可选

Proto3 中所有字段都是可选的，使用以下方式表达必填：

```protobuf
message CreateOrderRequest {
  // 必填字段 - 在文档中说明
  string customer_id = 1;  // Required

  // 可选字段 - 使用 optional 关键字或 wrapper
  optional string reference = 2;
}
```

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
