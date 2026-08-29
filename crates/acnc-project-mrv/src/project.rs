//! Project boundary, methodology classification, and registration metadata.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectClass {
    HomeEnergyRetrofit,
    ApplianceEfficiency,
    CommunitySolar,
    OrganicWasteDiversion,
    BiocharProduction,
    ReforestationRemoval,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectDefinition {
    pub project_id: String,
    pub name: String,
    pub project_class: ProjectClass,
    pub target_registry: String,
    pub methodology_code: String,
    pub country: String,
    pub crediting_period_start: i64,
    pub crediting_period_end: i64,
}
