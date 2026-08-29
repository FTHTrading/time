//! Policy caps and issuance limits.

use crate::types::{CarbonError, GramsCo2e};

pub const DAILY_FOCUS_CAP_VTIME: i64 = 50_000_000; // 50.00 VTIME (micro-units)
pub const DAILY_IMPACT_CAP_VTIME: i64 = 50_000_000; // 50.00 VTIME
pub const DAILY_GLOBAL_CAP_VTIME: i64 = 200_000_000; // 200.00 VTIME

pub fn enforce_daily_cap(current_daily_issued: i64, requested: i64) -> Result<i64, CarbonError> {
    if requested < 0 {
        return Err(CarbonError::InvalidInput);
    }
    let available = DAILY_GLOBAL_CAP_VTIME
        .saturating_sub(current_daily_issued)
        .max(0);
    Ok(requested.min(available))
}

pub fn grams_to_vtime_micro(grams: GramsCo2e, multiplier_bps: i64) -> Result<i64, CarbonError> {
    // 1 kg CO2e reduced = 2.5 points = 0.25 VTIME = 250,000 micro-VTIME
    let base_points = (grams / 1_000)
        .checked_mul(250)
        .ok_or(CarbonError::Overflow)?;
    base_points
        .checked_mul(multiplier_bps)
        .ok_or(CarbonError::Overflow)
        .map(|v| v / 10_000)
}
