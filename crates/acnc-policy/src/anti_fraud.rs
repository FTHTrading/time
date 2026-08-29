//! Duplicate evidence detection and anomaly filtering.

use std::collections::HashSet;

#[derive(Default)]
pub struct AntiFraudValidator {
    processed_hashes: HashSet<[u8; 32]>,
}

impl AntiFraudValidator {
    pub fn new() -> Self {
        Self {
            processed_hashes: HashSet::new(),
        }
    }

    pub fn validate_and_record_hash(&mut self, hash: [u8; 32]) -> Result<(), &'static str> {
        if self.processed_hashes.contains(&hash) {
            return Err("Duplicate evidence hash detected");
        }
        self.processed_hashes.insert(hash);
        Ok(())
    }
}
