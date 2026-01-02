//! 领域实体
//!
//! 定义财务领域的实体对象

use crate::domain::error::{DomainError, DomainResult};
use crate::domain::value_objects::{DebitCredit, ProfitCenter};
use chrono::NaiveDate;
use killer_domain_primitives::{AccountCode, CostCenter, Money};
use serde::{Deserialize, Serialize};

// =============================================================================
// 会计凭证行项目实体
// =============================================================================

/// 会计凭证行项目
///
/// 对应 SAP ACDOCA 表的单行记录
/// 不可变实体 - 构造后不可修改
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryLineItem {
    /// 行号 (从1开始)
    line_number: u32,

    /// 会计科目
    account_code: AccountCode,

    /// 金额 (始终为正数)
    amount: Money,

    /// 借贷方向
    debit_credit: DebitCredit,

    /// 成本中心 (可选)
    cost_center: Option<CostCenter>,

    /// 利润中心 (可选)
    profit_center: Option<ProfitCenter>,

    /// 行项目文本
    text: Option<String>,

    /// 功能范围 (对应 SAP FKBER)
    functional_area: Option<String>,

    /// 业务范围 (对应 SAP GSBER)
    business_area: Option<String>,

    /// 订单号 (对应 SAP AUFNR)
    order_number: Option<String>,
}

impl JournalEntryLineItem {
    /// 创建新的行项目
    ///
    /// # 参数
    /// - `line_number`: 行号
    /// - `account_code`: 会计科目
    /// - `amount`: 金额 (必须为正数)
    /// - `debit_credit`: 借贷方向
    ///
    /// # 不变式
    /// - 金额必须大于零
    /// - 行号必须大于零
    pub fn new(
        line_number: u32,
        account_code: AccountCode,
        amount: Money,
        debit_credit: DebitCredit,
    ) -> DomainResult<Self> {
        // 验证行号
        if line_number == 0 {
            return Err(DomainError::ValidationError {
                message: "行号必须大于零".to_string(),
            });
        }

        // 验证金额为正数
        if amount.amount() <= 0.0 {
            return Err(DomainError::InvalidAmount {
                reason: "金额必须大于零".to_string(),
            });
        }

        Ok(Self {
            line_number,
            account_code,
            amount,
            debit_credit,
            cost_center: None,
            profit_center: None,
            text: None,
            functional_area: None,
            business_area: None,
            order_number: None,
        })
    }

    /// 设置成本中心
    pub fn with_cost_center(mut self, cost_center: CostCenter) -> Self {
        self.cost_center = Some(cost_center);
        self
    }

    /// 设置利润中心
    pub fn with_profit_center(mut self, profit_center: ProfitCenter) -> Self {
        self.profit_center = Some(profit_center);
        self
    }

    /// 设置行项目文本
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// 设置功能范围
    pub fn with_functional_area(mut self, functional_area: impl Into<String>) -> Self {
        self.functional_area = Some(functional_area.into());
        self
    }

    /// 设置业务范围
    pub fn with_business_area(mut self, business_area: impl Into<String>) -> Self {
        self.business_area = Some(business_area.into());
        self
    }

    /// 设置订单号
    pub fn with_order_number(mut self, order_number: impl Into<String>) -> Self {
        self.order_number = Some(order_number.into());
        self
    }

    // Getters

    /// 获取行号
    pub fn line_number(&self) -> u32 {
        self.line_number
    }

    /// 获取会计科目
    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }

    /// 获取金额
    pub fn amount(&self) -> &Money {
        &self.amount
    }

    /// 获取借贷方向
    pub fn debit_credit(&self) -> DebitCredit {
        self.debit_credit
    }

    /// 获取成本中心
    pub fn cost_center(&self) -> Option<&CostCenter> {
        self.cost_center.as_ref()
    }

    /// 获取利润中心
    pub fn profit_center(&self) -> Option<&ProfitCenter> {
        self.profit_center.as_ref()
    }

    /// 获取行项目文本
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// 获取功能范围
    pub fn functional_area(&self) -> Option<&str> {
        self.functional_area.as_deref()
    }

    /// 获取业务范围
    pub fn business_area(&self) -> Option<&str> {
        self.business_area.as_deref()
    }

    /// 获取订单号
    pub fn order_number(&self) -> Option<&str> {
        self.order_number.as_deref()
    }

    /// 获取带符号的金额 (借方为正，贷方为负)
    pub fn signed_amount(&self) -> Money {
        match self.debit_credit {
            DebitCredit::Debit => self.amount.clone(),
            DebitCredit::Credit => Money::new(-self.amount.amount(), self.amount.currency().clone())
                .expect("金额符号转换失败"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use killer_domain_primitives::CurrencyCode;

    #[test]
    fn test_create_line_item() {
        let account = AccountCode::new("1001").unwrap();
        let amount = Money::new(1000.0, CurrencyCode::new("CNY").unwrap()).unwrap();

        let line_item = JournalEntryLineItem::new(1, account, amount, DebitCredit::Debit).unwrap();

        assert_eq!(line_item.line_number(), 1);
        assert_eq!(line_item.debit_credit(), DebitCredit::Debit);
    }

    #[test]
    fn test_line_item_with_cost_center() {
        let account = AccountCode::new("1001").unwrap();
        let amount = Money::new(1000.0, CurrencyCode::new("CNY").unwrap()).unwrap();
        let cost_center = CostCenter::new("CC001").unwrap();

        let line_item = JournalEntryLineItem::new(1, account, amount, DebitCredit::Debit)
            .unwrap()
            .with_cost_center(cost_center.clone());

        assert_eq!(line_item.cost_center(), Some(&cost_center));
    }

    #[test]
    fn test_invalid_line_number() {
        let account = AccountCode::new("1001").unwrap();
        let amount = Money::new(1000.0, CurrencyCode::new("CNY").unwrap()).unwrap();

        let result = JournalEntryLineItem::new(0, account, amount, DebitCredit::Debit);

        assert!(result.is_err());
    }

    #[test]
    fn test_signed_amount() {
        let account = AccountCode::new("1001").unwrap();
        let amount = Money::new(1000.0, CurrencyCode::new("CNY").unwrap()).unwrap();

        let debit_item = JournalEntryLineItem::new(1, account.clone(), amount.clone(), DebitCredit::Debit).unwrap();
        assert_eq!(debit_item.signed_amount().amount(), 1000.0);

        let credit_item = JournalEntryLineItem::new(2, account, amount, DebitCredit::Credit).unwrap();
        assert_eq!(credit_item.signed_amount().amount(), -1000.0);
    }
}
