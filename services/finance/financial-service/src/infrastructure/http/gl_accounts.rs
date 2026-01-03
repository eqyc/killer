//! 总账科目 HTTP API

use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct GLAccountResponse {
    pub chart_of_accounts: String,
    pub account_code: String,
    pub company_code: String,
    pub account_type: String,
    pub description: String,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct CreateGLAccountRequest {
    pub chart_of_accounts: String,
    pub account_code: String,
    pub company_code: String,
    pub account_type: String,
    pub balance_sheet_indicator: String,
    pub currency: String,
    pub description: String,
}

async fn list_gl_accounts(pool: Arc<PgPool>) -> Json<Vec<GLAccountResponse>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
        "SELECT chart_of_accounts, account_code, company_code, account_type, description, currency FROM gl_accounts WHERE is_deleted = false ORDER BY account_code"
    )
    .fetch_all(&*pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let response: Vec<GLAccountResponse> = rows
        .into_iter()
        .map(|(coa, code, cc, at, desc, cur)| GLAccountResponse {
            chart_of_accounts: coa,
            account_code: code,
            company_code: cc,
            account_type: at,
            description: desc,
            currency: cur,
        })
        .collect();

    Json(response)
}

async fn create_gl_account(
    Json(req): Json<CreateGLAccountRequest>,
    pool: Arc<PgPool>,
) -> Result<impl axum::response::IntoResponse, impl axum::response::IntoResponse> {
    let query = r#"
        INSERT INTO gl_accounts (
            chart_of_accounts, account_code, company_code, account_type,
            balance_sheet_indicator, currency, description, short_description,
            account_group, account_indicator_group, cost_control_area, is_deleted, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'GL', '', '', false, 'SYSTEM')
        ON CONFLICT (chart_of_accounts, account_code, company_code) DO UPDATE SET
            description = EXCLUDED.description,
            short_description = EXCLUDED.short_description
    "#;

    sqlx::query(query)
        .bind(&req.chart_of_accounts)
        .bind(&req.account_code)
        .bind(&req.company_code)
        .bind(&req.account_type)
        .bind(&req.balance_sheet_indicator)
        .bind(&req.currency)
        .bind(&req.description)
        .bind(&req.description)
        .execute(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<_, (axum::http::StatusCode, String)>((axum::http::StatusCode::CREATED, "GL Account created".to_string()))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/gl-accounts", get({
            let value = pool.clone();
            move || list_gl_accounts(value)
        }))
        .route("/api/gl-accounts", post({
            let value = pool.clone();
            move |j| create_gl_account(j, value)
        }))
}
