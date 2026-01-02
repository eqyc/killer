//! 会计凭证数据传输对象
//!
//! 包含命令请求、查询响应、事件载荷的 DTO 定义
//! 所有对外接口使用 DTO，内部领域对象不泄露

use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================
// 命令请求 DTO - Create
// =============================================================================

/// 创建会计凭证请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateJournalEntryRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 过账日期
    #[validate(required)]
    pub posting_date: Option<NaiveDate>,

    /// 凭证日期
    #[validate(required)]
    pub document_date: Option<NaiveDate>,

    /// 币种代码
    #[validate(length(min = 1))]
    pub currency_code: String,

    /// 凭证抬头文本
    #[validate(length(max = 200))]
    pub header_text: Option<String>,

    /// 参考凭证号
    #[validate(length(max = 20))]
    pub reference_document: Option<String>,

    /// 行项目列表（至少2行）
    #[validate(length(min = 2, max = 999))]
    pub line_items: Vec<JournalEntryLineItemRequest>,
}

/// 行项目请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct JournalEntryLineItemRequest {
    /// 行号
    #[validate(range(min = 1))]
    pub line_number: u32,

    /// 会计科目代码
    #[validate(length(min = 1))]
    pub account_code: String,

    /// 金额（必须为正）
    #[validate(range(min = 0.01))]
    pub amount: f64,

    /// 借贷方向 (debit/credit)
    #[validate(length(min = 1))]
    pub debit_credit: String,

    /// 成本中心（可选）
    #[validate(length(max = 10))]
    pub cost_center: Option<String>,

    /// 利润中心（可选）
    #[validate(length(max = 10))]
    pub profit_center: Option<String>,

    /// 行项目文本（可选）
    #[validate(length(max = 200))]
    pub text: Option<String>,

    /// 功能范围（可选）
    #[validate(length(max = 4))]
    pub functional_area: Option<String>,

    /// 业务范围（可选）
    #[validate(length(max = 4))]
    pub business_area: Option<String>,

    /// 订单号（可选）
    #[validate(length(max = 12))]
    pub order_number: Option<String>,
}

// =============================================================================
// 命令请求 DTO - Post/Reverse/Close
// =============================================================================

/// 过账凭证请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PostJournalEntryRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 凭证号
    #[validate(length(min = 1))]
    pub document_number: String,

    /// 过账日期（可选，默认使用凭证创建时的日期）
    #[validate(required)]
    pub posting_date: Option<NaiveDate>,
}

/// 冲销凭证请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReverseJournalEntryRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 原凭证号
    #[validate(length(min = 1))]
    pub original_document_number: String,

    /// 冲销日期
    #[validate(required)]
    pub reversal_date: Option<NaiveDate>,

    /// 冲销原因代码 (1-9)
    #[validate(range(min = 1, max = 9))]
    pub reversal_reason: Option<u8>,
}

/// 关闭会计期间请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CloseFiscalPeriodRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 期间号 (1-16)
    #[validate(range(min = 1, max = 16))]
    pub period: u8,

    /// 是否强制关闭（即使有未过账凭证）
    pub force: Option<bool>,
}

// =============================================================================
// 查询请求 DTO
// =============================================================================

/// 获取凭证详情请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GetJournalEntryRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 凭证号
    #[validate(length(min = 1))]
    pub document_number: String,
}

/// 列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListJournalEntriesRequest {
    /// 公司代码（可选）
    pub company_code: Option<String>,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 凭证状态过滤
    pub status: Option<String>,

    /// 过账日期范围 - 开始
    pub posting_date_from: Option<NaiveDate>,

    /// 过账日期范围 - 结束
    pub posting_date_to: Option<NaiveDate>,

    /// 会计科目过滤
    pub account_code: Option<String>,

    /// 成本中心过滤
    pub cost_center: Option<String>,

    /// 金额范围 - 最小值
    pub amount_min: Option<f64>,

    /// 金额范围 - 最大值
    pub amount_max: Option<f64>,

    /// 文本搜索（抬头文本或行项目文本）
    pub text_search: Option<String>,

    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub page_size: Option<u32>,
}

/// 获取科目余额请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GetAccountBalanceRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 会计科目代码
    #[validate(length(min = 1))]
    pub account_code: String,

    /// 期间号（可选，不传则查询年度累计）
    #[validate(range(min = 1, max = 16))]
    pub period: Option<u8>,
}

/// 获取试算平衡表请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GetTrialBalanceRequest {
    /// 公司代码
    #[validate(length(min = 1))]
    pub company_code: String,

    /// 会计年度
    #[validate(range(min = 1970, max = 9999))]
    pub fiscal_year: i32,

    /// 期间号（可选，不传则查询年度累计）
    #[validate(range(min = 1, max = 16))]
    pub period: Option<u8>,

    /// 是否展开科目层级
    pub expand_hierarchy: Option<bool>,

    /// 是否隐藏零余额科目
    pub hide_zero_balance: Option<bool>,
}

