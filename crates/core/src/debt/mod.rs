//! Brownfield Knowledge Debt Baseline structures and stable signatures.

pub mod baseline;
pub mod signatures;

pub use baseline::*;
pub use signatures::*;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Persistent snapshot of legacy debt established on repository onboarding or baseline reset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeDebtBaseline {
    /// Commit hash when this baseline was created
    pub baseline_commit: String,
    /// RFC 3339 timestamp of baseline generation
    pub timestamp: String,
    /// Known legacy broken link signatures
    pub broken_links: Vec<BrokenLinkSignature>,
    /// Known legacy orphan document paths
    pub orphan_nodes: HashSet<String>,
    /// Known legacy semantic collision pairs
    pub collisions: Vec<CollisionSignature>,
    /// Known legacy technical drift findings
    pub technical_drifts: Vec<DriftSignature>,
    /// Known legacy unreviewed documentation obligations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unreviewed_docs: Vec<String>,
    /// Aggregate legacy debt counter
    pub total_debt: u32,
}

impl Default for KnowledgeDebtBaseline {
    fn default() -> Self {
        Self {
            baseline_commit: String::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            broken_links: Vec::new(),
            orphan_nodes: HashSet::new(),
            collisions: Vec::new(),
            technical_drifts: Vec::new(),
            unreviewed_docs: Vec::new(),
            total_debt: 0,
        }
    }
}

/// Brownfield debt accounting summary across all 5 pillars.
/// Invariant IX: Complete integer representations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DebtSummary {
    pub total_debt: u32,
    pub legacy_debt: u32,
    pub new_debt: u32,
    pub resolved_debt: u32,
    pub regressions: u32,
}

/// Stable signature identifying a broken link independent of file reordering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BrokenLinkSignature {
    pub source_path: String,
    pub target_ref: String,
    pub line_number: u32,
}

/// Stable signature identifying an unordered pair of colliding documents.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollisionSignature {
    pub doc_a: String,
    pub doc_b: String,
}

impl CollisionSignature {
    pub fn new(a: impl Into<String>, b: impl Into<String>) -> Self {
        let mut a = a.into();
        let mut b = b.into();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        Self { doc_a: a, doc_b: b }
    }
}

/// Stable signature identifying an unaddressed code-to-doc technical drift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DriftSignature {
    pub doc_path: String,
    pub referenced_code_path: String,
    pub symbol: String,
}
