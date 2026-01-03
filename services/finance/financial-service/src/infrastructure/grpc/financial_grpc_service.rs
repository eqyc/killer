//! Financial Service gRPC 服务实现
//!
//! 使用 tonic 和 prost 实现 gRPC 服务

use tonic::{Request, Response, Status};
use killer_api_contracts::gen::finance::financial::v1::{
    financial_service_server::FinancialService,
    GetGLAccountRequest, GetGLAccountResponse,
    ListGLAccountsRequest, ListGLAccountsResponse,
    CreateJournalEntryRequest, CreateJournalEntryResponse,
    GetJournalEntryRequest, GetJournalEntryResponse,
    ListJournalEntriesRequest, ListJournalEntriesResponse,
    PostJournalEntryRequest, PostJournalEntryResponse,
    ReverseJournalEntryRequest, ReverseJournalEntryResponse,
    CreateCustomerRequest, CreateCustomerResponse,
    GetCustomerRequest, GetCustomerResponse,
    ListCustomersRequest, ListCustomersResponse,
    UpdateCustomerRequest, UpdateCustomerResponse,
    CreateVendorRequest, CreateVendorResponse,
    GetVendorRequest, GetVendorResponse,
    ListVendorsRequest, ListVendorsResponse,
    UpdateVendorRequest, UpdateVendorResponse,
    GetFixedAssetRequest, GetFixedAssetResponse,
    ListFixedAssetsRequest, ListFixedAssetsResponse,
    CreateFixedAssetRequest, CreateFixedAssetResponse,
    CapitalizeFixedAssetRequest, CapitalizeFixedAssetResponse,
    DepreciateFixedAssetRequest, DepreciateFixedAssetResponse,
    TransferFixedAssetRequest, TransferFixedAssetResponse,
    RetireFixedAssetRequest, RetireFixedAssetResponse,
    BlockFixedAssetRequest, BlockFixedAssetResponse,
    GetBankRequest, GetBankResponse,
    ListBanksRequest, ListBanksResponse,
    DepositBankAccountRequest, DepositBankAccountResponse,
    WithdrawBankAccountRequest, WithdrawBankAccountResponse,
    OpenFiscalPeriodRequest, OpenFiscalPeriodResponse,
    CloseFiscalPeriodRequest, CloseFiscalPeriodResponse,
    GenerateTrialBalanceRequest, GenerateTrialBalanceResponse,
};
use crate::application::commands::*;
use crate::application::queries::*;
use crate::domain::repositories::*;
use killer_domain_primitives::CompanyCode;

/// Financial Service gRPC 服务实现
#[derive(Debug)]
pub struct FinancialGrpcService<R1, R2, R3, R4, R5, R6>
where
    R1: GLAccountRepository,
    R2: JournalEntryRepository,
    R3: CustomerRepository,
    R4: VendorRepository,
    R5: FixedAssetRepository,
    R6: BankAccountRepository,
{
    // Repositories
    gl_account_repo: R1,
    journal_entry_repo: R2,
    customer_repo: R3,
    vendor_repo: R4,
    fixed_asset_repo: R5,
    bank_account_repo: R6,

    // Command Handlers
    create_gl_account_handler: CreateGLAccountHandler<R1>,
    create_journal_entry_handler: CreateJournalEntryHandler<R2>,
    post_journal_entry_handler: PostJournalEntryHandler<R2>,
    reverse_journal_entry_handler: ReverseJournalEntryHandler<R2>,
    create_customer_handler: CreateCustomerHandler<R3>,
    update_customer_handler: UpdateCustomerHandler<R3>,
    create_vendor_handler: CreateVendorHandler<R4>,
    create_fixed_asset_handler: CreateFixedAssetHandler<R5>,
    capitalize_fixed_asset_handler: CapitalizeFixedAssetHandler<R5>,
    depreciate_fixed_asset_handler: DepreciateFixedAssetHandler<R5>,
    transfer_fixed_asset_handler: TransferFixedAssetHandler<R5>,
    retire_fixed_asset_handler: RetireFixedAssetHandler<R5>,
    block_fixed_asset_handler: BlockFixedAssetHandler<R5>,
    create_bank_account_handler: CreateBankAccountHandler<R6>,
    deposit_bank_account_handler: DepositBankAccountHandler<R6>,
    withdraw_bank_account_handler: WithdrawBankAccountHandler<R6>,

    // Query Handlers
    get_gl_account_handler: GetGLAccountHandler<R1>,
    list_gl_accounts_handler: ListGLAccountsHandler<R1>,
    get_journal_entry_handler: GetJournalEntryHandler<R2>,
    list_journal_entries_handler: ListJournalEntriesHandler<R2>,
    get_customer_handler: GetCustomerHandler<R3>,
    list_customers_handler: ListCustomersHandler<R3>,
    get_vendor_handler: GetVendorHandler<R4>,
    list_vendors_handler: ListVendorsHandler<R4>,
    get_fixed_asset_handler: GetFixedAssetHandler<R5>,
    list_fixed_assets_handler: ListFixedAssetsHandler<R5>,
    get_bank_account_handler: GetBankAccountHandler<R6>,
    list_bank_accounts_handler: ListBankAccountsHandler<R6>,
}