// =============================================================================
// 响应 DTO - Create
// =============================================================================

/// 创建凭证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalEntryResponse {
    /// 凭证编号
    pub document_number: String,

    /// 凭证状态
    pub status: String,

    /// 创建时间
    pub created_at: DateTime<chrono::Utc>,
}

/// 过账凭证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostJournalEntryResponse {
    /// 凭证编号
    pub document_number: String,

    /// 凭证状态
    pub status: String,

    /// 过账日期
    pub posting_date: NaiveDate,

    /// 借方总额
    pub total_debit: f64,

    /// 贷方总额
    pub total_credit: f64,
}

/// 冲销凭证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseJournalEntryResponse {
    /// 原凭证编号
    pub original_document_number: String,

    /// 冲销凭证编号
    pub reversal_document_number: String,

    /// 冲销日期
    pub reversal_date: NaiveDate,

    /// 冲销凭证状态
    pub status: String,
}

/// 关闭期间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseFiscalPeriodResponse {
    /// 公司代码
    pub company_code: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 期间号
    pub period: u8,

    /// 期间状态
    pub status: String,

    /// 有效开始日期
    pub valid_from: NaiveDate,

    /// 有效结束日期
    pub valid_to: NaiveDate,
}

// =============================================================================
// 响应 DTO - Query
// =============================================================================

/// 凭证摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntrySummary {
    /// 凭证编号
    pub document_number: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 过账日期
    pub posting_date: NaiveDate,

    /// 凭证日期
    pub document_date: NaiveDate,

    /// 币种代码
    pub currency_code: String,

    /// 凭证状态
    pub status: String,

    /// 总金额
    pub total_amount: f64,

    /// 行项目数量
    pub line_count: u32,

    /// 抬头文本
    pub header_text: Option<String>,
}

/// 凭证详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryDetail {
    /// 凭证编号
    pub document_number: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 公司代码
    pub company_code: String,

    /// 过账日期
    pub posting_date: NaiveDate,

    /// 凭证日期
    pub document_date: NaiveDate,

    /// 币种代码
    pub currency_code: String,

    /// 凭证状态
    pub status: String,

    /// 抬头文本
    pub header_text: Option<String>,

    /// 参考凭证号
    pub reference_document: Option<String>,

    /// 借方总额
    pub total_debit: f64,

    /// 贷方总额
    pub total_credit: f64,

    /// 行项目列表
    pub line_items: Vec<JournalEntryLineItemDetail>,

    /// 版本号
    pub version: u64,

    /// 创建时间
    pub created_at: DateTime<chrono::Utc>,

    /// 过账时间
    pub posted_at: Option<DateTime<chrono::Utc>>,
}

/// 行项目详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryLineItemDetail {
    /// 行号
    pub line_number: u32,

    /// 会计科目代码
    pub account_code: String,

    /// 金额
    pub amount: f64,

    /// 借贷方向
    pub debit_credit: String,

    /// 成本中心
    pub cost_center: Option<String>,

    /// 利润中心
    pub profit_center: Option<String>,

    /// 行项目文本
    pub text: Option<String>,

    /// 功能范围
    pub functional_area: Option<String>,

    /// 业务范围
    pub business_area: Option<String>,

    /// 订单号
    pub order_number: Option<String>,
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    /// 数据项列表
    pub items: Vec<T>,

    /// 总数量
    pub total_count: u64,

    /// 当前页码
    pub page: u32,

    /// 每页大小
    pub page_size: u32,

    /// 总页数
    pub total_pages: u32,
}

// =============================================================================
// 响应 DTO - Balance & Trial Balance
// =============================================================================

/// 科目余额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// 公司代码
    pub company_code: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 会计科目代码
    pub account_code: String,

    /// 会计科目名称
    pub account_name: Option<String>,

    /// 期间
    pub period: Option<u8>,

    /// 借方发生额
    pub debit_amount: f64,

    /// 贷方发生额
    pub credit_amount: f64,

    /// 期初余额
    pub opening_balance: f64,

    /// 期末余额
    pub closing_balance: f64,

    /// 余额方向
    pub balance_direction: String,
}

/// 试算平衡表行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceLine {
    /// 会计科目代码
    pub account_code: String,

    /// 会计科目名称
    pub account_name: Option<String>,

    /// 借方金额
    pub debit_amount: f64,

    /// 贷方金额
    pub credit_amount: f64,

    /// 净额
    pub net_amount: f64,

    /// 余额方向
    pub balance_direction: String,
}

/// 试算平衡表汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceSummary {
    /// 公司代码
    pub company_code: String,

    /// 会计年度
    pub fiscal_year: i32,

    /// 期间
    pub period: Option<u8>,

    /// 借方合计
    pub total_debit: f64,

    /// 贷方合计
    pub total_credit: f64,

    /// 差额
    pub difference: f64,

    /// 是否平衡
    pub is_balanced: bool,

    /// 明细行
    pub lines: Vec<TrialBalanceLine>,
}
