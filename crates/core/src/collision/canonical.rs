//! Canonical candidate heuristics based on deterministic graph and harness evidence.

use super::SemanticCollision;

/// Contextual metadata for evaluating canonical precedence between two notes.
#[derive(Debug, Clone)]
pub struct DocumentMetadataContext<'a> {
    pub doc_id: &'a str,
    pub is_entrypoint_referenced: bool,
    pub degree: u32,
    pub betweenness: f32,
    pub heading_count: usize,
    pub token_count: u32,
    pub commit_timestamp: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalEvaluation {
    pub candidate_id: String,
    pub confidence: f32,
    pub evidence: Vec<String>,
}

/// Evaluates deterministic evidence to designate a canonical candidate between two colliding notes.
pub fn evaluate_canonical_candidate(
    doc_a: &DocumentMetadataContext,
    doc_b: &DocumentMetadataContext,
) -> CanonicalEvaluation {
    let mut score_a = 0.0f32;
    let mut score_b = 0.0f32;

    let mut evidence_a = Vec::new();
    let mut evidence_b = Vec::new();

    // 1. Entrypoint citation precedence (AGENTS.md, CLAUDE.md, README.md)
    if doc_a.is_entrypoint_referenced && !doc_b.is_entrypoint_referenced {
        score_a += 3.5;
        evidence_a
            .push("Referenced by project harness entrypoint (AGENTS.md / README.md)".to_string());
    } else if doc_b.is_entrypoint_referenced && !doc_a.is_entrypoint_referenced {
        score_b += 3.5;
        evidence_b
            .push("Referenced by project harness entrypoint (AGENTS.md / README.md)".to_string());
    }

    // 2. Graph Centrality (Degree and Betweenness)
    if doc_a.degree > doc_b.degree {
        let diff = doc_a.degree - doc_b.degree;
        let weight = (diff as f32 * 0.5).min(2.5);
        score_a += weight;
        evidence_a.push(format!(
            "Higher graph connectivity (degree {} vs {})",
            doc_a.degree, doc_b.degree
        ));
    } else if doc_b.degree > doc_a.degree {
        let diff = doc_b.degree - doc_a.degree;
        let weight = (diff as f32 * 0.5).min(2.5);
        score_b += weight;
        evidence_b.push(format!(
            "Higher graph connectivity (degree {} vs {})",
            doc_b.degree, doc_a.degree
        ));
    }

    if doc_a.betweenness > doc_b.betweenness + 0.05 {
        score_a += 1.5;
        evidence_a.push(format!(
            "Higher architectural centrality (betweenness {:.2} vs {:.2})",
            doc_a.betweenness, doc_b.betweenness
        ));
    } else if doc_b.betweenness > doc_a.betweenness + 0.05 {
        score_b += 1.5;
        evidence_b.push(format!(
            "Higher architectural centrality (betweenness {:.2} vs {:.2})",
            doc_b.betweenness, doc_a.betweenness
        ));
    }

    // 3. Structural Heading Completeness
    if doc_a.heading_count > doc_b.heading_count + 1 {
        score_a += 1.5;
        evidence_a.push(format!(
            "More comprehensive section headings ({} vs {})",
            doc_a.heading_count, doc_b.heading_count
        ));
    } else if doc_b.heading_count > doc_a.heading_count + 1 {
        score_b += 1.5;
        evidence_b.push(format!(
            "More comprehensive section headings ({} vs {})",
            doc_b.heading_count, doc_a.heading_count
        ));
    }

    // 4. Content Depth (tokens)
    if doc_a.token_count > (doc_b.token_count as f32 * 1.5) as u32 {
        score_a += 1.0;
        evidence_a.push(format!(
            "Greater technical depth ({} tokens vs {})",
            doc_a.token_count, doc_b.token_count
        ));
    } else if doc_b.token_count > (doc_a.token_count as f32 * 1.5) as u32 {
        score_b += 1.0;
        evidence_b.push(format!(
            "Greater technical depth ({} tokens vs {})",
            doc_b.token_count, doc_a.token_count
        ));
    }

    // 5. Chronology (earlier creation timestamp)
    if let (Some(ts_a), Some(ts_b)) = (doc_a.commit_timestamp, doc_b.commit_timestamp) {
        if ts_a < ts_b {
            score_a += 1.0;
            evidence_a.push("Created earlier in repository chronology".to_string());
        } else if ts_b < ts_a {
            score_b += 1.0;
            evidence_b.push("Created earlier in repository chronology".to_string());
        }
    }

    if score_a >= score_b {
        let diff = score_a - score_b;
        let confidence = (0.50 + diff * 0.10).min(1.0);
        CanonicalEvaluation {
            candidate_id: doc_a.doc_id.to_string(),
            confidence,
            evidence: evidence_a,
        }
    } else {
        let diff = score_b - score_a;
        let confidence = (0.50 + diff * 0.10).min(1.0);
        CanonicalEvaluation {
            candidate_id: doc_b.doc_id.to_string(),
            confidence,
            evidence: evidence_b,
        }
    }
}

/// Applies canonical candidate analysis to a SemanticCollision diagnostic.
pub fn enrich_collision_with_canonical(
    collision: &mut SemanticCollision,
    doc_a: &DocumentMetadataContext,
    doc_b: &DocumentMetadataContext,
) {
    let evaluation = evaluate_canonical_candidate(doc_a, doc_b);
    collision.canonical_candidate = Some(evaluation.candidate_id);
    collision.canonical_confidence = evaluation.confidence;
    collision.canonical_evidence = evaluation.evidence;
}
