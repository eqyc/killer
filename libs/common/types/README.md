# Types

通用类型定义库，提供跨模块共享的基础类型。包括统一的错误处理类型、Result 别名、以及类型安全的实体标识符。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Result<T>` | 统一的 Result 类型别名 |
| `Error` | 应用级错误枚举 |
| `ErrorCode` | 错误码定义 |
| `Id<T>` | 泛型实体标识符 |
| `EntityId` | 实体 ID trait |
| `AggregateId` | 聚合根 ID trait |
| `Timestamp` | 时间戳类型 |
| `Version` | 乐观锁版本号 |

## 使用示例

```text
// 类型安全的 ID
let user_id: Id<User> = Id::new();
let order_id: Id<Order> = Id::new();
// user_id 和 order_id 不能混用

// 统一错误处理
fn find_user(id: Id<User>) -> Result<User> {
    // 返回 Ok(user) 或 Err(Error::NotFound(...))
}
```

## 错误码规范

| 范围 | 类别 |
|------|------|
| 1xxx | 认证/授权错误 |
| 2xxx | 业务规则违反 |
| 3xxx | 数据验证错误 |
| 4xxx | 资源不存在 |
| 5xxx | 系统内部错误 |
