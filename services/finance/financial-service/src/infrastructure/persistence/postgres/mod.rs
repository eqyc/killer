//! PostgreSQL 持久化实现

use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use std::str::FromStr;
use killer_domain_primitives::{CompanyCode, AccountCode, DocumentNumber, Money};
use crate::domain::aggregates::{
    gl_account::GLAccount,
    journal_entry::JournalEntry,
    customer::Customer,
    vendor::Vendor,
    fixed_asset::FixedAsset,
    bank_account::BankAccount,
};
use crate::domain::repositories::*;

/// PostgreSQL 仓储实现
#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pool: Pool<Postgres>,
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

// GLAccount Repository 实现
#[async_trait]
impl GLAccountRepository for PostgresRepository {
    async fn find_by_id(
        &self,
        company_code: &CompanyCode,
        account_code: &AccountCode,
    ) -> Option<GLAccount> {
        let query = "SELECT * FROM gl_accounts WHERE bukrs = $1 AND saknr = $2 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(account_code.as_str())
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_gl_account(&row))
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<GLAccount> {
        let query = "SELECT * FROM gl_accounts WHERE bukrs = $1 AND xloeb = false ORDER BY saknr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_gl_account).collect()
    }

    async fn save(&self, account: &GLAccount) -> Result<(), String> {
        let query = r#"
            INSERT INTO gl_accounts (
                ktopl, saknr, bukrs, kontt, xbilk, kstbs, waers, bilkt, mitkz,
                saknr_glv, ktext, mtxt1, mtxt2, xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (bukrs, saknr) DO UPDATE SET
                ktext = EXCLUDED.ktext,
                mtxt1 = EXCLUDED.mtxt1,
                mtxt2 = EXCLUDED.mtxt2,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit = account.audit_info();

        sqlx::query(query)
            .bind(account.chart_of_accounts())
            .bind(account.account_code().as_str())
            .bind(account.company_code().as_str())
            .bind(account.account_type())
            .bind(account.balance_sheet_indicator())
            .bind(account.cost_control_area())
            .bind(account.currency())
            .bind(account.account_group())
            .bind(account.account_indicator_group())
            .bind(account.consolidation_account())
            .bind(account.description())
            .bind(account.short_description())
            .bind(account.long_description())
            .bind(account.is_deleted())
            .bind(audit.created_at().date_naive())
            .bind(audit.created_by())
            .bind(now)
            .bind("SYSTEM")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, account_code: &AccountCode) -> Result<(), String> {
        let query = "UPDATE gl_accounts SET xloeb = true WHERE bukrs = $1 AND saknr = $2";
        sqlx::query(query)
            .bind(company_code.as_str())
            .bind(account_code.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// JournalEntry Repository 实现
#[async_trait]
impl JournalEntryRepository for PostgresRepository {
    async fn find_by_id(
        &self,
        company_code: &CompanyCode,
        document_number: &DocumentNumber,
        fiscal_year: &str,
    ) -> Option<JournalEntry> {
        // 简化实现
        None
    }

    async fn find_all(&self, company_code: &CompanyCode, fiscal_year: &str) -> Vec<JournalEntry> {
        vec![]
    }

    async fn save(&self, entry: &JournalEntry) -> Result<(), String> {
        // 简化实现
        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, document_number: &DocumentNumber, fiscal_year: &str) -> Result<(), String> {
        Ok(())
    }
}

// Customer Repository 实现
#[async_trait]
impl CustomerRepository for PostgresRepository {
    async fn find_by_id(&self, company_code: &CompanyCode, customer_id: &str) -> Option<Customer> {
        None
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Customer> {
        vec![]
    }

    async fn save(&self, customer: &Customer) -> Result<(), String> {
        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, customer_id: &str) -> Result<(), String> {
        Ok(())
    }
}

// Vendor Repository 实现
#[async_trait]
impl VendorRepository for PostgresRepository {
    async fn find_by_id(&self, company_code: &CompanyCode, vendor_id: &str) -> Option<Vendor> {
        None
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Vendor> {
        vec![]
    }

    async fn save(&self, vendor: &Vendor) -> Result<(), String> {
        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, vendor_id: &str) -> Result<(), String> {
        Ok(())
    }
}

// FixedAsset Repository 实现
#[async_trait]
impl FixedAssetRepository for PostgresRepository {
    async fn find_by_id(
        &self,
        company_code: &CompanyCode,
        asset_number: &str,
        sub_number: &str,
    ) -> Option<FixedAsset> {
        let query = "SELECT * FROM fixed_assets WHERE bukrs = $1 AND anlznr = $2 AND sub_number = $3 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(asset_number)
            .bind(sub_number)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_fixed_asset(&row))
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<FixedAsset> {
        let query = "SELECT * FROM fixed_assets WHERE bukrs = $1 AND xloeb = false ORDER BY anlznr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_fixed_asset).collect()
    }

    async fn find_by_status(&self, company_code: &CompanyCode, status: i32) -> Vec<FixedAsset> {
        let query = "SELECT * FROM fixed_assets WHERE bukrs = $1 AND status = $2 AND xloeb = false ORDER BY anlznr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(status)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_fixed_asset).collect()
    }

    async fn save(&self, asset: &FixedAsset) -> Result<(), String> {
        let query = r#"
            INSERT INTO fixed_assets (
                bukrs, anlznr, sub_number, anlkl, txkfs, kstbs, prfs, bwkey, stort, invest,
                txt50, anlhtxt, aktiv, acqu, aacc, plan, nafa, sauss, xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            ON CONFLICT (bukrs, anlznr, sub_number) DO UPDATE SET
                anlkl = EXCLUDED.anlkl,
                txkfs = EXCLUDED.txkfs,
                kstbs = EXCLUDED.kstbs,
                prfs = EXCLUDED.prfs,
                bwkey = EXCLUDED.bwkey,
                stort = EXCLUDED.stort,
                invest = EXCLUDED.invest,
                txt50 = EXCLUDED.txt50,
                anlhtxt = EXCLUDED.anlhtxt,
                aktiv = EXCLUDED.aktiv,
                aacc = EXCLUDED.aacc,
                status = EXCLUDED.status,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit_info = &asset.audit_info;

        sqlx::query(query)
            .bind(asset.company_code().as_str())
            .bind(asset.asset_number())
            .bind(asset.sub_number())
            .bind(asset.asset_class())
            .bind(asset.valuation_class())
            .bind(asset.cost_center().unwrap_or(""))
            .bind(asset.profit_center().unwrap_or(""))
            .bind(asset.business_area().unwrap_or(""))
            .bind(asset.location().unwrap_or(""))
            .bind(asset.investment_order().unwrap_or(""))
            .bind(asset.description())
            .bind(asset.long_description().unwrap_or(""))
            .bind(asset.capitalization_date().map(|d| d.to_string()).as_deref())
            .bind(asset.acquisition_value().amount().to_string())
            .bind(asset.accumulated_depreciation().amount().to_string())
            .bind(asset.unplanned_depreciation().amount().to_string())
            .bind(false)
            .bind(audit_info.created_at().date_naive())
            .bind(audit_info.created_by())
            .bind(now)
            .bind("SYSTEM")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Result<(), String> {
        let query = "UPDATE fixed_assets SET xloeb = true WHERE bukrs = $1 AND anlznr = $2 AND sub_number = $3";
        sqlx::query(query)
            .bind(company_code.as_str())
            .bind(asset_number)
            .bind(sub_number)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// BankAccount Repository 实现
#[async_trait]
impl BankAccountRepository for PostgresRepository {
    async fn find_by_id(&self, bank_key: &str, bank_account: &str) -> Option<BankAccount> {
        let query = "SELECT * FROM bank_accounts WHERE bank_key = $1 AND bank_account_number = $2 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(bank_key)
            .bind(bank_account)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_bank_account(&row))
    }

    async fn find_all(&self) -> Vec<BankAccount> {
        let query = "SELECT * FROM bank_accounts WHERE xloeb = false ORDER BY bank_key, bank_account_number";
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_bank_account).collect()
    }

    async fn find_by_country(&self, country_code: &str) -> Vec<BankAccount> {
        let query = "SELECT * FROM bank_accounts WHERE bank_country_code = $1 AND xloeb = false ORDER BY bank_key";
        let rows = sqlx::query(query)
            .bind(country_code)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_bank_account).collect()
    }

    async fn find_by_swift(&self, swift_code: &str) -> Option<BankAccount> {
        let query = "SELECT * FROM bank_accounts WHERE swift_code = $1 AND xloeb = false LIMIT 1";
        let row = sqlx::query(query)
            .bind(swift_code)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_bank_account(&row))
    }

    async fn save(&self, account: &BankAccount) -> Result<(), String> {
        let query = r#"
            INSERT INTO bank_accounts (
                bank_country_code, bank_key, bank_name, street_address, city, postal_code,
                swift_code, iban, bank_account_number, bank_type, current_balance, available_balance,
                xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (bank_key, bank_account_number) DO UPDATE SET
                bank_name = EXCLUDED.bank_name,
                street_address = EXCLUDED.street_address,
                city = EXCLUDED.city,
                postal_code = EXCLUDED.postal_code,
                swift_code = EXCLUDED.swift_code,
                iban = EXCLUDED.iban,
                bank_type = EXCLUDED.bank_type,
                current_balance = EXCLUDED.current_balance,
                available_balance = EXCLUDED.available_balance,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit_info = &account.audit_info;

        sqlx::query(query)
            .bind(account.bank_country_code())
            .bind(account.bank_key())
            .bind(account.bank_name())
            .bind(account.street_address().unwrap_or(""))
            .bind(account.city().unwrap_or(""))
            .bind(account.postal_code().unwrap_or(""))
            .bind(account.swift_code().unwrap_or(""))
            .bind(account.iban().unwrap_or(""))
            .bind(account.bank_account_number().unwrap_or(""))
            .bind(account.bank_type().unwrap_or(""))
            .bind(account.current_balance().amount().to_string())
            .bind(account.available_balance().amount().to_string())
            .bind(false)
            .bind(audit_info.created_at().date_naive())
            .bind(audit_info.created_by())
            .bind(now)
            .bind("SYSTEM")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete(&self, bank_key: &str, bank_account: &str) -> Result<(), String> {
        let query = "UPDATE bank_accounts SET xloeb = true WHERE bank_key = $1 AND bank_account_number = $2";
        sqlx::query(query)
            .bind(bank_key)
            .bind(bank_account)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_balance(&self, bank_key: &str, bank_account: &str, new_balance: Money) -> Result<(), String> {
        let query = r#"
            UPDATE bank_accounts SET
                current_balance = $3,
                available_balance = $3,
                aedat = $4,
                aenam = $5
            WHERE bank_key = $1 AND bank_account_number = $2 AND xloeb = false
        "#;

        let now = chrono::Utc::now().date_naive();

        sqlx::query(query)
            .bind(bank_key)
            .bind(bank_account)
            .bind(new_balance.amount().to_string())
            .bind(now)
            .bind("SYSTEM")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// 辅助函数
fn row_to_gl_account(row: &sqlx::postgres::PgRow) -> GLAccount {
    // 简化实现
    GLAccount::new(
        row.try_get::<String, _>("ktopl").unwrap_or_default(),
        AccountCode::new(row.try_get::<String, _>("saknr").unwrap_or_default()).unwrap(),
        CompanyCode::new(row.try_get::<String, _>("bukrs").unwrap_or_default()).unwrap(),
        row.try_get::<String, _>("kontt").unwrap_or_default(),
        row.try_get::<String, _>("xbilk").unwrap_or_default(),
        row.try_get::<String, _>("waers").unwrap_or_default(),
        row.try_get::<String, _>("ktext").unwrap_or_default(),
    )
}

fn row_to_fixed_asset(row: &sqlx::postgres::PgRow) -> FixedAsset {
    let mut asset = FixedAsset::new(
        CompanyCode::new(row.try_get::<String, _>("bukrs").unwrap_or_default()).unwrap_or_default(),
        row.try_get::<String, _>("anlkl").unwrap_or_default(),
        row.try_get::<String, _>("txkfs").unwrap_or_default(),
        row.try_get::<String, _>("txt50").unwrap_or_default(),
    );

    asset.set_asset_number(row.try_get::<String, _>("anlznr").unwrap_or_default());
    asset.set_sub_number(row.try_get::<String, _>("sub_number").unwrap_or_default());

    if let Ok(kstbs) = row.try_get::<String, _>("kstbs") {
        if !kstbs.is_empty() {
            asset.set_cost_center(kstbs);
        }
    }

    if let Ok(prfs) = row.try_get::<String, _>("prfs") {
        if !prfs.is_empty() {
            asset.set_profit_center(prfs);
        }
    }

    if let Ok(bwkey) = row.try_get::<String, _>("bwkey") {
        if !bwkey.is_empty() {
            asset.set_location(bwkey);
        }
    }

    asset
}

fn row_to_bank_account(row: &sqlx::postgres::PgRow) -> BankAccount {
    let mut account = BankAccount::new(
        row.try_get::<String, _>("bank_country_code").unwrap_or_default(),
        row.try_get::<String, _>("bank_key").unwrap_or_default(),
        row.try_get::<String, _>("bank_name").unwrap_or_default(),
    );

    // 设置银行类型
    if let Ok(bt) = row.try_get::<String, _>("bank_type") {
        if !bt.is_empty() {
            account.set_bank_type(bt);
        }
    }

    // 设置余额
    let current = Money::from_str(row.try_get::<String, _>("current_balance").unwrap_or_default().as_str())
        .unwrap_or_else(|_| Money::zero());
    let available = Money::from_str(row.try_get::<String, _>("available_balance").unwrap_or_default().as_str())
        .unwrap_or_else(|_| Money::zero());
    account.set_balance(current, available);

    account
}
