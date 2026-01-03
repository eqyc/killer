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
        let query = "SELECT * FROM journal_entries WHERE bukrs = $1 AND belnr = $2 AND gjahr = $3 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(document_number.as_str())
            .bind(fiscal_year)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_journal_entry(&row))
    }

    async fn find_all(&self, company_code: &CompanyCode, fiscal_year: &str) -> Vec<JournalEntry> {
        let query = "SELECT * FROM journal_entries WHERE bukrs = $1 AND gjahr = $2 AND xloeb = false ORDER BY belnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(fiscal_year)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_journal_entry).collect()
    }

    async fn find_by_status(&self, company_code: &CompanyCode, fiscal_year: &str, status: i32) -> Vec<JournalEntry> {
        let query = "SELECT * FROM journal_entries WHERE bukrs = $1 AND gjahr = $2 AND status = $3 AND xloeb = false ORDER BY belnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(fiscal_year)
            .bind(status)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_journal_entry).collect()
    }

    async fn find_posted_in_period(
        &self,
        company_code: &CompanyCode,
        posting_date_from: chrono::NaiveDate,
        posting_date_to: chrono::NaiveDate,
    ) -> Vec<JournalEntry> {
        let query = r#"SELECT * FROM journal_entries
            WHERE bukrs = $1 AND status = 2 AND xloeb = false
            AND bldat >= $2 AND bldat <= $3
            ORDER BY belnr"#;
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(posting_date_from)
            .bind(posting_date_to)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_journal_entry).collect()
    }

    async fn save(&self, entry: &JournalEntry) -> Result<(), String> {
        let query = r#"
            INSERT INTO journal_entries (
                bukrs, belnr, gjahr, blart, bldat, budat, waers, stblg, xblnr, bktxt,
                sumde, sumcr, status, xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (bukrs, belnr, gjahr) DO UPDATE SET
                bldat = EXCLUDED.bldat,
                budat = EXCLUDED.budat,
                waers = EXCLUDED.waers,
                bktxt = EXCLUDED.bktxt,
                status = EXCLUDED.status,
                sumde = EXCLUDED.sumde,
                sumcr = EXCLUDED.sumcr,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit_info = &entry.document.audit_info();

        sqlx::query(query)
            .bind(entry.company_code().as_str())
            .bind(entry.document_number())
            .bind(entry.fiscal_year())
            .bind(entry.document_type().to_string())
            .bind(entry.document_date())
            .bind(entry.posting_date())
            .bind(entry.currency())
            .bind("")
            .bind(entry.reference_document().unwrap_or(""))
            .bind(entry.header_text().unwrap_or(""))
            .bind(entry.total_debit().amount().to_string())
            .bind(entry.total_credit().amount().to_string())
            .bind(entry.status() as i32)
            .bind(false)
            .bind(audit_info.created_at().date_naive())
            .bind(audit_info.created_by())
            .bind(now)
            .bind("SYSTEM")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // 保存行项目
        self.save_journal_entry_items(entry).await?;

        Ok(())
    }

    async fn delete(&self, company_code: &CompanyCode, document_number: &DocumentNumber, fiscal_year: &str) -> Result<(), String> {
        let query = "UPDATE journal_entries SET xloeb = true, status = 5 WHERE bukrs = $1 AND belnr = $2 AND gjahr = $3";
        sqlx::query(query)
            .bind(company_code.as_str())
            .bind(document_number.as_str())
            .bind(fiscal_year)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl PostgresRepository {
    async fn save_journal_entry_items(&self, entry: &JournalEntry) -> Result<(), String> {
        // 删除现有行项目
        let delete_query = "DELETE FROM journal_entry_items WHERE bukrs = $1 AND belnr = $2 AND gjahr = $3";
        sqlx::query(delete_query)
            .bind(entry.company_code().as_str())
            .bind(entry.document_number())
            .bind(entry.fiscal_year())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // 插入新行项目
        let insert_query = r#"
            INSERT INTO journal_entry_items (bukrs, belnr, gjahr, buzei, saknr, koart, shkzg, wrbtr, mwsts, dmbtr, kostl, prctr, aufnr, xloeb)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#;

        for item in entry.items() {
            sqlx::query(insert_query)
                .bind(entry.company_code().as_str())
                .bind(entry.document_number())
                .bind(entry.fiscal_year())
                .bind(item.line_number())
                .bind(item.account_code().as_str())
                .bind(item.account_type())
                .bind(item.debit_credit_indicator().to_string())
                .bind(item.document_currency_amount().amount().to_string())
                .bind(item.tax_amount().amount().to_string())
                .bind(item.local_currency_amount().amount().to_string())
                .bind(item.cost_center().unwrap_or(""))
                .bind(item.profit_center().unwrap_or(""))
                .bind(item.order_number().unwrap_or(""))
                .bind(false)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

// Customer Repository 实现
#[async_trait]
impl CustomerRepository for PostgresRepository {
    async fn find_by_id(&self, company_code: &CompanyCode, customer_id: &str) -> Option<Customer> {
        let query = "SELECT * FROM customers WHERE bukrs = $1 AND kunnr = $2 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(customer_id)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_customer(&row))
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Customer> {
        let query = "SELECT * FROM customers WHERE bukrs = $1 AND xloeb = false ORDER BY kunnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_customer).collect()
    }

    async fn find_by_status(&self, company_code: &CompanyCode, status: i32) -> Vec<Customer> {
        let query = "SELECT * FROM customers WHERE bukrs = $1 AND status = $2 AND xloeb = false ORDER BY kunnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(status)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_customer).collect()
    }

    async fn find_by_account_group(&self, company_code: &CompanyCode, account_group: &str) -> Vec<Customer> {
        let query = "SELECT * FROM customers WHERE bukrs = $1 AND ktokd = $2 AND xloeb = false ORDER BY kunnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(account_group)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_customer).collect()
    }

    async fn save(&self, customer: &Customer) -> Result<(), String> {
        let query = r#"
            INSERT INTO customers (
                kunnr, bukrs, stcd1, stcd2, ktokd, vwerk, waers, konto, zterm, zlsch,
                mahna, parnr, name1, name2, street, city, land1, tel_number, email,
                status, xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            ON CONFLICT (kunnr, bukrs) DO UPDATE SET
                stcd1 = EXCLUDED.stcd1,
                name1 = EXCLUDED.name1,
                street = EXCLUDED.street,
                city = EXCLUDED.city,
                waers = EXCLUDED.waers,
                konto = EXCLUDED.konto,
                zterm = EXCLUDED.zterm,
                status = EXCLUDED.status,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit_info = &customer.audit_info;

        sqlx::query(query)
            .bind(customer.customer_id())
            .bind(customer.company_code().as_str())
            .bind(customer.tax_number_1().unwrap_or(""))
            .bind(customer.tax_number_2().unwrap_or(""))
            .bind(customer.account_group())
            .bind(customer.sold_to_party().unwrap_or(""))
            .bind(customer.currency())
            .bind(customer.reconciliation_account())
            .bind(customer.payment_terms())
            .bind(customer.payment_methods().unwrap_or(""))
            .bind(customer.dunning_area().unwrap_or(""))
            .bind(customer.customer_representative().unwrap_or(""))
            .bind(customer.name_1())
            .bind(customer.name_2().unwrap_or(""))
            .bind(customer.street().unwrap_or(""))
            .bind(customer.city().unwrap_or(""))
            .bind(customer.country())
            .bind(customer.phone_number().unwrap_or(""))
            .bind(customer.email_address().unwrap_or(""))
            .bind(customer.status() as i32)
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

    async fn delete(&self, company_code: &CompanyCode, customer_id: &str) -> Result<(), String> {
        let query = "UPDATE customers SET xloeb = true, status = 3 WHERE bukrs = $1 AND kunnr = $2";
        sqlx::query(query)
            .bind(company_code.as_str())
            .bind(customer_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// Vendor Repository 实现
#[async_trait]
impl VendorRepository for PostgresRepository {
    async fn find_by_id(&self, company_code: &CompanyCode, vendor_id: &str) -> Option<Vendor> {
        let query = "SELECT * FROM vendors WHERE bukrs = $1 AND lifnr = $2 AND xloeb = false";
        let row = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(vendor_id)
            .fetch_optional(&self.pool)
            .await
            .ok()??;

        Some(row_to_vendor(&row))
    }

    async fn find_all(&self, company_code: &CompanyCode) -> Vec<Vendor> {
        let query = "SELECT * FROM vendors WHERE bukrs = $1 AND xloeb = false ORDER BY lifnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_vendor).collect()
    }

    async fn find_by_status(&self, company_code: &CompanyCode, status: i32) -> Vec<Vendor> {
        let query = "SELECT * FROM vendors WHERE bukrs = $1 AND status = $2 AND xloeb = false ORDER BY lifnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(status)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_vendor).collect()
    }

    async fn find_by_account_group(&self, company_code: &CompanyCode, account_group: &str) -> Vec<Vendor> {
        let query = "SELECT * FROM vendors WHERE bukrs = $1 AND ktokk = $2 AND xloeb = false ORDER BY lifnr";
        let rows = sqlx::query(query)
            .bind(company_code.as_str())
            .bind(account_group)
            .fetch_all(&self.pool)
            .await
            .ok()?;

        rows.into_iter().map(row_to_vendor).collect()
    }

    async fn save(&self, vendor: &Vendor) -> Result<(), String> {
        let query = r#"
            INSERT INTO vendors (
                lifnr, bukrs, stcd1, stcd2, ktokk, waers, konto, zterm, zlsch, payer,
                pargr, verkr, name1, name2, street, city, land1, tel_number, email,
                status, xloeb, erdat, ernam, aedat, aenam
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            ON CONFLICT (lifnr, bukrs) DO UPDATE SET
                stcd1 = EXCLUDED.stcd1,
                name1 = EXCLUDED.name1,
                street = EXCLUDED.street,
                city = EXCLUDED.city,
                waers = EXCLUDED.waers,
                konto = EXCLUDED.konto,
                zterm = EXCLUDED.zterm,
                status = EXCLUDED.status,
                aedat = EXCLUDED.aedat,
                aenam = EXCLUDED.aenam
        "#;

        let now = chrono::Utc::now().date_naive();
        let audit_info = &vendor.audit_info;

        sqlx::query(query)
            .bind(vendor.vendor_id())
            .bind(vendor.company_code().as_str())
            .bind(vendor.tax_number_1().unwrap_or(""))
            .bind(vendor.tax_number_2().unwrap_or(""))
            .bind(vendor.account_group())
            .bind(vendor.currency())
            .bind(vendor.reconciliation_account())
            .bind(vendor.payment_terms())
            .bind(vendor.payment_methods().unwrap_or(""))
            .bind(vendor.payer().unwrap_or(""))
            .bind(vendor.partner_role().unwrap_or(""))
            .bind(vendor.vendor_representative().unwrap_or(""))
            .bind(vendor.name_1())
            .bind(vendor.name_2().unwrap_or(""))
            .bind(vendor.street().unwrap_or(""))
            .bind(vendor.city().unwrap_or(""))
            .bind(vendor.country())
            .bind(vendor.phone_number().unwrap_or(""))
            .bind(vendor.email_address().unwrap_or(""))
            .bind(vendor.status() as i32)
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

    async fn delete(&self, company_code: &CompanyCode, vendor_id: &str) -> Result<(), String> {
        let query = "UPDATE vendors SET xloeb = true, status = 3 WHERE bukrs = $1 AND lifnr = $2";
        sqlx::query(query)
            .bind(company_code.as_str())
            .bind(vendor_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
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

fn row_to_journal_entry(_row: &sqlx::postgres::PgRow) -> JournalEntry {
    // 由于 JournalEntry 结构复杂，这里返回简化版本
    // 实际实现需要加载行项目
    JournalEntry::new(
        crate::domain::entities::document::DocumentType::StandardDocument,
        crate::domain::value_objects::document_number::DocumentNumber::new("0000000000").unwrap(),
        "2024".to_string(),
        CompanyCode::new("1000").unwrap(),
        chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        "CNY",
        "SYSTEM",
    )
}

fn row_to_customer(row: &sqlx::postgres::PgRow) -> Customer {
    let mut customer = Customer::new(
        row.try_get::<String, _>("kunnr").unwrap_or_default(),
        CompanyCode::new(row.try_get::<String, _>("bukrs").unwrap_or_default()).unwrap_or_default(),
        row.try_get::<String, _>("ktokd").unwrap_or_default(),
        row.try_get::<String, _>("name1").unwrap_or_default(),
        row.try_get::<String, _>("land1").unwrap_or_default(),
        row.try_get::<String, _>("waers").unwrap_or_default(),
    );

    // 设置税号
    if let Ok(stcd1) = row.try_get::<String, _>("stcd1") {
        if !stcd1.is_empty() {
            customer.set_tax_number(stcd1, "SYSTEM");
        }
    }

    customer
}

fn row_to_vendor(row: &sqlx::postgres::PgRow) -> Vendor {
    let mut vendor = Vendor::new(
        row.try_get::<String, _>("lifnr").unwrap_or_default(),
        CompanyCode::new(row.try_get::<String, _>("bukrs").unwrap_or_default()).unwrap_or_default(),
        row.try_get::<String, _>("ktokk").unwrap_or_default(),
        row.try_get::<String, _>("name1").unwrap_or_default(),
        row.try_get::<String, _>("land1").unwrap_or_default(),
        row.try_get::<String, _>("waers").unwrap_or_default(),
    );

    // 设置税号
    if let Ok(stcd1) = row.try_get::<String, _>("stcd1") {
        if !stcd1.is_empty() {
            vendor.set_tax_number(stcd1, "SYSTEM");
        }
    }

    vendor
}
