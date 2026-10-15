//! Knowledge Drift and Context Trace Routing KOS evaluators.
//! Implements FR-026, FR-027, and SC-004.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Evaluates temporal citation turnover (Knowledge Drift / Tennis Subject Ontogeny).
/// Δ_drift(d, t1, t2) = 1 - |Citations(t1) ∩ Citations(t2)| / |Citations(t1) ∪ Citations(t2)|
pub fn evaluate_knowledge_drift(
    t1_citations: &HashSet<String>,
    t2_citations: &HashSet<String>,
) -> f64 {
    let intersection = t1_citations.intersection(t2_citations).count();
    let union = t1_citations.union(t2_citations).count();

    if union > 0 {
        1.0 - (intersection as f64 / union as f64)
    } else {
        0.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTraceHop {
    pub document_id: String,
    pub fragment: Option<String>,
    pub fragment_tokens: u64,
    pub full_document_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTraceEvaluation {
    pub total_trace_tokens: u64,
    pub total_full_tokens: u64,
    pub reduction_percentage: f64,
    pub hops: usize,
}

/// Evaluates Section-Level Context Trace routing and prompt context reduction percentage.
pub fn evaluate_context_trace_routing(hops: &[ContextTraceHop]) -> ContextTraceEvaluation {
    let mut total_trace_tokens = 0;
    let mut total_full_tokens = 0;

    for h in hops {
        total_trace_tokens += h.fragment_tokens;
        total_full_tokens += h.full_document_tokens;
    }

    let reduction_percentage = if total_full_tokens > 0 {
        1.0 - (total_trace_tokens as f64 / total_full_tokens as f64)
    } else {
        0.0
    };

    ContextTraceEvaluation {
        total_trace_tokens,
        total_full_tokens,
        reduction_percentage,
        hops: hops.len(),
    }
}
