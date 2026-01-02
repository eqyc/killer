//! 领域服务
//!
//! 包含跨聚合根的业务逻辑

use crate::domain::aggregates::{FiscalPeriod, JournalEntry};
use crate::domain::error::{DomainError, DomainResult};
use crate::domain::value_objects::DebitCredit;
use killer_domain_primitives::{AccountCode, CompanyCode, Money};
use std::collections::HashMap;

// =============================================================================
// 余额计算服务
// =============================================================================

/// 余额计算服务
///
/// 提供账户余额计算功能，支持多维度查询
pub struct BalanceCalculator;

impl BalanceCalculator {
    /// 计算账户余额
    ///
    /// # 参数
    /// - `entries`: 会计凭证列表
    /// - `account_code`: 账户代码
    ///
    /// # 返回
    /// - 账户余额（借方为正，贷方为负）
    pub fn calculate_account_balance(
        entries: &[JournalEntry],
        account_code: &AccountCode,
    ) -> Money {
        let mut total = 0.0;
        let mut currency = None;

        for entry in entries {
            for line_item in entry.line_items() {
                if line_item.account_code() == account_code {
                    if currency.is_none() {
                        currency = Some(line_item.amount().currency().clone());
                    }

                    let signed_amount = line_item.signed_amount();
                    total += signed_amount.amount();
                }
            }
        }

        let currency = currency.unwrap_or_else(|| {
            killer_domain_primitives::CurrencyCode::new("CNY").unwrap()
        });

        Money::new(total, currency).expect("余额计算失败")
    }

    /// 计算多个账户的余额
    ///
    /// # 返回
    /// - HashMap<AccountCode, Money>
    pub fn calculate_balances(
        entries: &[JournalEntry],
        account_codes: &[AccountCode],
    ) -> HashMap<AccountCode, Money> {
        let mut balances = HashMap::new();

        for account_code in account_codes {
            let balance = Self::calculate_account_balance(entries, account_code);
            balances.insert(account_code.clone(), balance);
        }

        balances
    }

    /// 计算期间内的账户余额
    ///
    /// # 参数
    /// - `entries`: 会计凭证列表
    /// - `account_code`: 账户代码
    /// - `fiscal_period`: 会计期间
    ///
    /// # 返回
    /// - 期间内的账户余额
    pub fn calculate_period_balance(
        entries: &[JournalEntry],
        account_code: &AccountCode,
        fiscal_period: &FiscalPeriod,
    ) -> Money {
        let period_entries: Vec<&JournalEntry> = entries
            .iter()
            .filter(|e| {
                e.fiscal_year() == fiscal_period.fiscal_year()
                    && e.company_code() == fiscal_period.company_code()
                    && fiscal_period.validity_range().contains(e.posting_date())
            })
            .collect();

        Self::calculate_account_balance(&period_entries.into_iter().cloned().collect::<Vec<_>>(), account_code)
    }

    /// 验证试算平衡
    ///
    /// # 参数
    /// - `entries`: 会计凭证列表
    ///
    /// # 返回
    /// - 如果借贷平衡返回 Ok(())，否则返回错误
    pub fn verify_trial_balance(entries: &[JournalEntry]) -> DomainResult<()> {
        let mut total_debit = 0.0;
        let mut total_credit = 0.0;

        for entry in entries {
            for line_item in entry.line_items() {
                match line_item.debit_credit() {
                    DebitCredit::Debit => total_debit += line_item.amount().amount(),
                    DebitCredit::Credit => total_credit += line_item.amount().amount(),
                }
            }
        }

        let difference = (total_debit - total_credit).abs();
        if difference > 0.01 {
            return Err(DomainError::UnbalancedEntry {
                debit: format!("{:.2}", total_debit),
                credit: format!("{:.2}", total_credit),
                difference: format!("{:.2}", difference),
            });
        }

        Ok(())
    }
}

// =============================================================================
// 期间关闭服务
// =============================================================================

/// 期间关闭服务
///
/// 协调期间关闭流程，确保所有前置条件满足
pub struct PeriodCloseService;

