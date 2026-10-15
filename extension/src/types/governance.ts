/**
 * Domain types and RPC structures for AI-Oriented Documentation Governance.
 *
 * Constitution Authority: CONSTITUTION.md v1.6.0 (The 16 Critical Invariants)
 * Contract Source: specs/008-ai-doc-governance/contracts/ai_governance_rpc_protocol.json
 */

export type HarnessType =
  | 'agents_md'
  | 'claude_md'
  | 'gemini_md'
  | 'copilot_instructions'
  | 'custom';

export type RouteResolutionStatus =
  | 'resolved'
  | 'broken_file'
  | 'broken_anchor'
  | 'excluded_path';

export type GovernanceStrategyType =
  | 'embed_knowledge_index'
  | 'eliminate_secondary_routing_layer'
  | 'create_skill_from_procedure'
  | 'consolidate_duplicate_skills'
  | 'decompose_harness'
  | 'introduce_section_addressing'
  | 'restructure_directory_domain'
  | 'sync_harness_routes';

export interface KnowledgeIndexRoute {
  domain: string;
  label: string;
  target_file: string;
  target_anchor: string | null;
  resolution_status: RouteResolutionStatus;
  hop_distance_from_entry: number;
  target_tokens: number;
  line_number: number;
}

export interface EmbeddedIndexMetadata {
  heading_anchor: string;
  start_line: number;
  end_line: number;
  routes: KnowledgeIndexRoute[];
  domain_coverage_ratio: number;
}

export interface SecondaryRoutingLayer {
  file_path: string;
  parent_harness: string;
  outgoing_routes_count: number;
  intermediate_hops: number;
  penalty_points: number;
  description: string;
  recommendation_hint: string;
}

export interface HarnessQualityRatings {
  integrity_score: number;
  structure_score: number;
  consistency_score: number;
  discoverability_score: number;
  freshness_score: number;
  context_efficiency: number;
}

export interface HarnessEntryPoint {
  file_path: string;
  harness_type: HarnessType;
  is_primary: boolean;
  total_tokens: number;
  heading_count: number;
  embedded_index: EmbeddedIndexMetadata | null;
  secondary_layers_detected: SecondaryRoutingLayer[];
  quality_ratings: HarnessQualityRatings;
  last_modified?: number | null;
}

export interface AIOrientedCapabilityMatrix {
  entry_score: number;
  discovery_score: number;
  retrieval_score: number;
  reuse_score: number;
  overall_governance_score: number;
}

export type SkillValidationStatus =
  | 'valid'
  | 'missing_manifest'
  | 'broken_asset_reference'
  | 'duplicate_capability';

export type SkillAssetType = 'script' | 'template' | 'reference';

export interface SkillAsset {
  asset_type: SkillAssetType;
  file_path: string;
  exists_on_disk: boolean;
  byte_size: number;
}

export interface SkillDefinition {
  skill_id: string;
  dir_path: string;
  manifest_path: string;
  name: string;
  description: string;
  assets?: SkillAsset[];
  status: SkillValidationStatus;
  unreferenced: boolean;
  overlap_ids: string[];
}

export interface ProcedureCandidate {
  candidate_id: string;
  step_signatures: string[];
  occurrence_count: number;
  source_documents: string[];
  associated_scripts: string[];
  recurrence_score: number;
  recommended_skill_name: string;
}

export type OrganizationalSignalType =
  | 'excessive_depth'
  | 'document_overconcentration'
  | 'scattered_related_knowledge'
  | 'high_navigational_coupling';

export interface OrganizationalSignal {
  target_directory: string;
  signal_type: string;
  message: string;
  cost_evidence: string;
}

export interface DirectoryStructureMetrics {
  max_directory_depth: number;
  average_directory_depth: number;
  concentration_entropy: number;
  domain_mixing_score: number;
  cross_folder_coupling: number;
  organizational_signals: OrganizationalSignal[];
}

export interface GovernanceTradeOffMatrix {
  expected_benefits: string[];
  countervailing_trade_offs: string[];
}

export interface ProjectedImpact {
  delta_tokens: number;
  delta_hops: number;
  delta_governance_score: number;
}

export interface HarnessEvolutionRecommendation {
  plan_id: string;
  approval_token: string;
  strategy_type: GovernanceStrategyType;
  target_file_or_dir: string;
  title: string;
  description: string;
  unified_diff: string;
  trade_offs: GovernanceTradeOffMatrix;
  projected_impact: ProjectedImpact;
}

export interface GovernancePlan {
  plan_id: string;
  approval_token: string;
  strategy_type: GovernanceStrategyType;
  target_file_or_dir: string;
  title: string;
  description: string;
  unified_diff: string;
  expected_benefits: string[];
  countervailing_trade_offs: string[];
  projected_delta_tokens: number;
  projected_delta_hops: number;
  actions?: any[];
}

export interface GovernanceApplyResult {
  applied: boolean;
  plan_id: string;
  files_modified: string[];
  re_measurement: {
    delta_tokens: number;
    delta_hops: number;
    delta_governance_score: number;
  };
}

// Calibration Thresholds (Invariant XIII)
export const MAX_NAVIGATION_DEPTH_THRESHOLD = 5;
export const CONCENTRATION_ENTROPY_THRESHOLD = 0.35;
export const DOMAIN_MIXING_THRESHOLD = 0.60;
export const CROSS_FOLDER_COUPLING_THRESHOLD = 0.75;
export const LOW_CONTEXT_EFFICIENCY_THRESHOLD = 0.40;

export type HarnessToolCategory = 'core' | 'agent' | 'runtime';

export interface HarnessToolStatus {
  id: string;
  name: string;
  category: HarnessToolCategory;
  description: string;
  binary: string;
  is_installed: boolean;
  version?: string | null;
  bin_path?: string | null;
  install_command: string;
  required_for?: string;
  docs_url?: string;
}

