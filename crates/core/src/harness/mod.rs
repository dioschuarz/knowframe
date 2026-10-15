//! AI-Oriented Documentation Governance module.
//!
//! Models, audits, and evolves AI Harness Entry Points, embedded Knowledge Index
//! routes, secondary routing layer detection, reusable skills, procedure candidates,
//! and directory metrics according to CONSTITUTION.md v1.6.0.

use serde::{Deserialize, Serialize};
use std::fmt;

pub mod directory_metrics;
pub mod discovery;
pub mod index_parser;
pub mod matrix;
pub mod procedure;
pub mod secondary_layer;
pub mod skill;

// Calibrated Reference Threshold Constants for Evidence-Based Signal Detection (Invariant XIII):
pub const MAX_NAVIGATION_DEPTH_THRESHOLD: u32 = 5;
pub const CONCENTRATION_ENTROPY_THRESHOLD: f32 = 0.35;
pub const DOMAIN_MIXING_THRESHOLD: f32 = 0.60;
pub const CROSS_FOLDER_COUPLING_THRESHOLD: f32 = 0.75;
pub const LOW_CONTEXT_EFFICIENCY_THRESHOLD: f32 = 0.40;

/// Type of AI Harness Entry Point supported deterministically.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HarnessType {
    #[serde(rename = "agents_md")]
    AgentsMd,
    #[serde(rename = "claude_md")]
    ClaudeMd,
    #[serde(rename = "gemini_md")]
    GeminiMd,
    #[serde(rename = "copilot_instructions")]
    CopilotInstructions,
    #[serde(rename = "custom")]
    Custom,
}

impl HarnessType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HarnessType::AgentsMd => "agents_md",
            HarnessType::ClaudeMd => "claude_md",
            HarnessType::GeminiMd => "gemini_md",
            HarnessType::CopilotInstructions => "copilot_instructions",
            HarnessType::Custom => "custom",
        }
    }

    pub fn default_filename(&self) -> Option<&'static str> {
        match self {
            HarnessType::AgentsMd => Some("AGENTS.md"),
            HarnessType::ClaudeMd => Some("CLAUDE.md"),
            HarnessType::GeminiMd => Some("GEMINI.md"),
            HarnessType::CopilotInstructions => Some(".github/copilot-instructions.md"),
            HarnessType::Custom => None,
        }
    }

    pub fn from_path(path: &str) -> Self {
        let normalized = path.replace('\\', "/");
        if normalized.ends_with("AGENTS.md") || normalized == "AGENTS.md" {
            HarnessType::AgentsMd
        } else if normalized.ends_with("CLAUDE.md") || normalized == "CLAUDE.md" {
            HarnessType::ClaudeMd
        } else if normalized.ends_with("GEMINI.md") || normalized == "GEMINI.md" {
            HarnessType::GeminiMd
        } else if normalized.ends_with(".github/copilot-instructions.md")
            || normalized == ".github/copilot-instructions.md"
        {
            HarnessType::CopilotInstructions
        } else {
            HarnessType::Custom
        }
    }
}

/// Status of resolving an index route target against vault documents and anchors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RouteResolutionStatus {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "broken_file")]
    BrokenFile,
    #[serde(rename = "broken_anchor")]
    BrokenAnchor,
    #[serde(rename = "excluded_path")]
    ExcludedPath,
}

/// Canonical governance transformation strategies.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GovernanceStrategyType {
    #[serde(rename = "embed_knowledge_index")]
    EmbedKnowledgeIndex,
    #[serde(rename = "eliminate_secondary_routing_layer")]
    EliminateSecondaryRoutingLayer,
    #[serde(rename = "create_skill_from_procedure")]
    CreateSkillFromProcedure,
    #[serde(rename = "consolidate_duplicate_skills")]
    ConsolidateDuplicateSkills,
    #[serde(rename = "decompose_harness")]
    DecomposeHarness,
    #[serde(rename = "introduce_section_addressing")]
    IntroduceSectionAddressing,
    #[serde(rename = "restructure_directory_domain")]
    RestructureDirectoryDomain,
    #[serde(rename = "sync_harness_routes")]
    SyncHarnessRoutes,
}

