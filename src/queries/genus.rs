use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::error::DatabaseError;
use crate::types::Genus;

/// Insert a new genus into the database
pub async fn insert_genus(pool: &SqlitePool, genus: &Genus) -> Result<(), DatabaseError> {
    sqlx::query("INSERT INTO genera (id, family_id, name, authority) VALUES (?, ?, ?, ?)")
        .bind(genus.id.to_string())
        .bind(genus.family_id.to_string())
        .bind(&genus.name)
        .bind(&genus.authority)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Get a genus by ID
pub async fn get_genus_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Genus>, DatabaseError> {
    let row = sqlx::query("SELECT id, family_id, name, authority FROM genera WHERE id = ?")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;
    
    if let Some(row) = row {
        let id_str: String = row.get("id");
        let family_id_str: String = row.get("family_id");
        let name: String = row.get("name");
        let authority: String = row.get("authority");
        
        Ok(Some(Genus::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            Uuid::parse_str(&family_id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            name,
            authority,
        )))
    } else {
        Ok(None)
    }
}

/// Get genera by family ID
pub async fn get_genera_by_family_id(pool: &SqlitePool, family_id: Uuid) -> Result<Vec<Genus>, DatabaseError> {
    let rows = sqlx::query("SELECT id, family_id, name, authority FROM genera WHERE family_id = ? ORDER BY name")
        .bind(family_id.to_string())
        .fetch_all(pool)
        .await?;
    
    let mut genera = Vec::new();
    for row in rows {
        let id_str: String = row.get("id");
        let family_id_str: String = row.get("family_id");
        let name: String = row.get("name");
        let authority: String = row.get("authority");
        
        genera.push(Genus::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            Uuid::parse_str(&family_id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            name,
            authority,
        ));
    }
    
    Ok(genera)
}

/// Update a genus
pub async fn update_genus(pool: &SqlitePool, id: Uuid, genus: &Genus) -> Result<bool, DatabaseError> {
    let result = sqlx::query("UPDATE genera SET family_id = ?, name = ?, authority = ? WHERE id = ?")
        .bind(genus.family_id.to_string())
        .bind(&genus.name)
        .bind(&genus.authority)
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Delete a genus
pub async fn delete_genus(pool: &SqlitePool, id: Uuid) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM genera WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}