/**
 * Domain types and RPC structures for Knowledge Organization Systems,
 * Structural Evidence & Trade-Offs, Documentation Orientations & User-Controlled Downstream Automation.
 *
 * Constitution Authority: CONSTITUTION.md v1.6.0 (The 16 Critical Invariants)
 */

export type DocumentationOrientation = 'ai_oriented' | 'human_oriented' | 'balanced' | 'custom';
export type ObjectiveLevel = 'low' | 'medium' | 'high';
export type ConstraintLevel = 'preference' | 'constraint' | 'hard_constraint';

export interface Tier1Objectives {
  aiDiscoverability: ObjectiveLevel;
  contextEfficiency: ObjectiveLevel;
  humanReadability: ObjectiveLevel;
  knowledgeReuse: ObjectiveLevel;
  narrativeCoherence: ObjectiveLevel;
  maintenanceSimplicity: ObjectiveLevel;
}

export interface Tier2Parameters {
  cohesionThreshold: number;
  semanticDiversityThreshold: number;
  referenceConcentrationThreshold: number;
  fragmentSizeTarget: number;
  maxTraceDepth: number;
  candidateExpansionFactor: number;
  maxContextSurfaceTokens: number;
  splitSensitivity: number;
  canonicalOwnershipSensitivity: number;
  duplicateTolerance: number;
  fragmentationTolerance: number;
}

export interface ConstraintRule {
  id: string;
  level: ConstraintLevel;
  ruleKey: string;
  description: string;
}

export interface ConstraintConflict {
  conflictingRules: string[];
  reason: string;
  suggestion: string;
}

export interface OptimizationProfile {
  orientation: DocumentationOrientation;
  cohesionWeight: number;
  couplingWeight: number;
  concentrationWeight: number;
  contextEfficiencyWeight: number;
  discoverabilityWeight: number;
  narrativeCoherenceWeight: number;
  maintenanceSimplicityWeight: number;
  maxTraceDepth: number;
  maxContextSurfaceTokens: number;
  fragmentSizeTarget: number;
  splitSensitivity: number;
  canonicalOwnershipSensitivity: number;
  constraints: ConstraintRule[];
}

export interface StructuralMetrics {
  cohesion: number;
  coupling: number;
  referenceConcentration: number;
  granularity: number;
  contextEfficiency: number;
  discoverability: number;
  reusability: number;
  semanticDiversity: number;
  navigationalComplexity: number;
  maintenanceCoupling: number;
}

export type ProblemSignatureType =
  | 'god_document'
  | 'implicit_router'
  | 'fragmented_concept'
  | 'isolated_silo'
  | 'healthy_monolith';

export interface StructuralProblemSignature {
  signatureId: string;
  documentPath: string;
  problemType: ProblemSignatureType;
  confidence: number;
  evidence: StructuralMetrics;
  rationale: string;
}

export type KnowledgeOrganizationStrategy =
  | 'section_addressing'
  | 'topic_decomposition'
  | 'lightweight_index'
  | 'faceted_classification'
  | 'preserve_document'
  | 'hybrid'
  | 'split_decompose'
  | 'extract_facet_metadata'
  | 'consolidate_index'
  | 'merge_candidates'
  | 'standardize_terminology'
  | 'classification'
  | 'taxonomy'
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

export interface MetricDeltaComparisonItem {
  metricName: string;
  beforeValue: string | number;
  afterValue: string | number;
  impact: string;
}

export interface MetricDeltaReport {
  efficiencyDelta: number;
  traceDepthDelta: number;
  couplingDelta: number;
  docCountDelta: number;
  refCountDelta?: number;
  narrativeSummary?: string;
  contextEfficiencyDeltaPct?: number;
  navigationalComplexityDeltaPct?: number;
  maintenanceCouplingDeltaPct?: number;
}

export type TransformationAction =
  | { type: 'create_document'; payload: { filePath: string; initialContent: string } }
  | { type: 'update_document_content'; payload: { filePath: string; updatedContent: string } }
  | { type: 'update_document_tags'; payload: { filePath: string; tags: string[] } }
  | { type: 'append_document_content'; payload: { filePath: string; content: string } }
  | { type: 'rewrite_link_references'; payload: { sourcePath: string; oldTarget: string; newTarget: string } }
  | { type: 'inject_fragment_frontmatter'; payload: { filePath: string; fragments?: Array<{ anchor: string; heading: string; start_line: number; end_line: number; tokens: number; inbound: number; outbound: number }> } }
  | { type: 'delete_document'; payload: { filePath: string; file_path?: string } };

