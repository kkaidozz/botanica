use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::error::DatabaseError;
use crate::types::Family;

/// Insert a new family into the database
pub async fn insert_family(pool: &SqlitePool, family: &Family) -> Result<(), DatabaseError> {
    sqlx::query("INSERT INTO families (id, name, authority) VALUES (?, ?, ?)")
        .bind(family.id.to_string())
        .bind(&family.name)
        .bind(&family.authority)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Get a family by ID
pub async fn get_family_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Family>, DatabaseError> {
    let row = sqlx::query("SELECT id, name, authority FROM families WHERE id = ?")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;
    
    if let Some(row) = row {
        let id_str: String = row.get("id");
        let name: String = row.get("name");
        let authority: String = row.get("authority");
        
        Ok(Some(Family::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            name,
            authority,
        )))
    } else {
        Ok(None)
    }
}

/// Get families by name pattern
pub async fn get_families_by_name(pool: &SqlitePool, name: &str) -> Result<Vec<Family>, DatabaseError> {
    let rows = sqlx::query("SELECT id, name, authority FROM families WHERE name LIKE ? ORDER BY name")
        .bind(format!("%{}%", name))
        .fetch_all(pool)
        .await?;
    
    let mut families = Vec::new();
    for row in rows {
        let id_str: String = row.get("id");
        let name: String = row.get("name");
        let authority: String = row.get("authority");
        
        families.push(Family::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            name,
            authority,
        ));
    }
    
    Ok(families)
}

/// Update a family
pub async fn update_family(pool: &SqlitePool, id: Uuid, family: &Family) -> Result<bool, DatabaseError> {
    let result = sqlx::query("UPDATE families SET name = ?, authority = ? WHERE id = ?")
        .bind(&family.name)
        .bind(&family.authority)
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Delete a family
pub async fn delete_family(pool: &SqlitePool, id: Uuid) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM families WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}