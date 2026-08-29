//! Leakage deductions and displacement calculations.

use acnc_carbon_core::types::GramsCo2e;

pub fn calculate_leakage(activity_grams: GramsCo2e, leakage_factor_bps: i64) -> GramsCo2e {
    (activity_grams * leakage_factor_bps) / 10_000
}
