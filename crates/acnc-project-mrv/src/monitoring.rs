//! Monitoring-period aggregation and reporting.

use acnc_carbon_core::types::GramsCo2e;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringPeriodReport {
    pub report_id: String,
    pub project_id: String,
    pub period_index: u32,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub total_baseline_grams: GramsCo2e,
    pub total_observed_grams: GramsCo2e,
    pub total_leakage_grams: GramsCo2e,
    pub net_conservative_reduction_grams: GramsCo2e,
    pub participant_count: usize,
}
