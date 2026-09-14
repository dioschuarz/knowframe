//! JSON-RPC 2.0 protocol messages and payloads adhering to ipc_jsonrpc_protocol.json.

use crate::model::{LinkType, NodeStatus, NoteEdge, NoteNode, TokenBudgetStatus, TokenSummary};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
}

impl JsonRpcNotification {
    pub fn new(method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanResultPayload {
    pub nodes: Vec<NodePayload>,
    pub edges: Vec<EdgePayload>,
    pub token_summary: TokenSummaryPayload,
    pub recommended_hub: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observatory_summary: Option<crate::model::ObservatorySummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodePayload {
    pub id: String,
    pub label: String,
    pub file_path: String,
    pub tags: Vec<String>,
    pub color: String,
    pub status: NodeStatus,
    pub tokens: u32,
    pub hop_distance: i32,
    pub in_degree: u32,
    pub out_degree: u32,
    pub betweenness: f64,
    pub broken_links: u32,
    #[serde(default)]
    pub fragments: Vec<crate::fragment::FragmentMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<crate::drift::FreshnessEvidence>,
}

impl From<&NoteNode> for NodePayload {
    fn from(n: &NoteNode) -> Self {
        Self {
            id: n.id.clone(),
            label: n.label.clone(),
            file_path: n.file_path.clone(),
            tags: n.tags.clone(),
            color: n.color.clone(),
            status: n.status,
            tokens: n.tokens,
            hop_distance: n.hop_distance,
            in_degree: n.in_degree,
            out_degree: n.out_degree,
            betweenness: n.betweenness,
            broken_links: n.broken_links,
            fragments: n.fragments.clone(),
            freshness: n.freshness.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgePayload {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_fragment: Option<String>,
    pub is_broken: bool,
    pub link_type: LinkType,
    #[serde(default)]
    pub anchor: Option<String>,
    #[serde(default)]
    pub line: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_status: Option<crate::model::LinkResolutionStatus>,
}

impl From<&NoteEdge> for EdgePayload {
    fn from(e: &NoteEdge) -> Self {
        Self {
            id: e.id.clone(),
            source: e.source.clone(),
            target: e.target.clone(),
            target_fragment: e.target_fragment.clone(),
            is_broken: e.is_broken,
            link_type: e.link_type,
            anchor: e.anchor.clone(),
            line: e.line,
            resolution_status: e.resolution_status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSummaryPayload {
    pub active_harness_tokens: u64,
    pub subcall_accessible_tokens: u64,
    pub warning_threshold: u64,
    pub critical_threshold: u64,
    pub status: TokenBudgetStatus,
}

impl From<&TokenSummary> for TokenSummaryPayload {
    fn from(s: &TokenSummary) -> Self {
        Self {
            active_harness_tokens: s.active_harness_tokens,
            subcall_accessible_tokens: s.subcall_accessible_tokens,
            warning_threshold: s.warning_threshold,
            critical_threshold: s.critical_threshold,
            status: s.status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitDeltaParams {
    pub repo_path: String,
    pub commit_hash: Option<String>,
    pub branch: Option<String>,
    pub changed_files: Vec<ChangedFileItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedFileItem {
    pub path: String,
    pub status: String, // "added", "modified", "deleted"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetHubParams {
    pub hub_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewExclusionParams {
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewExclusionResult {
    pub matching_count: u32,
    pub sample_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludePathParams {
    pub pattern: String,
    pub scope: String, // "scan" | "graph"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcludePathResult {
    pub success: bool,
    pub affected_files_count: u32,
    pub updated_summary: ObservatorySummaryCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservatorySummaryCounts {
    pub total_notes: u32,
    pub connected_notes: u32,
    pub orphan_notes: u32,
    pub excluded_notes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestExclusionsResult {
    pub suggestions: Vec<crate::model::NoiseSuggestion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCentralHubParams {
    pub hub_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCentralHubResult {
    pub success: bool,
    pub primary_hub: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFocusModeParams {
    pub focus_mode: crate::model::FocusMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenFileNotificationParams {
    pub file_path: String,
    pub line_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixBrokenLinkNotificationParams {
    pub source_path: String,
    pub target_path: String,
    pub line_number: u32,
}
