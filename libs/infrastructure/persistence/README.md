# Persistence

持久化基础设施库，提供 Repository 模式抽象、工作单元（Unit of Work）模式和数据库连接池管理。支持事务管理和乐观锁。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Repository<T, ID>` | 仓储 trait |
| `ReadRepository<T, ID>` | 只读仓储 trait |
| `WriteRepository<T, ID>` | 写仓储 trait |
| `UnitOfWork` | 工作单元 trait |
| `Transaction` | 事务抽象 |
| `DbPool` | 数据库连接池 |
| `Pagination` | 分页参数 |
| `Page<T>` | 分页结果 |
| `Specification<T>` | 查询规约 |

## 使用示例

```text
// 仓储操作
let order = order_repository.find_by_id(order_id).await?;
let orders = order_repository.find_all(pagination).await?;
order_repository.save(&order).await?;

// 工作单元（事务）
uow.begin().await?;
order_repository.save(&order).await?;
inventory_repository.update(&inventory).await?;
uow.commit().await?;

// 规约查询
let spec = OrderSpec::new()
    .customer_id(customer_id)
    .status(OrderStatus::Pending)
    .created_after(yesterday);
let orders = order_repository.find_by_spec(spec).await?;

// 分页
let page = order_repository.find_all(Pagination::new(0, 20)).await?;
// page.items, page.total, page.has_next
```

## 特性

- **泛型仓储** - 类型安全的 CRUD 操作
- **事务管理** - 自动回滚、嵌套事务
- **乐观锁** - 基于版本号的并发控制
- **规约模式** - 可组合的查询条件
