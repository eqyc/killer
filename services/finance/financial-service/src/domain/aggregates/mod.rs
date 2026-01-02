//! 聚合根模块
//!
//! 包含财务领域的所有聚合根

mod fiscal_period;
mod journal_entry;

pub use fiscal_period::FiscalPeriod;
pub use journal_entry::JournalEntry;
