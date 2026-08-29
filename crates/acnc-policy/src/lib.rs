//! ALL COUCH NO CAGE — Policy Engine & Reward Economics

pub mod anti_fraud;
pub mod audit_log;
pub mod eligibility;
pub mod rewards;

pub use anti_fraud::AntiFraudValidator;
pub use audit_log::PolicyDecisionLog;
pub use eligibility::{get_multiplier_bps, is_reward_eligible};
pub use rewards::RewardEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use acnc_carbon_core::types::EvidenceStatus;

    #[test]
    fn test_reward_engine_and_caps() {
        let engine = RewardEngine::new();
        // 100 kg reduced @ ReceiptBacked (0.8x)
        // Base = 100 * 250,000 = 25,000,000 micro-VTIME (25 VTIME)
        // Multiplier = 0.8x -> 20,000,000 micro-VTIME (20 VTIME)
        let vtime = engine
            .calculate_reduction_vtime(100_000, EvidenceStatus::ReceiptBacked, 0)
            .unwrap();
        assert_eq!(vtime, 20_000_000);
    }

    #[test]
    fn test_anti_fraud_duplicate_check() {
        let mut validator = AntiFraudValidator::new();
        let hash = [42u8; 32];
        assert!(validator.validate_and_record_hash(hash).is_ok());
        assert!(validator.validate_and_record_hash(hash).is_err());
    }
}
