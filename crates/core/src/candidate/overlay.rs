//! Ephemeral candidate graph overlay construction and routing.
//! Combines immutable committed base graph with staged Git delta without deep cloning.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::ast::parser::parse_markdown_ast;
use crate::delta::updater::FileDeltaStatus;
use crate::delta::GitDelta;
use crate::graph::builder::GraphTopology;
use crate::model::{NodeStatus, NoteEdge, NoteNode};
use crate::tokens::counter::count_bpe_tokens;

/// In-memory overlay containing staged file mutations.
#[derive(Debug, Clone, Default)]
pub struct OverlayDelta {
    /// Mutated or newly added nodes (path -> NoteNode)
    pub mutated_nodes: HashMap<String, NoteNode>,
    /// Paths of deleted nodes
    pub deleted_nodes: HashSet<String>,
    /// Outgoing edges originating from mutated nodes
    pub mutated_edges: Vec<NoteEdge>,
    /// Headings for mutated nodes (path -> headings)
    pub mutated_headings: HashMap<String, Vec<String>>,
}

/// Ephemeral candidate graph view referencing base committed graph.
pub struct CandidateOverlay<'a> {
    pub base_graph: &'a GraphTopology,
    pub staged_delta: &'a GitDelta,
    pub overlay: OverlayDelta,
}

impl<'a> CandidateOverlay<'a> {
    /// Builds a candidate overlay from base graph and staged Git delta.
    pub fn new(
        base_graph: &'a GraphTopology,
        workspace_root: &Path,
        staged_delta: &'a GitDelta,
    ) -> Self {
        let mut overlay = OverlayDelta::default();

        for item in &staged_delta.items {
            let rel_path = item.path.replace('\\', "/");

            match item.status {
                FileDeltaStatus::Deleted => {
                    overlay.deleted_nodes.insert(rel_path);
                }
                FileDeltaStatus::Added | FileDeltaStatus::Modified => {
                    let abs_path = workspace_root.join(&rel_path);
                    if !abs_path.is_file() {
                        continue;
                    }

                    let content = match std::fs::read_to_string(&abs_path) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };

                    let ast = parse_markdown_ast(&rel_path, &content);
                    let token_count = count_bpe_tokens(&content).unwrap_or(0);

                    let label = ast
                        .headings
                        .first()
                        .cloned()
                        .unwrap_or_else(|| {
                            Path::new(&rel_path)
                                .file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_else(|| rel_path.clone())
                        });

                    let node = NoteNode {
                        id: rel_path.clone(),
                        file_path: abs_path.to_string_lossy().to_string(),
                        label,
                        tags: ast.tags,
                        tokens: token_count,
                        in_degree: 0,
                        out_degree: ast.edges.len() as u32,
                        betweenness: 0.0,
                        hop_distance: -1,
                        status: NodeStatus::Active,
                        color: "#64748B".to_string(),
                        broken_links: 0,
                        fragments: Vec::new(),
                        freshness: None,
                    };

                    overlay.mutated_headings.insert(rel_path.clone(), ast.headings);
                    overlay.mutated_edges.extend(ast.edges);
                    overlay.mutated_nodes.insert(rel_path, node);
                }
            }
        }

        Self {
            base_graph,
            staged_delta,
            overlay,
        }
    }

    /// Retrieves a node from overlay if present, falling back to base graph if not deleted.
    pub fn get_node(&self, id: &str) -> Option<&NoteNode> {
        if self.overlay.deleted_nodes.contains(id) {
            return None;
        }
        if let Some(node) = self.overlay.mutated_nodes.get(id) {
            return Some(node);
        }
        self.base_graph.get_node(id)
    }

    /// Checks if a node exists in candidate state.
    pub fn node_exists(&self, id: &str) -> bool {
        if self.overlay.deleted_nodes.contains(id) {
            return false;
        }
        if self.overlay.mutated_nodes.contains_key(id) {
            return true;
        }
        self.base_graph.get_node(id).is_some()
    }

    /// Retrieves headings for a node in candidate state.
    pub fn get_headings(&self, id: &str) -> Option<&[String]> {
        if self.overlay.deleted_nodes.contains(id) {
            return None;
        }
        if let Some(headings) = self.overlay.mutated_headings.get(id) {
            return Some(headings.as_slice());
        }
        self.base_graph.headings.get(id).map(|v| v.as_slice())
    }

    /// Collects all candidate node paths for building resolver indexes.
    pub fn collect_known_nodes(&self) -> HashMap<String, PathBuf> {
        let mut nodes = HashMap::new();

        for (k, node) in &self.base_graph.nodes {
            if !self.overlay.deleted_nodes.contains(k) && !self.overlay.mutated_nodes.contains_key(k) {
                nodes.insert(k.clone(), PathBuf::from(&node.file_path));
            }
        }

        for (k, node) in &self.overlay.mutated_nodes {
            nodes.insert(k.clone(), PathBuf::from(&node.file_path));
        }

        nodes
    }

    /// Collects candidate headings map for heading anchor resolution.
    pub fn collect_headings_map(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        for (k, v) in &self.base_graph.headings {
            if !self.overlay.deleted_nodes.contains(k) && !self.overlay.mutated_headings.contains_key(k) {
                map.insert(k.clone(), v.clone());
            }
        }

        for (k, v) in &self.overlay.mutated_headings {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
