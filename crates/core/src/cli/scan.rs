//! Standalone headless vault scanner producing full JSON knowledge topology.

use crate::ast::parser::parse_markdown_ast;
use crate::config::ContextVaultConfig;
use crate::graph::builder::GraphTopology;
use crate::ipc::protocol::{EdgePayload, NodePayload, ScanResultPayload, TokenSummaryPayload};
use crate::model::NoteNode;
use crate::tokens::counter::{compute_token_summary, count_bpe_tokens};
use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct ScanOptions {
    pub workspace_root: PathBuf,
    pub hub: Option<String>,
    pub pretty: bool,
}

pub fn build_globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat).with_context(|| format!("Invalid glob pattern: {}", pat))?);
    }
    builder.build().context("Failed to compile glob set")
}

pub fn execute_scan(options: ScanOptions) -> Result<ScanResultPayload> {
    let root = &options.workspace_root;
    let config = ContextVaultConfig::load_from_workspace(root);

    let include_set = build_globset(&config.get_scan_include())?;
    let exclude_set = build_globset(&config.get_scan_exclude())?;
    let exclusion_engine = crate::graph::exclusions::ExclusionEngine::new(
        &config.get_scan_exclude(),
        &config.get_graph_exclude(),
    )?;

    let mut topology = GraphTopology::new();

    // 1. Walk directory and collect markdown files, pruning ignored directories
    let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            let name = e.file_name().to_string_lossy();
            if name == ".git"
                || name == "node_modules"
                || name == "target"
                || name == "dist"
                || name == "out"
                || name == "build"
                || name == ".venv"
                || name == "venv"
                || name == ".tox"
                || name == ".mypy_cache"
                || name == ".pytest_cache"
                || name == ".next"
                || name == ".nuxt"
                || name == ".cache"
                || name == ".turbo"
                || name == ".code-review-graph"
                || name == ".understand-anything"
                || name == "coverage"
            {
                return false;
            }
        }
        true
    });

    use rayon::prelude::*;
    use std::path::Path;

    let mut matching_files: Vec<(PathBuf, String)> = walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let abs_path = e.path().to_path_buf();
            let rel_path = abs_path.strip_prefix(root).ok()?;
            let rel_str = rel_path.to_string_lossy().replace('\\', "/");
            if !include_set.is_match(&rel_str) || exclude_set.is_match(&rel_str) {
                return None;
            }
            Some((abs_path, rel_str))
        })
        .collect();

    matching_files.sort_by(|a, b| a.1.cmp(&b.1));

    let mut parsed_items: Vec<_> = matching_files
        .into_par_iter()
        .filter_map(|(abs_path, rel_str)| {
            let content = std::fs::read_to_string(&abs_path).ok()?;
            let tokens = count_bpe_tokens(&content).unwrap_or(0);
            let parsed = parse_markdown_ast(&rel_str, &content);

            let label = parsed.title.unwrap_or_else(|| {
                Path::new(&rel_str)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| rel_str.clone())
            });

            let fragments = crate::fragment::extractor::extract_fragments_from_document(
                &content,
                &parsed.edges,
            );

            let mut node = NoteNode::new(&rel_str, abs_path.to_string_lossy().to_string(), label);
            node.tokens = tokens;
            node.tags = parsed.tags;
            node.fragments = fragments;
            node.freshness = None;
            if exclusion_engine.is_graph_excluded(&rel_str) {
                node.status = crate::model::NodeStatus::Excluded;
            }

            Some((node, rel_str, parsed.headings, parsed.edges))
        })
        .collect();

    parsed_items.sort_by(|a, b| a.1.cmp(&b.1));

    for (node, rel_str, headings, mut edges) in parsed_items {
        topology.add_node(node);
        topology.set_headings(rel_str, headings);
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        for edge in edges {
            topology.add_edge(edge);
        }
    }

    // 2. Resolve link destinations and anchor integrity
    topology.resolve_links();

    // 3. Compute Brandes betweenness centrality
    topology.compute_betweenness();

    // 4. Determine Hub
    let suggested = topology.suggest_hub();
    let designated_hub = options
        .hub
        .or_else(|| config.vault.hubs.first().cloned())
        .or_else(|| config.vault.auto_discovered_hub.clone())
        .or_else(|| suggested.clone());

    // 5. Compute BFS hops from designated Hub
    if let Some(ref hub_id) = designated_hub {
        topology.compute_hops(hub_id);
    }

    // 6. Apply cluster color groups
    for node in topology.nodes.values_mut() {
        let mut matched_color = None;
        for group in &config.color_groups {
            if !group.enabled {
                continue;
            }
            match group.query_type() {
                crate::model::ClusterQueryType::Tag => {
                    let pat = group
                        .pattern()
                        .to_lowercase()
                        .trim_start_matches('#')
                        .to_string();
                    if node.tags.iter().any(|t| t.to_lowercase() == pat) {
                        matched_color = Some(group.color.clone());
                        break;
                    }
                }
                crate::model::ClusterQueryType::Path => {
                    let pat = group.pattern();
                    if let Ok(glob) = Glob::new(pat) {
                        if glob.compile_matcher().is_match(&node.id) {
                            matched_color = Some(group.color.clone());
                            break;
                        }
                    }
                }
            }
        }

        if let Some(color) = matched_color {
            node.color = color;
        } else if node.status == crate::model::NodeStatus::Excluded {
            node.color = "#475569".to_string(); // Excluded Slate/Ghost
        } else if node.hop_distance >= 0 || node.in_degree + node.out_degree > 0 {
            node.color = "#64748B".to_string(); // Default Connected Slate Grey (Hub has active blue ring in renderer)
        } else {
            node.color = "#6B7280".to_string(); // Orphan Muted Gray
        }
    }

    // 7. Calculate Token Summary
    let mut node_vec: Vec<NoteNode> = topology.nodes.values().cloned().collect();
    node_vec.sort_by(|a, b| a.id.cmp(&b.id));
    let token_summary = compute_token_summary(
        &node_vec,
        config.token_budget.warning_threshold,
        config.token_budget.critical_threshold,
    );

    let node_payloads: Vec<NodePayload> = node_vec.iter().map(NodePayload::from).collect();
    let edge_payloads: Vec<EdgePayload> = topology.edges.iter().map(EdgePayload::from).collect();
    let observatory_summary = topology.compute_observatory_summary(None);

    Ok(ScanResultPayload {
        nodes: node_payloads,
        edges: edge_payloads,
        token_summary: TokenSummaryPayload::from(&token_summary),
        recommended_hub: suggested,
        observatory_summary: Some(observatory_summary),
    })
}
