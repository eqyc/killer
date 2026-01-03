//! 数据库迁移模块

use sqlx::PgPool;
use std::path::Path;

/// 运行所有待执行的迁移
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrations_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    sqlx::migrate!("migrations")
        .run(pool)
        .await?;

    Ok(())
}
