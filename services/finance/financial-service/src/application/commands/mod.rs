//! 命令处理器模块

use async_trait::async_trait;
use crate::domain::repositories::{
    GLAccountRepository,
    JournalEntryRepository,
    CustomerRepository,
    VendorRepository,
    FixedAssetRepository,
    BankAccountRepository,
};
use crate::application::dto::*;
use crate::domain::aggregates::*;
use crate::domain::value_objects::*;
use crate::domain::entities::*;
use killer_domain_primitives::{CompanyCode, DocumentNumber, Money};

/// 创建总账科目命令处理器
#[async_trait]
pub struct CreateGLAccountHandler<R: GLAccountRepository> {
    repository: R,
}

impl<R: GLAccountRepository> CreateGLAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateGLAccountDto) -> Result<GLAccount, String> {
        // 验证科目代码
        let account_code = AccountCode::new(dto.account_code)
            .map_err(|e| e.to_string())?;

        // 创建聚合根
        let mut account = GLAccount::new(
            dto.chart_of_accounts,
            account_code,
            dto.company_code,
            dto.account_type,
            dto.balance_sheet_indicator,
            dto.currency,
            dto.description,
        );

        // 设置可选字段
        if let Some(cost_control_area) = dto.cost_control_area {
            account.set_cost_control_area(cost_control_area, "SYSTEM");
        }
        if let Some(account_group) = dto.account_group {
            account.set_account_group(account_group, "SYSTEM");
        }

        // 保存
        self.repository.save(&account).await?;

        Ok(account)
    }
}

/// 创建会计凭证命令处理器
#[async_trait]
pub struct CreateJournalEntryHandler<R: JournalEntryRepository> {
    repository: R,
}

impl<R: JournalEntryRepository> CreateJournalEntryHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateJournalEntryDto) -> Result<JournalEntry, String> {
        // 验证日期
        if dto.posting_date < dto.document_date {
            return Err("过账日期不能早于凭证日期".to_string());
        }

        // 生成凭证号（实际业务中可能需要从编号范围获取）
        let document_number = DocumentNumber::from_number(
            chrono::Local::now().timestamp() as u32 % 10000000000u32,
        );

        // 确定凭证类型
        let doc_type = match dto.document_type.as_str() {
            "SA" => DocumentType::StandardDocument,
            "KR" => DocumentType::InvoiceReceipt,
            "KG" => DocumentType::PaymentDocument,
            "AA" => DocumentType::AdjustmentDocument,
            _ => DocumentType::StandardDocument,
        };

        // 创建凭证
        let mut entry = JournalEntry::new(
            doc_type,
            document_number,
            dto.posting_date.year().to_string(),
            dto.company_code,
            dto.document_date,
            dto.posting_date,
            dto.currency,
            "SYSTEM",
        );

        // 设置可选字段
        if let Some(ref_text) = dto.reference_document {
            entry.set_reference(ref_text);
        }
        if let Some(text) = dto.header_text {
            entry.set_header_text(text);
        }

        // 添加行项目
        for item_dto in dto.items {
            let account_code = AccountCode::new(item_dto.gl_account)
                .map_err(|e| e.to_string())?;

            let debit_credit = DebitCreditIndicator::try_from(item_dto.debit_credit)
                .map_err(|e| e.to_string())?;

            let amount = Money::new(item_dto.amount, &item_dto.currency)
                .map_err(|e| e.to_string())?;

            let item = JournalEntryItem::new(
                0, // 内部生成行号
                account_code,
                debit_credit,
                amount.clone(),
                amount,
            )
            .map_err(|e| e.to_string())?
            .with_customer(item_dto.customer_id.unwrap_or_default())
            .with_vendor(item_dto.vendor_id.unwrap_or_default())
            .with_cost_center(item_dto.cost_center.unwrap_or_default())
            .with_profit_center(item_dto.profit_center.unwrap_or_default())
            .with_line_text(item_dto.line_text.unwrap_or_default())
            .with_assignment(item_dto.assignment.unwrap_or_default());

            entry.add_item(item)?;
        }

        // 验证借贷平衡
        if !entry.is_balanced() {
            return Err("借贷不平衡".to_string());
        }

