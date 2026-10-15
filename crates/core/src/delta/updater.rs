//! In-memory incremental graph updater for Git commit delta events.
//! Updates nodes, edges, betweenness centrality, and token counts in <=50ms.

use crate::ast::parser::parse_markdown_ast;
use crate::graph::builder::GraphTopology;
use crate::model::NoteNode;
use crate::tokens::counter::count_bpe_tokens;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileDeltaStatus {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDeltaItem {
    pub path: String,
    pub status: FileDeltaStatus,
}

/// Applies incremental file delta changes to the in-memory GraphTopology.
pub fn apply_file_delta(
    topology: &mut GraphTopology,
    workspace_root: &Path,
    deltas: &[FileDeltaItem],
    active_hub: Option<&str>,
) {
    for item in deltas {
        let rel_str = item.path.replace('\\', "/");

        match item.status {
            FileDeltaStatus::Deleted => {
                // Remove node from map
                topology.nodes.remove(&rel_str);
                topology.headings.remove(&rel_str);

                // Remove outgoing edges from deleted file
                topology.edges.retain(|e| e.source != rel_str);

                // Mark any edges pointing to this deleted file as broken
                for edge in &mut topology.edges {
                    if edge.target == rel_str {
                        edge.is_broken = true;
                    }
                }
            }
            FileDeltaStatus::Added | FileDeltaStatus::Modified => {
                let abs_path = workspace_root.join(&rel_str);
                if !abs_path.is_file() {
                    continue;
                }

                let content = match std::fs::read_to_string(&abs_path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                let tokens = count_bpe_tokens(&content).unwrap_or(0);
                let parsed = parse_markdown_ast(&rel_str, &content);

                let label = parsed.title.unwrap_or_else(|| {
                    Path::new(&rel_str)
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| rel_str.clone())
                });

                let mut node =
                    NoteNode::new(&rel_str, abs_path.to_string_lossy().to_string(), label);
                node.tokens = tokens;
                node.tags = parsed.tags;

                topology.add_node(node);
                topology.set_headings(rel_str.clone(), parsed.headings);

                // Replace outgoing edges from this file
                topology.edges.retain(|e| e.source != rel_str);
                for edge in parsed.edges {
                    topology.add_edge(edge);
                }
            }
        }
    }

    // 1. Re-resolve all link destinations and anchors
    topology.resolve_links();

    // 2. Recompute betweenness centrality
    topology.compute_betweenness();

    // 3. Recompute BFS hops if hub is specified or suggested
    if let Some(hub) = active_hub {
        topology.compute_hops(hub);
    } else if let Some(suggested) = topology.suggest_hub() {
        topology.compute_hops(&suggested);
    }
}

/// Parses the output of `git diff --name-status` into a list of FileDeltaItem.
pub fn parse_git_diff_name_status(stdout: &str) -> Vec<FileDeltaItem> {
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
        if status_code.starts_with('A') {
            let path = parts[1].replace('\\', "/");
            if crate::drift::extractor::validate_path_containment(&path).is_ok() {
                items.push(FileDeltaItem {
                    path,
                    status: FileDeltaStatus::Added,
                });
            }
        } else if status_code.starts_with('D') {
            let path = parts[1].replace('\\', "/");
            if crate::drift::extractor::validate_path_containment(&path).is_ok() {
                items.push(FileDeltaItem {
                    path,
                    status: FileDeltaStatus::Deleted,
                });
            }
        } else if status_code.starts_with('R') {
            let old_path = parts[1].replace('\\', "/");
            let new_path = if parts.len() >= 3 {
                parts[2].replace('\\', "/")
            } else {
                old_path.clone()
            };
            if crate::drift::extractor::validate_path_containment(&old_path).is_ok() {
                items.push(FileDeltaItem {
                    path: old_path,
                    status: FileDeltaStatus::Deleted,
                });
            }
            if crate::drift::extractor::validate_path_containment(&new_path).is_ok() {
                items.push(FileDeltaItem {
                    path: new_path,
                    status: FileDeltaStatus::Added,
                });
            }
        } else {
            let path = parts[1].replace('\\', "/");
            if crate::drift::extractor::validate_path_containment(&path).is_ok() {
                items.push(FileDeltaItem {
                    path,
                    status: FileDeltaStatus::Modified,
                });
            }
        }
    }
    items
}

/// Executes `git diff --name-status <range>` (e.g. `base...head` or `A..B`) and parses changed files.
pub fn parse_git_diff_range(workspace_root: &Path, range: &str) -> Result<Vec<FileDeltaItem>, String> {
    let output = std::process::Command::new("git")
        .args(["diff", "--name-status", range])
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to execute git diff: {e}"))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git diff error: {err}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_git_diff_name_status(&stdout))
}

/// Executes `git diff --cached --name-status` and parses staged changes.
pub fn parse_git_diff_staged(workspace_root: &Path) -> Result<Vec<FileDeltaItem>, String> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-status"])
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to execute git diff --cached: {e}"))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git diff error: {err}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_git_diff_name_status(&stdout))
}
