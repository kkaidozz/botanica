/// Database schema definitions

/// SQL for the families table
pub const FAMILIES_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS families (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    authority TEXT
)
"#;

/// SQL for the genera table
pub const GENERA_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS genera (
    id TEXT PRIMARY KEY,
    family_id TEXT NOT NULL,
    name TEXT NOT NULL,
    authority TEXT,
    FOREIGN KEY (family_id) REFERENCES families(id)
)
"#;

/// SQL for the species table
pub const SPECIES_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS species (
    id TEXT PRIMARY KEY,
    genus_id TEXT NOT NULL,
    specific_epithet TEXT NOT NULL,
    authority TEXT NOT NULL,
    publication_year INTEGER,
    conservation_status TEXT,
    FOREIGN KEY (genus_id) REFERENCES genera(id)
)
"#;