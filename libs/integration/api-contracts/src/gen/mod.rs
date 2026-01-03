//! 自动生成的 gRPC/Protobuf 代码模块
//!
//! 由 buf generate 命令从 proto/ 目录生成

#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::large_enum_variant)]
#![allow(non_camel_case_types)]

// =============================================================================
// Common 公共类型
// =============================================================================
pub mod common {
    pub mod v1 {
        include!("killer.common.v1.rs");
    }
}

// =============================================================================
// Finance 财务域
// =============================================================================
pub mod finance {
    pub mod financial {
        pub mod v1 {
            include!("killer.finance.financial.v1.rs");
        }
    }
    pub mod controlling {
        pub mod v1 {
            include!("killer.finance.controlling.v1.rs");
        }
    }
    pub mod treasury {
        pub mod v1 {
            include!("killer.finance.treasury.v1.rs");
        }
    }
}

// =============================================================================
// Logistics 物流域
// =============================================================================
pub mod logistics {
    pub mod materials {
        pub mod v1 {
            include!("killer.logistics.materials.v1.rs");
        }
    }
    pub mod warehouse {
        pub mod v1 {
            include!("killer.logistics.warehouse.v1.rs");
        }
    }
    pub mod shipping {
        pub mod v1 {
            include!("killer.logistics.shipping.v1.rs");
        }
    }
}

// =============================================================================
// Operations 运营域
// =============================================================================
pub mod operations {
    pub mod production {
        pub mod v1 {
            include!("killer.operations.production.v1.rs");
        }
    }
    pub mod maintenance {
        pub mod v1 {
            include!("killer.operations.maintenance.v1.rs");
        }
    }
    pub mod quality {
        pub mod v1 {
            include!("killer.operations.quality.v1.rs");
        }
    }
}

// =============================================================================
// Procurement 采购域
// =============================================================================
pub mod procurement {
    pub mod purchasing {
        pub mod v1 {
            include!("killer.procurement.purchasing.v1.rs");
        }
    }
    pub mod scm {
        pub mod v1 {
            include!("killer.procurement.scm.v1.rs");
        }
    }
}

// =============================================================================
// Commercial 商业域
// =============================================================================
pub mod commercial {
    pub mod sales {
        pub mod v1 {
            include!("killer.commercial.sales.v1.rs");
        }
    }
    pub mod crm {
        pub mod v1 {
            include!("killer.commercial.crm.v1.rs");
        }
    }
    pub mod field_service {
        pub mod v1 {
            include!("killer.commercial.field_service.v1.rs");
        }
    }
}

// =============================================================================
// Human Capital 人力资本域
// =============================================================================
pub mod human_capital {
    pub mod hr {
        pub mod v1 {
            include!("killer.human_capital.hr.v1.rs");
        }
    }
    pub mod payroll {
        pub mod v1 {
            include!("killer.human_capital.payroll.v1.rs");
        }
    }
}

// =============================================================================
// Project RD 项目研发域
// =============================================================================
pub mod project_rd {
    pub mod project {
        pub mod v1 {
            include!("killer.project_rd.project.v1.rs");
        }
    }
    pub mod plm {
        pub mod v1 {
            include!("killer.project_rd.plm.v1.rs");
        }
    }
}

// =============================================================================
// 便捷重导出
// =============================================================================

/// 重导出常用类型
pub mod prelude {
    // Common types
    pub use super::common::v1::{
        Address, AuditInfo, ContactInfo, DateRange, Money, PageInfo, PageRequest, Period, Quantity,
    };

    // Financial
    pub use super::finance::financial::v1::{JournalEntry, JournalEntryItem};

    // Sales
    pub use super::commercial::sales::v1::{SalesOrder, SalesOrderItem};

    // Purchasing
    pub use super::procurement::purchasing::v1::{PurchaseOrder, PurchaseOrderItem};
}