        // 保存
        self.repository.save(&entry).await?;

        Ok(entry)
    }
}

/// 过账会计凭证命令处理器
#[async_trait]
pub struct PostJournalEntryHandler<R: JournalEntryRepository> {
    repository: R,
}

impl<R: JournalEntryRepository> PostJournalEntryHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: PostJournalEntryDto) -> Result<JournalEntry, String> {
        let doc_number = DocumentNumber::new(dto.document_number)
            .map_err(|e| e.to_string())?;

        // 查找凭证
        let mut entry = self.repository
            .find_by_id(&dto.company_code, &doc_number, &dto.fiscal_year)
            .await
            .ok_or("凭证不存在")?;

        // 过账
        entry.post().map_err(|e| e.to_string())?;

        // 保存
        self.repository.save(&entry).await?;

        Ok(entry)
    }
}

/// 冲销会计凭证命令处理器
#[async_trait]
pub struct ReverseJournalEntryHandler<R: JournalEntryRepository> {
    repository: R,
}

impl<R: JournalEntryRepository> ReverseJournalEntryHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: ReverseJournalEntryDto) -> Result<DocumentNumber, String> {
        let doc_number = DocumentNumber::new(dto.document_number)
            .map_err(|e| e.to_string())?;

        // 查找凭证
        let mut entry = self.repository
            .find_by_id(&dto.company_code, &doc_number, &dto.fiscal_year)
            .await
            .ok_or("凭证不存在")?;

        // 冲销
        let reversal_doc_number = entry.reverse(dto.reversal_date, &dto.reversal_reason)?;

        // 保存
        self.repository.save(&entry).await?;

        Ok(reversal_doc_number)
    }
}

/// 创建客户命令处理器
#[async_trait]
pub struct CreateCustomerHandler<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> CreateCustomerHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateCustomerDto) -> Result<Customer, String> {
        // 生成客户编号（实际业务中可能需要从编号范围获取）
        let customer_id = format!("C{:010}", chrono::Local::now().timestamp() as u32 % 10000000000u32);

        // 创建客户
        let mut customer = Customer::new(
            customer_id,
            dto.company_code,
            dto.account_group,
            dto.name_1,
            dto.country,
            dto.currency,
        );

        // 设置可选字段
        if let Some(tax_number) = dto.tax_number {
            customer.set_tax_number(tax_number, "SYSTEM");
        }
        customer.update_financial_info(
            None,
            dto.reconciliation_account,
            dto.payment_terms,
            dto.payment_methods,
            "SYSTEM",
        );
        customer.set_contact(dto.phone_number, dto.email_address, "SYSTEM");

        // 保存
        self.repository.save(&customer).await?;

        Ok(customer)
    }
}

/// 更新客户命令处理器
#[async_trait]
pub struct UpdateCustomerHandler<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> UpdateCustomerHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: UpdateCustomerDto) -> Result<Customer, String> {
        // 查找客户
        let mut customer = self.repository
            .find_by_id(&dto.company_code, &dto.customer_id)
            .await
            .ok_or("客户不存在")?;

        // 更新
        customer.update_basic_info(
            dto.name_1.unwrap_or(customer.name_1().to_string()),
            dto.street,
            dto.city,
            dto.postal_code,
            dto.country,
            "SYSTEM",
        );

        if let Some(payment_terms) = dto.payment_terms {
            customer.update_financial_info(
                None,
                customer.reconciliation_account().to_string(),
                payment_terms,
                None,
                "SYSTEM",
            );
        }

        // 保存
        self.repository.save(&customer).await?;

        Ok(customer)
    }
}

/// 创建供应商命令处理器
#[async_trait]
pub struct CreateVendorHandler<R: VendorRepository> {
    repository: R,
}

impl<R: VendorRepository> CreateVendorHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateVendorDto) -> Result<Vendor, String> {
        // 生成供应商编号
        let vendor_id = format!("V{:010}", chrono::Local::now().timestamp() as u32 % 10000000000u32);

        // 创建供应商
        let mut vendor = Vendor::new(
            vendor_id,
            dto.company_code,
            dto.account_group,
            dto.name_1,
            dto.country,
            dto.currency,
        );

