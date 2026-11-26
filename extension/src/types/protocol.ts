/**
 * Strongly typed JSON-RPC 2.0 protocol definitions matching contracts/
 */

export interface JsonRpcRequest<T = unknown> {
  jsonrpc: '2.0';
  id: string | number;
  method: string;
  params?: T;
}

export interface JsonRpcResponse<T = unknown> {
  jsonrpc: '2.0';
  id: string | number;
  result?: T;
  error?: JsonRpcError;
}

export interface JsonRpcError {
  code: number;
  message: string;
  data?: unknown;
}

export interface JsonRpcNotification<T = unknown> {
  jsonrpc: '2.0';
  method: string;
  params: T;
}

export interface NodePayload {
  id: string;
  label: string;
  file_path: string;
  tags: string[];
  color: string;
  status: 'active' | 'connected_inactive' | 'orphan' | 'excluded';
  tokens: number;
  hop_distance: number;
  in_degree: number;
  out_degree: number;
  betweenness: number;
  broken_links: number;
}

export type NonClickableReason =
  | 'wikilink_syntax'
  | 'missing_extension'
  | 'non_relative_path'
  | 'case_mismatch'
  | 'unencoded_spaces'
  | 'unslugified_anchor';

export interface EdgePayload {
  id: string;
  source: string;
  target: string;
  target_fragment?: string | null;
  is_broken: boolean;
  link_type: 'markdown' | 'wikilink';
  anchor?: string | null;
  line?: number | null;
  resolution_status?:
    | 'resolved'
    | 'resolved_excluded'
    | 'broken_missing_file'
    | 'broken_missing_anchor'
    | 'broken_fragment_anchor'
    | 'broken_wikilink';
  raw_target?: string | null;
  is_clickable?: boolean;
  non_clickable_reason?: NonClickableReason | null;
  clickable_target?: string | null;
}

export interface TokenSummaryPayload {
  active_harness_tokens: number;
  subcall_accessible_tokens: number;
  warning_threshold: number;
  critical_threshold: number;
  status: 'normal' | 'warning' | 'critical';
}

export interface ObservatorySummaryPayload {
  totalNotes: number;
  connectedNotes: number;
  orphanNotes: number;
  excludedNotes: number;
  brokenLinksCount: number;
  activeTokens: number;
  reachableTokens: number;
  syncStatus: 'synced' | 'syncing' | 'error';
  lastSyncTimestamp: number;
}

export interface ScanResultPayload {
  nodes: NodePayload[];
  edges: EdgePayload[];
  token_summary: TokenSummaryPayload;
  recommended_hub: string | null;
  observatory_summary?: ObservatorySummaryPayload;
}

export interface PreviewExclusionParams {
  pattern: string;
}

export interface PreviewExclusionResult {
  matchingCount: number;
  samplePaths: string[];
}

export interface ExcludePathParams {
  pattern: string;
  scope: 'scan' | 'graph';
}

export interface ExcludePathResult {
  success: boolean;
  affectedFilesCount: number;
  updatedSummary: {
    totalNotes: number;
    connectedNotes: number;
    orphanNotes: number;
    excludedNotes: number;
  };
}

export interface NoiseSuggestionItem {
  pattern: string;
  matchingCount: number;
  reason: string;
  recommended: boolean;
}

export interface SuggestExclusionsResult {
  suggestions: NoiseSuggestionItem[];
}

export interface SetCentralHubParams {
  hubPath: string;
}

export interface SetCentralHubResult {
  success: boolean;
  primaryHub: string;
}

export interface SetFocusModeParams {
  focusMode: 'hub' | '1_hop' | '2_hops' | 'all';
}

export interface FixBrokenLinkParams {
  sourcePath: string;
  targetPath: string;
  lineNumber: number;
}

export interface ChangedFileItem {
  path: string;
  status: 'added' | 'modified' | 'deleted';
}

export interface GitCommitDeltaPayload {
  repo_path: string;
  commit_hash?: string;
  branch?: string;
  changed_files: ChangedFileItem[];
}

export interface OpenFilePayload {
  file_path: string;
  line?: number;
}

export interface SetHubPayload {
  hub_id: string;
}

export interface UpdateColorGroupsPayload {
  color_groups: {
    query: string;
    color: string;
    label: string;
    enabled?: boolean;
  }[];
}

