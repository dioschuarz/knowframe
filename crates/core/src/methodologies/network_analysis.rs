//! Knowledge Graph / Network Analysis KOS evaluator.
//! Implements FR-023 and SC-008.

use petgraph::graph::UnGraph;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Default)]
pub struct NetworkGraph {
    pub nodes: HashSet<String>,
    pub edges: Vec<(String, String)>,
}

impl NetworkGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, source: &str, target: &str) {
        self.nodes.insert(source.to_string());
        self.nodes.insert(target.to_string());
        self.edges.push((source.to_string(), target.to_string()));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAnalysisEvaluation {
    pub betweenness: HashMap<String, f64>,
    pub pagerank: HashMap<String, f64>,
    pub bridges: Vec<(String, String)>,
}

/// Computes Brandes betweenness centrality, PageRank, and bridges.
pub fn evaluate_network_analysis(graph: &NetworkGraph) -> NetworkAnalysisEvaluation {
    let mut nodes_vec: Vec<String> = graph.nodes.iter().cloned().collect();
    nodes_vec.sort();

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for (s, t) in &graph.edges {
        adj.entry(s.clone()).or_default().push(t.clone());
        adj.entry(t.clone()).or_default().push(s.clone()); // undirected view for betweenness and bridges
    }

    // 1. Brandes Betweenness Centrality
    let mut cb: HashMap<String, f64> = nodes_vec.iter().map(|n| (n.clone(), 0.0)).collect();

    for s in &nodes_vec {
        let mut stack = Vec::new();
        let mut pred: HashMap<String, Vec<String>> = HashMap::new();
        let mut sigma: HashMap<String, f64> = nodes_vec.iter().map(|n| (n.clone(), 0.0)).collect();
        let mut dist: HashMap<String, i32> = nodes_vec.iter().map(|n| (n.clone(), -1)).collect();

        sigma.insert(s.clone(), 1.0);
        dist.insert(s.clone(), 0);

        let mut queue = VecDeque::new();
        queue.push_back(s.clone());

        while let Some(v) = queue.pop_front() {
            stack.push(v.clone());
            let d_v = dist[&v];
            if let Some(neighbors) = adj.get(&v) {
                for w in neighbors {
                    if dist[w] < 0 {
                        dist.insert(w.clone(), d_v + 1);
                        queue.push_back(w.clone());
                    }
                    if dist[w] == d_v + 1 {
                        let sigma_v = sigma[&v];
                        *sigma.get_mut(w).unwrap() += sigma_v;
                        pred.entry(w.clone()).or_default().push(v.clone());
                    }
                }
            }
        }

        let mut delta: HashMap<String, f64> = nodes_vec.iter().map(|n| (n.clone(), 0.0)).collect();
        while let Some(w) = stack.pop() {
            if let Some(predecessors) = pred.get(&w) {
                let delta_w = delta[&w];
                let sigma_w = sigma[&w];
                for v in predecessors {
                    let sigma_v = sigma[v];
                    let coeff = (sigma_v / sigma_w) * (1.0 + delta_w);
                    *delta.get_mut(v).unwrap() += coeff;
                }
            }
            if w != *s {
                *cb.get_mut(&w).unwrap() += delta[&w];
            }
        }
    }

    // Normalize betweenness
    let n = nodes_vec.len() as f64;
    let norm = if n > 2.0 { 2.0 / ((n - 1.0) * (n - 2.0)) } else { 1.0 };
    for score in cb.values_mut() {
        *score = *score * norm * 0.5; // halve because undirected paths are counted twice
    }

    // 2. Power-iteration PageRank
    let mut pr: HashMap<String, f64> = nodes_vec.iter().map(|node| (node.clone(), 1.0 / n.max(1.0))).collect();
    let d = 0.85;
    for _ in 0..50 {
        let mut next_pr: HashMap<String, f64> = HashMap::new();
        for u in &nodes_vec {
            let mut sum = 0.0;
            // incoming from neighbors
            if let Some(neighbors) = adj.get(u) {
                for v in neighbors {
                    let deg = adj[v].len() as f64;
                    if deg > 0.0 {
                        sum += pr[v] / deg;
                    }
                }
            }
            next_pr.insert(u.clone(), (1.0 - d) / n.max(1.0) + d * sum);
        }
        pr = next_pr;
    }

    // 3. Bridges detection
    let mut petgraph_graph = UnGraph::<String, ()>::new_undirected();
    let mut node_indices = HashMap::new();
    for node in &nodes_vec {
        let idx = petgraph_graph.add_node(node.clone());
        node_indices.insert(node.clone(), idx);
    }

    let mut bridges = Vec::new();
    for (s, t) in &graph.edges {
        let idx_s = node_indices[s];
        let idx_t = node_indices[t];
        let edge_idx = petgraph_graph.add_edge(idx_s, idx_t, ());

        // Check if removing edge disconnects components
        petgraph_graph.remove_edge(edge_idx);
        let scc_before = petgraph::algo::connected_components(&petgraph_graph);
        petgraph_graph.add_edge(idx_s, idx_t, ());
        let scc_with = petgraph::algo::connected_components(&petgraph_graph);

        if scc_before > scc_with {
            bridges.push((s.clone(), t.clone()));
        }
    }

    NetworkAnalysisEvaluation {
        betweenness: cb,
        pagerank: pr,
        bridges,
    }
}
