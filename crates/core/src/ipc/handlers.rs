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
