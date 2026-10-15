//! Context Trace graph traversal, target selection, and BPE token accounting.

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use super::ContextTraceResult;
use crate::collision::fingerprint::tokenize;
use crate::graph::builder::GraphTopology;

/// Selects the best target note in the topology matching query tokens.
pub fn select_query_target(
    topology: &GraphTopology,
    query: &str,
    entrypoint: &str,
) -> Option<(String, f32)> {
    let q_tokens = tokenize(query);
    if q_tokens.is_empty() {
        return None;
    }

    let mut best_node = None;
    let mut best_score = 0.0f32;

    for (id, node) in &topology.nodes {
        if id == entrypoint {
            continue;
        }

        let mut score = 0.0f32;
        let title_tokens: HashSet<String> = tokenize(&node.label).into_iter().collect();

        // Title matches weighted 4.0
        for qt in &q_tokens {
            if title_tokens.contains(qt) {
                score += 4.0;
            }
        }

        // Heading matches weighted 2.5
        if let Some(headings) = topology.headings.get(id) {
            for h in headings {
                for ht in tokenize(h) {
                    if q_tokens.contains(&ht) {
                        score += 2.5;
                    }
                }
            }
        }

        // Tag matches weighted 2.0
        for tag in &node.tags {
            let clean_tag = tag.trim_start_matches('#').to_lowercase();
            if q_tokens.contains(&clean_tag) {
                score += 2.0;
            }
        }

        if score > best_score {
            best_score = score;
            best_node = Some(id.clone());
        }
    }

    best_node.map(|id| (id, best_score))
}

/// Finds the shortest directed path from entrypoint to target_node over valid (non-broken) edges.
pub fn find_shortest_path(
    topology: &GraphTopology,
    entrypoint: &str,
    target: &str,
) -> Option<Vec<String>> {
    if entrypoint == target {
        return Some(vec![entrypoint.to_string()]);
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

    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();

    queue.push_back((entrypoint.to_string(), vec![entrypoint.to_string()]));
    visited.insert(entrypoint.to_string());

    while let Some((curr, path)) = queue.pop_front() {
        if curr == target {
            return Some(path);
        }

        if let Some(neighbors) = adj.get(&curr) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    let mut next_path = path.clone();
                    next_path.push(neighbor.clone());
                    queue.push_back((neighbor.clone(), next_path));
                }
            }
        }
    }

    None
}

