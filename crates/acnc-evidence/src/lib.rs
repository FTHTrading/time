//! ALL COUCH NO CAGE — Cryptographic Evidence & Receipts Engine

pub mod canonical_json;
pub mod hash;
pub mod receipts;
pub mod signatures;

pub use canonical_json::to_canonical_json_bytes;
pub use hash::{compute_sha256, format_evidence_hash_hex};
pub use receipts::{seal_activity_record, SealedEvidenceReceipt};
pub use signatures::EIP712RewardClaim;

#[cfg(test)]
mod tests {
    use super::*;
    use acnc_carbon_core::types::{ActivityRecord, EvidenceStatus};

    #[test]
    fn test_canonical_json_and_sha256() {
        let record = ActivityRecord {
            record_id: "rec_test_123".to_string(),
            participant_id_hash: [0u8; 32],
            category: "electricity".to_string(),
            quantity: 320,
            unit: "kWh".to_string(),
            period_start: 1704067200,
            period_end: 1706745600,
            evidence_status: EvidenceStatus::ReceiptBacked,
            evidence_hash: [0u8; 32],
            methodology_version: "2026.1".to_string(),
        };

        let receipt = seal_activity_record(&record).unwrap();
        assert!(receipt.receipt_id.starts_with("RCPT-ACT-"));
        assert_eq!(receipt.data_status, "ReceiptBacked");
    }
}
