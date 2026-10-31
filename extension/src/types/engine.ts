/**
 * Domain types and contracts for Feature 009: Deterministic Knowledge Engineering Engine.
 * Matches contracts/knowledge_engine_protocol.json and specs/009-deterministic-knowledge-engine/data-model.md
 */

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
  | 'traceability'
  | 'freshness';

export type FindingSeverity =
  | 'critical'
  | 'high'
  | 'medium'
  | 'low'
  | 'informational';

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

export interface GraphTopology {
  nodes: DocumentNode[];
  edges: ReferenceEdge[];
}

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
  traceability?: PillarHealth;
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

export interface PillarDeltas {
  integrity: number;
  structure: number;
  consistency: number;
  discoverability: number;
  freshness: number;
  traceability?: number;
}

export interface ReMeasurementDelta {
  aggregate_score_delta: number;
  raw_token_delta: number;
  graph_diameter_delta?: number;
  pillar_deltas: PillarDeltas;
}

export interface ProfileWeights {
  integrity: number;
  structure: number;
  consistency: number;
  discoverability: number;
  freshness: number;
  traceability?: number;
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

export interface OptimizationProfile {
  mode: EvolutionMode;
  profile_version: string;
  weights: ProfileWeights;
  thresholds: ProfileThresholds;
  constraints: ProfileConstraints;
}

// Feature 010: Deterministic Traceability & Review Types

export type ReviewStatus = 'updated' | 'reviewed_unchanged' | 'pending' | 'blocked';

export type DependencySource = 'frontmatter' | 'inline_markdown_link' | 'inline_symbol_citation';

export type DeterministicFindingSeverity = 'info' | 'warning' | 'error';

export interface FindingEvidence {
  changed_paths?: string[];
  dependency_sources?: string[];
  rule_description: string;
  line_number?: number | null;
  anchor?: string | null;
  target?: string | null;
}

export interface DeterministicFinding {
  finding_id: string;
  rule_id: string;
  severity: DeterministicFindingSeverity;
  subject: string;
  evidence: FindingEvidence;
  required_action: string;
  epistemic_tier?: EpistemicTier | null;
}

export interface ReviewObligation {
  doc_id: string;
  status: ReviewStatus;
  triggering_code_files: string[];
  dependency_sources: DependencySource[];
  resolution_receipt?: string | null;
}

export interface AffectedDocumentsReport {
  changed_code_files: string[];
  obligations: ReviewObligation[];
  findings: DeterministicFinding[];
  pending_count: number;
  reviewed_count: number;
  blocked_count: number;
}

export interface GetAffectedDocumentsParams {
  changed_paths?: string[];
  git_range?: string;
  include_unchanged_reviews?: boolean;
}

export interface ResolveDocumentReviewParams {
  doc_ids?: string[];
  resolved_reviews?: string[];
  receipt_source?: 'user_action' | 'commit_trailer' | 'ci_check';
  commit_hash?: string;
}

export interface ResolveDocumentReviewResult {
  resolved_count: number;
  receipt_id: string;
}

