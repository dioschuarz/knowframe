//! PR/CI Review Lifecycle & Verifiable Receipt Management (Feature 010).
//! Implements FR-005, FR-006, FR-007, FR-008, FR-009.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::ReverseImpactIndex;
use crate::delta::updater::{FileDeltaItem, FileDeltaStatus};
use crate::model::{
    AffectedDocumentsReport, DependencySource, DeterministicFinding,
    DeterministicFindingSeverity, FindingEvidence, ReviewGatingConfig,
    ReviewObligation, ReviewReceiptStore, ReviewStatus,
};

pub const REVIEWS_FILE_PATH: &str = ".context-vault/reviews.json";

/// Loads the review receipt store from `.context-vault/reviews.json`.
pub fn load_review_receipts(workspace_root: &Path) -> ReviewReceiptStore {
    let path = workspace_root.join(REVIEWS_FILE_PATH);
    if path.is_file() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(store) = serde_json::from_str::<ReviewReceiptStore>(&content) {
                return store;
            }
        }
    }
    ReviewReceiptStore::default()
}

/// Saves the review receipt store to `.context-vault/reviews.json`.
pub fn save_review_receipts(
    workspace_root: &Path,
    store: &ReviewReceiptStore,
) -> Result<(), String> {
    let dir = workspace_root.join(".context-vault");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create .context-vault: {e}"))?;
    }
    let path = workspace_root.join(REVIEWS_FILE_PATH);
    let serialized = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize review receipts: {e}"))?;
    fs::write(path, serialized).map_err(|e| format!("Failed to write reviews.json: {e}"))?;
    Ok(())
}

/// Records a review receipt for the specified document IDs.
pub fn resolve_document_review(
    workspace_root: &Path,
    doc_ids: &[String],
    commit_or_ref: Option<&str>,
) -> Result<(), String> {
    let mut store = load_review_receipts(workspace_root);
    let key = commit_or_ref.unwrap_or("HEAD").to_string();

    let entry = store.reviews.entry(key).or_default();
    for id in doc_ids {
        let clean = id.trim().replace('\\', "/");
        if !entry.contains(&clean) {
            entry.push(clean);
        }
    }
    store.reviews.values_mut().for_each(|v| v.sort());

    save_review_receipts(workspace_root, &store)
}

/// Parses `Reviewed-Docs: <path1>, <path2>` or `Reviewed-Docs: all` commit trailers.
pub fn parse_commit_trailers(commit_message: &str) -> HashSet<String> {
    let mut reviewed = HashSet::new();

    for line in commit_message.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Reviewed-Docs:") {
            let val = rest.trim();
            if val.eq_ignore_ascii_case("all") {
                reviewed.insert("all".to_string());
            } else {
                for item in val.split(',') {
                    let clean = item.trim().replace('\\', "/");
                    if !clean.is_empty() {
                        reviewed.insert(clean);
                    }
                }
            }
        }
    }

    reviewed
}