impl GovernanceStrategyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GovernanceStrategyType::EmbedKnowledgeIndex => "embed_knowledge_index",
            GovernanceStrategyType::EliminateSecondaryRoutingLayer => {
                "eliminate_secondary_routing_layer"
            }
            GovernanceStrategyType::CreateSkillFromProcedure => "create_skill_from_procedure",
            GovernanceStrategyType::ConsolidateDuplicateSkills => "consolidate_duplicate_skills",
            GovernanceStrategyType::DecomposeHarness => "decompose_harness",
            GovernanceStrategyType::IntroduceSectionAddressing => "introduce_section_addressing",
            GovernanceStrategyType::RestructureDirectoryDomain => "restructure_directory_domain",
            GovernanceStrategyType::SyncHarnessRoutes => "sync_harness_routes",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().replace('-', "_").as_str() {
            "embed_knowledge_index" => Some(GovernanceStrategyType::EmbedKnowledgeIndex),
            "eliminate_secondary_routing_layer" => {
                Some(GovernanceStrategyType::EliminateSecondaryRoutingLayer)
            }
            "create_skill_from_procedure" => {
                Some(GovernanceStrategyType::CreateSkillFromProcedure)
            }
            "consolidate_duplicate_skills" => {
                Some(GovernanceStrategyType::ConsolidateDuplicateSkills)
            }
            "decompose_harness" => Some(GovernanceStrategyType::DecomposeHarness),
            "introduce_section_addressing" => {
                Some(GovernanceStrategyType::IntroduceSectionAddressing)
            }
            "restructure_directory_domain" => {
                Some(GovernanceStrategyType::RestructureDirectoryDomain)
            }
            "sync_harness_routes" | "sync_harnesses" => {
                Some(GovernanceStrategyType::SyncHarnessRoutes)
            }
            _ => None,
        }
    }
}

/// An individual navigational route declared within the embedded Knowledge Index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeIndexRoute {
    pub domain: String,
    pub label: String,
    pub target_file: String,
    pub target_anchor: Option<String>,
    pub resolution_status: RouteResolutionStatus,
    pub hop_distance_from_entry: u32,
    pub target_tokens: usize,
    pub line_number: usize,
}

/// Metadata extracted from the `## Knowledge Index` AST section.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddedIndexMetadata {
    pub heading_anchor: String,
    pub start_line: usize,
    pub end_line: usize,
    pub routes: Vec<KnowledgeIndexRoute>,
    pub domain_coverage_ratio: f32,
}

/// Quality ratings computed for a Harness Entry Point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HarnessQualityRatings {
    pub integrity_score: f32,
    pub structure_score: f32,
    pub consistency_score: f32,
    pub discoverability_score: f32,
    pub freshness_score: f32,
    pub context_efficiency: f32,
}

impl Default for HarnessQualityRatings {
    fn default() -> Self {
        Self {
            integrity_score: 100.0,
            structure_score: 100.0,
            consistency_score: 100.0,
            discoverability_score: 100.0,
            freshness_score: 100.0,
            context_efficiency: 100.0,
        }
    }
}

/// An intermediate or secondary routing document detected as a structural anti-pattern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecondaryRoutingLayer {
    pub file_path: String,
    pub parent_harness: String,
    pub outgoing_routes_count: usize,
    pub intermediate_hops: u32,
    pub penalty_points: f32,
    pub description: String,
    pub recommendation_hint: String,
}

/// Configured or auto-detected AI agent harness entry point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HarnessEntryPoint {
    pub file_path: String,
    pub harness_type: HarnessType,
    pub is_primary: bool,
    pub total_tokens: usize,
    pub heading_count: usize,
    pub embedded_index: Option<EmbeddedIndexMetadata>,
    pub secondary_layers_detected: Vec<SecondaryRoutingLayer>,
    pub quality_ratings: HarnessQualityRatings,
    pub last_modified: Option<i64>,
}

/// 4-capability quality matrix synthesized for AI-Oriented mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AIOrientedCapabilityMatrix {
    pub entry_score: f32,
    pub discovery_score: f32,
    pub retrieval_score: f32,
    pub reuse_score: f32,
    pub overall_governance_score: f32,
}

