use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Growth stage enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrowthStage {
    Seed,
    Germination,
    Seedling,
    Vegetative,
    Flowering,
    Harvest,
    Drying,
    Curing,
}

/// Environmental conditions during cultivation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub temperature_celsius: Option<f32>,
    pub humidity_percent: Option<f32>,
    pub ph_level: Option<f32>,
    pub light_hours: Option<f32>,
    pub co2_ppm: Option<i32>,
    pub recorded_at: DateTime<Utc>,
}

/// Cultivation record for tracking plant growth
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CultivationRecord {
    pub id: Uuid,
    pub species_id: Uuid,
    pub growth_stage: GrowthStage,
    pub environment_id: Option<Uuid>,
    pub notes: Option<String>,
    pub photos: Vec<String>,
    pub recorded_at: DateTime<Utc>,
    pub cultivator: String,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            temperature_celsius: None,
            humidity_percent: None,
            ph_level: None,
            light_hours: None,
            co2_ppm: None,
            recorded_at: Utc::now(),
        }
    }
}

impl CultivationRecord {
    pub fn new(species_id: Uuid, growth_stage: GrowthStage, cultivator: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species_id,
            growth_stage,
            environment_id: None,
            notes: None,
            photos: Vec::new(),
            recorded_at: Utc::now(),
            cultivator,
        }
    }
}