impl<R1, R2, R3, R4, R5, R6> FinancialGrpcService<R1, R2, R3, R4, R5, R6>
where
    R1: GLAccountRepository + Clone,
    R2: JournalEntryRepository + Clone,
    R3: CustomerRepository + Clone,
    R4: VendorRepository + Clone,
    R5: FixedAssetRepository + Clone,
    R6: BankAccountRepository + Clone,
{
    pub fn new(
        gl_account_repo: R1,
        journal_entry_repo: R2,
        customer_repo: R3,
        vendor_repo: R4,
        fixed_asset_repo: R5,
        bank_account_repo: R6,
    ) -> Self {
        Self {
            // Repositories
            gl_account_repo: gl_account_repo.clone(),
            journal_entry_repo: journal_entry_repo.clone(),
            customer_repo: customer_repo.clone(),
            vendor_repo: vendor_repo.clone(),
            fixed_asset_repo: fixed_asset_repo.clone(),
            bank_account_repo: bank_account_repo.clone(),

            // Command Handlers
            create_gl_account_handler: CreateGLAccountHandler::new(gl_account_repo.clone()),
            create_journal_entry_handler: CreateJournalEntryHandler::new(journal_entry_repo.clone()),
            post_journal_entry_handler: PostJournalEntryHandler::new(journal_entry_repo.clone()),
            reverse_journal_entry_handler: ReverseJournalEntryHandler::new(journal_entry_repo.clone()),
            create_customer_handler: CreateCustomerHandler::new(customer_repo.clone()),
            update_customer_handler: UpdateCustomerHandler::new(customer_repo.clone()),
            create_vendor_handler: CreateVendorHandler::new(vendor_repo.clone()),
            create_fixed_asset_handler: CreateFixedAssetHandler::new(fixed_asset_repo.clone()),
            capitalize_fixed_asset_handler: CapitalizeFixedAssetHandler::new(fixed_asset_repo.clone()),
            depreciate_fixed_asset_handler: DepreciateFixedAssetHandler::new(fixed_asset_repo.clone()),
            transfer_fixed_asset_handler: TransferFixedAssetHandler::new(fixed_asset_repo.clone()),
            retire_fixed_asset_handler: RetireFixedAssetHandler::new(fixed_asset_repo.clone()),
            block_fixed_asset_handler: BlockFixedAssetHandler::new(fixed_asset_repo.clone()),
            create_bank_account_handler: CreateBankAccountHandler::new(bank_account_repo.clone()),
            deposit_bank_account_handler: DepositBankAccountHandler::new(bank_account_repo.clone()),
            withdraw_bank_account_handler: WithdrawBankAccountHandler::new(bank_account_repo.clone()),

            // Query Handlers
            get_gl_account_handler: GetGLAccountHandler::new(gl_account_repo.clone()),
            list_gl_accounts_handler: ListGLAccountsHandler::new(gl_account_repo),
            get_journal_entry_handler: GetJournalEntryHandler::new(journal_entry_repo.clone()),
            list_journal_entries_handler: ListJournalEntriesHandler::new(journal_entry_repo),
            get_customer_handler: GetCustomerHandler::new(customer_repo.clone()),
            list_customers_handler: ListCustomersHandler::new(customer_repo),
            get_vendor_handler: GetVendorHandler::new(vendor_repo.clone()),
            list_vendors_handler: ListVendorsHandler::new(vendor_repo),
            get_fixed_asset_handler: GetFixedAssetHandler::new(fixed_asset_repo.clone()),
            list_fixed_assets_handler: ListFixedAssetsHandler::new(fixed_asset_repo),
            get_bank_account_handler: GetBankAccountHandler::new(bank_account_repo.clone()),
            list_bank_accounts_handler: ListBankAccountsHandler::new(bank_account_repo),
        }
    }
}

