//! Confidence scores and uncertainty deductions.
//!
//! Uncertainty discounts should be methodology-approved and context-specific,
//! not universal. These defaults are conservative starting points; a qualified
//! methodology advisor must confirm appropriate deductions per project and geography.

use crate::types::{BasisPoints, EvidenceStatus};

/// Recommend a default uncertainty deduction based on evidence provenance.
/// These are conservative defaults — actual deductions must be confirmed
/// by methodology-specific rules and expert review.
pub fn recommended_uncertainty_bps(status: EvidenceStatus) -> BasisPoints {
    match status {
        EvidenceStatus::Metered => 500,                // 5% uncertainty
        EvidenceStatus::Attested => 500,               // 5% uncertainty
        EvidenceStatus::Validated => 500,              // 5% uncertainty
        EvidenceStatus::PendingVvbVerification => 500, // 5% uncertainty
        EvidenceStatus::RegistryIssued => 0,           // 0% — registry-confirmed
        EvidenceStatus::RegistryRetired => 0,          // 0% — registry-confirmed
        EvidenceStatus::ReceiptBacked => 1_500,        // 15% uncertainty
        EvidenceStatus::Estimated => 3_500,            // 35% uncertainty
        EvidenceStatus::LocalRecord => 10_000,         // 100% deduction
    }
}
