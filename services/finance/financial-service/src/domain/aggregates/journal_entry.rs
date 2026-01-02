//! 会计凭证聚合根
//!
//! 对应 SAP ACDOCA 表，是财务领域的核心聚合根

use crate::domain::entities::JournalEntryLineItem;
use crate::domain::error::{DomainError, DomainResult};
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::{DebitCredit, JournalEntryId, JournalEntryStatus};
use chrono::NaiveDate;
use killer_domain_primitives::{CompanyCode, CurrencyCode, DocumentNumber};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// 会计凭证聚合根
// =============================================================================

/// 会计凭证聚合根
///
/// 对应 SAP ACDOCA 表，管理会计凭证的完整生命周期
///
/// # 不变式
/// - 借贷必须平衡（精确到货币精度）
/// - 至少包含 2 行项目
/// - 过账日期必须在开放的会计期间内
/// - 已过账的凭证不可修改，仅可冲销
/// - 所有行项目的 tenant_id 必须一致
/// - 所有行项目的币种必须与凭证头一致
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    /// 租户ID
    tenant_id: String,

    /// 公司代码
    company_code: CompanyCode,

    /// 会计年度
    fiscal_year: i32,

    /// 凭证号
    document_number: DocumentNumber,

    /// 过账日期
    posting_date: NaiveDate,

    /// 凭证日期
    document_date: NaiveDate,

    /// 币种
    currency: CurrencyCode,

    /// 凭证状态
    status: JournalEntryStatus,

    /// 凭证抬头文本
    header_text: Option<String>,

    /// 参考凭证号
    reference_document: Option<String>,

    /// 行项目列表
    line_items: Vec<JournalEntryLineItem>,

    /// 版本号（乐观锁）
    version: u64,

    /// 冲销凭证号（如果已被冲销）
    reversed_by: Option<DocumentNumber>,

    /// 原始凭证号（如果是冲销凭证）
    reversal_of: Option<DocumentNumber>,
}

impl JournalEntry {
    /// 创建新的会计凭证（草稿状态）
    ///
    /// # 参数
    /// - `tenant_id`: 租户ID
    /// - `company_code`: 公司代码
    /// - `fiscal_year`: 会计年度
    /// - `document_number`: 凭证号
    /// - `posting_date`: 过账日期
    /// - `document_date`: 凭证日期
    /// - `currency`: 币种
    /// - `line_items`: 行项目列表
    ///
    /// # 不变式验证
    /// - 至少 2 行项目
    /// - 借贷平衡
    /// - 所有行项目币种一致
    pub fn create(
        tenant_id: impl Into<String>,
        company_code: CompanyCode,
        fiscal_year: i32,
        document_number: DocumentNumber,
        posting_date: NaiveDate,
        document_date: NaiveDate,
        currency: CurrencyCode,
        line_items: Vec<JournalEntryLineItem>,
    ) -> DomainResult<Self> {
        let tenant_id = tenant_id.into();

        // 验证至少 2 行
        if line_items.len() < 2 {
            return Err(DomainError::InsufficientLineItems {
                required: 2,
                actual: line_items.len(),
            });
        }

        // 验证所有行项目币种一致
        for item in &line_items {
            if item.amount().currency() != &currency {
                return Err(DomainError::ValidationError {
                    message: format!(
                        "行项目币种 {} 与凭证头币种 {} 不一致",
                        item.amount().currency().as_str(),
                        currency.as_str()
                    ),
                });
            }
        }

        let entry = Self {
            tenant_id,
            company_code,
            fiscal_year,
            document_number,
            posting_date,
            document_date,
            currency,
            status: JournalEntryStatus::Draft,
            header_text: None,
            reference_document: None,
            line_items,
            version: 1,
            reversed_by: None,
            reversal_of: None,
        };

        // 验证借贷平衡
        entry.validate_balance()?;

        Ok(entry)
    }

