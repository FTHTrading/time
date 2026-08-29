//! Evidence receipts generation and validation.

use crate::canonical_json::to_canonical_json_bytes;
use crate::hash::compute_sha256;
use acnc_carbon_core::types::ActivityRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealedEvidenceReceipt {
    pub receipt_id: String,
    pub record_canonical_hash: [u8; 32],
    pub data_status: String,
    pub timestamp: i64,
}

pub fn seal_activity_record(record: &ActivityRecord) -> Result<SealedEvidenceReceipt, String> {
    let canonical_bytes = to_canonical_json_bytes(record).map_err(|e| e.to_string())?;
    let hash = compute_sha256(&canonical_bytes);
    Ok(SealedEvidenceReceipt {
        receipt_id: format!("RCPT-ACT-{}", hex::encode(&hash[0..4]).to_uppercase()),
        record_canonical_hash: hash,
        data_status: format!("{:?}", record.evidence_status),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
