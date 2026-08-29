//! EIP-712 structured claim payload generation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EIP712RewardClaim {
    pub recipient: String,
    pub amount_micro_vtime: i64,
    pub receipt_id: [u8; 32],
    pub evidence_hash: [u8; 32],
    pub nonce: u64,
    pub deadline: i64,
}