/// Executes complete Context Trace algorithm for a given query and entrypoint.
pub fn execute_context_trace(
    topology: &GraphTopology,
    query: &str,
    entrypoint: Option<&str>,
) -> ContextTraceResult {
    let start = Instant::now();

    // Default entrypoint precedence: AGENTS.md -> README.md -> first hub -> first node
    let designated_entry = entrypoint
        .map(|s| s.to_string())
        .or_else(|| {
            if topology.nodes.contains_key("AGENTS.md") {
                Some("AGENTS.md".to_string())
            } else if topology.nodes.contains_key("README.md") {
                Some("README.md".to_string())
            } else {
                topology.suggest_hub()
            }
        })
        .unwrap_or_else(|| "AGENTS.md".to_string());

    let target_opt = select_query_target(topology, query, &designated_entry);

    if target_opt.is_none() {
        return ContextTraceResult {
            query: query.to_string(),
            entrypoint: designated_entry,
            target_node: String::new(),
            path: Vec::new(),
            depth: 0,
            path_quality_score: 0.0,
            cumulative_bpe_tokens: 0,
            relevant_tokens: 0,
            weak_intermediate_nodes: Vec::new(),
            latency_ms: start.elapsed().as_millis() as u64,
        };
    }

    let (target_node, _relevance) = target_opt.unwrap();
    let path_opt = find_shortest_path(topology, &designated_entry, &target_node);

    let (path, depth) = if let Some(p) = path_opt {
        let d = (p.len() as u32).saturating_sub(1);
        (p, d)
    } else {
        (Vec::new(), 0)
    };

    let mut cumulative_tokens = 0u32;
    let mut target_tokens = 0u32;
    let mut weak_nodes = Vec::new();

    for (idx, node_id) in path.iter().enumerate() {
        if let Some(node) = topology.nodes.get(node_id) {
            cumulative_tokens += node.tokens;
            if idx == path.len() - 1 {
                target_tokens = node.tokens;
            } else if idx > 0 {
                // Intermediate node check
                let q_tokens = tokenize(query);
                let title_tokens: HashSet<String> = tokenize(&node.label).into_iter().collect();
                let matches = q_tokens
                    .iter()
                    .filter(|t| title_tokens.contains(*t))
                    .count();
                if matches == 0 {
                    weak_nodes.push(node_id.clone());
                }
            }
        }
    }

    // Path Quality Percentage calculation
    let path_quality_score = if path.is_empty() {
        0.0f32
    } else {
        let mut score = 100.0f32;
        // Deduct 4% per hop beyond 1
        if depth > 1 {
            score -= (depth - 1) as f32 * 4.0;
        }
        // Deduct 5% per weak intermediate node
        score -= weak_nodes.len() as f32 * 5.0;

        // Token load penalty if path exceeds 6,000 tokens
        if cumulative_tokens > 6000 {
            let ratio = 6000.0 / (cumulative_tokens as f32);
            score *= ratio;
        }

        score.clamp(0.0, 100.0)
    };

    ContextTraceResult {
        query: query.to_string(),
        entrypoint: designated_entry,
        target_node,
        path,
        depth,
        path_quality_score,
        cumulative_bpe_tokens: cumulative_tokens,
        relevant_tokens: target_tokens,
        weak_intermediate_nodes: weak_nodes,
        latency_ms: start.elapsed().as_millis() as u64,
    }
}

/// Result of running Context Trace anchored strictly to the active Harness Entry Point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct HarnessTraceResult {
    pub harness_file: String,
    pub total_hops: u32,
    pub total_tokens_traversed: usize,
    pub target_fragment: String,
    pub path: Vec<String>,
    pub relevance_progression: Vec<f32>,
}

/// Runs Context Trace anchored strictly to the configured Harness Entry Point.
pub fn trace_from_harness(
    topology: &GraphTopology,
    query: &str,
    harness_file: &str,
    _max_hops: u32,
) -> HarnessTraceResult {
    let target_opt = select_query_target(topology, query, harness_file);
    if target_opt.is_none() {
        return HarnessTraceResult {
            harness_file: harness_file.to_string(),
            total_hops: 0,
            total_tokens_traversed: 0,
            target_fragment: String::new(),
            path: Vec::new(),
            relevance_progression: Vec::new(),
        };
    }

    let (target_node, _rel) = target_opt.unwrap();
    let path_opt = find_shortest_path(topology, harness_file, &target_node);

    let path = path_opt.unwrap_or_else(|| vec![harness_file.to_string(), target_node.clone()]);
    let total_hops = (path.len() as u32).saturating_sub(1);

    let mut total_tokens = 0usize;
    let mut relevance_progression = Vec::new();
    let q_tokens = tokenize(query);

    for (idx, node_id) in path.iter().enumerate() {
        if let Some(node) = topology.nodes.get(node_id) {
            total_tokens += node.tokens as usize;
            if idx == 0 {
                relevance_progression.push(0.1);
            } else if idx == path.len() - 1 {
                relevance_progression.push(1.0);
            } else {
                let title_tokens: HashSet<String> = tokenize(&node.label).into_iter().collect();
                let matches = q_tokens.iter().filter(|t| title_tokens.contains(*t)).count();
                let rel = if !q_tokens.is_empty() {
                    (matches as f32 / q_tokens.len() as f32).max(0.3)
                } else {
                    0.5
                };
                relevance_progression.push(rel);
            }
        } else {
            relevance_progression.push(0.5);
        }
    }

    HarnessTraceResult {
        harness_file: harness_file.to_string(),
        total_hops,
        total_tokens_traversed: total_tokens,
        target_fragment: target_node,
        path,
        relevance_progression,
    }
}

