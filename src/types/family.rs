use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a family in the botanical taxonomy system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Family {
    /// Unique identifier for the family
    pub id: Uuid,
    
    /// The family name
    pub name: String,
    
    /// The author(s) who first described this family
    pub authority: String,
}

impl Family {
    /// Creates a new Family instance with a generated UUID.
    pub fn new(
        name: String,
        authority: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            authority,
        }
    }
    
    /// Creates a new Family instance with a specific UUID.
    pub fn with_id(
        id: Uuid,
        name: String,
        authority: String,
    ) -> Self {
        Self {
            id,
            name,
            authority,
        }
    }
}