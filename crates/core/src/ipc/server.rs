//! Async WebSocket JSON-RPC 2.0 daemon server using tokio-tungstenite.

use super::handlers::{handle_notification, handle_request, AppState};
use super::protocol::{JsonRpcNotification, JsonRpcRequest};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{oneshot, RwLock};
use tokio_tungstenite::accept_hdr_async_with_config;
use tokio_tungstenite::tungstenite::{
    handshake::server::{Request, Response},
    protocol::WebSocketConfig,
    Message,
};

pub async fn start_ipc_server(
    port: u16,
    workspace_root: PathBuf,
    auth_token: Option<String>,
) -> anyhow::Result<(SocketAddr, oneshot::Sender<()>)> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    let addr = listener.local_addr()?;

    // Record port in .git/llm-wiki.port if .git exists
    let git_dir = workspace_root.join(".git");
    if git_dir.exists() && git_dir.is_dir() {
        let port_file = git_dir.join("llm-wiki.port");
        let _ = std::fs::write(&port_file, addr.port().to_string());
    }

    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
    let state = Arc::new(RwLock::new(AppState::new(workspace_root)));
    let token_arc = Arc::new(auth_token);

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, peer_addr)) => {
                            let state_clone = Arc::clone(&state);
                            let token_clone = Arc::clone(&token_arc);
                            tokio::spawn(handle_connection(stream, peer_addr, state_clone, token_clone));
                        }
                        Err(e) => {
                            eprintln!("Error accepting connection: {}", e);
                        }
                    }
                }
            }
        }
    });

    Ok((addr, shutdown_tx))
}

#[allow(clippy::result_large_err)]
async fn handle_connection(
    stream: TcpStream,
    peer_addr: SocketAddr,
    state: Arc<RwLock<AppState>>,
    expected_token: Arc<Option<String>>,
) {
    let mut is_authenticated = expected_token.is_none();
    let is_loopback = peer_addr.ip().is_loopback();

    let callback = |req: &Request, response: Response| {
        // Check Authorization: Bearer <token>
        if let Some(ref token) = *expected_token {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(val) = auth_header.to_str() {
                    if val.trim().strip_prefix("Bearer ").map(|s| s.trim()) == Some(token.as_str())
                    {
                        is_authenticated = true;
                    }
                }
            }
        }
        Ok(response)
    };

    let mut ws_config = WebSocketConfig::default();
    ws_config.max_message_size = Some(256 * 1024 * 1024);
    ws_config.max_frame_size = Some(256 * 1024 * 1024);

    let ws_stream = match accept_hdr_async_with_config(stream, callback, Some(ws_config)).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake error with {}: {}", peer_addr, e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg_res) = read.next().await {
        let msg = match msg_res {
            Ok(m) => m,
            Err(_) => break,
        };

        if msg.is_close() {
            break;
        }

        if let Ok(text) = msg.to_text() {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(text) {
                if val.get("id").is_some() {
                    // JSON-RPC Request
                    if let Ok(req) = serde_json::from_value::<JsonRpcRequest>(val) {
                        if !is_authenticated {
                            let err = super::protocol::JsonRpcResponse::error(
                                req.id,
                                -32001,
                                "Unauthorized: valid token required for RPC requests",
                            );
                            let res_str = serde_json::to_string(&err).unwrap();
                            let _ = write.send(Message::Text(res_str)).await;
                            continue;
                        }

                        let response = handle_request(&state, req).await;
                        if let Ok(res_str) = serde_json::to_string(&response) {
                            if write.send(Message::Text(res_str)).await.is_err() {
                                break;
                            }
                        }
                    }
                } else {
                    // JSON-RPC Notification (no id)
                    if let Ok(notif) = serde_json::from_value::<JsonRpcNotification>(val) {
                        if (notif.method == "git/commitDelta" && is_loopback) || is_authenticated {
                            handle_notification(&state, notif).await;
                        }
                    }
                }
            }
        }
    }
}
