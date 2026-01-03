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
