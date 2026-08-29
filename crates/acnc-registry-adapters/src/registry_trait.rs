//! Trait definition for third-party carbon registry retirement verification.

use acnc_carbon_core::types::RegistryRetirementRecord;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Certificate not found in registry")]
    NotFound,
    #[error("Certificate is not marked as permanently retired")]
    NotRetired,
    #[error("API or network failure: {0}")]
    Network(String),
}

pub trait CarbonRegistryAdapter {
    fn verify_retirement(
        &self,
        serial_number: &str,
    ) -> Result<RegistryRetirementRecord, RegistryError>;
}
