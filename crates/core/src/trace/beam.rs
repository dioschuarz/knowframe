//! Constrained beam search for section-level Context Trace.
//! FR-030: Beam width W <= 5, max depth D <= 4 from AGENTS.md or Hub.
//! FR-031: Candidate scoring: 0.4 BM25/Lexical + 0.3 TitleMatch + 0.2 GraphProximity + 0.1 Centrality.
//! FR-032: Section-level refinement routing to doc.md#fragment.
//! FR-033: Context reduction % = (T_doc - T_frag) / T_doc * 100%.

use super::ContextTraceSectionResult;
use crate::collision::fingerprint::tokenize;
use crate::graph::builder::GraphTopology;
use crate::model::NoteNode;
use std::collections::{HashMap, HashSet};

/// Calculates quantitative context reduction percentage:
/// (T_doc - T_fragment) / T_doc * 100%
/// Defaulting to 0.0% if no fragment targeted or invalid.
pub fn calculate_context_reduction(doc_tokens: u32, fragment_tokens: Option<u32>) -> f64 {
    match fragment_tokens {
        Some(f_tokens) if doc_tokens > 0 && f_tokens < doc_tokens => {
            let reduction = (doc_tokens - f_tokens) as f64 / doc_tokens as f64 * 100.0;
            (reduction * 10.0).round() / 10.0
        }
        _ => 0.0,
    }
}

/// Computes composite candidate score for a document node.
/// Score(u) = 0.4 * FragmentMatch + 0.3 * TitleMatch + 0.2 * GraphProximity + 0.1 * Centrality.
pub fn compute_candidate_score(
    node: &NoteNode,
    query_tokens: &[String],
    depth: usize,
) -> (f64, Vec<String>) {
    if query_tokens.is_empty() {
        return (0.0, Vec::new());
    }

    let mut reasons = Vec::new();
    let q_set: HashSet<_> = query_tokens.iter().collect();

    // 1. TitleMatch (0.3 weight): overlap of title tokens with query tokens
    let title_toks = tokenize(&node.label);
    let title_matches: Vec<_> = title_toks.iter().filter(|t| q_set.contains(t)).collect();
    let title_score = if !query_tokens.is_empty() {
        (title_matches.len() as f64 / query_tokens.len() as f64).min(1.0)
    } else {
        0.0
    };
    if !title_matches.is_empty() {
        reasons.push(format!(
            "Document title matches query terms: {:?}",
            title_matches
        ));
    }

    // 2. FragmentMatch (0.4 weight): best fragment heading match
    let mut best_frag_score = 0.0f64;
    let mut best_frag_name = None;
    for frag in &node.fragments {
        let frag_toks = tokenize(&frag.heading);
        let frag_matches: Vec<_> = frag_toks.iter().filter(|t| q_set.contains(t)).collect();
        let frag_s = if !query_tokens.is_empty() {
            (frag_matches.len() as f64 / query_tokens.len() as f64).min(1.0)
        } else {
            0.0
        };
        if frag_s > best_frag_score {
            best_frag_score = frag_s;
            best_frag_name = Some(frag.anchor.clone());
        }
    }
    if let Some(ref anchor) = best_frag_name {
        if best_frag_score > 0.0 {
            reasons.push(format!(
                "Section heading '#{}' matches query terms ({:.0}% match)",
                anchor,
                best_frag_score * 100.0
            ));
        }
    }

    // 3. GraphProximity (0.2 weight): 1.0 at depth 1, decaying down
    let proximity_score = if depth <= 4 {
        1.0 - (depth.saturating_sub(1) as f64 * 0.2)
    } else {
        0.2
    };

    // 4. Centrality (0.1 weight): Brandes betweenness bounded in [0.0, 1.0]
    let centrality_score = node.betweenness.clamp(0.0, 1.0);

    let composite_score =
        0.4 * best_frag_score + 0.3 * title_score + 0.2 * proximity_score + 0.1 * centrality_score;

    (composite_score, reasons)
}

