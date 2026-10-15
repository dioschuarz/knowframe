//! Technical drift detection triggered by staged code file deletions and renames.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

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
            let (deps, _) = super::extractor::extract_all_code_dependencies(doc_id, &content);
            for dep in &deps {
                index.insert_dependency(doc_id, dep);
            }
            let citations = extract_code_citations(doc_id, &content);
            for cit in citations {
                if cit.symbol.is_some() {
                    index.insert_citation(doc_id, &cit);
                }
            }
        }
    }

    index
}

/// Emits deterministic findings (DELETED_CODE_FILE and OBSOLETE_CODE_REFERENCE) for code mutations.
pub fn detect_deterministic_code_drift(
    workspace_root: &Path,
    changed_files: &[crate::delta::updater::FileDeltaItem],
    reverse_index: &ReverseImpactIndex,
) -> Vec<crate::model::DeterministicFinding> {
    let mut findings = Vec::new();

    // Collect set of markdown documents that are modified in this change set
    let modified_docs: HashSet<String> = changed_files
        .iter()
        .filter(|i| i.path.ends_with(".md") && i.status != FileDeltaStatus::Deleted)
        .map(|i| i.path.replace('\\', "/"))
        .collect();

    let mut deleted_paths = Vec::new();
    let mut added_paths = HashSet::new();

    for item in changed_files {
        let clean_path = item.path.replace('\\', "/");
        if item.status == FileDeltaStatus::Deleted {
            deleted_paths.push(clean_path);
        } else if item.status == FileDeltaStatus::Added {
            added_paths.insert(clean_path);
        }
    }

    let mut finding_counter = 1;

    for old_path in deleted_paths {
        let citing_docs = reverse_index.get_docs_for_code(&old_path);
        for doc_id in citing_docs {
            if modified_docs.contains(&doc_id) {
                continue;
            }

            let abs_path = workspace_root.join(&doc_id);
            let line_number = if let Ok(content) = std::fs::read_to_string(&abs_path) {
                extract_code_citations(&doc_id, &content)
                    .into_iter()
                    .find(|c| c.code_path == old_path || old_path.ends_with(&c.code_path))
                    .map(|c| c.line_number)
            } else {
                None
            };

            // Detect potential rename if there is a newly added file with matching stem or same directory
            let old_stem = Path::new(&old_path).file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let rename_target = added_paths.iter().find(|added| {
                let added_stem = Path::new(added.as_str()).file_stem().and_then(|s| s.to_str()).unwrap_or("");
                !old_stem.is_empty() && (added_stem == old_stem || added.ends_with(&old_path))
            });

            let (rule_id, required_action, rule_description, target) = if let Some(new_path) = rename_target {
                (
                    "OBSOLETE_CODE_REFERENCE".to_string(),
                    format!("Update citation to reference new location '{}'", new_path),
                    format!("Referenced code file '{}' was renamed to '{}'", old_path, new_path),
                    Some(new_path.clone()),
                )
            } else {
                (
                    "DELETED_CODE_FILE".to_string(),
                    format!("Remove or update citation to deleted code file '{}'", old_path),
                    format!("Referenced code file '{}' was deleted from repository", old_path),
                    Some(old_path.clone()),
                )
            };

            findings.push(crate::model::DeterministicFinding {
                finding_id: format!("FINDING-DRIFT-{:03}", finding_counter),
                rule_id,
                severity: crate::model::DeterministicFindingSeverity::Error,
                subject: doc_id,
                evidence: crate::model::FindingEvidence {
                    changed_paths: vec![old_path.clone()],
                    dependency_sources: vec!["code_reference".to_string()],
                    rule_description,
                    line_number,
                    anchor: None,
                    target,
                },
                required_action,
                epistemic_tier: Some(crate::model::EpistemicTier::TierAFullyDeterministic),
            });
            finding_counter += 1;
        }
    }

    findings.sort_by(|a, b| a.finding_id.cmp(&b.finding_id));
    findings
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

    let mut nodes: Vec<_> = topology.nodes.values().cloned().collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    if nodes.is_empty() {
        return Vec::new();
    }

    let home_dir = std::env::var("HOME").ok();

    // Collect non-hidden workspace subdirectories (e.g. sub-projects/repos)
    let workspace_subdirs: Vec<PathBuf> = if let Ok(entries) = std::fs::read_dir(workspace_root) {
        let mut dirs = Vec::new();
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if !name_str.starts_with('.') && name_str != "target" && name_str != "node_modules" {
                        dirs.push(entry.path());
                    }
                }
            }
        }
        dirs
    } else {
        Vec::new()
    };

    let mut findings: Vec<TechnicalDriftFinding> = nodes
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

                // 1. Direct path from workspace root or notes directory
                if workspace_root.join(target).exists()
                    || workspace_root.join("notes").join(target).exists()
                {
                    exists = true;
                }

                // 2. Relative to the note's own directory or any ancestor folder up to workspace_root
                if !exists {
                    let mut curr = file_path.parent();
                    while let Some(parent) = curr {
                        if parent.join(target).exists() {
                            exists = true;
                            break;
                        }
                        if parent == workspace_root {
                            break;
                        }
                        curr = parent.parent();
                    }
                }

                // 3. Subdirectories of workspace_root (e.g. sub-repositories or sub-projects)
                if !exists {
                    for sub in &workspace_subdirs {
                        if sub.join(target).exists() {
                            exists = true;
                            break;
                        }
                    }
                }

                // 4. Home directory expansion
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

    findings.sort_by(|a, b| {
        a.doc_id
            .cmp(&b.doc_id)
            .then_with(|| a.code_target.cmp(&b.code_target))
            .then_with(|| a.citation_line.cmp(&b.citation_line))
    });

    findings
}
