//! 会计凭证行项目实体

use std::collections::HashMap;
use crate::domain::value_objects::{account_code::AccountCode, posting_date::PostingDate};

/// 会计凭证行项目
///
/// 凭证的每一行记录，包含借贷方向、科目、金额等信息
#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntryItem {
    /// 行项目号
    line_number: u32,
    /// 总账科目
    gl_account: AccountCode,
    /// 借贷标识
    debit_credit: DebitCreditIndicator,
    /// 凭证货币金额
    document_currency_amount: killer_domain_primitives::Money,
    /// 本位币金额
    local_currency_amount: killer_domain_primitives::Money,
    /// 客户编号（可选）
    customer_id: Option<String>,
    /// 供应商编号（可选）
    vendor_id: Option<String>,
    /// 成本中心（可选）
    cost_center: Option<String>,
    /// 利润中心（可选）
    profit_center: Option<String>,
    /// 业务范围（可选）
    business_area: Option<String>,
    /// 功能范围（可选）
    functional_area: Option<String>,
    /// 税码（可选）
    tax_code: Option<String>,
    /// 行项目文本
    line_text: Option<String>,
    /// 分配字段
    assignment_number: Option<String>,
    /// 内部订单（可选）
    internal_order: Option<String>,
    /// WBS元素（可选）
    wbs_element: Option<String>,
    /// 采购订单（可选）
    purchase_order: Option<String>,
    /// 采购订单行项目（可选）
    purchase_order_item: Option<String>,
}

impl JournalEntryItem {
    /// 创建新的行项目
    pub fn new(
        line_number: u32,
        gl_account: AccountCode,
        debit_credit: DebitCreditIndicator,
        document_currency_amount: killer_domain_primitives::Money,
        local_currency_amount: killer_domain_primitives::Money,
    ) -> Result<Self, JournalEntryItemError> {
        if line_number == 0 {
            return Err(JournalEntryItemError::InvalidLineNumber);
        }
        Ok(Self {
            line_number,
            gl_account,
            debit_credit,
            document_currency_amount,
            local_currency_amount,
            customer_id: None,
            vendor_id: None,
            cost_center: None,
            profit_center: None,
            business_area: None,
            functional_area: None,
            tax_code: None,
            line_text: None,
            assignment_number: None,
            internal_order: None,
            wbs_element: None,
            purchase_order: None,
            purchase_order_item: None,
        })
    }

    // Getters
    pub fn line_number(&self) -> u32 {
        self.line_number
    }

    pub fn gl_account(&self) -> &AccountCode {
        &self.gl_account
    }

    pub fn debit_credit(&self) -> DebitCreditIndicator {
        self.debit_credit
    }

    pub fn document_currency_amount(&self) -> killer_domain_primitives::Money {
        self.document_currency_amount
    }

    pub fn local_currency_amount(&self) -> killer_domain_primitives::Money {
        self.local_currency_amount
    }

    pub fn customer_id(&self) -> Option<&str> {
        self.customer_id.as_deref()
    }

    pub fn vendor_id(&self) -> Option<&str> {
        self.vendor_id.as_deref()
    }

    pub fn cost_center(&self) -> Option<&str> {
        self.cost_center.as_deref()
    }

    pub fn profit_center(&self) -> Option<&str> {
        self.profit_center.as_deref()
    }

    pub fn business_area(&self) -> Option<&str> {
        self.business_area.as_deref()
    }

    pub fn functional_area(&self) -> Option<&str> {
        self.functional_area.as_deref()
    }

    pub fn tax_code(&self) -> Option<&str> {
        self.tax_code.as_deref()
    }

    pub fn line_text(&self) -> Option<&str> {
        self.line_text.as_deref()
    }

    pub fn assignment_number(&self) -> Option<&str> {
        self.assignment_number.as_deref()
    }

    pub fn internal_order(&self) -> Option<&str> {
        self.internal_order.as_deref()
    }

    pub fn wbs_element(&self) -> Option<&str> {
        self.wbs_element.as_deref()
    }

    pub fn purchase_order(&self) -> Option<&str> {
        self.purchase_order.as_deref()
    }

    pub fn purchase_order_item(&self) -> Option<&str> {
        self.purchase_order_item.as_deref()
    }

    /// 设置客户
    pub fn with_customer(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// 设置供应商
    pub fn with_vendor(mut self, vendor_id: impl Into<String>) -> Self {
        self.vendor_id = Some(vendor_id.into());
        self
    }

    /// 设置成本中心
    pub fn with_cost_center(mut self, cost_center: impl Into<String>) -> Self {
        self.cost_center = Some(cost_center.into());
        self
    }

    /// 设置利润中心
    pub fn with_profit_center(mut self, profit_center: impl Into<String>) -> Self {
        self.profit_center = Some(profit_center.into());
        self
    }

    /// 设置行项目文本
    pub fn with_line_text(mut self, text: impl Into<String>) -> Self {
        self.line_text = Some(text.into());
        self
    }

    /// 设置分配字段
    pub fn with_assignment(mut self, assignment: impl Into<String>) -> Self {
        self.assignment_number = Some(assignment.into());
        self
    }

    /// 判断是否为借方
    pub fn is_debit(&self) -> bool {
        self.debit_credit == DebitCreditIndicator::Debit
    }

    /// 判断是否为贷方
    pub fn is_credit(&self) -> bool {
        self.debit_credit == DebitCreditIndicator::Credit
    }
}

/// 借贷标识
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebitCreditIndicator {
    Debit = 1,   // 借方 S
    Credit = 2,  // 贷方 H
}

impl TryFrom<i32> for DebitCreditIndicator {
    type Error = JournalEntryItemError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Debit),
            2 => Ok(Self::Credit),
            _ => Err(JournalEntryItemError::InvalidDebitCredit(value)),
        }
    }
}

/// 行项目错误
#[derive(Debug, thiserror::Error)]
pub enum JournalEntryItemError {
    #[error("无效的行项目号: {0}")]
    InvalidLineNumber,
    #[error("无效的借贷标识: {0}")]
    InvalidDebitCredit(i32),
}
