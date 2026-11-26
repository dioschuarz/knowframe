//! Multi-signal deterministic document fingerprinting and semantic collision engine.

pub mod canonical;
pub mod fingerprint;
pub mod scorer;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

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
) -> (
    Vec<SemanticCollision>,
    Vec<CollisionSignature>,
    Vec<WarningItem>,
) {
    let known_nodes = overlay.collect_known_nodes();
    let mut index = InvertedIndex::new();

    // 1. Build fingerprints for all surviving candidate nodes
    for (id, path) in &known_nodes {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let headings = overlay.get_headings(id).unwrap_or(&[]).to_vec();
        let tags = overlay
            .get_node(id)
            .map(|n| n.tags.clone())
            .unwrap_or_default();

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
                            heading_count: overlay
                                .get_headings(staged_id)
                                .map(|h| h.len())
                                .unwrap_or(0),
                            token_count: staged_node.map(|n| n.tokens).unwrap_or(0),
                            commit_timestamp: None,
                        };

                        let cand_ctx = DocumentMetadataContext {
                            doc_id: &cand_id,
                            is_entrypoint_referenced: entrypoint_citations.contains(&cand_id),
                            degree: cand_node.map(|n| n.in_degree + n.out_degree).unwrap_or(0),
                            betweenness: cand_node.map(|n| n.betweenness as f32).unwrap_or(0.0),
                            heading_count: overlay
                                .get_headings(&cand_id)
                                .map(|h| h.len())
                                .unwrap_or(0),
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

/// Maximum collision pairs retained from a full-graph sweep so pathological boilerplate
/// monorepos cannot exhaust memory.
const MAX_GRAPH_COLLISIONS: usize = 2000;

fn resolve_node_disk_path(
    workspace_root: &Path,
    node: &crate::model::NoteNode,
) -> std::path::PathBuf {
    let file_path = workspace_root.join(&node.file_path);
    if file_path.exists() {
        return file_path;
    }
    let alt = workspace_root.join(&node.id);
    if alt.exists() {
        return alt;
    }
    let notes_alt = workspace_root.join("notes").join(&node.file_path);
    if notes_alt.exists() {
        return notes_alt;
    }
    file_path
}

fn classify_compact_collision(
    a: &CompactFingerprint,
    b: &CompactFingerprint,
    composite: f32,
    title_sim: f32,
    heading_sim: f32,
    bm25_sim: f32,
    ngram_sim: f32,
) -> CollisionClassification {
    let headings_a = a.heading_hashes.len();
    let headings_b = b.heading_hashes.len();
    let ngrams_a = a.ngram_count;
    let ngrams_b = b.ngram_count;

    let heading_ratio = if headings_a > 0 && headings_b > 0 {
        (headings_a as f32 / headings_b as f32).max(headings_b as f32 / headings_a as f32)
    } else {
        1.0
    };

    let length_ratio = if ngrams_a > 0 && ngrams_b > 0 {
        (ngrams_a as f32 / ngrams_b as f32).min(ngrams_b as f32 / ngrams_a as f32)
    } else {
        1.0
    };

    let has_summary_intent = a.has_summary_intent || b.has_summary_intent;

    if length_ratio <= 0.40
        && (has_summary_intent || heading_ratio < 2.0)
        && (title_sim >= 0.35 || bm25_sim >= 0.35)
    {
        return CollisionClassification::PotentialSummary;
    }
    if heading_ratio >= 2.0 && (bm25_sim >= 0.35 || title_sim >= 0.40) {
        return CollisionClassification::PotentialSpecialization;
    }
    if (title_sim >= 0.70 || heading_sim >= 0.60) && bm25_sim < 0.50 && ngram_sim < 0.40 {
        return CollisionClassification::PotentialConflict;
    }
    if composite < 0.85 && heading_sim < 0.40 && bm25_sim >= 0.60 {
        return CollisionClassification::PotentialFragmentation;
    }
    CollisionClassification::PotentialDuplicate
}

fn score_compact_collision(
    a: &CompactFingerprint,
    b: &CompactFingerprint,
    doc_a: &str,
    doc_b: &str,
    threshold: f32,
) -> Option<SemanticCollision> {
    let title_similarity = sorted_jaccard(&a.title_hashes, &b.title_hashes);
    let heading_similarity = sorted_jaccard(&a.heading_hashes, &b.heading_hashes);
    let bm25_similarity = sorted_cosine_tf(&a.top_terms, a.tf_norm, &b.top_terms, b.tf_norm);
    let ngram_similarity = minhash_jaccard(&a.minhash, &b.minhash);
    let tag_similarity = sorted_jaccard(&a.tag_hashes, &b.tag_hashes);

    let mut total_weight = DEFAULT_WEIGHT_TITLE;
    let mut weighted_sum = DEFAULT_WEIGHT_TITLE * title_similarity;

    if !a.heading_hashes.is_empty() || !b.heading_hashes.is_empty() {
        total_weight += DEFAULT_WEIGHT_HEADING;
        weighted_sum += DEFAULT_WEIGHT_HEADING * heading_similarity;
    }
    if !a.top_terms.is_empty() || !b.top_terms.is_empty() {
        total_weight += DEFAULT_WEIGHT_BM25;
        weighted_sum += DEFAULT_WEIGHT_BM25 * bm25_similarity;
    }
    if !a.minhash.is_empty() || !b.minhash.is_empty() {
        total_weight += DEFAULT_WEIGHT_NGRAM;
        weighted_sum += DEFAULT_WEIGHT_NGRAM * ngram_similarity;
    }
    if !a.tag_hashes.is_empty() || !b.tag_hashes.is_empty() {
        total_weight += DEFAULT_WEIGHT_TAG;
        weighted_sum += DEFAULT_WEIGHT_TAG * tag_similarity;
    }

    let composite_similarity = if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    };

    if composite_similarity < threshold {
        return None;
    }

    let classification = classify_compact_collision(
        a,
        b,
        composite_similarity,
        title_similarity,
        heading_similarity,
        bm25_similarity,
        ngram_similarity,
    );

    Some(SemanticCollision {
        source_id: doc_a.to_string(),
        target_id: doc_b.to_string(),
        composite_similarity,
        title_similarity,
        heading_similarity,
        bm25_similarity,
        ngram_similarity,
        tag_similarity,
        classification,
        shared_key_terms: Vec::new(),
        canonical_candidate: None,
        canonical_confidence: 0.0,
        canonical_evidence: Vec::new(),
    })
}

fn extract_shared_key_terms_from_disk(
    workspace_root: &Path,
    node_a: &crate::model::NoteNode,
    node_b: &crate::model::NoteNode,
) -> Vec<String> {
    let content_a =
        std::fs::read_to_string(resolve_node_disk_path(workspace_root, node_a)).unwrap_or_default();
    let content_b =
        std::fs::read_to_string(resolve_node_disk_path(workspace_root, node_b)).unwrap_or_default();

    let count_terms = |text: &str| -> HashMap<String, f32> {
        let toks = tokenize(text);
        let denom = toks.len().max(1) as f32;
        let mut counts: HashMap<String, u32> = HashMap::new();
        for t in toks {
            *counts.entry(t).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .map(|(k, v)| (k, (v as f32) / denom))
            .collect()
    };

    let tf_a = count_terms(&content_a);
    let tf_b = count_terms(&content_b);
    let mut shared: Vec<(String, f32)> = tf_a
        .into_iter()
        .filter_map(|(term, va)| tf_b.get(&term).map(|vb| (term, va + vb)))
        .collect();
    shared.sort_by(|x, y| {
        y.1.partial_cmp(&x.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| x.0.cmp(&y.0))
    });
    shared.into_iter().take(10).map(|(t, _)| t).collect()
}

/// Detects semantic collisions across all notes in the knowledge graph topology.
/// Uses compact hashed fingerprints + bottom-K MinHash (~1.5 KB/doc) so enterprise vaults
/// with 20,000+ documents stay under ~60 MB peak RAM instead of multi-gigabyte heap spikes.
pub fn detect_collisions_in_graph(
    topology: &crate::graph::builder::GraphTopology,
    workspace_root: &Path,
    threshold: f32,
) -> Vec<SemanticCollision> {
    use rayon::prelude::*;

    // Borrow active (non-excluded) nodes in deterministic order without cloning NoteNode payloads
    let mut nodes: Vec<&crate::model::NoteNode> = topology
        .nodes
        .values()
        .filter(|n| n.status != crate::model::NodeStatus::Excluded)
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    if nodes.len() < 2 {
        return Vec::new();
    }

    // 1. Extract compact fingerprints in parallel (~1.5 KB/doc, zero String ngrams retained)
    let fingerprints: Vec<CompactFingerprint> = nodes
        .par_iter()
        .enumerate()
        .map(|(idx, node)| {
            let file_path = resolve_node_disk_path(workspace_root, node);
            let content = std::fs::read_to_string(&file_path).unwrap_or_default();
            extract_compact_fingerprint(idx as u32, &node.id, &content, &node.tags)
        })
        .collect();

    // 2. Build compact interned inverted index (u64 -> Vec<u32>)
    let index = CompactInvertedIndex::build(&fingerprints);

    let entrypoint_citations: HashSet<&str> = topology
        .edges
        .iter()
        .filter(|e| {
            e.source == "AGENTS.md"
                || e.source == "README.md"
                || e.source == "CLAUDE.md"
                || e.source.ends_with("wiki_index.md")
        })
        .map(|e| e.target.as_str())
        .collect();

    // 3. Score top-30 candidates per document in parallel with zero String allocations
    let mut collisions: Vec<SemanticCollision> = fingerprints
        .par_iter()
        .flat_map(|fp| {
            let candidates = index.query_candidates(fp, 30);
            let mut doc_cols = Vec::new();
            let node_a = nodes[fp.doc_idx as usize];
            for cand_idx in candidates {
                let cand_fp = &fingerprints[cand_idx as usize];
                let node_b = nodes[cand_idx as usize];

                if let Some(mut col) = score_compact_collision(
                    fp,
                    cand_fp,
                    &node_a.id,
                    &node_b.id,
                    threshold,
                ) {
                    let ctx_a = DocumentMetadataContext {
                        doc_id: node_a.id.as_str(),
                        is_entrypoint_referenced: entrypoint_citations.contains(node_a.id.as_str()),
                        degree: node_a.in_degree + node_a.out_degree,
                        betweenness: node_a.betweenness as f32,
                        heading_count: fp.heading_hashes.len(),
                        token_count: node_a.tokens,
                        commit_timestamp: None,
                    };

                    let ctx_b = DocumentMetadataContext {
                        doc_id: node_b.id.as_str(),
                        is_entrypoint_referenced: entrypoint_citations.contains(node_b.id.as_str()),
                        degree: node_b.in_degree + node_b.out_degree,
                        betweenness: node_b.betweenness as f32,
                        heading_count: cand_fp.heading_hashes.len(),
                        token_count: node_b.tokens,
                        commit_timestamp: None,
                    };

                    enrich_collision_with_canonical(&mut col, &ctx_a, &ctx_b);
                    doc_cols.push(col);
                }
            }
            doc_cols
        })
        .collect();

    // Drop the index and fingerprints immediately before enriching top collisions
    drop(index);
    drop(fingerprints);

    collisions.sort_by(|a, b| {
        b.composite_similarity
            .partial_cmp(&a.composite_similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.source_id.cmp(&b.source_id))
            .then_with(|| a.target_id.cmp(&b.target_id))
    });
    if collisions.len() > MAX_GRAPH_COLLISIONS {
        collisions.truncate(MAX_GRAPH_COLLISIONS);
    }

    // 4. Populate human-readable shared key terms only for the top winning collisions
    collisions.par_iter_mut().take(250).for_each(|col| {
        if let (Some(node_a), Some(node_b)) = (
            topology.nodes.get(&col.source_id),
            topology.nodes.get(&col.target_id),
        ) {
            col.shared_key_terms = extract_shared_key_terms_from_disk(workspace_root, node_a, node_b);
        }
    });

    collisions
}