    /// 过账凭证
    ///
    /// # 参数
    /// - `period_start`: 会计期间开始日期
    /// - `period_end`: 会计期间结束日期
    ///
    /// # 业务规则
    /// - 只有草稿状态可以过账
    /// - 过账日期必须在会计期间内
    /// - 必须借贷平衡
    ///
    /// # 返回
    /// - 成功返回 JournalEntryPosted 事件
    pub fn post(
        &mut self,
        period_start: NaiveDate,
        period_end: NaiveDate,
    ) -> DomainResult<Vec<DomainEvent>> {
        // 检查状态
        if !self.status.can_post() {
            return Err(DomainError::AlreadyPosted {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        // 检查过账日期在期间内
        if self.posting_date < period_start || self.posting_date > period_end {
            return Err(DomainError::InvalidPostingDate {
                date: self.posting_date.to_string(),
                period_start: period_start.to_string(),
                period_end: period_end.to_string(),
            });
        }

        // 再次验证借贷平衡
        self.validate_balance()?;

        // 更新状态
        self.status = JournalEntryStatus::Posted;
        self.version += 1;

        // 生成事件
        let event = DomainEvent::JournalEntryPosted {
            tenant_id: self.tenant_id.clone(),
            company_code: self.company_code.clone(),
            fiscal_year: self.fiscal_year,
            document_number: self.document_number.clone(),
            posting_date: self.posting_date,
            currency: self.currency.clone(),
            total_debit: self.calculate_total_debit(),
            total_credit: self.calculate_total_credit(),
        };

        Ok(vec![event])
    }

    /// 冲销凭证
    ///
    /// # 参数
    /// - `reversal_document_number`: 冲销凭证号
    /// - `reversal_date`: 冲销日期
    ///
    /// # 业务规则
    /// - 只有已过账的凭证可以冲销
    /// - 不能冲销已被冲销的凭证
    ///
    /// # 返回
    /// - 成功返回冲销凭证和 JournalEntryReversed 事件
    pub fn reverse(
        &mut self,
        reversal_document_number: DocumentNumber,
        reversal_date: NaiveDate,
    ) -> DomainResult<(Self, Vec<DomainEvent>)> {
        // 检查状态
        if !self.status.can_reverse() {
            return Err(DomainError::AlreadyReversed {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        // 创建冲销凭证（借贷方向相反）
        let reversal_line_items: Vec<JournalEntryLineItem> = self
            .line_items
            .iter()
            .map(|item| {
                JournalEntryLineItem::new(
                    item.line_number(),
                    item.account_code().clone(),
                    item.amount().clone(),
                    item.debit_credit().opposite(), // 借贷方向相反
                )
                .expect("冲销行项目创建失败")
            })
            .collect();

        let mut reversal_entry = Self::create(
            self.tenant_id.clone(),
            self.company_code.clone(),
            self.fiscal_year,
            reversal_document_number.clone(),
            reversal_date,
            reversal_date,
            self.currency.clone(),
            reversal_line_items,
        )?;

        // 标记为冲销凭证
        reversal_entry.reversal_of = Some(self.document_number.clone());
        reversal_entry.header_text = Some(format!(
            "冲销凭证 {}",
            self.document_number.as_str()
        ));

        // 更新原凭证状态
        self.status = JournalEntryStatus::Reversed;
        self.reversed_by = Some(reversal_document_number.clone());
        self.version += 1;

        // 生成事件
        let event = DomainEvent::JournalEntryReversed {
            tenant_id: self.tenant_id.clone(),
            company_code: self.company_code.clone(),
            fiscal_year: self.fiscal_year,
            original_document_number: self.document_number.clone(),
            reversal_document_number: reversal_document_number.clone(),
            reversal_date,
        };

        Ok((reversal_entry, vec![event]))
    }

    /// 添加行项目（仅草稿状态）
    pub fn add_line_item(&mut self, line_item: JournalEntryLineItem) -> DomainResult<()> {
        if !self.status.is_modifiable() {
            return Err(DomainError::AlreadyPosted {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        // 验证币种一致
        if line_item.amount().currency() != &self.currency {
            return Err(DomainError::ValidationError {
                message: format!(
                    "行项目币种 {} 与凭证头币种 {} 不一致",
                    line_item.amount().currency().as_str(),
                    self.currency.as_str()
                ),
            });
        }

        self.line_items.push(line_item);
        self.version += 1;

        Ok(())
    }

    /// 删除行项目（仅草稿状态）
    pub fn remove_line_item(&mut self, line_number: u32) -> DomainResult<()> {
        if !self.status.is_modifiable() {
            return Err(DomainError::AlreadyPosted {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        // 删除前检查至少保留 2 行
        if self.line_items.len() <= 2 {
            return Err(DomainError::InsufficientLineItems {
                required: 2,
                actual: self.line_items.len() - 1,
            });
        }

        self.line_items.retain(|item| item.line_number() != line_number);
        self.version += 1;

        Ok(())
    }

    /// 设置抬头文本
    pub fn set_header_text(&mut self, text: impl Into<String>) -> DomainResult<()> {
        if !self.status.is_modifiable() {
            return Err(DomainError::AlreadyPosted {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        self.header_text = Some(text.into());
        self.version += 1;

        Ok(())
    }

    /// 设置参考凭证号
    pub fn set_reference_document(&mut self, reference: impl Into<String>) -> DomainResult<()> {
        if !self.status.is_modifiable() {
            return Err(DomainError::AlreadyPosted {
                document_number: self.document_number.as_str().to_string(),
            });
        }

        self.reference_document = Some(reference.into());
        self.version += 1;

        Ok(())
    }

    /// 验证借贷平衡
    ///
    /// # 业务规则
    /// - 借方总额 == 贷方总额（精确到货币精度，通常 0.01）
    fn validate_balance(&self) -> DomainResult<()> {
        let total_debit = self.calculate_total_debit();
        let total_credit = self.calculate_total_credit();

        // 使用货币精度比较（0.01）
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

    /// 计算借方总额
    fn calculate_total_debit(&self) -> f64 {
        self.line_items
            .iter()
            .filter(|item| item.debit_credit().is_debit())
            .map(|item| item.amount().amount())
            .sum()
    }

    /// 计算贷方总额
    fn calculate_total_credit(&self) -> f64 {
        self.line_items
            .iter()
            .filter(|item| item.debit_credit().is_credit())
            .map(|item| item.amount().amount())
            .sum()
    }

    // Getters

    /// 获取聚合根 ID
    pub fn id(&self) -> JournalEntryId {
        JournalEntryId::new(
            self.tenant_id.clone(),
            self.company_code.as_str(),
            self.fiscal_year,
            self.document_number.as_str(),
        )
    }

    /// 获取租户ID
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    /// 获取公司代码
    pub fn company_code(&self) -> &CompanyCode {
        &self.company_code
    }

    /// 获取会计年度
    pub fn fiscal_year(&self) -> i32 {
        self.fiscal_year
    }

    /// 获取凭证号
    pub fn document_number(&self) -> &DocumentNumber {
        &self.document_number
    }

    /// 获取过账日期
    pub fn posting_date(&self) -> NaiveDate {
        self.posting_date
    }

    /// 获取凭证日期
    pub fn document_date(&self) -> NaiveDate {
        self.document_date
    }

    /// 获取币种
    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }

    /// 获取状态
    pub fn status(&self) -> JournalEntryStatus {
        self.status
    }

    /// 获取抬头文本
    pub fn header_text(&self) -> Option<&str> {
        self.header_text.as_deref()
    }

    /// 获取参考凭证号
    pub fn reference_document(&self) -> Option<&str> {
        self.reference_document.as_deref()
    }

    /// 获取行项目列表
    pub fn line_items(&self) -> &[JournalEntryLineItem] {
        &self.line_items
    }

    /// 获取版本号
    pub fn version(&self) -> u64 {
        self.version
    }

    /// 获取冲销凭证号
    pub fn reversed_by(&self) -> Option<&DocumentNumber> {
        self.reversed_by.as_ref()
    }

    /// 获取原始凭证号
    pub fn reversal_of(&self) -> Option<&DocumentNumber> {
        self.reversal_of.as_ref()
    }

    /// 是否为冲销凭证
    pub fn is_reversal(&self) -> bool {
        self.reversal_of.is_some()
    }

    /// 是否已被冲销
    pub fn is_reversed(&self) -> bool {
        self.status == JournalEntryStatus::Reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use killer_domain_primitives::{AccountCode, Money};

    fn create_test_line_items() -> Vec<JournalEntryLineItem> {
        let currency = CurrencyCode::new("CNY").unwrap();
        vec![
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
        ]
    }

    #[test]
    fn test_create_journal_entry() {
        let entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        );

        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.status(), JournalEntryStatus::Draft);
        assert_eq!(entry.line_items().len(), 2);
    }

    #[test]
    fn test_insufficient_line_items() {
        let currency = CurrencyCode::new("CNY").unwrap();
        let line_items = vec![JournalEntryLineItem::new(
            1,
            AccountCode::new("1001").unwrap(),
            Money::new(1000.0, currency.clone()).unwrap(),
            DebitCredit::Debit,
        )
        .unwrap()];

        let result = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::InsufficientLineItems { .. }
        ));
    }

    #[test]
    fn test_unbalanced_entry() {
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
                Money::new(500.0, currency.clone()).unwrap(),
                DebitCredit::Credit,
            )
            .unwrap(),
        ];

        let result = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            currency,
            line_items,
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::UnbalancedEntry { .. }
        ));
    }

    #[test]
    fn test_post_journal_entry() {
        let mut entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        )
        .unwrap();

        let events = entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        assert_eq!(entry.status(), JournalEntryStatus::Posted);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_post_invalid_date() {
        let mut entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        )
        .unwrap();

        let result = entry.post(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::InvalidPostingDate { .. }
        ));
    }

    #[test]
    fn test_reverse_journal_entry() {
        let mut entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        )
        .unwrap();

        // 先过账
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        // 冲销
        let (reversal_entry, events) = entry
            .reverse(
                DocumentNumber::new("JE-002").unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            )
            .unwrap();

        assert_eq!(entry.status(), JournalEntryStatus::Reversed);
        assert!(reversal_entry.is_reversal());
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_add_line_item() {
        let mut entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        )
        .unwrap();

        let new_item = JournalEntryLineItem::new(
            3,
            AccountCode::new("3001").unwrap(),
            Money::new(500.0, CurrencyCode::new("CNY").unwrap()).unwrap(),
            DebitCredit::Debit,
        )
        .unwrap();

        let result = entry.add_line_item(new_item);
        assert!(result.is_ok());
        assert_eq!(entry.line_items().len(), 3);
    }

    #[test]
    fn test_cannot_modify_posted_entry() {
        let mut entry = JournalEntry::create(
            "tenant-001",
            CompanyCode::new("1000").unwrap(),
            2024,
            DocumentNumber::new("JE-001").unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            CurrencyCode::new("CNY").unwrap(),
            create_test_line_items(),
        )
        .unwrap();

        // 过账
        entry
            .post(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            )
            .unwrap();

        // 尝试添加行项目
        let new_item = JournalEntryLineItem::new(
            3,
            AccountCode::new("3001").unwrap(),
            Money::new(500.0, CurrencyCode::new("CNY").unwrap()).unwrap(),
            DebitCredit::Debit,
        )
        .unwrap();

        let result = entry.add_line_item(new_item);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::AlreadyPosted { .. }
        ));
    }
}
