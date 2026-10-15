---
wiki:
  content_hash: fd80d25d75babcc66db3fe9d34ea1dff1061a329d1f5a7e6529f0294fa045fd6
  fragments:
    data-model-deterministic-knowledge-engineering-engine:
      tokens: 3297
      inbound: 0
      outbound: 0
    1-rust-domain-models-cratescoresrcmodel:
      tokens: 1952
      inbound: 0
      outbound: 0
    11-core-epistemic-profile-enums:
      tokens: 261
      inbound: 0
      outbound: 0
    12-optimization-profile-configuration:
      tokens: 304
      inbound: 0
      outbound: 0
    13-knowledge-graph-document-model:
      tokens: 324
      inbound: 0
      outbound: 0
    14-structural-findings-health-scoring:
      tokens: 392
      inbound: 0
      outbound: 0
    15-strategy-recommendations-user-controlled-governance:
      tokens: 655
      inbound: 0
      outbound: 0
    2-typescript-domain-models-extensionsrctypesenginets:
      tokens: 1284
      inbound: 0
      outbound: 0
---
# Data Model: Deterministic Knowledge Engineering Engine

**Feature**: `009-deterministic-knowledge-engine`  
**Date**: 2026-10-06  
**Status**: Completed  
**Sources**: `SPEC.md`, `CONSTITUTION.md` (v1.6.0)

---

## 1. Rust Domain Models (`crates/core/src/model/`)

