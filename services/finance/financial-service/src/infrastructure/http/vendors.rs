//! 供应商 HTTP API

use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct VendorResponse {
    pub vendor_id: String,
    pub company_code: String,
    pub account_group: String,
    pub name_1: String,
    pub country: String,
    pub currency: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateVendorRequest {
    pub vendor_id: String,
    pub company_code: String,
    pub account_group: String,
    pub name_1: String,
    pub country: String,
    pub currency: String,
}

async fn list_vendors(pool: Arc<PgPool>) -> Json<Vec<VendorResponse>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
        "SELECT vendor_id, account_group, company_code, name_1, country, status FROM vendors"
    )
    .fetch_all(&*pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let response: Vec<VendorResponse> = rows
        .into_iter()
        .map(|(id, ag, cc, n1, c, s)| VendorResponse {
            vendor_id: id,
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

async fn create_vendor(
    Json(req): Json<CreateVendorRequest>,
    pool: Arc<PgPool>,
) -> Result<impl axum::response::IntoResponse, impl axum::response::IntoResponse> {
    let query = r#"
        INSERT INTO vendors (vendor_id, company_code, account_group, name_1, country, currency, status, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, '1', 'SYSTEM')
        ON CONFLICT (vendor_id) DO UPDATE SET
            name_1 = EXCLUDED.name_1,
            country = EXCLUDED.country
    "#;

    sqlx::query(query)
        .bind(&req.vendor_id)
        .bind(&req.company_code)
        .bind(&req.account_group)
        .bind(&req.name_1)
        .bind(&req.country)
        .bind(&req.currency)
        .execute(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<_, (axum::http::StatusCode, String)>((axum::http::StatusCode::CREATED, "Vendor created".to_string()))
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/vendors", get({
            let value = pool.clone();
            move || list_vendors(value)
        }))
        .route("/api/vendors", post({
            let value = pool.clone();
            move |j| create_vendor(j, value)
        }))
}
