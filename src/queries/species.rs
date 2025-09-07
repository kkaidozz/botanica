use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::error::DatabaseError;
use crate::types::Species;

/// Insert a new species into the database
pub async fn insert_species(pool: &SqlitePool, species: &Species) -> Result<(), DatabaseError> {
    sqlx::query(
        "INSERT INTO species (id, genus_id, specific_epithet, authority, publication_year, conservation_status) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(species.id.to_string())
    .bind(species.genus_id.to_string())
    .bind(&species.specific_epithet)
    .bind(&species.authority)
    .bind(species.publication_year)
    .bind(&species.conservation_status)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get a species by ID
pub async fn get_species_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Species>, DatabaseError> {
    let row = sqlx::query("SELECT id, genus_id, specific_epithet, authority, publication_year, conservation_status FROM species WHERE id = ?")
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;
    
    if let Some(row) = row {
        let id_str: String = row.get("id");
        let genus_id_str: String = row.get("genus_id");
        let specific_epithet: String = row.get("specific_epithet");
        let authority: String = row.get("authority");
        let publication_year: Option<i32> = row.get("publication_year");
        let conservation_status: Option<String> = row.get("conservation_status");
        
        Ok(Some(Species::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            Uuid::parse_str(&genus_id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            specific_epithet,
            authority,
            publication_year,
            conservation_status,
        )))
    } else {
        Ok(None)
    }
}

/// Get species by name pattern
pub async fn get_species_by_name(pool: &SqlitePool, name: &str) -> Result<Vec<Species>, DatabaseError> {
    let rows = sqlx::query("SELECT id, genus_id, specific_epithet, authority, publication_year, conservation_status FROM species WHERE specific_epithet LIKE ?")
        .bind(format!("%{}%", name))
        .fetch_all(pool)
        .await?;
    
    let mut species = Vec::new();
    for row in rows {
        let id_str: String = row.get("id");
        let genus_id_str: String = row.get("genus_id");
        let specific_epithet: String = row.get("specific_epithet");
        let authority: String = row.get("authority");
        let publication_year: Option<i32> = row.get("publication_year");
        let conservation_status: Option<String> = row.get("conservation_status");
        
        species.push(Species::with_id(
            Uuid::parse_str(&id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            Uuid::parse_str(&genus_id_str).map_err(|e| DatabaseError::validation(e.to_string()))?,
            specific_epithet,
            authority,
            publication_year,
            conservation_status,
        ));
    }
    
    Ok(species)
}

/// Update a species
pub async fn update_species(pool: &SqlitePool, id: Uuid, species: &Species) -> Result<bool, DatabaseError> {
    let result = sqlx::query("UPDATE species SET genus_id = ?, specific_epithet = ?, authority = ?, publication_year = ?, conservation_status = ? WHERE id = ?")
        .bind(species.genus_id.to_string())
        .bind(&species.specific_epithet)
        .bind(&species.authority)
        .bind(species.publication_year)
        .bind(&species.conservation_status)
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Delete a species
pub async fn delete_species(pool: &SqlitePool, id: Uuid) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM species WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}