impl Default for AIOrientedCapabilityMatrix {
    fn default() -> Self {
        Self {
            entry_score: 100.0,
            discovery_score: 100.0,
            retrieval_score: 100.0,
            reuse_score: 100.0,
            overall_governance_score: 100.0,
        }
    }
}

/// Skill validation status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillValidationStatus {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "missing_manifest")]
    MissingManifest,
    #[serde(rename = "broken_asset_reference")]
    BrokenAssetReference,
    #[serde(rename = "duplicate_capability")]
    DuplicateCapability,
}

/// Skill asset types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillAssetType {
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "reference")]
    Reference,
}

/// Supporting asset associated with a Skill.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillAsset {
    pub asset_type: SkillAssetType,
    pub file_path: String,
    pub exists_on_disk: bool,
    pub byte_size: u64,
}

/// A reusable operational capability defined in `.agents/skills/<skill_id>/SKILL.md`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillDefinition {
    pub skill_id: String,
    pub dir_path: String,
    pub manifest_path: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub assets: Vec<SkillAsset>,
    pub status: SkillValidationStatus,
    pub unreferenced: bool,
    pub overlap_ids: Vec<String>,
}

/// Recurring multi-step procedure candidate detected across documents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcedureCandidate {
    pub candidate_id: String,
    pub step_signatures: Vec<String>,
    pub occurrence_count: usize,
    pub source_documents: Vec<String>,
    pub associated_scripts: Vec<String>,
    pub recurrence_score: f32,
    pub recommended_skill_name: String,
}

/// Types of physical directory organizational signals.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrganizationalSignalType {
    #[serde(rename = "excessive_depth")]
    ExcessiveDepth,
    #[serde(rename = "document_overconcentration")]
    DocumentOverconcentration,
    #[serde(rename = "scattered_related_knowledge")]
    ScatteredRelatedKnowledge,
    #[serde(rename = "high_navigational_coupling")]
    HighNavigationalCoupling,
}

/// Signal emitted when physical directory organization exhibits measurable degradation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationalSignal {
    pub target_directory: String,
    pub signal_type: OrganizationalSignalType,
    pub message: String,
    pub cost_evidence: String,
}

/// Physical directory and file structure metrics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectoryStructureMetrics {
    pub max_directory_depth: u32,
    pub average_directory_depth: f32,
    pub concentration_entropy: f32,
    pub domain_mixing_score: f32,
    pub cross_folder_coupling: f32,
    pub organizational_signals: Vec<OrganizationalSignal>,
}

/// Expected benefits and countervailing trade-offs of a governance strategy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GovernanceTradeOffMatrix {
    pub expected_benefits: Vec<String>,
    pub countervailing_trade_offs: Vec<String>,
}

/// Projected impact deltas of a governance transformation plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectedImpact {
    pub delta_tokens: i64,
    pub delta_hops: i32,
    pub delta_governance_score: f32,
}

/// A structural transformation recommendation for AI documentation governance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HarnessEvolutionRecommendation {
    pub plan_id: String,
    pub approval_token: String,
    pub strategy_type: GovernanceStrategyType,
    pub target_file_or_dir: String,
    pub title: String,
    pub description: String,
    pub unified_diff: String,
    pub trade_offs: GovernanceTradeOffMatrix,
    pub projected_impact: ProjectedImpact,
}

/// Governance subsystem errors.
#[derive(Debug)]
pub enum GovernanceError {
    MissingHarness,
    InvalidHarness(String),
    BrokenRoute { target: String, line: usize },
    SecondaryLayerViolation(String),
    SkillError(String),
    ApprovalTokenMismatch,
    PlanGenerationFailed(String),
    PlanNotFound(String),
    Io(std::io::Error),
}

impl fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GovernanceError::MissingHarness => write!(f, "No valid AI Harness Entry Point found"),
            GovernanceError::InvalidHarness(msg) => write!(f, "Invalid harness: {}", msg),
            GovernanceError::BrokenRoute { target, line } => {
                write!(f, "Broken route to '{}' at line {}", target, line)
            }
            GovernanceError::SecondaryLayerViolation(msg) => {
                write!(f, "Secondary routing layer violation: {}", msg)
            }
            GovernanceError::SkillError(msg) => write!(f, "Skill error: {}", msg),
            GovernanceError::ApprovalTokenMismatch => {
                write!(f, "Invalid or unverified approval token (Invariant XVI)")
            }
            GovernanceError::PlanGenerationFailed(msg) => {
                write!(f, "Governance plan generation failed: {}", msg)
            }
            GovernanceError::PlanNotFound(msg) => write!(f, "Governance plan not found: {}", msg),
            GovernanceError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for GovernanceError {}

