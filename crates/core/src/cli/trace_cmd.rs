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
        topology.set_headings(
            node.id.clone(),
            node.fragments.iter().map(|f| f.heading.clone()).collect(),
        );
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
            fragments: node.fragments,
            freshness: None,
        });
    }
    for edge in scan_res.edges {
        let target_fragment = edge.anchor.as_ref().map(|a| a.trim_start_matches('#').to_string());
        topology.add_edge(crate::model::NoteEdge {
            id: edge.id,
            source: edge.source,
            target: edge.target,
            target_fragment,
            link_type: edge.link_type,
            is_broken: edge.is_broken,
            raw_text: String::new(),
            anchor: edge.anchor,
            line: edge.line,
            resolution_status: edge.resolution_status,
        });
    }

    let section_result = crate::trace::beam::trace_context_section(&topology, query, entrypoint);
    let result = execute_context_trace(&topology, query, entrypoint);

    if json_output {
        if let Some(ref sec) = section_result {
            println!("{}", serde_json::to_string_pretty(sec)?);
        } else {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        return Ok(());
    }

    println!("──────────────────────────────────────────────────────────────");
    println!("Context Trace Result (Section-Level Constrained Beam Search)");
    println!("──────────────────────────────────────────────────────────────");
    println!("Query:       \"{}\"", query);

    if let Some(ref sec) = section_result {
        println!("Entry Point: {}", sec.entrypoint);
        let target_display = if let Some(ref frag) = sec.target_fragment {
            format!("{}#{}", sec.target_document, frag)
        } else {
            sec.target_document.clone()
        };
        println!("Target Note: {}", target_display);
        println!();

        if sec.path.is_empty() {
            println!("No valid discovery path found from entrypoint to target.");
        } else {
            println!("Best Discovery Path:");
            for (idx, step) in sec.path.iter().enumerate() {
                if idx == 0 {
                    println!("  {} (Entrypoint)", step);
                } else if idx == sec.path.len() - 1 && sec.target_fragment.is_some() {
                    let indent = "      ".repeat(idx - 1);
                    println!("  {}└─► {}#{} (Hop {})", indent, step, sec.target_fragment.as_ref().unwrap(), idx);
                } else {
                    let indent = "      ".repeat(idx - 1);
                    println!("  {}└─► {} (Hop {})", indent, step, idx);
                }
            }
            println!();
            println!("Traversal Depth:      {} hop(s)", sec.depth);
            println!("Document Raw Tokens:  {} tokens (cl100k_base)", sec.document_raw_tokens);
            if let Some(frag_toks) = sec.fragment_raw_tokens {
                println!("Fragment Raw Tokens:  {} tokens (cl100k_base)", frag_toks);
                println!("Context Reduction:    {:.1}%", sec.context_reduction_percent);
            } else {
                println!("Context Reduction:    0.0% (document node targeted)");
            }
            if !sec.relevance_reasons.is_empty() {
                println!("Relevance Reasons:");
                for r in &sec.relevance_reasons {
                    println!("  └─► {}", r);
                }
            }
        }
    } else {
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
    }

    Ok(())
}
