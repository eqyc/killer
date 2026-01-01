# KILLER ERP 项目检查清单

本文档用于验证 KILLER ERP 项目的初始化完整性。在项目搭建完成后，逐项检查以确保所有必要组件已就位。

---

## 使用说明

1. 复制此文档或在 GitHub 中直接编辑
2. 完成一项后将 `[ ]` 改为 `[x]`
3. 所有项目完成后，项目基础架构搭建完毕
4. 建议在首次提交前完成所有检查

**进度统计**：`0 / 75` 项完成

---

## 1. 目录结构检查

### 1.1 根目录结构

- [ ] `libs/` 目录已创建
- [ ] `services/` 目录已创建
- [ ] `proto/` 目录已创建
- [ ] `infrastructure/` 目录已创建
- [ ] `tools/` 目录已创建
- [ ] `scripts/` 目录已创建
- [ ] `docs/` 目录已创建
- [ ] `config/` 目录已创建

### 1.2 libs/ 子目录

- [ ] `libs/common/` 已创建（domain-primitives, types, utils）
- [ ] `libs/frameworks/` 已创建（cqrs, event-sourcing, saga-orchestration, batch-framework）
- [ ] `libs/infrastructure/` 已创建（messaging, observability, auth, persistence）
- [ ] `libs/master-data/` 已创建（business-partner, material, cost-center, organizational-units）
- [ ] `libs/integration/` 已创建（api-contracts, idoc-adapter）

### 1.3 services/ 子目录

- [ ] `services/infrastructure/` 已创建（api-gateway, identity-iam, mdg-service）
- [ ] `services/finance/` 已创建（financial-service, controlling-service, treasury-service）
- [ ] `services/procurement-ops/` 已创建（scm-service, purchasing-service）
- [ ] `services/operations/` 已创建（production-service, quality-service, maintenance-service）
- [ ] `services/logistics/` 已创建（materials-service, warehouse-service, shipping-service）
- [ ] `services/commercial/` 已创建（sales-service, crm-service, field-service）
- [ ] `services/project-rd/` 已创建（project-service, plm-service）
- [ ] `services/human-capital/` 已创建（hr-service, payroll-service）

---

## 2. 配置文件检查

### 2.1 Cargo 配置

- [ ] `Cargo.toml`（workspace）已配置
- [ ] workspace.members 包含所有 crate
- [ ] workspace.dependencies 定义共享依赖
- [ ] workspace.package 定义共享元数据

### 2.2 环境配置

- [ ] `.env.example` 已创建（50+ 环境变量）
- [ ] `config/dev.toml` 已创建
- [ ] `config/staging.toml` 已创建
- [ ] `config/production.toml` 已创建
- [ ] `config/README.md` 已创建

### 2.3 代码质量配置

- [ ] `rustfmt.toml` 已配置
- [ ] `clippy.toml` 已配置
- [ ] `deny.toml` 已配置（cargo-deny）
- [ ] `rust-toolchain.toml` 已配置

---

## 3. 文档完整性检查

### 3.1 核心文档

- [ ] `README.md` 已创建（项目概述、快速启动）
- [ ] `docs/ARCHITECTURE.md` 已创建（架构设计）
- [ ] `docs/DEVELOPMENT.md` 已创建（开发指南）
- [ ] `docs/DOMAIN-MODELING.md` 已创建（领域建模指南）
- [ ] `docs/API-STANDARDS.md` 已创建（API 设计规范）
- [ ] `CONTRIBUTING.md` 已创建（贡献指南）
- [ ] `CHANGELOG.md` 已创建（变更日志）
- [ ] `LICENSE` 已创建

### 3.2 ADR 文档

- [ ] `docs/adr/README.md` 已创建（ADR 索引和说明）
- [ ] `docs/adr/TEMPLATE.md` 已创建（ADR 模板）
- [ ] `docs/adr/0000-use-architecture-decision-records.md` 已创建
- [ ] `docs/adr/0001-choose-rust-as-primary-language.md` 已创建
- [ ] `docs/adr/0002-adopt-grpc-for-service-communication.md` 已创建
- [ ] `docs/adr/0003-implement-cqrs-pattern.md` 已创建
- [ ] `docs/adr/0004-use-postgresql-and-clickhouse.md` 已创建
- [ ] `docs/adr/0005-centralize-master-data-in-mdg-service.md` 已创建