export type WebviewInboundType =
  | 'openFile'
  | 'setHub'
  | 'setCentralHub'
  | 'updateColorGroups'
  | 'fixBrokenLinks'
  | 'fixBrokenLink'
  | 'rescanVault'
  | 'requestGraphData'
  | 'triggerSetHubQuickPick'
  | 'previewExclusion'
  | 'excludePath'
  | 'removeExclusion'
  | 'requestExclusions'
  | 'suggestExclusions'
  | 'setFocusMode'
  | 'openSettings'
  | 'reviewIssues'
  | 'requestIssues'
  | 'navigateToDetail'
  | 'installHooks'
  | 'getEvolutionState'
  | 'setOrientation'
  | 'updateCustomConfig'
  | 'validateConstraints'
  | 'generateTransformationPlan'
  | 'applyTransformationPlan'
  | 'getStrategySet'
  | 'validateStrategySelection'
  | 'calculatePlanValue'
  | 'requestHarnessStatus'
  | 'getHarnessStatus'
  | 'setActiveHarness'
  | 'initializeHarness'
  | 'syncHarnesses'
  | 'configureHarnessBase'
  | 'getSkillCatalog'
  | 'analyzeDirectoryStructure'
  | 'generateGovernancePlan'
  | 'applyGovernancePlan'
  | 'installLLMWikiSkill'
  | 'resolveDocumentReview'
  | 'reviewWithAi'
  | 'reviewAllWithAi'
  | 'getAffectedDocuments'
  | 'requestHarnessTools'
  | 'installHarnessTool'
  | 'webviewReady';

export interface WebviewInboundMessage {
  requestId?: string;
  type: WebviewInboundType;
  payload?: unknown;
}

export interface LoadingProgressPayload {
  stage: 'initializing' | 'graph' | 'quality' | 'issues' | 'complete';
  percent: number;
  message: string;
}

export type WebviewOutboundType =
  | 'graphDataUpdate'
  | 'deltaUpdate'
  | 'tokenBudgetAlert'
  | 'sidecarStatus'
  | 'exclusionPreviewResult'
  | 'excludePathResult'
  | 'noiseSuggestions'
  | 'loadingProgress'
  | 'issuesReportUpdate'
  | 'openSettingsDrawer'
  | 'evolutionStateUpdate'
  | 'strategySetResult'
  | 'strategyValidationResult'
  | 'planValueResult'
  | 'transformationPlanReady'
  | 'transformationPlanApplied'
  | 'transformationPlanError'
  | 'transformationPlanApplyError'
  | 'constraintValidationResult'
  | 'harnessStatus'
  | 'harnessToolsUpdate'
  | 'skillCatalogUpdate'
  | 'directoryMetricsUpdate'
  | 'contextTraceResult'
  | 'affectedDocumentsReport'
  | 'documentReviewResolved'
  | 'error';

export interface WebviewOutboundMessage {
  requestId?: string;
  type: WebviewOutboundType;
  payload: unknown;
}

// ─────────────────────────────────────────────────────────────────────────────
// Admission Control & Knowledge Quality Protocol Interfaces (matching contracts/)
// ─────────────────────────────────────────────────────────────────────────────

export interface ValidateCandidateParams {
  staged_files: {
    path: string;
    status: 'added' | 'modified' | 'deleted';
  }[];
  base_commit?: string;
}

