//! Git delta notification dispatcher for `llm-wiki-core git-delta`.
//! Enforces Constitution Principle II:
//! <= 500ms fail-open timeout. Always exits 0 if daemon is offline or on timeout.

use std::path::Path;
use std::process::Command;
use std::time::Duration;
use futures_util::SinkExt;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::debt::baseline::{
    diff_against_baseline, extract_findings_from_topology, load_baseline, ratchet_baseline,
    save_baseline,
};
use crate::graph::builder::GraphTopology;
use crate::ipc::protocol::{ChangedFileItem, GitCommitDeltaParams, JsonRpcNotification};

/// Resolves daemon port by reading `.git/llm-wiki.port` or falling back to default 8989.
pub fn resolve_daemon_port(workspace_root: &Path) -> u16 {
    let port_file = workspace_root.join(".git").join("llm-wiki.port");
    if port_file.is_file() {
        if let Ok(content) = std::fs::read_to_string(&port_file) {
            if let Ok(port) = content.trim().parse::<u16>() {
                return port;
            }
        }
    }
    8989
}

/// Dispatches git commit delta notification to running sidecar daemon and applies baseline ratchet.
/// Enforces non-blocking fail-open: returns Ok(()) even if daemon is unreachable or times out.
pub async fn execute_git_delta(
    workspace_root: &Path,
    files: &[String],
    commit_sha: Option<&str>,
    quiet: bool,
    timeout_duration: Duration,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let delta_task = async {
        let sha = commit_sha
            .map(|s| s.to_string())
            .or_else(|| get_head_commit_sha(workspace_root))
            .unwrap_or_else(|| "HEAD".to_string());

        let resolved_files: Vec<String> = if files.is_empty() {
            get_committed_files(workspace_root).unwrap_or_default()
        } else {
            files.to_vec()
        };

        // 1. Attempt WebSocket notification to running daemon
        let port = resolve_daemon_port(workspace_root);
        let url = format!("ws://127.0.0.1:{}/ws", port);

        let changed_files: Vec<ChangedFileItem> = resolved_files
            .iter()
            .map(|f| {
                let full_path = workspace_root.join(f);
                let status = if full_path.exists() {
                    "modified".to_string()
                } else {
                    "deleted".to_string()
                };
                ChangedFileItem {
                    path: f.clone(),
                    status,
                }
            })
            .collect();

        if !changed_files.is_empty() {
            let params = GitCommitDeltaParams {
                repo_path: workspace_root.to_string_lossy().to_string(),
                commit_hash: Some(sha.clone()),
                branch: None,
                changed_files,
            };

            let notification =
                JsonRpcNotification::new("git/commitDelta", serde_json::to_value(params).unwrap_or_default());
            if let Ok(serialized) = serde_json::to_string(&notification) {
                if let Ok((mut ws_stream, _)) = connect_async(&url).await {
                    let _ = ws_stream.send(Message::Text(serialized)).await;
                    let _ = ws_stream.close(None).await;
                }
            }
        }

        // 2. Baseline Ratchet: if baseline exists, check if debt was resolved by this commit
        if let Ok(Some(mut baseline)) = load_baseline(workspace_root) {
            if let Ok(scan_res) = execute_scan(ScanOptions {
                workspace_root: workspace_root.to_path_buf(),
                hub: None,
                pretty: false,
            }) {
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

                let current_findings = extract_findings_from_topology(&topology, vec![], vec![]);
                let diff = diff_against_baseline(&current_findings, &baseline);

                if diff.summary.resolved_debt > 0 {
                    ratchet_baseline(&mut baseline, &diff, &sha);
                    let _ = save_baseline(workspace_root, &baseline);
                    if !quiet {
                        println!(
                            "[post-commit] Synced commit {}. Updated debt baseline: {} items.",
                            sha, baseline.total_debt
                        );
                    }
                } else if !quiet {
                    println!("[post-commit] Synced commit {}.", sha);
                }
            }
        } else if !quiet {
            println!("[post-commit] Synced commit {}.", sha);
        }

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    };

    // Strict non-blocking timeout: if anything takes > timeout_duration, fail open cleanly
    match timeout(timeout_duration, delta_task).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(_err)) => Ok(()),
        Err(_timeout_elapsed) => Ok(()),
    }
}

fn get_head_commit_sha(workspace_root: &Path) -> Option<String> {
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

fn get_committed_files(workspace_root: &Path) -> Option<Vec<String>> {
    let output = Command::new("git")
        .args(["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"])
        .current_dir(workspace_root)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        return Some(files);
    }
    None
}
