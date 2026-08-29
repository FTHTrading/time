//! Append-only policy decision logging.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyDecisionLog {
    pub decision_id: String,
    pub target_id: String,
    pub rule_name: String,
    pub status: String,
    pub timestamp: i64,
}
