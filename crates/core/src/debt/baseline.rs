//! Baseline persistence, differential comparison, ratchet logic, and debt accounting.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::{BrokenLinkSignature, CollisionSignature, DriftSignature, KnowledgeDebtBaseline};
use crate::candidate::DebtAccountingSummary;
use crate::graph::builder::GraphTopology;

pub const BASELINE_RELATIVE_PATH: &str = ".context-vault/baseline.json";

/// Extracts CandidateFindings from a GraphTopology.
pub fn extract_findings_from_topology(
    topology: &GraphTopology,
    collisions: Vec<CollisionSignature>,
    technical_drifts: Vec<DriftSignature>,
) -> CandidateFindings {
    let mut broken_links = Vec::new();
    for edge in &topology.edges {
        if edge.is_broken {
            broken_links.push(BrokenLinkSignature {
                source_path: edge.source.clone(),
                target_ref: edge.target.clone(),
                line_number: edge.line.unwrap_or(1),
            });
        }
    }

    let mut orphan_nodes = HashSet::new();
    for (id, node) in &topology.nodes {
        if node.in_degree == 0 && node.out_degree == 0 {
            orphan_nodes.insert(id.clone());
        }
    }

    CandidateFindings {
        broken_links,
        orphan_nodes,
        collisions,
        technical_drifts,
        unreviewed_docs: Vec::new(),
    }
}

/// Captures an initial KnowledgeDebtBaseline from an unbaselined repository's GraphTopology.
pub fn capture_initial_baseline(
    topology: &GraphTopology,
    commit_sha: &str,
    collisions: Vec<CollisionSignature>,
    technical_drifts: Vec<DriftSignature>,
) -> KnowledgeDebtBaseline {
    let findings = extract_findings_from_topology(topology, collisions, technical_drifts);
    let total_debt = (findings.broken_links.len()
        + findings.orphan_nodes.len()
        + findings.collisions.len()
        + findings.technical_drifts.len()
        + findings.unreviewed_docs.len()) as u32;

    KnowledgeDebtBaseline {
        baseline_commit: commit_sha.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        broken_links: findings.broken_links,
        orphan_nodes: findings.orphan_nodes,
        collisions: findings.collisions,
        technical_drifts: findings.technical_drifts,
        unreviewed_docs: findings.unreviewed_docs,
        total_debt,
    }
}

/// Aggregation of findings discovered in a candidate graph or workspace scan.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CandidateFindings {
    pub broken_links: Vec<BrokenLinkSignature>,
    pub orphan_nodes: HashSet<String>,
    pub collisions: Vec<CollisionSignature>,
    pub technical_drifts: Vec<DriftSignature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unreviewed_docs: Vec<String>,
}

/// Differential evaluation outcome comparing candidate findings against baseline.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DifferentialEvaluation {
    pub new_broken_links: Vec<BrokenLinkSignature>,
    pub new_orphans: HashSet<String>,
    pub new_collisions: Vec<CollisionSignature>,
    pub new_technical_drifts: Vec<DriftSignature>,
    pub new_unreviewed_docs: Vec<String>,

    pub resolved_broken_links: Vec<BrokenLinkSignature>,
    pub resolved_orphans: HashSet<String>,
    pub resolved_collisions: Vec<CollisionSignature>,
    pub resolved_technical_drifts: Vec<DriftSignature>,
    pub resolved_unreviewed_docs: Vec<String>,

    pub summary: DebtAccountingSummary,
}

impl DifferentialEvaluation {
    pub fn has_regressions(&self) -> bool {
        !self.new_broken_links.is_empty()
            || !self.new_orphans.is_empty()
            || !self.new_collisions.is_empty()
            || !self.new_technical_drifts.is_empty()
            || !self.new_unreviewed_docs.is_empty()
    }
}

/// Returns the canonical path to the baseline file.
pub fn get_baseline_path(workspace_root: &Path) -> PathBuf {
    workspace_root.join(BASELINE_RELATIVE_PATH)
}

