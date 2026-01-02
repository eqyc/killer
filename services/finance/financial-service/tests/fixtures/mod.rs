//! 测试配置
//!
//! 提供测试环境配置和 fixtures

use chrono::{NaiveDate, Utc};
use killer_financial_service::application::commands::*;
use killer_financial_service::application::dto::*;
use killer_financial_service::application::queries::*;
use killer_financial_service::domain::*;
use rand::Rng;
use std::sync::Arc;
use uuid::Uuid;

// =============================================================================
// 常量
// =============================================================================

/// 测试租户 ID
pub const TEST_TENANT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

/// 测试用户 ID
pub const TEST_USER_ID: &str = "550e8400-e29b-41d4-a716-446655440001";

/// 测试公司代码
pub const TEST_COMPANY_CODE: &str = "1000";

/// 测试会计年度
pub const TEST_FISCAL_YEAR: i32 = 2024;

/// 测试币种
pub const TEST_CURRENCY_CODE: &str = "CNY";

// =============================================================================
// 测试数据生成器
// =============================================================================

/// 生成唯一的测试 ID
pub fn generate_test_id() -> String {
    Uuid::new_v4().to_string()
}

/// 生成唯一的凭证号
pub fn generate_document_number() -> String {
    format!("JE-{}", Uuid::new_v4().to_string()[..8].to_uppercase())
}

/// 生成测试金额
pub fn generate_amount(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen_range(min..max) * 100.0).round() / 100.0
}

/// 生成测试日期
pub fn generate_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

// =============================================================================
// 测试数据 Factories
// =============================================================================

/// 凭证行项目工厂
#[derive(Default, Clone)]
pub struct JournalEntryLineItemFactory {
    line_number: i32,
    account_code: String,
    amount: f64,
    debit_credit: String,
    cost_center: Option<String>,
    profit_center: Option<String>,
    text: Option<String>,
}

impl JournalEntryLineItemFactory {
    pub fn new() -> Self {
        Self {
            line_number: 1,
            account_code: "10010001".to_string(),
            amount: 1000.00,
            debit_credit: "D".to_string(),
            cost_center: Some("CC001".to_string()),
            profit_center: None,
            text: Some("Test line item".to_string()),
        }
    }

    pub fn with_account_code(mut self, code: &str) -> Self {
        self.account_code = code.to_string();
        self
    }

    pub fn with_amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn with_debit_credit(mut self, dc: &str) -> Self {
        self.debit_credit = dc.to_string();
        self
    }

    pub fn with_cost_center(mut self, cc: &str) -> Self {
        self.cost_center = Some(cc.to_string());
        self
    }

    pub fn with_profit_center(mut self, pc: &str) -> Self {
        self.profit_center = Some(pc.to_string());
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn build(&self) -> JournalEntryLineItemRequest {
        JournalEntryLineItemRequest {
            line_number: self.line_number,
            account_code: self.account_code.clone(),
            amount: self.amount,
            debit_credit: self.debit_credit.clone(),
            cost_center: self.cost_center.clone(),
            profit_center: self.profit_center.clone(),
            text: self.text.clone(),
            functional_area: None,
            business_area: None,
            order_number: None,
            tax_code: None,
            tax_amount: None,
        }
    }

    /// 构建借方行
    pub fn build_debit(&self) -> JournalEntryLineItemRequest {
        let mut item = self.build();
        item.debit_credit = "D".to_string();
        item
    }

    /// 构建贷方行
    pub fn build_credit(&self) -> JournalEntryLineItemRequest {
        let mut item = self.build();
        item.debit_credit = "C".to_string();
        item
    }
}

/// 凭证工厂
#[derive(Default, Clone)]
pub struct JournalEntryFactory {
    tenant_id: String,
    company_code: String,
    fiscal_year: i32,
    document_number: String,
    posting_date: NaiveDate,
    document_date: NaiveDate,
    currency_code: String,
    header_text: Option<String>,
    reference_document: Option<String>,
    line_items: Vec<JournalEntryLineItemRequest>,
}

impl JournalEntryFactory {
    pub fn new() -> Self {
        let now = Utc::now().date_naive();
        Self {
            tenant_id: TEST_TENANT_ID.to_string(),
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: TEST_FISCAL_YEAR,
            document_number: generate_document_number(),
            posting_date: now,
            document_date: now,
            currency_code: TEST_CURRENCY_CODE.to_string(),
            header_text: None,
            reference_document: None,
            line_items: vec![],
        }
    }

    pub fn with_tenant_id(mut self, tenant_id: &str) -> Self {
        self.tenant_id = tenant_id.to_string();
        self
    }

    pub fn with_company_code(mut self, code: &str) -> Self {
        self.company_code = code.to_string();
        self
    }

    pub fn with_fiscal_year(mut self, year: i32) -> Self {
        self.fiscal_year = year;
        self
    }

