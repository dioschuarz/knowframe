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
#[allow(clippy::enum_variant_names)]
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

/// Detects semantic collisions across all notes in the knowledge graph topology.
pub fn detect_collisions_in_graph(
    topology: &crate::graph::builder::GraphTopology,
    workspace_root: &Path,
    threshold: f32,
) -> Vec<SemanticCollision> {
    use rayon::prelude::*;

    let nodes: Vec<_> = topology.nodes.values().cloned().collect();
    if nodes.is_empty() {
        return Vec::new();
    }

    // Extract fingerprints in parallel across all notes in topology
    let fingerprints: Vec<DocumentFingerprint> = nodes
        .par_iter()
        .map(|node| {
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
            let content = std::fs::read_to_string(&file_path).unwrap_or_default();
            extract_fingerprint(&node.id, &content, &[], &node.tags)
        })
        .collect();

    let mut index = InvertedIndex::new();
    for fp in fingerprints {
        index.insert(fp);
    }

    let entrypoint_citations: HashSet<String> = topology
        .edges
        .iter()
        .filter(|e| {
            e.source == "AGENTS.md"
                || e.source == "README.md"
                || e.source == "CLAUDE.md"
                || e.source.ends_with("wiki_index.md")
        })
        .map(|e| e.target.clone())
        .collect();

    let mut collisions: Vec<SemanticCollision> = index
        .doc_to_fingerprint
        .par_iter()
        .flat_map(|(doc_id, fp)| {
            let candidates = index.query_candidates(fp, 30);
            let mut doc_cols = Vec::new();
            for cand_id in candidates {
                if doc_id >= &cand_id {
                    continue;
                }

                if let Some(cand_fp) = index.doc_to_fingerprint.get(&cand_id) {
                    if let Some(mut col) = score_collision(fp, cand_fp, threshold) {
                        let node_a = topology.nodes.get(doc_id);
                        let node_b = topology.nodes.get(&cand_id);

                        let ctx_a = DocumentMetadataContext {
                            doc_id: doc_id.as_str(),
                            is_entrypoint_referenced: entrypoint_citations.contains(doc_id),
                            degree: node_a.map(|n| n.in_degree + n.out_degree).unwrap_or(0),
                            betweenness: node_a.map(|n| n.betweenness as f32).unwrap_or(0.0),
                            heading_count: fp.heading_tokens.len(),
                            token_count: node_a.map(|n| n.tokens).unwrap_or(0),
                            commit_timestamp: None,
                        };

                        let ctx_b = DocumentMetadataContext {
                            doc_id: cand_id.as_str(),
                            is_entrypoint_referenced: entrypoint_citations.contains(&cand_id),
                            degree: node_b.map(|n| n.in_degree + n.out_degree).unwrap_or(0),
                            betweenness: node_b.map(|n| n.betweenness as f32).unwrap_or(0.0),
                            heading_count: cand_fp.heading_tokens.len(),
                            token_count: node_b.map(|n| n.tokens).unwrap_or(0),
                            commit_timestamp: None,
                        };

                        enrich_collision_with_canonical(&mut col, &ctx_a, &ctx_b);
                        doc_cols.push(col);
                    }
                }
            }
            doc_cols
        })
        .collect();

    collisions.sort_by(|a, b| {
        b.composite_similarity
            .partial_cmp(&a.composite_similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    collisions
}

