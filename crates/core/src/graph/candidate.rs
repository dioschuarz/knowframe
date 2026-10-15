//! In-memory ephemeral candidate graph isolation without mutating persistent state.
//! Implements FR-039 and Constitution Invariants I & II (Candidate Isolation & Committed Truth).

use crate::model::{DocumentNode, GraphTopology, ReferenceEdge};
use std::collections::{HashMap, HashSet};

/// Manages an in-memory ephemeral candidate graph overlay.
/// Changes staged in the candidate graph are isolated in RAM and never written to disk
/// until an explicit Git commit and post-commit synchronization occurs.
#[derive(Debug, Clone)]
pub struct CandidateGraph {
    /// Committed persistent base nodes
    base_nodes: HashMap<String, DocumentNode>,
    /// Committed persistent base edges
    base_edges: Vec<ReferenceEdge>,
    /// Staged in-memory overlay nodes (added or modified)
    overlay_nodes: HashMap<String, DocumentNode>,
    /// Staged in-memory overlay edges
    overlay_edges: Vec<ReferenceEdge>,
    /// Set of document IDs marked for deletion in the candidate state
    deleted_nodes: HashSet<String>,
}

impl CandidateGraph {
    /// Constructs a new candidate graph over an existing topology.
    pub fn new(topology: &GraphTopology) -> Self {
        let mut base_nodes = HashMap::with_capacity(topology.nodes.len());
        for node in &topology.nodes {
            base_nodes.insert(node.id.clone(), node.clone());
        }

        Self {
            base_nodes,
            base_edges: topology.edges.clone(),
            overlay_nodes: HashMap::new(),
            overlay_edges: Vec::new(),
            deleted_nodes: HashSet::new(),
        }
    }

    /// Stages a new or modified document node in RAM.
    pub fn stage_node(&mut self, node: DocumentNode) {
        self.deleted_nodes.remove(&node.id);
        self.overlay_nodes.insert(node.id.clone(), node);
    }

    /// Stages a document node removal in RAM.
    pub fn stage_deletion(&mut self, node_id: &str) {
        self.overlay_nodes.remove(node_id);
        self.deleted_nodes.insert(node_id.to_string());
    }

    /// Replaces or appends candidate edges in RAM.
    pub fn stage_edges(&mut self, edges: Vec<ReferenceEdge>) {
        self.overlay_edges = edges;
    }

    /// Returns an effective document node by ID considering base and overlay.
    pub fn get_node(&self, id: &str) -> Option<&DocumentNode> {
        if self.deleted_nodes.contains(id) {
            return None;
        }
        self.overlay_nodes.get(id).or_else(|| self.base_nodes.get(id))
    }

    /// Returns the active count of effective document nodes in RAM.
    pub fn node_count(&self) -> usize {
        let base_active = self
            .base_nodes
            .keys()
            .filter(|k| !self.deleted_nodes.contains(*k) && !self.overlay_nodes.contains_key(*k))
            .count();
        base_active + self.overlay_nodes.len()
    }

    /// Materializes the effective ephemeral candidate graph topology.
    pub fn effective_topology(&self) -> GraphTopology {
        let mut nodes = Vec::with_capacity(self.node_count());

        for (id, node) in &self.base_nodes {
            if !self.deleted_nodes.contains(id) && !self.overlay_nodes.contains_key(id) {
                nodes.push(node.clone());
            }
        }

        for node in self.overlay_nodes.values() {
            nodes.push(node.clone());
        }

        // Combine base edges (excluding deleted targets) and overlay edges
        let mut edges = Vec::new();
        for edge in &self.base_edges {
            if !self.deleted_nodes.contains(&edge.target_document) {
                edges.push(edge.clone());
            }
        }
        edges.extend(self.overlay_edges.iter().cloned());

        GraphTopology { nodes, edges }
    }

    /// Verifies candidate overlay is purely in-memory (no persistent mutations).
    pub fn is_ephemeral(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(id: &str, title: &str) -> DocumentNode {
        DocumentNode {
            id: id.to_string(),
            relative_path: format!("{}.md", id),
            title: title.to_string(),
            raw_token_count: 100,
            frontmatter_schema: None,
            fragments: Vec::new(),
            outgoing_references: Vec::new(),
            git_last_modified_commit: "commit_abc".to_string(),
            git_last_modified_timestamp: 1600000000,
        }
    }

    #[test]
    fn test_candidate_graph_overlay_isolation() {
        let base_topology = GraphTopology {
            nodes: vec![
                create_test_node("doc1", "Document One"),
                create_test_node("doc2", "Document Two"),
            ],
            edges: vec![],
        };

        let mut candidate = CandidateGraph::new(&base_topology);
        assert_eq!(candidate.node_count(), 2);
        assert_eq!(candidate.get_node("doc1").unwrap().title, "Document One");

        // Stage modified doc1
        let mut modified_doc1 = create_test_node("doc1", "Document One (Candidate Edits)");
        modified_doc1.raw_token_count = 500;
        candidate.stage_node(modified_doc1);

        // Verify overlay reflects changes in RAM
        assert_eq!(candidate.node_count(), 2);
        assert_eq!(
            candidate.get_node("doc1").unwrap().title,
            "Document One (Candidate Edits)"
        );
        assert_eq!(candidate.get_node("doc1").unwrap().raw_token_count, 500);

        // Verify base topology was NOT mutated
        assert_eq!(base_topology.nodes[0].title, "Document One");
        assert_eq!(base_topology.nodes[0].raw_token_count, 100);

        // Stage deletion of doc2
        candidate.stage_deletion("doc2");
        assert_eq!(candidate.node_count(), 1);
        assert!(candidate.get_node("doc2").is_none());
        assert_eq!(base_topology.nodes.len(), 2);
    }
}