    pub fn with_posting_date(mut self, date: NaiveDate) -> Self {
        self.posting_date = date;
        self
    }

    pub fn with_header_text(mut self, text: &str) -> Self {
        self.header_text = Some(text.to_string());
        self
    }

    pub fn with_line_items(mut self, items: Vec<JournalEntryLineItemRequest>) -> Self {
        self.line_items = items;
        self
    }

    /// 添加平衡的行项目
    pub fn with_balanced_lines(mut self, amount: f64) -> Self {
        let debit = JournalEntryLineItemFactory::new()
            .with_amount(amount)
            .with_debit_credit("D")
            .build();

        let credit = JournalEntryLineItemFactory::new()
            .with_amount(amount)
            .with_debit_credit("C")
            .build();

        self.line_items = vec![debit, credit];
        self
    }

    /// 添加多行平衡项目
    pub fn with_multiple_balanced_lines(mut self, amounts: Vec<f64>) -> Self {
        let total: f64 = amounts.iter().sum();

        let mut items = Vec::new();

        // 添加借方行
        for (i, &amount) in amounts.iter().enumerate() {
            items.push(
                JournalEntryLineItemFactory::new()
                    .with_line_number((i + 1) as i32)
                    .with_amount(amount)
                    .with_debit_credit("D")
                    .build()
            );
        }

        // 添加贷方行（总额）
        items.push(
            JournalEntryLineItemFactory::new()
                .with_line_number((amounts.len() + 1) as i32)
                .with_amount(total)
                .with_debit_credit("C")
                .build()
        );

        self.line_items = items;
        self
    }

    pub fn build(&self) -> CreateJournalEntryCommand {
        CreateJournalEntryCommand {
            tenant_id: self.tenant_id.clone(),
            user_id: TEST_USER_ID.to_string(),
            company_code: self.company_code.clone(),
            fiscal_year: self.fiscal_year,
            posting_date: self.posting_date,
            document_date: self.document_date,
            currency_code: self.currency_code.clone(),
            header_text: self.header_text.clone(),
            reference_document: self.reference_document.clone(),
            line_items: self.line_items.clone(),
            idempotency_key: None,
            extensions: None,
        }
    }
}

/// 会计期间工厂
#[derive(Default, Clone)]
pub struct FiscalPeriodFactory {
    tenant_id: String,
    company_code: String,
    fiscal_year: i32,
    period: i32,
    status: PeriodStatus,
    valid_from: NaiveDate,
    valid_to: NaiveDate,
}

impl FiscalPeriodFactory {
    pub fn new() -> Self {
        let now = Utc::now().date_naive();
        Self {
            tenant_id: TEST_TENANT_ID.to_string(),
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: TEST_FISCAL_YEAR,
            period: 1,
            status: PeriodStatus::Open,
            valid_from: now.with_day(1).unwrap(),
            valid_to: now.with_day(28).unwrap(),
        }
    }

    pub fn with_status(mut self, status: PeriodStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_period(mut self, period: i32) -> Self {
        self.period = period;
        self
    }

    pub fn with_year(mut self, year: i32) -> Self {
        self.fiscal_year = year;
        self
    }

    pub fn build(&self) -> FiscalPeriod {
        FiscalPeriod::new(
            self.tenant_id.clone(),
            self.company_code.clone(),
            self.fiscal_year,
            self.period,
            self.valid_from,
            self.valid_to,
        ).unwrap()
    }
}

// =============================================================================
// Test Fixtures
// =============================================================================

/// 标准测试夹具
pub struct StandardTestFixture {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub company_code: String,
    pub fiscal_year: i32,
    pub open_period: FiscalPeriod,
    pub closed_period: FiscalPeriod,
}

impl StandardTestFixture {
    /// 创建标准测试夹具
    pub async fn create() -> Self {
        let tenant_id = Uuid::parse_str(TEST_TENANT_ID).unwrap();
        let user_id = Uuid::parse_str(TEST_USER_ID).unwrap();

        // 创建开放期间
        let open_period = FiscalPeriodFactory::new()
            .with_status(PeriodStatus::Open)
            .with_period(1)
            .build();

        // 创建已关闭期间
        let closed_period = FiscalPeriodFactory::new()
            .with_status(PeriodStatus::Closed)
            .with_period(12)
            .build();

        Self {
            tenant_id,
            user_id,
            company_code: TEST_COMPANY_CODE.to_string(),
            fiscal_year: TEST_FISCAL_YEAR,
            open_period,
            closed_period,
        }
    }
}

/// 多租户测试夹具
pub struct MultiTenantTestFixture {
    pub tenant_a: (Uuid, String),
    pub tenant_b: (Uuid, String),
    pub tenant_a_entries: Vec<Uuid>,
    pub tenant_b_entries: Vec<Uuid>,
}

impl MultiTenantTestFixture {
    pub async fn create() -> Self {
        Self {
            tenant_a: (Uuid::new_v4(), "Tenant A".to_string()),
            tenant_b: (Uuid::new_v4(), "Tenant B".to_string()),
            tenant_a_entries: vec![],
            tenant_b_entries: vec![],
        }
    }
}

/// 平衡凭证测试夹具
pub struct BalancedEntryFixture {
    pub command: CreateJournalEntryCommand,
    pub total_amount: f64,
}

impl BalancedEntryFixture {
    pub fn create(amount: f64) -> Self {
        let command = JournalEntryFactory::new()
            .with_balanced_lines(amount)
            .build();

        Self {
            command,
            total_amount: amount,
        }
    }

