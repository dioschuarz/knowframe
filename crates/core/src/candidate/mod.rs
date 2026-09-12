//! Ephemeral candidate graph overlay and admission control evaluation models.

pub mod overlay;
pub mod evaluator;

pub use evaluator::*;
pub use overlay::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::graph::builder::GraphTopology;
use crate::delta::GitDelta;
use crate::model::{NoteNode, NoteEdge};

/// Ephemeral candidate graph constructed in RAM during pre-commit admission control.
/// Discarded immediately upon hook completion without mutating persistent graph.
pub struct CandidateGraph<'a> {
    /// Reference to persistent committed graph
    pub base_graph: &'a GraphTopology,
    /// Staged Git delta
    pub staged_delta: &'a GitDelta,
    /// In-memory mutated nodes (added or modified)
    pub candidate_nodes: HashMap<String, NoteNode>,
    /// In-memory mutated edges
    pub candidate_edges: Vec<NoteEdge>,
    /// Candidate debt evaluation result
    pub evaluation: CandidateValidationReport,
}

/// Evaluation outcome of pre-commit admission control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateValidationReport {
    pub base_commit: String,
    pub decision: AdmissionDecision,
    pub exit_code: i32,
    pub summary: DebtAccountingSummary,
    pub regressions: Vec<RegressionItem>,
    pub resolved_findings: Vec<ResolvedItem>,
    pub warnings: Vec<WarningItem>,
}

impl Default for CandidateValidationReport {
    fn default() -> Self {
        Self {
            base_commit: String::new(),
            decision: AdmissionDecision::Pass,
            exit_code: 0,
            summary: DebtAccountingSummary::default(),
            regressions: Vec::new(),
            resolved_findings: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionDecision {
    #[default]
    Pass,
    Warn,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DebtAccountingSummary {
    pub total_debt: u32,
    pub legacy_debt: u32,
    pub new_debt: u32,
    pub resolved_debt: u32,
    pub regressions: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionItem {
    pub id: String,
    pub category: RegressionCategory,
    pub severity: RegressionSeverity,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub message: String,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionCategory {
    BrokenReference,
    InvalidHeadingAnchor,
    SemanticCollision,
    TechnicalDriftContradiction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionSeverity {
    Blocker,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedItem {
    pub id: String,
    pub category: String,
    pub file_path: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarningItem {
    pub message: String,
    pub file_path: Option<String>,
}