        // 设置可选字段
        if let Some(tax_number) = dto.tax_number {
            vendor.set_tax_number(tax_number, "SYSTEM");
        }
        vendor.update_financial_info(
            None,
            dto.reconciliation_account,
            dto.payment_terms,
            dto.payment_methods,
            "SYSTEM",
        );
        vendor.set_contact(dto.phone_number, dto.email_address, "SYSTEM");

        // 保存
        self.repository.save(&vendor).await?;

        Ok(vendor)
    }
}

/// 冻结客户命令处理器
#[async_trait]
pub struct BlockCustomerHandler<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> BlockCustomerHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, customer_id: &str) -> Result<(), String> {
        let mut customer = self.repository
            .find_by_id(company_code, customer_id)
            .await
            .ok_or("客户不存在")?;

        customer.block("SYSTEM");
        self.repository.save(&customer).await?;

        Ok(())
    }
}

/// 冻结供应商命令处理器
#[async_trait]
pub struct BlockVendorHandler<R: VendorRepository> {
    repository: R,
}

impl<R: VendorRepository> BlockVendorHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, vendor_id: &str) -> Result<(), String> {
        let mut vendor = self.repository
            .find_by_id(company_code, vendor_id)
            .await
            .ok_or("供应商不存在")?;

        vendor.block("SYSTEM");
        self.repository.save(&vendor).await?;

        Ok(())
    }
}

// =============================================================================
// 固定资产命令处理器
// =============================================================================

/// 创建固定资产命令处理器
#[async_trait]
pub struct CreateFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> CreateFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateFixedAssetDto) -> Result<FixedAsset, String> {
        // 生成资产编号
        let asset_number = format!("{:010}", chrono::Local::now().timestamp() as u32 % 10000000000u32);

        // 创建固定资产
        let mut asset = FixedAsset::new(
            dto.company_code,
            dto.asset_class,
            dto.valuation_class,
            dto.description,
        );

        asset.set_asset_number(asset_number);

        if let Some(cc) = dto.cost_center {
            asset.set_cost_center(cc);
        }
        if let Some(pc) = dto.profit_center {
            asset.set_profit_center(pc);
        }
        if let Some(loc) = dto.location {
            asset.set_location(loc);
        }

        // 保存
        self.repository.save(&asset).await?;

        Ok(asset)
    }
}

/// 固定资产资本化命令处理器
#[async_trait]
pub struct CapitalizeFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> CapitalizeFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CapitalizeFixedAssetDto) -> Result<FixedAsset, String> {
        // 查找资产
        let mut asset = self.repository
            .find_by_id(&dto.company_code, &dto.asset_number, &dto.sub_number)
            .await
            .ok_or("固定资产不存在")?;

        // 资本化
        let acquisition_value = Money::new(dto.acquisition_value, &dto.currency)
            .map_err(|e| e.to_string())?;

        asset.capitalize(dto.capitalization_date, acquisition_value);

        // 保存
        self.repository.save(&asset).await?;

        Ok(asset)
    }
}

/// 固定资产折旧命令处理器
#[async_trait]
pub struct DepreciateFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> DepreciateFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: DepreciateFixedAssetDto) -> Result<FixedAsset, String> {
        // 查找资产
        let mut asset = self.repository
            .find_by_id(&dto.company_code, &dto.asset_number, &dto.sub_number)
            .await
            .ok_or("固定资产不存在")?;

        // 折旧
        let depreciation_amount = Money::new(dto.depreciation_amount, &dto.currency)
            .map_err(|e| e.to_string())?;

        asset.depreciate(depreciation_amount);

        // 保存
        self.repository.save(&asset).await?;

        Ok(asset)
    }
}

/// 固定资产转移命令处理器
#[async_trait]
pub struct TransferFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> TransferFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: TransferFixedAssetDto) -> Result<FixedAsset, String> {
        // 查找资产
        let mut asset = self.repository
            .find_by_id(&dto.company_code, &dto.asset_number, &dto.sub_number)
            .await
            .ok_or("固定资产不存在")?;

