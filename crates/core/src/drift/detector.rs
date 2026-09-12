//! Technical drift detection triggered by staged code file deletions and renames.

use std::collections::HashSet;
use std::path::Path;

use super::extractor::{extract_code_citations, is_code_path};
use super::{DriftConfidence, DriftType, ReverseImpactIndex, TechnicalDriftFinding};
use crate::debt::DriftSignature;
use crate::delta::updater::FileDeltaStatus;
use crate::delta::GitDelta;

/// Builds a complete `ReverseImpactIndex` across all known markdown notes in workspace.
pub fn build_reverse_impact_index(
    workspace_root: &Path,
    doc_paths: &[String],
) -> ReverseImpactIndex {
    let mut index = ReverseImpactIndex::new();

    for doc_id in doc_paths {
        let abs_path = workspace_root.join(doc_id);
        if let Ok(content) = std::fs::read_to_string(&abs_path) {
            let citations = extract_code_citations(doc_id, &content);
            for cit in citations {
                index.insert_citation(doc_id, &cit);
            }
        }
    }

    index
}

/// Detects technical drift findings between staged code mutations and existing documentation.
pub fn detect_technical_drift(
    workspace_root: &Path,
    staged_delta: &GitDelta,
    reverse_index: &ReverseImpactIndex,
) -> (Vec<TechnicalDriftFinding>, Vec<DriftSignature>) {
    let mut findings = Vec::new();
    let mut signatures = Vec::new();

    // Collect set of markdown documents that are staged in this commit
    let staged_docs: HashSet<String> = staged_delta
        .items
        .iter()
        .filter(|i| i.path.ends_with(".md"))
        .map(|i| i.path.replace('\\', "/"))
        .collect();

    for item in &staged_delta.items {
        let path = item.path.replace('\\', "/");
        if !is_code_path(&path) {
            continue;
        }

        match item.status {
            FileDeltaStatus::Deleted => {
                let citing_docs = reverse_index.get_docs_for_code(&path);

                for doc_id in citing_docs {
                    // If the citing doc is concurrently modified in staged changes, drift is addressed
                    if staged_docs.contains(&doc_id) {
                        continue;
                    }

                    // Locate citation line
                    let abs_path = workspace_root.join(&doc_id);
                    let line_number = if let Ok(content) = std::fs::read_to_string(&abs_path) {
                        extract_code_citations(&doc_id, &content)
                            .into_iter()
                            .find(|c| c.code_path == path || path.ends_with(&c.code_path))
                            .map(|c| c.line_number)
                            .unwrap_or(1)
                    } else {
                        1
                    };

                    let explanation = format!(
                        "Documentation cites deleted source file '{}' at line {} which was removed without updating documentation",
                        path, line_number
                    );

                    let finding = TechnicalDriftFinding {
                        doc_id: doc_id.clone(),
                        code_target: path.clone(),
                        symbol_name: None,
                        citation_line: line_number,
                        drift_type: DriftType::DeletedCodeFile,
                        confidence: DriftConfidence::High,
                        explanation,
                    };

                    let sig = DriftSignature {
                        doc_path: doc_id,
                        referenced_code_path: path.clone(),
                        symbol: String::new(),
                    };

                    findings.push(finding);
                    signatures.push(sig);
                }
            }
            FileDeltaStatus::Modified | FileDeltaStatus::Added => {
                // Code file modified or added - check if any deleted symbols if applicable
            }
        }
    }

    (findings, signatures)
}

/// Detects all technical drift and broken code citations across all notes in the knowledge graph.
pub fn detect_all_drifts_in_graph(
    topology: &crate::graph::builder::GraphTopology,
    workspace_root: &Path,
) -> Vec<TechnicalDriftFinding> {
    use rayon::prelude::*;

    let nodes: Vec<_> = topology.nodes.values().cloned().collect();
    if nodes.is_empty() {
        return Vec::new();
    }

    let home_dir = std::env::var("HOME").ok();

    let findings: Vec<TechnicalDriftFinding> = nodes
        .par_iter()
        .flat_map(|node| {
            let mut file_path = workspace_root.join(&node.file_path);
            if !file_path.exists() {
                let alt = workspace_root.join(&node.id);
                if alt.exists() {
                    file_path = alt;
                } else {
                    let notes_alt = workspace_root.join("notes").join(&node.file_path);
                    if notes_alt.exists() {
                        file_path = notes_alt;
                    }
                }
            }

            let content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(_) => return Vec::new(),
            };

            let citations = extract_code_citations(&node.id, &content);
            let mut node_drifts = Vec::new();
            let mut seen_targets = HashSet::new();

            for cit in citations {
                if cit.code_path.is_empty() {
                    continue;
                }
                let target = &cit.code_path;

                // Skip documentation template placeholders and globs
                if target.contains('[')
                    || target.contains(']')
                    || target.contains('<')
                    || target.contains('>')
                    || target.contains('$')
                    || target.contains('*')
                    || target.contains("...")
                {
                    continue;
                }

                if seen_targets.contains(target) {
                    continue;
                }

                let mut exists = false;

                // 1. Direct path from workspace root
                if workspace_root.join(target).exists()
                    || workspace_root.join("notes").join(target).exists()
                    || Path::new(target).exists()
                {
                    exists = true;
                }

                // 2. Relative to the note's own directory
                if !exists {
                    if let Some(parent) = file_path.parent() {
                        if parent.join(target).exists() {
                            exists = true;
                        }
                    }
                }

                // 3. Home directory expansion
                if !exists && target.starts_with('~') {
                    if let Some(ref home) = home_dir {
                        let expanded = target.replacen('~', home, 1);
                        if Path::new(&expanded).exists() {
                            exists = true;
                        }
                    }
                }

                if !exists {
                    seen_targets.insert(target.clone());
                    node_drifts.push(TechnicalDriftFinding {
                        doc_id: node.id.clone(),
                        code_target: cit.code_path.clone(),
                        symbol_name: cit.symbol.clone(),
                        citation_line: cit.line_number,
                        drift_type: DriftType::DeletedCodeFile,
                        confidence: DriftConfidence::High,
                        explanation: format!(
                            "Referenced code/config file '{}' at line {} does not exist in workspace",
                            cit.code_path, cit.line_number
                        ),
                    });
                }
            }

            node_drifts
        })
        .collect();

    findings
}

