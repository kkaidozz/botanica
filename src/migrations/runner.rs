use sqlx::SqlitePool;
use crate::error::DatabaseError;

/// Run all database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), DatabaseError> {
    crate::migrations::run_migrations(pool).await
}