---

## 4. 基础设施配置检查

### 4.1 Docker 配置

- [ ] `infrastructure/docker/docker-compose.yml` 已创建（核心服务）
- [ ] `infrastructure/docker/docker-compose.dev.yml` 已创建（开发扩展）
- [ ] `infrastructure/docker/Dockerfile.dev` 已创建
- [ ] `infrastructure/docker/Dockerfile.release` 已创建
- [ ] `infrastructure/docker/.dockerignore` 已创建

### 4.2 Kubernetes 配置

- [ ] `infrastructure/k8s/base/` 目录已创建
- [ ] `infrastructure/k8s/overlays/dev/` 已创建
- [ ] `infrastructure/k8s/overlays/staging/` 已创建
- [ ] `infrastructure/k8s/overlays/production/` 已创建

### 4.3 监控配置

- [ ] `infrastructure/monitoring/prometheus/` 已创建
- [ ] `infrastructure/monitoring/grafana/` 已创建
- [ ] `infrastructure/monitoring/loki/` 已创建
- [ ] `infrastructure/monitoring/tempo/` 已创建
- [ ] `infrastructure/monitoring/otel-collector/` 已创建

---

## 5. CI/CD 配置检查

- [ ] `.github/workflows/ci.yml` 已创建（持续集成）
- [ ] `.github/workflows/security-scan.yml` 已创建（安全扫描）
- [ ] `.github/workflows/release.yml` 已创建（发布流程）
- [ ] `.github/dependabot.yml` 已创建（依赖更新）
- [ ] CI 工作流包含多平台矩阵构建
- [ ] CI 工作流包含测试覆盖率上报

---

## 6. 共享库检查

对于 `libs/` 下的每个库：

- [ ] 每个库有 `Cargo.toml`
- [ ] 每个库有 `README.md`（说明用途和导出类型）
- [ ] 每个库有 `src/lib.rs`
- [ ] 每个库有 `src/` 目录结构
- [ ] 库之间的依赖关系正确配置

---

## 7. 微服务检查

以 `financial-service` 为例，验证服务结构：

### 7.1 目录结构

- [ ] `services/finance/financial-service/` 目录已创建
- [ ] `src/api/` 目录已创建（API 层）
- [ ] `src/application/` 目录已创建（应用层）
- [ ] `src/domain/` 目录已创建（领域层）
- [ ] `src/infrastructure/` 目录已创建（基础设施层）

### 7.2 配置文件

- [ ] `Cargo.toml` 已配置
- [ ] `README.md` 已创建
- [ ] `config/` 目录已创建（服务特定配置）

### 7.3 测试与迁移

- [ ] `tests/` 目录已创建
- [ ] `migrations/` 目录已创建
- [ ] `Dockerfile` 已创建

---

## 8. 开发工具检查

### 8.1 构建工具

- [ ] `Makefile` 已创建
- [ ] `make help` 显示所有可用任务
- [ ] `make dev` 可启动开发环境
- [ ] `make build` 可编译项目
- [ ] `make test` 可运行测试

### 8.2 编辑器配置

- [ ] `.editorconfig` 已创建
- [ ] `.vscode/settings.json` 已创建
- [ ] `.vscode/extensions.json` 已创建
- [ ] `.vscode/launch.json` 已创建

### 8.3 Git 配置

- [ ] `.gitignore` 已创建（包含 Rust、IDE、环境文件）
- [ ] `.gitattributes` 已创建

---

## 9. Proto 定义检查

### 9.1 Buf 配置

- [ ] `proto/buf.yaml` 已配置
- [ ] `proto/buf.gen.yaml` 已配置
- [ ] `proto/buf.lock` 已生成

### 9.2 Proto 文件结构

- [ ] `proto/killer/common/v1/` 已创建（共享类型）
- [ ] `proto/killer/finance/v1/` 已创建
- [ ] `proto/killer/sales/v1/` 已创建
- [ ] `proto/killer/purchasing/v1/` 已创建
- [ ] `proto/README.md` 已创建

