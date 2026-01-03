//! HTTP API 模块
//!
//! 使用 Axum 框架提供 RESTful API 接口

use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::PgPool;

mod health;
mod gl_accounts;
mod customers;
mod vendors;

pub fn create_router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/index.html") }))
        .route("/index.html", get(|| async {
            include_str!("../../../static/index.html")
        }))
        .merge(health::router())
        .merge(gl_accounts::router(pool.clone()))
        .merge(customers::router(pool.clone()))
        .merge(vendors::router(pool))
        .layer(tower_http::cors::CorsLayer::permissive())
}
