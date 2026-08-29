//! ALL COUCH NO CAGE — Third-Party Carbon Registry Adapters

pub mod gold_standard;
pub mod puro_earth;
pub mod registry_trait;
pub mod verra_vcs;

pub use gold_standard::GoldStandardAdapter;
pub use puro_earth::PuroEarthAdapter;
pub use registry_trait::{CarbonRegistryAdapter, RegistryError};
pub use verra_vcs::VerraVcsAdapter;