impl From<std::io::Error> for GovernanceError {
    fn from(err: std::io::Error) -> Self {
        GovernanceError::Io(err)
    }
}

/// Evaluates Harness Entry Point quality ratings and context efficiency (FR-005..FR-011, Invariant XIII).
pub struct HarnessQualityAuditor;

impl HarnessQualityAuditor {
    pub fn audit_harness(entry: &HarnessEntryPoint, content: &str) -> HarnessQualityRatings {
        // 1. Integrity Score (broken files: -25, broken anchors: -15)
        let mut integrity_deductions = 0.0f32;
        if let Some(ref index) = entry.embedded_index {
            for route in &index.routes {
                match route.resolution_status {
                    RouteResolutionStatus::BrokenFile => integrity_deductions += 25.0,
                    RouteResolutionStatus::BrokenAnchor => integrity_deductions += 15.0,
                    RouteResolutionStatus::ExcludedPath => integrity_deductions += 5.0,
                    RouteResolutionStatus::Resolved => {}
                }
            }
        } else {
            integrity_deductions += 30.0;
        }
        let integrity_score = (100.0f32 - integrity_deductions).clamp(0.0, 100.0);

        // 2. Structure Score & Context Efficiency
        let mut structure_deductions = 0.0f32;
        for sec in &entry.secondary_layers_detected {
            structure_deductions += sec.penalty_points;
        }

        let total_tokens = if entry.total_tokens == 0 {
            crate::tokens::counter::count_bpe_tokens(content).unwrap_or(0) as usize
        } else {
            entry.total_tokens
        };

        let useful_tokens = if let Some(ref index) = entry.embedded_index {
            let lines: Vec<&str> = content.lines().collect();
            let start = index.start_line.saturating_sub(1);
            let end = index.end_line.min(lines.len());
            let mut index_text = String::new();
            if start < end {
                for line in &lines[start..end] {
                    index_text.push_str(line);
                    index_text.push('\n');
                }
            }
            crate::tokens::counter::count_bpe_tokens(&index_text).unwrap_or(0) as usize
        } else {
            0
        };

        let context_efficiency = if total_tokens == 0 {
            100.0
        } else {
            ((useful_tokens as f32 / total_tokens as f32) * 100.0).clamp(0.0, 100.0)
        };

        // Invariant XIII: Non-prescriptive size evaluation.
        // Large entry points (>20,000 tokens) ONLY trigger structural penalty if accompanied
        // by poor context efficiency (< 40%) AND broken routes/dead hops.
        if total_tokens > 20_000
            && (context_efficiency < (LOW_CONTEXT_EFFICIENCY_THRESHOLD * 100.0)
                && integrity_deductions > 0.0)
        {
            structure_deductions += 20.0;
        }
        let structure_score = (100.0f32 - structure_deductions).clamp(0.0, 100.0);

        // 3. Consistency Score (duplicate routes / targets)
        let mut consistency_deductions = 0.0f32;
        if let Some(ref index) = entry.embedded_index {
            let mut seen_routes = std::collections::HashSet::new();
            for route in &index.routes {
                let key = format!(
                    "{}:{}",
                    route.target_file,
                    route.target_anchor.as_deref().unwrap_or("")
                );
                if !seen_routes.insert(key) {
                    consistency_deductions += 10.0;
                }
            }
        }
        let consistency_score = (100.0f32 - consistency_deductions).clamp(0.0, 100.0);

        // 4. Discoverability Score
        let discoverability_score = if let Some(ref index) = entry.embedded_index {
            (index.domain_coverage_ratio * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };

        // 5. Freshness Score
        let freshness_score = 100.0;

        HarnessQualityRatings {
            integrity_score,
            structure_score,
            consistency_score,
            discoverability_score,
            freshness_score,
            context_efficiency,
        }
    }
}
