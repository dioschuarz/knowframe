/**
 * Section Fragment & 5-Pillar Quality Type Definitions.
 * Invariant V: Document-Centric Graph (Fragments are metadata, not nodes).
 * Invariant IX: Raw Numeric Integrity (All numerical fields retain integer numbers).
 * Invariant X: Presentation Separation.
 */

export interface FragmentMetadata {
  readonly anchor: string;
  readonly heading: string;
  readonly start_line: number;
  readonly end_line: number;
  readonly tokens: number;
  readonly inbound: number;
  readonly outbound: number;
}

export type FreshnessTier = 'fresh' | 'potentially_stale' | 'stale';

export interface FreshnessEvidence {
  readonly score: number;
  readonly tier: FreshnessTier;
  readonly missing_citations?: readonly string[];
  readonly code_modified_after_doc?: readonly string[];
  readonly deleted_symbols?: readonly string[];
  readonly canonical_contradiction?: string;
  readonly reasons?: readonly string[];
}

export type QualityPillar =
  | 'integrity'
  | 'structure'
  | 'consistency'
  | 'discoverability'
  | 'traceability'
  | 'freshness';

export type StructureDebtType =
  | 'missing_fragment_metadata'
  | 'oversized_document'
  | 'high_reference_concentration'
  | 'unfragmented_central_hub'
  | 'secondary_routing_layer'
  | 'harness_drift';

export interface StructureDebtFinding {
  readonly debt_type: StructureDebtType;
  readonly document_path: string;
  readonly fragment_anchor?: string;
  readonly raw_metric_value: number;
  readonly penalty: number;
  readonly guidance: string;
}

export interface DebtSummary {
  readonly total_debt: number;
  readonly legacy_debt: number;
  readonly new_debt: number;
  readonly resolved_debt: number;
  readonly regressions: number;
}

export interface FivePillarQualityScore {
  readonly overall_score: number;
  readonly integrity_score: number;
  readonly structure_score: number;
  readonly consistency_score: number;
  readonly discoverability_score: number;
  readonly freshness_score: number;
  readonly traceability_score?: number;
  readonly reasons: Record<QualityPillar, readonly string[]>;
  readonly debt_summary: DebtSummary;
}

export interface ContextTraceSectionResponse {
  readonly query: string;
  readonly entrypoint: string;
  readonly target_document: string;
  readonly target_fragment?: string;
  readonly path: readonly string[];
  readonly depth: number;
  readonly document_raw_tokens: number;
  readonly fragment_raw_tokens?: number;
  readonly context_reduction_percent: number;
  readonly relevance_reasons: readonly string[];
}
