//! CLI subcommand implementation for `llm-wiki-core baseline init` and `baseline reset`.

use std::path::Path;
use std::process::Command;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::{capture_initial_baseline, load_baseline, save_baseline};
use crate::graph::builder::GraphTopology;

/// Retrieves current Git commit SHA, or None if git is unavailable.
pub fn get_current_git_commit(workspace_root: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(workspace_root)
        .output()
        .ok()?;

    if output.status.success() {
        let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !sha.is_empty() {
            return Some(sha);
        }
    }
    None
}

/// Executes `baseline init`.
pub fn execute_baseline_init(
    workspace_root: &Path,
    force: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !force {
        if let Ok(Some(existing)) = load_baseline(workspace_root) {
            println!(
                "Knowledge Debt Baseline already established for commit {}",
                existing.baseline_commit
            );
            println!(
                "  Legacy broken references: {}",
                existing.broken_links.len()
            );
            println!(
                "  Legacy orphan nodes:      {}",
                existing.orphan_nodes.len()
            );
            println!("  Legacy collisions:        {}", existing.collisions.len());
            println!(
                "  Legacy technical drifts:  {}",
                existing.technical_drifts.len()
            );
            println!("  Total Legacy Debt:        {}", existing.total_debt);
            println!(
                "Pre-commit will not block on these {} items.",
                existing.total_debt
            );
            println!("(Use --force or 'baseline reset' to re-capture)");
            return Ok(());
        }
    }

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
            code_dependencies: Vec::new(),
        });
    }
    for edge in scan_res.edges {
        let target_fragment = edge
            .anchor
            .as_ref()
            .map(|a| a.trim_start_matches('#').to_string());
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

    use crate::debt::{CollisionSignature, DriftSignature};
    let collisions = crate::collision::detect_collisions_in_graph(&topology, workspace_root, 0.70);
    let col_sigs: Vec<CollisionSignature> = collisions
        .iter()
        .map(|c| CollisionSignature::new(&c.source_id, &c.target_id))
        .collect();
    let drifts = crate::drift::detect_all_drifts_in_graph(&topology, workspace_root);
    let drift_sigs: Vec<DriftSignature> = drifts
        .iter()
        .map(|d| DriftSignature {
            doc_path: d.doc_id.clone(),
            referenced_code_path: d.code_target.clone(),
            symbol: d.symbol_name.clone().unwrap_or_default(),
        })
        .collect();

    let commit_sha =
        get_current_git_commit(workspace_root).unwrap_or_else(|| "initial".to_string());
    let baseline = capture_initial_baseline(&topology, &commit_sha, col_sigs, drift_sigs);
    save_baseline(workspace_root, &baseline)?;

    println!(
        "Knowledge Debt Baseline established for commit {}",
        commit_sha
    );
    println!(
        "  Legacy broken references: {}",
        baseline.broken_links.len()
    );
    println!(
        "  Legacy orphan nodes:      {}",
        baseline.orphan_nodes.len()
    );
    println!("  Legacy collisions:        {}", baseline.collisions.len());
    println!(
        "  Legacy technical drifts:  {}",
        baseline.technical_drifts.len()
    );
    println!("  Total Legacy Debt:        {}", baseline.total_debt);
    println!(
        "Pre-commit will not block on these {} items.",
        baseline.total_debt
    );

    Ok(())
}

/// Executes `baseline reset`.
pub fn execute_baseline_reset(
    workspace_root: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    execute_baseline_init(workspace_root, true)
}
