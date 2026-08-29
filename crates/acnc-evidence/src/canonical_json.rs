//! Deterministic canonical JSON serialization.

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanonicalJsonError {
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Convert any serializable struct to sorted canonical JSON bytes.
pub fn to_canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalJsonError> {
    let raw_value = serde_json::to_value(value)?;
    let sorted_value = sort_json_value(raw_value);
    Ok(serde_json::to_vec(&sorted_value)?)
}

fn sort_json_value(val: serde_json::Value) -> serde_json::Value {
    match val {
        serde_json::Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                if let Some(v) = map.get(&k) {
                    sorted.insert(k, sort_json_value(v.clone()));
                }
            }
            serde_json::Value::Object(sorted)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(sort_json_value).collect())
        }
        other => other,
    }
}
