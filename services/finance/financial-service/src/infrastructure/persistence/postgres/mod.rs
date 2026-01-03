//! PostgreSQL 持久化实现

use sqlx::{Pool, Postgres};

/// PostgreSQL 仓储实现
#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pool: Pool<Postgres>,
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
