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
