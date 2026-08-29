//! Additionality assessment checklist (evidence compilation, not automated legal determination).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalityChecklist {
    pub project_id: String,
    pub regulatory_surplus_verified: bool,
    pub financial_barrier_documented: bool,
    pub common_practice_analysis_attached: bool,
    pub notes: String,
}
