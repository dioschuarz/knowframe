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
    pub cached_plans: std::collections::HashMap<String, crate::evolution::TransformationPlan>,
    pub active_harness: Option<String>,
    pub governance_store: crate::evolution::transformation::GovernancePlanStore,
}

impl AppState {
    pub fn new(workspace_root: PathBuf) -> Self {
        let config = ContextVaultConfig::load_from_workspace(&workspace_root);
        Self {
            workspace_root,
            topology: GraphTopology::new(),
            config,
            active_hub: None,
            cached_plans: std::collections::HashMap::new(),
            active_harness: None,
            governance_store: crate::evolution::transformation::GovernancePlanStore::default(),
        }
    }

    pub fn get_active_profile(&self) -> crate::evolution::OptimizationProfile {
        crate::evolution::EvolutionEngine::compile_active_profile(
            self.config.get_orientation(),
            self.config.custom_profile.as_ref(),
        )
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
                    st.topology = GraphTopology::new();
                    for node in &payload.nodes {
                        let mut n = crate::model::NoteNode::new(
                            node.id.clone(),
                            node.file_path.clone(),
                            node.label.clone(),
                        );
                        n.tags = node.tags.clone();
                        n.tokens = node.tokens;
                        n.status = node.status;
                        n.color = node.color.clone();
                        n.hop_distance = node.hop_distance;
                        n.in_degree = node.in_degree;
                        n.out_degree = node.out_degree;
                        n.betweenness = node.betweenness;
                        n.broken_links = node.broken_links;
                        n.fragments = node.fragments.clone();
                        st.topology.set_headings(
                            node.id.clone(),
                            node.fragments.iter().map(|f| f.heading.clone()).collect(),
                        );
                        st.topology.add_node(n);
                    }
                    for edge in &payload.edges {
                        let mut e = crate::model::NoteEdge::new(
                            edge.source.clone(),
                            edge.target.clone(),
                            edge.link_type,
                            String::new(),
                            edge.anchor.clone(),
                            edge.line,
                        );
                        e.is_broken = edge.is_broken;
                        e.target_fragment = edge.target_fragment.clone();
                        e.resolution_status = edge.resolution_status;
                        st.topology.add_edge(e);
                    }
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

            let pattern_trimmed = params.pattern.trim().to_string();
            let glob = match globset::Glob::new(&pattern_trimmed) {
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
            let config_path = crate::config::canonical_config_path(&st.workspace_root);
            if let Ok(fresh_cfg) = ContextVaultConfig::load_from_file(&config_path) {
                st.config = fresh_cfg;
            }

            let mut affected_count = 0u32;

            if params.scope == "scan" {
                let scan_mut = st.config.scan.get_or_insert_with(Default::default);
                if !scan_mut.exclude.contains(&pattern_trimmed) {
                    scan_mut.exclude.push(pattern_trimmed.clone());
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
                if !graph_mut.exclude.contains(&pattern_trimmed) {
                    graph_mut.exclude.push(pattern_trimmed.clone());
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

            if let Err(e) = st.config.save_to_file(&config_path) {
                eprintln!("[ERROR] Failed to save config to {:?}: {}", config_path, e);
            }

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
        "graph/getExclusions" => {
            let mut st = state.write().await;
            let config_path = crate::config::canonical_config_path(&st.workspace_root);
            if let Ok(cfg) = ContextVaultConfig::load_from_file(&config_path) {
                st.config = cfg;
            }
            JsonRpcResponse::success(
                req.id,
                json!({
                    "scan_exclusions": st.config.get_scan_exclude(),
                    "graph_exclusions": st.config.get_graph_exclude(),
                }),
            )
        }
        "graph/removeExclusion" => {
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

            let mut st = state.write().await;
            let config_path = crate::config::canonical_config_path(&st.workspace_root);
            if let Ok(fresh_cfg) = ContextVaultConfig::load_from_file(&config_path) {
                st.config = fresh_cfg;
            }

            let pattern_to_remove = params.pattern.trim();
            let pattern_no_prefix = pattern_to_remove.trim_start_matches("./");

            if params.scope == "scan" {
                if let Some(ref mut scan) = st.config.scan {
                    scan.exclude.retain(|p| {
                        let p_clean = p.trim();
                        p_clean != pattern_to_remove
                            && p_clean != pattern_no_prefix
                            && p_clean.trim_start_matches("./") != pattern_no_prefix
                    });
                }
            } else {
                if let Some(ref mut graph) = st.config.graph {
                    graph.exclude.retain(|p| {
                        let p_clean = p.trim();
                        p_clean != pattern_to_remove
                            && p_clean != pattern_no_prefix
                            && p_clean.trim_start_matches("./") != pattern_no_prefix
                    });
                }
                let remaining_graph_excludes = st.config.get_graph_exclude();
                let remaining_engine = crate::graph::exclusions::ExclusionEngine::new(
                    &st.config.get_scan_exclude(),
                    &remaining_graph_excludes,
                );
                if let Ok(engine) = remaining_engine {
                    for (id, node) in st.topology.nodes.iter_mut() {
                        if node.status == crate::model::NodeStatus::Excluded
                            && !engine.is_graph_excluded(id)
                        {
                            if node.in_degree == 0 && node.out_degree == 0 {
                                node.status = crate::model::NodeStatus::Orphan;
                            } else {
                                node.status = crate::model::NodeStatus::Active;
                            }
                        }
                    }
                }
            }

            st.topology.resolve_links();
            if let Some(ref hub) = st.active_hub.clone() {
                st.topology.compute_hops(hub);
            }

            if let Err(e) = st.config.save_to_file(&config_path) {
                eprintln!("[ERROR] Failed to save config to {:?}: {}", config_path, e);
            }

            JsonRpcResponse::success(
                req.id,
                json!({
                    "success": true,
                    "scan_exclusions": st.config.get_scan_exclude(),
                    "graph_exclusions": st.config.get_graph_exclude(),
                }),
            )
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
        "workspace/getQualityScore" => {
            let (root, mut topology) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.topology.clone())
            };
            if topology.nodes.is_empty() {
                if let Ok(scan_res) = execute_scan(ScanOptions {
                    workspace_root: root.clone(),
                    hub: None,
                    pretty: false,
                }) {
                    for node in scan_res.nodes {
                        let mut n = crate::model::NoteNode::new(
                            node.id.clone(),
                            node.file_path,
                            node.label,
                        );
                        n.tags = node.tags;
                        n.tokens = node.tokens;
                        n.status = node.status;
                        n.color = node.color;
                        n.fragments = node.fragments.clone();
                        topology.set_headings(
                            node.id.clone(),
                            node.fragments.iter().map(|f| f.heading.clone()).collect(),
                        );
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
                }
            }
            topology.compute_degrees();
            topology.compute_betweenness();
            if let Some(hub) = topology.suggest_hub() {
                topology.compute_hops(&hub);
            }
            let baseline = crate::debt::baseline::load_baseline(&root).ok().flatten();
            let collisions = crate::collision::detect_collisions_in_graph(&topology, &root, 0.70);
            let drifts = crate::drift::detect_all_drifts_in_graph(&topology, &root);
            let quality = crate::quality::calculate_quality_score(
                &topology,
                baseline.as_ref(),
                &collisions,
                &drifts,
            );
            JsonRpcResponse::success(
                req.id,
                json!({
                    "overall_score": quality.overall_score,
                    "integrity_score": quality.integrity_score,
                    "structure_score": quality.structure_score,
                    "consistency_score": quality.consistency_score,
                    "discoverability_score": quality.discoverability_score,
                    "freshness_score": quality.freshness_score,
                    "legacy_debt_count": quality.legacy_debt_count,
                    "regressions_count": quality.new_regressions_count,
                    "resolved_debt_count": quality.resolved_debt_count,
                    "central_hub_id": quality.central_hub_id,
                    "top_degradation_factors": quality.top_degradation_factors,
                    "total_issues": quality.total_issues,
                    "findings_count": quality.findings_count,
                    "broken_references": quality.broken_references,
                    "invalid_anchors": quality.invalid_anchors,
                    "raw_score": quality.raw_score,
                    "is_effective": quality.is_effective,
                    "active_issues_count": quality.active_issues_count,
                }),
            )
        }
        "workspace/getIssues" => {
            let (root, mut topology) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.topology.clone())
            };
            if topology.nodes.is_empty() {
                if let Ok(scan_res) = execute_scan(ScanOptions {
                    workspace_root: root.clone(),
                    hub: None,
                    pretty: false,
                }) {
                    for node in scan_res.nodes {
                        let mut n = crate::model::NoteNode::new(
                            node.id.clone(),
                            node.file_path,
                            node.label,
                        );
                        n.tags = node.tags;
                        n.tokens = node.tokens;
                        n.status = node.status;
                        n.color = node.color;
                        n.fragments = node.fragments.clone();
                        topology.set_headings(
                            node.id.clone(),
                            node.fragments.iter().map(|f| f.heading.clone()).collect(),
                        );
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
                    topology.compute_degrees();
                    topology.compute_betweenness();
                    if let Some(hub) = topology.suggest_hub() {
                        topology.compute_hops(&hub);
                    }
                }
            }
            let baseline = crate::debt::baseline::load_baseline(&root).ok().flatten();
            let collisions = crate::collision::detect_collisions_in_graph(&topology, &root, 0.70);
            let drifts = crate::drift::detect_all_drifts_in_graph(&topology, &root);
            let quality = crate::quality::calculate_quality_score(
                &topology,
                baseline.as_ref(),
                &collisions,
                &drifts,
            );
            let all_issues = crate::quality::extract_all_issues_with_baseline(
                &topology,
                &collisions,
                &drifts,
                baseline.as_ref(),
            );

            let include_debt = req
                .params
                .as_ref()
                .and_then(|p| p.get("include_debt").or_else(|| p.get("includeDebt")))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let include_excluded = req
                .params
                .as_ref()
                .and_then(|p| p.get("include_excluded").or_else(|| p.get("includeExcluded")))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let filter_path = req
                .params
                .as_ref()
                .and_then(|p| {
                    p.get("filter_path")
                        .or_else(|| p.get("filterPath"))
                        .or_else(|| p.get("path"))
                })
                .and_then(|v| v.as_str())
                .map(|s| s.trim_start_matches("./").to_string());

            let total_issues = all_issues.len();
            let legacy_debt_count = all_issues.iter().filter(|i| i.is_legacy).count();
            let active_issues_count = all_issues.iter().filter(|i| !i.is_legacy).count();

            let mut filtered_issues = all_issues;

            if !include_debt {
                filtered_issues.retain(|i| !i.is_legacy);
            }

            if !include_excluded {
                filtered_issues.retain(|i| {
                    if let Some(node) = topology.nodes.get(&i.file_path) {
                        node.status != crate::model::NodeStatus::Excluded
                    } else {
                        true
                    }
                });
            }

            if let Some(ref prefix) = filter_path {
                filtered_issues.retain(|i| {
                    i.file_path.starts_with(prefix) || i.file_path.contains(prefix)
                });
            }

            let pillar_filter = req
                .params
                .as_ref()
                .and_then(|p| p.get("pillar"))
                .and_then(|v| v.as_str())
                .unwrap_or("all")
                .to_lowercase();

            if pillar_filter != "all" && !pillar_filter.is_empty() {
                filtered_issues.retain(|i| i.pillar.to_lowercase() == pillar_filter);
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

            let limit = req
                .params
                .as_ref()
                .and_then(|p| p.get("limit"))
                .and_then(|v| v.as_u64())
                .map(|v| v as usize);

            let report_issues = if let Some(lim) = limit {
                filtered_issues.into_iter().take(lim).collect()
            } else {
                filtered_issues
            };

            JsonRpcResponse::success(
                req.id,
                json!({
                    "overall_score": quality.overall_score,
                    "total_issues": total_issues,
                    "active_issues_count": active_issues_count,
                    "legacy_debt_count": legacy_debt_count,
                    "is_quiet_default": !include_debt,
                    "raw_score": quality.raw_score,
                    "is_effective": quality.is_effective,
                    "integrity_count": integrity_count,
                    "structure_count": structure_count,
                    "consistency_count": consistency_count,
                    "discoverability_count": discoverability_count,
                    "freshness_count": freshness_count,
                    "issues": report_issues,
                }),
            )
        }
        "workspace/getBaseline" => {
            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };
            match crate::debt::baseline::load_baseline(&root) {
                Ok(Some(base)) => JsonRpcResponse::success(
                    req.id,
                    json!({
                        "baseline_commit": base.baseline_commit,
                        "timestamp": base.timestamp,
                        "broken_links_count": base.broken_links.len(),
                        "orphan_nodes_count": base.orphan_nodes.len(),
                        "collisions_count": base.collisions.len(),
                        "drifts_count": base.technical_drifts.len(),
                        "total_debt": base.total_debt,
                    }),
                ),
                Ok(None) => JsonRpcResponse::success(
                    req.id,
                    json!({
                        "baseline_commit": "unbaselined",
                        "timestamp": "",
                        "broken_links_count": 0,
                        "orphan_nodes_count": 0,
                        "collisions_count": 0,
                        "drifts_count": 0,
                        "total_debt": 0,
                    }),
                ),
                Err(e) => JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Failed to read baseline: {}", e),
                ),
            }
        }
        "workspace/updateBaseline" => {
            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };
            let prev_total = crate::debt::baseline::load_baseline(&root)
                .ok()
                .flatten()
                .map(|b| b.total_debt)
                .unwrap_or(0);

            match crate::cli::baseline_cmd::execute_baseline_reset(&root) {
                Ok(_) => {
                    let new_base = crate::debt::baseline::load_baseline(&root).ok().flatten();
                    let (commit, total) = new_base
                        .map(|b| (b.baseline_commit, b.total_debt))
                        .unwrap_or_else(|| ("unknown".to_string(), 0));
                    JsonRpcResponse::success(
                        req.id,
                        json!({
                            "baseline_commit": commit,
                            "total_debt": total,
                            "previous_total_debt": prev_total,
                        }),
                    )
                }
                Err(e) => JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Failed to update baseline: {}", e),
                ),
            }
        }
        "workspace/traceContext" => {
            let (query, entrypoint) = match req.params {
                Some(ref p) => (
                    p.get("query")
                        .and_then(|q| q.as_str())
                        .unwrap_or("")
                        .to_string(),
                    p.get("entrypoint")
                        .and_then(|e| e.as_str())
                        .map(|s| s.to_string()),
                ),
                None => return JsonRpcResponse::error(req.id, -32602, "Missing params"),
            };

            let (root, mut topology) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.topology.clone())
            };
            if topology.nodes.is_empty() {
                if let Ok(scan_res) = execute_scan(ScanOptions {
                    workspace_root: root,
                    hub: None,
                    pretty: false,
                }) {
                    for node in scan_res.nodes {
                        let mut n = crate::model::NoteNode::new(
                            node.id.clone(),
                            node.file_path,
                            node.label,
                        );
                        n.tags = node.tags;
                        n.tokens = node.tokens;
                        n.status = node.status;
                        n.color = node.color;
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
                }
            }

            let trace_res = crate::trace::traversal::execute_context_trace(
                &topology,
                &query,
                entrypoint.as_deref(),
            );
            JsonRpcResponse::success(req.id, json!(trace_res))
        }
        "workspace/validateCandidate" => {
            let (root, base_graph) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.topology.clone())
            };
            let staged_delta = match req.params {
                Some(ref p) => {
                    let items_json = p.get("staged_files").and_then(|v| v.as_array());
                    let mut items = Vec::new();
                    if let Some(arr) = items_json {
                        for item in arr {
                            if let (Some(path), Some(status_str)) = (
                                item.get("path").and_then(|s| s.as_str()),
                                item.get("status").and_then(|s| s.as_str()),
                            ) {
                                let status = match status_str {
                                    "added" => crate::delta::updater::FileDeltaStatus::Added,
                                    "deleted" => crate::delta::updater::FileDeltaStatus::Deleted,
                                    _ => crate::delta::updater::FileDeltaStatus::Modified,
                                };
                                items.push(crate::delta::updater::FileDeltaItem {
                                    path: path.to_string(),
                                    status,
                                });
                            }
                        }
                    }
                    let commit = p
                        .get("base_commit")
                        .and_then(|c| c.as_str())
                        .map(|s| s.to_string());
                    crate::delta::GitDelta {
                        commit_sha: commit,
                        items,
                    }
                }
                None => crate::delta::GitDelta {
                    commit_sha: Some("HEAD".to_string()),
                    items: Vec::new(),
                },
            };

            let baseline = match crate::debt::baseline::load_baseline(&root) {
                Ok(Some(b)) => b,
                _ => crate::debt::baseline::capture_initial_baseline(
                    &base_graph,
                    "initial",
                    vec![],
                    vec![],
                ),
            };

            let overlay =
                crate::candidate::overlay::CandidateOverlay::new(&base_graph, &root, &staged_delta);
            let report = crate::candidate::evaluator::evaluate_candidate(
                &overlay,
                &baseline,
                vec![],
                vec![],
                crate::candidate::evaluator::AdmissionConfig::default(),
            );

            JsonRpcResponse::success(req.id, json!(report))
        }
        "graph/getDocumentFragments" => {
            let path = match req.params.and_then(|p| {
                p.get("path")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'path' parameter"),
            };
            let st = state.read().await;
            if let Some(node) = st.topology.get_node(&path) {
                JsonRpcResponse::success(
                    req.id,
                    json!({
                        "path": path,
                        "total_tokens": node.tokens,
                        "fragments": node.fragments,
                    }),
                )
            } else {
                let abs_path = st.workspace_root.join(&path);
                if let Ok(content) = std::fs::read_to_string(&abs_path) {
                    let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                    let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                    let fragments = crate::fragment::extractor::extract_fragments_from_document(
                        &content,
                        &parsed.edges,
                    );
                    JsonRpcResponse::success(
                        req.id,
                        json!({
                            "path": path,
                            "total_tokens": tokens,
                            "fragments": fragments,
                        }),
                    )
                } else {
                    JsonRpcResponse::error(req.id, -32004, format!("Document '{}' not found", path))
                }
            }
        }
        "graph/materializeFrontmatter" => {
            let path = match req.params.as_ref().and_then(|p| {
                p.get("path")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'path' parameter"),
            };
            let dry_run = req
                .params
                .as_ref()
                .and_then(|p| p.get("dry_run").and_then(|v| v.as_bool()))
                .unwrap_or(false);
            let st = state.read().await;
            let abs_path = st.workspace_root.join(&path);
            if !abs_path.exists() {
                return JsonRpcResponse::error(
                    req.id,
                    -32004,
                    format!("Document '{}' not found on disk", path),
                );
            }
            if dry_run {
                if let Ok(content) = std::fs::read_to_string(&abs_path) {
                    let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                    let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                    let fragments = crate::fragment::extractor::extract_fragments_from_document(
                        &content,
                        &parsed.edges,
                    );
                    JsonRpcResponse::success(
                        req.id,
                        json!({
                            "path": path,
                            "updated": false,
                            "fragment_count": fragments.len(),
                            "total_tokens": tokens,
                        }),
                    )
                } else {
                    JsonRpcResponse::error(req.id, -32000, "Failed to read file for dry run")
                }
            } else {
                match crate::cli::materialize_cmd::materialize_file(&abs_path, false) {
                    Ok(updated) => {
                        let content = std::fs::read_to_string(&abs_path).unwrap_or_default();
                        let tokens =
                            crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                        let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                        let fragments = crate::fragment::extractor::extract_fragments_from_document(
                            &content,
                            &parsed.edges,
                        );
                        JsonRpcResponse::success(
                            req.id,
                            json!({
                                "path": path,
                                "updated": updated,
                                "fragment_count": fragments.len(),
                                "total_tokens": tokens,
                            }),
                        )
                    }
                    Err(e) => {
                        JsonRpcResponse::error(req.id, -32000, format!("Materialize failed: {}", e))
                    }
                }
            }
        }
        "trace/contextTraceSection" => {
            let query = match req.params.as_ref().and_then(|p| {
                p.get("query")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(q) => q,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'query' parameter"),
            };
            let entrypoint = req.params.as_ref().and_then(|p| {
                p.get("entrypoint")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            });
            let st = state.read().await;
            match crate::trace::beam::trace_context_section(
                &st.topology,
                &query,
                entrypoint.as_deref(),
            ) {
                Some(res) => JsonRpcResponse::success(req.id, json!(res)),
                None => {
                    JsonRpcResponse::error(req.id, -32004, "No context trace section match found")
                }
            }
        }
        "quality/getFivePillarScore" => {
            let st = state.read().await;
            let root = st.workspace_root.clone();
            let base = crate::debt::baseline::load_baseline(&root).ok().flatten();
            let collisions =
                crate::collision::detect_collisions_in_graph(&st.topology, &root, 0.70);
            let drifts = crate::drift::detect_all_drifts_in_graph(&st.topology, &root);
            let score = crate::quality::scorer::calculate_quality_score(
                &st.topology,
                base.as_ref(),
                &collisions,
                &drifts,
            );

            let debt_summary = crate::debt::DebtSummary {
                total_debt: score.legacy_debt_count + score.new_regressions_count,
                legacy_debt: score.legacy_debt_count,
                new_debt: score.new_regressions_count,
                resolved_debt: score.resolved_debt_count,
                regressions: score.new_regressions_count,
            };

            let mut reasons: std::collections::HashMap<crate::quality::QualityPillar, Vec<String>> =
                std::collections::HashMap::new();
            for f in &score.top_degradation_factors {
                let pillar = match f.category.as_str() {
                    "broken_link" | "broken_anchor" => crate::quality::QualityPillar::Integrity,
                    "structure_debt" => crate::quality::QualityPillar::Structure,
                    "collision" => crate::quality::QualityPillar::Consistency,
                    "technical_drift" | "missing_code_citation" | "code_modified_after_doc" => {
                        crate::quality::QualityPillar::Freshness
                    }
                    _ => crate::quality::QualityPillar::Discoverability,
                };
                reasons
                    .entry(pillar)
                    .or_default()
                    .push(format!("{}: {}", f.affected_node, f.description));
            }

            let five_pillar = crate::quality::FivePillarQualityScore {
                overall_score: score.overall_score as f64,
                integrity_score: score.integrity_score as f64,
                structure_score: score.structure_score as f64,
                consistency_score: score.consistency_score as f64,
                discoverability_score: score.discoverability_score as f64,
                freshness_score: score.freshness_score as f64,
                reasons,
                debt_summary,
            };

            JsonRpcResponse::success(req.id, json!(five_pillar))
        }
        "manager/getSummary" => {
            let st = state.read().await;
            let orientation = st.config.get_orientation();
            let constraints = st.get_active_profile().constraints;
            let summary = crate::evolution::summary::compute_manager_summary(
                &st.topology,
                &st.workspace_root,
                orientation,
                &constraints,
            );
            JsonRpcResponse::success(req.id, json!(summary))
        }
        "profile/getOrientation" => {
            let st = state.read().await;
            let orientation = st.config.get_orientation();
            let profile = st.get_active_profile();
            JsonRpcResponse::success(
                req.id,
                json!({
                    "orientation": orientation,
                    "profile": profile,
                }),
            )
        }
        "profile/setOrientation" | "evolution/setOrientation" => {
            let orientation = match req.params.as_ref().and_then(|p| {
                p.get("orientation").and_then(|v| {
                    serde_json::from_value::<crate::evolution::DocumentationOrientation>(v.clone())
                        .ok()
                })
            }) {
                Some(o) => o,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Invalid or missing 'orientation' parameter",
                    )
                }
            };
            let mut st = state.write().await;
            st.config.orientation = Some(orientation);
            let config_path = st.workspace_root.join(".context-vault.yaml");
            let _ = st.config.save_to_file(&config_path);
            let profile = st.get_active_profile();
            JsonRpcResponse::success(
                req.id,
                json!({
                    "success": true,
                    "orientation": orientation,
                    "profile": profile,
                }),
            )
        }
        "evolution/updateCustomProfile" => {
            let params = match req.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(req.id, -32602, "Missing request parameters")
                }
            };
            let tier1 = params
                .get("tier1_preferences")
                .or_else(|| params.get("tier1"))
                .and_then(|v| serde_json::from_value::<crate::evolution::Tier1Objectives>(v.clone()).ok());
            let tier2 = params
                .get("tier2_parameters")
                .or_else(|| params.get("tier2"))
                .and_then(|v| serde_json::from_value::<crate::evolution::Tier2Parameters>(v.clone()).ok());
            let constraints: Vec<crate::evolution::ConstraintRule> = if let Some(rules) = params
                .get("hard_constraints")
                .and_then(|v| serde_json::from_value::<Vec<String>>(v.clone()).ok())
            {
                rules
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| crate::evolution::ConstraintRule {
                        id: format!("hc_{}", i),
                        level: crate::evolution::ConstraintLevel::HardConstraint,
                        rule_key: s.clone(),
                        description: s,
                    })
                    .collect()
            } else {
                params
                    .get("constraints")
                    .and_then(|v| serde_json::from_value::<Vec<crate::evolution::ConstraintRule>>(v.clone()).ok())
                    .unwrap_or_default()
            };

            let detector = crate::evolution::ConflictDetector::new();
            let (is_valid, conflicts) = detector.validate(&constraints);
            let conflict_reasons: Vec<String> = conflicts.into_iter().map(|c| c.reason).collect();

            let mut st = state.write().await;
            st.config.orientation = Some(crate::evolution::DocumentationOrientation::Custom);
            st.config.custom_profile = Some(crate::config::CustomProfileConfig {
                tier1,
                tier2,
                constraints,
            });
            let config_path = st.workspace_root.join(".context-vault.yaml");
            let _ = st.config.save_to_file(&config_path);
            let profile = st.get_active_profile();
            JsonRpcResponse::success(
                req.id,
                json!({
                    "success": true,
                    "profile": profile,
                    "conflicting_constraints": !is_valid,
                    "conflict_reasons": conflict_reasons,
                }),
            )
        }
        "profile/getCustomConfig" => {
            let st = state.read().await;
            let custom = st.config.custom_profile.clone().unwrap_or_default();
            let tier1 = custom.tier1.unwrap_or_default();
            let tier2 = custom.tier2.unwrap_or_else(|| {
                let compiler = crate::evolution::ProfileCompiler::new();
                compiler.map_tier1_to_tier2(&tier1)
            });
            let constraints = if custom.constraints.is_empty() {
                crate::evolution::ConflictDetector::builtin_constraints()
            } else {
                custom.constraints
            };
            JsonRpcResponse::success(
                req.id,
                json!({
                    "tier1": tier1,
                    "tier2": tier2,
                    "constraints": constraints,
                }),
            )
        }
        "profile/updateCustomConfig" => {
            let params = match req.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(req.id, -32602, "Missing request parameters")
                }
            };
            let tier1 = params.get("tier1").and_then(|v| {
                serde_json::from_value::<crate::evolution::Tier1Objectives>(v.clone()).ok()
            });
            let tier2 = params.get("tier2").and_then(|v| {
                serde_json::from_value::<crate::evolution::Tier2Parameters>(v.clone()).ok()
            });
            let constraints = params
                .get("constraints")
                .and_then(|v| {
                    serde_json::from_value::<Vec<crate::evolution::ConstraintRule>>(v.clone()).ok()
                })
                .unwrap_or_default();

            let mut st = state.write().await;
            st.config.custom_profile = Some(crate::config::CustomProfileConfig {
                tier1,
                tier2,
                constraints,
            });
            let config_path = st.workspace_root.join(".context-vault.yaml");
            let _ = st.config.save_to_file(&config_path);
            let profile = st.get_active_profile();
            JsonRpcResponse::success(
                req.id,
                json!({
                    "success": true,
                    "profile": profile,
                }),
            )
        }
        "profile/validateConstraints" => {
            let constraints = match req.params.as_ref().and_then(|p| {
                p.get("constraints").and_then(|v| {
                    serde_json::from_value::<Vec<crate::evolution::ConstraintRule>>(v.clone()).ok()
                })
            }) {
                Some(c) => c,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing or invalid 'constraints' parameter",
                    )
                }
            };
            let detector = crate::evolution::ConflictDetector::new();
            let (is_valid, conflicts) = detector.validate(&constraints);
            JsonRpcResponse::success(
                req.id,
                json!({
                    "is_valid": is_valid,
                    "conflicts": conflicts,
                }),
            )
        }
        "analytics/getStructuralProblems" => {
            let file_path = req.params.as_ref().and_then(|p| {
                p.get("file_path")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            });
            let st = state.read().await;
            let target = st.get_active_profile().fragment_size_target;

            let nodes: Vec<crate::model::NoteNode> = if let Some(ref fp) = file_path {
                st.topology
                    .nodes
                    .values()
                    .filter(|n| n.file_path == *fp || n.id == *fp)
                    .cloned()
                    .collect()
            } else {
                st.topology.nodes.values().cloned().collect()
            };

            let profile = st.get_active_profile();
            let signatures = crate::evolution::detect_all_signatures(&nodes, target);
            let mut sig_values = Vec::new();
            for sig in &signatures {
                let mut v = serde_json::to_value(sig).unwrap_or(json!({}));
                let strat_set = crate::evolution::strategies::select_strategy_set(
                    &sig.signature_id,
                    &sig.document_path,
                    sig,
                    &profile,
                );
                v["strategy_set"] = json!(strat_set);
                v["strategySet"] = json!(strat_set);
                sig_values.push(v);
            }
            JsonRpcResponse::success(req.id, json!({ "signatures": sig_values }))
        }
        "recommendation/getRecommendations" => {
            let signature_id = req.params.as_ref().and_then(|p| {
                p.get("signature_id")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            });
            let st = state.read().await;
            let profile = st.get_active_profile();
            let target = profile.fragment_size_target;

            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            let signatures = crate::evolution::detect_all_signatures(&nodes, target);

            let mut recommendations = Vec::new();
            for sig in signatures {
                if let Some(ref target_id) = signature_id {
                    if sig.signature_id != *target_id {
                        continue;
                    }
                }
                let recs = crate::evolution::evaluate_recommendations_for_signature(&sig, &profile);
                recommendations.extend(recs);
            }

            JsonRpcResponse::success(req.id, json!({ "recommendations": recommendations }))
        }
        "transformation/generatePlan" => {
            let recommendation_id = match req.params.as_ref().and_then(|p| {
                p.get("recommendation_id")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(id) => id,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing 'recommendation_id' parameter",
                    )
                }
            };
            let st = state.read().await;
            let profile = st.get_active_profile();
            let target = profile.fragment_size_target;

            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            let signatures = crate::evolution::detect_all_signatures(&nodes, target);

            let mut found = None;
            for sig in &signatures {
                let recs = crate::evolution::evaluate_recommendations_for_signature(sig, &profile);
                for rec in recs {
                    if rec.recommendation_id == recommendation_id {
                        let node = nodes
                            .iter()
                            .find(|n| n.file_path == sig.document_path || n.id == sig.document_path)
                            .cloned()
                            .unwrap_or_else(|| {
                                crate::model::NoteNode::new(
                                    sig.document_path.clone(),
                                    st.workspace_root
                                        .join(&sig.document_path)
                                        .to_string_lossy()
                                        .to_string(),
                                    "Document",
                                )
                            });
                        found = Some((rec, node));
                        break;
                    }
                }
                if found.is_some() {
                    break;
                }
            }

            let plan = match found {
                Some((rec, node)) => crate::evolution::TransformationEngine::build_plan(
                    &rec,
                    &node,
                    &st.workspace_root,
                ),
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32004,
                        format!("Recommendation '{}' not found", recommendation_id),
                    );
                }
            };
            drop(st);

            let mut st_write = state.write().await;
            st_write
                .cached_plans
                .insert(plan.plan_id.clone(), plan.clone());

            JsonRpcResponse::success(req.id, json!(plan))
        }
        "transformation/applyPlan" => {
            let plan_id = match req.params.as_ref().and_then(|p| {
                p.get("plan_id")
                    .or_else(|| p.get("planId"))
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(id) => id,
                None => {
                    return JsonRpcResponse::error(req.id, -32602, "Missing 'plan_id' parameter")
                }
            };

            let st = state.read().await;
            let plan_opt = if let Some(plan_val) = req.params.as_ref().and_then(|p| p.get("plan")) {
                match serde_json::from_value::<crate::evolution::TransformationPlan>(plan_val.clone()) {
                    Ok(p) => Some(p),
                    Err(e) => {
                        eprintln!("[transformation/applyPlan] plan deserialization fallback: {}", e);
                        st.cached_plans
                            .get(&plan_id)
                            .or_else(|| st.cached_plans.get(plan_id.strip_prefix("plan_").unwrap_or(&plan_id)))
                            .or_else(|| st.cached_plans.get(&format!("plan_{}", plan_id)))
                            .cloned()
                    }
                }
            } else {
                st.cached_plans
                    .get(&plan_id)
                    .or_else(|| st.cached_plans.get(plan_id.strip_prefix("plan_").unwrap_or(&plan_id)))
                    .or_else(|| st.cached_plans.get(&format!("plan_{}", plan_id)))
                    .cloned()
            };

            let root = st.workspace_root.clone();
            let plan = match plan_opt {
                Some(p) => p,
                None => {
                    // Resilient dynamic on-the-fly reconstruction
                    let opp_id = plan_id.strip_prefix("plan_").unwrap_or(&plan_id);
                    let profile = st.get_active_profile();
                    let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
                    if let Some(opp) = crate::evolution::opportunity::build_opportunity_detail(opp_id, &nodes, &profile) {
                        if let Some(node) = nodes.iter().find(|n| n.file_path == opp.target_document || n.id == opp.target_document) {
                            let strats: Vec<crate::evolution::strategies::KnowledgeOrganizationStrategy> = if let Some(ref set) = opp.strategy_set {
                                let mut s = vec![set.primary.strategy];
                                for sup in &set.supporting {
                                    s.push(sup.strategy);
                                }
                                if s.is_empty() {
                                    opp.candidate_strategies.iter().map(|cs| cs.strategy.into()).collect()
                                } else {
                                    s
                                }
                            } else {
                                opp.candidate_strategies.iter().map(|cs| cs.strategy.into()).collect()
                            };
                            crate::evolution::TransformationEngine::build_multi_strategy_plan(
                                &plan_id,
                                &format!("rec_{}", opp.id),
                                &strats,
                                node,
                                &root,
                            )
                        } else {
                            return JsonRpcResponse::error(
                                req.id,
                                -32004,
                                format!("Transformation plan '{}' not found", plan_id),
                            );
                        }
                    } else {
                        return JsonRpcResponse::error(
                            req.id,
                            -32004,
                            format!("Transformation plan '{}' not found", plan_id),
                        );
                    }
                }
            };
            drop(st);

            // Invariant XVI: Require approval must be satisfied
            if let Err(e) = crate::evolution::TransformationEngine::verify_invariant_xvi(&plan) {
                return JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Invariant XVI approval gate check failed: {}", e),
                );
            }

            match crate::evolution::TransformationEngine::apply_plan_atomic(&plan, &root) {
                Ok(actual_delta) => JsonRpcResponse::success(
                    req.id,
                    json!({
                        "success": true,
                        "actual_delta": actual_delta,
                    }),
                ),
                Err(e) => JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Atomic transformation failed and rolled back: {}", e),
                ),
            }
        }
        "evolution/getOpportunities" => {
            let limit = req
                .params
                .as_ref()
                .and_then(|p| p.get("limit").and_then(|v| v.as_u64()))
                .unwrap_or(50) as usize;

            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            let mut opportunities =
                crate::evolution::opportunity::build_structural_opportunities(&nodes, &profile);
            opportunities.truncate(limit);
            JsonRpcResponse::success(req.id, json!({ "opportunities": opportunities }))
        }
        "evolution/getOpportunityDetail" => {
            let opp_id = match req.params.as_ref().and_then(|p| {
                p.get("opportunityId")
                    .or_else(|| p.get("opportunity_id"))
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(id) => id,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing 'opportunityId' parameter",
                    )
                }
            };
            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            match crate::evolution::opportunity::build_opportunity_detail(&opp_id, &nodes, &profile) {
                Some(opp) => JsonRpcResponse::success(req.id, json!(opp)),
                None => JsonRpcResponse::error(
                    req.id,
                    -32004,
                    format!("Opportunity '{}' not found", opp_id),
                ),
            }
        }
        "evolution/previewTransformation" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let opp_id = match params
                .get("opportunityId")
                .or_else(|| params.get("opportunity_id"))
                .and_then(|v| v.as_str())
            {
                Some(id) => id.to_string(),
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'opportunityId' parameter"),
            };
            let strategy = match params.get("strategy").and_then(|v| {
                serde_json::from_value::<crate::evolution::opportunity::EvolutionStrategy>(v.clone()).ok()
            }) {
                Some(s) => s,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing or invalid 'strategy' parameter"),
            };
            let hybrid_partner = params.get("hybrid_partner").and_then(|v| {
                serde_json::from_value::<crate::evolution::opportunity::EvolutionStrategy>(v.clone()).ok()
            });

            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            let root = st.workspace_root.clone();
            drop(st);

            match crate::evolution::opportunity::build_transformation_preview_and_plan(
                &opp_id,
                strategy,
                hybrid_partner,
                &nodes,
                &root,
                &profile,
            ) {
                Some((preview, plan)) => {
                    let mut st_write = state.write().await;
                    st_write.cached_plans.insert(plan.plan_id.clone(), plan);
                    JsonRpcResponse::success(req.id, json!(preview))
                }
                None => JsonRpcResponse::error(
                    req.id,
                    -32004,
                    format!("Could not generate preview for opportunity '{}'", opp_id),
                ),
            }
        }
        "evolution/applyTransformation" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let plan_id = match params
                .get("plan_id")
                .or_else(|| params.get("planId"))
                .and_then(|v| v.as_str())
            {
                Some(id) => id.to_string(),
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'plan_id' parameter"),
            };
            let require_approval = params
                .get("requireApproval")
                .or_else(|| params.get("require_approval"))
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            // Invariant XVI safety gate: require explicit approval
            if !require_approval {
                return JsonRpcResponse::error(
                    req.id,
                    -32000,
                    "Invariant XVI violation: User approval is required for transformations",
                );
            }

            let st = state.read().await;
            let plan_opt = if let Some(plan_val) = req.params.as_ref().and_then(|p| p.get("plan")) {
                match serde_json::from_value::<crate::evolution::TransformationPlan>(plan_val.clone()) {
                    Ok(p) => Some(p),
                    Err(e) => {
                        eprintln!("[evolution/applyTransformation] plan deserialization fallback: {}", e);
                        st.cached_plans
                            .get(&plan_id)
                            .or_else(|| st.cached_plans.get(plan_id.strip_prefix("plan_").unwrap_or(&plan_id)))
                            .or_else(|| st.cached_plans.get(&format!("plan_{}", plan_id)))
                            .cloned()
                    }
                }
            } else {
                st.cached_plans
                    .get(&plan_id)
                    .or_else(|| st.cached_plans.get(plan_id.strip_prefix("plan_").unwrap_or(&plan_id)))
                    .or_else(|| st.cached_plans.get(&format!("plan_{}", plan_id)))
                    .cloned()
            };
            let root = st.workspace_root.clone();

            let plan = match plan_opt {
                Some(p) => p,
                None => {
                    let opp_id = plan_id.strip_prefix("plan_").unwrap_or(&plan_id);
                    let profile = st.get_active_profile();
                    let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
                    if let Some(opp) = crate::evolution::opportunity::build_opportunity_detail(opp_id, &nodes, &profile) {
                        if let Some(node) = nodes.iter().find(|n| n.file_path == opp.target_document || n.id == opp.target_document) {
                            let strats: Vec<crate::evolution::strategies::KnowledgeOrganizationStrategy> = if let Some(ref set) = opp.strategy_set {
                                let mut s = vec![set.primary.strategy];
                                for sup in &set.supporting {
                                    s.push(sup.strategy);
                                }
                                if s.is_empty() {
                                    opp.candidate_strategies.iter().map(|cs| cs.strategy.into()).collect()
                                } else {
                                    s
                                }
                            } else {
                                opp.candidate_strategies.iter().map(|cs| cs.strategy.into()).collect()
                            };
                            crate::evolution::TransformationEngine::build_multi_strategy_plan(
                                &plan_id,
                                &format!("rec_{}", opp.id),
                                &strats,
                                node,
                                &root,
                            )
                        } else {
                            return JsonRpcResponse::error(
                                req.id,
                                -32004,
                                format!("Transformation plan '{}' not found", plan_id),
                            );
                        }
                    } else {
                        return JsonRpcResponse::error(
                            req.id,
                            -32004,
                            format!("Transformation plan '{}' not found", plan_id),
                        );
                    }
                }
            };
            drop(st);

            // Invariant XVI: Require approval must be satisfied
            if let Err(e) = crate::evolution::TransformationEngine::verify_invariant_xvi(&plan) {
                return JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Invariant XVI approval gate check failed: {}", e),
                );
            }

            match crate::evolution::TransformationEngine::apply_plan_atomic(&plan, &root) {
                Ok(actual_delta) => {
                    let re_measurement = crate::evolution::opportunity::ReMeasurementReport {
                        context_efficiency_delta_pct: (actual_delta.efficiency_delta as f64) * 100.0,
                        trace_depth_delta_pct: (actual_delta.trace_depth_delta as f64) * 100.0,
                        maintenance_coupling_delta_pct: (actual_delta.coupling_delta as f64) * 100.0,
                        document_count_delta: actual_delta.doc_count_delta,
                        reference_count_delta: 0,
                        narrative_summary: "The selected objectives improved, with the following structural trade-offs.".to_string(),
                    };
                    JsonRpcResponse::success(
                        req.id,
                        json!({
                            "success": true,
                            "reMeasurement": re_measurement,
                        }),
                    )
                }
                Err(e) => JsonRpcResponse::error(
                    req.id,
                    -32000,
                    format!("Atomic transformation failed: {}", e),
                ),
            }
        }
        "evolution/getStrategySet" => {
            let opp_id = match req.params.as_ref().and_then(|p| {
                p.get("opportunity_id")
                    .or_else(|| p.get("opportunityId"))
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(id) => id,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing 'opportunity_id' parameter",
                    );
                }
            };
            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            match crate::evolution::opportunity::build_opportunity_strategy_set(&opp_id, &nodes, &profile) {
                Some(strategy_set) => JsonRpcResponse::success(req.id, json!(strategy_set)),
                None => JsonRpcResponse::error(
                    req.id,
                    -32004,
                    format!("Opportunity '{}' not found", opp_id),
                ),
            }
        }
        "evolution/validateStrategySelection" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let selected_strategies = match params
                .get("selected_strategies")
                .or_else(|| params.get("selectedStrategies"))
                .and_then(|v| {
                    serde_json::from_value::<Vec<crate::evolution::strategies::KnowledgeOrganizationStrategy>>(
                        v.clone(),
                    )
                    .ok()
                }) {
                Some(strats) => strats,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing or invalid 'selected_strategies' parameter",
                    );
                }
            };

            let mut incompatible_pairs = Vec::new();
            for i in 0..selected_strategies.len() {
                for j in (i + 1)..selected_strategies.len() {
                    let a = selected_strategies[i];
                    let b = selected_strategies[j];
                    let rel = crate::evolution::strategies::get_strategy_relationship(a, b);
                    if rel == crate::evolution::strategies::StrategyRelationshipType::Alternative
                        || rel == crate::evolution::strategies::StrategyRelationshipType::MutuallyExclusive
                    {
                        incompatible_pairs.push(json!({
                            "strategy_a": a,
                            "strategy_b": b,
                            "relationship": rel,
                            "reason": format!(
                                "Strategy '{:?}' cannot be combined with '{:?}' (relationship: {:?}).",
                                a, b, rel
                            ),
                        }));
                    }
                }
            }

            JsonRpcResponse::success(
                req.id,
                json!({
                    "is_valid": incompatible_pairs.is_empty(),
                    "incompatible_pairs": incompatible_pairs,
                }),
            )
        }
        "evolution/calculatePlanValue" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let opp_id = match params
                .get("opportunity_id")
                .or_else(|| params.get("opportunityId"))
                .and_then(|v| v.as_str())
            {
                Some(id) => id.to_string(),
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'opportunity_id' parameter"),
            };
            let selected_strategies = match params
                .get("selected_strategies")
                .or_else(|| params.get("selectedStrategies"))
                .and_then(|v| {
                    serde_json::from_value::<Vec<crate::evolution::strategies::KnowledgeOrganizationStrategy>>(
                        v.clone(),
                    )
                    .ok()
                }) {
                Some(strats) => strats,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing or invalid 'selected_strategies' parameter",
                    );
                }
            };

            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();

            let signatures = crate::evolution::signatures::detect_all_signatures(
                &nodes,
                profile.fragment_size_target,
            );
            let sig = match signatures.iter().find(|s| s.signature_id == opp_id) {
                Some(s) => s.clone(),
                None => {
                    match crate::evolution::opportunity::build_opportunity_detail(&opp_id, &nodes, &profile) {
                        Some(opp) => crate::evolution::signatures::StructuralProblemSignature {
                            signature_id: opp.id,
                            document_path: opp.target_document,
                            problem_type: match opp.signature {
                                crate::evolution::opportunity::ProblemSignature::StructurallyOverloadedDocument => {
                                    crate::evolution::signatures::ProblemSignatureType::GodDocument
                                }
                                crate::evolution::opportunity::ProblemSignature::KnowledgeFragmentation => {
                                    crate::evolution::signatures::ProblemSignatureType::FragmentedConcept
                                }
                                crate::evolution::opportunity::ProblemSignature::WeakDiscoveryPath => {
                                    crate::evolution::signatures::ProblemSignatureType::ImplicitRouter
                                }
                                crate::evolution::opportunity::ProblemSignature::IsolatedKnowledgeSilo => {
                                    crate::evolution::signatures::ProblemSignatureType::IsolatedSilo
                                }
                            },
                            confidence: opp.confidence as f32,
                            evidence: Default::default(),
                            rationale: opp.narrative_why,
                        },
                        None => {
                            return JsonRpcResponse::error(
                                req.id,
                                -32004,
                                format!("Opportunity '{}' not found", opp_id),
                            );
                        }
                    }
                }
            };

            let fit_lookup: Vec<(crate::evolution::strategies::KnowledgeOrganizationStrategy, u32)> =
                crate::evolution::strategies::STANDARD_CANDIDATE_STRATEGIES
                    .iter()
                    .map(|&strat| {
                        (
                            strat,
                            crate::evolution::strategies::calculate_integer_fit_score(
                                strat, &sig, &profile,
                            ),
                        )
                    })
                    .collect();

            let composite_plan_value = crate::evolution::plan_value::calculate_composite_plan_value(
                &selected_strategies,
                &fit_lookup,
            );

            let mut conflict_message = None;
            let mut is_valid = true;
            for i in 0..selected_strategies.len() {
                for j in (i + 1)..selected_strategies.len() {
                    let a = selected_strategies[i];
                    let b = selected_strategies[j];
                    let rel = crate::evolution::strategies::get_strategy_relationship(a, b);
                    if rel != crate::evolution::strategies::StrategyRelationshipType::Complementary {
                        is_valid = false;
                        conflict_message = Some(format!(
                            "Strategies '{:?}' and '{:?}' are incompatible ({:?}).",
                            a, b, rel
                        ));
                        break;
                    }
                }
                if !is_valid {
                    break;
                }
            }

            JsonRpcResponse::success(
                req.id,
                json!({
                    "composite_plan_value": composite_plan_value,
                    "is_valid": is_valid,
                    "conflict_message": conflict_message,
                }),
            )
        }
        "evolution/generatePlan" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let opp_id = match params
                .get("opportunity_id")
                .or_else(|| params.get("opportunityId"))
                .and_then(|v| v.as_str())
            {
                Some(id) => id.to_string(),
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'opportunity_id' parameter"),
            };
            let selected_strategies = match params
                .get("selected_strategies")
                .or_else(|| params.get("selectedStrategies"))
                .and_then(|v| {
                    serde_json::from_value::<Vec<crate::evolution::strategies::KnowledgeOrganizationStrategy>>(
                        v.clone(),
                    )
                    .ok()
                }) {
                Some(strats) => strats,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing or invalid 'selected_strategies' parameter",
                    );
                }
            };

            let st = state.read().await;
            let profile = st.get_active_profile();
            let nodes: Vec<crate::model::NoteNode> = st.topology.nodes.values().cloned().collect();
            let root = st.workspace_root.clone();
            drop(st);

            match crate::evolution::opportunity::build_multi_strategy_transformation_preview_and_plan(
                &opp_id,
                &selected_strategies,
                &nodes,
                &root,
                &profile,
            ) {
                Some((_preview, plan)) => {
                    let mut st_write = state.write().await;
                    st_write.cached_plans.insert(plan.plan_id.clone(), plan.clone());
                    JsonRpcResponse::success(
                        req.id,
                        json!({
                            "plan_id": plan.plan_id,
                            "planId": plan.plan_id,
                            "opportunity_id": opp_id,
                            "opportunityId": opp_id,
                            "recommendation_id": plan.recommendation_id,
                            "recommendationId": plan.recommendation_id,
                            "actions": plan.actions,
                            "diff_preview": plan.diff_preview,
                            "diffPreview": plan.diff_preview,
                            "projected_delta": plan.projected_delta,
                            "projectedDelta": plan.projected_delta,
                            "requires_approval": plan.requires_approval,
                            "requiresApproval": plan.requires_approval,
                        }),
                    )
                }
                None => JsonRpcResponse::error(
                    req.id,
                    -32004,
                    format!("Could not generate plan for opportunity '{}'", opp_id),
                ),
            }
        }
        "governance/getHarnessStatus" => {
            let st = state.read().await;
            let root = st.workspace_root.clone();
            let mut custom_paths = st.config.get_harness_entry_points();
            if let Some(ref hub) = st.active_hub {
                if !custom_paths.contains(hub) {
                    custom_paths.push(hub.clone());
                }
            }
            for hub in &st.config.vault.hubs {
                if !custom_paths.contains(hub) {
                    custom_paths.push(hub.clone());
                }
            }
            let active_override = st.active_harness.clone().or_else(|| st.config.get_active_harness());
            drop(st);

            let mut harnesses =
                crate::harness::discovery::HarnessDetector::detect_all_with_active(&root, &custom_paths, active_override.as_deref());
            let parser = crate::harness::index_parser::IndexParser::new(&root);
            let sec_detector =
                crate::harness::secondary_layer::SecondaryLayerDetector::new(&root);

            for h in harnesses.iter_mut() {
                let full = root.join(&h.file_path);
                if let Ok(content) = std::fs::read_to_string(&full) {
                    h.embedded_index = parser.parse_harness_index(&h.file_path, &content);
                    h.secondary_layers_detected =
                        sec_detector.detect_secondary_layers(&h.file_path);
                    h.quality_ratings =
                        crate::harness::HarnessQualityAuditor::audit_harness(h, &content);
                }
            }

            let active_harness = harnesses
                .iter()
                .find(|h| h.is_primary)
                .cloned()
                .or_else(|| harnesses.first().cloned());

            let skills = crate::harness::skill::SkillAuditor::audit_repository(&root, &[], &[]);
            let procedure_candidates =
                crate::harness::procedure::ProcedureExtractor::extract_candidates_from_vault(&root, &[]);
            let capability_matrix =
                crate::harness::matrix::AIOrientedCapabilityMatrixEvaluator::evaluate(
                    active_harness.as_ref(),
                    &skills,
                    &procedure_candidates,
                    &[],
                );

            JsonRpcResponse::success(
                req.id,
                json!({
                    "harnesses": harnesses,
                    "active_harness": active_harness,
                    "capability_matrix": capability_matrix
                }),
            )
        }
        "governance/setActiveHarness" => {
            let harness_file = match req.params {
                Some(p) => match p.get("harness_file").and_then(|v| v.as_str()) {
                    Some(s) => s.to_string(),
                    None => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            "Missing required parameter 'harness_file'".to_string(),
                        );
                    }
                },
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing required parameter 'harness_file'".to_string(),
                    );
                }
            };

            let mut st = state.write().await;
            let previous = st.active_harness.clone();
            st.active_harness = Some(harness_file.clone());

            // Persist to .context-vault.yaml
            let root = st.workspace_root.clone();
            let mut config = crate::config::ContextVaultConfig::load_from_workspace(&root);
            let mut harness_cfg = config.harness.unwrap_or_default();
            harness_cfg.active = Some(harness_file.clone());
            if !harness_cfg.entry_points.contains(&harness_file) {
                harness_cfg.entry_points.push(harness_file.clone());
            }
            config.harness = Some(harness_cfg);
            let _ = config.save_to_workspace(&root);
            st.config = config;

            JsonRpcResponse::success(
                req.id,
                json!({
                    "active_harness": harness_file,
                    "previous_harness": previous
                }),
            )
        }
        "governance/initializeHarness" => {
            let (target_harness, project_name) = match req.params {
                Some(p) => (
                    p.get("target_harness").and_then(|v| v.as_str()).unwrap_or("AGENTS.md").to_string(),
                    p.get("project_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                ),
                None => ("AGENTS.md".to_string(), None),
            };

            let st = state.read().await;
            let root = st.workspace_root.clone();
            drop(st);

            let target_path = root.join(&target_harness);
            if !target_path.exists() {
                let mut custom_routes = Vec::new();
                if root.join("AGENTS.md").exists() {
                    if let Ok(c) = std::fs::read_to_string(root.join("AGENTS.md")) {
                        custom_routes = crate::evolution::transformation::extract_routes_from_markdown(&c);
                    }
                } else if root.join("index.md").exists() {
                    if let Ok(c) = std::fs::read_to_string(root.join("index.md")) {
                        custom_routes = crate::evolution::transformation::extract_routes_from_markdown(&c);
                    }
                }

                let harness_type = crate::harness::HarnessType::from_path(&target_harness);
                let content = crate::harness::discovery::generate_template_harness(
                    harness_type,
                    project_name.as_deref(),
                    if custom_routes.is_empty() { None } else { Some(&custom_routes) },
                );
                let _ = std::fs::write(&target_path, content);
            }

            JsonRpcResponse::success(req.id, json!({ "initialized": target_harness }))
        }
        "governance/configureHarnessBase" => {
            let params = match req.params.as_ref() {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing request parameters"),
            };
            let enabled_harnesses: Vec<String> = params
                .get("enabled_harnesses")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|s| s.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_else(|| vec!["AGENTS.md".to_string()]);
            let active_harness = params
                .get("active_harness")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| enabled_harnesses.first().map(|s| s.as_str()).unwrap_or("AGENTS.md"))
                .to_string();
            let create_folders: Vec<String> = params
                .get("create_folders")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|s| s.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            let consolidate_index = params
                .get("consolidate_index")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let clean_unused = params
                .get("clean_unused")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let mut st = state.write().await;
            let root = st.workspace_root.clone();

            // 1. Create requested base folders
            let mut created_dirs = Vec::new();
            for folder in &create_folders {
                let dir_path = root.join(folder);
                if !dir_path.exists() {
                    let _ = std::fs::create_dir_all(&dir_path);
                    created_dirs.push(folder.clone());
                }
            }

            // 2. Extract best routes from active harness or index.md
            let mut routes = Vec::new();
            if root.join(&active_harness).exists() {
                if let Ok(c) = std::fs::read_to_string(root.join(&active_harness)) {
                    routes = crate::evolution::transformation::extract_routes_from_markdown(&c);
                }
            }
            if routes.is_empty() && root.join("AGENTS.md").exists() {
                if let Ok(c) = std::fs::read_to_string(root.join("AGENTS.md")) {
                    routes = crate::evolution::transformation::extract_routes_from_markdown(&c);
                }
            }
            if routes.is_empty() && root.join("index.md").exists() {
                if let Ok(c) = std::fs::read_to_string(root.join("index.md")) {
                    routes = crate::evolution::transformation::extract_routes_from_markdown(&c);
                }
            }

            // 3. For enabled harnesses, ensure they exist with index
            for h in &enabled_harnesses {
                let h_path = root.join(h);
                if !h_path.exists() {
                    let h_type = crate::harness::HarnessType::from_path(h);
                    let content = crate::harness::discovery::generate_template_harness(
                        h_type,
                        None,
                        if routes.is_empty() { None } else { Some(&routes) },
                    );
                    let _ = std::fs::write(&h_path, content);
                }
            }

            // 4. For standard harnesses that are NOT enabled, remove them if clean_unused is true
            let mut removed = Vec::new();
            if clean_unused {
                let standard_harnesses = ["AGENTS.md", "CLAUDE.md", "GEMINI.md", ".github/copilot-instructions.md"];
                for sh in standard_harnesses {
                    if !enabled_harnesses.iter().any(|eh| eh == sh) {
                        let sh_path = root.join(sh);
                        if sh_path.is_file() {
                            let _ = std::fs::remove_file(&sh_path);
                            removed.push(sh.to_string());
                        }
                    }
                }
            }

            // 5. If consolidate_index is true, consolidate and safely remove index.md
            if consolidate_index {
                let index_path = root.join("index.md");
                if index_path.exists() {
                    for h_rel in &enabled_harnesses {
                        let h_path = root.join(h_rel);
                        if !h_path.exists() {
                            continue;
                        }
                        let h_audit = crate::evolution::transformation::audit_secondary_layer_routes(
                            &root,
                            "index.md",
                            h_rel,
                        );
                        if !h_audit.missing_from_harness.is_empty() {
                            let mut h_content = std::fs::read_to_string(&h_path).unwrap_or_default();
                            let mut routes_to_append = String::new();
                            for r in &h_audit.missing_from_harness {
                                routes_to_append.push_str(r);
                                routes_to_append.push('\n');
                            }
                            if h_content.contains("## Knowledge Index") {
                                if let Some(pos) = h_content.find("## Knowledge Index") {
                                    let end_of_line = h_content[pos..]
                                        .find('\n')
                                        .map(|p| pos + p + 1)
                                        .unwrap_or(h_content.len());
                                    h_content.insert_str(
                                        end_of_line,
                                        &format!("\n{}", routes_to_append.trim_start()),
                                    );
                                } else {
                                    h_content.push_str(&format!(
                                        "\n\n## Knowledge Index\n{}",
                                        routes_to_append.trim_start()
                                    ));
                                }
                            } else {
                                h_content.push_str(&format!(
                                    "\n\n## Knowledge Index\n{}",
                                    routes_to_append.trim_start()
                                ));
                            }
                            let _ = std::fs::write(&h_path, h_content);
                        }
                    }

                    // Clean references to index.md in enabled harnesses
                    for h_rel in &enabled_harnesses {
                        let h_path = root.join(h_rel);
                        if let Ok(h_content) = std::fs::read_to_string(&h_path) {
                            if h_content.contains("(index.md)") || h_content.contains("[[index.md]]") || h_content.contains("[[index]]") {
                                let cleaned = h_content
                                    .lines()
                                    .filter(|l| !l.contains("(index.md)") && !l.contains("[[index.md]]") && !l.contains("[[index]]"))
                                    .collect::<Vec<_>>()
                                    .join("\n") + "\n";
                                let _ = std::fs::write(&h_path, cleaned);
                            }
                        }
                    }

                    // Safely delete index.md
                    let _ = std::fs::remove_file(&index_path);
                    removed.push("index.md".to_string());
                }
            }

            // 6. Update .context-vault.yaml
            let mut config = crate::config::ContextVaultConfig::load_from_workspace(&root);
            let mut harness_cfg = config.harness.unwrap_or_default();
            harness_cfg.active = Some(active_harness.clone());
            harness_cfg.entry_points = enabled_harnesses.clone();
            config.harness = Some(harness_cfg);
            config.vault.hubs = vec![active_harness.clone()];
            let _ = config.save_to_workspace(&root);
            st.config = config;
            st.active_harness = Some(active_harness.clone());

            JsonRpcResponse::success(
                req.id,
                json!({
                    "active_harness": active_harness,
                    "enabled_harnesses": enabled_harnesses,
                    "created_folders": created_dirs,
                    "removed_files": removed,
                    "consolidated_index": consolidate_index
                }),
            )
        }
        "governance/syncHarnesses" => {
            let st = state.read().await;
            let root = st.workspace_root.clone();
            let custom_paths = st.config.get_harness_entry_points();
            let active_override = st.active_harness.clone().or_else(|| st.config.get_active_harness());
            let standard_harness_names = [
                "AGENTS.md",
                "CLAUDE.md",
                "GEMINI.md",
                ".github/copilot-instructions.md",
            ];

            let mut all_harness_paths: Vec<String> = Vec::new();
            for std_cand in &standard_harness_names {
                if root.join(std_cand).is_file() {
                    all_harness_paths.push(std_cand.to_string());
                }
            }
            for c in &custom_paths {
                if root.join(c).is_file() && !all_harness_paths.contains(c) {
                    all_harness_paths.push(c.clone());
                }
            }
            for node_path in st.topology.nodes.keys() {
                let normalized = node_path.replace('\\', "/");
                if standard_harness_names
                    .iter()
                    .any(|h| normalized == *h || normalized.ends_with(&format!("/{}", h)))
                    && !all_harness_paths.contains(node_path)
                {
                    all_harness_paths.push(node_path.clone());
                }
            }
            drop(st);

            use std::collections::HashMap;
            let mut scope_groups: HashMap<String, Vec<String>> = HashMap::new();
            for h_path in all_harness_paths {
                let scope = crate::quality::structure::get_harness_dir_scope(&h_path);
                scope_groups.entry(scope).or_default().push(h_path);
            }

            let parser = crate::harness::index_parser::IndexParser::new(&root);
            let mut synced_count = 0;

            for (_scope, peer_paths) in scope_groups {
                if peer_paths.len() <= 1 {
                    continue; // Single isolated harness in this scope has no peers to sync
                }

                // Determine primary harness for this scope
                let preferred_primary = peer_paths
                    .iter()
                    .find(|p| active_override.as_deref() == Some(p.as_str()))
                    .or_else(|| {
                        peer_paths
                            .iter()
                            .find(|p| p.ends_with("AGENTS.md") || p.as_str() == "AGENTS.md")
                    })
                    .or_else(|| peer_paths.first());

                let mut effective_primary: Option<(String, String, crate::harness::EmbeddedIndexMetadata)> = None;

                if let Some(pri_rel) = preferred_primary {
                    let pri_full = root.join(pri_rel);
                    if let Ok(pri_content) = std::fs::read_to_string(&pri_full) {
                        if let Some(idx) = parser.parse_harness_index(pri_rel, &pri_content) {
                            effective_primary = Some((pri_rel.clone(), pri_content, idx));
                        }
                    }
                }

                if effective_primary.is_none() {
                    for peer_rel in &peer_paths {
                        let peer_full = root.join(peer_rel);
                        if let Ok(peer_content) = std::fs::read_to_string(&peer_full) {
                            if let Some(idx) = parser.parse_harness_index(peer_rel, &peer_content) {
                                effective_primary = Some((peer_rel.clone(), peer_content, idx));
                                break;
                            }
                        }
                    }
                }

                if let Some((pri_rel, pri_content, idx)) = effective_primary {
                    let lines: Vec<&str> = pri_content.lines().collect();
                    let start = idx.start_line.saturating_sub(1);
                    let end = idx.end_line.min(lines.len());
                    let mut index_block = lines[start..end].join("\n");

                    // Merge routes ONLY from other peer harnesses within THIS directory scope
                    for other_rel in &peer_paths {
                        if other_rel != &pri_rel {
                            let other_full = root.join(other_rel);
                            if let Ok(other_content) = std::fs::read_to_string(&other_full) {
                                if let Some(ref oi) = parser.parse_harness_index(other_rel, &other_content) {
                                    for route in &oi.routes {
                                        if !index_block.contains(&route.target_file) {
                                            let route_line = format!("- [{}](%TARGET%)", route.label)
                                                .replace("%TARGET%", &route.target_file);
                                            index_block.push_str(&format!("\n{}", route_line));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Write unified index_block ONLY to harnesses in this directory scope
                    for h_rel in &peer_paths {
                        let h_full = root.join(h_rel);
                        if let Ok(h_content) = std::fs::read_to_string(&h_full) {
                            let h_idx = parser.parse_harness_index(h_rel, &h_content);
                            let new_content = if let Some(ref hi) = h_idx {
                                let h_lines: Vec<&str> = h_content.lines().collect();
                                let h_start = hi.start_line.saturating_sub(1);
                                let h_end = hi.end_line.min(h_lines.len());
                                let mut res = h_lines[..h_start].join("\n");
                                if !res.is_empty() { res.push('\n'); }
                                res.push_str(&index_block);
                                if h_end < h_lines.len() {
                                    res.push('\n');
                                    res.push_str(&h_lines[h_end..].join("\n"));
                                }
                                if h_content.ends_with('\n') && !res.ends_with('\n') {
                                    res.push('\n');
                                }
                                res
                            } else {
                                let mut res = format!("{}\n\n{}", h_content.trim_end(), index_block);
                                if h_content.ends_with('\n') && !res.ends_with('\n') {
                                    res.push('\n');
                                }
                                res
                            };

                            if new_content != h_content {
                                let _ = std::fs::write(&h_full, new_content);
                                synced_count += 1;
                            }
                        }
                    }
                }
            }

            JsonRpcResponse::success(req.id, json!({ "synced_count": synced_count }))
        }
        "governance/getSkillCatalog" => {
            let st = state.read().await;
            let root = st.workspace_root.clone();
            let vault_files: Vec<String> = if !st.topology.nodes.is_empty() {
                st.topology.nodes.keys().cloned().collect()
            } else {
                // If topology not yet populated in RAM, discover markdown files in root
                let mut files = Vec::new();
                for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
                    if entry.file_type().is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "md" {
                                if let Ok(rel) = entry.path().strip_prefix(&root) {
                                    files.push(rel.to_string_lossy().replace('\\', "/"));
                                }
                            }
                        }
                    }
                }
                files
            };
            drop(st);

            // Fetch any routes from detected harnesses
            let harnesses = crate::harness::discovery::HarnessDetector::detect_all(&root, &[]);
            let mut routes = Vec::new();
            let parser = crate::harness::index_parser::IndexParser::new(&root);
            for h in &harnesses {
                let full = root.join(&h.file_path);
                if let Ok(content) = std::fs::read_to_string(&full) {
                    if let Some(idx) = parser.parse_harness_index(&h.file_path, &content) {
                        routes.extend(idx.routes);
                    }
                }
            }

            let skills = crate::harness::skill::SkillAuditor::audit_repository(&root, &routes, &vault_files);
            let procedure_candidates = crate::harness::procedure::ProcedureExtractor::extract_candidates_from_vault(&root, &vault_files);

            JsonRpcResponse::success(
                req.id,
                json!({
                    "skills": skills,
                    "procedure_candidates": procedure_candidates
                }),
            )
        }
        "governance/traceFromHarness" => {
            let (query, harness_file, max_hops) = match req.params {
                Some(ref p) => {
                    let q = p.get("query").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let h = p.get("harness_file").and_then(|v| v.as_str()).unwrap_or("AGENTS.md").to_string();
                    let m = p.get("max_hops").and_then(|v| v.as_u64()).unwrap_or(3) as u32;
                    (q, h, m)
                }
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing required parameter 'query'".to_string(),
                    );
                }
            };

            if query.is_empty() {
                return JsonRpcResponse::error(
                    req.id,
                    -32602,
                    "Parameter 'query' cannot be empty".to_string(),
                );
            }

            let mut st = state.write().await;
            if st.topology.nodes.is_empty() {
                let root = st.workspace_root.clone();
                for entry in walkdir::WalkDir::new(&root).into_iter().flatten() {
                    if entry.file_type().is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "md" {
                                if let Ok(rel) = entry.path().strip_prefix(&root) {
                                    let rel_str = rel.to_string_lossy().replace('\\', "/");
                                    let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
                                    let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0) as u32;
                                    let label = entry.path().file_stem().and_then(|s| s.to_str()).unwrap_or(&rel_str).to_string();
                                    let mut node = crate::model::NoteNode::new(&rel_str, rel_str.clone(), &label);
                                    node.tokens = tokens;
                                    st.topology.add_node(node);
                                }
                            }
                        }
                    }
                }
                st.topology.resolve_links();
            }

            let trace_res = crate::trace::traversal::trace_from_harness(
                &st.topology,
                &query,
                &harness_file,
                max_hops,
            );
            drop(st);

            JsonRpcResponse::success(
                req.id,
                json!({
                    "harness_file": trace_res.harness_file,
                    "total_hops": trace_res.total_hops,
                    "total_tokens_traversed": trace_res.total_tokens_traversed,
                    "target_fragment": trace_res.target_fragment,
                    "path": trace_res.path,
                    "relevance_progression": trace_res.relevance_progression
                }),
            )
        }
        "governance/generateGovernancePlan" => {
            let (strat, target) = match req.params {
                Some(ref p) => {
                    let s_val = p.get("strategy_type").cloned().unwrap_or(serde_json::Value::Null);
                    let strat: crate::harness::GovernanceStrategyType = match serde_json::from_value(s_val) {
                        Ok(s) => s,
                        Err(e) => {
                            return JsonRpcResponse::error(
                                req.id,
                                -32602,
                                format!("Invalid strategy_type: {}", e),
                            );
                        }
                    };
                    let target = p
                        .get("target_file_or_dir")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    (strat, target)
                }
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing params for generateGovernancePlan".to_string(),
                    );
                }
            };

            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };

            match crate::evolution::transformation::GovernancePlanGenerator::generate_plan(
                &root,
                strat,
                &target,
            ) {
                Ok(plan) => {
                    let st = state.read().await;
                    st.governance_store.save_plan(plan.clone());
                    JsonRpcResponse::success(req.id, json!(plan))
                }
                Err(e) => JsonRpcResponse::error(req.id, -32603, e.to_string()),
            }
        }
        "governance/applyGovernancePlan" => {
            let (plan_id, token) = match req.params {
                Some(ref p) => {
                    let pid = p
                        .get("plan_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let tok = p
                        .get("approval_token")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    (pid, tok)
                }
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing params for applyGovernancePlan".to_string(),
                    );
                }
            };

            let (root, store) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.governance_store.clone())
            };

            match store.apply_plan(&root, &plan_id, &token) {
                Ok(result) => JsonRpcResponse::success(req.id, json!(result)),
                Err(e) => JsonRpcResponse::error(req.id, -32001, e.to_string()),
            }
        }
        "governance/analyzeDirectoryStructure" => {
            let (root, topology) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.topology.clone())
            };

            let metrics = crate::harness::directory_metrics::DirectoryMetricsAnalyzer::analyze(
                &root,
                Some(&topology),
            );
            JsonRpcResponse::success(req.id, json!(metrics))
        }
        "trace/getAffectedDocuments" => {
            let (root, config, topology) = {
                let st = state.read().await;
                (st.workspace_root.clone(), st.config.clone(), st.topology.clone())
            };

            let params: super::protocol::GetAffectedDocumentsParams = match req.params {
                Some(p) => match serde_json::from_value(p) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params for trace/getAffectedDocuments: {e}"),
                        );
                    }
                },
                None => super::protocol::GetAffectedDocumentsParams::default(),
            };

            // Derive changed files
            let delta_items = if let Some(paths) = params.changed_paths {
                let mut items = Vec::new();
                for p in paths {
                    let clean = p.trim().replace('\\', "/");
                    if crate::drift::extractor::validate_path_containment(&clean).is_ok() {
                        let status = if root.join(&clean).exists() {
                            crate::delta::updater::FileDeltaStatus::Modified
                        } else {
                            crate::delta::updater::FileDeltaStatus::Deleted
                        };
                        items.push(crate::delta::updater::FileDeltaItem { path: clean, status });
                    }
                }
                items
            } else if let Some(ref range) = params.git_range {
                match crate::delta::updater::parse_git_diff_range(&root, range) {
                    Ok(items) => items,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32000,
                            format!("Failed to parse git diff range: {e}"),
                        );
                    }
                }
            } else {
                match crate::cli::validate::get_staged_git_delta(&root) {
                    Ok(delta) => delta.items,
                    Err(_) => Vec::new(),
                }
            };

            let doc_paths: Vec<String> = if !topology.nodes.is_empty() {
                topology.nodes.keys().cloned().collect()
            } else {
                crate::cli::scan::execute_scan(ScanOptions {
                    workspace_root: root.clone(),
                    hub: None,
                    pretty: false,
                })
                .map(|p| p.nodes.into_iter().map(|n| n.id).collect())
                .unwrap_or_default()
            };

            let reverse_index = crate::drift::detector::build_reverse_impact_index(&root, &doc_paths);
            let mut report = crate::drift::review::calculate_affected_documents(
                &root,
                &delta_items,
                &reverse_index,
                config.review_policy.as_ref(),
                None,
                None,
            );

            if !params.include_unchanged_reviews {
                report.obligations.retain(|o| o.status != crate::model::ReviewStatus::ReviewedUnchanged);
            }

            JsonRpcResponse::success(req.id, json!(report))
        }
        "review/resolveDocumentReview" => {
            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };

            let params: super::protocol::ResolveDocumentReviewParams = match req.params {
                Some(p) => match serde_json::from_value(p) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return JsonRpcResponse::error(
                            req.id,
                            -32602,
                            format!("Invalid params for review/resolveDocumentReview: {e}"),
                        );
                    }
                },
                None => {
                    return JsonRpcResponse::error(req.id, -32602, "Missing params");
                }
            };

            let doc_list = params
                .doc_ids
                .or(params.resolved_reviews)
                .unwrap_or_default();

            if doc_list.is_empty() {
                return JsonRpcResponse::error(req.id, -32602, "Missing doc_ids");
            }

            match crate::drift::review::resolve_document_review(
                &root,
                &doc_list,
                params.commit_hash.as_deref(),
            ) {
                Ok(()) => {
                    let receipt_id = format!(
                        "RECEIPT-{}-{}",
                        params.commit_hash.as_deref().unwrap_or("HEAD"),
                        doc_list.len()
                    );
                    JsonRpcResponse::success(
                        req.id,
                        json!(super::protocol::ResolveDocumentReviewResult {
                            resolved_count: doc_list.len(),
                            receipt_id,
                        }),
                    )
                }
                Err(e) => JsonRpcResponse::error(req.id, -32000, format!("Failed to record review receipts: {e}")),
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipc_profile_orientation_methods() {
        let temp_dir = tempfile::tempdir().unwrap();
        let state = Arc::new(RwLock::new(AppState::new(temp_dir.path().to_path_buf())));

        // 1. Get initial orientation (default: AiOriented)
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "profile/getOrientation".to_string(),
            params: None,
        };
        let res = handle_request(&state, req).await;
        assert!(res.error.is_none());
        let result = res.result.unwrap();
        assert_eq!(result["orientation"], "ai_oriented");
        assert_eq!(result["profile"]["fragment_size_target"], 3000);

        // 2. Set orientation to balanced
        let req2 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "profile/setOrientation".to_string(),
            params: Some(json!({ "orientation": "balanced" })),
        };
        let res2 = handle_request(&state, req2).await;
        assert!(res2.error.is_none());
        let result2 = res2.result.unwrap();
        assert_eq!(result2["success"], true);
        assert_eq!(result2["orientation"], "balanced");
        assert_eq!(result2["profile"]["fragment_size_target"], 5000);

        // 3. Confirm getOrientation returns balanced
        let req3 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(3),
            method: "profile/getOrientation".to_string(),
            params: None,
        };
        let res3 = handle_request(&state, req3).await;
        assert_eq!(res3.result.unwrap()["orientation"], "balanced");

        // 4. Set orientation to human_oriented
        let req4 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(4),
            method: "profile/setOrientation".to_string(),
            params: Some(json!({ "orientation": "human_oriented" })),
        };
        let res4 = handle_request(&state, req4).await;
        let result4 = res4.result.unwrap();
        assert_eq!(result4["orientation"], "human_oriented");
        assert_eq!(result4["profile"]["fragment_size_target"], 8000);
    }

    #[tokio::test]
    async fn test_ipc_custom_config_and_validation_handlers() {
        let temp_dir = tempfile::tempdir().unwrap();
        let state = Arc::new(RwLock::new(AppState::new(temp_dir.path().to_path_buf())));

        // 1. Get initial custom config
        let req1 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "profile/getCustomConfig".to_string(),
            params: None,
        };
        let res1 = handle_request(&state, req1).await;
        assert!(res1.error.is_none());
        let result1 = res1.result.unwrap();
        assert!(result1.get("tier1").is_some());
        assert!(result1.get("tier2").is_some());
        assert!(result1.get("constraints").is_some());

        // 2. Validate valid constraints
        let builtins = crate::evolution::ConflictDetector::builtin_constraints();
        let req2 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "profile/validateConstraints".to_string(),
            params: Some(json!({ "constraints": builtins })),
        };
        let res2 = handle_request(&state, req2).await;
        assert!(res2.error.is_none());
        let result2 = res2.result.unwrap();
        assert_eq!(result2["is_valid"], true);
        assert_eq!(result2["conflicts"].as_array().unwrap().len(), 0);

        // 3. Validate contradictory constraints
        let mut contradictory = builtins.clone();
        contradictory.push(crate::evolution::ConstraintRule {
            id: "contradict_decomp".to_string(),
            level: crate::evolution::ConstraintLevel::HardConstraint,
            rule_key: crate::evolution::RULE_FORCE_DECOMPOSITION.to_string(),
            description: "Force decomposition".to_string(),
        });
        let req3 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(3),
            method: "profile/validateConstraints".to_string(),
            params: Some(json!({ "constraints": contradictory })),
        };
        let res3 = handle_request(&state, req3).await;
        let result3 = res3.result.unwrap();
        assert_eq!(result3["is_valid"], false);
        assert!(!result3["conflicts"].as_array().unwrap().is_empty());

        // 4. Update custom config
        let req4 = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(4),
            method: "profile/updateCustomConfig".to_string(),
            params: Some(json!({
                "tier1": {
                    "ai_discoverability": "high",
                    "context_efficiency": "medium",
                    "human_readability": "high",
                    "knowledge_reuse": "medium",
                    "narrative_coherence": "high",
                    "maintenance_simplicity": "high"
                },
                "tier2": {
                    "cohesion_threshold": 0.7,
                    "semantic_diversity_threshold": 0.4,
                    "reference_concentration_threshold": 0.6,
                    "fragment_size_target": 4000,
                    "max_trace_depth": 4,
                    "candidate_expansion_factor": 3,
                    "max_context_surface_tokens": 15000,
                    "split_sensitivity": 0.5,
                    "canonical_ownership_sensitivity": 0.7,
                    "duplicate_tolerance": 0.1,
                    "fragmentation_tolerance": 0.15
                },
                "constraints": builtins
            })),
        };
        let res4 = handle_request(&state, req4).await;
        assert!(res4.error.is_none());
        let result4 = res4.result.unwrap();
        assert_eq!(result4["success"], true);
        assert!(result4.get("profile").is_some());
    }

    #[tokio::test]
    async fn test_ipc_get_structural_problems() {
        let temp_dir = tempfile::tempdir().unwrap();
        let state = Arc::new(RwLock::new(AppState::new(temp_dir.path().to_path_buf())));

        // Insert an implicit router node into state topology
        {
            let mut st = state.write().await;
            let mut router = crate::model::NoteNode::new("router.md", "/path/router.md", "Router");
            router.out_degree = 25;
            router.tokens = 300;
            router.betweenness = 0.50;
            st.topology.add_node(router);
        }

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "analytics/getStructuralProblems".to_string(),
            params: None,
        };
        let res = handle_request(&state, req).await;
        assert!(res.error.is_none());
        let result = res.result.unwrap();
        let signatures = result["signatures"]
            .as_array()
            .expect("signatures is an array");
        assert!(
            !signatures.is_empty(),
            "Should detect the implicit router signature"
        );
        assert_eq!(signatures[0]["problem_type"], "implicit_router");
    }

    #[tokio::test]
    async fn test_ipc_get_recommendations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let state = Arc::new(RwLock::new(AppState::new(temp_dir.path().to_path_buf())));

        // Insert a God Document node into state topology
        {
            let mut st = state.write().await;
            let mut god = crate::model::NoteNode::new("god.md", "/path/god.md", "God Document");
            god.tokens = 10_000;
            god.in_degree = 8;
            god.out_degree = 8;
            god.fragments = vec![
                crate::fragment::FragmentMetadata {
                    anchor: "sec1".to_string(),
                    heading: "Alpha Subject".to_string(),
                    start_line: 1,
                    end_line: 50,
                    tokens: 5000,
                    inbound: 19,
                    outbound: 0,
                },
                crate::fragment::FragmentMetadata {
                    anchor: "sec2".to_string(),
                    heading: "Zeta Completely Unrelated Subject".to_string(),
                    start_line: 51,
                    end_line: 100,
                    tokens: 5000,
                    inbound: 1,
                    outbound: 0,
                },
            ];
            st.topology.add_node(god);
        }

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "recommendation/getRecommendations".to_string(),
            params: None,
        };
        let res = handle_request(&state, req).await;
        assert!(res.error.is_none());
        let result = res.result.unwrap();
        let recs = result["recommendations"]
            .as_array()
            .expect("recommendations is an array");
        assert!(
            !recs.is_empty(),
            "Should generate recommendations for God Document"
        );
        assert!(recs[0].get("strategy").is_some());
        assert!(recs[0].get("confidence").is_some());
        assert!(recs[0].get("expected_benefits").is_some());
        assert!(recs[0].get("trade_offs").is_some());
    }

    #[tokio::test]
    async fn test_ipc_transformation_plan_handlers() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path().to_path_buf();
        let god_file = root.join("god.md");
        std::fs::write(&god_file, "# God Document\n\n## Alpha Subject\n\nContent here.\n\n## Zeta Unrelated\n\nMore content.\n").unwrap();

        let state = Arc::new(RwLock::new(AppState::new(root.clone())));

        // Add node to state topology
        {
            let mut st = state.write().await;
            let mut god = crate::model::NoteNode::new(
                "god.md",
                god_file.to_string_lossy().to_string(),
                "God Document",
            );
            god.tokens = 10_000;
            god.in_degree = 8;
            god.out_degree = 8;
            god.fragments = vec![
                crate::fragment::FragmentMetadata {
                    anchor: "sec1".to_string(),
                    heading: "Alpha Subject".to_string(),
                    start_line: 1,
                    end_line: 50,
                    tokens: 5000,
                    inbound: 19,
                    outbound: 0,
                },
                crate::fragment::FragmentMetadata {
                    anchor: "sec2".to_string(),
                    heading: "Zeta Unrelated".to_string(),
                    start_line: 51,
                    end_line: 100,
                    tokens: 5000,
                    inbound: 1,
                    outbound: 0,
                },
            ];
            st.topology.add_node(god);
        }

        // 1. Get recommendations
        let req_rec = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "recommendation/getRecommendations".to_string(),
            params: None,
        };
        let res_rec = handle_request(&state, req_rec).await;
        assert!(res_rec.error.is_none());
        let recs = res_rec.result.unwrap()["recommendations"]
            .as_array()
            .unwrap()
            .clone();
        assert!(!recs.is_empty());
        let rec_id = recs[0]["recommendation_id"].as_str().unwrap().to_string();

        // 2. Generate transformation plan
        let req_gen = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "transformation/generatePlan".to_string(),
            params: Some(json!({ "recommendation_id": rec_id })),
        };
        let res_gen = handle_request(&state, req_gen).await;
        assert!(
            res_gen.error.is_none(),
            "generatePlan error: {:?}",
            res_gen.error
        );
        let plan_json = res_gen.result.unwrap();
        let plan_id = plan_json["plan_id"].as_str().unwrap().to_string();
        assert_eq!(
            plan_json["requires_approval"], true,
            "Must require approval by default per Invariant XVI"
        );
        assert!(!plan_json["diff_preview"].as_str().unwrap().is_empty());
        assert!(!plan_json["actions"].as_array().unwrap().is_empty());

        // 3. Apply transformation plan
        let req_apply = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(3),
            method: "transformation/applyPlan".to_string(),
            params: Some(json!({ "plan_id": plan_id })),
        };
        let res_apply = handle_request(&state, req_apply).await;
        assert!(
            res_apply.error.is_none(),
            "applyPlan error: {:?}",
            res_apply.error
        );
        let apply_res = res_apply.result.unwrap();
        assert_eq!(apply_res["success"], true);
        assert!(apply_res.get("actual_delta").is_some());

        // 4. Invariant XVI test: attempt to apply plan with requires_approval = false
        let illegal_plan = crate::evolution::TransformationPlan {
            plan_id: "illegal_plan".to_string(),
            recommendation_id: rec_id,
            actions: vec![],
            diff_preview: "".to_string(),
            projected_delta: Default::default(),
            requires_approval: false,
        };
        let req_illegal = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(4),
            method: "transformation/applyPlan".to_string(),
            params: Some(json!({
                "plan_id": "illegal_plan",
                "plan": illegal_plan
            })),
        };
        let res_illegal = handle_request(&state, req_illegal).await;
        assert!(
            res_illegal.error.is_some(),
            "Must reject unapproved plan per Invariant XVI"
        );
    }

    #[tokio::test]
    async fn test_exclusions_disk_persistence_lifecycle() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join(".context-vault.yaml");
        let initial_cfg = ContextVaultConfig::default();
        initial_cfg.save_to_file(&config_path).unwrap();

        let state = Arc::new(RwLock::new(AppState::new(temp_dir.path().to_path_buf())));

        // 1. Exclude path with scope "graph"
        let req_ex_graph = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "graph/excludePath".to_string(),
            params: Some(json!({
                "pattern": "tests/**",
                "scope": "graph"
            })),
        };
        let res_ex_graph = handle_request(&state, req_ex_graph).await;
        assert!(res_ex_graph.error.is_none());

        // 2. Exclude path with scope "scan"
        let req_ex_scan = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "graph/excludePath".to_string(),
            params: Some(json!({
                "pattern": "vendor/**",
                "scope": "scan"
            })),
        };
        let res_ex_scan = handle_request(&state, req_ex_scan).await;
        assert!(res_ex_scan.error.is_none());

        // Verify on-disk config updated
        let on_disk_cfg = ContextVaultConfig::load_from_file(&config_path).unwrap();
        assert!(on_disk_cfg.get_graph_exclude().contains(&"tests/**".to_string()));
        assert!(on_disk_cfg.get_scan_exclude().contains(&"vendor/**".to_string()));

        // 3. getExclusions
        let req_get = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(3),
            method: "graph/getExclusions".to_string(),
            params: None,
        };
        let res_get = handle_request(&state, req_get).await;
        assert!(res_get.error.is_none());
        let get_val = res_get.result.unwrap();
        let graph_ex = get_val["graph_exclusions"].as_array().unwrap();
        let scan_ex = get_val["scan_exclusions"].as_array().unwrap();
        assert!(graph_ex.iter().any(|v| v.as_str() == Some("tests/**")));
        assert!(scan_ex.iter().any(|v| v.as_str() == Some("vendor/**")));

        // 4. removeExclusion for graph
        let req_rm_graph = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(4),
            method: "graph/removeExclusion".to_string(),
            params: Some(json!({
                "pattern": "tests/**",
                "scope": "graph"
            })),
        };
        let res_rm_graph = handle_request(&state, req_rm_graph).await;
        assert!(res_rm_graph.error.is_none());

        // 5. removeExclusion for scan
        let req_rm_scan = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(5),
            method: "graph/removeExclusion".to_string(),
            params: Some(json!({
                "pattern": "vendor/**",
                "scope": "scan"
            })),
        };
        let res_rm_scan = handle_request(&state, req_rm_scan).await;
        assert!(res_rm_scan.error.is_none());

        // Verify on-disk config updated after removal
        let final_disk_cfg = ContextVaultConfig::load_from_file(&config_path).unwrap();
        assert!(!final_disk_cfg.get_graph_exclude().contains(&"tests/**".to_string()));
        assert!(!final_disk_cfg.get_scan_exclude().contains(&"vendor/**".to_string()));
    }
}
