use sqlx::SqlitePool;
use crate::error::DatabaseError;

/// Stub implementation for specimens
pub async fn insert_specimen(_pool: &SqlitePool) -> Result<(), DatabaseError> {
    Ok(())
}