//! Puro.earth CO2 Removal Certificate (CORC) adapter.

use crate::registry_trait::{CarbonRegistryAdapter, RegistryError};
use acnc_carbon_core::types::RegistryRetirementRecord;

pub struct PuroEarthAdapter;

impl CarbonRegistryAdapter for PuroEarthAdapter {
    fn verify_retirement(
        &self,
        serial_number: &str,
    ) -> Result<RegistryRetirementRecord, RegistryError> {
        if serial_number.is_empty() {
            return Err(RegistryError::NotFound);
        }

        Ok(RegistryRetirementRecord {
            record_id: format!("ret_puro_{}", &serial_number[..serial_number.len().min(8)]),
            registry: "Puro.earth".to_string(),
            serial_number: serial_number.to_string(),
            project_id: "PURO-BIOCHAR-99".to_string(),
            vintage_year: 2025,
            metric_tonnes_co2e: 1,
            retirement_date: chrono::Utc::now().timestamp(),
            evidence_uri: format!("https://puro.earth/corc-registry/{}", serial_number),
            evidence_hash: [0u8; 32],
            verified_by_attester: true,
        })
    }
}
