//! Admission decision engine comparing candidate graph findings against debt baseline.
//! Guarantees ephemeral in-memory evaluation without mutating persistent graph topology.

use std::collections::HashSet;

use super::overlay::CandidateOverlay;
use super::{
    AdmissionDecision, CandidateValidationReport, RegressionCategory, RegressionItem,
    RegressionSeverity, ResolvedItem, WarningItem,
};
use crate::debt::baseline::{diff_against_baseline, CandidateFindings};
use crate::debt::{BrokenLinkSignature, CollisionSignature, DriftSignature, KnowledgeDebtBaseline};
use crate::graph::resolver::{resolve_edge_integrity, ResolverIndex};

/// Configuration controlling admission gate blocking behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionConfig {
    pub block_on_broken_links: bool,
    pub block_on_duplicate_collision: bool,
    pub block_on_technical_drift: bool,
    pub block_on_unreviewed_docs: bool,
    pub block_on_legacy_debt: bool,
}

impl Default for AdmissionConfig {
    fn default() -> Self {
        Self {
            block_on_broken_links: true,
            block_on_duplicate_collision: true,
            block_on_technical_drift: false, // default WARN
            block_on_unreviewed_docs: true,  // default BLOCK per FR-007
            block_on_legacy_debt: false,     // default FALSE per Invariant XI and FR-009
        }
    }
}

