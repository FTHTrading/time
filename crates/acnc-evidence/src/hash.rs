//! SHA-256 evidence hashing and digest formatting.

use sha2::{Digest, Sha256};

pub fn compute_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn format_evidence_hash_hex(hash: &[u8; 32]) -> String {
    format!("sha256:{}", hex::encode(hash))
}
