//! Context Trace graph traversal and BPE token accounting.

pub mod dijkstra;
pub mod traversal;

use serde::{Deserialize, Serialize};

pub use traversal::*;

/// Result of deterministic graph-constrained pathfinding from entrypoint to query target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextTraceResult {
    pub query: String,
    pub entrypoint: String,
    pub target_node: String,
    pub path: Vec<String>,
    pub depth: u32,
    pub path_quality_score: f32, // 0.0 to 100.0%
    pub cumulative_bpe_tokens: u32,
    pub relevant_tokens: u32,
    pub weak_intermediate_nodes: Vec<String>,
    pub latency_ms: u64,
}

impl Default for ContextTraceResult {
    fn default() -> Self {
        Self {
            query: String::new(),
            entrypoint: String::new(),
            target_node: String::new(),
            path: Vec::new(),
            depth: 0,
            path_quality_score: 0.0,
            cumulative_bpe_tokens: 0,
            relevant_tokens: 0,
            weak_intermediate_nodes: Vec::new(),
            latency_ms: 0,
        }
    }
}
