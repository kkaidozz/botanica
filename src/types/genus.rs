use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a genus in the botanical taxonomy system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genus {
    /// Unique identifier for the genus
    pub id: Uuid,
    
    /// Reference to the family this genus belongs to
    pub family_id: Uuid,
    
    /// The genus name
    pub name: String,
    
    /// The author(s) who first described this genus
    pub authority: String,
}

impl Genus {
    /// Creates a new Genus instance with a generated UUID.
    pub fn new(
        family_id: Uuid,
        name: String,
        authority: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            family_id,
            name,
            authority,
        }
    }
    
    /// Creates a new Genus instance with a specific UUID.
    pub fn with_id(
        id: Uuid,
        family_id: Uuid,
        name: String,
        authority: String,
    ) -> Self {
        Self {
            id,
            family_id,
            name,
            authority,
        }
    }
}