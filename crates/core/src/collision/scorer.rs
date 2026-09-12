//! Pairwise semantic similarity scoring and collision classification engine.

use std::collections::{HashMap, HashSet};

use super::{CollisionClassification, DocumentFingerprint, SemanticCollision};

pub const DEFAULT_WEIGHT_TITLE: f32 = 0.25;
pub const DEFAULT_WEIGHT_HEADING: f32 = 0.20;
pub const DEFAULT_WEIGHT_BM25: f32 = 0.30;
pub const DEFAULT_WEIGHT_NGRAM: f32 = 0.15;
pub const DEFAULT_WEIGHT_TAG: f32 = 0.10;

/// Computes Jaccard set similarity |A ∩ B| / |A ∪ B|.
pub fn compute_jaccard<T: std::hash::Hash + Eq>(a: &HashSet<T>, b: &HashSet<T>) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 {
        0.0
    } else {
        (intersection as f32) / (union as f32)
    }
}

/// Computes Cosine similarity of term frequency vectors.
pub fn compute_cosine_tf(a: &HashMap<String, f32>, b: &HashMap<String, f32>) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    for (term, tf_a) in a {
        if let Some(tf_b) = b.get(term) {
            dot += tf_a * tf_b;
        }
    }
    let norm_a: f32 = a.values().map(|v| v * v).sum::<f32>().sqrt();
    let norm_b: f32 = b.values().map(|v| v * v).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot / (norm_a * norm_b)).min(1.0)
    }
}

/// Classifies a collision pair based on structural and lexical heuristics.
pub fn classify_collision(
    a: &DocumentFingerprint,
    b: &DocumentFingerprint,
    composite: f32,
    title_sim: f32,
    heading_sim: f32,
    bm25_sim: f32,
    ngram_sim: f32,
) -> CollisionClassification {
    let headings_a = a.heading_tokens.len();
    let headings_b = b.heading_tokens.len();
    let ngrams_a = a.ngrams.len();
    let ngrams_b = b.ngrams.len();
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

    let has_summary_intent = a.title_tokens.contains("summary")
        || b.title_tokens.contains("summary")
        || a.title_tokens.contains("overview")
        || b.title_tokens.contains("overview");

    // 1. Potential Summary: explicit summary intent or condensed structure
    if length_ratio <= 0.40
        && (has_summary_intent || heading_ratio < 2.0)
        && (title_sim >= 0.35 || bm25_sim >= 0.35)
    {
        return CollisionClassification::PotentialSummary;
    }

    // 2. Potential Specialization: high term overlap with deeper heading structure
    if heading_ratio >= 2.0 && (bm25_sim >= 0.35 || title_sim >= 0.40) {
        return CollisionClassification::PotentialSpecialization;
    }

    // 3. Potential Conflict: high title/heading overlap, but diverging body terms or ngrams
    if (title_sim >= 0.70 || heading_sim >= 0.60) && bm25_sim < 0.50 && ngram_sim < 0.40 {
        return CollisionClassification::PotentialConflict;
    }

    // 4. Potential Fragmentation: moderate composite overlap with fragmented heading structure
    if composite < 0.85 && heading_sim < 0.40 && bm25_sim >= 0.60 {
        return CollisionClassification::PotentialFragmentation;
    }

    // 5. Default high overlap: Potential Duplicate
    CollisionClassification::PotentialDuplicate
}

/// Evaluates pairwise semantic collision between two document fingerprints.
pub fn score_collision(
    a: &DocumentFingerprint,
    b: &DocumentFingerprint,
    threshold: f32,
) -> Option<SemanticCollision> {
    let title_similarity = compute_jaccard(&a.title_tokens, &b.title_tokens);
    let heading_similarity = compute_jaccard(&a.heading_tokens, &b.heading_tokens);
    let bm25_similarity = compute_cosine_tf(&a.term_frequencies, &b.term_frequencies);
    let ngram_similarity = compute_jaccard(&a.ngrams, &b.ngrams);
    let tag_similarity = compute_jaccard(&a.tags, &b.tags);

    let mut total_weight = 0.0f32;
    let mut weighted_sum = 0.0f32;

    // Title is always present
    total_weight += DEFAULT_WEIGHT_TITLE;
    weighted_sum += DEFAULT_WEIGHT_TITLE * title_similarity;

    // Headings
    if !a.heading_tokens.is_empty() || !b.heading_tokens.is_empty() {
        total_weight += DEFAULT_WEIGHT_HEADING;
        weighted_sum += DEFAULT_WEIGHT_HEADING * heading_similarity;
    }

    // BM25
    if !a.term_frequencies.is_empty() || !b.term_frequencies.is_empty() {
        total_weight += DEFAULT_WEIGHT_BM25;
        weighted_sum += DEFAULT_WEIGHT_BM25 * bm25_similarity;
    }

    // Ngrams
    if !a.ngrams.is_empty() || !b.ngrams.is_empty() {
        total_weight += DEFAULT_WEIGHT_NGRAM;
        weighted_sum += DEFAULT_WEIGHT_NGRAM * ngram_similarity;
    }

    // Tags
    if !a.tags.is_empty() || !b.tags.is_empty() {
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

    let classification = classify_collision(
        a,
        b,
        composite_similarity,
        title_similarity,
        heading_similarity,
        bm25_similarity,
        ngram_similarity,
    );

    // Collect shared key terms (sorted by combined TF)
    let mut shared_terms: Vec<(String, f32)> = Vec::new();
    for (term, tf_a) in &a.term_frequencies {
        if let Some(tf_b) = b.term_frequencies.get(term) {
            shared_terms.push((term.clone(), tf_a + tf_b));
        }
    }
    shared_terms.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap_or(std::cmp::Ordering::Equal));
    let shared_key_terms: Vec<String> = shared_terms.into_iter().take(10).map(|(t, _)| t).collect();

    Some(SemanticCollision {
        source_id: a.doc_id.clone(),
        target_id: b.doc_id.clone(),
        composite_similarity,
        title_similarity,
        heading_similarity,
        bm25_similarity,
        ngram_similarity,
        tag_similarity,
        classification,
        shared_key_terms,
        canonical_candidate: None,
        canonical_confidence: 0.0,
        canonical_evidence: Vec::new(),
    })
}