/// Loads the debt baseline from `.context-vault/baseline.json`. Returns None if it does not exist.
pub fn load_baseline(
    workspace_root: &Path,
) -> Result<Option<KnowledgeDebtBaseline>, std::io::Error> {
    let path = get_baseline_path(workspace_root);
    if !path.is_file() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    let baseline: KnowledgeDebtBaseline = serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(Some(baseline))
}

/// Saves the debt baseline to `.context-vault/baseline.json`.
pub fn save_baseline(
    workspace_root: &Path,
    baseline: &KnowledgeDebtBaseline,
) -> Result<(), std::io::Error> {
    let path = get_baseline_path(workspace_root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(baseline)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(&path, json)
}

/// Computes the differential evaluation between candidate findings and the debt baseline:
/// New Regressions = Candidate Findings \ Baseline Findings
/// Resolved Findings = Baseline Findings \ Candidate Findings
pub fn diff_against_baseline(
    candidate: &CandidateFindings,
    baseline: &KnowledgeDebtBaseline,
) -> DifferentialEvaluation {
    let base_broken: HashSet<&BrokenLinkSignature> = baseline.broken_links.iter().collect();
    let base_orphans: &HashSet<String> = &baseline.orphan_nodes;
    let base_collisions: HashSet<&CollisionSignature> = baseline.collisions.iter().collect();
    let base_drifts: HashSet<&DriftSignature> = baseline.technical_drifts.iter().collect();

    let cand_broken: HashSet<&BrokenLinkSignature> = candidate.broken_links.iter().collect();
    let cand_orphans: &HashSet<String> = &candidate.orphan_nodes;
    let cand_collisions: HashSet<&CollisionSignature> = candidate.collisions.iter().collect();
    let cand_drifts: HashSet<&DriftSignature> = candidate.technical_drifts.iter().collect();

    // New regressions: present in candidate, absent in baseline
    let new_broken_links: Vec<BrokenLinkSignature> = candidate
        .broken_links
        .iter()
        .filter(|sig| !base_broken.contains(sig))
        .cloned()
        .collect();

    let new_orphans: HashSet<String> = candidate
        .orphan_nodes
        .iter()
        .filter(|path| !base_orphans.contains(*path))
        .cloned()
        .collect();

    let new_collisions: Vec<CollisionSignature> = candidate
        .collisions
        .iter()
        .filter(|sig| !base_collisions.contains(sig))
        .cloned()
        .collect();

    let new_technical_drifts: Vec<DriftSignature> = candidate
        .technical_drifts
        .iter()
        .filter(|sig| !base_drifts.contains(sig))
        .cloned()
        .collect();

    // Resolved findings: present in baseline, absent in candidate
    let resolved_broken_links: Vec<BrokenLinkSignature> = baseline
        .broken_links
        .iter()
        .filter(|sig| !cand_broken.contains(sig))
        .cloned()
        .collect();

    let resolved_orphans: HashSet<String> = baseline
        .orphan_nodes
        .iter()
        .filter(|path| !cand_orphans.contains(*path))
        .cloned()
        .collect();

    let resolved_collisions: Vec<CollisionSignature> = baseline
        .collisions
        .iter()
        .filter(|sig| !cand_collisions.contains(sig))
        .cloned()
        .collect();

    let resolved_technical_drifts: Vec<DriftSignature> = baseline
        .technical_drifts
        .iter()
        .filter(|sig| !cand_drifts.contains(sig))
        .cloned()
        .collect();

    let base_unreviewed: HashSet<&String> = baseline.unreviewed_docs.iter().collect();
    let cand_unreviewed: HashSet<&String> = candidate.unreviewed_docs.iter().collect();

    let new_unreviewed_docs: Vec<String> = candidate
        .unreviewed_docs
        .iter()
        .filter(|doc| !base_unreviewed.contains(doc))
        .cloned()
        .collect();

    let resolved_unreviewed_docs: Vec<String> = baseline
        .unreviewed_docs
        .iter()
        .filter(|doc| !cand_unreviewed.contains(doc))
        .cloned()
        .collect();

    let new_debt = (new_broken_links.len()
        + new_orphans.len()
        + new_collisions.len()
        + new_technical_drifts.len()
        + new_unreviewed_docs.len()) as u32;

    let resolved_debt = (resolved_broken_links.len()
        + resolved_orphans.len()
        + resolved_collisions.len()
        + resolved_technical_drifts.len()
        + resolved_unreviewed_docs.len()) as u32;

    let legacy_debt = baseline.total_debt;
    let total_debt = (candidate.broken_links.len()
        + candidate.orphan_nodes.len()
        + candidate.collisions.len()
        + candidate.technical_drifts.len()
        + candidate.unreviewed_docs.len()) as u32;

    DifferentialEvaluation {
        new_broken_links,
        new_orphans,
        new_collisions,
        new_technical_drifts,
        new_unreviewed_docs,
        resolved_broken_links,
        resolved_orphans,
        resolved_collisions,
        resolved_technical_drifts,
        resolved_unreviewed_docs,
        summary: DebtAccountingSummary {
            total_debt,
            legacy_debt,
            new_debt,
            resolved_debt,
            regressions: new_debt,
        },
    }
}

/// Ratchets the baseline down when a commit resolves legacy debt items.
pub fn ratchet_baseline(
    baseline: &mut KnowledgeDebtBaseline,
    diff: &DifferentialEvaluation,
    new_commit_sha: &str,
) {
    if !diff.resolved_broken_links.is_empty() {
        let resolved_set: HashSet<&BrokenLinkSignature> =
            diff.resolved_broken_links.iter().collect();
        baseline
            .broken_links
            .retain(|sig| !resolved_set.contains(sig));
    }

    if !diff.resolved_orphans.is_empty() {
        for resolved in &diff.resolved_orphans {
            baseline.orphan_nodes.remove(resolved);
        }
    }

    if !diff.resolved_collisions.is_empty() {
        let resolved_set: HashSet<&CollisionSignature> = diff.resolved_collisions.iter().collect();
        baseline
            .collisions
            .retain(|sig| !resolved_set.contains(sig));
    }

    if !diff.resolved_technical_drifts.is_empty() {
        let resolved_set: HashSet<&DriftSignature> =
            diff.resolved_technical_drifts.iter().collect();
        baseline
            .technical_drifts
            .retain(|sig| !resolved_set.contains(sig));
    }

    baseline.baseline_commit = new_commit_sha.to_string();
    baseline.timestamp = chrono::Utc::now().to_rfc3339();
    baseline.total_debt = (baseline.broken_links.len()
        + baseline.orphan_nodes.len()
        + baseline.collisions.len()
        + baseline.technical_drifts.len()) as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differential_comparison_and_ratchet() {
        let mut baseline = KnowledgeDebtBaseline::default();
        baseline.baseline_commit = "commit_1".to_string();
        baseline.broken_links.push(BrokenLinkSignature {
            source_path: "notes/a.md".to_string(),
            target_ref: "old_target".to_string(),
            line_number: 10,
        });
        baseline.orphan_nodes.insert("notes/orphan.md".to_string());
        baseline.total_debt = 2;

        // Candidate fixes notes/a.md broken link, but introduces a new collision
        let mut candidate = CandidateFindings::default();
        candidate.orphan_nodes.insert("notes/orphan.md".to_string());
        candidate
            .collisions
            .push(CollisionSignature::new("notes/c.md", "notes/d.md"));

        let diff = diff_against_baseline(&candidate, &baseline);
        assert_eq!(diff.new_collisions.len(), 1);
        assert_eq!(diff.new_broken_links.len(), 0);
        assert_eq!(diff.resolved_broken_links.len(), 1);
        assert_eq!(diff.summary.resolved_debt, 1);
        assert_eq!(diff.summary.new_debt, 1);
        assert_eq!(diff.summary.regressions, 1);

        // Ratchet baseline
        ratchet_baseline(&mut baseline, &diff, "commit_2");
        assert_eq!(baseline.baseline_commit, "commit_2");
        assert_eq!(baseline.broken_links.len(), 0);
        assert_eq!(baseline.orphan_nodes.len(), 1);
        assert_eq!(baseline.total_debt, 1);
    }
}