/// Calculates affected documentation notes from a set of Git file deltas.
pub fn calculate_affected_documents(
    workspace_root: &Path,
    changed_files: &[FileDeltaItem],
    reverse_index: &ReverseImpactIndex,
    gating_config: Option<&ReviewGatingConfig>,
    active_receipts: Option<&[String]>,
    commit_message: Option<&str>,
) -> AffectedDocumentsReport {
    let default_config = ReviewGatingConfig::default();
    let config = gating_config.unwrap_or(&default_config);

    let mut changed_code_files = Vec::new();
    let mut modified_docs = HashSet::new();

    // Separate code files from docs
    for item in changed_files {
        let clean_path = item.path.trim().replace('\\', "/");
        if clean_path.ends_with(".md") {
            if item.status != FileDeltaStatus::Deleted {
                modified_docs.insert(clean_path.clone());
            }
        } else {
            changed_code_files.push(clean_path);
        }
    }
    changed_code_files.sort();
    changed_code_files.dedup();

    // Collect all valid review receipts (file store + active in-memory parameter + commit trailer)
    let receipt_store = load_review_receipts(workspace_root);
    let mut receipt_docs = HashSet::new();

    for doc_list in receipt_store.reviews.values() {
        for d in doc_list {
            receipt_docs.insert(d.clone());
        }
    }

    if let Some(active) = active_receipts {
        for d in active {
            receipt_docs.insert(d.trim().replace('\\', "/"));
        }
    }

    let trailer_docs = commit_message.map(parse_commit_trailers).unwrap_or_default();
    let all_reviewed_by_trailer = trailer_docs.contains("all");

    // Map: doc_id -> (HashSet<triggering_code_file>, HashSet<DependencySource>)
    let mut doc_obligations: HashMap<String, (HashSet<String>, HashSet<DependencySource>)> = HashMap::new();

    for code_file in &changed_code_files {
        let citing_docs = reverse_index.get_docs_for_code(code_file);
        for doc_id in citing_docs {
            let sources = reverse_index.get_sources_for_doc_and_code(&doc_id, code_file);
            let entry = doc_obligations.entry(doc_id).or_default();
            entry.0.insert(code_file.clone());
            entry.1.extend(sources);
        }
    }

    // Build deterministic obligations
    let mut sorted_doc_ids: Vec<String> = doc_obligations.keys().cloned().collect();
    sorted_doc_ids.sort();

    let mut obligations = Vec::new();
    let mut findings = Vec::new();

    let mut pending_count = 0u32;
    let mut reviewed_count = 0u32;
    let mut blocked_count = 0u32;

    for (idx, doc_id) in sorted_doc_ids.into_iter().enumerate() {
        let (triggering_files, source_set) = doc_obligations.remove(&doc_id).unwrap();

        let mut trigger_vec: Vec<String> = triggering_files.into_iter().collect();
        trigger_vec.sort();

        let mut sources_vec: Vec<DependencySource> = source_set.into_iter().collect();
        sources_vec.sort_by_key(|s| match s {
            DependencySource::Frontmatter => 0,
            DependencySource::InlineMarkdownLink => 1,
            DependencySource::InlineSymbolCitation => 2,
        });

        // Determine resolution status
        let (status, receipt_str) = if modified_docs.contains(&doc_id) {
            (ReviewStatus::Updated, None)
        } else if all_reviewed_by_trailer || trailer_docs.contains(&doc_id) {
            reviewed_count += 1;
            (ReviewStatus::ReviewedUnchanged, Some("commit_trailer".to_string()))
        } else if receipt_docs.contains(&doc_id) {
            reviewed_count += 1;
            (ReviewStatus::ReviewedUnchanged, Some("reviews_json".to_string()))
        } else if config.enforce_doc_review {
            blocked_count += 1;
            (ReviewStatus::Blocked, None)
        } else {
            pending_count += 1;
            (ReviewStatus::Pending, None)
        };

        if status == ReviewStatus::Pending || status == ReviewStatus::Blocked {
            let severity = if status == ReviewStatus::Blocked {
                DeterministicFindingSeverity::Error
            } else {
                DeterministicFindingSeverity::Warning
            };

            let finding = DeterministicFinding {
                finding_id: format!("FINDING-REV-{:03}", idx + 1),
                rule_id: "DOCUMENT_REVIEW_REQUIRED".to_string(),
                severity,
                subject: doc_id.clone(),
                evidence: FindingEvidence {
                    changed_paths: trigger_vec.clone(),
                    dependency_sources: sources_vec.iter().map(|s| match s {
                        DependencySource::Frontmatter => "frontmatter".to_string(),
                        DependencySource::InlineMarkdownLink => "inline_markdown_link".to_string(),
                        DependencySource::InlineSymbolCitation => "inline_symbol_citation".to_string(),
                    }).collect(),
                    rule_description: "Documentation references code modified in change range".to_string(),
                    line_number: None,
                    anchor: None,
                    target: None,
                },
                required_action: "Update documentation to reflect code alterations or provide reviewed_unchanged receipt".to_string(),
                epistemic_tier: Some(crate::model::EpistemicTier::TierAFullyDeterministic),
            };
            findings.push(finding);
        }

        obligations.push(ReviewObligation {
            doc_id,
            status,
            triggering_code_files: trigger_vec,
            dependency_sources: sources_vec,
            resolution_receipt: receipt_str,
        });
    }

    AffectedDocumentsReport {
        changed_code_files,
        obligations,
        findings,
        pending_count,
        reviewed_count,
        blocked_count,
    }
}
