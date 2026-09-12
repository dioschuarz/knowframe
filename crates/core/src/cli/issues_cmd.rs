//! CLI command implementation for `llm-wiki-core issues`.

use std::path::Path;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::load_baseline;
use crate::graph::builder::GraphTopology;
use crate::quality::scorer::{calculate_quality_score, extract_all_issues};
use crate::quality::{KnowledgeIssue, KnowledgeIssuesReport};

/// Executes the `issues` CLI command.
pub fn execute_issues(
    workspace_root: &Path,
    pillar_filter: &str,
    limit: Option<usize>,
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

    let baseline = load_baseline(workspace_root).ok().flatten();
    let quality = calculate_quality_score(&topology, baseline.as_ref(), &[], &[]);
    let all_issues = extract_all_issues(&topology, &[], &[]);

    let integrity_count = all_issues.iter().filter(|i| i.pillar == "integrity").count();
    let consistency_count = all_issues.iter().filter(|i| i.pillar == "consistency").count();
    let discoverability_count = all_issues.iter().filter(|i| i.pillar == "discoverability").count();
    let freshness_count = all_issues.iter().filter(|i| i.pillar == "freshness").count();
    let total_issues = all_issues.len();

    let normalized_filter = pillar_filter.to_lowercase();
    let filtered_issues: Vec<KnowledgeIssue> = if normalized_filter == "all" || normalized_filter.is_empty() {
        all_issues
    } else {
        all_issues
            .into_iter()
            .filter(|i| i.pillar.to_lowercase() == normalized_filter)
            .collect()
    };

    let report_issues = if let Some(lim) = limit {
        filtered_issues.into_iter().take(lim).collect()
    } else {
        filtered_issues
    };

    let report = KnowledgeIssuesReport {
        overall_score: quality.overall_score,
        total_issues,
        integrity_count,
        consistency_count,
        discoverability_count,
        freshness_count,
        issues: report_issues,
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("──────────────────────────────────────────────────────────────");
    println!(
        "Knowledge Issues Report — {} Total Issues (Quality: {:.1}/100)",
        report.total_issues, report.overall_score
    );
    println!("──────────────────────────────────────────────────────────────");
    println!(
        "Pillar Counts: Integrity ({}) | Consistency ({}) | Discoverability ({}) | Freshness ({})",
        report.integrity_count,
        report.consistency_count,
        report.discoverability_count,
        report.freshness_count
    );
    if normalized_filter != "all" && !normalized_filter.is_empty() {
        println!("Filtered by pillar: {}", normalized_filter.to_uppercase());
    }
    println!();

    if report.issues.is_empty() {
        println!("No issues found for the selected criteria!");
        return Ok(());
    }

    let mut current_pillar = String::new();
    for (idx, issue) in report.issues.iter().enumerate() {
        if issue.pillar != current_pillar {
            current_pillar = issue.pillar.clone();
            println!();
            println!("▶ {} ISSUES:", current_pillar.to_uppercase());
        }

        let loc = if let Some(line) = issue.line_number {
            format!("{}:{}", issue.file_path, line)
        } else {
            issue.file_path.clone()
        };

        println!(
            "  {:3}. [{}] {} (-{:.1} pts, weight: {:.2}x)",
            idx + 1,
            issue.category,
            loc,
            issue.deduction_points,
            issue.weight
        );
        println!("       └─► {}", issue.description);
        println!("       └─► Remediation: {}", issue.remediation_hint);
    }

    println!();
    println!("Total shown: {} / {} issues", report.issues.len(), report.total_issues);

    Ok(())
}
