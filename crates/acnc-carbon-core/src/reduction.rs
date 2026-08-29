//! Conservative reduction calculations applying leakage and uncertainty discounts.

use crate::types::{BasisPoints, CarbonError, GramsCo2e};

/// Calculate conservative reduction:
/// Gross = max(0, Baseline - Observed - Leakage)
/// Net Conservative = Gross * (10,000 - Uncertainty_Bps) / 10,000
pub fn calculate_conservative_reduction(
    baseline: GramsCo2e,
    observed: GramsCo2e,
    leakage: GramsCo2e,
    uncertainty_discount_bps: BasisPoints,
) -> Result<GramsCo2e, CarbonError> {
    if !(0..=10_000).contains(&uncertainty_discount_bps) {
        return Err(CarbonError::InvalidInput);
    }
    if baseline < 0 || observed < 0 || leakage < 0 {
        return Err(CarbonError::InvalidInput);
    }

    let gross = baseline
        .saturating_sub(observed)
        .saturating_sub(leakage)
        .max(0);

    gross
        .checked_mul(10_000 - uncertainty_discount_bps)
        .ok_or(CarbonError::Overflow)
        .map(|value| value / 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservative_reduction() {
        // Baseline: 1,000,000 g (1,000 kg)
        // Observed: 600,000 g (600 kg)
        // Leakage: 50,000 g (50 kg)
        // Gross = 350,000 g
        // Uncertainty discount: 1000 bps (10%)
        // Result = 350,000 * 0.90 = 315,000 g (315 kg)
        let res = calculate_conservative_reduction(1_000_000, 600_000, 50_000, 1_000).unwrap();
        assert_eq!(res, 315_000);
    }
}
