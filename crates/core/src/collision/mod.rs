//! Multi-signal deterministic document fingerprinting and semantic collision engine.

pub mod canonical;
pub mod fingerprint;
pub mod scorer;

use std::collections::{HashMap, HashSet};
use std::path::Path;
use serde::{Deserialize, Serialize};

pub use canonical::*;
pub use fingerprint::*;
pub use scorer::*;

use crate::candidate::overlay::CandidateOverlay;
use crate::candidate::WarningItem;
use crate::debt::CollisionSignature;

/// Deterministic multi-signal representation of a markdown note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentFingerprint {
    pub doc_id: String,
    pub title_tokens: HashSet<String>,
    pub heading_tokens: HashSet<String>,
    pub term_frequencies: HashMap<String, f32>,
    pub tags: HashSet<String>,
    pub ngrams: HashSet<String>,
}

impl DocumentFingerprint {
    pub fn new(doc_id: impl Into<String>) -> Self {
        Self {
            doc_id: doc_id.into(),
            title_tokens: HashSet::new(),
            heading_tokens: HashSet::new(),
            term_frequencies: HashMap::new(),
            tags: HashSet::new(),
            ngrams: HashSet::new(),
        }
    }
}

/// Pairwise collision diagnostic between two documents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticCollision {
    pub source_id: String,
    pub target_id: String,
    pub composite_similarity: f32, // 0.0 to 1.0
    pub title_similarity: f32,
    pub heading_similarity: f32,
    pub bm25_similarity: f32,
    pub ngram_similarity: f32,
    pub tag_similarity: f32,
    pub classification: CollisionClassification,
    pub shared_key_terms: Vec<String>,
    pub canonical_candidate: Option<String>,
    pub canonical_confidence: f32,
    pub canonical_evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionClassification {
    PotentialDuplicate,
    PotentialFragmentation,
    PotentialSpecialization,
    PotentialSummary,
    PotentialConflict,
}

/// Detects semantic collisions for staged candidate files against the indexed knowledge base.
/// Returns (high_confidence_collisions, collision_signatures, moderate_warnings).
pub fn detect_collisions_in_candidate(
    overlay: &CandidateOverlay,
    _workspace_root: &Path,
) -> (Vec<SemanticCollision>, Vec<CollisionSignature>, Vec<WarningItem>) {
    let known_nodes = overlay.collect_known_nodes();
    let mut index = InvertedIndex::new();

    // 1. Build fingerprints for all surviving candidate nodes
    for (id, path) in &known_nodes {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let headings = overlay.get_headings(id).unwrap_or(&[]).to_vec();
        let tags = overlay.get_node(id).map(|n| n.tags.clone()).unwrap_or_default();

        let fp = extract_fingerprint(id, &content, &headings, &tags);
        index.insert(fp);
    }

    let mut high_collisions = Vec::new();
    let mut signatures = Vec::new();
    let mut warnings = Vec::new();
    let mut seen_pairs = HashSet::new();

    // Check entrypoint references in base graph
    let entrypoint_citations: HashSet<String> = overlay
        .base_graph
        .edges
        .iter()
        .filter(|e| e.source == "AGENTS.md" || e.source == "README.md" || e.source == "CLAUDE.md")
        .map(|e| e.target.clone())
        .collect();

    // 2. Query candidates for each staged file
    for staged_id in overlay.overlay.mutated_nodes.keys() {
        if let Some(staged_fp) = index.doc_to_fingerprint.get(staged_id) {
            let candidates = index.query_candidates(staged_fp, 50);

            for cand_id in candidates {
                let pair_sig = CollisionSignature::new(staged_id, &cand_id);
                if seen_pairs.contains(&pair_sig) {
                    continue;
                }
                seen_pairs.insert(pair_sig.clone());

                if let Some(cand_fp) = index.doc_to_fingerprint.get(&cand_id) {
                    if let Some(mut col) = score_collision(staged_fp, cand_fp, 0.75) {
                        let staged_node = overlay.get_node(staged_id);
                        let cand_node = overlay.get_node(&cand_id);

                        let staged_ctx = DocumentMetadataContext {
                            doc_id: staged_id,
                            is_entrypoint_referenced: entrypoint_citations.contains(staged_id),
                            degree: staged_node.map(|n| n.in_degree + n.out_degree).unwrap_or(0),
                            betweenness: staged_node.map(|n| n.betweenness as f32).unwrap_or(0.0),
                            heading_count: overlay.get_headings(staged_id).map(|h| h.len()).unwrap_or(0),
                            token_count: staged_node.map(|n| n.tokens).unwrap_or(0),
                            commit_timestamp: None,
                        };

                        let cand_ctx = DocumentMetadataContext {
                            doc_id: &cand_id,
                            is_entrypoint_referenced: entrypoint_citations.contains(&cand_id),
                            degree: cand_node.map(|n| n.in_degree + n.out_degree).unwrap_or(0),
                            betweenness: cand_node.map(|n| n.betweenness as f32).unwrap_or(0.0),
                            heading_count: overlay.get_headings(&cand_id).map(|h| h.len()).unwrap_or(0),
                            token_count: cand_node.map(|n| n.tokens).unwrap_or(0),
                            commit_timestamp: None,
                        };

                        enrich_collision_with_canonical(&mut col, &staged_ctx, &cand_ctx);

                        if col.composite_similarity >= 0.85 {
                            high_collisions.push(col);
                            signatures.push(pair_sig);
                        } else {
                            warnings.push(WarningItem {
                                message: format!(
                                    "Moderate semantic overlap ({:.1}%) between '{}' and '{}' ({:?})",
                                    col.composite_similarity * 100.0,
                                    staged_id,
                                    cand_id,
                                    col.classification
                                ),
                                file_path: Some(staged_id.clone()),
                            });
                        }
                    }
                }
            }
        }
    }

    (high_collisions, signatures, warnings)
}
