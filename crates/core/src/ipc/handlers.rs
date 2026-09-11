//! Request and notification handlers for JSON-RPC 2.0 sidecar server.

use super::protocol::{
    ExcludePathParams, ExcludePathResult, GitCommitDeltaParams, JsonRpcNotification,
    JsonRpcRequest, JsonRpcResponse, ObservatorySummaryCounts, PreviewExclusionParams,
    SetCentralHubParams, SetCentralHubResult, SetHubParams, SuggestExclusionsResult,
};
use crate::cli::scan::{execute_scan, ScanOptions};
use crate::config::ContextVaultConfig;
use crate::graph::builder::GraphTopology;
use crate::graph::exclusions::ExclusionEngine;
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

pub async fn handle_request(state: &Arc<RwLock<AppState>>, req: JsonRpcRequest) -> JsonRpcResponse {
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
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params: {}", e),
                        )
                    }
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
        "graph/setCentralHub" => {
            let params = match req.params {
                Some(p) => match serde_json::from_value::<SetCentralHubParams>(p) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params: {}", e),
                        )
                    }
                },
                None => return JsonRpcResponse::error(req.id, -32602, "Missing params"),
            };

            let mut st = state.write().await;
            st.active_hub = Some(params.hub_path.clone());
            st.config.vault.auto_discovered_hub = Some(params.hub_path.clone());
            st.config.vault.hubs.retain(|h| h != &params.hub_path);
            st.config.vault.hubs.insert(0, params.hub_path.clone());

            let config_path = st.workspace_root.join(".context-vault.yaml");
            let _ = st.config.save_to_file(&config_path);

            st.topology.compute_hops(&params.hub_path);

            let result = SetCentralHubResult {
                success: true,
                primary_hub: params.hub_path,
            };

            JsonRpcResponse::success(req.id, json!(result))
        }
        "graph/previewExclusion" => {
            let params = match req.params {
                Some(p) => match serde_json::from_value::<PreviewExclusionParams>(p) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params: {}", e),
                        )
                    }
                },
                None => return JsonRpcResponse::error(req.id, -32602, "Missing params"),
            };

            let candidates: Vec<String> = {
                let st = state.read().await;
                st.topology.nodes.keys().cloned().collect()
            };

            match ExclusionEngine::preview(&params.pattern, &candidates) {
                Ok(result) => JsonRpcResponse::success(req.id, json!(result)),
                Err(e) => {
                    JsonRpcResponse::error(req.id, -32602, format!("Invalid glob pattern: {}", e))
                }
            }
        }
        "graph/excludePath" => {
            let params = match req.params {
                Some(p) => match serde_json::from_value::<ExcludePathParams>(p) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params: {}", e),
                        )
                    }
                },
                None => return JsonRpcResponse::error(req.id, -32602, "Missing params"),
            };

            let glob = match globset::Glob::new(&params.pattern) {
                Ok(g) => g.compile_matcher(),
                Err(e) => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        format!("Invalid glob pattern: {}", e),
                    )
                }
            };

            let mut st = state.write().await;
            let mut affected_count = 0u32;

            if params.scope == "scan" {
                let scan_mut = st.config.scan.get_or_insert_with(Default::default);
                if !scan_mut.exclude.contains(&params.pattern) {
                    scan_mut.exclude.push(params.pattern.clone());
                }
                let matching_keys: Vec<String> = st
                    .topology
                    .nodes
                    .keys()
                    .filter(|k| glob.is_match(k))
                    .cloned()
                    .collect();
                affected_count = matching_keys.len() as u32;
                for k in matching_keys {
                    st.topology.nodes.remove(&k);
                    st.topology.headings.remove(&k);
                    st.topology.edges.retain(|e| e.source != k);
                }
            } else {
                let graph_mut = st.config.graph.get_or_insert_with(Default::default);
                if !graph_mut.exclude.contains(&params.pattern) {
                    graph_mut.exclude.push(params.pattern.clone());
                }
                for (id, node) in st.topology.nodes.iter_mut() {
                    if glob.is_match(id) {
                        node.status = crate::model::NodeStatus::Excluded;
                        affected_count += 1;
                    }
                }
            }

            st.topology.resolve_links();
            if let Some(ref hub) = st.active_hub.clone() {
                st.topology.compute_hops(hub);
            }

            let config_path = st.workspace_root.join(".context-vault.yaml");
            let _ = st.config.save_to_file(&config_path);

            let summary = st.topology.compute_observatory_summary(None);

            let result = ExcludePathResult {
                success: true,
                affected_files_count: affected_count,
                updated_summary: ObservatorySummaryCounts {
                    total_notes: summary.total_notes,
                    connected_notes: summary.connected_notes,
                    orphan_notes: summary.orphan_notes,
                    excluded_notes: summary.excluded_notes,
                },
            };

            JsonRpcResponse::success(req.id, json!(result))
        }
        "graph/suggestExclusions" => {
            let orphans: Vec<String> = {
                let st = state.read().await;
                st.topology
                    .nodes
                    .values()
                    .filter(|n| n.status == crate::model::NodeStatus::Orphan)
                    .map(|n| n.id.clone())
                    .collect()
            };

            let suggestions = ExclusionEngine::detect_noise_suggestions(&orphans);
            let result = SuggestExclusionsResult { suggestions };

            JsonRpcResponse::success(req.id, json!(result))
        }
        _ => JsonRpcResponse::error(req.id, -32601, format!("Method not found: {}", req.method)),
    }
}

pub async fn handle_notification(state: &Arc<RwLock<AppState>>, notif: JsonRpcNotification) {
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
