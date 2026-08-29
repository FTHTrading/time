//! Core deterministic integer types, evidence statuses, and record models.
//!
//! Status vocabulary (used consistently across site, Rust engine, reports, and contracts):
//!
//! | Status                    | Meaning                                                                         |
//! |---------------------------|---------------------------------------------------------------------------------|
//! | LOCAL_RECORD              | User-created record stored locally or in the ACNC evidence system.              |
//! | ESTIMATED                 | Calculated from declared inputs and a disclosed factor/model.                   |
//! | RECEIPT_BACKED            | Supported by a submitted invoice, receipt, or statement; not yet independently verified. |
//! | METERED                   | Supported by an approved meter, utility, facility, or partner data source.      |
//! | ATTESTED                  | Confirmed by an authorized third party under documented rules.                  |
//! | VALIDATED                 | Reviewed for ACNC program completeness; not a registry-issued unit.             |
//! | PENDING_VVB_VERIFICATION  | Included in a project monitoring package awaiting independent verification.     |
//! | REGISTRY_ISSUED           | Confirmed by the applicable registry through official issued-unit records.      |
//! | REGISTRY_RETIRED          | Confirmed by the applicable registry as retired and unavailable for further use. |

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
    #[error("Transition not permitted: external authority required")]
    ExternalAuthorityRequired,
}

/// Evidence and review status vocabulary.
///
/// IMPORTANT: `RegistryIssued` and `RegistryRetired` are READ-ONLY states.
/// They may only be populated from a confirmed registry reference or API response.
/// The ACNC platform cannot set these statuses through internal review.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvidenceStatus {
    /// User-created record stored locally or in the ACNC evidence system.
    LocalRecord,
    /// Calculated from declared inputs and a disclosed factor/model.
    Estimated,
    /// Supported by a submitted invoice, receipt, or statement; not yet independently verified.
    ReceiptBacked,
    /// Supported by an approved meter, utility, facility, or partner data source.
    Metered,
    /// Confirmed by an authorized third party under documented rules.
    Attested,
    /// Reviewed for ACNC program completeness; not a registry-issued unit.
    Validated,
    /// Included in a project monitoring package awaiting independent verification.
    PendingVvbVerification,
    /// READ-ONLY. Confirmed by the applicable registry through official issued-unit records.
    RegistryIssued,
    /// READ-ONLY. Confirmed by the applicable registry as retired and unavailable for further use.
    RegistryRetired,
}

impl EvidenceStatus {
    /// Returns basis points multiplier for reward/credit calculations (e.g. 8000 = 0.8x).
    pub fn evidence_multiplier_bps(&self) -> BasisPoints {
        match self {
            EvidenceStatus::LocalRecord => 0,
            EvidenceStatus::Estimated => 3_000,
            EvidenceStatus::ReceiptBacked => 8_000,
            EvidenceStatus::Metered => 10_000,
            EvidenceStatus::Attested => 10_000,
            EvidenceStatus::Validated => 10_000,
            EvidenceStatus::PendingVvbVerification => 10_000,
            EvidenceStatus::RegistryIssued => 10_000,
            EvidenceStatus::RegistryRetired => 10_000,
        }
    }

    /// Whether this status can be set by the ACNC platform internally.
    pub fn is_internally_settable(&self) -> bool {
        !matches!(
            self,
            EvidenceStatus::RegistryIssued | EvidenceStatus::RegistryRetired
        )
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
    /// Pseudonymous participant identifier (SHA-256 hash).
    /// Identity data collected only where a registry, payment, anti-fraud,
    /// or benefit-sharing requirement mandates it.
    pub participant_id_hash: [u8; 32],
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

/// Review lifecycle status for a ReductionCandidate.
///
/// IMPORTANT: `RegistryIssued` is READ-ONLY — populated exclusively from
/// a confirmed registry reference. ACNC cannot set this status through
/// internal review or platform action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CandidateStatus {
    Draft,
    EvidenceComplete,
    PendingInternalReview,
    PendingIndependentValidation,
    Validated,
    PendingVvbVerification,
    VerifiedForRegistrySubmission,
    /// READ-ONLY. Set exclusively from registry confirmation.
    RegistryIssued,
    Rejected,
}

impl CandidateStatus {
    /// Whether this status can be set by the ACNC platform.
    pub fn is_internally_settable(&self) -> bool {
        !matches!(self, CandidateStatus::RegistryIssued)
    }
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
