//! ALL COUCH NO CAGE — Deterministic Carbon & Accounting Core Engine

pub mod baseline;
pub mod caps;
pub mod factors;
pub mod footprint;
pub mod reduction;
pub mod types;
pub mod uncertainty;

pub use baseline::BaselineWindow;
pub use caps::*;
pub use factors::FactorRegistry;
pub use footprint::*;
pub use reduction::calculate_conservative_reduction;
pub use types::*;
pub use uncertainty::recommended_uncertainty_bps;