export interface RegressionItemPayload {
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

export interface ResolvedItemPayload {
  id: string;
  category: string;
  file_path: string;
  description: string;
}

export interface CandidateValidationReportPayload {
  base_commit: string;
  decision: 'pass' | 'warn' | 'block';
  exit_code: 0 | 1;
  summary: {
    total_debt: number;
    legacy_debt: number;
    new_debt: number;
    resolved_debt: number;
    regressions: number;
  };
  regressions: RegressionItemPayload[];
  resolved_findings: ResolvedItemPayload[];
  warnings: string[];
}

export interface GetBaselineResultPayload {
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

export interface UpdateBaselineResultPayload {
  success: boolean;
  baseline_commit: string;
  total_debt: number;
}

export interface TraceContextParams {
  query: string;
  entrypoint?: string;
}

export interface ContextTraceResultPayload {
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

export interface QualityDegradationFactorPayload {
  category: string;
  affected_node: string;
  betweenness_weight: number;
  deduction_points: number;
  description: string;
}

export interface KnowledgeQualityScorePayload {
  overall_score: number;
  integrity_score: number;
  consistency_score: number;
  discoverability_score: number;
  freshness_score: number;
  traceability_score?: number;
  legacy_debt_count: number;
  regressions_count: number;
  resolved_debt_count?: number;
  central_hub_id?: string;
  top_degradation_factors: QualityDegradationFactorPayload[];
}

// ==========================================
// 006 Management Experience & Evolution Types
// ==========================================

export type SyncStatus = 'synced' | 'syncing' | 'error' | 'uninitialized';

export interface HealthSummaryState {
  quality_score: number;
  integrity: number;
  structure: number;
  consistency: number;
  discoverability: number;
  freshness: number;
  traceability?: number;
  findings_count: number;
  broken_references: number;
  invalid_anchors: number;
  legacy_debt: number;
  new_regressions: number;
}

export interface StructureSummaryState {
  total_documents: number;
  connected_documents: number;
  isolated_documents: number;
  total_references: number;
}

export interface EvolutionSummaryState {
  orientation: 'ai_oriented' | 'human_oriented' | 'balanced' | 'custom';
  opportunity_count: number;
  primary_objective: string;
  conflicting_constraints: boolean;
}

export interface SyncSummaryState {
  status: SyncStatus;
  last_commit_sha: string | null;
  last_commit_time: string | null;
}

export interface ManagerSummaryState {
  health: HealthSummaryState;
  structure: StructureSummaryState;
  evolution: EvolutionSummaryState;
  sync: SyncSummaryState;
}

export type DrawerPrimaryTab = 'insights' | 'settings';
export type InsightsSubTab = 'health' | 'structure' | 'evolution';
export type SettingsSubTab =
  | 'knowledge'
  | 'exclusions'
  | 'clusters'
  | 'git'
  | 'harness'
  | 'git_harness';

export interface NavigationIntent {
  primaryTab: DrawerPrimaryTab;
  subTab: InsightsSubTab | SettingsSubTab;
  targetSection?: string;
  targetId?: string;
}

export interface ManagerViewState {
  health: {
    qualityScore: number;
    integrity: number;
    structure: number;
    consistency: number;
    discoverability: number;
    freshness: number;
    traceability?: number;
    findingsCount: number;
    brokenReferences: number;
    invalidAnchors: number;
    legacyDebt: number;
    newRegressions: number;
  };
  structure: {
    totalDocuments: number;
    connectedDocuments: number;
    isolatedDocuments: number;
    totalReferences: number;
  };
  evolution: {
    orientation: 'ai_oriented' | 'human_oriented' | 'balanced' | 'custom';
    orientationTitle: string;
    primaryObjective: string;
    opportunityCount: number;
    hasConflictingConstraints: boolean;
  };
  sync: {
    status: SyncStatus;
    lastCommitSha: string | null;
    lastCommitTime: string | null;
  };
}

export type ProblemSignature =
  | 'structurally_overloaded_document'
  | 'knowledge_fragmentation'
  | 'weak_discovery_path'
  | 'isolated_knowledge_silo';

export type EvolutionStrategy =
  | 'section_addressing'
  | 'topic_decomposition'
  | 'lightweight_index'
  | 'faceted_classification'
  | 'preserve_document'
  | 'hybrid';

export interface OpportunityEvidence {
  document_size_tokens: number;
  reference_concentration: number;
  cohesion: number;
  coupling: number;
  granularity: number;
  context_efficiency: number;
  discoverability: number;
  knowledge_reuse: number;
  semantic_diversity: number;
  navigational_complexity: number;
  maintenance_coupling: number;
}

export interface BenefitItem {
  metric: string;
  direction: 'increase' | 'decrease';
  description: string;
}

export interface TradeOffItem {
  metric: string;
  description: string;
}

export interface StrategyRecommendation {
  strategy: EvolutionStrategy;
  fit_score: number;
  confidence: number;
  why: string;
  expected_benefits: BenefitItem[];
  countervailing_trade_offs: TradeOffItem[];
  effort: 'low' | 'medium' | 'high';
  is_hybrid: boolean;
  hybrid_partner?: EvolutionStrategy | null;
}

export interface StructuralOpportunity {
  id: string;
  target_document: string;
  signature: ProblemSignature;
  confidence: number;
  narrative_why: string;
  evidence: OpportunityEvidence;
  candidate_strategies: StrategyRecommendation[];
}

export interface MetricDeltaComparison {
  metric_name: string;
  before_value: number;
  after_value: number;
  unit: string;
}

export interface ProposedActionPreview {
  action_type: 'create_document' | 'update_document_links' | 'inject_frontmatter' | 'delete_section';
  file_path: string;
  details: string;
}

export interface TransformationPlanPreview {
  plan_id: string;
  target_document: string;
  strategy: EvolutionStrategy;
  hybrid_partner?: EvolutionStrategy | null;
  proposed_actions: ProposedActionPreview[];
  expected_impact: MetricDeltaComparison[];
  trade_offs: string[];
}

export interface ReMeasurementReport {
  context_efficiency_delta_pct: number;
  trace_depth_delta_pct: number;
  maintenance_coupling_delta_pct: number;
  document_count_delta: number;
  reference_count_delta: number;
  narrative_summary: string;
}

export interface InfoBubbleContent {
  title: string;
  meaning: string;
  implications: {
    higher: string;
    lower: string;
  };
  whyItMatters: string;
  affects: string[];
  tradeOff: string;
}

export type InfoBubbleDictionary = Record<string, InfoBubbleContent>;

/**
 * Invariant X: Format raw numbers to compact display format strictly <= 5 chars.
 */
export function formatCompactNumber(n: number): string {
  if (n < 1000) {
    return n.toString();
  }
  if (n < 10000) {
    return (n / 1000).toFixed(1) + 'k';
  }
  if (n < 100000) {
    return Math.round(n / 1000) + 'k';
  }
  if (n < 1000000) {
    return Math.round(n / 1000) + 'k';
  }
  if (n < 10000000) {
    return (n / 1000000).toFixed(1) + 'M';
  }
  return Math.round(n / 1000000) + 'M';
}

