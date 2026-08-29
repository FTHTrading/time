//! Baseline models and comparative historical window assessment.

use crate::types::{CarbonError, GramsCo2e, UnixSeconds};

pub struct BaselineWindow {
    pub start: UnixSeconds,
    pub end: UnixSeconds,
    pub baseline_grams_co2e: GramsCo2e,
}

impl BaselineWindow {
    pub fn new(
        start: UnixSeconds,
        end: UnixSeconds,
        baseline_grams_co2e: GramsCo2e,
    ) -> Result<Self, CarbonError> {
        if start >= end || baseline_grams_co2e < 0 {
            return Err(CarbonError::InvalidBaseline);
        }
        Ok(Self {
            start,
            end,
            baseline_grams_co2e,
        })
    }

    /// Calculate gross delta against observed period of equal duration.
    pub fn calculate_gross_delta(
        &self,
        observed_grams_co2e: GramsCo2e,
    ) -> Result<GramsCo2e, CarbonError> {
        if observed_grams_co2e < 0 {
            return Err(CarbonError::InvalidInput);
        }
        Ok(self.baseline_grams_co2e.saturating_sub(observed_grams_co2e).max(0))
    }
}
