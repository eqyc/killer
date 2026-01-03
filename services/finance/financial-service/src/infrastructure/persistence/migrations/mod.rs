//! 数据库迁移模块
//!
//! 使用 sqlx-migrate 管理数据库迁移

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

/// 回滚所有迁移（慎用！）
pub async fn rollback_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("migrations")
        .redo(pool)
        .await?;

    Ok(())
}

/// 检查迁移状态
pub async fn check_migration_status(pool: &PgPool) -> Result<Vec<sqlx::migrate::Migration>, sqlx::Error> {
    let migrations_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations_path).await?;

    Ok(migrator.migrations().to_vec())
}
