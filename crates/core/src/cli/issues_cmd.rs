//! CLI command implementation for `llm-wiki-core issues`.

use std::collections::BTreeMap;
use std::path::Path;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::load_baseline;
use crate::graph::builder::GraphTopology;
use crate::quality::scorer::{calculate_quality_score, extract_all_issues_with_baseline};
use crate::quality::KnowledgeIssuesReport;

#[derive(Debug, Clone, Default)]
pub struct IssuesOptions {
    pub pillar: String,
    pub limit: Option<usize>,
    pub json: bool,
    pub include_debt: bool,
    pub summary: bool,
    pub filter_path: Option<String>,
    pub include_excluded: bool,
}

/// Executes the `issues` CLI command with default options.
pub fn execute_issues(
    workspace_root: &Path,
    pillar_filter: &str,
    limit: Option<usize>,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    execute_issues_with_options(
        workspace_root,
        IssuesOptions {
            pillar: pillar_filter.to_string(),
            limit,
            json: json_output,
            include_debt: false,
            summary: false,
            filter_path: None,
            include_excluded: false,
        },
    )
}

/// Executes the `issues` CLI command with extended options.
pub fn execute_issues_with_options(
    workspace_root: &Path,
    options: IssuesOptions,
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
        n.fragments = node.fragments;
        n.freshness = node.freshness;
        topology.add_node(n);
    }
    let node_headings: Vec<(String, Vec<String>)> = topology
        .nodes
        .values()
        .filter(|n| !n.fragments.is_empty())
        .map(|n| {
            (
                n.file_path.clone(),
                n.fragments.iter().map(|f| f.anchor.clone()).collect(),
            )
        })
        .collect();
    for (path, headings) in node_headings {
        topology.set_headings(path, headings);
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
    let collisions = crate::collision::detect_collisions_in_graph(&topology, workspace_root, 0.70);
    let drifts = crate::drift::detect_all_drifts_in_graph(&topology, workspace_root);
    let quality = calculate_quality_score(&topology, baseline.as_ref(), &collisions, &drifts);
    let all_issues =
        extract_all_issues_with_baseline(&topology, &collisions, &drifts, baseline.as_ref());

    let total_issues = all_issues.len();
    let legacy_debt_count = all_issues.iter().filter(|i| i.is_legacy).count();
    let active_issues_count = all_issues.iter().filter(|i| !i.is_legacy).count();

    // Filtering
    let mut filtered_issues = all_issues;

    // 1. Debt filtering: default is quiet (active regressions only)
    if !options.include_debt {
        filtered_issues.retain(|i| !i.is_legacy);
    }

    // 2. Excluded filtering: exclude issues from excluded files unless requested
    if !options.include_excluded {
        filtered_issues.retain(|i| {
            if let Some(node) = topology.nodes.get(&i.file_path) {
                node.status != crate::model::NodeStatus::Excluded
            } else {
                true
            }
        });
    }

    // 3. Path prefix filtering
    if let Some(ref prefix) = options.filter_path {
        let clean_prefix = prefix.trim_start_matches("./");
        filtered_issues.retain(|i| {
            i.file_path.starts_with(clean_prefix) || i.file_path.contains(clean_prefix)
        });
    }

    // 4. Pillar filtering
    let normalized_pillar = options.pillar.to_lowercase();
    if normalized_pillar != "all" && !normalized_pillar.is_empty() {
        filtered_issues.retain(|i| i.pillar.to_lowercase() == normalized_pillar);
    }

    let integrity_count = filtered_issues
        .iter()
        .filter(|i| i.pillar == "integrity")
        .count();
    let structure_count = filtered_issues
        .iter()
        .filter(|i| i.pillar == "structure")
        .count();
    let consistency_count = filtered_issues
        .iter()
        .filter(|i| i.pillar == "consistency")
        .count();
    let discoverability_count = filtered_issues
        .iter()
        .filter(|i| i.pillar == "discoverability")
        .count();
    let freshness_count = filtered_issues
        .iter()
        .filter(|i| i.pillar == "freshness")
        .count();

    let report_issues = if let Some(lim) = options.limit {
        filtered_issues.into_iter().take(lim).collect()
    } else {
        filtered_issues
    };

    let report = KnowledgeIssuesReport {
        overall_score: quality.overall_score,
        total_issues,
        integrity_count,
        structure_count,
        consistency_count,
        discoverability_count,
        freshness_count,
        issues: report_issues,
        active_issues_count,
        legacy_debt_count,
        is_quiet_default: !options.include_debt,
    };

    if options.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    // Terminal Summary Mode
    if options.summary {
        println!("─────────────────────────────────────────────────────────────────────────────");
        println!(
            "Knowledge Issues Summary — {} Issues Grouped by Directory",
            report.issues.len()
        );
        if !options.include_debt && legacy_debt_count > 0 {
            println!(
                "(Showing active regressions only; {} legacy items hidden. Use -d for all)",
                legacy_debt_count
            );
        }
        println!("─────────────────────────────────────────────────────────────────────────────");

        // Group by directory
        #[derive(Default)]
        struct DirStats {
            total: usize,
            integrity: usize,
            structure: usize,
            consistency: usize,
            discoverability: usize,
            freshness: usize,
        }

        let mut dir_map: BTreeMap<String, DirStats> = BTreeMap::new();
        for issue in &report.issues {
            let dir_name = Path::new(&issue.file_path)
                .parent()
                .and_then(|p| p.to_str())
                .filter(|s| !s.is_empty())
                .unwrap_or(".")
                .to_string();
            let entry = dir_map.entry(dir_name).or_default();
            entry.total += 1;
            match issue.pillar.as_str() {
                "integrity" => entry.integrity += 1,
                "structure" => entry.structure += 1,
                "consistency" => entry.consistency += 1,
                "discoverability" => entry.discoverability += 1,
                "freshness" => entry.freshness += 1,
                _ => {}
            }
        }

        if dir_map.is_empty() {
            println!("✓ No issues found for the selected scope!");
            return Ok(());
        }

        println!(
            "{:<36} | {:>5} | {:>5} | {:>6} | {:>5} | {:>5} | {:>5}",
            "Directory", "Total", "Integ", "Struct", "Cons", "Disc", "Fresh"
        );
        println!(
            "{:-<36}-+-{:-<5}-+-{:-<5}-+-{:-<6}-+-{:-<5}-+-{:-<5}-+-{:-<5}",
            "", "", "", "", "", "", ""
        );

        for (dir, stats) in &dir_map {
            let display_dir = if dir.len() > 36 {
                format!("…{}", &dir[dir.len() - 35..])
            } else {
                dir.clone()
            };
            println!(
                "{:<36} | {:>5} | {:>5} | {:>6} | {:>5} | {:>5} | {:>5}",
                display_dir,
                stats.total,
                stats.integrity,
                stats.structure,
                stats.consistency,
                stats.discoverability,
                stats.freshness
            );
        }

        println!("─────────────────────────────────────────────────────────────────────────────");
        println!("Total Issues: {}", report.issues.len());
        return Ok(());
    }

    // Default Detailed Terminal View
    println!("──────────────────────────────────────────────────────────────");
    if !options.include_debt {
        println!(
            "Active Knowledge Issues — {} Active (Quality: {:.1}/100)",
            report.active_issues_count, report.overall_score
        );
    } else {
        println!(
            "Knowledge Issues Report — {} Total Issues (Quality: {:.1}/100)",
            report.total_issues, report.overall_score
        );
    }
    println!("──────────────────────────────────────────────────────────────");

    if report.issues.is_empty() {
        if !options.include_debt && legacy_debt_count > 0 {
            println!("✓ 0 active regressions detected across the knowledge graph!");
            println!();
            println!(
                "ℹ {} legacy debt findings recorded in brownfield baseline (.context-vault/baseline.json).",
                legacy_debt_count
            );
            println!("  • Run 'llm-wiki-core issues --include-debt' (or -d) to inspect baselined backlog.");
            println!("  • Run 'llm-wiki-core issues --include-debt --summary' (or -s) for directory-level breakdown.");
            if let Some(ref p) = options.filter_path {
                println!("  • Current filter path: '{}'", p);
            }
        } else {
            println!("✓ No issues found matching the selected criteria!");
        }
        return Ok(());
    }

    println!(
        "Pillar Counts: Integrity ({}) | Structure ({}) | Consistency ({}) | Discoverability ({}) | Freshness ({})",
        report.integrity_count,
        report.structure_count,
        report.consistency_count,
        report.discoverability_count,
        report.freshness_count
    );
    if normalized_pillar != "all" && !normalized_pillar.is_empty() {
        println!("Filtered by pillar: {}", normalized_pillar.to_uppercase());
    }
    if let Some(ref p) = options.filter_path {
        println!("Filtered by path: {}", p);
    }
    println!();

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

        let legacy_badge = if issue.is_legacy { " [BASELINED]" } else { "" };

        println!(
            "  {:3}. [{}{}] {} (-{:.1} pts, weight: {:.2}x)",
            idx + 1,
            issue.category,
            legacy_badge,
            loc,
            issue.deduction_points,
            issue.weight
        );
        println!("       └─► {}", issue.description);
        println!("       └─► Remediation: {}", issue.remediation_hint);
    }

    println!();
    println!(
        "Total shown: {} / {} issues{}",
        report.issues.len(),
        if options.include_debt {
            report.total_issues
        } else {
            report.active_issues_count
        },
        if !options.include_debt && legacy_debt_count > 0 {
            format!(
                " ({} legacy debt items hidden; use -d to include)",
                legacy_debt_count
            )
        } else {
            String::new()
        }
    );

    Ok(())
}
