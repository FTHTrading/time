//! Standardized factor registry and versioned factor lookups.

use crate::types::{CarbonError, EmissionFactor, UnixSeconds};
use std::collections::HashMap;

pub struct FactorRegistry {
    factors: HashMap<String, Vec<EmissionFactor>>,
}

impl FactorRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factors: HashMap::new(),
        };
        registry.load_default_factors();
        registry
    }

    pub fn register(&mut self, factor: EmissionFactor) {
        let entry = self.factors.entry(factor.factor_id.clone()).or_default();
        entry.push(factor);
    }

    pub fn lookup(
        &self,
        factor_id: &str,
        timestamp: UnixSeconds,
    ) -> Result<&EmissionFactor, CarbonError> {
        if let Some(list) = self.factors.get(factor_id) {
            for f in list {
                if timestamp >= f.valid_from {
                    if let Some(to) = f.valid_to {
                        if timestamp > to {
                            continue;
                        }
                    }
                    return Ok(f);
                }
            }
        }
        Err(CarbonError::FactorNotFound)
    }

    fn load_default_factors(&mut self) {
        // Electricity US Avg: 385 grams CO2e per kWh (EPA eGRID 2024)
        self.register(EmissionFactor {
            factor_id: "electricity_us_grid".to_string(),
            geography: "US".to_string(),
            source_uri: "https://www.epa.gov/egrid".to_string(),
            version: "2026.1".to_string(),
            grams_co2e_per_unit: 385,
            valid_from: 1704067200, // 2024-01-01
            valid_to: None,
        });

        // Gasoline Car: 404 grams CO2e per passenger-mile (EPA GHG Hub)
        self.register(EmissionFactor {
            factor_id: "gasoline_passenger_car_mile".to_string(),
            geography: "US".to_string(),
            source_uri: "https://www.epa.gov/climateleadership/ghg-emission-factors-hub".to_string(),
            version: "2026.1".to_string(),
            grams_co2e_per_unit: 404,
            valid_from: 1704067200,
            valid_to: None,
        });

        // Transit Bus: 140 grams CO2e per passenger-mile (DOT FTA)
        self.register(EmissionFactor {
            factor_id: "transit_bus_passenger_mile".to_string(),
            geography: "US".to_string(),
            source_uri: "https://www.transit.dot.gov/ntd".to_string(),
            version: "2026.1".to_string(),
            grams_co2e_per_unit: 140,
            valid_from: 1704067200,
            valid_to: None,
        });

        // Food Waste to Landfill: 2,500 grams CO2e per kg (EPA WARM v15)
        self.register(EmissionFactor {
            factor_id: "food_waste_landfill_kg".to_string(),
            geography: "GLOBAL".to_string(),
            source_uri: "https://www.epa.gov/warm".to_string(),
            version: "2026.1".to_string(),
            grams_co2e_per_unit: 2_500,
            valid_from: 1704067200,
            valid_to: None,
        });

        // Cloud GPU Compute: 180 grams CO2e per hour
        self.register(EmissionFactor {
            factor_id: "cloud_gpu_hour".to_string(),
            geography: "GLOBAL".to_string(),
            source_uri: "https://time.unykorn.ai/factors/gpu".to_string(),
            version: "2026.1".to_string(),
            grams_co2e_per_unit: 180,
            valid_from: 1704067200,
            valid_to: None,
        });
    }
}

impl Default for FactorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