    pub fn create_multiple(amounts: Vec<f64>) -> Self {
        let command = JournalEntryFactory::new()
            .with_multiple_balanced_lines(amounts.clone())
            .build();

        Self {
            command,
            total_amount: amounts.iter().sum(),
        }
    }
}

/// 不平衡凭证测试夹具
pub struct UnbalancedEntryFixture {
    pub command: CreateJournalEntryCommand,
}

impl UnbalancedEntryFixture {
    pub fn create() -> Self {
        let debit = JournalEntryLineItemFactory::new()
            .with_amount(1000.00)
            .with_debit_credit("D")
            .build();

        let credit = JournalEntryLineItemFactory::new()
            .with_amount(900.00) // 不平衡
            .with_debit_credit("C")
            .build();

        let command = JournalEntryFactory::new()
            .with_line_items(vec![debit, credit])
            .build();

        Self { command }
    }
}

// =============================================================================
// Mock Implementations
// =============================================================================

/// Mock 仓储
#[derive(Clone, Default)]
pub struct MockRepository<T> {
    pub storage: Arc<std::sync::Mutex<Vec<T>>>,
    pub save_calls: Arc<std::sync::Mutex<Vec<T>>>,
    pub find_calls: Arc<std::sync::Mutex<Vec<String>>>,
}

impl<T> MockRepository<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            storage: Arc::new(std::sync::Mutex::new(Vec::new())),
            save_calls: Arc::new(std::sync::Mutex::new(Vec::new())),
            find_calls: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn save(&self, item: &T) -> Result<(), anyhow::Error> {
        let mut storage = self.storage.lock().unwrap();
        storage.push(item.clone());

        let mut calls = self.save_calls.lock().unwrap();
        calls.push(item.clone());

        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<T>, anyhow::Error> {
        let mut calls = self.find_calls.lock().unwrap();
        calls.push(id.to_string());

        let storage = self.storage.lock().unwrap();
        // Simple search - in real tests use proper mocking library
        Ok(None)
    }

    pub fn get_saved_items(&self) -> Vec<T> {
        let storage = self.storage.lock().unwrap();
        storage.clone()
    }

    pub fn get_save_call_count(&self) -> usize {
        let calls = self.save_calls.lock().unwrap();
        calls.len()
    }
}

/// Mock 事件总线
#[derive(Clone, Default)]
pub struct MockEventBus {
    pub published_events: Arc<std::sync::Mutex<Vec<DomainEvent>>>,
    pub publish_calls: Arc<std::sync::Mutex<usize>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(std::sync::Mutex::new(Vec::new())),
            publish_calls: Arc::new(std::sync::Mutex::new(0)),
        }
    }

    pub async fn publish(&self, event: DomainEvent) -> Result<(), anyhow::Error> {
        let mut events = self.published_events.lock().unwrap();
        events.push(event);

        let mut calls = self.publish_calls.lock().unwrap();
        *calls += 1;

        Ok(())
    }

    pub fn get_published_events(&self) -> Vec<DomainEvent> {
        let events = self.published_events.lock().unwrap();
        events.clone()
    }

    pub fn get_publish_call_count(&self) -> usize {
        let calls = self.publish_calls.lock().unwrap();
        *calls
    }
}

/// Mock 主数据验证器
#[derive(Clone, Default)]
pub struct MockMasterDataValidator {
    pub validated_accounts: Arc<std::sync::Mutex<Vec<String>>>,
    pub validated_cost_centers: Arc<std::sync::Mutex<Vec<String>>>,
}

impl MockMasterDataValidator {
    pub fn new() -> Self {
        Self {
            validated_accounts: Arc::new(std::sync::Mutex::new(Vec::new())),
            validated_cost_centers: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn validate_account(&self, _tenant_id: &str, code: &str) -> Result<bool, anyhow::Error> {
        let mut accounts = self.validated_accounts.lock().unwrap();
        accounts.push(code.to_string());

        // Mock: 1001 开头科目有效
        Ok(code.starts_with("1001") || code.starts_with("2001"))
    }

    pub async fn validate_cost_center(&self, _tenant_id: &str, code: &str) -> Result<bool, anyhow::Error> {
        let mut centers = self.validated_cost_centers.lock().unwrap();
        centers.push(code.to_string());

        // Mock: CC 开头有效
        Ok(code.starts_with("CC"))
    }
}
