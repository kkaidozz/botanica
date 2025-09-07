use sqlx::SqlitePool;
use crate::error::DatabaseError;

/// Stub implementation for search
pub async fn search_species(_pool: &SqlitePool, _query: &str) -> Result<Vec<String>, DatabaseError> {
    Ok(Vec::new())
}