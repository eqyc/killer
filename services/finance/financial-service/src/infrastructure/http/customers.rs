//! 客户 HTTP API

use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use crate::infrastructure::persistence::postgres::PostgresRepository;
use killer_domain_primitives::CompanyCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct CustomerResponse {
    pub customer_id: String,
    pub company_code: String,
    pub account_group: String,
    pub name_1: String,
    pub country: String,
    pub currency: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub customer_id: String,
    pub company_code: String,
    pub account_group: String,
    pub name_1: String,
    pub country: String,
    pub currency: String,
}

async fn list_customers(pool: Arc<PgPool>) -> Json<Vec<CustomerResponse>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
        "SELECT customer_id, account_group, company_code, name_1, country, status FROM customers"
    )
    .fetch_all(&*pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let response: Vec<CustomerResponse> = rows
        .into_iter()
        .map(|(id, ag, cc, n1, c, s)| CustomerResponse {
            customer_id: id,
            company_code: cc,
            account_group: ag,
            name_1: n1,
            country: c,
            currency: "CNY".to_string(),
            status: s,
        })
        .collect();

    Json(response)
}

async fn create_customer(
    Json(req): Json<CreateCustomerRequest>,
    pool: Arc<PgPool>,
) -> Result<impl axum::response::IntoResponse, impl axum::response::IntoResponse> {
    let query = r#"
        INSERT INTO customers (customer_id, company_code, account_group, name_1, country, currency, status, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, '1', 'SYSTEM')
        ON CONFLICT (customer_id) DO UPDATE SET
            name_1 = EXCLUDED.name_1,
            country = EXCLUDED.country
    "#;

    sqlx::query(query)
        .bind(&req.customer_id)
        .bind(&req.company_code)
        .bind(&req.account_group)
        .bind(&req.name_1)
        .bind(&req.country)
        .bind(&req.currency)
        .execute(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<_, (axum::http::StatusCode, String)>((axum::http::StatusCode::CREATED, "Customer created".to_string()))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/customers", get({
            let value = pool.clone();
            move || list_customers(value)
        }))
        .route("/api/customers", post({
            let value = pool.clone();
            move |j| create_customer(j, value)
        }))
}
