# 🤝 贡献指南 (Contributing Guide)

感谢你为 KILLER ERP 系统做出贡献！本指南旨在确保所有贡献者都能在一致的标准下高效协作。

---

## 🏗️ 核心原则：极简与自律

1.  **代码即文档**: 我们不推崇冗长的文档。除了 `docs/` 下的核心指南，尽量通过清晰的代码命名、类型系统和 `Makefile` 来表达逻辑。
2.  **约定优于配置**: 遵循 Rust 社区的标准实践（Rust 2024 Edition, Clippy）。
3.  **自动化优先**: 所有手动操作都应尽量集成到 `Makefile` 中。

---

## 🌿 分支策略与命名

-   **`main`**: 生产分支，保护模式。
-   **功能开发**: `feature/<service>-<desc>` (例: `feature/crm-add-customer`)。
-   **缺陷修复**: `fix/<service>-<issue-id>`。
-   **重构/文档**: `refactor/<desc>` 或 `docs/<desc>`。

---

## 🛠️ 开发工作流

1.  **同步主干**: `git pull origin main --rebase`
2.  **创建分支**: 按照上述规范命名。
3.  **开发与测试**:
    *   **接口变更**: 修改 `.proto` 文件 -> 运行 `make proto`。
    *   **业务实现**: 在 `services/` 目录下编写 Rust 代码。
    *   **数据库变更**: 在 `migrations/` 下添加 SQL -> 运行 `make db-migrate`。
4.  **本地预检查** (合并前必做):
    ```bash
    make ci-local
    ```
    *这会运行格式化、Clippy 检查、所有测试和 Release 构建。*
5.  **提交 PR**: 使用提供的 Pull Request 模板，详细描述变更内容。

---

## 💻 代码标准

-   **格式化**: 必须通过 `make fmt`。
-   **Lint**: 必须通过 `make lint` (Clippy 无警告)。
-   **错误处理**: 使用 `thiserror` 处理库错误，`anyhow` 处理应用层错误（如果适用）。
-   **日志**: 使用 `tracing` 库。

---

## 📝 文档贡献

我们不希望增加文档数量。如果你觉得需要更新文档：
-   **操作流程**: 请更新 `docs/DEVELOPMENT.md`。
-   **架构变动**: 请更新 `docs/ARCHITECTURE.md`。
-   **重大决策**: 如果是不可逆的架构选择，请在 `docs/adr/` (如果存在) 下记录。

**禁止**: 在各服务子目录下创建新的 README.md 文件。

---

## 🔒 安全与秘密

-   **禁止**提交任何 `.env` 文件、密钥、API Keys 或证书。
-   **发现漏洞**: 请发送邮件至 `security@eqy.cc`，不要直接在 Issue 中公开。

---

## 🚀 提交信息规范 (Conventional Commits)

建议使用以下前缀：
-   `feat:` 新功能
-   `fix:` 修复 Bug
-   `docs:` 文档更新
-   `style:` 格式调整 (不影响代码逻辑)
-   `refactor:` 重构代码
-   `test:` 增加测试

### 参考案例 (基于 Rust-BOM 任务清单)

| 类型 | 提交信息示例 | 对应 Rust-BOM 任务 |
| :--- | :--- | :--- |
| **feat** | `feat(fi): implement general ledger posting logic` | 波次 1 (FI): 凭证录入与过账 |
| **feat** | `feat(mm): add three-way matching for invoices` | 波次 4 (MM): 发票三单匹配 |
| **fix** | `fix(mrp): resolve calculation error in safety stock` | 波次 6 (PP): MRP 运算与例外 |
| **docs** | `docs(api): update grpc contract for sales order` | 波次 5 (SD): API/gRPC 契约设计 |
| **refactor** | `refactor(domain): use newtype pattern for currency amount` | 专项 19: 强类型业务建模 |
| **test** | `test(e2e): add p2p flow integration test` | 波次 4 (MM): P2P 全链路测试 |

*提示: 括号内的范围 (scope) 建议对应微服务名称，如 `fi`, `crm`, `gateway` 等。*

---
*如有疑问，请查阅 [README.md](./README.md) 或在 PR 中提问。*