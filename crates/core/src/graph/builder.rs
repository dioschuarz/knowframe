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
            if !edge.is_broken
                && self.nodes.contains_key(&edge.source)
                && self.nodes.contains_key(&edge.target)
            {
                // Avoid duplicating multi-edges in adjacency
                let neighbors = adj.entry(edge.source.clone()).or_default();
                if !neighbors.contains(&edge.target) {
                    neighbors.push(edge.target.clone());
                }
            }
        }

        for neighbors in adj.values_mut() {
            neighbors.sort();
        }

        adj
    }

    /// Computes in-degree and out-degree metrics for all nodes.
    pub fn compute_degrees(&mut self) {
        let mut in_degrees: HashMap<String, u32> =
            self.nodes.keys().map(|k| (k.clone(), 0)).collect();
        let mut out_degrees: HashMap<String, u32> =
            self.nodes.keys().map(|k| (k.clone(), 0)).collect();
        let mut broken_counts: HashMap<String, u32> =
            self.nodes.keys().map(|k| (k.clone(), 0)).collect();

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

            if node.status != crate::model::NodeStatus::Excluded {
                if node.in_degree + node.out_degree > 0 {
                    if node.hop_distance == 0 {
                        node.status = crate::model::NodeStatus::Active;
                    } else {
                        node.status = crate::model::NodeStatus::ConnectedInactive;
                    }
                } else {
                    node.status = crate::model::NodeStatus::Orphan;
                }
            }
        }
    }

    /// Computes Brandes betweenness centrality for all nodes.
    pub fn compute_betweenness(&mut self) {
        let mut node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        node_ids.sort();
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
        let mut nodes: Vec<&NoteNode> = self.nodes.values().collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        nodes
            .into_iter()
            .max_by(|a, b| {
                a.betweenness
                    .partial_cmp(&b.betweenness)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| (a.in_degree + a.out_degree).cmp(&(b.in_degree + b.out_degree)))
                    .then_with(|| {
                        let a_is_agents = a.id.eq_ignore_ascii_case("AGENTS.md") || a.id.ends_with("/AGENTS.md");
                        let b_is_agents = b.id.eq_ignore_ascii_case("AGENTS.md") || b.id.ends_with("/AGENTS.md");
                        if a_is_agents && !b_is_agents {
                            std::cmp::Ordering::Greater
                        } else if !a_is_agents && b_is_agents {
                            std::cmp::Ordering::Less
                        } else {
                            b.id.cmp(&a.id)
                        }
                    })
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

        let excluded_nodes: std::collections::HashSet<String> = self
            .nodes
            .iter()
            .filter(|(_, v)| v.status == crate::model::NodeStatus::Excluded)
            .map(|(k, _)| k.clone())
            .collect();

        let index = ResolverIndex::with_exclusions(&known_nodes, excluded_nodes);

        for edge in &mut self.edges {
            resolve_edge_integrity(edge, &index, &self.headings);
        }

        crate::fragment::extractor::populate_fragment_inbounds(&mut self.nodes, &self.edges);

        self.compute_degrees();
    }

    /// Computes ObservatorySummary adhering to the Node Lifecycle Invariant:
    /// total_notes = connected_notes + orphan_notes + excluded_notes.
    pub fn compute_observatory_summary(
        &self,
        last_sync: Option<u64>,
    ) -> crate::model::ObservatorySummary {
        let total_notes = self.nodes.len() as u32;
        let mut connected_notes = 0u32;
        let mut orphan_notes = 0u32;
        let mut excluded_notes = 0u32;
        let mut active_tokens = 0u64;
        let mut reachable_tokens = 0u64;

        for node in self.nodes.values() {
            match node.status {
                crate::model::NodeStatus::Excluded => {
                    excluded_notes += 1;
                }
                crate::model::NodeStatus::Orphan => {
                    orphan_notes += 1;
                }
                crate::model::NodeStatus::Active | crate::model::NodeStatus::ConnectedInactive => {
                    connected_notes += 1;
                }
            }

            if node.hop_distance == 0 {
                active_tokens += node.tokens as u64;
            }
            if node.hop_distance >= 0 && node.hop_distance <= 2 {
                reachable_tokens += node.tokens as u64;
            }
        }

        let broken_links_count = self.edges.iter().filter(|e| e.is_broken).count() as u32;

        let last_sync_timestamp = last_sync.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0)
        });

        crate::model::ObservatorySummary {
            total_notes,
            connected_notes,
            orphan_notes,
            excluded_notes,
            broken_links_count,
            active_tokens,
            reachable_tokens,
            sync_status: "synced".to_string(),
            last_sync_timestamp,
        }
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
                if let (Some(&src_idx), Some(&tgt_idx)) =
                    (index_map.get(&edge.source), index_map.get(&edge.target))
                {
                    graph.add_edge(src_idx, tgt_idx, ());
                }
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::NodeStatus;

    #[test]
    fn test_observatory_summary_node_lifecycle_invariant() {
        let mut topology = GraphTopology::new();

        let mut n1 = NoteNode::new("hub.md", "/vault/hub.md", "Hub");
        n1.status = NodeStatus::Active;
        n1.hop_distance = 0;
        n1.tokens = 1000;
        topology.add_node(n1);

        let mut n2 = NoteNode::new("doc1.md", "/vault/doc1.md", "Doc 1");
        n2.status = NodeStatus::ConnectedInactive;
        n2.hop_distance = 1;
        n2.tokens = 500;
        topology.add_node(n2);

        let mut n3 = NoteNode::new("orphan.md", "/vault/orphan.md", "Orphan");
        n3.status = NodeStatus::Orphan;
        n3.hop_distance = -1;
        n3.tokens = 200;
        topology.add_node(n3);

        let mut n4 = NoteNode::new(
            "packages/lib/README.md",
            "/vault/packages/lib/README.md",
            "Excluded Readme",
        );
        n4.status = NodeStatus::Excluded;
        n4.hop_distance = -1;
        n4.tokens = 300;
        topology.add_node(n4);

        let summary = topology.compute_observatory_summary(Some(123456789));

        assert_eq!(summary.total_notes, 4);
        assert_eq!(summary.connected_notes, 2);
        assert_eq!(
            summary.orphan_notes, 1,
            "Excluded note must NOT be counted as orphan"
        );
        assert_eq!(summary.excluded_notes, 1);
        assert_eq!(
            summary.total_notes,
            summary.connected_notes + summary.orphan_notes + summary.excluded_notes
        );
        assert_eq!(summary.active_tokens, 1000);
        assert_eq!(summary.reachable_tokens, 1500);
        assert_eq!(summary.sync_status, "synced");
        assert_eq!(summary.last_sync_timestamp, 123456789);
    }
}
