# Saga Orchestration

Saga 编排框架，用于管理跨服务的分布式事务。采用编排器模式，支持定义执行步骤和补偿动作，确保最终一致性。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Saga<S>` | Saga 定义 trait |
| `SagaStep` | Saga 步骤 trait |
| `SagaOrchestrator` | Saga 编排器 |
| `SagaState` | Saga 执行状态 |
| `CompensatingAction` | 补偿动作 trait |
| `SagaLog` | Saga 执行日志 |
| `SagaRepository` | Saga 状态持久化 |
| `StepResult` | 步骤执行结果 |

## 使用示例

```text
// 定义 Saga
struct CreateOrderSaga {
    order_id: OrderId,
    customer_id: CustomerId,
    items: Vec<OrderItem>,
}

impl Saga for CreateOrderSaga {
    fn steps(&self) -> Vec<SagaStep> {
        vec![
            step("reserve_inventory", reserve_inventory, release_inventory),
            step("create_payment", create_payment, cancel_payment),
            step("confirm_order", confirm_order, cancel_order),
        ]
    }
}

// 执行 Saga
let result = orchestrator.execute(create_order_saga).await;
match result {
    SagaResult::Completed => { ... }
    SagaResult::Compensated { failed_step, error } => { ... }
}
```

## 执行状态

| 状态 | 说明 |
|------|------|
| `Pending` | 等待执行 |
| `Running` | 正在执行 |
| `Completed` | 执行成功 |
| `Compensating` | 正在补偿 |
| `Compensated` | 补偿完成 |
| `Failed` | 执行失败（无法补偿） |
