//! In-memory GraphTopology with petgraph integration, degrees, betweenness, and BFS hops.

use super::centrality::compute_brandes_betweenness;
use super::hops::assign_bfs_hops;
use super::resolver::{resolve_edge_integrity, ResolverIndex};
use crate::model::{NoteEdge, NoteNode};
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GraphTopology {
    pub nodes: HashMap<String, NoteNode>,
    pub edges: Vec<NoteEdge>,
    pub headings: HashMap<String, Vec<String>>,
}

impl Default for GraphTopology {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphTopology {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            headings: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: NoteNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: NoteEdge) {
        self.edges.push(edge);
    }

    pub fn set_headings(&mut self, file_id: String, headings: Vec<String>) {
        self.headings.insert(file_id, headings);
    }

    pub fn get_node(&self, id: &str) -> Option<&NoteNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut NoteNode> {
        self.nodes.get_mut(id)
    }

    /// Builds a directed adjacency list representation of valid edges between known nodes.
    pub fn build_adjacency(&self) -> HashMap<String, Vec<String>> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for id in self.nodes.keys() {
            adj.insert(id.clone(), Vec::new());
        }

        for edge in &self.edges {
            if !edge.is_broken && self.nodes.contains_key(&edge.source) && self.nodes.contains_key(&edge.target) {
                // Avoid duplicating multi-edges in adjacency
                let neighbors = adj.entry(edge.source.clone()).or_default();
                if !neighbors.contains(&edge.target) {
                    neighbors.push(edge.target.clone());
                }
            }
        }

        adj
    }

    /// Computes in-degree and out-degree metrics for all nodes.
    pub fn compute_degrees(&mut self) {
        let mut in_degrees: HashMap<String, u32> = self.nodes.keys().map(|k| (k.clone(), 0)).collect();
        let mut out_degrees: HashMap<String, u32> = self.nodes.keys().map(|k| (k.clone(), 0)).collect();
        let mut broken_counts: HashMap<String, u32> = self.nodes.keys().map(|k| (k.clone(), 0)).collect();

        for edge in &self.edges {
            if edge.is_broken {
                *broken_counts.entry(edge.source.clone()).or_insert(0) += 1;
            } else {
                if self.nodes.contains_key(&edge.source) && self.nodes.contains_key(&edge.target) {
                    *out_degrees.entry(edge.source.clone()).or_insert(0) += 1;
                    *in_degrees.entry(edge.target.clone()).or_insert(0) += 1;
                }
            }
        }

        for (id, node) in self.nodes.iter_mut() {
            node.in_degree = *in_degrees.get(id).unwrap_or(&0);
            node.out_degree = *out_degrees.get(id).unwrap_or(&0);
            node.broken_links = *broken_counts.get(id).unwrap_or(&0);
        }
    }

    /// Computes Brandes betweenness centrality for all nodes.
    pub fn compute_betweenness(&mut self) {
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        let adj = self.build_adjacency();
        let centrality_map = compute_brandes_betweenness(&node_ids, &adj);

        for (id, score) in centrality_map {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.betweenness = score;
            }
        }
    }

    /// Suggests the highest betweenness centrality node as the default Level 0 Hub.
    pub fn suggest_hub(&self) -> Option<String> {
        self.nodes
            .values()
            .max_by(|a, b| {
                a.betweenness
                    .partial_cmp(&b.betweenness)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| (a.in_degree + a.out_degree).cmp(&(b.in_degree + b.out_degree)))
            })
            .map(|n| n.id.clone())
    }

    /// Computes BFS shortest-path hop distances from the designated hub.
    pub fn compute_hops(&mut self, hub_id: &str) {
        let adj = self.build_adjacency();
        assign_bfs_hops(hub_id, &mut self.nodes, &adj);
    }

    /// Resolves all edges: normalizes relative paths, validates disk presence and section heading anchors.
    pub fn resolve_links(&mut self) {
        let known_nodes: HashMap<String, PathBuf> = self
            .nodes
            .iter()
            .map(|(k, v)| (k.clone(), PathBuf::from(&v.file_path)))
            .collect();

        let index = ResolverIndex::new(&known_nodes);

        for edge in &mut self.edges {
            resolve_edge_integrity(edge, &index, &self.headings);
        }

        self.compute_degrees();
    }

    /// Converts graph to petgraph DiGraph representation.
    pub fn to_petgraph(&self) -> DiGraph<String, ()> {
        let mut graph = DiGraph::new();
        let mut index_map = HashMap::new();

        for id in self.nodes.keys() {
            let idx = graph.add_node(id.clone());
            index_map.insert(id.clone(), idx);
        }

        for edge in &self.edges {
            if !edge.is_broken {
                if let (Some(&src_idx), Some(&tgt_idx)) = (index_map.get(&edge.source), index_map.get(&edge.target)) {
                    graph.add_edge(src_idx, tgt_idx, ());
                }
            }
        }

        graph
    }
}