impl PeriodCloseService {
    /// 检查期间是否可以关闭
    ///
    /// # 业务规则
    /// - 期间内所有凭证必须已过账
    /// - 试算必须平衡
    ///
    /// # 参数
    /// - `fiscal_period`: 会计期间
    /// - `entries`: 期间内的所有凭证
    ///
    /// # 返回
    /// - 如果可以关闭返回 Ok(())，否则返回错误
    pub fn can_close_period(
        fiscal_period: &FiscalPeriod,
        entries: &[JournalEntry],
    ) -> DomainResult<()> {
        // 检查是否有未过账的凭证
        let unposted_count = entries
            .iter()
            .filter(|e| !e.status().can_reverse()) // 未过账的凭证
            .count();

        if unposted_count > 0 {
            return Err(DomainError::UnpostedEntriesExist {
                count: unposted_count,
            });
        }

        // 验证试算平衡
        BalanceCalculator::verify_trial_balance(entries)?;

        Ok(())
    }

    /// 执行期间关闭
    ///
    /// # 参数
    /// - `fiscal_period`: 会计期间
    /// - `entries`: 期间内的所有凭证
    ///
    /// # 返回
    /// - 成功返回事件列表
    pub fn close_period(
        fiscal_period: &mut FiscalPeriod,
        entries: &[JournalEntry],
    ) -> DomainResult<Vec<crate::domain::events::DomainEvent>> {
        // 检查前置条件
        Self::can_close_period(fiscal_period, entries)?;

        // 关闭期间
        fiscal_period.close()
    }

    /// 检查期间是否可以重新开放
    ///
    /// # 业务规则
    /// - 后续期间必须未关闭
    ///
    /// # 参数
    /// - `fiscal_period`: 要重新开放的期间
    /// - `subsequent_periods`: 后续期间列表
    ///
    /// # 返回
    /// - 如果可以重新开放返回 Ok(())，否则返回错误
    pub fn can_reopen_period(
        fiscal_period: &FiscalPeriod,
        subsequent_periods: &[FiscalPeriod],
    ) -> DomainResult<()> {
        // 检查后续期间是否有已关闭的
        let closed_subsequent = subsequent_periods
            .iter()
            .filter(|p| {
                p.fiscal_year() == fiscal_period.fiscal_year()
                    && p.period() > fiscal_period.period()
                    && p.status() == crate::domain::value_objects::PeriodStatus::Closed
            })
            .count();

        if closed_subsequent > 0 {
            return Err(DomainError::BusinessRuleViolation {
                rule: format!(
                    "不能重新开放期间 {}，因为后续期间已关闭",
                    fiscal_period.period()
                ),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::JournalEntryLineItem;
    use crate::domain::value_objects::DebitCredit;
    use chrono::NaiveDate;
    use killer_domain_primitives::{CurrencyCode, DocumentNumber};

    fn create_test_entry() -> JournalEntry {
        let currency = CurrencyCode::new("CNY").unwrap();
        let line_items = vec![
            JournalEntryLineItem::new(
                1,
                AccountCode::new("1001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Debit,
            )
            .unwrap(),
            JournalEntryLineItem::new(
                2,
                AccountCode::new("2001").unwrap(),
                Money::new(1000.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
        ];

        JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        )
        .unwrap()
    }

    #[test]
    fn test_calculate_account_balance() {
        let entry = create_test_entry();
        let entries = vec![entry];

        let balance = BalanceCalculator::calculate_account_balance(
            &entries,
            &AccountCode::new("1001").unwrap(),
        );

        assert_eq!(balance.amount(), 1000.0);
    }

    #[test]
    fn test_verify_trial_balance() {
        let entry = create_test_entry();
        let entries = vec![entry];

        let result = BalanceCalculator::verify_trial_balance(&entries);
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_close_period_with_unposted_entries() {
        let period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let entry = create_test_entry(); // 草稿状态
        let entries = vec![entry];

        let result = PeriodCloseService::can_close_period(&period, &entries);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_close_period_with_posted_entries() {
        let period = FiscalPeriod::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            1,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        )
        .unwrap();

        let mut entry = create_test_entry();
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        let entries = vec![entry];

        let result = PeriodCloseService::can_close_period(&period, &entries);
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_balances() {
        let entry = create_test_entry();
        let entries = vec![entry];

        let account_codes = vec![
            AccountCode::new("1001").unwrap(),
            AccountCode::new("2001").unwrap(),
        ];

        let balances = BalanceCalculator::calculate_balances(&entries, &account_codes);

        assert_eq!(balances.len(), 2);
        assert_eq!(balances[&AccountCode::new("1001").unwrap()].amount(), 1000.0);
        assert_eq!(balances[&AccountCode::new("2001").unwrap()].amount(), -1000.0);
    }
}
