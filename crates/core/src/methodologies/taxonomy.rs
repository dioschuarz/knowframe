//! Classification and Taxonomy KOS evaluators.
//! Implements FR-010, FR-011, and SC-008.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationCoverageResult {
    pub total_documents: usize,
    pub classified_documents: usize,
    pub coverage_ratio: f64,
    pub unclassified_documents: Vec<String>,
}

pub fn evaluate_classification_coverage(
    doc_categories: &HashMap<String, Option<String>>,
) -> ClassificationCoverageResult {
    let total_documents = doc_categories.len();
    let mut unclassified_documents = Vec::new();
    let mut classified_count = 0;

    for (doc, category) in doc_categories {
        if category.is_some() {
            classified_count += 1;
        } else {
            unclassified_documents.push(doc.clone());
        }
    }
    unclassified_documents.sort();

    let coverage_ratio = if total_documents > 0 {
        classified_count as f64 / total_documents as f64
    } else {
        1.0
    };

    ClassificationCoverageResult {
        total_documents,
        classified_documents: classified_count,
        coverage_ratio,
        unclassified_documents,
    }
}

#[derive(Debug, Clone, Default)]
pub struct TaxonomyGraph {
    pub nodes: HashSet<String>,
    pub children: HashMap<String, Vec<String>>,
    pub parents: HashMap<String, Vec<String>>,
}

impl TaxonomyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: &str) {
        self.nodes.insert(node.to_string());
    }

    pub fn add_child(&mut self, parent: &str, child: &str) {
        self.nodes.insert(parent.to_string());
        self.nodes.insert(child.to_string());
        self.children.entry(parent.to_string()).or_default().push(child.to_string());
        self.parents.entry(child.to_string()).or_default().push(parent.to_string());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyEvaluation {
    pub cycles: Vec<Vec<String>>,
    pub imbalance_ratio: f64,
    pub pathological_branching: Vec<(String, usize)>,
}

pub fn evaluate_taxonomy(graph: &TaxonomyGraph) -> TaxonomyEvaluation {
    let mut cycles = Vec::new();

    // Tarjan's SCC algorithm for cycle detection
    let mut index = 0;
    let mut indices: HashMap<String, usize> = HashMap::new();
    let mut lowlink: HashMap<String, usize> = HashMap::new();
    let mut on_stack: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();

    fn strongconnect(
        v: &str,
        graph: &TaxonomyGraph,
        index: &mut usize,
        indices: &mut HashMap<String, usize>,
        lowlink: &mut HashMap<String, usize>,
        on_stack: &mut HashSet<String>,
        stack: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        indices.insert(v.to_string(), *index);
        lowlink.insert(v.to_string(), *index);
        *index += 1;
        stack.push(v.to_string());
        on_stack.insert(v.to_string());

        if let Some(children) = graph.children.get(v) {
            for w in children {
                if !indices.contains_key(w) {
                    strongconnect(w, graph, index, indices, lowlink, on_stack, stack, cycles);
                    let w_low = lowlink[w];
                    let v_low = lowlink.get_mut(v).unwrap();
                    *v_low = (*v_low).min(w_low);
                } else if on_stack.contains(w) {
                    let w_idx = indices[w];
                    let v_low = lowlink.get_mut(v).unwrap();
                    *v_low = (*v_low).min(w_idx);
                }
            }
        }

        if lowlink[v] == indices[v] {
            let mut scc = Vec::new();
            while let Some(w) = stack.pop() {
                on_stack.remove(&w);
                scc.push(w.clone());
                if w == v {
                    break;
                }
            }
            if scc.len() > 1 || (graph.children.get(v).map_or(false, |c| c.contains(&v.to_string()))) {
                scc.reverse();
                cycles.push(scc);
            }
        }
    }

    let mut sorted_nodes: Vec<_> = graph.nodes.iter().cloned().collect();
    sorted_nodes.sort();
    for node in &sorted_nodes {
        if !indices.contains_key(node) {
            strongconnect(
                node,
                graph,
                &mut index,
                &mut indices,
                &mut lowlink,
                &mut on_stack,
                &mut stack,
                &mut cycles,
            );
        }
    }

    // Branching factor
    let mut pathological_branching = Vec::new();
    for (node, children) in &graph.children {
        if children.len() > 15 {
            pathological_branching.push((node.clone(), children.len()));
        }
    }

    // Calculate leaf depths from roots
    let roots: Vec<_> = graph.nodes.iter().filter(|n| !graph.parents.contains_key(*n)).collect();
    let mut leaf_depths = Vec::new();

    fn dfs_depth(node: &str, depth: usize, graph: &TaxonomyGraph, leaf_depths: &mut Vec<usize>, visited: &mut HashSet<String>) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_string());
        if let Some(children) = graph.children.get(node) {
            if children.is_empty() {
                leaf_depths.push(depth);
            } else {
                for c in children {
                    dfs_depth(c, depth + 1, graph, leaf_depths, visited);
                }
            }
        } else {
            leaf_depths.push(depth);
        }
    }

    for root in &roots {
        let mut visited = HashSet::new();
        dfs_depth(root, 0, graph, &mut leaf_depths, &mut visited);
    }

    let imbalance_ratio = if !leaf_depths.is_empty() {
        let n = leaf_depths.len() as f64;
        let mean = leaf_depths.iter().sum::<usize>() as f64 / n;
        if mean > 0.0 {
            let variance = leaf_depths.iter().map(|d| (*d as f64 - mean).powi(2)).sum::<f64>() / n;
            variance.sqrt() / mean
        } else {
            0.0
        }
    } else {
        0.0
    };

    TaxonomyEvaluation {
        cycles,
        imbalance_ratio,
        pathological_branching,
    }
}
