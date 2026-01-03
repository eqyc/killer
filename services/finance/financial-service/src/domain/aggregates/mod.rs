//! 聚合根模块
//!
//! 财务领域的聚合根定义

pub mod gl_account;
pub mod journal_entry;
pub mod customer;
pub mod vendor;
pub mod fixed_asset;
pub mod bank_account;

#[cfg(test)]
mod tests;

// Re-exports for easier access
pub use gl_account::GLAccount;
pub use journal_entry::{JournalEntry, JournalEntryItem, DocumentStatus};
pub use customer::{Customer, CustomerStatus};
pub use vendor::{Vendor, VendorStatus};
pub use fixed_asset::{FixedAsset, AssetStatus};
pub use bank_account::BankAccount;
