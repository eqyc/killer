//! 健康检查端点

use axum::{routing::get, Json, response::IntoResponse, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "financial-service".to_string(),
        timestamp: chrono::Utc::now(),
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
}
