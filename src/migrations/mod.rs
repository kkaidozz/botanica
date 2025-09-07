use sqlx::{SqlitePool, query};
use crate::error::DatabaseError;

pub mod runner;
pub mod schemas;


/// Initialize the database with all required tables
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), DatabaseError> {
    // Create families table
    query(r#"
        CREATE TABLE IF NOT EXISTS families (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            authority TEXT
        )
    "#)
    .execute(pool)
    .await?;

    // Create genera table
    query(r#"
        CREATE TABLE IF NOT EXISTS genera (
            id TEXT PRIMARY KEY,
            family_id TEXT NOT NULL,
            name TEXT NOT NULL,
            authority TEXT,
            FOREIGN KEY (family_id) REFERENCES families(id)
        )
    "#)
    .execute(pool)
    .await?;

    // Create species table
    query(r#"
        CREATE TABLE IF NOT EXISTS species (
            id TEXT PRIMARY KEY,
            genus_id TEXT NOT NULL,
            specific_epithet TEXT NOT NULL,
            authority TEXT,
            publication_year INTEGER,
            conservation_status TEXT,
            FOREIGN KEY (genus_id) REFERENCES genera(id)
        )
    "#)
    .execute(pool)
    .await?;

    // Create specimens table
    query(r#"
        CREATE TABLE IF NOT EXISTS specimens (
            id TEXT PRIMARY KEY,
            species_id TEXT NOT NULL,
            collector TEXT,
            collection_date TEXT,
            location TEXT,
            notes TEXT,
            FOREIGN KEY (species_id) REFERENCES species(id)
        )
    "#)
    .execute(pool)
    .await?;

    Ok(())
}