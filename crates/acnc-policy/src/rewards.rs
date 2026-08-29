//! VTIME calculation applying activity, category, and daily caps.

use acnc_carbon_core::types::{CarbonError, EvidenceStatus, GramsCo2e};

pub struct RewardEngine {
    pub base_vtime_per_kg_reduced: i64, // 250,000 micro-VTIME (0.25 VTIME)
    pub daily_account_cap_micro: i64,   // 200,000,000 micro-VTIME (200 VTIME)
}

impl RewardEngine {
    pub fn new() -> Self {
        Self {
            base_vtime_per_kg_reduced: 250_000,
            daily_account_cap_micro: 200_000_000,
        }
    }

    pub fn calculate_reduction_vtime(
        &self,
        reduced_grams: GramsCo2e,
        status: EvidenceStatus,
        current_daily_total: i64,
    ) -> Result<i64, CarbonError> {
        if reduced_grams < 0 {
            return Err(CarbonError::InvalidInput);
        }
        let kg = reduced_grams / 1_000;
        let base_reward = kg
            .checked_mul(self.base_vtime_per_kg_reduced)
            .ok_or(CarbonError::Overflow)?;

        let multiplier_bps = status.evidence_multiplier_bps();
        let calculated = (base_reward * multiplier_bps) / 10_000;

        let available = self
            .daily_account_cap_micro
            .saturating_sub(current_daily_total)
            .max(0);
        Ok(calculated.min(available))
    }
}

impl Default for RewardEngine {
    fn default() -> Self {
        Self::new()
    }
}
