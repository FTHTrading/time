//! Activity-to-CO2e calculation engine with integer precision.

use crate::types::{CarbonError, GramsCo2e, Meters, WattHours};

/// Calculate emissions from electricity consumption in WattHours.
/// factor_grams_per_kwh: e.g. 385 grams per kWh.
/// Result in Grams CO2e.
pub fn electricity_co2e_grams(
    watt_hours: WattHours,
    factor_grams_per_kwh: GramsCo2e,
) -> Result<GramsCo2e, CarbonError> {
    if watt_hours < 0 || factor_grams_per_kwh < 0 {
        return Err(CarbonError::InvalidInput);
    }
    watt_hours
        .checked_mul(factor_grams_per_kwh)
        .ok_or(CarbonError::Overflow)
        .map(|value| value / 1_000)
}

/// Calculate emissions from passenger travel distance.
/// distance_meters: Distance in meters.
/// factor_grams_per_km: Grams CO2e per kilometer.
pub fn transport_co2e_grams(
    distance_meters: Meters,
    factor_grams_per_km: GramsCo2e,
) -> Result<GramsCo2e, CarbonError> {
    if distance_meters < 0 || factor_grams_per_km < 0 {
        return Err(CarbonError::InvalidInput);
    }
    distance_meters
        .checked_mul(factor_grams_per_km)
        .ok_or(CarbonError::Overflow)
        .map(|value| value / 1_000)
}

/// Generic unit multiplication for quantity and factor.
pub fn generic_co2e_grams(
    quantity_units: i64,
    factor_grams_per_unit: GramsCo2e,
) -> Result<GramsCo2e, CarbonError> {
    if quantity_units < 0 || factor_grams_per_unit < 0 {
        return Err(CarbonError::InvalidInput);
    }
    quantity_units
        .checked_mul(factor_grams_per_unit)
        .ok_or(CarbonError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electricity_co2e_calculation() {
        // 320 kWh = 320,000 WattHours @ 385 grams/kWh = 123,200 grams CO2e (123.20 kg)
        let grams = electricity_co2e_grams(320_000, 385).unwrap();
        assert_eq!(grams, 123_200);
    }

    #[test]
    fn test_transport_co2e_calculation() {
        // 100 km = 100,000 meters @ 251 grams/km = 25,100 grams CO2e (25.10 kg)
        let grams = transport_co2e_grams(100_000, 251).unwrap();
        assert_eq!(grams, 25_100);
    }
}