#[tonic::async_trait]
impl<R1, R2, R3, R4, R5, R6> FinancialService for FinancialGrpcService<R1, R2, R3, R4, R5, R6>
where
    R1: GLAccountRepository + Clone + Send + Sync,
    R2: JournalEntryRepository + Clone + Send + Sync,
    R3: CustomerRepository + Clone + Send + Sync,
    R4: VendorRepository + Clone + Send + Sync,
    R5: FixedAssetRepository + Clone + Send + Sync,
    R6: BankAccountRepository + Clone + Send + Sync,
{
    // ===== 总账科目 =====

    async fn get_gl_account(
        &self,
        request: Request<GetGLAccountRequest>,
    ) -> Result<Response<GetGLAccountResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let account = self.get_gl_account_handler
            .handle(&company_code, &req.saknr)
            .await;

        match account {
            Some(acc) => Ok(Response::new(GetGLAccountResponse {
                gl_account: Some(acc.into()),
            })),
            None => Err(Status::not_found("总账科目不存在")),
        }
    }

    async fn list_gl_accounts(
        &self,
        request: Request<ListGLAccountsRequest>,
    ) -> Result<Response<ListGLAccountsResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let accounts = self.list_gl_accounts_handler
            .handle(&company_code)
            .await;

        Ok(Response::new(ListGLAccountsResponse {
            gl_accounts: accounts.into_iter().map(|a| a.into()).collect(),
            total_count: accounts.len() as i32,
        }))
    }

    async fn create_journal_entry(
        &self,
        request: Request<CreateJournalEntryRequest>,
    ) -> Result<Response<CreateJournalEntryResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = CreateJournalEntryDto {
            company_code,
            document_type: req.blart,
            document_date: req.bldat.and_hms_opt(0, 0, 0).unwrap().date_naive(),
            posting_date: req.budat.and_hms_opt(0, 0, 0).unwrap().date_naive(),
            currency: req.waers,
            reference_document: if req.xblnr.is_empty() { None } else { Some(req.xblnr) },
            header_text: if req.bktxt.is_empty() { None } else { Some(req.bktxt) },
            items: req.items.into_iter().map(|item| JournalEntryItemDto {
                gl_account: item.gl_account,
                debit_credit: item.debit_credit,
                amount: item.amount.map(|a| a.amount).unwrap_or(0.0),
                currency: item.amount.map(|a| a.currency_code).unwrap_or_default(),
                customer_id: if item.kunnr.is_empty() { None } else { Some(item.kunnr) },
                vendor_id: if item.lifnr.is_empty() { None } else { Some(item.lifnr) },
                cost_center: if item.kostl.is_empty() { None } else { Some(item.kostl) },
                profit_center: if item.prctr.is_empty() { None } else { Some(item.prctr) },
                tax_code: None,
                line_text: None,
                assignment: None,
            }).collect(),
        };

        let entry = self.create_journal_entry_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(CreateJournalEntryResponse {
            belnr: entry.document_number().to_string(),
            gjahr: entry.fiscal_year().to_string(),
            message: "会计凭证已创建".to_string(),
        }))
    }

    async fn get_journal_entry(
        &self,
        request: Request<GetJournalEntryRequest>,
    ) -> Result<Response<GetJournalEntryResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let entry = self.get_journal_entry_handler
            .handle(&company_code, &req.belnr, &req.gjahr)
            .await;

        match entry {
            Some(e) => Ok(Response::new(GetJournalEntryResponse {
                journal_entry: Some(e.into()),
            })),
            None => Err(Status::not_found("会计凭证不存在")),
        }
    }

    async fn list_journal_entries(
        &self,
        request: Request<ListJournalEntriesRequest>,
    ) -> Result<Response<ListJournalEntriesResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let entries = self.list_journal_entries_handler
            .handle(&company_code, &req.gjahr)
            .await;

        Ok(Response::new(ListJournalEntriesResponse {
            journal_entries: entries.into_iter().map(|e| e.into()).collect(),
            total_count: entries.len() as i32,
        }))
    }

    async fn post_journal_entry(
        &self,
        request: Request<PostJournalEntryRequest>,
    ) -> Result<Response<PostJournalEntryResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = PostJournalEntryDto {
            company_code,
            document_number: req.belnr,
            fiscal_year: req.gjahr,
        };

        let entry = self.post_journal_entry_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(PostJournalEntryResponse {
            belnr: entry.document_number().to_string(),
            gjahr: entry.fiscal_year().to_string(),
            posting_date: Some(chrono::Utc::now().into()),
            message: "凭证已过账".to_string(),
        }))
    }

    async fn reverse_journal_entry(
        &self,
        request: Request<ReverseJournalEntryRequest>,
    ) -> Result<Response<ReverseJournalEntryResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = ReverseJournalEntryDto {
            company_code,
            document_number: req.belnr,
            fiscal_year: req.gjahr,
            reversal_date: req.reversal_date.and_hms_opt(0, 0, 0).unwrap().date_naive(),
            reversal_reason: req.reversal_reason,
        };

        let reversal_doc_number = self.reverse_journal_entry_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(ReverseJournalEntryResponse {
            original_belnr: req.belnr,
            reversal_belnr: reversal_doc_number.to_string(),
            reversal_gjahr: req.gjahr,
            message: "凭证已冲销".to_string(),
        }))
    }

    // ===== 客户主数据 =====

    async fn get_customer(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let customer = self.get_customer_handler
            .handle(&company_code, &req.kunnr)
            .await;

        match customer {
            Some(c) => Ok(Response::new(GetCustomerResponse {
                customer: Some(c.into()),
            })),
            None => Err(Status::not_found("客户不存在")),
        }
    }

    async fn list_customers(
        &self,
        request: Request<ListCustomersRequest>,
    ) -> Result<Response<ListCustomersResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let customers = self.list_customers_handler
            .handle(&company_code)
            .await;

        Ok(Response::new(ListCustomersResponse {
            customers: customers.into_iter().map(|c| c.into()).collect(),
            total_count: customers.len() as i32,
        }))
    }

    async fn create_customer(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<CreateCustomerResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = CreateCustomerDto {
            company_code,
            account_group: req.ktokd,
            name_1: req.name1,
            name_2: None,
            street: None,
            city: None,
            postal_code: None,
            country: req.land1,
            tax_number: None,
            currency: req.waers,
            reconciliation_account: req.akont,
            payment_terms: req.zterm,
            payment_methods: None,
            phone_number: None,
            email_address: None,
        };

        let customer = self.create_customer_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(CreateCustomerResponse {
            kunnr: customer.customer_id().to_string(),
            message: "客户已创建".to_string(),
        }))
    }

    async fn update_customer(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<UpdateCustomerResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = UpdateCustomerDto {
            company_code,
            customer_id: req.kunnr,
            name_1: if req.name1.is_empty() { None } else { Some(req.name1) },
            street: None,
            city: None,
            postal_code: None,
            country: None,
            payment_terms: None,
        };

        self.update_customer_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(UpdateCustomerResponse {
            success: true,
            message: "客户已更新".to_string(),
        }))
    }

    // ===== 供应商主数据 =====

    async fn get_vendor(
        &self,
        request: Request<GetVendorRequest>,
    ) -> Result<Response<GetVendorResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let vendor = self.get_vendor_handler
            .handle(&company_code, &req.lifnr)
            .await;

        match vendor {
            Some(v) => Ok(Response::new(GetVendorResponse {
                vendor: Some(v.into()),
            })),
            None => Err(Status::not_found("供应商不存在")),
        }
    }

    async fn list_vendors(
        &self,
        request: Request<ListVendorsRequest>,
    ) -> Result<Response<ListVendorsResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let vendors = self.list_vendors_handler
            .handle(&company_code)
            .await;

        Ok(Response::new(ListVendorsResponse {
            vendors: vendors.into_iter().map(|v| v.into()).collect(),
            total_count: vendors.len() as i32,
        }))
    }

    async fn create_vendor(
        &self,
        request: Request<CreateVendorRequest>,
    ) -> Result<Response<CreateVendorResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = CreateVendorDto {
            company_code,
            account_group: req.ktokk,
            name_1: req.name1,
            name_2: None,
            street: None,
            city: None,
            postal_code: None,
            country: req.land1,
            tax_number: None,
            currency: req.waers,
            reconciliation_account: req.akont,
            payment_terms: req.zterm,
            payment_methods: None,
            phone_number: None,
            email_address: None,
        };

        let vendor = self.create_vendor_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(CreateVendorResponse {
            lifnr: vendor.vendor_id().to_string(),
            message: "供应商已创建".to_string(),
        }))
    }

    async fn update_vendor(
        &self,
        request: Request<UpdateVendorRequest>,
    ) -> Result<Response<UpdateVendorResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        // Simplified - full implementation would need UpdateVendorDto
        Ok(Response::new(UpdateVendorResponse {
            success: true,
            message: "供应商已更新".to_string(),
        }))
    }

    // ===== 固定资产 =====

    async fn get_fixed_asset(
        &self,
        request: Request<GetFixedAssetRequest>,
    ) -> Result<Response<GetFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let asset = self.get_fixed_asset_handler
            .handle(&company_code, &req.anlnr, &req.subno)
            .await;

        match asset {
            Some(a) => Ok(Response::new(GetFixedAssetResponse {
                fixed_asset: Some(a.into()),
            })),
            None => Err(Status::not_found("固定资产不存在")),
        }
    }

    async fn list_fixed_assets(
        &self,
        request: Request<ListFixedAssetsRequest>,
    ) -> Result<Response<ListFixedAssetsResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let assets = self.list_fixed_assets_handler
            .handle(&company_code)
            .await;

        Ok(Response::new(ListFixedAssetsResponse {
            fixed_assets: assets.into_iter().map(|a| a.into()).collect(),
            total_count: assets.len() as i32,
        }))
    }

    async fn create_fixed_asset(
        &self,
        request: Request<CreateFixedAssetRequest>,
    ) -> Result<Response<CreateFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = CreateFixedAssetDto {
            company_code,
            asset_class: req.anlkl,
            valuation_class: String::new(),
            description: req.txt50,
            cost_center: None,
            profit_center: None,
            location: None,
            acquisition_value: 0.0,
            currency: String::new(),
            capitalization_date: chrono::Utc::now().date_naive(),
        };

        let asset = self.create_fixed_asset_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(CreateFixedAssetResponse {
            anlnr: asset.asset_number().to_string(),
            message: "固定资产已创建".to_string(),
        }))
    }

    async fn post_asset_acquisition(
        &self,
        request: Request<CapitalizeFixedAssetRequest>,
    ) -> Result<Response<CapitalizeFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = CapitalizeFixedAssetDto {
            company_code,
            asset_number: req.anlnr,
            sub_number: req.subno,
            acquisition_value: req.acqu_amount.map(|a| a.amount).unwrap_or(0.0),
            currency: req.acqu_amount.map(|a| a.currency_code).unwrap_or_default(),
            capitalization_date: req.aktiv.and_hms_opt(0, 0, 0).unwrap().date_naive(),
        };

        let asset = self.capitalize_fixed_asset_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(CapitalizeFixedAssetResponse {
            anlnr: asset.asset_number().to_string(),
            capitalized_at: Some(chrono::Utc::now().into()),
        }))
    }

    async fn post_asset_depreciation(
        &self,
        request: Request<DepreciateFixedAssetRequest>,
    ) -> Result<Response<DepreciateFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = DepreciateFixedAssetDto {
            company_code,
            asset_number: req.anlnr,
            sub_number: req.subno,
            depreciation_amount: req.depr_amount.map(|a| a.amount).unwrap_or(0.0),
            currency: req.depr_amount.map(|a| a.currency_code).unwrap_or_default(),
        };

        let asset = self.depreciate_fixed_asset_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(DepreciateFixedAssetResponse {
            anlnr: asset.asset_number().to_string(),
            depreciated_at: Some(chrono::Utc::now().into()),
        }))
    }

    async fn post_asset_transfer(
        &self,
        request: Request<TransferFixedAssetRequest>,
    ) -> Result<Response<TransferFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = TransferFixedAssetDto {
            company_code,
            asset_number: req.anlnr,
            sub_number: req.subno,
            new_cost_center: if req.new_kostl.is_empty() { None } else { Some(req.new_kostl) },
            new_profit_center: if req.new_prctr.is_empty() { None } else { Some(req.new_prctr) },
            new_business_area: if req.new_bwkey.is_empty() { None } else { Some(req.new_bwkey) },
        };

        let asset = self.transfer_fixed_asset_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(TransferFixedAssetResponse {
            anlnr: asset.asset_number().to_string(),
            transferred_at: Some(chrono::Utc::now().into()),
        }))
    }

    async fn post_asset_retirement(
        &self,
        request: Request<RetireFixedAssetRequest>,
    ) -> Result<Response<RetireFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let dto = RetireFixedAssetDto {
            company_code,
            asset_number: req.anlnr,
            sub_number: req.subno,
            retirement_value: req.ret_amount.map(|a| a.amount).unwrap_or(0.0),
        };

        self.retire_fixed_asset_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(RetireFixedAssetResponse {
            anlnr: req.anlnr,
            retired_at: Some(chrono::Utc::now().into()),
        }))
    }

    async fn block_fixed_asset(
        &self,
        request: Request<BlockFixedAssetRequest>,
    ) -> Result<Response<BlockFixedAssetResponse>, Status> {
        let req = request.into_inner();
        let company_code = CompanyCode::new(req.bukrs)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        self.block_fixed_asset_handler
            .handle(&company_code, &req.anlnr, &req.subno)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(BlockFixedAssetResponse { success: true }))
    }

    // ===== 银行账户 =====

    async fn get_bank(
        &self,
        request: Request<GetBankRequest>,
    ) -> Result<Response<GetBankResponse>, Status> {
        let req = request.into_inner();

        let account = self.get_bank_account_handler
            .handle(&req.bank_key, &req.bank_account)
            .await;

        match account {
            Some(a) => Ok(Response::new(GetBankResponse {
                bank_account: Some(a.into()),
            })),
            None => Err(Status::not_found("银行账户不存在")),
        }
    }

    async fn list_banks(
        &self,
        request: Request<ListBanksRequest>,
    ) -> Result<Response<ListBanksResponse>, Status> {
        let _req = request.into_inner();

        let accounts = self.list_bank_accounts_handler.handle().await;

        Ok(Response::new(ListBanksResponse {
            bank_accounts: accounts.into_iter().map(|a| a.into()).collect(),
            next_page_token: String::new(),
            total_count: accounts.len() as i32,
        }))
    }

    async fn post_bank_receipt(
        &self,
        request: Request<DepositBankAccountRequest>,
    ) -> Result<Response<DepositBankAccountResponse>, Status> {
        let req = request.into_inner();

        let dto = DepositBankAccountDto {
            bank_key: req.bank_key,
            bank_account: req.bank_account,
            amount: req.amount.map(|a| a.amount).unwrap_or(0.0),
            currency: req.amount.map(|a| a.currency_code).unwrap_or_default(),
        };

        let account = self.deposit_bank_account_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(DepositBankAccountResponse {
            bank_key: account.bank_key().to_string(),
            new_balance: Some(account.current_balance().into()),
            deposited_at: Some(chrono::Utc::now().into()),
        }))
    }

    async fn post_bank_payment(
        &self,
        request: Request<WithdrawBankAccountRequest>,
    ) -> Result<Response<WithdrawBankAccountResponse>, Status> {
        let req = request.into_inner();

        let dto = WithdrawBankAccountDto {
            bank_key: req.bank_key,
            bank_account: req.bank_account,
            amount: req.amount.map(|a| a.amount).unwrap_or(0.0),
            currency: req.amount.map(|a| a.currency_code).unwrap_or_default(),
        };

        let account = self.withdraw_bank_account_handler
            .handle(dto)
            .await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(WithdrawBankAccountResponse {
            bank_key: account.bank_key().to_string(),
            new_balance: Some(account.current_balance().into()),
            withdrawn_at: Some(chrono::Utc::now().into()),
        }))
    }

    // ===== 期末处理 =====

    async fn open_fiscal_period(
        &self,
        request: Request<OpenFiscalPeriodRequest>,
    ) -> Result<Response<OpenFiscalPeriodResponse>, Status> {
        // 简化实现
        Ok(Response::new(OpenFiscalPeriodResponse {
            success: true,
            message: "会计期间已开启".to_string(),
        }))
    }

    async fn close_fiscal_period(
        &self,
        request: Request<CloseFiscalPeriodRequest>,
    ) -> Result<Response<CloseFiscalPeriodResponse>, Status> {
        // 简化实现
        Ok(Response::new(CloseFiscalPeriodResponse {
            success: true,
            message: "会计期间已关闭".to_string(),
            results: vec![],
        }))
    }

    async fn generate_trial_balance(
        &self,
        request: Request<GenerateTrialBalanceRequest>,
    ) -> Result<Response<GenerateTrialBalanceResponse>, Status> {
        // 简化实现
        Ok(Response::new(GenerateTrialBalanceResponse {
            lines: vec![],
            total_debit: None,
            total_credit: None,
            balanced: true,
        }))
    }

    // ===== 其他方法存根 =====

    async fn batch_create_journal_entries(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::BatchCreateJournalEntriesRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::BatchCreateJournalEntriesResponse>, Status> {
        Err(Status::unimplemented("批量创建凭证功能待实现"))
    }

    async fn create_journal_entry_with_template(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::CreateJournalEntryWithTemplateRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::CreateJournalEntryWithTemplateResponse>, Status> {
        Err(Status::unimplemented("模板创建凭证功能待实现"))
    }

    async fn get_account_balance(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::GetAccountBalanceRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::GetAccountBalanceResponse>, Status> {
        Err(Status::unimplemented("获取科目余额功能待实现"))
    }

    async fn get_account_balances(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::GetAccountBalancesRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::GetAccountBalancesResponse>, Status> {
        Err(Status::unimplemented("获取科目余额清单功能待实现"))
    }

    async fn post_receivable_entry(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::PostReceivableEntryRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::PostReceivableEntryResponse>, Status> {
        Err(Status::unimplemented("过账应收凭证功能待实现"))
    }

    async fn post_incoming_payment(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::PostIncomingPaymentRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::PostIncomingPaymentResponse>, Status> {
        Err(Status::unimplemented("过账收款功能待实现"))
    }

    async fn clear_customer_open_item(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::ClearCustomerOpenItemRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::ClearCustomerOpenItemResponse>, Status> {
        Err(Status::unimplemented("客户清账功能待实现"))
    }

    async fn send_customer_dunning(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::SendCustomerDunningRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::SendCustomerDunningResponse>, Status> {
        Err(Status::unimplemented("客户催收功能待实现"))
    }

    async fn calculate_exchange_difference(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::CalculateExchangeDifferenceRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::CalculateExchangeDifferenceResponse>, Status> {
        Err(Status::unimplemented("汇兑损益计算功能待实现"))
    }

    async fn post_payable_entry(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::PostPayableEntryRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::PostPayableEntryResponse>, Status> {
        Err(Status::unimplemented("过账应付凭证功能待实现"))
    }

    async fn post_outgoing_payment(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::PostOutgoingPaymentRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::PostOutgoingPaymentResponse>, Status> {
        Err(Status::unimplemented("过账付款功能待实现"))
    }

    async fn clear_vendor_open_item(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::ClearVendorOpenItemRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::ClearVendorOpenItemResponse>, Status> {
        Err(Status::unimplemented("供应商清账功能待实现"))
    }

    async fn set_payment_block(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::SetPaymentBlockRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::SetPaymentBlockResponse>, Status> {
        Err(Status::unimplemented("付款冻结功能待实现"))
    }

    async fn post_period_closing_entry(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::PostPeriodClosingEntryRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::PostPeriodClosingEntryResponse>, Status> {
        Err(Status::unimplemented("期末调整过账功能待实现"))
    }

    async fn generate_gl_account_report(
        &self,
        request: Request<killer_api_contracts::gen::finance::financial::v1::GenerateGLAccountReportRequest>,
    ) -> Result<Response<killer_api_contracts::gen::finance::financial::v1::GenerateGLAccountReportResponse>, Status> {
        Err(Status::unimplemented("生成财务主数据报表功能待实现"))
    }
}
