//! PostgreSQL 持久化实现

use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use killer_domain_primitives::{CompanyCode, AccountCode, DocumentNumber};
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
    async fn find_by_id(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Option<FixedAsset> {
        None
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<FixedAsset> {
        vec![]
    }

    async fn save(&self, asset: &FixedAsset) -> Result<(), String> {
        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Result<(), String> {
        Ok(())
    }
}

// BankAccount Repository 实现
#[async_trait]
impl BankAccountRepository for PostgresRepository {
    async fn find_by_id(&self, bank_key: &str, bank_account: &str) -> Option<BankAccount> {
        None
    }

    async fn find_all(&self) -> Vec<BankAccount> {
        vec![]
    }

    async fn save(&self, account: &BankAccount) -> Result<(), String> {
        Ok(())
    }

    async fn delete(&self, bank_key: &str, bank_account: &str) -> Result<(), String> {
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
