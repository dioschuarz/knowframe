/**
 * Admission Control & Knowledge Quality Protocol Types
 * Matching contracts/admission_control_protocol.json
 */

export interface StagedFileItem {
  path: string;
  status: 'added' | 'modified' | 'deleted' | 'renamed';
  old_path?: string;
}

export interface ValidateCandidateParams {
  staged_files: StagedFileItem[];
  base_commit?: string;
}

export type AdmissionDecision = 'pass' | 'warn' | 'block';

export interface DebtAccountingSummary {
  total_debt: number;
  legacy_debt: number;
  new_debt: number;
  resolved_debt: number;
  regressions: number;
}

export interface RegressionItem {
  id: string;
  category:
    | 'broken_reference'
    | 'invalid_heading_anchor'
    | 'semantic_collision'
    | 'technical_drift_contradiction';
  severity: 'blocker' | 'warning';
  file_path: string;
  line_number?: number;
  message: string;
  remediation_hint: string;
}

export interface ResolvedItem {
  id: string;
  category: string;
  file_path: string;
  description: string;
}

export interface CandidateValidationReport {
  base_commit: string;
  decision: AdmissionDecision;
  exit_code: 0 | 1;
  summary: DebtAccountingSummary;
  regressions: RegressionItem[];
  resolved_findings: ResolvedItem[];
  warnings: string[];
}

export interface KnowledgeDebtBaselineDetails {
  baseline_commit: string;
  timestamp: string;
  broken_links_count: number;
  orphan_nodes_count: number;
  collisions_count: number;
  drifts_count: number;
  total_debt: number;
}

export interface UpdateBaselineParams {
  force_new_commit?: string;
}

export interface UpdateBaselineResult {
  baseline_commit: string;
  total_debt: number;
  previous_total_debt: number;
}

export interface ContextTraceParams {
  query: string;
  entrypoint?: string;
}

export interface ContextTraceResult {
  query: string;
  entrypoint: string;
  target_node: string;
  path: string[];
  depth: number;
  path_quality_score: number;
  cumulative_bpe_tokens: number;
  relevant_tokens: number;
  weak_intermediate_nodes: string[];
  latency_ms: number;
}

export interface QualityDegradationFactor {
  category: string;
  affected_node: string;
  betweenness_weight: number;
  deduction_points: number;
  description: string;
}

export interface KnowledgeQualityScore {
  overall_score: number;
  integrity_score: number;
  consistency_score: number;
  discoverability_score: number;
  freshness_score: number;
  legacy_debt_count: number;
  regressions_count: number;
  resolved_debt_count?: number;
  central_hub_id?: string;
  top_degradation_factors: QualityDegradationFactor[];
}

export interface KnowledgeIssue {
  id: string;
  pillar: "integrity" | "consistency" | "discoverability" | "freshness" | string;
  category: string;
  file_path: string;
  line_number?: number | null;
  target?: string | null;
  deduction_points: number;
  weight: number;
  description: string;
  remediation_hint: string;
}

export interface KnowledgeIssuesReport {
  overall_score: number;
  total_issues: number;
  integrity_count: number;
  consistency_count: number;
  discoverability_count: number;
  freshness_count: number;
  issues: KnowledgeIssue[];
}