### 1.1 Core Epistemic & Profile Enums

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicTier {
    TierA_FullyDeterministic,
    TierB_CandidateGeneration,
    TierC_DecisionSupport,
    TierD_NotReliablyDeterminable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionMode {
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
```

### 1.2 Optimization Profile & Configuration

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationProfile {
    pub mode: EvolutionMode,
    pub profile_version: String,
    pub weights: ProfileWeights,
    pub thresholds: ProfileThresholds,
    pub constraints: ProfileConstraints,
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
```

### 1.3 Knowledge Graph & Document Model

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentNode {
    pub id: DocumentId,
    pub relative_path: String,
    pub title: String,
    pub raw_token_count: u64,
    pub frontmatter_schema: Option<String>,
    pub fragments: Vec<FragmentMetadata>,
    pub outgoing_references: Vec<ReferenceEdge>,
    pub git_last_modified_commit: String,
    pub git_last_modified_timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentMetadata {
    pub anchor: String,
    pub heading_text: String,
    pub heading_level: u8,
    pub line_start: usize,
    pub line_end: usize,
    pub raw_token_count: u64,
    pub code_citations: Vec<CodeCitation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeCitation {
    pub target_file_path: String,
    pub symbol_name: Option<String>,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceEdge {
    pub target_document: DocumentId,
    pub target_fragment: Option<String>,
    pub relation_type: Option<String>, // e.g. "broader", "narrower", "depends_on"
    pub is_resolved: bool,
    pub line_number: usize,
}
```

### 1.4 Structural Findings & Health Scoring

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralFinding {
    pub id: String,
    pub methodology: Methodology,
    pub pillar: QualityPillar,
    pub tier: EpistemicTier,
    pub severity: FindingSeverity,
    pub document_id: DocumentId,
    pub fragment_anchor: Option<String>,
    pub line_number: usize,
    pub rule_key: String,
    pub message: String,
    pub evidence_metrics: serde_json::Value,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PillarHealth {
    pub raw_score: f64, // 0.0 .. 100.0
    pub legacy_debt_count: u64,
    pub new_regressions_count: u64,
    pub findings: Vec<StructuralFinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthScore {
    pub analysis_id: String,
    pub aggregate_score: f64, // 0.0 .. 100.0
    pub profile_mode: EvolutionMode,
    pub integrity: PillarHealth,
    pub structure: PillarHealth,
    pub consistency: PillarHealth,
    pub discoverability: PillarHealth,
    pub freshness: PillarHealth,
}
```

### 1.5 Strategy Recommendations & User-Controlled Governance

```rust
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
pub struct GraphTopology {
    pub nodes: Vec<DocumentNode>,
    pub edges: Vec<ReferenceEdge>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyRecommendation {
    pub strategy_id: String,
    pub strategy_type: StrategyType,
    pub role: StrategyRole,
    pub target_documents: Vec<DocumentId>,
    pub fit_score: f64, // 0.0 .. 1.0 via AHP
    pub implementation_cost: f64, // Normalized cost based on predicted mutations/churn
    pub expected_benefits: Vec<String>,
    pub countervailing_trade_offs: Vec<String>,
    pub hard_constraints_satisfied: bool,
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
#[serde(tag = "action_type", rename_all = "snake_case")]
pub enum TransformationAction {
    MoveFile { source: String, destination: String },
    UpdateFrontmatter { file: String, key: String, value: serde_json::Value },
    RetargetLink { file: String, line: usize, old_target: String, new_target: String },
    ExtractSectionToFile { source_file: String, anchor: String, new_file: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReMeasurementDelta {
    pub aggregate_score_delta: f64,
    pub raw_token_delta: i64,
    pub graph_diameter_delta: Option<i32>,
    pub pillar_deltas: PillarDeltas,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PillarDeltas {
    pub integrity: f64,
    pub structure: f64,
    pub consistency: f64,
    pub discoverability: f64,
    pub freshness: f64,
}
```

---

## 2. TypeScript Domain Models (`extension/src/types/engine.ts`)

```typescript
export type EpistemicTier =
  | 'tier_a_fully_deterministic'
  | 'tier_b_candidate_generation'
  | 'tier_c_decision_support'
  | 'tier_d_not_reliably_determinable';

export type EvolutionMode =
  | 'ai_oriented'
  | 'human_oriented'
  | 'balanced'
  | 'custom';

export type Methodology =
  | 'classification'
  | 'taxonomy'
  | 'faceted_classification'
  | 'controlled_vocabulary'
  | 'thesaurus'
  | 'semantic_networks'
  | 'conceptual_maps'
  | 'ontology_engineering'
  | 'domain_analysis'
  | 'metadata_application_profiles'
  | 'information_architecture'
  | 'information_retrieval'
  | 'terminology_management'
  | 'knowledge_graph_network_analysis'
  | 'knowledge_quality'
  | 'provenance'
  | 'knowledge_drift'
  | 'context_routing';

export type QualityPillar =
  | 'integrity'
  | 'structure'
  | 'consistency'
  | 'discoverability'
  | 'freshness';

export type FindingSeverity =
  | 'critical'
  | 'high'
  | 'medium'
  | 'low'
  | 'informational';

export interface StructuralFinding {
  id: string;
  methodology: Methodology;
  pillar: QualityPillar;
  tier: EpistemicTier;
  severity: FindingSeverity;
  document_id: string;
  fragment_anchor?: string;
  line_number: number;
  rule_key: string;
  message: string;
  evidence_metrics: Record<string, unknown>;
  remediation_hint: string;
}

export interface PillarHealth {
  raw_score: number;
  legacy_debt_count: number;
  new_regressions_count: number;
  findings: StructuralFinding[];
}

export interface HealthScore {
  analysis_id: string;
  aggregate_score: number;
  profile_mode: EvolutionMode;
  integrity: PillarHealth;
  structure: PillarHealth;
  consistency: PillarHealth;
  discoverability: PillarHealth;
  freshness: PillarHealth;
}

export interface GraphTopology {
  nodes: DocumentNode[];
  edges: ReferenceEdge[];
}

export interface ContextTrace {
  query: string;
  target_fragment?: string;
  trace_path: string[];
  hop_count: number;
  total_tokens: number;
  context_reduction_percent: number;
  relevance_score: number;
}

export type StrategyType =
  | 'preserve_document'
  | 'split_decompose'
  | 'extract_facet_metadata'
  | 'consolidate_index'
  | 'merge_candidates'
  | 'standardize_terminology';

export type StrategyRole =
  | 'primary'
  | 'supporting'
  | 'alternative'
  | 'mutually_exclusive';

export interface StrategyRecommendation {
  strategy_id: string;
  strategy_type: StrategyType;
  role: StrategyRole;
  target_documents: string[];
  fit_score: number;
  implementation_cost: number;
  expected_benefits: string[];
  countervailing_trade_offs: string[];
  hard_constraints_satisfied: boolean;
}

export type TransformationAction =
  | { action_type: 'move_file'; source: string; destination: string }
  | { action_type: 'update_frontmatter'; file: string; key: string; value: unknown }
  | { action_type: 'retarget_link'; file: string; line: number; old_target: string; new_target: string }
  | { action_type: 'extract_section_to_file'; source_file: string; anchor: string; new_file: string };

export interface TransformationPlan {
  plan_id: string;
  approval_token: string;
  strategy_id: string;
  target_files: string[];
  unified_diff: string;
  projected_score_delta: number;
  actions: TransformationAction[];
}

export interface OptimizationProfile {
  mode: EvolutionMode;
  profile_version: string;
  weights: ProfileWeights;
  thresholds: ProfileThresholds;
  constraints: ProfileConstraints;
}

export interface ProfileWeights {
  integrity: number;
  structure: number;
  consistency: number;
  discoverability: number;
  freshness: number;
  context_efficiency: number;
  navigation_depth: number;
  vocabulary_adherence: number;
}

export interface ProfileThresholds {
  max_document_tokens: number;
  max_fragment_tokens: number;
  max_navigation_depth: number;
  min_scent_similarity: number;
  max_betweenness_centrality: number;
  min_category_coverage: number;
  max_taxonomy_imbalance: number;
}

export interface ProfileConstraints {
  never_auto_split_monoliths: boolean;
  enforce_controlled_tags: boolean;
  require_dcap_frontmatter: boolean;
  prohibit_circular_thesaurus: boolean;
  isolated_directories: string[];
}

export interface CodeCitation {
  target_file_path: string;
  symbol_name?: string;
  line_number?: number;
}

export interface FragmentMetadata {
  anchor: string;
  heading_text: string;
  heading_level: number;
  line_start: number;
  line_end: number;
  raw_token_count: number;
  code_citations: CodeCitation[];
}

export interface ReferenceEdge {
  target_document: string;
  target_fragment?: string;
  relation_type?: string;
  is_resolved: boolean;
  line_number: number;
}

export interface DocumentNode {
  id: string;
  relative_path: string;
  title: string;
  raw_token_count: number;
  frontmatter_schema?: string;
  fragments: FragmentMetadata[];
  outgoing_references: ReferenceEdge[];
  git_last_modified_commit: string;
  git_last_modified_timestamp: number;
}

export interface PillarDeltas {
  integrity: number;
  structure: number;
  consistency: number;
  discoverability: number;
  freshness: number;
}

export interface ReMeasurementDelta {
  aggregate_score_delta: number;
  raw_token_delta: number;
  graph_diameter_delta?: number;
  pillar_deltas: PillarDeltas;
}
```
