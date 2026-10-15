//! Epistemic tier tagging and cryptographic analysis identification.

pub mod analysis_id;
pub mod classifier;

pub use analysis_id::{compute_analysis_id, METHODOLOGY_VERSION};
pub use classifier::EpistemicClassifier;
