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
}

impl AppState {
    pub fn new(workspace_root: PathBuf) -> Self {
        let config_path = workspace_root.join(".context-vault.yaml");
        let config = ContextVaultConfig::load_from_file(&config_path).unwrap_or_default();
        Self {
            workspace_root,
            topology: GraphTopology::new(),
            config,
            active_hub: None,
            cached_plans: std::collections::HashMap::new(),
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
                        let mut n = crate::model::NoteNode::new(node.id.clone(), node.file_path.clone(), node.label.clone());
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
                        let mut n = crate::model::NoteNode::new(node.id.clone(), node.file_path, node.label);
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
            let quality = crate::quality::calculate_quality_score(&topology, baseline.as_ref(), &collisions, &drifts);
            JsonRpcResponse::success(req.id, json!({
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
            }))
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
                        let mut n = crate::model::NoteNode::new(node.id.clone(), node.file_path, node.label);
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
            let quality = crate::quality::calculate_quality_score(&topology, baseline.as_ref(), &collisions, &drifts);
            let all_issues = crate::quality::extract_all_issues(&topology, &collisions, &drifts);

            let integrity_count = all_issues.iter().filter(|i| i.pillar == "integrity").count();
            let structure_count = all_issues.iter().filter(|i| i.pillar == "structure").count();
            let consistency_count = all_issues.iter().filter(|i| i.pillar == "consistency").count();
            let discoverability_count = all_issues.iter().filter(|i| i.pillar == "discoverability").count();
            let freshness_count = all_issues.iter().filter(|i| i.pillar == "freshness").count();
            let total_issues = all_issues.len();

            let pillar_filter = req.params.as_ref()
                .and_then(|p| p.get("pillar"))
                .and_then(|v| v.as_str())
                .unwrap_or("all")
                .to_lowercase();

            let limit = req.params.as_ref()
                .and_then(|p| p.get("limit"))
                .and_then(|v| v.as_u64())
                .map(|v| v as usize);

            let filtered_issues: Vec<_> = if pillar_filter == "all" || pillar_filter.is_empty() {
                all_issues
            } else {
                all_issues
                    .into_iter()
                    .filter(|i| i.pillar.to_lowercase() == pillar_filter)
                    .collect()
            };

            let report_issues = if let Some(lim) = limit {
                filtered_issues.into_iter().take(lim).collect()
            } else {
                filtered_issues
            };

            JsonRpcResponse::success(req.id, json!({
                "overall_score": quality.overall_score,
                "total_issues": total_issues,
                "integrity_count": integrity_count,
                "structure_count": structure_count,
                "consistency_count": consistency_count,
                "discoverability_count": discoverability_count,
                "freshness_count": freshness_count,
                "issues": report_issues,
            }))
        }
        "workspace/getBaseline" => {
            let root = {
                let st = state.read().await;
                st.workspace_root.clone()
            };
            match crate::debt::baseline::load_baseline(&root) {
                Ok(Some(base)) => {
                    JsonRpcResponse::success(req.id, json!({
                        "baseline_commit": base.baseline_commit,
                        "timestamp": base.timestamp,
                        "broken_links_count": base.broken_links.len(),
                        "orphan_nodes_count": base.orphan_nodes.len(),
                        "collisions_count": base.collisions.len(),
                        "drifts_count": base.technical_drifts.len(),
                        "total_debt": base.total_debt,
                    }))
                }
                Ok(None) => {
                    JsonRpcResponse::success(req.id, json!({
                        "baseline_commit": "unbaselined",
                        "timestamp": "",
                        "broken_links_count": 0,
                        "orphan_nodes_count": 0,
                        "collisions_count": 0,
                        "drifts_count": 0,
                        "total_debt": 0,
                    }))
                }
                Err(e) => JsonRpcResponse::error(req.id, -32000, format!("Failed to read baseline: {}", e)),
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
                    let new_base = crate::debt::baseline::load_baseline(&root)
                        .ok()
                        .flatten();
                    let (commit, total) = new_base
                        .map(|b| (b.baseline_commit, b.total_debt))
                        .unwrap_or_else(|| ("unknown".to_string(), 0));
                    JsonRpcResponse::success(req.id, json!({
                        "baseline_commit": commit,
                        "total_debt": total,
                        "previous_total_debt": prev_total,
                    }))
                }
                Err(e) => JsonRpcResponse::error(req.id, -32000, format!("Failed to update baseline: {}", e)),
            }
        }
        "workspace/traceContext" => {
            let (query, entrypoint) = match req.params {
                Some(ref p) => (
                    p.get("query").and_then(|q| q.as_str()).unwrap_or("").to_string(),
                    p.get("entrypoint").and_then(|e| e.as_str()).map(|s| s.to_string()),
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
                        let mut n = crate::model::NoteNode::new(node.id.clone(), node.file_path, node.label);
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

            let trace_res = crate::trace::traversal::execute_context_trace(&topology, &query, entrypoint.as_deref());
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
                    let commit = p.get("base_commit").and_then(|c| c.as_str()).map(|s| s.to_string());
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
                _ => crate::debt::baseline::capture_initial_baseline(&base_graph, "initial", vec![], vec![]),
            };

            let overlay = crate::candidate::overlay::CandidateOverlay::new(&base_graph, &root, &staged_delta);
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
            let path = match req.params.and_then(|p| p.get("path").and_then(|v| v.as_str().map(|s| s.to_string()))) {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'path' parameter"),
            };
            let st = state.read().await;
            if let Some(node) = st.topology.get_node(&path) {
                JsonRpcResponse::success(req.id, json!({
                    "path": path,
                    "total_tokens": node.tokens,
                    "fragments": node.fragments,
                }))
            } else {
                let abs_path = st.workspace_root.join(&path);
                if let Ok(content) = std::fs::read_to_string(&abs_path) {
                    let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                    let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                    let fragments = crate::fragment::extractor::extract_fragments_from_document(&content, &parsed.edges);
                    JsonRpcResponse::success(req.id, json!({
                        "path": path,
                        "total_tokens": tokens,
                        "fragments": fragments,
                    }))
                } else {
                    JsonRpcResponse::error(req.id, -32004, format!("Document '{}' not found", path))
                }
            }
        }
        "graph/materializeFrontmatter" => {
            let path = match req.params.as_ref().and_then(|p| p.get("path").and_then(|v| v.as_str().map(|s| s.to_string()))) {
                Some(p) => p,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'path' parameter"),
            };
            let dry_run = req.params.as_ref().and_then(|p| p.get("dry_run").and_then(|v| v.as_bool())).unwrap_or(false);
            let st = state.read().await;
            let abs_path = st.workspace_root.join(&path);
            if !abs_path.exists() {
                return JsonRpcResponse::error(req.id, -32004, format!("Document '{}' not found on disk", path));
            }
            if dry_run {
                if let Ok(content) = std::fs::read_to_string(&abs_path) {
                    let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                    let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                    let fragments = crate::fragment::extractor::extract_fragments_from_document(&content, &parsed.edges);
                    JsonRpcResponse::success(req.id, json!({
                        "path": path,
                        "updated": false,
                        "fragment_count": fragments.len(),
                        "total_tokens": tokens,
                    }))
                } else {
                    JsonRpcResponse::error(req.id, -32000, "Failed to read file for dry run")
                }
            } else {
                match crate::cli::materialize_cmd::materialize_file(&abs_path, false) {
                    Ok(updated) => {
                        let content = std::fs::read_to_string(&abs_path).unwrap_or_default();
                        let tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0);
                        let parsed = crate::ast::parser::parse_markdown_ast(&path, &content);
                        let fragments = crate::fragment::extractor::extract_fragments_from_document(&content, &parsed.edges);
                        JsonRpcResponse::success(req.id, json!({
                            "path": path,
                            "updated": updated,
                            "fragment_count": fragments.len(),
                            "total_tokens": tokens,
                        }))
                    }
                    Err(e) => JsonRpcResponse::error(req.id, -32000, format!("Materialize failed: {}", e)),
                }
            }
        }
        "trace/contextTraceSection" => {
            let query = match req.params.as_ref().and_then(|p| p.get("query").and_then(|v| v.as_str().map(|s| s.to_string()))) {
                Some(q) => q,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'query' parameter"),
            };
            let entrypoint = req.params.as_ref().and_then(|p| p.get("entrypoint").and_then(|v| v.as_str().map(|s| s.to_string())));
            let st = state.read().await;
            match crate::trace::beam::trace_context_section(&st.topology, &query, entrypoint.as_deref()) {
                Some(res) => JsonRpcResponse::success(req.id, json!(res)),
                None => JsonRpcResponse::error(req.id, -32004, "No context trace section match found"),
            }
        }
        "quality/getFivePillarScore" => {
            let st = state.read().await;
            let root = st.workspace_root.clone();
            let base = crate::debt::baseline::load_baseline(&root).ok().flatten();
            let collisions = crate::collision::detect_collisions_in_graph(&st.topology, &root, 0.70);
            let drifts = crate::drift::detect_all_drifts_in_graph(&st.topology, &root);
            let score = crate::quality::scorer::calculate_quality_score(&st.topology, base.as_ref(), &collisions, &drifts);

            let debt_summary = crate::debt::DebtSummary {
                total_debt: score.legacy_debt_count + score.new_regressions_count,
                legacy_debt: score.legacy_debt_count,
                new_debt: score.new_regressions_count,
                resolved_debt: score.resolved_debt_count,
                regressions: score.new_regressions_count,
            };

            let mut reasons: std::collections::HashMap<crate::quality::QualityPillar, Vec<String>> = std::collections::HashMap::new();
            for f in &score.top_degradation_factors {
                let pillar = match f.category.as_str() {
                    "broken_link" | "broken_anchor" => crate::quality::QualityPillar::Integrity,
                    "structure_debt" => crate::quality::QualityPillar::Structure,
                    "collision" => crate::quality::QualityPillar::Consistency,
                    "technical_drift" | "missing_code_citation" | "code_modified_after_doc" => crate::quality::QualityPillar::Freshness,
                    _ => crate::quality::QualityPillar::Discoverability,
                };
                reasons.entry(pillar).or_default().push(format!("{}: {}", f.affected_node, f.description));
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
        "profile/setOrientation" => {
            let orientation = match req.params.as_ref().and_then(|p| {
                p.get("orientation").and_then(|v| {
                    serde_json::from_value::<crate::evolution::DocumentationOrientation>(v.clone()).ok()
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
                p.get("file_path").and_then(|v| v.as_str().map(|s| s.to_string()))
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

            let signatures = crate::evolution::detect_all_signatures(&nodes, target);
            JsonRpcResponse::success(req.id, json!({ "signatures": signatures }))
        }
        "recommendation/getRecommendations" => {
            let signature_id = req.params.as_ref().and_then(|p| {
                p.get("signature_id").and_then(|v| v.as_str().map(|s| s.to_string()))
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
                p.get("recommendation_id").and_then(|v| v.as_str().map(|s| s.to_string()))
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
                                    st.workspace_root.join(&sig.document_path).to_string_lossy().to_string(),
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
                Some((rec, node)) => {
                    crate::evolution::TransformationEngine::build_plan(&rec, &node, &st.workspace_root)
                }
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
                p.get("plan_id").and_then(|v| v.as_str().map(|s| s.to_string()))
            }) {
                Some(id) => id,
                None => return JsonRpcResponse::error(req.id, -32602, "Missing 'plan_id' parameter"),
            };

            let st = state.read().await;
            let plan_opt = if let Some(plan_val) = req.params.as_ref().and_then(|p| p.get("plan")) {
                serde_json::from_value::<crate::evolution::TransformationPlan>(plan_val.clone()).ok()
            } else {
                st.cached_plans.get(&plan_id).cloned()
            };

            let plan = match plan_opt {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(
                        req.id,
                        -32004,
                        format!("Transformation plan '{}' not found", plan_id),
                    )
                }
            };
            let root = st.workspace_root.clone();
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
        let signatures = result["signatures"].as_array().expect("signatures is an array");
        assert!(!signatures.is_empty(), "Should detect the implicit router signature");
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
        let recs = result["recommendations"].as_array().expect("recommendations is an array");
        assert!(!recs.is_empty(), "Should generate recommendations for God Document");
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
            let mut god = crate::model::NoteNode::new("god.md", god_file.to_string_lossy().to_string(), "God Document");
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
        let recs = res_rec.result.unwrap()["recommendations"].as_array().unwrap().clone();
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
        assert!(res_gen.error.is_none(), "generatePlan error: {:?}", res_gen.error);
        let plan_json = res_gen.result.unwrap();
        let plan_id = plan_json["plan_id"].as_str().unwrap().to_string();
        assert_eq!(plan_json["requires_approval"], true, "Must require approval by default per Invariant XVI");
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
        assert!(res_apply.error.is_none(), "applyPlan error: {:?}", res_apply.error);
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
        assert!(res_illegal.error.is_some(), "Must reject unapproved plan per Invariant XVI");
    }
}

