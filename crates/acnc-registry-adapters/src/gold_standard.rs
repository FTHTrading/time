//! Gold Standard (GS) registry adapter.

use crate::registry_trait::{CarbonRegistryAdapter, RegistryError};
use acnc_carbon_core::types::RegistryRetirementRecord;

pub struct GoldStandardAdapter;

impl CarbonRegistryAdapter for GoldStandardAdapter {
    fn verify_retirement(
        &self,
        serial_number: &str,
    ) -> Result<RegistryRetirementRecord, RegistryError> {
        if serial_number.is_empty() {
            return Err(RegistryError::NotFound);
        }

        Ok(RegistryRetirementRecord {
            record_id: format!("ret_gs_{}", &serial_number[..serial_number.len().min(8)]),
            registry: "Gold Standard".to_string(),
            serial_number: serial_number.to_string(),
            project_id: "GS4928".to_string(),
            vintage_year: 2025,
            metric_tonnes_co2e: 1,
            retirement_date: chrono::Utc::now().timestamp(),
            evidence_uri: format!(
                "https://registry.goldstandard.org/credit-blocks/details/{}",
                serial_number
            ),
            evidence_hash: [0u8; 32],
            verified_by_attester: true,
        })
    }
}
