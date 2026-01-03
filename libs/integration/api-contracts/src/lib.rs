//! API 契约定义库
//!
//! gRPC 服务接口、REST DTO、OpenAPI 规范。
//!
//! # 模块结构
//!
//! - `gen` - 自动生成的 protobuf/gRPC 代码
//!   - `common::v1` - 通用类型 (Money, Quantity, PageRequest 等)
//!   - `finance::*` - 财务域 (JournalEntry, CostDocument, CashFlow)
//!   - `logistics::*` - 物流域 (MaterialDocument, Delivery, WarehouseTask)
//!   - `operations::*` - 运营域 (ProductionOrder, MaintenanceOrder, InspectionLot)
//!   - `procurement::*` - 采购域 (PurchaseOrder, SupplyPlan)
//!   - `commercial::*` - 商业域 (SalesOrder, CRMOrder, ServiceOrder)
//!   - `human_capital::*` - 人力资本域 (Employee, PayrollRecord)
//!   - `project_rd::*` - 项目研发域 (Project, BillOfMaterial)
//!
//! # 使用示例
//!
//! ```ignore
//! use killer_api_contracts::gen::prelude::*;
//!
//! let order = SalesOrder {
//!     vbeln: "0000012345".to_string(),
//!     ..Default::default()
//! };
//! ```

pub mod gen;

/// 重导出 prelude 模块
pub use gen::prelude;
