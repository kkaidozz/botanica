use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a species in the botanical taxonomy system.
/// 
/// A species is the basic unit of classification and a taxonomic rank of an organism,
/// as well as a unit of biodiversity. It represents a group of living organisms
/// consisting of similar individuals capable of exchanging genes or interbreeding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Species {
    /// Unique identifier for the species
    pub id: Uuid,
    
    /// Reference to the genus this species belongs to
    pub genus_id: Uuid,
    
    /// The specific epithet (species name) in binomial nomenclature
    pub specific_epithet: String,
    
    /// The author(s) who first described this species
    pub authority: String,
    
    /// The year this species was first published/described
    pub publication_year: Option<i32>,
    
    /// Conservation status according to IUCN or other conservation organizations
    pub conservation_status: Option<String>,
}

impl Species {
    /// Creates a new Species instance with a generated UUID.
    pub fn new(
        genus_id: Uuid,
        specific_epithet: String,
        authority: String,
        publication_year: Option<i32>,
        conservation_status: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            genus_id,
            specific_epithet,
            authority,
            publication_year,
            conservation_status,
        }
    }

    /// Creates a new Species instance with a specific UUID.
    pub fn with_id(
        id: Uuid,
        genus_id: Uuid,
        specific_epithet: String,
        authority: String,
        publication_year: Option<i32>,
        conservation_status: Option<String>,
    ) -> Self {
        Self {
            id,
            genus_id,
            specific_epithet,
            authority,
            publication_year,
            conservation_status,
        }
    }

    /// Returns the specific epithet.
    pub fn get_specific_epithet(&self) -> &str {
        &self.specific_epithet
    }

    /// Returns the taxonomic authority.
    pub fn get_authority(&self) -> &str {
        &self.authority
    }

    /// Returns the publication year if available.
    pub fn get_publication_year(&self) -> Option<i32> {
        self.publication_year
    }

    /// Returns the conservation status if available.
    pub fn get_conservation_status(&self) -> Option<&str> {
        self.conservation_status.as_deref()
    }

    /// Updates the conservation status.
    pub fn set_conservation_status(&mut self, status: Option<String>) {
        self.conservation_status = status;
    }

    /// Checks if the species has a conservation status.
    pub fn has_conservation_status(&self) -> bool {
        self.conservation_status.is_some()
    }
}