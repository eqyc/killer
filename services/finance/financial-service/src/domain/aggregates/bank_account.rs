//! 银行账户聚合根

use chrono::{DateTime, Utc};
use killer_domain_primitives::{Money, AuditInfo};

/// 银行账户聚合根
///
/// 代表银行主数据
#[derive(Debug, Clone)]
pub struct BankAccount {
    /// 银行国家代码
    bank_country_code: String,
    /// 银行代码
    bank_key: String,
    /// 银行名称
    bank_name: String,
    /// 街道地址
    street_address: Option<String>,
    /// 城市
    city: Option<String>,
    /// 邮编
    postal_code: Option<String>,
    /// SWIFT代码
    swift_code: Option<String>,
    /// IBAN
    iban: Option<String>,
    /// 银行账户号
    bank_account_number: Option<String>,
    /// 银行类型
    bank_type: Option<String>,
    /// 当前余额
    current_balance: Money,
    /// 可用余额
    available_balance: Money,
    /// 审计信息
    audit_info: AuditInfo,
}

impl BankAccount {
    /// 创建新的银行账户
    pub fn new(
        bank_country_code: impl Into<String>,
        bank_key: impl Into<String>,
        bank_name: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            bank_country_code: bank_country_code.into(),
            bank_key: bank_key.into(),
            bank_name: bank_name.into(),
            street_address: None,
            city: None,
            postal_code: None,
            swift_code: None,
            iban: None,
            bank_account_number: None,
            bank_type: None,
            current_balance: Money::zero(),
            available_balance: Money::zero(),
            audit_info: AuditInfo::new("SYSTEM".to_string(), now),
        }
    }

    // Getters
    pub fn bank_country_code(&self) -> &str {
        &self.bank_country_code
    }

    pub fn bank_key(&self) -> &str {
        &self.bank_key
    }

    pub fn bank_name(&self) -> &str {
        &self.bank_name
    }

    pub fn street_address(&self) -> Option<&str> {
        self.street_address.as_deref()
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn postal_code(&self) -> Option<&str> {
        self.postal_code.as_deref()
    }

    pub fn swift_code(&self) -> Option<&str> {
        self.swift_code.as_deref()
    }

    pub fn iban(&self) -> Option<&str> {
        self.iban.as_deref()
    }

    pub fn bank_account_number(&self) -> Option<&str> {
        self.bank_account_number.as_deref()
    }

    pub fn bank_type(&self) -> Option<&str> {
        self.bank_type.as_deref()
    }

    pub fn current_balance(&self) -> Money {
        self.current_balance
    }

    pub fn available_balance(&self) -> Money {
        self.available_balance
    }

    // Commands

    /// 更新地址信息
    pub fn update_address(
        &mut self,
        street_address: Option<String>,
        city: Option<String>,
        postal_code: Option<String>,
        updated_by: impl Into<String>,
    ) {
        self.street_address = street_address;
        self.city = city;
        self.postal_code = postal_code;
        self.audit_info.update(updated_by);
    }

    /// 设置SWIFT代码
    pub fn set_swift_code(&mut self, swift_code: impl Into<String>, updated_by: impl Into<String>) {
        self.swift_code = Some(swift_code.into());
        self.audit_info.update(updated_by);
    }

    /// 设置IBAN
    pub fn set_iban(&mut self, iban: impl Into<String>, updated_by: impl Into<String>) {
        self.iban = Some(iban.into());
        self.audit_info.update(updated_by);
    }

    /// 设置银行账户号
    pub fn set_bank_account_number(&mut self, account_number: impl Into<String>, updated_by: impl Into<String>) {
        self.bank_account_number = Some(account_number.into());
        self.audit_info.update(updated_by);
    }

    /// 存入资金
    pub fn deposit(&mut self, amount: Money) {
        self.current_balance = self.current_balance.add(amount);
        self.available_balance = self.available_balance.add(amount);
    }

    /// 支出资金
    pub fn withdraw(&mut self, amount: Money) -> Result<(), String> {
        if self.available_balance < amount {
            return Err("可用余额不足".to_string());
        }
        self.available_balance = self.available_balance.sub(amount);
        // 实际扣款可能在清算后才发生
        Ok(())
    }

    /// 确认扣款
    pub fn confirm_debit(&mut self, amount: Money) {
        self.current_balance = self.current_balance.sub(amount);
    }

    /// 更新余额（用于对账）
    pub fn update_balance(&mut self, new_balance: Money) {
        self.current_balance = new_balance;
    }

    /// 从数据库加载时设置银行类型
    pub fn set_bank_type(&mut self, bank_type: impl Into<String>) {
        self.bank_type = Some(bank_type.into());
    }

    /// 从数据库加载时设置余额
    pub fn set_balance(&mut self, current: Money, available: Money) {
        self.current_balance = current;
        self.available_balance = available;
    }
}
