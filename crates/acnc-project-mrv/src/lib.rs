//! ALL COUCH NO CAGE — Project-Level MRV Accounting Module

pub mod additionality;
pub mod double_counting;
pub mod leakage;
pub mod monitoring;
pub mod permanence;
pub mod project;

pub use additionality::AdditionalityChecklist;
pub use double_counting::DoubleCountingGuard;
pub use leakage::calculate_leakage;
pub use monitoring::MonitoringPeriodReport;
pub use permanence::calculate_buffer_deduction;
pub use project::{ProjectClass, ProjectDefinition};
