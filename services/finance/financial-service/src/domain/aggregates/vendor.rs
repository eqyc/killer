//! 供应商聚合根

use chrono::{DateTime, Utc};
use crate::domain::events::{VendorCreated, VendorPaymentMade};
use killer_domain_primitives::{CompanyCode, Money, AuditInfo};

/// 供应商主数据聚合根
///
/// 代表 SAP 风格的供应商主数据
#[derive(Debug, Clone)]
pub struct Vendor {
    /// 供应商编号
    vendor_id: String,
    /// 公司代码
    company_code: CompanyCode,
    /// 税号
    tax_number_1: Option<String>,
    tax_number_2: Option<String>,
    /// 供应商账户组
    account_group: String,
    /// 货币
    currency: String,
    /// 统驭科目
    reconciliation_account: String,
    /// 付款条件
    payment_terms: String,
    /// 付款方式
    payment_methods: Option<String>,
    /// 付款方
    payer: Option<String>,
    /// 供应商角色
    partner_role: Option<String>,
    /// 供应商代表
    vendor_representative: Option<String>,
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
    status: VendorStatus,
    /// 审计信息
    audit_info: AuditInfo,
}

impl Vendor {
    /// 创建新的供应商
    pub fn new(
        vendor_id: impl Into<String>,
        company_code: CompanyCode,
        account_group: impl Into<String>,
        name_1: impl Into<String>,
        country: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            vendor_id: vendor_id.into(),
            company_code,
            tax_number_1: None,
            tax_number_2: None,
            account_group: account_group.into(),
            currency: currency.into(),
            reconciliation_account: String::new(),
            payment_terms: String::new(),
            payment_methods: None,
            payer: None,
            partner_role: None,
            vendor_representative: None,
            name_1: name_1.into(),
            name_2: None,
            street: None,
            city: None,
            postal_code: None,
            country: country.into(),
            phone_number: None,
            email_address: None,
            status: VendorStatus::Active,
            audit_info: AuditInfo::new("SYSTEM".to_string(), now),
        }
    }

    // Getters
    pub fn vendor_id(&self) -> &str {
        &self.vendor_id
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

    pub fn status(&self) -> VendorStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == VendorStatus::Active
    }

    pub fn is_blocked(&self) -> bool {
        self.status == VendorStatus::Blocked
    }

    pub fn tax_number(&self) -> Option<&str> {
        self.tax_number_1.as_deref()
    }

    pub fn street(&self) -> Option<&str> {
        self.street.as_deref()
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn postal_code(&self) -> Option<&str> {
        self.postal_code.as_deref()
    }

    pub fn payment_methods(&self) -> Option<&str> {
        self.payment_methods.as_deref()
    }

    pub fn phone_number(&self) -> Option<&str> {
        self.phone_number.as_deref()
    }

    pub fn email_address(&self) -> Option<&str> {
        self.email_address.as_deref()
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

    /// 冻结供应商
    pub fn block(&mut self, updated_by: impl Into<String>) {
        self.status = VendorStatus::Blocked;
        self.audit_info.update(updated_by);
    }

    /// 解冻供应商
    pub fn unblock(&mut self, updated_by: impl Into<String>) {
        self.status = VendorStatus::Active;
        self.audit_info.update(updated_by);
    }

    /// 删除供应商
    pub fn delete(&mut self, updated_by: impl Into<String>) {
        self.status = VendorStatus::Deleted;
        self.audit_info.update(updated_by);
    }

    // Events

    pub fn into_created_event(self) -> VendorCreated {
        VendorCreated {
            company_code: self.company_code,
            vendor_id: self.vendor_id,
            created_at: self.audit_info.created_at(),
        }
    }

    pub fn into_payment_made_event(self, document_number: &str, amount: Money) -> VendorPaymentMade {
        VendorPaymentMade {
            company_code: self.company_code,
            vendor_id: self.vendor_id,
            document_number: crate::domain::value_objects::document_number::DocumentNumber::new(document_number).unwrap(),
            amount,
            paid_at: Utc::now(),
        }
    }
}

/// 供应商状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VendorStatus {
    Active = 1,
    Blocked = 2,
    Deleted = 3,
}

impl TryFrom<i32> for VendorStatus {
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