export interface TransformationPlan {
  planId: string;
  recommendationId: string;
  actions: TransformationAction[];
  diffPreview: string;
  projectedDelta: MetricDeltaReport;
  requiresApproval: boolean;
  targetDocument?: string;
  expectedImpact?: MetricDeltaComparisonItem[];
  tradeOffs?: string[];
}

export interface StructuralRecommendation {
  recommendationId: string;
  signatureId: string;
  strategy: KnowledgeOrganizationStrategy;
  confidence: number;
  expectedBenefits: string[];
  tradeOffs: string[];
  transformationPlan?: TransformationPlan;
}

// RPC Request & Response contracts matching contracts/evolution_rpc_protocol.json

export interface GetOrientationResponse {
  orientation: DocumentationOrientation;
  profile: OptimizationProfile;
}

export interface SetOrientationRequest {
  orientation: DocumentationOrientation;
}

export interface SetOrientationResponse {
  success: boolean;
  orientation: DocumentationOrientation;
  profile: OptimizationProfile;
}

export interface GetCustomConfigResponse {
  tier1: Tier1Objectives;
  tier2: Tier2Parameters;
  constraints: ConstraintRule[];
}

export interface UpdateCustomConfigRequest {
  tier1: Tier1Objectives;
  tier2: Tier2Parameters;
  constraints: ConstraintRule[];
}

export interface UpdateCustomConfigResponse {
  success: boolean;
  profile: OptimizationProfile;
}

export interface ValidateConstraintsRequest {
  constraints: ConstraintRule[];
}

export interface ValidateConstraintsResponse {
  isValid: boolean;
  conflicts: ConstraintConflict[];
}

export interface GetStructuralProblemsRequest {
  filePath?: string;
}

export interface GetStructuralProblemsResponse {
  signatures: StructuralProblemSignature[];
}

export interface GetRecommendationsRequest {
  signatureId?: string;
}

export interface GetRecommendationsResponse {
  recommendations: StructuralRecommendation[];
}

export interface GeneratePlanRequest {
  recommendationId?: string;
  opportunityId?: string;
  selectedStrategies?: KnowledgeOrganizationStrategy[];
}

export interface GeneratePlanResponse {
  planId: string;
  recommendationId?: string;
  opportunityId?: string;
  actions: TransformationAction[];
  diffPreview: string;
  projectedDelta: MetricDeltaReport;
  requiresApproval: boolean;
}

export interface ApplyPlanRequest {
  planId: string;
}

export interface ApplyPlanResponse {
  success: boolean;
  actualDelta: MetricDeltaReport;
}

// Strategy Selection & Combination Types (Feature 007)

export type StrategyRole = 'primary' | 'supporting' | 'alternative';
export type StrategyRelationshipType = 'complementary' | 'alternative' | 'mutually_exclusive';

export interface StrategyRecommendationItem {
  recommendationId: string;
  strategy: KnowledgeOrganizationStrategy;
  fitScore: number;
  role: StrategyRole;
  rationale: string;
  expectedBenefits: string[];
  countervailingTradeOffs: string[];
  transformationCost: number;
}

export interface CompositePlanValue {
  baseFitSum: number;
  synergyBonus: number;
  overlapPenalty: number;
  transformationCost: number;
  netValue: number;
  narrative: string;
}

export interface StrategySet {
  setId: string;
  opportunityId: string;
  targetDocument: string;
  primary: StrategyRecommendationItem;
  supporting: StrategyRecommendationItem[];
  alternatives: StrategyRecommendationItem[];
  compositePlanValue: CompositePlanValue;
  activeSelection: KnowledgeOrganizationStrategy[];
}

export interface GetStrategySetRequest {
  opportunityId: string;
  documentPath?: string;
}

export interface GetStrategySetResponse {
  strategySet: StrategySet;
}

export interface CalculatePlanValueRequest {
  opportunityId: string;
  selectedStrategies: KnowledgeOrganizationStrategy[];
}

export interface CalculatePlanValueResponse {
  compositePlanValue: CompositePlanValue;
  isValid: boolean;
  conflictMessage?: string | null;
}

export interface ValidateSelectionRequest {
  selectedStrategies: KnowledgeOrganizationStrategy[];
}

export interface IncompatiblePairItem {
  strategyA: KnowledgeOrganizationStrategy;
  strategyB: KnowledgeOrganizationStrategy;
  relationship: StrategyRelationshipType;
  reason: string;
}

export interface ValidateSelectionResponse {
  isValid: boolean;
  incompatiblePairs: IncompatiblePairItem[];
}