---

## 10. 验证步骤

### 10.1 代码检查

- [ ] 运行 `cargo check --workspace` 无错误
- [ ] 运行 `cargo fmt --all -- --check` 无格式问题
- [ ] 运行 `cargo clippy --workspace -- -D warnings` 无警告
- [ ] 运行 `cargo test --workspace` 测试通过

### 10.2 Proto 检查

- [ ] 运行 `buf lint` 无错误
- [ ] 运行 `buf format --diff` 无格式问题
- [ ] 运行 `buf generate` 代码生成成功

### 10.3 基础设施检查

- [ ] 运行 `make dev` 启动基础设施成功
- [ ] PostgreSQL 可连接（localhost:5432）
- [ ] Redis 可连接（localhost:6379）
- [ ] Kafka 可连接（localhost:9092）
- [ ] 运行 `make dev-down` 停止成功

### 10.4 Git 检查

- [ ] Git 仓库已初始化（`git init`）
- [ ] 首次提交已完成
- [ ] 远程仓库已配置（如适用）

---

## 检查完成统计

| 分类 | 完成数 | 总数 |
|------|--------|------|
| 目录结构 | 0 | 16 |
| 配置文件 | 0 | 12 |
| 文档完整性 | 0 | 16 |
| 基础设施 | 0 | 14 |
| CI/CD | 0 | 6 |
| 共享库 | 0 | 5 |
| 微服务 | 0 | 11 |
| 开发工具 | 0 | 11 |
| Proto 定义 | 0 | 8 |
| 验证步骤 | 0 | 13 |
| **总计** | **0** | **112** |

---

## 下一步

完成所有检查项后，建议按以下顺序继续：

### 阶段 1：核心框架实现（1-2 周）

1. **实现 domain-primitives 库**
   - 定义基础值对象（Money, Quantity, EntityId）
   - 实现领域事件基础 trait
   - 编写单元测试

2. **实现 CQRS 框架**
   - Command/Query trait 定义
   - CommandBus/QueryBus 实现
   - 中间件管道

3. **实现 Event Sourcing 框架**
   - Aggregate trait 定义
   - EventStore 接口
   - 事件重放机制

### 阶段 2：基础设施库实现（1-2 周）

4. **实现 persistence 库**
   - PostgreSQL 连接池
   - Repository 基础实现
   - 事务管理

5. **实现 messaging 库**
   - Kafka 生产者/消费者
   - 事件发布/订阅
   - 死信队列处理

6. **实现 observability 库**
   - 结构化日志
   - 分布式追踪
   - 指标收集

### 阶段 3：首个业务服务（2-3 周）

7. **实现 financial-service**
   - JournalEntry 聚合根
   - 凭证创建/过账/冲销
   - gRPC API 实现
   - 集成测试

8. **实现 API Gateway**
   - 路由配置
   - 认证中间件
   - 限流实现

### 阶段 4：集成与部署（1 周）

9. **端到端测试**
   - 服务间通信测试
   - 事件流测试
   - 性能基准测试

10. **部署流水线**
    - Docker 镜像构建
    - Kubernetes 部署
    - 监控告警配置

---

## 常见问题

### Q: 检查项太多，可以跳过一些吗？

建议不要跳过。每个检查项都是项目长期维护的基础。如果时间紧迫，可以：
- 先完成必需项（标记为 ✓ 的配置节）
- 文档可以后续补充，但目录结构和配置文件应优先完成

### Q: 某些检查项不适用于我的项目怎么办？

在检查项后添加 `N/A` 标记，并说明原因：
```markdown
- [x] `infrastructure/k8s/` 配置已创建 (N/A - 暂不使用 K8s)
```

### Q: 如何自动化这个检查清单？

可以编写脚本自动检查文件存在性：
```bash
# scripts/check-project.sh
#!/bin/bash
check_file() {
    if [ -f "$1" ] || [ -d "$1" ]; then
        echo "✓ $1"
    else
        echo "✗ $1"
    fi
}

check_file "Cargo.toml"
check_file "libs/"
check_file "services/"
# ... 更多检查
```

---

*最后更新: 2024-01*
