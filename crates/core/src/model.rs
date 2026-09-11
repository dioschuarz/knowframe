//! Core domain entities and data models for LLM Wiki knowledge graph.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Active,
    ConnectedInactive,
    Orphan,
    Excluded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LinkResolutionStatus {
    #[default]
    Resolved,
    ResolvedExcluded,
    BrokenMissingFile,
    BrokenMissingAnchor,
    BrokenWikilink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum FocusMode {
    #[serde(rename = "hub")]
    Hub,
    #[serde(rename = "1_hop")]
    OneHop,
    #[serde(rename = "2_hops")]
    TwoHops,
    #[serde(rename = "all")]
    #[default]
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkType {
    Markdown,
    Wikilink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterQueryType {
    Tag,
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum VaultMode {
    #[default]
    SingleRepo,
    MultiRepo,
    Vault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TokenBudgetStatus {
    #[default]
    Normal,
    Warning,
    Critical,
}

/// Represents an individual note node in the knowledge graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteNode {
    /// Canonical relative POSIX path (e.g. "docs/architecture.md")
    pub id: String,
    /// Absolute filesystem path
    pub file_path: String,
    /// Human-readable title
    pub label: String,
    /// Normalized tags
    pub tags: Vec<String>,
    /// Exact BPE token count (cl100k_base)
    pub tokens: u32,
    /// Number of incoming references
    pub in_degree: u32,
    /// Number of outgoing references
    pub out_degree: u32,
    /// Brandes betweenness centrality [0.0, 1.0]
    pub betweenness: f64,
    /// BFS hop distance: 0 for Hub, >=1 for hops, -1 for orphan
    pub hop_distance: i32,
    /// Operational context state
    pub status: NodeStatus,
    /// Hex color #RRGGBB
    pub color: String,
    /// Dead links originating in this note
    pub broken_links: u32,
}

impl NoteNode {
    pub fn new(
        id: impl Into<String>,
        file_path: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        let id_str = id.into();
        Self {
            id: id_str.clone(),
            file_path: file_path.into(),
            label: label.into(),
            tags: Vec::new(),
            tokens: 0,
            in_degree: 0,
            out_degree: 0,
            betweenness: 0.0,
            hop_distance: -1,
            status: NodeStatus::Orphan,
            color: "#6B7280".to_string(), // Muted gray default
            broken_links: 0,
        }
    }
}

/// Represents a directed edge between two notes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NoteEdge {
    /// Unique edge ID ("source->target")
    pub id: String,
    /// Source node canonical ID
    pub source: String,
    /// Target document canonical ID
    pub target: String,
    /// Markdown syntax style
    pub link_type: LinkType,
    /// True if destination file or section heading anchor does not exist
    pub is_broken: bool,
    /// Exact raw link text from source document
    pub raw_text: String,
    /// Optional section heading anchor (e.g. "#section-heading")
    pub anchor: Option<String>,
    /// 1-based source line number in originating document
    pub line: Option<u32>,
    /// Link resolution status
    #[serde(default)]
    pub resolution_status: Option<LinkResolutionStatus>,
}

impl NoteEdge {
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        link_type: LinkType,
        raw_text: impl Into<String>,
        anchor: Option<String>,
        line: Option<u32>,
    ) -> Self {
        let src = source.into();
        let tgt = target.into();
        let id = format!("{}->{}", src, tgt);
        Self {
            id,
            source: src,
            target: tgt,
            link_type,
            is_broken: false,
            raw_text: raw_text.into(),
            anchor,
            line,
            resolution_status: Some(LinkResolutionStatus::Resolved),
        }
    }
}

/// Token metrics summary across active and accessible documentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSummary {
    pub active_harness_tokens: u64,
    pub subcall_accessible_tokens: u64,
    pub warning_threshold: u64,
    pub critical_threshold: u64,
    pub status: TokenBudgetStatus,
}

impl TokenSummary {
    pub fn new(active: u64, subcall: u64, warn: u64, crit: u64) -> Self {
        let status = if active >= crit {
            TokenBudgetStatus::Critical
        } else if active >= warn {
            TokenBudgetStatus::Warning
        } else {
            TokenBudgetStatus::Normal
        };

        Self {
            active_harness_tokens: active,
            subcall_accessible_tokens: subcall,
            warning_threshold: warn,
            critical_threshold: crit,
            status,
        }
    }
}

/// Represents an active path ignore or graph exclusion rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExclusionRule {
    pub id: String,
    pub pattern: String,
    pub scope: String,
    #[serde(default)]
    pub matching_count: Option<u32>,
    pub created_at: String,
}

/// Automated recommendation for noisy clusters of disconnected files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoiseSuggestion {
    pub pattern: String,
    pub matching_count: u32,
    pub reason: String,
    pub recommended: bool,
}

/// Aggregated vault health, token load, and topology metrics for Observatory UX.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservatorySummary {
    pub total_notes: u32,
    pub connected_notes: u32,
    pub orphan_notes: u32,
    pub excluded_notes: u32,
    pub broken_links_count: u32,
    pub active_tokens: u64,
    pub reachable_tokens: u64,
    pub sync_status: String,
    pub last_sync_timestamp: u64,
}
