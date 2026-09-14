//! Code-to-doc reverse impact index and technical drift detection.

pub mod detector;
pub mod extractor;
pub mod freshness;
pub mod index;

pub use detector::*;
pub use freshness::*;

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Bidirectional reverse impact index mapping code files and symbols to markdown notes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReverseImpactIndex {
    /// Maps code file paths -> set of Markdown doc IDs that cite this path
    pub code_to_docs: HashMap<String, HashSet<String>>,
    /// Maps code symbol names (classes/functions) -> set of doc IDs citing them
    pub symbol_to_docs: HashMap<String, HashSet<String>>,
    /// Maps doc ID -> set of code paths referenced in the doc
    pub doc_to_code: HashMap<String, HashSet<String>>,
}

/// Technical drift diagnostic identifying stale code citations in markdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TechnicalDriftFinding {
    pub doc_id: String,
    pub code_target: String,
    pub symbol_name: Option<String>,
    pub citation_line: u32,
    pub drift_type: DriftType,
    pub confidence: DriftConfidence,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftType {
    DeletedCodeFile,
    RenamedCodeFile,
    DeletedSymbol,
    StaleSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftConfidence {
    High,
    Moderate,
    Low,
}
