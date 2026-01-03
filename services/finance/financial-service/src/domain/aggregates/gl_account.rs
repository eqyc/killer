//! 总账科目聚合根

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::account_code::AccountCode;
use crate::domain::events::{GLAccountCreated, GLAccountUpdated};
use killer_domain_primitives::{CompanyCode, Money, AuditInfo};

/// 总账科目聚合根
///
/// 代表 SAP 风格的总账科目主数据
#[derive(Debug, Clone)]
pub struct GLAccount {
    /// 科目表
    chart_of_accounts: String,
    /// 科目代码
    account_code: AccountCode,
    /// 公司代码
    company_code: CompanyCode,
    /// 科目类型
    account_type: String,
    /// 资产负债表标识
    balance_sheet_indicator: String,
    /// 成本控制范围
    cost_control_area: String,
    /// 货币
    currency: String,
    /// 科目组
    account_group: String,
    /// 科目标识组
    account_indicator_group: String,
    /// 合并科目代码
    consolidation_account: Option<String>,
    /// 科目描述（标准）
    description: String,
    /// 科目描述（短）
    short_description: String,
    /// 科目描述（长）
    long_description: Option<String>,
    /// 删除标识
    is_deleted: bool,
    /// 审计信息
    audit_info: AuditInfo,
}

impl GLAccount {
    /// 创建新的总账科目
    pub fn new(
        chart_of_accounts: impl Into<String>,
        account_code: AccountCode,
        company_code: CompanyCode,
        account_type: impl Into<String>,
        balance_sheet_indicator: impl Into<String>,
        currency: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            chart_of_accounts: chart_of_accounts.into(),
            account_code,
            company_code,
            account_type: account_type.into(),
            balance_sheet_indicator: balance_sheet_indicator.into(),
            cost_control_area: String::new(),
            currency: currency.into(),
            account_group: String::new(),
            account_indicator_group: String::new(),
            consolidation_account: None,
            description: description.into(),
            short_description: String::new(),
            long_description: None,
            is_deleted: false,
            audit_info: AuditInfo::new("SYSTEM".to_string(), now),
        }
    }

    // Getters
    pub fn chart_of_accounts(&self) -> &str {
        &self.chart_of_accounts
    }

    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }

    pub fn company_code(&self) -> &CompanyCode {
        &self.company_code
    }

    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn balance_sheet_indicator(&self) -> &str {
        &self.balance_sheet_indicator
    }

    pub fn cost_control_area(&self) -> &str {
        &self.cost_control_area
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn account_group(&self) -> &str {
        &self.account_group
    }

    pub fn account_indicator_group(&self) -> &str {
        &self.account_indicator_group
    }

    pub fn consolidation_account(&self) -> Option<&str> {
        self.consolidation_account.as_deref()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn short_description(&self) -> &str {
        &self.short_description
    }

    pub fn long_description(&self) -> Option<&str> {
        self.long_description.as_deref()
    }

    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    pub fn audit_info(&self) -> &AuditInfo {
        &self.audit_info
    }

    // Setters / Commands

    /// 更新科目描述
    pub fn update_description(&mut self, description: impl Into<String>, updated_by: impl Into<String>) {
        self.description = description.into();
        self.audit_info.update(updated_by);
    }

    /// 更新货币
    pub fn update_currency(&mut self, currency: impl Into<String>, updated_by: impl Into<String>) {
        self.currency = currency.into();
        self.audit_info.update(updated_by);
    }

    /// 设置成本控制范围
    pub fn set_cost_control_area(&mut self, area: impl Into<String>, updated_by: impl Into<String>) {
        self.cost_control_area = area.into();
        self.audit_info.update(updated_by);
    }

    /// 设置科目组
    pub fn set_account_group(&mut self, group: impl Into<String>, updated_by: impl Into<String>) {
        self.account_group = group.into();
        self.audit_info.update(updated_by);
    }

    /// 设置合并科目
    pub fn set_consolidation_account(&mut self, account: impl Into<String>, updated_by: impl Into<String>) {
        self.consolidation_account = Some(account.into());
        self.audit_info.update(updated_by);
    }

    /// 删除科目（软删除）
    pub fn mark_deleted(&mut self, updated_by: impl Into<String>) {
        self.is_deleted = true;
        self.audit_info.update(updated_by);
    }

    /// 恢复科目
    pub fn restore(&mut self, updated_by: impl Into<String>) {
        self.is_deleted = false;
        self.audit_info.update(updated_by);
    }

    /// 生成领域事件
    pub fn into_created_event(self) -> GLAccountCreated {
        GLAccountCreated {
            company_code: self.company_code,
            account_code: self.account_code,
            created_at: self.audit_info.created_at(),
        }
    }

    /// 生成更新事件
    pub fn into_updated_event(self) -> GLAccountUpdated {
        GLAccountUpdated {
            company_code: self.company_code,
            account_code: self.account_code,
            updated_at: self.audit_info.updated_at().unwrap_or_else(Utc::now),
        }
    }
}
