//! Standalone headless vault scanner producing full JSON knowledge topology.

use crate::ast::parser::parse_markdown_ast;
use crate::config::ContextVaultConfig;
use crate::graph::builder::GraphTopology;
use crate::ipc::protocol::{EdgePayload, NodePayload, ScanResultPayload, TokenSummaryPayload};
use crate::model::NoteNode;
use crate::tokens::counter::{compute_token_summary, count_bpe_tokens};
use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};
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
    let config_path = root.join(".context-vault.yaml");
    let config = if config_path.exists() {
        ContextVaultConfig::load_from_file(&config_path)
            .unwrap_or_else(|_| ContextVaultConfig::default())
    } else {
        ContextVaultConfig::default()
    };

    let include_set = build_globset(&config.include)?;
    let exclude_set = build_globset(&config.exclude)?;

    let mut topology = GraphTopology::new();

    // 1. Walk directory and collect markdown files
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let abs_path = entry.path();
        let rel_path = match abs_path.strip_prefix(root) {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Convert path to POSIX string
        let rel_str = rel_path.to_string_lossy().replace('\\', "/");

        if !include_set.is_match(&rel_str) || exclude_set.is_match(&rel_str) {
            continue;
        }

        // Read file content
        let content = match std::fs::read_to_string(abs_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Compute BPE tokens
        let tokens = count_bpe_tokens(&content).unwrap_or(0);

        // Parse AST
        let parsed = parse_markdown_ast(&rel_str, &content);

        let label = parsed.title.unwrap_or_else(|| {
            rel_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| rel_str.clone())
        });

        let mut node = NoteNode::new(&rel_str, abs_path.to_string_lossy().to_string(), label);
        node.tokens = tokens;
        node.tags = parsed.tags;

        topology.add_node(node);
        topology.set_headings(rel_str.clone(), parsed.headings);

        for edge in parsed.edges {
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
                    let pat = group.pattern().to_lowercase().trim_start_matches('#').to_string();
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
        } else if node.hop_distance == 0 {
            node.color = "#3B82F6".to_string(); // Active Hub Blue
        } else if node.hop_distance > 0 {
            node.color = "#64748B".to_string(); // Connected Slate
        } else {
            node.color = "#6B7280".to_string(); // Orphan Muted Gray
        }
    }

    // 7. Calculate Token Summary
    let node_vec: Vec<NoteNode> = topology.nodes.values().cloned().collect();
    let token_summary = compute_token_summary(
        &node_vec,
        config.token_budget.warning_threshold,
        config.token_budget.critical_threshold,
    );

    let node_payloads: Vec<NodePayload> = node_vec.iter().map(NodePayload::from).collect();
    let edge_payloads: Vec<EdgePayload> = topology.edges.iter().map(EdgePayload::from).collect();

    Ok(ScanResultPayload {
        nodes: node_payloads,
        edges: edge_payloads,
        token_summary: TokenSummaryPayload::from(&token_summary),
        recommended_hub: suggested,
    })
}
