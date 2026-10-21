//! Core domain entities and data models for LLM Wiki knowledge graph.

use serde::{Deserialize, Serialize};

use crate::drift::FreshnessEvidence;
pub use crate::fragment::FragmentMetadata;

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
    BrokenFragmentAnchor,
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
    #[serde(rename = "3_hops")]
    ThreeHops,
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
    /// Addressable section fragments (Invariant V: metadata, not graph nodes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fragments: Vec<FragmentMetadata>,
    /// Deterministic freshness evidence (Pillar 5)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<FreshnessEvidence>,
    /// Explicit code dependencies declared in frontmatter or citations (Invariant VIII)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_dependencies: Vec<CodeDependency>,
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
            fragments: Vec::new(),
            freshness: None,
            code_dependencies: Vec::new(),
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
    /// Optional section heading anchor target (e.g. "section-heading")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_fragment: Option<String>,
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
        let target_fragment = anchor
            .as_ref()
            .map(|a| a.trim_start_matches('#').to_string());
        Self {
            id,
            source: src,
            target: tgt,
            link_type,
            is_broken: false,
            raw_text: raw_text.into(),
            anchor,
            target_fragment,
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

// Re-export Admission Control, Debt Baseline & Quality Models
pub use crate::candidate::{
    AdmissionDecision, CandidateGraph, CandidateValidationReport, DebtAccountingSummary,
    RegressionCategory, RegressionItem, RegressionSeverity, ResolvedItem, WarningItem,
};
pub use crate::collision::{CollisionClassification, DocumentFingerprint, SemanticCollision};
pub use crate::debt::{
    BrokenLinkSignature, CollisionSignature, DriftSignature, KnowledgeDebtBaseline,
};
pub use crate::drift::{DriftConfidence, DriftType, ReverseImpactIndex, TechnicalDriftFinding};
pub use crate::quality::{KnowledgeQualityScore, QualityDegradationFactor};
pub use crate::trace::ContextTraceResult;

// ==============================================================================
// Feature 009: Deterministic Knowledge Engineering Engine Domain Models
// ==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicTier {
    TierAFullyDeterministic,
    TierBCandidateGeneration,
    TierCDecisionSupport,
    TierDNotReliablyDeterminable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionMode {
    #[default]
    AiOriented,
    HumanOriented,
    Balanced,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Methodology {
    Classification,
    Taxonomy,
    FacetedClassification,
    ControlledVocabulary,
    Thesaurus,
    SemanticNetworks,
    ConceptualMaps,
    OntologyEngineering,
    DomainAnalysis,
    MetadataApplicationProfiles,
    InformationArchitecture,
    InformationRetrieval,
    TerminologyManagement,
    KnowledgeGraphNetworkAnalysis,
    KnowledgeQuality,
    Provenance,
    KnowledgeDrift,
    ContextRouting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityPillar {
    Integrity,
    Structure,
    Consistency,
    Discoverability,
    Freshness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeCitation {
    pub target_file_path: String,
    pub symbol_name: Option<String>,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceEdge {
    pub target_document: String,
    pub target_fragment: Option<String>,
    pub relation_type: Option<String>,
    pub is_resolved: bool,
    pub line_number: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentNode {
    pub id: String,
    pub relative_path: String,
    pub title: String,
    pub raw_token_count: u64,
    pub frontmatter_schema: Option<String>,
    pub fragments: Vec<FragmentMetadata>,
    pub outgoing_references: Vec<ReferenceEdge>,
    pub git_last_modified_commit: String,
    pub git_last_modified_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphTopology {
    pub nodes: Vec<DocumentNode>,
    pub edges: Vec<ReferenceEdge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralFinding {
    pub id: String,
    pub methodology: Methodology,
    pub pillar: QualityPillar,
    pub tier: EpistemicTier,
    pub severity: FindingSeverity,
    pub document_id: String,
    pub fragment_anchor: Option<String>,
    pub line_number: usize,
    pub rule_key: String,
    pub message: String,
    pub evidence_metrics: serde_json::Value,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PillarHealth {
    pub raw_score: f64,
    pub legacy_debt_count: u64,
    pub new_regressions_count: u64,
    pub findings: Vec<StructuralFinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthScore {
    pub analysis_id: String,
    pub aggregate_score: f64,
    pub profile_mode: EvolutionMode,
    pub integrity: PillarHealth,
    pub structure: PillarHealth,
    pub consistency: PillarHealth,
    pub discoverability: PillarHealth,
    pub freshness: PillarHealth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextTrace {
    pub query: String,
    pub target_fragment: Option<String>,
    pub trace_path: Vec<String>,
    pub hop_count: u32,
    pub total_tokens: u64,
    pub context_reduction_percent: f64,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyType {
    PreserveDocument,
    SplitDecompose,
    ExtractFacetMetadata,
    ConsolidateIndex,
    MergeCandidates,
    StandardizeTerminology,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyRole {
    Primary,
    Supporting,
    Alternative,
    MutuallyExclusive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyRecommendation {
    pub strategy_id: String,
    pub strategy_type: StrategyType,
    pub role: StrategyRole,
    pub target_documents: Vec<String>,
    pub fit_score: f64,
    pub implementation_cost: f64,
    pub expected_benefits: Vec<String>,
    pub countervailing_trade_offs: Vec<String>,
    pub hard_constraints_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action_type", rename_all = "snake_case")]
pub enum TransformationAction {
    MoveFile { source: String, destination: String },
    UpdateFrontmatter { file: String, key: String, value: serde_json::Value },
    RetargetLink { file: String, line: usize, old_target: String, new_target: String },
    ExtractSectionToFile { source_file: String, anchor: String, new_file: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationPlan {
    pub plan_id: String,
    pub approval_token: String,
    pub strategy_id: String,
    pub target_files: Vec<String>,
    pub unified_diff: String,
    pub projected_score_delta: f64,
    pub actions: Vec<TransformationAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PillarDeltas {
    pub integrity: f64,
    pub structure: f64,
    pub consistency: f64,
    pub discoverability: f64,
    pub freshness: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReMeasurementDelta {
    pub aggregate_score_delta: f64,
    pub raw_token_delta: i64,
    pub graph_diameter_delta: Option<i32>,
    pub pillar_deltas: PillarDeltas,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileWeights {
    pub integrity: f64,
    pub structure: f64,
    pub consistency: f64,
    pub discoverability: f64,
    pub freshness: f64,
    pub context_efficiency: f64,
    pub navigation_depth: f64,
    pub vocabulary_adherence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileThresholds {
    pub max_document_tokens: u64,
    pub max_fragment_tokens: u64,
    pub max_navigation_depth: u32,
    pub min_scent_similarity: f64,
    pub max_betweenness_centrality: f64,
    pub min_category_coverage: f64,
    pub max_taxonomy_imbalance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileConstraints {
    pub never_auto_split_monoliths: bool,
    pub enforce_controlled_tags: bool,
    pub require_dcap_frontmatter: bool,
    pub prohibit_circular_thesaurus: bool,
    pub isolated_directories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationProfile {
    pub mode: EvolutionMode,
    pub profile_version: String,
    pub weights: ProfileWeights,
    pub thresholds: ProfileThresholds,
    pub constraints: ProfileConstraints,
}

pub mod metrics;
pub mod profile;
pub use profile::ProfileValidationError;

// ==============================================================================
// Feature 010: Deterministic Traceability, Review & Knowledge Consistency Models
// ==============================================================================

/// Origin of a code dependency citation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencySource {
    /// Declared in Markdown YAML frontmatter `code_dependencies` array
    Frontmatter,
    /// Markdown inline link pointing to a code file `[label](path/to/file.rs)`
    InlineMarkdownLink,
    /// Markdown backticked symbol citing code entity
    InlineSymbolCitation,
}

/// Represents an explicit dependency from a documentation note to a code file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeDependency {
    /// Normalized relative POSIX path from workspace root (e.g. "crates/core/src/engine.rs")
    pub code_path: String,
    /// Origin of declaration
    pub source: DependencySource,
    /// Line number where declared (Some(line) for inline citations, None for frontmatter)
    pub line_number: Option<u32>,
}

/// Status of a documentation review obligation following code changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Documentation was modified and updated in the same change set
    Updated,
    /// Document was reviewed and explicitly verified as correct without code edits
    ReviewedUnchanged,
    /// Pending review (change detected, documentation untouched)
    Pending,
    /// Blocked by repository gate policy (unreviewed regression in blocking mode)
    Blocked,
}

/// A concrete review obligation generated for a document affected by Git changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewObligation {
    /// Relative path of the affected Markdown document
    pub doc_id: String,
    /// Current resolution status
    pub status: ReviewStatus,
    /// List of modified/deleted/renamed code files triggering this obligation
    pub triggering_code_files: Vec<String>,
    /// All declaration origins triggering this obligation
    pub dependency_sources: Vec<DependencySource>,
    /// Receipt evidence string if marked `ReviewedUnchanged` (commit trailer or receipt ID)
    pub resolution_receipt: Option<String>,
}

/// Structured review receipt stored in `.context-vault/reviews.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewReceiptStore {
    /// Map of Git commit SHA or branch ref -> list of approved document IDs
    pub reviews: std::collections::HashMap<String, Vec<String>>,
}

/// Deterministic severity tiers for traceability and architectural findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicFindingSeverity {
    /// Information-only diagnostic (code changed without declaring docs)
    Info,
    /// Warning diagnostic requiring human/agent review (`review_required`)
    Warning,
    /// Error diagnostic violating an explicit architectural constraint or broken link
    Error,
}

/// Tri-state classification for change evaluation (including Epistemic Tier D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeEvaluationCategory {
    /// Code modification detected, no declaring documentation
    ChangeDetected,
    /// Document declared dependency on modified code; review required
    ReviewRequired,
    /// Concrete architectural constraint or integrity rule broken
    RuleViolation,
    /// Epistemic Tier D: Freeform semantic claim unverified by deterministic evidence
    NotVerifiable,
}

/// Machine-readable structured evidence attached to a diagnostic finding.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FindingEvidence {
    /// Changed code files triggering the finding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changed_paths: Vec<String>,
    /// Declaration origins (e.g. `frontmatter`, `inline_markdown_link`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_sources: Vec<String>,
    /// Human-readable explanation of the evaluated rule or invariant
    pub rule_description: String,
    /// Source line number where violation occurs (if applicable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u32>,
    /// Invalid anchor slug (for broken fragment anchor findings)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Target document ID (for broken fragment anchor findings)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Standardized diagnostic finding emitted by deterministic traceability and review engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicFinding {
    /// Unique deterministic ID (e.g. "FINDING-001" or hashed rule+subject)
    pub finding_id: String,
    /// Canonical rule identifier (e.g. "DOCUMENT_REVIEW_REQUIRED", "BROKEN_FRAGMENT_ANCHOR")
    pub rule_id: String,
    /// Severity level
    pub severity: DeterministicFindingSeverity,
    /// Target document path or code file under evaluation
    pub subject: String,
    /// Structured evidence payload
    pub evidence: FindingEvidence,
    /// Concrete remediation guidance
    pub required_action: String,
    /// Epistemic validation tier (Tier A to D)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epistemic_tier: Option<EpistemicTier>,
}

fn default_true() -> bool {
    true
}

/// Configuration for PR/CI documentation review gating in `.context-vault.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewGatingConfig {
    /// If true, unreviewed changes to documented code block PR validation (exit code 1)
    #[serde(default = "default_true")]
    pub enforce_doc_review: bool,
    /// If true, legacy baseline debt blocks PR validation (defaults to false per Invariant XI)
    #[serde(default)]
    pub block_on_legacy_debt: bool,
    /// If true, missing section fragments on links abort pre-commit admission (default true)
    #[serde(default = "default_true")]
    pub block_on_missing_fragments: bool,
}

impl Default for ReviewGatingConfig {
    fn default() -> Self {
        Self {
            enforce_doc_review: true,
            block_on_legacy_debt: false,
            block_on_missing_fragments: true,
        }
    }
}

/// Consolidated impact and review report for a Git change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AffectedDocumentsReport {
    pub changed_code_files: Vec<String>,
    pub obligations: Vec<ReviewObligation>,
    pub findings: Vec<DeterministicFinding>,
    pub pending_count: u32,
    pub reviewed_count: u32,
    pub blocked_count: u32,
}