/// Refines section target on document: if a fragment has high query match (>= 0.75 or best match > 0.0),
/// routes to doc.md#fragment. Otherwise targets document directly.
pub fn refine_section_target(
    node: &NoteNode,
    query_tokens: &[String],
) -> (Option<String>, Option<u32>, f64, Vec<String>) {
    if node.fragments.is_empty() || query_tokens.is_empty() {
        return (None, None, 0.0, Vec::new());
    }

    let q_set: HashSet<_> = query_tokens.iter().collect();
    let mut best_frag = None;
    let mut highest_score = 0.0f64;
    let mut matching_terms = Vec::new();

    for frag in &node.fragments {
        let frag_toks = tokenize(&frag.heading);
        let matches: Vec<String> = frag_toks
            .into_iter()
            .filter(|t| q_set.contains(t))
            .collect();
        let s = matches.len() as f64 / query_tokens.len() as f64;
        if s > highest_score {
            highest_score = s;
            best_frag = Some(frag);
            matching_terms = matches;
        }
    }

    if let Some(frag) = best_frag {
        // Route directly if heading matches query terms
        if highest_score > 0.0 {
            let reduction = calculate_context_reduction(node.tokens, Some(frag.tokens));
            let reasons = vec![format!(
                "Section '#{}' matched query terms: {:?} (context reduced by {:.1}%)",
                frag.anchor, matching_terms, reduction
            )];
            return (
                Some(frag.anchor.clone()),
                Some(frag.tokens),
                reduction,
                reasons,
            );
        }
    }

    (None, None, 0.0, Vec::new())
}

/// Executes constrained beam search from entrypoint targeting relevant document nodes.
/// Beam width W <= 5, Max depth D <= 4.
pub fn beam_search(
    topology: &GraphTopology,
    entrypoint: &str,
    query: &str,
    beam_width: usize,
    max_depth: usize,
) -> Option<(Vec<String>, f64)> {
    let query_tokens = tokenize(query);
    if query_tokens.is_empty() {
        return None;
    }

    // Build directed adjacency list of valid edges
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &topology.edges {
        if !edge.is_broken {
            adj.entry(edge.source.clone())
                .or_default()
                .push(edge.target.clone());
        }
    }

    // Initial beam contains entrypoint
    let mut current_beam: Vec<(Vec<String>, f64)> = vec![(vec![entrypoint.to_string()], 0.0)];
    let mut best_path = None;
    let mut highest_score = -1.0f64;

    for depth in 1..=max_depth {
        let mut candidates: Vec<(Vec<String>, f64)> = Vec::new();

        for (path, _) in &current_beam {
            let curr = path.last().unwrap();
            if let Some(neighbors) = adj.get(curr) {
                for nbr in neighbors {
                    if !path.contains(nbr) {
                        let mut new_path = path.clone();
                        new_path.push(nbr.clone());

                        if let Some(node) = topology.get_node(nbr) {
                            let (score, _) = compute_candidate_score(node, &query_tokens, depth);
                            if score > highest_score {
                                highest_score = score;
                                best_path = Some((new_path.clone(), score));
                            }
                            candidates.push((new_path, score));
                        }
                    }
                }
            }
        }

        if candidates.is_empty() {
            break;
        }

        // Keep top W candidate paths
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(beam_width);
        current_beam = candidates;
    }

    best_path
}

/// Full section-level Context Trace execution.
pub fn trace_context_section(
    topology: &GraphTopology,
    query: &str,
    entrypoint_opt: Option<&str>,
) -> Option<ContextTraceSectionResult> {
    let query_tokens = tokenize(query);
    if query_tokens.is_empty() {
        return None;
    }

    // Determine entrypoint
    let entrypoint = if let Some(ep) = entrypoint_opt {
        ep.to_string()
    } else if topology.get_node("AGENTS.md").is_some() {
        "AGENTS.md".to_string()
    } else if let Some(ref hub) = topology.suggest_hub() {
        hub.clone()
    } else {
        topology.nodes.keys().next()?.clone()
    };

    // Run constrained beam search (W=5, D=4)
    let (path, _score) = if let Some(res) = beam_search(topology, &entrypoint, query, 5, 4) {
        res
    } else {
        // Fallback: search best node directly in graph if disconnected
        let mut best_direct = None;
        let mut best_score = -1.0;
        for (id, node) in &topology.nodes {
            if id != &entrypoint {
                let (s, _) = compute_candidate_score(node, &query_tokens, 1);
                if s > best_score {
                    best_score = s;
                    best_direct = Some(id.clone());
                }
            }
        }
        if let Some(target) = best_direct {
            (vec![entrypoint.clone(), target], best_score)
        } else {
            (vec![entrypoint.clone()], 0.0)
        }
    };

    let target_doc = path.last()?.clone();
    let target_node = topology.get_node(&target_doc)?;

    let depth = (path.len() - 1) as u32;
    let (target_fragment, fragment_raw_tokens, reduction_percent, mut reasons) =
        refine_section_target(target_node, &query_tokens);

    let (_, doc_reasons) = compute_candidate_score(target_node, &query_tokens, depth as usize);
    reasons.extend(doc_reasons);

    Some(ContextTraceSectionResult {
        query: query.to_string(),
        entrypoint,
        target_document: target_doc,
        target_fragment,
        path,
        depth,
        document_raw_tokens: target_node.tokens,
        fragment_raw_tokens,
        context_reduction_percent: reduction_percent,
        relevance_reasons: reasons,
    })
}
