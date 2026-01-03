//! 应用层模块
//!
//! 包含 CQRS 命令处理器、查询处理器、应用服务

pub mod commands;
pub mod queries;
pub mod dto;

// Command handlers re-exports
pub use commands::{
    CreateGLAccountHandler,
    CreateJournalEntryHandler,
    PostJournalEntryHandler,
    ReverseJournalEntryHandler,
    CreateCustomerHandler,
    UpdateCustomerHandler,
    BlockCustomerHandler,
    CreateVendorHandler,
    BlockVendorHandler,
    CreateFixedAssetHandler,
    CapitalizeFixedAssetHandler,
    DepreciateFixedAssetHandler,
    TransferFixedAssetHandler,
    RetireFixedAssetHandler,
    BlockFixedAssetHandler,
    CreateBankAccountHandler,
    DepositBankAccountHandler,
    WithdrawBankAccountHandler,
    UpdateBankAccountBalanceHandler,
};

// Query handlers re-exports
pub use queries::{
    GetGLAccountHandler,
    ListGLAccountsHandler,
    GetJournalEntryHandler,
    ListJournalEntriesHandler,
    GetCustomerHandler,
    ListCustomersHandler,
    GetVendorHandler,
    ListVendorsHandler,
    GetFixedAssetHandler,
    ListFixedAssetsHandler,
    ListFixedAssetsByStatusHandler,
    GetBankAccountHandler,
    ListBankAccountsHandler,
    ListBankAccountsByCountryHandler,
    GetBankAccountBySwiftHandler,
};