        // 转移
        asset.transfer(dto.new_cost_center, dto.new_profit_center, dto.new_business_area);

        // 保存
        self.repository.save(&asset).await?;

        Ok(asset)
    }
}

/// 固定资产报废命令处理器
#[async_trait]
pub struct RetireFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> RetireFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: RetireFixedAssetDto) -> Result<(), String> {
        // 查找资产
        let mut asset = self.repository
            .find_by_id(&dto.company_code, &dto.asset_number, &dto.sub_number)
            .await
            .ok_or("固定资产不存在")?;

        // 报废
        let retirement_value = Money::from_str(&dto.retirement_value.to_string())
            .map_err(|e| e.to_string())?;

        asset.retire(retirement_value)?;

        // 保存
        self.repository.save(&asset).await?;

        Ok(())
    }
}

/// 冻结固定资产命令处理器
#[async_trait]
pub struct BlockFixedAssetHandler<R: FixedAssetRepository> {
    repository: R,
}

impl<R: FixedAssetRepository> BlockFixedAssetHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, company_code: &CompanyCode, asset_number: &str, sub_number: &str) -> Result<(), String> {
        let mut asset = self.repository
            .find_by_id(company_code, asset_number, sub_number)
            .await
            .ok_or("固定资产不存在")?;

        asset.block();
        self.repository.save(&asset).await?;

        Ok(())
    }
}

// =============================================================================
// 银行账户命令处理器
// =============================================================================

/// 创建银行账户命令处理器
#[async_trait]
pub struct CreateBankAccountHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> CreateBankAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: CreateBankAccountDto) -> Result<BankAccount, String> {
        // 创建银行账户
        let mut account = BankAccount::new(
            dto.bank_country_code,
            dto.bank_key,
            dto.bank_name,
        );

        // 更新地址
        account.update_address(
            dto.street_address,
            dto.city,
            dto.postal_code,
            "SYSTEM",
        );

        // 设置 SWIFT 和 IBAN
        if let Some(swift) = dto.swift_code {
            account.set_swift_code(swift, "SYSTEM");
        }
        if let Some(iban) = dto.iban {
            account.set_iban(iban, "SYSTEM");
        }
        if let Some(acc_num) = dto.bank_account_number {
            account.set_bank_account_number(acc_num, "SYSTEM");
        }

        // 保存
        self.repository.save(&account).await?;

        Ok(account)
    }
}

/// 银行账户存款命令处理器
#[async_trait]
pub struct DepositBankAccountHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> DepositBankAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: DepositBankAccountDto) -> Result<BankAccount, String> {
        // 查找账户
        let mut account = self.repository
            .find_by_id(&dto.bank_key, &dto.bank_account)
            .await
            .ok_or("银行账户不存在")?;

        // 存款
        let amount = Money::new(dto.amount, &dto.currency)
            .map_err(|e| e.to_string())?;

        account.deposit(amount);

        // 保存
        self.repository.save(&account).await?;

        Ok(account)
    }
}

/// 银行账户取款命令处理器
#[async_trait]
pub struct WithdrawBankAccountHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> WithdrawBankAccountHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: WithdrawBankAccountDto) -> Result<BankAccount, String> {
        // 查找账户
        let mut account = self.repository
            .find_by_id(&dto.bank_key, &dto.bank_account)
            .await
            .ok_or("银行账户不存在")?;

        // 取款
        let amount = Money::new(dto.amount, &dto.currency)
            .map_err(|e| e.to_string())?;

        account.withdraw(amount)?;

        // 保存
        self.repository.save(&account).await?;

        Ok(account)
    }
}

/// 银行账户更新余额命令处理器
#[async_trait]
pub struct UpdateBankAccountBalanceHandler<R: BankAccountRepository> {
    repository: R,
}

impl<R: BankAccountRepository> UpdateBankAccountBalanceHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, dto: UpdateBankAccountBalanceDto) -> Result<(), String> {
        let new_balance = Money::new(dto.new_balance, &dto.currency)
            .map_err(|e| e.to_string())?;

        self.repository.update_balance(&dto.bank_key, &dto.bank_account, new_balance).await?;

        Ok(())
    }
}
