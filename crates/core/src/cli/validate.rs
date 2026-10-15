//! CLI command implementation for `llm-wiki-core validate` (pre-commit hook).
//! Enforces Constitution Principle II: strict non-blocking fail-open ceiling (500ms exit 0).

use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;

use crate::candidate::evaluator::{evaluate_candidate, AdmissionConfig};
use crate::candidate::overlay::CandidateOverlay;
use crate::candidate::AdmissionDecision;
use crate::debt::baseline::{capture_initial_baseline, load_baseline, save_baseline};
use crate::delta::updater::{FileDeltaItem, FileDeltaStatus};
use crate::delta::GitDelta;
use crate::graph::builder::GraphTopology;

/// Extracts staged changes from git index using `git diff --cached --name-status`.
pub fn get_staged_git_delta(workspace_root: &Path) -> Result<GitDelta, std::io::Error> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-status"])
        .current_dir(workspace_root)
        .output()?;

    if !output.status.success() {
        return Ok(GitDelta::default());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut items = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let status_code = parts[0];
        let status = if status_code.starts_with('A') {
            FileDeltaStatus::Added
        } else if status_code.starts_with('D') {
            FileDeltaStatus::Deleted
        } else if status_code.starts_with('M') {
            FileDeltaStatus::Modified
        } else if status_code.starts_with('R') {
            // Renamed: parts[1] is old path (deleted), parts[2] is new path (added)
            let old_path = parts[1].to_string();
            let new_path = if parts.len() >= 3 {
                parts[2].to_string()
            } else {
                old_path.clone()
            };
            items.push(FileDeltaItem {
                path: old_path,
                status: FileDeltaStatus::Deleted,
            });
            items.push(FileDeltaItem {
                path: new_path,
                status: FileDeltaStatus::Added,
            });
            continue;
        } else {
            FileDeltaStatus::Modified
        };

        items.push(FileDeltaItem {
            path: parts[1].to_string(),
            status,
        });
    }

    Ok(GitDelta {
        commit_sha: None,
        items,
    })
}

/// Executes pre-commit admission validation.
/// Returns exit code (0 for PASS/WARN/Fail-Open, 1 for BLOCK).
pub async fn execute_validate(
    workspace_root: &Path,
    staged_only: bool,
    quiet: bool,
    json_output: bool,
    timeout_ms: u64,
) -> i32 {
    execute_validate_with_paths(workspace_root, None, staged_only, quiet, json_output, timeout_ms).await
}

