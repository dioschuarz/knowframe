//! Semantic Networks and Concept Maps KOS evaluators.
//! Implements FR-015 and FR-016.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct SemanticNetworkGraph {
    pub nodes: HashSet<String>,
    pub typed_edges: Vec<(String, String, String)>,
}

impl SemanticNetworkGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_typed_edge(&mut self, source: &str, target: &str, relation_type: &str) {
        self.nodes.insert(source.to_string());
        self.nodes.insert(target.to_string());
        self.typed_edges.push((source.to_string(), target.to_string(), relation_type.to_string()));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNetworkEvaluation {
    pub total_nodes: usize,
    pub total_typed_edges: usize,
    pub shannon_diversity: f64,
    pub relational_density: f64,
}

pub fn evaluate_semantic_network(network: &SemanticNetworkGraph) -> SemanticNetworkEvaluation {
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for (_, _, rel) in &network.typed_edges {
        *type_counts.entry(rel.clone()).or_default() += 1;
    }

    let m = network.typed_edges.len() as f64;
    let mut shannon_diversity = 0.0;
    if m > 0.0 {
        for count in type_counts.values() {
            let p = *count as f64 / m;
            if p > 0.0 {
                shannon_diversity -= p * p.log2();
            }
        }
    }

    let n = network.nodes.len();
    let relational_density = if n > 1 {
        m / (n * (n - 1)) as f64
    } else {
        0.0
    };

    SemanticNetworkEvaluation {
        total_nodes: n,
        total_typed_edges: network.typed_edges.len(),
        shannon_diversity,
        relational_density,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptProposition {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapEvaluation {
    pub proposition_count: usize,
    pub proposition_density: f64,
}

pub fn evaluate_concept_map(
    propositions: &[ConceptProposition],
    total_words: usize,
) -> ConceptMapEvaluation {
    let proposition_count = propositions.len();
    let proposition_density = if total_words > 0 {
        proposition_count as f64 / total_words as f64
    } else {
        0.0
    };

    ConceptMapEvaluation {
        proposition_count,
        proposition_density,
    }
}
