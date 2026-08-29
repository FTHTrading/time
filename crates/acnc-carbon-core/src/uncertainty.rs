//! Confidence scores and uncertainty deductions.

use crate::types::{BasisPoints, EvidenceStatus};

/// Recommend an uncertainty deduction based on evidence provenance.
pub fn recommended_uncertainty_bps(status: EvidenceStatus) -> BasisPoints {
    match status {
        EvidenceStatus::Metered => 500,           // 5% uncertainty
        EvidenceStatus::Attested => 500,          // 5% uncertainty
        EvidenceStatus::RegistryVerified => 0,    // 0% uncertainty
        EvidenceStatus::ReceiptBacked => 1_500,   // 15% uncertainty
        EvidenceStatus::Estimated => 3_500,       // 35% uncertainty
        EvidenceStatus::UserEntered => 5_000,     // 50% uncertainty
        EvidenceStatus::Unverified => 10_000,     // 100% deduction
    }
}
