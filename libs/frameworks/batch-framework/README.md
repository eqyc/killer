# Batch Framework

批处理框架，提供任务调度、批量数据处理、断点续传等功能。适用于 ERP 系统中的月结、对账、报表生成等批处理场景。

## 主要导出类型

| 类型 | 说明 |
|------|------|
| `Job` | 批处理任务 trait |
| `JobScheduler` | 任务调度器 |
| `BatchProcessor<I, O>` | 批量处理器（读取-处理-写入） |
| `ItemReader<I>` | 数据读取器 trait |
| `ItemProcessor<I, O>` | 数据处理器 trait |
| `ItemWriter<O>` | 数据写入器 trait |
| `JobExecution` | 任务执行上下文 |
| `StepExecution` | 步骤执行上下文 |
| `Checkpoint` | 断点信息 |

## 使用示例

```text
// 定义批处理任务
struct MonthEndClosingJob {
    fiscal_period: FiscalPeriod,
}

impl Job for MonthEndClosingJob {
    fn steps(&self) -> Vec<Step> {
        vec![
            Step::new("validate_postings", validate_step),
            Step::chunk("process_documents", 1000)
                .reader(document_reader)
                .processor(document_processor)
                .writer(document_writer),
            Step::new("generate_reports", report_step),
        ]
    }
}

// 调度任务
scheduler.schedule(job, CronExpr::parse("0 0 1 * *")?);

// 手动执行（支持断点续传）
let execution = scheduler.run(job, checkpoint).await?;
```

## 特性

- **Chunk Processing** - 分块处理，控制内存使用
- **Checkpoint/Restart** - 断点续传，失败后继续
- **Parallel Steps** - 并行步骤执行
- **Skip Policy** - 跳过策略（错误容忍）
- **Retry Policy** - 重试策略
