//! High-level persistent manager summary computation for LLM Wiki.
//! Implements FR-001 through FR-008, Invariants I, IX, X, XI, XIV, and latency budget <= 50 ms.

use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::evolution::conflict::ConflictDetector;
use crate::evolution::profile::{ConstraintRule, DocumentationOrientation};
use crate::evolution::signatures::{detect_all_signatures, ProblemSignatureType};
use crate::graph::builder::GraphTopology;
use crate::model::NodeStatus;

/// High-level persistent summary payload computed in <= 50 ms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManagerSummary {
    pub health: HealthSummary,
    pub structure: StructureSummary,
    pub evolution: EvolutionSummary,
    pub sync: SyncSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthSummary {
    /// Overall Quality Score (0..100)
    pub quality_score: u32,
    /// Breakdown across the 5 canonical dimensions (0..100 each)
    pub integrity: u32,
    pub structure: u32,
    pub consistency: u32,
    pub discoverability: u32,
    pub freshness: u32,
    /// Total actionable findings requiring attention
    pub findings_count: u32,
    /// Specific Integrity sub-counts
    pub broken_references: u32,
    pub invalid_anchors: u32,
    /// Debt tracking (Invariant XI)
    pub legacy_debt: u32,
    pub new_regressions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructureSummary {
    /// Total active Markdown documents (nodes in knowledge base)
    pub total_documents: u32,
    /// Documents participating in 1+ references
    pub connected_documents: u32,
    /// Documents with zero references (unconnected notes)
    pub isolated_documents: u32,
    /// Total valid references (edges in graph)
    pub total_references: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvolutionSummary {
    /// Active orientation preset
    pub orientation: DocumentationOrientation,
    /// Number of detected structural opportunities
    pub opportunity_count: u32,
    /// Summary of what current orientation optimizes (strictly <= 60 chars)
    pub primary_objective: String,
    /// True if configured constraints are mathematically irreconcilable
    pub conflicting_constraints: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncSummary {
    pub status: SyncStatus,
    pub last_commit_sha: Option<String>,
    pub last_commit_time: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Synced,
    Syncing,
    Error,
    Uninitialized,
}

/// Computes the complete 4-card ManagerSummary in <= 50 ms.
pub fn compute_manager_summary(
    topology: &GraphTopology,
    workspace_root: &Path,
    orientation: DocumentationOrientation,
    constraints: &[ConstraintRule],
) -> ManagerSummary {
    // 1. Structure Summary
    let total_documents = topology.nodes.len() as u32;
    let mut connected_count = 0u32;
    let mut isolated_count = 0u32;

    for node in topology.nodes.values() {
        if node.status == NodeStatus::Orphan || (node.in_degree == 0 && node.out_degree == 0) {
            isolated_count += 1;
        } else {
            connected_count += 1;
        }
    }

    let total_references = topology.edges.iter().filter(|e| !e.is_broken).count() as u32;

    let structure = StructureSummary {
        total_documents,
        connected_documents: connected_count,
        isolated_documents: isolated_count,
        total_references,
    };

    // 2. Health Summary
    let mut broken_references = 0u32;
    let mut invalid_anchors = 0u32;

    for edge in &topology.edges {
        if edge.is_broken {
            if edge.target_fragment.is_some() || edge.anchor.is_some() {
                invalid_anchors += 1;
            } else {
                broken_references += 1;
            }
        }
    }

    let baseline = crate::debt::baseline::load_baseline(workspace_root).ok().flatten();
    let legacy_debt = baseline.as_ref().map(|b| b.total_debt).unwrap_or(0);
    let new_regressions = 0u32; // Baseline delta evaluated at pre-commit

    // Calculate or approximate 5-pillar health
    let total_broken = broken_references + invalid_anchors;
    let integrity_score = if total_references + total_broken == 0 {
        100
    } else {
        let ratio = broken_references as f32 / (total_references + total_broken) as f32;
        ((1.0 - ratio) * 100.0).round().clamp(0.0, 100.0) as u32
    };

    let structure_findings: Vec<_> = crate::quality::structure::detect_structure_debt(topology)
        .into_iter()
        .filter(|sf| {
            if sf.debt_type == crate::quality::structure::StructureDebtType::UnanchoredReference {
                !baseline.as_ref().map(|b| b.total_debt > 0).unwrap_or(false)
            } else {
                true
            }
        })
        .collect();
    let structure_score = crate::quality::structure::calculate_structure_score(&structure_findings, topology)
        .round()
        .clamp(0.0, 100.0) as u32;
    let consistency_score = 100u32;
    let discoverability_score = if total_documents == 0 {
        100
    } else {
        let connected_ratio = connected_count as f32 / total_documents as f32;
        (connected_ratio * 100.0).round().clamp(0.0, 100.0) as u32
    };
    let freshness_score = 100u32;

    let quality_score = if total_documents == 0 {
        100
    } else {
        ((integrity_score as f32 * 0.35)
            + (structure_score as f32 * 0.20)
            + (consistency_score as f32 * 0.15)
            + (discoverability_score as f32 * 0.15)
            + (freshness_score as f32 * 0.15))
            .round()
            .clamp(0.0, 100.0) as u32
    };

    let findings_count = total_broken;

    let health = HealthSummary {
        quality_score,
        integrity: integrity_score,
        structure: structure_score,
        consistency: consistency_score,
        discoverability: discoverability_score,
        freshness: freshness_score,
        findings_count,
        broken_references,
        invalid_anchors,
        legacy_debt,
        new_regressions,
    };

    // 3. Evolution Summary
    let target_fragment_size = match orientation {
        DocumentationOrientation::AiOriented => 3000,
        DocumentationOrientation::HumanOriented => 8000,
        DocumentationOrientation::Balanced => 5000,
        DocumentationOrientation::Custom => 4000,
    };

    let nodes_vec: Vec<crate::model::NoteNode> = topology.nodes.values().cloned().collect();
    let signatures = detect_all_signatures(&nodes_vec, target_fragment_size);
    let opportunity_count = signatures
        .iter()
        .filter(|s| s.problem_type != ProblemSignatureType::HealthyMonolith)
        .count() as u32;

    let primary_objective = match orientation {
        DocumentationOrientation::AiOriented => {
            "Optimizing AI discovery, context efficiency & reuse".to_string()
        }
        DocumentationOrientation::HumanOriented => {
            "Prioritizing human reading, narrative & navigation".to_string()
        }
        DocumentationOrientation::Balanced => {
            "Balancing AI retrieval efficiency with human comprehension".to_string()
        }
        DocumentationOrientation::Custom => {
            "Custom objectives, engine parameters & constraints".to_string()
        }
    };

    let detector = ConflictDetector::new();
    let (is_valid, _) = detector.validate(constraints);
    let conflicting_constraints = !is_valid;

    let evolution = EvolutionSummary {
        orientation,
        opportunity_count,
        primary_objective,
        conflicting_constraints,
    };

    // 4. Sync Summary
    let git_dir = workspace_root.join(".git");
    let (status, last_commit_sha) = if git_dir.exists() {
        if let Some(sha) = crate::cli::baseline_cmd::get_current_git_commit(workspace_root) {
            let short_sha = if sha.len() >= 7 {
                sha[..7].to_string()
            } else {
                sha
            };
            (SyncStatus::Synced, Some(short_sha))
        } else {
            (SyncStatus::Uninitialized, None)
        }
    } else {
        (SyncStatus::Uninitialized, None)
    };

    let sync = SyncSummary {
        status,
        last_commit_sha,
        last_commit_time: None,
    };

    ManagerSummary {
        health,
        structure,
        evolution,
        sync,
    }
}
