//! CLI command implementation for `llm-wiki-core quality`.

use std::path::Path;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::load_baseline;
use crate::graph::builder::GraphTopology;
use crate::quality::scorer::calculate_quality_score;

/// Executes the `quality` CLI command.
pub fn execute_quality(
    workspace_root: &Path,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scan_res = execute_scan(ScanOptions {
        workspace_root: workspace_root.to_path_buf(),
        hub: None,
        pretty: false,
    })?;

    let mut topology = GraphTopology::new();
    for node in scan_res.nodes {
        let mut n = crate::model::NoteNode::new(node.id.clone(), node.file_path, node.label);
        n.tags = node.tags;
        n.tokens = node.tokens;
        n.in_degree = node.in_degree;
        n.out_degree = node.out_degree;
        n.betweenness = node.betweenness;
        n.hop_distance = node.hop_distance;
        n.status = node.status;
        n.color = node.color;
        n.broken_links = node.broken_links;
        topology.add_node(n);
    }
    for edge in scan_res.edges {
        let mut e = crate::model::NoteEdge::new(
            edge.source,
            edge.target,
            edge.link_type,
            String::new(),
            edge.anchor,
            edge.line,
        );
        e.is_broken = edge.is_broken;
        e.resolution_status = edge.resolution_status;
        topology.add_edge(e);
    }

    // Compute betweenness and BFS hops if not already populated
    topology.compute_degrees();
    topology.compute_betweenness();
    if let Some(hub) = topology.suggest_hub() {
        topology.compute_hops(&hub);
    }

    let baseline = load_baseline(workspace_root).ok().flatten();
    let quality = calculate_quality_score(&topology, baseline.as_ref(), &[], &[]);

    if json_output {
        println!("{}", serde_json::to_string_pretty(&quality)?);
        return Ok(());
    }

    println!("──────────────────────────────────────────────────────────────");
    println!("Knowledge Quality Score");
    println!("──────────────────────────────────────────────────────────────");
    println!("Overall Score:        {:.1} / 100", quality.overall_score);
    println!();
    println!("Pillar Breakdown:");
    println!("  Integrity (30%):    {:.1} / 100", quality.integrity_score);
    println!("  Consistency (25%):  {:.1} / 100", quality.consistency_score);
    println!("  Discoverability (25%): {:.1} / 100", quality.discoverability_score);
    println!("  Freshness (20%):    {:.1} / 100", quality.freshness_score);
    println!();
    println!("Debt Accounting:");
    println!("  Legacy Debt:        {} items", quality.legacy_debt_count);
    println!("  New Regressions:    {} items", quality.new_regressions_count);
    println!("  Resolved Debt:      {} items", quality.resolved_debt_count);
    if let Some(hub) = &quality.central_hub_id {
        println!("  Central Hub:        {}", hub);
    }
    println!();

    if quality.top_degradation_factors.is_empty() {
        println!("No degradation factors detected! Knowledge graph is in pristine condition.");
    } else {
        println!("Top Degradation Factors (ranked by topological impact):");
        for (i, factor) in quality.top_degradation_factors.iter().enumerate() {
            println!(
                "  {}. [{}] {} (-{:.1} pts, weight: {:.2}x)",
                i + 1,
                factor.category,
                factor.affected_node,
                factor.deduction_points,
                factor.betweenness_weight
            );
            println!("     └─► {}", factor.description);
        }
    }

    Ok(())
}
