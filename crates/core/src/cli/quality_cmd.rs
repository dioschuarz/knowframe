//! CLI command implementation for `llm-wiki-core quality`.

use std::path::Path;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::load_baseline;
use crate::graph::builder::GraphTopology;
use crate::quality::scorer::calculate_quality_score;

/// Executes the `quality` CLI command with default options.
pub fn execute_quality(
    workspace_root: &Path,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    execute_quality_with_options(workspace_root, false, json_output)
}

/// Executes the `quality` CLI command with extended options.
pub fn execute_quality_with_options(
    workspace_root: &Path,
    raw_mode: bool,
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
        n.fragments = node.fragments.clone();
        topology.set_headings(
            node.id.clone(),
            node.fragments.iter().map(|f| f.heading.clone()).collect(),
        );
        topology.add_node(n);
    }
    for edge in scan_res.edges {
        topology.add_edge(edge.to_note_edge());
    }

    let baseline = load_baseline(workspace_root).ok().flatten();
    let collisions = crate::collision::detect_collisions_in_graph(&topology, workspace_root, 0.70);
    let drifts = crate::drift::detect_all_drifts_in_graph(&topology, workspace_root);

    // When raw mode is requested, evaluate without debt baseline masking
    let quality = if raw_mode {
        calculate_quality_score(&topology, None, &collisions, &drifts)
    } else {
        calculate_quality_score(&topology, baseline.as_ref(), &collisions, &drifts)
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&quality)?);
        return Ok(());
    }

    println!("──────────────────────────────────────────────────────────────");
    if raw_mode {
        println!("Knowledge Quality Score — Raw Historical Audit Mode");
    } else {
        println!("Knowledge Quality Score — Effective Score");
    }
    println!("──────────────────────────────────────────────────────────────");

    if raw_mode {
        println!("Overall Score (Raw):  {:.1} / 100", quality.overall_score);
    } else {
        println!(
            "Overall Score:        {:.1} / 100 (Effective)",
            quality.overall_score
        );
        if let Some(raw) = quality.raw_score {
            if (raw - quality.overall_score).abs() > 0.01 {
                println!(
                    "  └─► Raw Audit:      {:.1} / 100 (Historical debt baselined; use --raw to audit)",
                    raw
                );
            }
        }
    }
    println!();
    println!("Pillar Breakdown:");
    println!("  Integrity (25%):    {:.1} / 100", quality.integrity_score);
    println!("  Structure (20%):    {:.1} / 100", quality.structure_score);
    println!(
        "  Consistency (20%):  {:.1} / 100",
        quality.consistency_score
    );
    println!(
        "  Discoverability (20%): {:.1} / 100",
        quality.discoverability_score
    );
    println!("  Freshness (15%):    {:.1} / 100", quality.freshness_score);
    println!();
    println!("Debt Accounting:");
    println!("  Legacy Debt:        {} items", quality.legacy_debt_count);
    println!(
        "  New Regressions:    {} items",
        quality.new_regressions_count
    );
    println!(
        "  Resolved Debt:      {} items",
        quality.resolved_debt_count
    );
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
