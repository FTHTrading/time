//! Anti-double-counting and serial cancellation guards.

use std::collections::HashSet;

#[derive(Default)]
pub struct DoubleCountingGuard {
    used_serials: HashSet<String>,
}

impl DoubleCountingGuard {
    pub fn new() -> Self {
        Self {
            used_serials: HashSet::new(),
        }
    }

    pub fn register_serial(&mut self, serial: &str) -> Result<(), &'static str> {
        if self.used_serials.contains(serial) {
            return Err("Serial already registered / redeemed");
        }
        self.used_serials.insert(serial.to_string());
        Ok(())
    }
}