/// Evaluates candidate overlay against debt baseline.
/// Completely drops candidate graph memory after execution.
pub fn evaluate_candidate(
    overlay: &CandidateOverlay,
    baseline: &KnowledgeDebtBaseline,
    collisions: Vec<CollisionSignature>,
    technical_drifts: Vec<DriftSignature>,
    config: AdmissionConfig,
) -> CandidateValidationReport {
    let known_nodes = overlay.collect_known_nodes();
    let headings_map = overlay.collect_headings_map();
    let resolver_index = ResolverIndex::new(&known_nodes);

    let mut candidate_broken_links = Vec::new();
    let mut in_degrees: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut out_degrees: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    // Initialize degrees for all surviving candidate nodes
    for id in known_nodes.keys() {
        in_degrees.insert(id.clone(), 0);
        out_degrees.insert(id.clone(), 0);
    }

    // 1. Evaluate base edges that were NOT replaced by staged mutations
    for edge in &overlay.base_graph.edges {
        if overlay.overlay.deleted_nodes.contains(&edge.source)
            || overlay.overlay.mutated_nodes.contains_key(&edge.source)
        {
            continue;
        }

        let mut candidate_edge = edge.clone();
        resolve_edge_integrity(&mut candidate_edge, &resolver_index, &headings_map);

        if candidate_edge.is_broken {
            let target_ref = if candidate_edge.resolution_status
                == Some(crate::model::LinkResolutionStatus::BrokenFragmentAnchor)
            {
                if let Some(ref a) = candidate_edge
                    .anchor
                    .as_ref()
                    .or(candidate_edge.target_fragment.as_ref())
                {
                    format!("{}#{}", candidate_edge.target, a.trim_start_matches('#'))
                } else {
                    candidate_edge.target.clone()
                }
            } else {
                candidate_edge.target.clone()
            };
            candidate_broken_links.push(BrokenLinkSignature {
                source_path: candidate_edge.source.clone(),
                target_ref,
                line_number: candidate_edge.line.unwrap_or(1),
            });
        } else {
            *out_degrees
                .entry(candidate_edge.source.clone())
                .or_insert(0) += 1;
            *in_degrees.entry(candidate_edge.target.clone()).or_insert(0) += 1;
        }
    }

    // 2. Evaluate staged mutated edges
    for edge in &overlay.overlay.mutated_edges {
        let mut candidate_edge = edge.clone();
        resolve_edge_integrity(&mut candidate_edge, &resolver_index, &headings_map);

        if candidate_edge.is_broken {
            let target_ref = if candidate_edge.resolution_status
                == Some(crate::model::LinkResolutionStatus::BrokenFragmentAnchor)
            {
                if let Some(ref a) = candidate_edge
                    .anchor
                    .as_ref()
                    .or(candidate_edge.target_fragment.as_ref())
                {
                    format!("{}#{}", candidate_edge.target, a.trim_start_matches('#'))
                } else {
                    candidate_edge.target.clone()
                }
            } else {
                candidate_edge.target.clone()
            };
            candidate_broken_links.push(BrokenLinkSignature {
                source_path: candidate_edge.source.clone(),
                target_ref,
                line_number: candidate_edge.line.unwrap_or(1),
            });
        } else {
            *out_degrees
                .entry(candidate_edge.source.clone())
                .or_insert(0) += 1;
            *in_degrees.entry(candidate_edge.target.clone()).or_insert(0) += 1;
        }
    }

    // 3. Compute candidate orphan nodes
    let mut candidate_orphans = HashSet::new();
    for id in known_nodes.keys() {
        let in_deg = in_degrees.get(id).copied().unwrap_or(0);
        let out_deg = out_degrees.get(id).copied().unwrap_or(0);
        if in_deg == 0 && out_deg == 0 {
            candidate_orphans.insert(id.clone());
        }
    }

    // Run collision detection if not explicitly passed
    let (detected_collisions, detected_sigs, collision_warnings) = if collisions.is_empty() {
        crate::collision::detect_collisions_in_candidate(overlay, overlay.workspace_root)
    } else {
        (Vec::new(), collisions.clone(), Vec::new())
    };

    let all_collisions = if collisions.is_empty() {
        detected_sigs
    } else {
        collisions
    };

    // Run technical drift detection if not explicitly passed
    let (detected_drifts, detected_drift_sigs) = if technical_drifts.is_empty() {
        let doc_paths: Vec<String> = known_nodes.keys().cloned().collect();
        let reverse_index = crate::drift::detector::build_reverse_impact_index(overlay.workspace_root, &doc_paths);
        crate::drift::detector::detect_technical_drift(
            overlay.workspace_root,
            overlay.staged_delta,
            &reverse_index,
        )
    } else {
        (Vec::new(), technical_drifts.clone())
    };

    let all_drifts = if technical_drifts.is_empty() {
        detected_drift_sigs
    } else {
        technical_drifts
    };

    // Run review obligations calculation
    let unreviewed_docs: Vec<String> = {
        let doc_paths: Vec<String> = known_nodes.keys().cloned().collect();
        let reverse_index = crate::drift::detector::build_reverse_impact_index(overlay.workspace_root, &doc_paths);
        let gating_config = crate::model::ReviewGatingConfig {
            enforce_doc_review: config.block_on_unreviewed_docs,
            block_on_legacy_debt: config.block_on_legacy_debt,
        };
        let affected = crate::drift::review::calculate_affected_documents(
            overlay.workspace_root,
            &overlay.staged_delta.items,
            &reverse_index,
            Some(&gating_config),
            None,
            None,
        );
        affected
            .obligations
            .into_iter()
            .filter(|o| o.status == crate::model::ReviewStatus::Blocked || o.status == crate::model::ReviewStatus::Pending)
            .map(|o| o.doc_id)
            .collect()
    };

    let candidate_findings = CandidateFindings {
        broken_links: candidate_broken_links,
        orphan_nodes: candidate_orphans,
        collisions: all_collisions,
        technical_drifts: all_drifts,
        unreviewed_docs,
    };

    // 4. Differential evaluation against baseline
    let diff = diff_against_baseline(&candidate_findings, baseline);

    let mut regressions = Vec::new();
    let mut warnings = Vec::new();
    let mut has_blocker = false;

    // Add moderate collision warnings
    warnings.extend(collision_warnings);

    // Evaluate broken links regressions
    for bl in &diff.new_broken_links {
        let severity = if config.block_on_broken_links {
            has_blocker = true;
            RegressionSeverity::Blocker
        } else {
            RegressionSeverity::Warning
        };

        let is_fragment_anchor = bl.target_ref.contains('#');
        let (category, message, remediation) = if is_fragment_anchor {
            (
                RegressionCategory::InvalidHeadingAnchor,
                format!("Broken fragment anchor in link to '{}'", bl.target_ref),
                format!(
                    "Fix or restore target heading anchor in '[{}]' from '{}'",
                    bl.target_ref, bl.source_path
                ),
            )
        } else {
            (
                RegressionCategory::BrokenReference,
                format!("Broken link to '{}'", bl.target_ref),
                format!(
                    "Fix or remove target reference '[{}]' in '{}'",
                    bl.target_ref, bl.source_path
                ),
            )
        };

        regressions.push(RegressionItem {
            id: format!("broken-link-{}", bl.source_path),
            category,
            severity,
            file_path: bl.source_path.clone(),
            line_number: Some(bl.line_number),
            message,
            remediation_hint: remediation,
        });
    }

    // Evaluate collision regressions
    for col in &diff.new_collisions {
        let severity = if config.block_on_duplicate_collision {
            has_blocker = true;
            RegressionSeverity::Blocker
        } else {
            RegressionSeverity::Warning
        };

        let diagnostic = detected_collisions.iter().find(|c| {
            (c.source_id == col.doc_a && c.target_id == col.doc_b)
                || (c.source_id == col.doc_b && c.target_id == col.doc_a)
        });

        let (message, remediation) = if let Some(diag) = diagnostic {
            let canon = diag.canonical_candidate.as_deref().unwrap_or(&col.doc_b);
            (
                format!(
                    "High-confidence semantic collision ({:.1}%, {:?}) between '{}' and '{}'",
                    diag.composite_similarity * 100.0,
                    diag.classification,
                    col.doc_a,
                    col.doc_b
                ),
                format!(
                    "Consolidate content into proposed canonical note '{}' (confidence: {:.0}%) or differentiate scope.",
                    canon,
                    diag.canonical_confidence * 100.0
                ),
            )
        } else {
            (
                format!(
                    "Semantic collision between '{}' and '{}'",
                    col.doc_a, col.doc_b
                ),
                format!(
                    "Consolidate or differentiate content with canonical note '{}'",
                    col.doc_b
                ),
            )
        };

        regressions.push(RegressionItem {
            id: format!("collision-{}-{}", col.doc_a, col.doc_b),
            category: RegressionCategory::SemanticCollision,
            severity,
            file_path: col.doc_a.clone(),
            line_number: None,
            message,
            remediation_hint: remediation,
        });
    }

    // Evaluate technical drift regressions
    for drift in &diff.new_technical_drifts {
        let severity = if config.block_on_technical_drift {
            has_blocker = true;
            RegressionSeverity::Blocker
        } else {
            RegressionSeverity::Warning
        };

        let diagnostic = detected_drifts
            .iter()
            .find(|d| d.doc_id == drift.doc_path && d.code_target == drift.referenced_code_path);

        let (message, remediation, line_num) = if let Some(diag) = diagnostic {
            (
                format!("Technical drift: {}", diag.explanation),
                format!(
                    "Update documentation in '{}' to reflect code change or restore '{}'",
                    diag.doc_id, diag.code_target
                ),
                Some(diag.citation_line),
            )
        } else {
            (
                format!(
                    "Technical drift: '{}' references missing code file '{}'",
                    drift.doc_path, drift.referenced_code_path
                ),
                format!(
                    "Update documentation in '{}' to reflect code removal",
                    drift.doc_path
                ),
                None,
            )
        };

        if severity == RegressionSeverity::Blocker {
            regressions.push(RegressionItem {
                id: format!("drift-{}-{}", drift.doc_path, drift.referenced_code_path),
                category: RegressionCategory::TechnicalDriftContradiction,
                severity,
                file_path: drift.doc_path.clone(),
                line_number: line_num,
                message,
                remediation_hint: remediation,
            });
        } else {
            warnings.push(WarningItem {
                message,
                file_path: Some(drift.doc_path.clone()),
            });
        }
    }

    // Evaluate unreviewed documentation obligations (Feature 010, Invariant XI & FR-007, FR-009)
    for doc in &diff.new_unreviewed_docs {
        let severity = if config.block_on_unreviewed_docs {
            has_blocker = true;
            RegressionSeverity::Blocker
        } else {
            RegressionSeverity::Warning
        };

        if severity == RegressionSeverity::Blocker {
            regressions.push(RegressionItem {
                id: format!("review-obligation-{}", doc),
                category: RegressionCategory::UnreviewedDocumentationObligation,
                severity,
                file_path: doc.clone(),
                line_number: None,
                message: format!("Documentation '{}' has pending review obligations for modified code files", doc),
                remediation_hint: format!("Update '{}', or run 'llm-wiki-core review resolve {}', or include 'Reviewed-Docs: {}' in commit message", doc, doc, doc),
            });
        } else {
            warnings.push(WarningItem {
                message: format!("Documentation '{}' has pending review obligations for modified code files", doc),
                file_path: Some(doc.clone()),
            });
        }
    }

    if config.block_on_legacy_debt {
        for doc in &baseline.unreviewed_docs {
            has_blocker = true;
            regressions.push(RegressionItem {
                id: format!("legacy-review-obligation-{}", doc),
                category: RegressionCategory::UnreviewedDocumentationObligation,
                severity: RegressionSeverity::Blocker,
                file_path: doc.clone(),
                line_number: None,
                message: format!("Legacy unreviewed documentation '{}' blocked by review policy (block_on_legacy_debt: true)", doc),
                remediation_hint: format!("Update '{}' or run 'llm-wiki-core review resolve {}'", doc, doc),
            });
        }
    }

    // Add orphan warnings (orphans warn by default, do not block commits)
    for orphan in &diff.new_orphans {
        warnings.push(WarningItem {
            message: format!(
                "New orphan note '{}' has 0 inbound and 0 outbound links",
                orphan
            ),
            file_path: Some(orphan.clone()),
        });
    }

    // Build resolved findings items
    let mut resolved_findings = Vec::new();
    for bl in &diff.resolved_broken_links {
        resolved_findings.push(ResolvedItem {
            id: format!("resolved-bl-{}", bl.source_path),
            category: "broken_link".to_string(),
            file_path: bl.source_path.clone(),
            description: format!("Resolved broken link to '{}'", bl.target_ref),
        });
    }
    for orphan in &diff.resolved_orphans {
        resolved_findings.push(ResolvedItem {
            id: format!("resolved-orphan-{}", orphan),
            category: "orphan_node".to_string(),
            file_path: orphan.clone(),
            description: format!("Connected previously orphaned note '{}'", orphan),
        });
    }
    for col in &diff.resolved_collisions {
        resolved_findings.push(ResolvedItem {
            id: format!("resolved-col-{}-{}", col.doc_a, col.doc_b),
            category: "collision".to_string(),
            file_path: col.doc_a.clone(),
            description: format!(
                "Resolved collision between '{}' and '{}'",
                col.doc_a, col.doc_b
            ),
        });
    }

    let (decision, exit_code) = if has_blocker {
        (AdmissionDecision::Block, 1)
    } else if !regressions.is_empty() || !warnings.is_empty() {
        (AdmissionDecision::Warn, 0)
    } else {
        (AdmissionDecision::Pass, 0)
    };

    CandidateValidationReport {
        base_commit: baseline.baseline_commit.clone(),
        decision,
        exit_code,
        summary: diff.summary,
        regressions,
        resolved_findings,
        warnings,
    }
}
