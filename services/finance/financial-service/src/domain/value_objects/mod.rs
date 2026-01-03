//! 值对象模块

pub mod account_code;
pub mod fiscal_period;
pub mod posting_date;

#[cfg(test)]
mod tests;

// Re-exports for easier access
// 使用 killer_domain_primitives 中的 DocumentNumber
pub use account_code::{AccountCode, AccountCodeError};
pub use fiscal_period::{FiscalPeriod, FiscalPeriodError};
pub use posting_date::{PostingDate, PostingDateError};
