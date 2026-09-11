//! Request and notification handlers for JSON-RPC 2.0 sidecar server.

use super::protocol::{
    GitCommitDeltaParams, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, SetHubParams,
};
use crate::cli::scan::{execute_scan, ScanOptions};
use crate::config::ContextVaultConfig;
use crate::graph::builder::GraphTopology;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub workspace_root: PathBuf,
    pub topology: GraphTopology,
    pub config: ContextVaultConfig,
    pub active_hub: Option<String>,
}

impl AppState {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            topology: GraphTopology::new(),
            config: ContextVaultConfig::default(),
            active_hub: None,
        }
    }
}

pub async fn handle_request(
    state: &Arc<RwLock<AppState>>,
    req: JsonRpcRequest,
) -> JsonRpcResponse {
    match req.method.as_str() {
        "workspace/scan" => {
            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };
            let options = ScanOptions {
                workspace_root: root,
                hub: None,
                pretty: false,
            };
            match execute_scan(options) {
                Ok(payload) => {
                    let mut st = state.write().await;
                    st.active_hub = payload.recommended_hub.clone();
                    JsonRpcResponse::success(req.id, json!(payload))
                }
                Err(e) => JsonRpcResponse::error(req.id, -32000, format!("Scan failed: {}", e)),
            }
        }
        "graph/setHub" => {
            let params = match req.params {
                Some(p) => match serde_json::from_value::<SetHubParams>(p) {
                    Ok(parsed) => parsed,
                    Err(e) => return JsonRpcResponse::error(req.id, -32602, format!("Invalid params: {}", e)),
                },
                None => return JsonRpcResponse::error(req.id, -32602, "Missing params"),
            };

            let mut st = state.write().await;
            st.active_hub = Some(params.hub_id.clone());
            st.topology.compute_hops(&params.hub_id);

            JsonRpcResponse::success(
                req.id,
                json!({
                    "success": true,
                    "hub_id": params.hub_id,
                }),
            )
        }
        _ => JsonRpcResponse::error(req.id, -32601, format!("Method not found: {}", req.method)),
    }
}

pub async fn handle_notification(
    state: &Arc<RwLock<AppState>>,
    notif: JsonRpcNotification,
) {
    if notif.method == "git/commitDelta" {
        if let Ok(params) = serde_json::from_value::<GitCommitDeltaParams>(notif.params) {
            let mut st = state.write().await;
            let root = st.workspace_root.clone();
            let hub = st.active_hub.clone();

            let items: Vec<crate::delta::updater::FileDeltaItem> = params
                .changed_files
                .into_iter()
                .map(|f| {
                    let status = match f.status.as_str() {
                        "added" => crate::delta::updater::FileDeltaStatus::Added,
                        "deleted" => crate::delta::updater::FileDeltaStatus::Deleted,
                        _ => crate::delta::updater::FileDeltaStatus::Modified,
                    };
                    crate::delta::updater::FileDeltaItem {
                        path: f.path,
                        status,
                    }
                })
                .collect();

            crate::delta::updater::apply_file_delta(
                &mut st.topology,
                &root,
                &items,
                hub.as_deref(),
            );
        }
    }
}
