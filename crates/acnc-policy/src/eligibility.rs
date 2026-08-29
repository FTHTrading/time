//! Evidence-tier reward eligibility and policy multipliers.

use acnc_carbon_core::types::EvidenceStatus;

pub fn is_reward_eligible(status: EvidenceStatus) -> bool {
    status != EvidenceStatus::LocalRecord
}

pub fn get_multiplier_bps(status: EvidenceStatus) -> i64 {
    status.evidence_multiplier_bps()
}
