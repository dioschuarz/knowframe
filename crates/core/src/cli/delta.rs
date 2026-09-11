//! Git delta notification dispatcher for `llm-wiki-core git-delta`.
//! Enforces Constitution Principle II:
//! <= 500ms fail-open timeout. Always exits 0 if daemon is offline.

use crate::ipc::protocol::{ChangedFileItem, GitCommitDeltaParams, JsonRpcNotification};
use futures_util::SinkExt;
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;


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

/// Dispatches git commit delta notification to running sidecar daemon.
/// Enforces non-blocking fail-open: returns Ok(()) even if daemon is unreachable.
pub async fn execute_git_delta(
    workspace_root: &Path,
    files: &[String],
    timeout_duration: Duration,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let delta_task = async {
        let port = resolve_daemon_port(workspace_root);
        let url = format!("ws://127.0.0.1:{}/ws", port);

        // Map files to ChangedFileItem
        let changed_files: Vec<ChangedFileItem> = files
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

        if changed_files.is_empty() {
            return Ok(());
        }

        let params = GitCommitDeltaParams {
            repo_path: workspace_root.to_string_lossy().to_string(),
            commit_hash: None,
            branch: None,
            changed_files,
        };

        let notification = JsonRpcNotification::new("git/commitDelta", serde_json::to_value(params)?);
        let serialized = serde_json::to_string(&notification)?;

        // Connect to WebSocket with loopback write-only authorization
        let (mut ws_stream, _) = connect_async(&url).await?;
        ws_stream.send(Message::Text(serialized)).await?;
        let _ = ws_stream.close(None).await;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    };

    // Strict non-blocking timeout: if daemon is offline or slow, fail open cleanly
    match timeout(timeout_duration, delta_task).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(_err)) => {
            // Daemon connection or write error: fail open silently
            Ok(())
        }
        Err(_timeout_elapsed) => {
            // Timeout elapsed: fail open silently
            Ok(())
        }
    }
}
