//! 实体模块

pub mod journal_entry_item;
pub mod document;

#[cfg(test)]
mod tests;

// Re-exports for easier access
pub use journal_entry_item::{JournalEntryItem, JournalEntryItemError, DebitCreditIndicator};
pub use document::{Document, DocumentStatus};
