//! Permanence and buffer pool allocation.

use acnc_carbon_core::types::{BasisPoints, GramsCo2e};

pub fn calculate_buffer_deduction(
    gross_reduction_grams: GramsCo2e,
    buffer_pool_bps: BasisPoints,
) -> (GramsCo2e, GramsCo2e) {
    let buffer_grams = (gross_reduction_grams * buffer_pool_bps) / 10_000;
    let net_claimable_grams = gross_reduction_grams.saturating_sub(buffer_grams);
    (net_claimable_grams, buffer_grams)
}
