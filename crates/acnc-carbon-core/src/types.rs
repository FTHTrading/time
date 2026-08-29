//! Core deterministic integer types, evidence statuses, and record models.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Integer units (no floating-point drift).
pub type GramsCo2e = i64;
pub type WattHours = i64;
pub type Meters = i64;
pub type BasisPoints = i64; // 10,000 bps = 100.00%
pub type UnixSeconds = i64;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CarbonError {
    #[error("Invalid input values or out-of-range parameters")]
    InvalidInput,
    #[error("Mathematical overflow during integer calculation")]
    Overflow,
    #[error("Baseline window is insufficient or invalid")]
    InvalidBaseline,
    #[error("Factor not found or expired for geography/vintage")]
    FactorNotFound,
    #[error("Evidence validation threshold not met")]
    InsufficientEvidence,
    #[error("Daily or category cap exceeded")]
    CapExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceStatus {
    Unverified,
    UserEntered,
    Estimated,
    ReceiptBacked,
    Metered,
    Attested,
    RegistryVerified,
}

impl EvidenceStatus {
    /// Returns basis points multiplier for reward/credit calculations (e.g. 8000 = 0.8x).
    pub fn evidence_multiplier_bps(&self) -> BasisPoints {
        match self {
            EvidenceStatus::Unverified => 0,
            EvidenceStatus::UserEntered => 2_000,
            EvidenceStatus::Estimated => 3_000,
            EvidenceStatus::ReceiptBacked => 8_000,
            EvidenceStatus::Metered => 10_000,
            EvidenceStatus::Attested => 10_000,
            EvidenceStatus::RegistryVerified => 10_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmissionFactor {
    pub factor_id: String,
    pub geography: String,
    pub source_uri: String,
    pub version: String,
    /// Stored in Grams CO2e per primary unit (e.g. per kWh or per mile).
    pub grams_co2e_per_unit: i64,
    pub valid_from: UnixSeconds,
    pub valid_to: Option<UnixSeconds>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub record_id: String,
    pub subject_id_hash: [u8; 32],
    pub category: String,
    pub quantity: i64,
    pub unit: String,
    pub period_start: UnixSeconds,
    pub period_end: UnixSeconds,
    pub evidence_status: EvidenceStatus,
    pub evidence_hash: [u8; 32],
    pub methodology_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FootprintEstimate {
    pub estimate_id: String,
    pub activity_record_id: String,
    pub gross_grams_co2e: GramsCo2e,
    pub factor_id: String,
    pub factor_version: String,
    pub confidence_score_bps: BasisPoints,
    pub calculated_at: UnixSeconds,
    pub evidence_hash: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReductionRecord {
    pub reduction_id: String,
    pub project_id: Option<String>,
    pub baseline_grams_co2e: GramsCo2e,
    pub observed_grams_co2e: GramsCo2e,
    pub gross_reduction_grams_co2e: GramsCo2e,
    pub leakage_grams_co2e: GramsCo2e,
    pub uncertainty_discount_bps: BasisPoints,
    pub conservative_reduction_grams_co2e: GramsCo2e,
    pub evidence_status: EvidenceStatus,
    pub period_start: UnixSeconds,
    pub period_end: UnixSeconds,
    pub methodology_version: String,
    pub evidence_root: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactAttestation {
    pub attestation_id: String,
    pub target_record_id: String,
    pub attester_id: String,
    pub methodology_version: String,
    pub evidence_hash: [u8; 32],
    pub approved_points: i64,
    pub eligible_vtime: i64, // Micro-VTIME (1e6)
    pub signed_at: UnixSeconds,
    pub signature_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRetirementRecord {
    pub record_id: String,
    pub registry: String,
    pub serial_number: String,
    pub project_id: String,
    pub vintage_year: u32,
    pub metric_tonnes_co2e: i64,
    pub retirement_date: UnixSeconds,
    pub evidence_uri: String,
    pub evidence_hash: [u8; 32],
    pub verified_by_attester: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CandidateStatus {
    Draft,
    EvidenceComplete,
    AwaitingIndependentValidation,
    Validated,
    VerifiedForRegistrySubmission,
    RegistryIssued,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReductionCandidate {
    pub candidate_id: String,
    pub project_id: String,
    pub period_start: UnixSeconds,
    pub period_end: UnixSeconds,
    pub baseline_grams_co2e: GramsCo2e,
    pub observed_grams_co2e: GramsCo2e,
    pub leakage_grams_co2e: GramsCo2e,
    pub uncertainty_discount_bps: BasisPoints,
    pub conservative_reduction_grams_co2e: GramsCo2e,
    pub methodology_version: String,
    pub evidence_root: [u8; 32],
    pub status: CandidateStatus,
}
