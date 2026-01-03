//! 客户聚合根

use chrono::{DateTime, Utc};
use crate::domain::events::{CustomerCreated, CustomerPaymentReceived};
use killer_domain_primitives::{CompanyCode, Money, AuditInfo};

/// 客户主数据聚合根
///
/// 代表 SAP 风格的客户主数据
#[derive(Debug, Clone)]
pub struct Customer {
    /// 客户编号
    customer_id: String,
    /// 公司代码
    company_code: CompanyCode,
    /// 税号
    tax_number_1: Option<String>,
    tax_number_2: Option<String>,
    /// 客户账户组
    account_group: String,
    /// 售达方
    sold_to_party: Option<String>,
    /// 收票方
    bill_to_party: Option<String>,
    /// 送达方
    ship_to_party: Option<String>,
    /// 货币
    currency: String,
    /// 统驭科目
    reconciliation_account: String,
    /// 付款条件
    payment_terms: String,
    /// 付款方式
    payment_methods: Option<String>,
    /// 催收代码
    dunning_area: Option<String>,
    /// 客户代表
    customer_representative: Option<String>,
    /// 名称
    name_1: String,
    name_2: Option<String>,
    /// 地址
    street: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country: String,
    /// 联系方式
    phone_number: Option<String>,
    email_address: Option<String>,
    /// 状态
    status: CustomerStatus,
    /// 审计信息
    audit_info: AuditInfo,
}

impl Customer {
    /// 创建新的客户
    pub fn new(
        customer_id: impl Into<String>,
        company_code: CompanyCode,
        account_group: impl Into<String>,
        name_1: impl Into<String>,
        country: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            customer_id: customer_id.into(),
            company_code,
            tax_number_1: None,
            tax_number_2: None,
            account_group: account_group.into(),
            sold_to_party: None,
            bill_to_party: None,
            ship_to_party: None,
            currency: currency.into(),
            reconciliation_account: String::new(),
            payment_terms: String::new(),
            payment_methods: None,
            dunning_area: None,
            customer_representative: None,
            name_1: name_1.into(),
            name_2: None,
            street: None,
            city: None,
            postal_code: None,
            country: country.into(),
            phone_number: None,
            email_address: None,
            status: CustomerStatus::Active,
            audit_info: AuditInfo::new("SYSTEM".to_string(), now),
        }
    }

    // Getters
    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn company_code(&self) -> &CompanyCode {
        &self.company_code
    }

    pub fn account_group(&self) -> &str {
        &self.account_group
    }

    pub fn name_1(&self) -> &str {
        &self.name_1
    }

    pub fn name_2(&self) -> Option<&str> {
        self.name_2.as_deref()
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn reconciliation_account(&self) -> &str {
        &self.reconciliation_account
    }

    pub fn payment_terms(&self) -> &str {
        &self.payment_terms
    }

    pub fn status(&self) -> CustomerStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == CustomerStatus::Active
    }

    pub fn is_blocked(&self) -> bool {
        self.status == CustomerStatus::Blocked
    }

    // Commands

    /// 更新基本信息
    pub fn update_basic_info(
        &mut self,
        name_1: impl Into<String>,
        street: Option<String>,
        city: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        updated_by: impl Into<String>,
    ) {
        self.name_1 = name_1.into();
        self.street = street;
        self.city = city;
        self.postal_code = postal_code;
        if let Some(c) = country {
            self.country = c;
        }
        self.audit_info.update(updated_by);
    }

    /// 更新财务信息
    pub fn update_financial_info(
        &mut self,
        currency: Option<String>,
        reconciliation_account: impl Into<String>,
        payment_terms: impl Into<String>,
        payment_methods: Option<String>,
        updated_by: impl Into<String>,
    ) {
        if let Some(c) = currency {
            self.currency = c;
        }
        self.reconciliation_account = reconciliation_account.into();
        self.payment_terms = payment_terms.into();
        self.payment_methods = payment_methods;
        self.audit_info.update(updated_by);
    }

    /// 设置税号
    pub fn set_tax_number(&mut self, tax_number_1: impl Into<String>, updated_by: impl Into<String>) {
        self.tax_number_1 = Some(tax_number_1.into());
        self.audit_info.update(updated_by);
    }

    /// 设置联系人
    pub fn set_contact(&mut self, phone: Option<String>, email: Option<String>, updated_by: impl Into<String>) {
        self.phone_number = phone;
        self.email_address = email;
        self.audit_info.update(updated_by);
    }

    /// 冻结客户
    pub fn block(&mut self, updated_by: impl Into<String>) {
        self.status = CustomerStatus::Blocked;
        self.audit_info.update(updated_by);
    }

    /// 解冻客户
    pub fn unblock(&mut self, updated_by: impl Into<String>) {
        self.status = CustomerStatus::Active;
        self.audit_info.update(updated_by);
    }

    /// 删除客户
    pub fn delete(&mut self, updated_by: impl Into<String>) {
        self.status = CustomerStatus::Deleted;
        self.audit_info.update(updated_by);
    }

    // Events

    pub fn into_created_event(self) -> CustomerCreated {
        CustomerCreated {
            company_code: self.company_code,
            customer_id: self.customer_id,
            created_at: self.audit_info.created_at(),
        }
    }

    pub fn into_payment_received_event(self, document_number: &str, amount: Money) -> CustomerPaymentReceived {
        CustomerPaymentReceived {
            company_code: self.company_code,
            customer_id: self.customer_id,
            document_number: crate::domain::value_objects::document_number::DocumentNumber::new(document_number).unwrap(),
            amount,
            received_at: Utc::now(),
        }
    }
}

/// 客户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerStatus {
    Active = 1,
    Blocked = 2,
    Deleted = 3,
}

impl TryFrom<i32> for CustomerStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Active),
            2 => Ok(Self::Blocked),
            3 => Ok(Self::Deleted),
            _ => Err(()),
        }
    }
}
