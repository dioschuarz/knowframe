//! CLI command implementation for `llm-wiki-core trace <query>`.

use std::path::Path;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::graph::builder::GraphTopology;
use crate::trace::traversal::execute_context_trace;

/// Executes context trace CLI command.
pub fn execute_trace(
    workspace_root: &Path,
    query: &str,
    entrypoint: Option<&str>,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scan_res = execute_scan(ScanOptions {
        workspace_root: workspace_root.to_path_buf(),
        hub: None,
        pretty: false,
    })?;

    let mut topology = GraphTopology::new();
    for node in scan_res.nodes {
        topology.add_node(crate::model::NoteNode {
            id: node.id.clone(),
            file_path: node.file_path,
            label: node.label,
            tags: node.tags,
            tokens: node.tokens,
            in_degree: node.in_degree,
            out_degree: node.out_degree,
            betweenness: node.betweenness,
            hop_distance: node.hop_distance,
            status: node.status,
            color: node.color,
            broken_links: node.broken_links,
        });
    }
    for edge in scan_res.edges {
        topology.add_edge(crate::model::NoteEdge {
            id: edge.id,
            source: edge.source,
            target: edge.target,
            link_type: edge.link_type,
            is_broken: edge.is_broken,
            raw_text: String::new(),
            anchor: edge.anchor,
            line: edge.line,
            resolution_status: edge.resolution_status,
        });
    }

    let result = execute_context_trace(&topology, query, entrypoint);

    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    println!("──────────────────────────────────────────────────────────────");
    println!("Context Trace Result");
    println!("──────────────────────────────────────────────────────────────");
    println!("Query:       \"{}\"", result.query);
    println!("Entry Point: {}", result.entrypoint);
    println!(
        "Target Note: {}",
        if result.target_node.is_empty() {
            "(no match found)"
        } else {
            &result.target_node
        }
    );
    println!();

    if result.path.is_empty() {
        println!("No valid discovery path found from entrypoint to target.");
    } else {
        println!("Best Discovery Path:");
        for (idx, step) in result.path.iter().enumerate() {
            if idx == 0 {
                println!("  {} (Entrypoint)", step);
            } else {
                let indent = "      ".repeat(idx - 1);
                println!("  {}└─► {} (Hop {})", indent, step, idx);
            }
        }
        println!();
        println!("Traversal Depth:      {} hop(s)", result.depth);
        println!(
            "Cumulative BPE Load:  {} tokens (cl100k_base)",
            result.cumulative_bpe_tokens
        );
        println!("Target Note Tokens:   {} tokens", result.relevant_tokens);
        println!("Path Quality Score:   {:.1}%", result.path_quality_score);
    }

    Ok(())
}