/// Executes admission validation with optional explicit file paths.
pub async fn execute_validate_with_paths(
    workspace_root: &Path,
    paths: Option<Vec<String>>,
    _staged_only: bool,
    quiet: bool,
    json_output: bool,
    timeout_ms: u64,
) -> i32 {
    let validate_task = async {
        let staged_delta = if let Some(ref p_list) = paths {
            let mut items = Vec::new();
            for p in p_list {
                let clean = p.replace('\\', "/");
                if crate::drift::extractor::validate_path_containment(&clean).is_ok() {
                    let status = if workspace_root.join(&clean).exists() {
                        FileDeltaStatus::Modified
                    } else {
                        FileDeltaStatus::Deleted
                    };
                    items.push(FileDeltaItem { path: clean, status });
                }
            }
            GitDelta { commit_sha: None, items }
        } else {
            get_staged_git_delta(workspace_root).unwrap_or_default()
        };

        // If no files are staged/provided, commit proceeds cleanly
        if staged_delta.items.is_empty() {
            if !quiet && !json_output {
                println!("[llm-wiki-validate] No staged changes found. Commit proceeds.");
            }
            return 0;
        }

        // Materialize frontmatter for staged markdown documents and re-stage into Git index
        let mut structure_warnings = Vec::new();
        for item in &staged_delta.items {
            if item.path.ends_with(".md") && item.status != FileDeltaStatus::Deleted {
                let file_path = workspace_root.join(&item.path);
                if file_path.is_file() {
                    if let Ok(content) = std::fs::read_to_string(&file_path) {
                        let parsed = crate::ast::parser::parse_markdown_ast(&item.path, &content);
                        let fragments = crate::fragment::extractor::extract_fragments_from_document(
                            &content,
                            &parsed.edges,
                        );
                        let token_count =
                            crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);

                        // Structure debt check: oversized document (>50k tokens or >10 headings) lacking frontmatter
                        let has_frontmatter_metadata =
                            content.contains("wiki:") && content.contains("fragments:");
                        if (token_count > 50_000 || parsed.headings.len() > 10)
                            && !has_frontmatter_metadata
                        {
                            structure_warnings.push(format!(
                                "Document '{}' exceeds structural threshold ({} tokens, {} headings) without materialized fragment metadata. Run 'llm-wiki-core materialize {}' to resolve structural debt.",
                                item.path, token_count, parsed.headings.len(), item.path
                            ));
                        }

                        // Non-destructively inject materialized frontmatter and re-stage into git index
                        if let Ok(updated) =
                            crate::fragment::frontmatter::inject_materialized_frontmatter(
                                &content, &fragments,
                            )
                        {
                            if updated != content && std::fs::write(&file_path, &updated).is_ok() {
                                let _ = std::process::Command::new("git")
                                    .arg("add")
                                    .arg(&item.path)
                                    .current_dir(workspace_root)
                                    .output();
                            }
                        }
                    }
                }
            }
        }

        // Scan base committed graph topology
        let base_graph = match crate::cli::scan::execute_scan(crate::cli::scan::ScanOptions {
            workspace_root: workspace_root.to_path_buf(),
            hub: None,
            pretty: false,
        }) {
            Ok(scan_result) => {
                let mut topology = GraphTopology::new();
                for node in scan_result.nodes {
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
                for edge in scan_result.edges {
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
                topology
            }
            Err(_) => GraphTopology::new(),
        };

        // Load or initialize brownfield debt baseline
        let baseline = match load_baseline(workspace_root) {
            Ok(Some(base)) => base,
            _ => {
                let initial = capture_initial_baseline(&base_graph, "initial", vec![], vec![]);
                let _ = save_baseline(workspace_root, &initial);
                initial
            }
        };

        let overlay = CandidateOverlay::new(&base_graph, workspace_root, &staged_delta);

        let vault_config = crate::config::ContextVaultConfig::load_from_workspace(workspace_root);
        let (enforce_doc_review, block_on_legacy_debt) = if let Some(ref pol) = vault_config.review_policy {
            (pol.enforce_doc_review, pol.block_on_legacy_debt)
        } else {
            (true, false)
        };

        let admission_config = AdmissionConfig {
            block_on_broken_links: true,
            block_on_duplicate_collision: true,
            block_on_technical_drift: false,
            block_on_unreviewed_docs: enforce_doc_review,
            block_on_legacy_debt,
        };

        let mut report = evaluate_candidate(
            &overlay,
            &baseline,
            vec![],
            vec![],
            admission_config,
        );

        for w in structure_warnings {
            report.warnings.push(crate::candidate::WarningItem {
                message: w,
                file_path: None,
            });
        }
        if report.decision == AdmissionDecision::Pass && !report.warnings.is_empty() {
            report.decision = AdmissionDecision::Warn;
        }

        if json_output {
            if let Ok(serialized) = serde_json::to_string_pretty(&report) {
                println!("{}", serialized);
            }
        } else if !quiet {
            match report.decision {
                AdmissionDecision::Block => {
                    eprintln!("══════════════════════════════════════════════════════════════");
                    eprintln!(
                        "LLM Wiki: Commit Blocked ({} knowledge regression(s) detected)",
                        report.regressions.len()
                    );
                    eprintln!("══════════════════════════════════════════════════════════════\n");
                    for reg in &report.regressions {
                        let line_str = reg
                            .line_number
                            .map(|l| format!(":{}", l))
                            .unwrap_or_default();
                        eprintln!("✕ [{:?}] {}{}", reg.category, reg.file_path, line_str);
                        eprintln!("  Message: {}", reg.message);
                        eprintln!("  Remediation: {}\n", reg.remediation_hint);
                    }
                    eprintln!("Commit aborted. Fix issues or use --no-verify if urgent.");
                }
                AdmissionDecision::Warn => {
                    println!("[llm-wiki-validate] Evaluating staged candidate graph...");
                    println!(
                        "⚠ WARN: 0 blocking regressions, {} warning(s). Commit proceeds.",
                        report.warnings.len()
                    );
                    for w in &report.warnings {
                        println!("  - {}", w.message);
                    }
                }
                AdmissionDecision::Pass => {
                    println!("[llm-wiki-validate] Evaluating staged candidate graph...");
                    if report.resolved_findings.is_empty() {
                        println!("✔ PASS: 0 new regressions. Commit proceeds.");
                    } else {
                        println!(
                            "✔ PASS: 0 new regressions. ({} legacy debt item(s) resolved!)",
                            report.resolved_findings.len()
                        );
                    }
                }
            }
        }

        report.exit_code
    };

    // Strict non-blocking fail-open ceiling: exit code 0 if anything fails or times out
    match timeout(Duration::from_millis(timeout_ms), validate_task).await {
        Ok(code) => code,
        Err(_) => {
            if !quiet && !json_output {
                eprintln!(
                    "[llm-wiki-validate] Warning: Validation timed out after {}ms. Non-blocking fail-open: commit proceeds.",
                    timeout_ms
                );
            }
            0
        }
    }
}
