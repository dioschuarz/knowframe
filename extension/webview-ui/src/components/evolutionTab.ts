/**
 * Structural Evolution Tab Component (T053)
 *
 * Implements FR-028, FR-029, FR-034, FR-035, Invariant X, US4, US5.
 * Renders inside SettingsDrawer under the 'evolution' tab:
 * - Detected problem signatures (God Document, Implicit Router, Fragmented Concept, etc.)
 * - 10-D Evidence metrics compactly formatted (<= 5 chars)
 * - Candidate KOS recommendations with trade-off matrices
 * - Action trigger to generate transformation plans
 */

import type {
  StructuralProblemSignature,
  StructuralRecommendation,
  ProblemSignatureType,
  KnowledgeOrganizationStrategy,
  StrategySet,
  StrategyRecommendationItem,
  StrategyRole,
  CompositePlanValue,
} from '../../../src/types/evolution.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';
import { renderInfoBubbleHtml } from './infoBubble.ts';

export function isSameFilePath(pathA?: string, pathB?: string): boolean {
  if (!pathA || !pathB) return false;
  if (pathA === pathB) return true;
  const cleanA = pathA.includes('\\') ? pathA.replace(/\\/g, '/') : pathA;
  const cleanB = pathB.includes('\\') ? pathB.replace(/\\/g, '/') : pathB;
  if (cleanA === cleanB) return true;
  return cleanA.endsWith('/' + cleanB) || cleanB.endsWith('/' + cleanA);
}

export interface EvolutionTabProps {
  signatures: StructuralProblemSignature[];
  recommendations: StructuralRecommendation[];
  selectedSignatureId?: string | null;
  isLoading?: boolean;
  activeOrientation?: string;
  structureFindings?: Array<{
    file_path: string;
    description: string;
    remediation_hint?: string;
    category?: string;
    methodology?: string;
    debt_type?: string;
    pillar?: string;
  }>;
  vaultFindings?: Array<{
    file_path: string;
    description: string;
    remediation_hint?: string;
    category?: string;
    methodology?: string;
    debt_type?: string;
    pillar?: string;
    target?: string | null;
    line_number?: number | null;
    deduction_points?: number;
    weight?: number;
    is_legacy?: boolean;
  }>;
  generatingPlanRecId?: string | null;   // recId whose plan is currently being generated
  planGenerationError?: string | null;   // error message for the failed plan generation
  // Pagination & 18 KOS / Pillar Filter state for findings:
  findingsPage?: number;
  findingsPageSize?: number;
  selectedKosFilter?: string;
  selectedPillarFilter?: string;
  // Pagination & Filter state for problem signatures / opportunities:
  signaturesPage?: number;
  signaturesPageSize?: number;
  selectedSignatureTypeFilter?: string;
}

export interface KosMethodologyItem {
  id: string;
  name: string;
  shortLabel: string;
  discipline: string;
  description: string;
  icon: string;
}

export const KOS_18_METHODOLOGIES: KosMethodologyItem[] = [
  { id: 'classification', name: 'Classification', shortLabel: 'Classification', discipline: 'Organization', description: 'Subject categorization, partition coverage, and mutually exclusive groupings', icon: '📂' },
  { id: 'taxonomy', name: 'Taxonomy', shortLabel: 'Taxonomy', discipline: 'Hierarchy', description: 'Hierarchical subsumption, parent-child inheritance, and strict tree acyclicity', icon: '🌳' },
  { id: 'faceted_classification', name: 'Faceted Classification', shortLabel: 'Facets', discipline: 'Multi-Axial', description: 'Orthogonal dimension tagging and multi-axial facet combinations', icon: '💎' },
  { id: 'controlled_vocabulary', name: 'Controlled Vocabulary', shortLabel: 'Vocabulary', discipline: 'Lexicon', description: 'Standardized terminology, synonym ring normalization, and authority files', icon: '📖' },
  { id: 'thesaurus', name: 'Thesaurus', shortLabel: 'Thesaurus', discipline: 'Relations', description: 'ISO 25964 relational reciprocity (BT Broader, NT Narrower, RT Related)', icon: '📚' },
  { id: 'semantic_networks', name: 'Semantic Networks', shortLabel: 'Semantics', discipline: 'Graph', description: 'Typed relations, labelled directed graphs, and conceptual proposition networks', icon: '🕸️' },
  { id: 'conceptual_maps', name: 'Conceptual Maps', shortLabel: 'Concept Maps', discipline: 'Mapping', description: 'Visual concept proposition density and spatial knowledge clustering', icon: '🗺️' },
  { id: 'ontology_engineering', name: 'Ontology Engineering', shortLabel: 'Ontology', discipline: 'Axioms', description: 'Formal domain entities, axioms, SHACL constraint shapes, and inference', icon: '🏛️' },
  { id: 'domain_analysis', name: 'Domain Analysis', shortLabel: 'Domain', discipline: 'Architecture', description: 'Bounded contexts, cohesion boundaries, and cross-subsystem coupling', icon: '🌐' },
  { id: 'metadata_application_profiles', name: 'Metadata', shortLabel: 'Metadata', discipline: 'Schema', description: 'Dublin Core Application Profiles and YAML frontmatter materialized validation', icon: '🏷️' },
  { id: 'information_architecture', name: 'Information Architecture', shortLabel: 'Inf. Arch.', discipline: 'Wayfinding', description: 'Wayfinding, scent navigation, hierarchy depth, and user mental models', icon: '🏗️' },
  { id: 'information_retrieval', name: 'Information Retrieval', shortLabel: 'Info Retrieval', discipline: 'Search', description: 'BM25F probabilistic ranking, inverted index tokenization, and query relevance', icon: '🔍' },
  { id: 'terminology_management', name: 'Terminology Management', shortLabel: 'Terminology', discipline: 'Canon', description: 'Aho-Corasick canon term adherence, lexical drift, and homograph collision guards', icon: '📑' },
  { id: 'knowledge_graph_network_analysis', name: 'Network Analysis', shortLabel: 'Network', discipline: 'Topology', description: 'Brandes betweenness centrality, PageRank, bridge detection, and cut vertices', icon: '📊' },
  { id: 'knowledge_quality', name: 'Knowledge Quality', shortLabel: 'Quality', discipline: 'Gate', description: '5-Pillar quality scoring (Integrity, Structure, Consistency, Discoverability, Freshness)', icon: '🛡️' },
  { id: 'provenance', name: 'Provenance', shortLabel: 'Provenance', discipline: 'History', description: 'Git commit chronology, co-change coupling, and AST code citation traceability', icon: '📜' },
  { id: 'knowledge_drift', name: 'Knowledge Drift', shortLabel: 'Drift', discipline: 'Temporal', description: 'Subject ontogeny divergence, stale citations, and code-evolution mismatch', icon: '⏳' },
  { id: 'context_routing', name: 'Context Routing', shortLabel: 'Context Routing', discipline: 'Prompt', description: 'Constrained beam search context trace and section-level prompt load routing', icon: '⚡' },
];

export function getKosMethodologyItem(id: string): KosMethodologyItem | undefined {
  return KOS_18_METHODOLOGIES.find((k) => k.id === id);
}

export function inferFindingKos(f: any): string {
  if (f.methodology && typeof f.methodology === 'string') {
    const m = f.methodology.toLowerCase();
    const found = KOS_18_METHODOLOGIES.find((k) => k.id === m);
    if (found) return found.id;
  }
  const cat = (f.category || f.debt_type || '').toLowerCase();
  const desc = (f.description || '').toLowerCase();

  // 1. Thesaurus (Relational reciprocity, broken references, missing targets, reciprocal pointers)
  if (
    cat.includes('broken_link') ||
    cat.includes('invalid_anchor') ||
    cat.includes('broken_fragment') ||
    cat.includes('thesaurus') ||
    desc.includes('broken reference') ||
    desc.includes('broken link') ||
    desc.includes('thesaurus') ||
    desc.includes('reciprocity')
  ) {
    return 'thesaurus';
  }

  // 2. Knowledge Drift (Temporal code-evolution mismatch, stale documentation, technical drift)
  if (
    cat.includes('technical_drift') ||
    cat.includes('code_modified_after_doc') ||
    cat.includes('drift') ||
    desc.includes('technical drift') ||
    desc.includes('modified in git after') ||
    desc.includes('cites modified/deleted') ||
    desc.includes('ontogeny') ||
    desc.includes('outdated')
  ) {
    return 'knowledge_drift';
  }

  // 3. Classification (Subject partitioning, concept fragmentation, unclassified notes)
  if (
    cat.includes('collision_potentialfragmentation') ||
    desc.includes('potentialfragmentation') ||
    cat.includes('classification') ||
    desc.includes('classification') ||
    desc.includes('partition') ||
    desc.includes('fragmented concept')
  ) {
    return 'classification';
  }

  // 4. Taxonomy (Hierarchy depth, high hop distance, specialization, acyclic subsumption)
  if (
    cat.includes('high_hop') ||
    cat.includes('collision_potentialspecialization') ||
    desc.includes('potentialspecialization') ||
    cat.includes('taxonomy') ||
    desc.includes('taxonomy') ||
    desc.includes('hops away') ||
    desc.includes('hop distance') ||
    desc.includes('acyclic') ||
    desc.includes('subsumption')
  ) {
    return 'taxonomy';
  }

  // 5. Information Architecture (Wayfinding, secondary routing layers, scent navigation, summaries)
  if (
    cat.includes('secondaryroutinglayer') ||
    cat.includes('secondary_routing_layer') ||
    cat.includes('unfragmentedcentralhub') ||
    cat.includes('unfragmented_central_hub') ||
    cat.includes('harnessdrift') ||
    cat.includes('harness_drift') ||
    cat.includes('collision_potentialsummary') ||
    desc.includes('potentialsummary') ||
    cat.includes('harness') ||
    cat.includes('wayfinding') ||
    desc.includes('secondary routing') ||
    desc.includes('harness route') ||
    desc.includes('wayfinding') ||
    desc.includes('unfragmented central hub')
  ) {
    return 'information_architecture';
  }

  // 6. Terminology Management (Canon terms, duplicate terms, homograph collisions)
  if (
    cat.includes('collision_potentialduplicate') ||
    cat.includes('collision_potentialconflict') ||
    cat.includes('terminology') ||
    desc.includes('semantic collision') ||
    desc.includes('canon term') ||
    desc.includes('homograph')
  ) {
    return 'terminology_management';
  }

  // 7. Controlled Vocabulary (Synonyms, vocabulary standardization, authority rings)
  if (
    cat.includes('vocabulary') ||
    desc.includes('vocabulary') ||
    desc.includes('synonym ring') ||
    desc.includes('authority file')
  ) {
    return 'controlled_vocabulary';
  }

  // 8. Conceptual Maps (Disconnected subgraphs, spatial knowledge clusters, proposition density)
  if (
    cat.includes('disconnected_subgraph') ||
    cat.includes('concept') ||
    desc.includes('disconnected subgraph') ||
    desc.includes('concept map') ||
    desc.includes('proposition density')
  ) {
    return 'conceptual_maps';
  }

  // 9. Network Analysis (Orphans, betweenness centrality, PageRank, bridge detection, cut vertices)
  if (
    cat.includes('orphan') ||
    cat.includes('bridge') ||
    cat.includes('centrality') ||
    cat.includes('network') ||
    desc.includes('orphan note') ||
    desc.includes('isolated orphan') ||
    desc.includes('betweenness') ||
    desc.includes('cut vertex') ||
    desc.includes('isolated silo')
  ) {
    return 'knowledge_graph_network_analysis';
  }

  // 10. Provenance (Missing code citations, git commit history, co-change coupling)
  if (
    cat.includes('missing_code_citation') ||
    cat.includes('provenance') ||
    cat.includes('co_change') ||
    desc.includes('cites code file') ||
    desc.includes('missing citation') ||
    desc.includes('provenance') ||
    desc.includes('co-change')
  ) {
    return 'provenance';
  }

  // 11. Metadata Profiles (DCAP, frontmatter materialized metadata, oversized docs, concentration)
  if (
    cat.includes('missingfragmentmetadata') ||
    cat.includes('missing_fragment_metadata') ||
    cat.includes('oversizeddocument') ||
    cat.includes('oversized_document') ||
    cat.includes('highreferenceconcentration') ||
    cat.includes('high_reference_concentration') ||
    cat.includes('frontmatter') ||
    cat.includes('dcap') ||
    cat.includes('metadata') ||
    desc.includes('fragment metadata') ||
    desc.includes('frontmatter') ||
    desc.includes('oversized document') ||
    desc.includes('oversized') ||
    desc.includes('inbound references target section')
  ) {
    return 'metadata_application_profiles';
  }

  // 12. Faceted Classification (Facets, orthogonal dimensions, tag clusters)
  if (
    cat.includes('facet') ||
    desc.includes('facet') ||
    desc.includes('multi-axial') ||
    desc.includes('dimension')
  ) {
    return 'faceted_classification';
  }

  // 13. Semantic Networks (Typed links, labelled relation graphs)
  if (
    cat.includes('semantic') ||
    desc.includes('semantic network') ||
    desc.includes('relation type')
  ) {
    return 'semantic_networks';
  }

  // 14. Ontology Engineering (SHACL shapes, domain axioms, schema conformance)
  if (
    cat.includes('ontology') ||
    desc.includes('ontology') ||
    desc.includes('shacl') ||
    desc.includes('axiom')
  ) {
    return 'ontology_engineering';
  }

  // 15. Domain Analysis (Bounded contexts, subsystem cohesion)
  if (
    cat.includes('domain') ||
    desc.includes('domain analysis') ||
    desc.includes('bounded context')
  ) {
    return 'domain_analysis';
  }

  // 16. Information Retrieval (BM25F, search scent, inverted index)
  if (
    cat.includes('retrieval') ||
    desc.includes('bm25') ||
    desc.includes('search scent') ||
    desc.includes('ir score')
  ) {
    return 'information_retrieval';
  }

  // 17. Context Routing (Context trace, beam search, prompt token load)
  if (
    cat.includes('routing') ||
    cat.includes('trace') ||
    desc.includes('context trace') ||
    desc.includes('prompt load') ||
    desc.includes('beam search')
  ) {
    return 'context_routing';
  }

  // 18. Knowledge Quality (Quality score regressions, lint gates)
  if (
    cat.includes('quality') ||
    desc.includes('quality score') ||
    desc.includes('regression')
  ) {
    return 'knowledge_quality';
  }

  return 'information_architecture';
}

export function formatMetricValue(val: number): string {
  if (typeof val !== 'number' || isNaN(val)) {
    return '0.00';
  }
  if (Math.abs(val) <= 1.0) {
    return val.toFixed(2);
  }
  return formatCompactNumber(val);
}

export function formatPillarName(pillar?: string): string {
  if (!pillar) return '';
  const p = pillar.toLowerCase().trim();
  switch (p) {
    case 'integrity':
      return 'Integrity';
    case 'structure':
      return 'Structure';
    case 'discoverability':
      return 'Discoverability';
    case 'consistency':
      return 'Consistency';
    case 'freshness':
      return 'Freshness';
    default:
      return p.charAt(0).toUpperCase() + p.slice(1);
  }
}

export function getProblemTypeBadge(type: ProblemSignatureType): { label: string; cssClass: string } {
  switch (type) {
    case 'god_document':
      return { label: 'God Document', cssClass: 'badge-god-doc' };
    case 'implicit_router':
      return { label: 'Implicit Router', cssClass: 'badge-router' };
    case 'fragmented_concept':
      return { label: 'Fragmented Concept', cssClass: 'badge-fragmented' };
    case 'isolated_silo':
      return { label: 'Isolated Silo', cssClass: 'badge-silo' };
    case 'healthy_monolith':
      return { label: 'Healthy Monolith', cssClass: 'badge-healthy' };
    default:
      return { label: type, cssClass: 'badge-default' };
  }
}

export function getStrategyLabel(strategy: KnowledgeOrganizationStrategy | string): string {
  switch (strategy) {
    case 'section_addressing':
      return 'Section Addressing';
    case 'topic_decomposition':
    case 'split_decompose':
      return 'Topic Decomposition';
    case 'lightweight_index':
    case 'consolidate_index':
      return 'Lightweight Index';
    case 'faceted_classification':
    case 'extract_facet_metadata':
      return 'Faceted Classification';
    case 'preserve_document':
      return 'Preserve Document';
    case 'hybrid':
      return 'Hybrid Architecture';
    case 'standardize_terminology':
      return 'Standardize Terminology';
    case 'merge_candidates':
      return 'Merge Candidates';
    // 18 KOS Methodologies
    case 'classification':
      return 'Classification';
    case 'taxonomy':
      return 'Taxonomy';
    case 'controlled_vocabulary':
      return 'Controlled Vocabulary';
    case 'thesaurus':
      return 'Thesaurus';
    case 'semantic_networks':
      return 'Semantic Networks';
    case 'conceptual_maps':
      return 'Conceptual Maps';
    case 'ontology_engineering':
      return 'Ontology Engineering';
    case 'domain_analysis':
      return 'Domain Analysis';
    case 'metadata_application_profiles':
      return 'Metadata Application Profiles';
    case 'information_architecture':
      return 'Information Architecture';
    case 'information_retrieval':
      return 'Information Retrieval';
    case 'terminology_management':
      return 'Terminology Management';
    case 'knowledge_graph_network_analysis':
      return 'Network Analysis';
    case 'knowledge_quality':
      return 'Knowledge Quality';
    case 'provenance':
      return 'Provenance';
    case 'knowledge_drift':
      return 'Knowledge Drift';
    case 'context_routing':
      return 'Context Routing';
    default:
      return String(strategy).replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
  }
}

export function renderStrategyRecommendationItemHtml(
  item: any,
  oppId: string,
  isChecked: boolean,
  isDisabled: boolean,
  role: StrategyRole,
  synergyInfo?: string,
): string {
  const strat = item.strategy;
  const stratLabel = getStrategyLabel(strat);
  const fit = item.fitScore ?? item.fit_score ?? 0;
  const cost = item.transformationCost ?? item.transformation_cost ?? 0;
  const rationale = item.rationale || '';

  const benefits: string[] = item.expectedBenefits || item.expected_benefits || [];
  const tradeOffs: string[] = item.countervailingTradeOffs || item.countervailing_trade_offs || [];

  const benefitsList = benefits.map((b: string) => `<li>↑ ${b}</li>`).join('');
  const tradeOffsList = tradeOffs.map((t: string) => `<li>↓ ${t}</li>`).join('');

  const badgeClass =
    role === 'primary'
      ? 'role-badge-primary'
      : role === 'supporting'
      ? 'role-badge-supporting'
      : 'role-badge-alt';
  const roleLabel = role.charAt(0).toUpperCase() + role.slice(1).toLowerCase();

  return `
    <div class="strategy-option-card role-${role}" data-strategy="${strat}" data-opp-id="${oppId}">
      <div class="strategy-option-header">
        <label class="strategy-checkbox-label">
          <input type="checkbox"
                 class="strategy-checkbox"
                 data-opp-id="${oppId}"
                 data-strategy="${strat}"
                 data-role="${role}"
                 data-fit="${fit}"
                 data-cost="${cost}"
                 ${isChecked ? 'checked' : ''}
                 ${isDisabled ? 'disabled' : ''} />
          <span class="strategy-role-badge ${badgeClass}">${roleLabel}</span>
          <span class="strategy-name">${stratLabel}</span>
        </label>
        <div class="strategy-option-meta">
          ${synergyInfo ? `<span class="synergy-pill">${synergyInfo}</span>` : ''}
          <span class="rec-confidence-badge">${fit}% Fit</span>
        </div>
      </div>
      <p class="strategy-rationale">${rationale}</p>
      <div class="trade-off-table compact">
        <div class="trade-off-col trade-off-benefits">
          <span class="col-title">Expected Benefits</span>
          <ul>${benefitsList}</ul>
        </div>
        <div class="trade-off-col trade-off-costs">
          <span class="col-title">Countervailing Trade-offs</span>
          <ul>${tradeOffsList}</ul>
        </div>
      </div>
    </div>
  `;
}

export function renderStrategySetHtml(stratSet: any, oppId: string, isGenerating = false, showButton = true): string {
  const primary = stratSet.primary;
  const supporting: any[] = stratSet.supporting || [];
  const alternatives: any[] = stratSet.alternatives || [];
  const cpv = stratSet.compositePlanValue || stratSet.composite_plan_value || {
    baseFitSum: 0,
    synergyBonus: 0,
    overlapPenalty: 0,
    transformationCost: 0,
    netValue: 0,
    narrative: '',
  };

  const isPrimaryPreserve = primary?.strategy === 'preserve_document';

  const primaryHtml = primary
    ? renderStrategyRecommendationItemHtml(primary, oppId, true, false, 'primary')
    : '';

  const supportingHtml = supporting
    .map((s) => renderStrategyRecommendationItemHtml(s, oppId, true, isPrimaryPreserve, 'supporting', '+15 Synergy'))
    .join('');

  const alternativesHtml = alternatives
    .map((a) => renderStrategyRecommendationItemHtml(a, oppId, false, isPrimaryPreserve, 'alternative'))
    .join('');

  const baseFit = cpv.baseFitSum ?? cpv.base_fit_sum ?? 0;
  const synergyBonus = cpv.synergyBonus ?? cpv.synergy_bonus ?? 0;
  const overlapPenalty = cpv.overlapPenalty ?? cpv.overlap_penalty ?? 0;
  const netValue = baseFit + synergyBonus - overlapPenalty;
  const narrative =
    supporting.length > 0 && synergyBonus > overlapPenalty
      ? `Combined plan yields net value ${netValue} with +${synergyBonus} synergy bonus exceeding ${overlapPenalty} overlap overhead.`
      : `Active plan yields net value ${netValue} (fit: ${baseFit}).`;

  return `
    <div class="strategy-set-container" data-opp-id="${oppId}">
      <div class="strategy-set-intro">
        <span class="section-micro-title">Multi-Strategy Selection:</span>
      </div>

      <div class="strategy-group primary-group">
        <span class="group-title">Primary Solution</span>
        ${primaryHtml}
      </div>

      ${supporting.length > 0 ? `
        <div class="strategy-group supporting-group">
          <span class="group-title">Supporting Strategies</span>
          ${supportingHtml}
        </div>
      ` : ''}

      ${alternatives.length > 0 ? `
        <div class="strategy-group alternatives-group">
          <span class="group-title">Alternative Options</span>
          ${alternativesHtml}
        </div>
      ` : ''}

      <div class="strategy-conflict-banner hidden" id="conflict-banner-${oppId}">
        <span class="conflict-icon">⚠️</span>
        <span class="conflict-msg"></span>
      </div>

      <div class="composite-plan-card" id="plan-value-card-${oppId}">
        <div class="plan-card-header">
          <span class="plan-card-title">Composite Plan Value & Trade-Off Valuation</span>
          <span class="net-value-badge" id="net-val-badge-${oppId}">Net Value: ${netValue}</span>
        </div>
        <div class="plan-metrics-grid">
          <div class="plan-metric-pill"><span class="pm-lbl">Base Fit Sum</span><span class="pm-val" id="base-fit-${oppId}">${baseFit}</span></div>
          <div class="plan-metric-pill synergy"><span class="pm-lbl">Synergy Bonus</span><span class="pm-val" id="synergy-bonus-${oppId}">+${synergyBonus}</span></div>
          <div class="plan-metric-pill overlap"><span class="pm-lbl">Overlap Penalty</span><span class="pm-val" id="overlap-penalty-${oppId}">-${overlapPenalty}</span></div>
        </div>
        <div class="plan-narrative" id="plan-narrative-${oppId}">${narrative}</div>
      </div>

      ${showButton ? `
      <div class="strategy-set-actions-bar">
        <button type="button"
                class="btn btn-primary btn-generate-multi-plan"
                data-action="generate-multi-plan"
                data-opp-id="${oppId}"
                title="Preview Transformation Plan"
                ${isGenerating ? 'disabled' : ''}>
          ${isGenerating
            ? '<span class="btn-spinner"></span> Generating Transformation Plan...'
            : '⚡ Generate Transformation Plan'}
        </button>
        <div class="empty-selection-warning hidden" id="empty-warning-${oppId}">
          ⚠️ Select at least 1 strategy to proceed
        </div>
      </div>
      ` : `
      <div class="empty-selection-warning hidden" id="empty-warning-${oppId}">
        ⚠️ Select at least 1 strategy to proceed
      </div>
      `}
    </div>
  `;
}



function renderPillarFilterChipsHtml(
  pillarCounts: Record<string, number>,
  selectedPillar: string,
  totalCount: number,
): string {
  const pillars = [
    { id: 'all', label: 'All Pillars', count: totalCount, color: 'var(--vscode-foreground, #ccc)' },
    { id: 'integrity', label: 'Integrity', count: pillarCounts['integrity'] || 0, color: '#ef4444' },
    { id: 'discoverability', label: 'Discoverability', count: pillarCounts['discoverability'] || 0, color: '#eab308' },
    { id: 'structure', label: 'Structure', count: pillarCounts['structure'] || 0, color: '#38bdf8' },
    { id: 'consistency', label: 'Consistency', count: pillarCounts['consistency'] || 0, color: '#f472b6' },
    { id: 'freshness', label: 'Freshness', count: pillarCounts['freshness'] || 0, color: '#c084fc' },
  ];

  return `
    <div class="findings-pillar-filter-bar">
      <span class="findings-filter-label">Filter by Quality Pillar:</span>
      <div class="findings-pillar-chips-container">
        ${pillars.map((p) => {
          const isActive = selectedPillar === p.id;
          return `
            <button type="button"
                    class="findings-pillar-chip ${isActive ? 'active' : ''} ${p.count > 0 ? 'has-findings' : 'zero-findings'}"
                    data-pillar-id="${p.id}"
                    title="${p.label} (${p.count} active issues)">
              <span class="pillar-chip-name">${p.label}</span>
              <span class="pillar-chip-count">${p.count}</span>
            </button>
          `;
        }).join('')}
      </div>
    </div>
  `;
}

function renderKosMenuBarHtml(
  kosCounts: Record<string, number>,
  selectedKosFilter: string,
  totalFindingsCount: number,
  pillarCounts: Record<string, number>,
  selectedPillarFilter: string,
): string {
  const chipsHtml = [
    `<button type="button" class="kos-filter-chip ${selectedKosFilter === 'all' ? 'active' : ''}" data-kos-id="all" title="All 18 KOS Methodologies (${totalFindingsCount} findings)">
      <span class="chip-name">All 18 KOS</span>
      <span class="chip-count">${totalFindingsCount}</span>
    </button>`,
    ...KOS_18_METHODOLOGIES.map((k) => {
      const count = kosCounts[k.id] || 0;
      const isActive = selectedKosFilter === k.id;
      return `
        <button type="button" class="kos-filter-chip ${isActive ? 'active' : ''} ${count > 0 ? 'has-findings' : 'zero-findings'}" data-kos-id="${k.id}" title="${k.name}: ${k.description}">
          <span class="chip-icon">${k.icon}</span>
          <span class="chip-name">${k.shortLabel}</span>
          <span class="chip-count">${count}</span>
        </button>
      `;
    }),
  ].join('');

  const pillarChipsHtml = renderPillarFilterChipsHtml(pillarCounts, selectedPillarFilter, totalFindingsCount);

  return `
    <div class="evolution-kos-menu-card">
      <div class="kos-menu-header">
        <div class="kos-menu-title-box">
          <span class="kos-menu-icon">🏛️</span>
          <div>
            <span class="kos-menu-title">Knowledge Organization Systems</span>
            <p class="kos-menu-subtitle">Explore vault findings and candidate optimization strategies across all 18 KOS methodologies.</p>
          </div>
        </div>
      </div>
      ${pillarChipsHtml}
      <div class="findings-kos-filter-bar">
        <span class="findings-filter-label">Filter by KOS Discipline:</span>
        <div class="kos-chips-container">
          ${chipsHtml}
        </div>
        <input type="hidden" id="select-kos-filter" value="${selectedKosFilter}" />
      </div>
    </div>
  `;
}

function renderPaginationBarHtml(
  position: 'top' | 'bottom',
  currentPage: number,
  totalPages: number,
  pageSize: number,
  startIndex: number,
  endIndex: number,
  totalItems: number,
  selectedKosName?: string,
  selectedTypeLabel?: string,
  selectedPillarLabel?: string,
): string {
  const isFirst = currentPage <= 1;
  const isLast = currentPage >= totalPages;

  let filterPill = '';
  if (selectedPillarLabel && selectedPillarLabel !== 'All' && selectedPillarLabel !== 'All Pillars') {
    filterPill = `<span class="kos-filter-pill">Pillar: ${selectedPillarLabel}</span>`;
  } else if (selectedKosName && selectedKosName !== 'all' && selectedKosName !== 'All 18 KOS') {
    filterPill = `<span class="kos-filter-pill">Discipline: ${selectedKosName}</span>`;
  } else if (selectedTypeLabel && selectedTypeLabel !== 'All' && selectedTypeLabel !== 'All Opportunities') {
    filterPill = `<span class="kos-filter-pill">Type: ${selectedTypeLabel}</span>`;
  }

  return `
    <div class="findings-pagination-bar single-pagination-bar pagination-${position}">
      <div class="pagination-info">
        <span>Showing <strong>${totalItems === 0 ? 0 : startIndex + 1}–${endIndex}</strong> of <strong>${totalItems}</strong> finding${totalItems === 1 ? '' : 's'}</span>
        ${filterPill}
      </div>
      <div class="pagination-actions">
        <div class="page-size-picker">
          <label class="page-size-label">Per page:</label>
          <div class="settings-scope-dropdown page-size-dropdown" data-position="${position}">
            <button type="button" class="settings-scope-btn page-size-btn btn-page-size-toggle" data-position="${position}" aria-haspopup="listbox" aria-expanded="false" title="Findings per page">
              <span class="label-page-size-selected" data-position="${position}">${pageSize}</span>
              <span class="scope-dropdown-arrow">▾</span>
            </button>
            <div class="settings-scope-menu page-size-menu" id="menu-page-size-${position}" data-position="${position}" style="display: none;" role="listbox">
              ${[10, 25, 50].map((s) => `
                <div class="scope-menu-item ${pageSize === s ? 'active' : ''}" data-value="${s}" data-position="${position}" role="option" aria-selected="${pageSize === s}">
                  <span class="scope-item-check" style="${pageSize === s ? '' : 'visibility: hidden;'}">✓</span>
                  <span class="scope-item-title">${s}</span>
                </div>
              `).join('')}
            </div>
            <input type="hidden" class="findings-page-size-select" data-position="${position}" value="${pageSize}" />
          </div>
        </div>
        <div class="pagination-nav-group">
          <button type="button" class="btn-tiny btn-page-nav" data-page="1" ${isFirst ? 'disabled' : ''} title="First page">⏮</button>
          <button type="button" class="btn-tiny btn-page-nav" data-page="${currentPage - 1}" ${isFirst ? 'disabled' : ''} title="Previous page">◀</button>
          <span class="pagination-page-label">Page <strong>${currentPage}</strong> of <strong>${totalPages}</strong></span>
          <button type="button" class="btn-tiny btn-page-nav" data-page="${currentPage + 1}" ${isLast ? 'disabled' : ''} title="Next page">▶</button>
          <button type="button" class="btn-tiny btn-page-nav" data-page="${totalPages}" ${isLast ? 'disabled' : ''} title="Last page">⏭</button>
        </div>
      </div>
    </div>
  `;
}

function renderSignaturesPaginationBarHtml(
  position: 'top' | 'bottom',
  currentPage: number,
  totalPages: number,
  pageSize: number,
  startIndex: number,
  endIndex: number,
  totalItems: number,
  selectedTypeLabel?: string,
): string {
  return renderPaginationBarHtml(
    position,
    currentPage,
    totalPages,
    pageSize,
    startIndex,
    endIndex,
    totalItems,
    undefined,
    selectedTypeLabel,
  );
}

function renderSignatureFilterChipsHtml(
  typeCounts: Record<string, number>,
  selectedType: string,
  totalCount: number,
): string {
  const types = [
    { id: 'all', label: 'All Opportunities', count: totalCount, icon: '🌟' },
    { id: 'isolated_silo', label: 'Isolated Silos', count: typeCounts['isolated_silo'] || 0, icon: '🏝️' },
    { id: 'god_document', label: 'God Documents', count: typeCounts['god_document'] || 0, icon: '📜' },
    { id: 'implicit_router', label: 'Implicit Routers', count: typeCounts['implicit_router'] || 0, icon: '🔀' },
    { id: 'fragmented_concept', label: 'Fragmented Concepts', count: typeCounts['fragmented_concept'] || 0, icon: '🧩' },
    { id: 'healthy_monolith', label: 'Healthy Monoliths', count: typeCounts['healthy_monolith'] || 0, icon: '🏛️' },
  ];

  return `
    <div class="signatures-filter-bar">
      <span class="signatures-filter-label">Filter by Opportunity Type:</span>
      <div class="signatures-chips-container">
        ${types.map((t) => {
          const isActive = selectedType === t.id;
          return `
            <button type="button"
                    class="signature-filter-chip ${isActive ? 'active' : ''} ${t.count > 0 ? 'has-items' : 'zero-items'}"
                    data-sig-type="${t.id}"
                    title="${t.label} (${t.count} detected)">
              <span class="chip-icon">${t.icon}</span>
              <span class="chip-name">${t.label}</span>
              <span class="chip-count">${t.count}</span>
            </button>
          `;
        }).join('')}
      </div>
    </div>
  `;
}

export function renderEvolutionTabHtml(props: EvolutionTabProps): string {
  if (props.isLoading) {
    return `
      <div class="evolution-tab-loading">
        <div class="loading-spinner"></div>
        <p>Analyzing vault structural metrics and candidate KOS strategies...</p>
      </div>
    `;
  }

  const { generatingPlanRecId, planGenerationError } = props;

  const orientationLabel = (props.activeOrientation || 'balanced')
    .replace(/_/g, ' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());

  const rawFindings = props.vaultFindings || props.structureFindings || [];
  const rawSignatures = props.signatures || [];
  const rawRecs = props.recommendations || [];

  interface UnifiedFindingItem {
    id: string;
    file_path: string;
    line_number?: number | null;
    target?: string | null;
    pillar: string;
    category: string;
    kos_discipline: string;
    description: string;
    remediation_hint?: string;
    signature?: any;
    recommendations?: any[];
    recommendation_id?: string;
    opportunity_id?: string;
    strategy_set?: any;
    deduction_points?: number;
    weight?: number;
    is_legacy?: boolean;
  }

  const allUnifiedFindings: UnifiedFindingItem[] = [];
  const seenFiles = new Set<string>();

  // 1. Ingest all vaultFindings
  for (const f of rawFindings) {
    const filePath = f.file_path;
    const matchingSig = rawSignatures.find((s: any) => {
      const dp = s.documentPath || s.document_path;
      return dp === filePath || isSameFilePath(dp, filePath);
    });
    const sigId = matchingSig ? (matchingSig.signatureId || matchingSig.signature_id) : undefined;
    const recsForSig = sigId
      ? rawRecs.filter((r: any) => (r.signatureId || r.signature_id) === sigId)
      : [];
    const primaryRec = recsForSig[0];
    const stratSet = matchingSig ? (matchingSig.strategySet || matchingSig.strategy_set) : undefined;

    const kos = inferFindingKos(f);
    const pillar = (f.pillar || (matchingSig ? ((matchingSig.problemType === 'isolated_silo' || matchingSig.problem_type === 'isolated_silo') ? 'discoverability' : 'structure') : 'structure')).toLowerCase();
    const cat = f.category || f.debt_type || (matchingSig ? (matchingSig.problemType || matchingSig.problem_type) : 'debt');

    allUnifiedFindings.push({
      id: `finding_${filePath}_${f.line_number || 0}_${cat}`,
      file_path: filePath,
      line_number: f.line_number,
      target: f.target,
      pillar,
      category: cat,
      kos_discipline: kos,
      description: f.description,
      remediation_hint: f.remediation_hint || (primaryRec ? (primaryRec.expectedBenefits?.[0] || primaryRec.expected_benefits?.[0] || primaryRec.rationale) : undefined),
      signature: matchingSig,
      recommendations: recsForSig,
      recommendation_id: primaryRec ? (primaryRec.recommendationId || primaryRec.recommendation_id) : undefined,
      opportunity_id: sigId,
      strategy_set: stratSet,
      deduction_points: f.deduction_points,
      weight: f.weight,
      is_legacy: f.is_legacy,
    });
    seenFiles.add(filePath);
  }

  // 2. Ingest structural problem signatures only if no vaultFindings were provided (e.g. standalone test mode)
  if (rawFindings.length === 0) {
    for (const s of rawSignatures) {
      const docPath = s.documentPath || s.document_path;
      let alreadySeen = seenFiles.has(docPath);
      if (!alreadySeen) {
        for (const f of seenFiles) {
          if (f === docPath || isSameFilePath(f, docPath)) {
            alreadySeen = true;
            break;
          }
        }
      }
      if (!alreadySeen) {
        const sigId = s.signatureId || s.signature_id;
        const recsForSig = rawRecs.filter((r: any) => (r.signatureId || r.signature_id) === sigId);
        const primaryRec = recsForSig[0];
        const stratSet = s.strategySet || s.strategy_set;
        const problemType = s.problemType || s.problem_type || 'god_document';

        const pillar = (problemType === 'isolated_silo' || problemType === 'implicit_router') ? 'discoverability' : 'structure';

        let kos = 'metadata_application_profiles';
        if (problemType === 'isolated_silo') kos = 'knowledge_graph_network_analysis';
        else if (problemType === 'implicit_router') kos = 'information_architecture';
        else if (problemType === 'god_document') kos = 'metadata_application_profiles';
        else if (problemType === 'fragmented_concept') kos = 'classification';
        else if (problemType === 'healthy_monolith') kos = 'taxonomy';

        allUnifiedFindings.push({
          id: `sig_${sigId || docPath}`,
          file_path: docPath,
          line_number: null,
          target: null,
          pillar,
          category: problemType,
          kos_discipline: kos,
          description: s.rationale || `${getProblemTypeBadge(problemType).label} detected on ${docPath}.`,
          remediation_hint: primaryRec ? (primaryRec.expectedBenefits?.[0] || primaryRec.expected_benefits?.[0] || primaryRec.rationale) : undefined,
          signature: s,
          recommendations: recsForSig,
          recommendation_id: primaryRec ? (primaryRec.recommendationId || primaryRec.recommendation_id) : undefined,
          opportunity_id: sigId,
          strategy_set: stratSet,
        });
        seenFiles.add(docPath);
      }
    }
  }

  if (allUnifiedFindings.length === 0) {
    return `
      <div class="evolution-tab-container">
        <div class="evolution-header">
          <h2>Structural Evolution & Knowledge Strategies</h2>
          <p class="evolution-subtitle">Empirical metric evidence, problem signature detection, and transparent trade-off matrices.</p>
        </div>
        <div class="evolution-empty-state">
          <span class="empty-icon">🌱</span>
          <h3>Vault Structure is Balanced & Clean</h3>
          <p>No structural problem signatures (God Documents, Implicit Routers, or Isolated Silos) or debt findings were detected under the active <strong>${orientationLabel}</strong> optimization profile.</p>
        </div>
      </div>
    `;
  }

  // Quality Pillar counts across all unified findings
  const pillarCounts: Record<string, number> = {
    integrity: 0,
    structure: 0,
    consistency: 0,
    discoverability: 0,
    freshness: 0,
  };
  for (const f of allUnifiedFindings) {
    const p = f.pillar.toLowerCase();
    if (pillarCounts[p] !== undefined) {
      pillarCounts[p]++;
    }
  }

  // Filter by selected quality pillar
  const selectedPillar = props.selectedPillarFilter || 'all';
  const pillarFiltered = selectedPillar === 'all'
    ? allUnifiedFindings
    : allUnifiedFindings.filter((f) => f.pillar.toLowerCase() === selectedPillar);

  // Filter by problem signature type (if specified)
  const selectedSigType = props.selectedSignatureTypeFilter || 'all';
  const sigTypeFiltered = selectedSigType === 'all'
    ? pillarFiltered
    : pillarFiltered.filter((f) => f.category === selectedSigType);

  // KOS counts across the filtered findings
  const kosCounts: Record<string, number> = {};
  for (const k of KOS_18_METHODOLOGIES) {
    kosCounts[k.id] = 0;
  }
  for (const f of sigTypeFiltered) {
    kosCounts[f.kos_discipline] = (kosCounts[f.kos_discipline] || 0) + 1;
  }

  // Filter by KOS methodology
  const selectedKos = props.selectedKosFilter || 'all';
  const selectedKosMeta = selectedKos !== 'all' ? getKosMethodologyItem(selectedKos) : undefined;
  const filteredFindings = selectedKos === 'all'
    ? sigTypeFiltered
    : sigTypeFiltered.filter((f) => f.kos_discipline === selectedKos);

  const totalFiltered = filteredFindings.length;
  const rawPageSize = props.findingsPageSize ?? props.signaturesPageSize ?? 10;
  const pageSize = [10, 25, 50].includes(rawPageSize) ? rawPageSize : 10;
  const totalPages = Math.max(1, Math.ceil(totalFiltered / pageSize));
  const currentPage = Math.min(Math.max(1, props.findingsPage || props.signaturesPage || 1), totalPages);
  const startIndex = (currentPage - 1) * pageSize;
  const endIndex = Math.min(startIndex + pageSize, totalFiltered);
  const displayedFindings = filteredFindings.slice(startIndex, endIndex);

  const selectedSigMeta = selectedSigType !== 'all' ? getProblemTypeBadge(selectedSigType as any) : undefined;
  const kosMenuHtml = renderKosMenuBarHtml(kosCounts, selectedKos, allUnifiedFindings.length, pillarCounts, selectedPillar);

  // Exactly ONE bottom pagination bar
  const bottomPaginationHtml = totalFiltered > 0
    ? renderPaginationBarHtml(
        'bottom',
        currentPage,
        totalPages,
        pageSize,
        startIndex,
        endIndex,
        totalFiltered,
        selectedKosMeta?.shortLabel,
        selectedSigMeta?.label,
        selectedPillar !== 'all' ? formatPillarName(selectedPillar) : undefined,
      )
    : '';

  let itemsHtml = '';
  if (displayedFindings.length === 0) {
    const activeFilterLabel =
      selectedPillar !== 'all'
        ? formatPillarName(selectedPillar)
        : (selectedKosMeta?.shortLabel || selectedSigMeta?.label || selectedKos);
    itemsHtml = `
      <div class="findings-empty-filtered">
        <p>No findings detected for <strong>${activeFilterLabel}</strong> in the vault.</p>
      </div>
    `;
  } else {
    itemsHtml = displayedFindings.map((f: UnifiedFindingItem) => {
      const fKosMeta = getKosMethodologyItem(f.kos_discipline);
      const descLower = (f.description || '').toLowerCase();
      const catLower = (f.category || '').toLowerCase();

      const isSecondary =
        catLower.includes('secondaryrouting') ||
        descLower.includes('secondary routing layer');
      const isHarnessDrift =
        catLower === 'harness_drift' ||
        descLower.includes('harness route drift') ||
        descLower.includes('harness drift');
      const isBrokenRef =
        catLower.includes('broken_link') ||
        catLower.includes('invalid_anchor') ||
        descLower.includes('broken reference') ||
        descLower.includes('broken link');

      const sig = f.signature;
      const sigId = sig ? (sig.signatureId || sig.signature_id) : undefined;
      const problemType = sig ? (sig.problemType || sig.problem_type) : undefined;
      const confidence = sig && typeof sig.confidence === 'number' ? sig.confidence : undefined;
      const problemBadge = problemType ? getProblemTypeBadge(problemType) : undefined;

      // 10-D Evidence metrics grid (<= 5 chars each)
      let evidenceGridHtml = '';
      if (sig && sig.evidence) {
        const e = sig.evidence || {};
        const coh = typeof e.cohesion === 'number' ? e.cohesion : 0;
        const coup = typeof e.coupling === 'number' ? e.coupling : 0;
        const conc = typeof e.referenceConcentration === 'number' ? e.referenceConcentration : (typeof e.reference_concentration === 'number' ? e.reference_concentration : 0);
        const gran = typeof e.granularity === 'number' ? e.granularity : 0;
        const eff = typeof e.contextEfficiency === 'number' ? e.contextEfficiency : (typeof e.context_efficiency === 'number' ? e.context_efficiency : 0);
        const disc = typeof e.discoverability === 'number' ? e.discoverability : 0;
        const reus = typeof e.reusability === 'number' ? e.reusability : 0;
        const div = typeof e.semanticDiversity === 'number' ? e.semanticDiversity : (typeof e.semantic_diversity === 'number' ? e.semantic_diversity : 0);
        const nav = typeof e.navigationalComplexity === 'number' ? e.navigationalComplexity : (typeof e.navigational_complexity === 'number' ? e.navigational_complexity : 0);
        const maint = typeof e.maintenanceCoupling === 'number' ? e.maintenanceCoupling : (typeof e.maintenance_coupling === 'number' ? e.maintenance_coupling : 0);

        evidenceGridHtml = `
          <div class="evidence-metrics-grid">
            <div class="evidence-pill" title="Cohesion"><span class="ep-label">Cohesion ${renderInfoBubbleHtml('cohesion')}</span><span class="ep-val">${formatMetricValue(coh)}</span></div>
            <div class="evidence-pill" title="Coupling"><span class="ep-label">Coupling ${renderInfoBubbleHtml('coupling')}</span><span class="ep-val">${formatMetricValue(coup)}</span></div>
            <div class="evidence-pill" title="Reference Concentration"><span class="ep-label">Reference Concentration ${renderInfoBubbleHtml('reference_concentration')}</span><span class="ep-val">${formatMetricValue(conc)}</span></div>
            <div class="evidence-pill" title="Granularity"><span class="ep-label">Granularity ${renderInfoBubbleHtml('granularity')}</span><span class="ep-val">${formatMetricValue(gran)}</span></div>
            <div class="evidence-pill" title="Context Efficiency"><span class="ep-label">Context Efficiency ${renderInfoBubbleHtml('context_efficiency')}</span><span class="ep-val">${formatMetricValue(eff)}</span></div>
            <div class="evidence-pill" title="Discoverability"><span class="ep-label">Discoverability ${renderInfoBubbleHtml('discoverability')}</span><span class="ep-val">${formatMetricValue(disc)}</span></div>
            <div class="evidence-pill" title="Knowledge Reuse"><span class="ep-label">Knowledge Reuse ${renderInfoBubbleHtml('knowledge_reuse')}</span><span class="ep-val">${formatMetricValue(reus)}</span></div>
            <div class="evidence-pill" title="Semantic Diversity"><span class="ep-label">Semantic Diversity ${renderInfoBubbleHtml('semantic_diversity')}</span><span class="ep-val">${formatMetricValue(div)}</span></div>
            <div class="evidence-pill" title="Navigational Complexity"><span class="ep-label">Navigational Complexity ${renderInfoBubbleHtml('navigational_complexity')}</span><span class="ep-val">${formatMetricValue(nav)}</span></div>
            <div class="evidence-pill" title="Maintenance Coupling"><span class="ep-label">Maintenance Coupling ${renderInfoBubbleHtml('maintenance_coupling')}</span><span class="ep-val">${formatMetricValue(maint)}</span></div>
          </div>
        `;
      }

      // Recommendations / Strategy Set
      let recsHtml = '';
      if (sig && (sig.strategySet || sig.strategy_set)) {
        recsHtml = renderStrategySetHtml(sig.strategySet || sig.strategy_set, sigId || '', generatingPlanRecId === sigId, false);
      } else if (f.recommendations && f.recommendations.length > 0) {
        recsHtml = `
          <div class="recs-list">
            ${f.recommendations.map((rawRec: any) => {
              const recId = rawRec.recommendationId || rawRec.recommendation_id || '';
              const recConf = typeof rawRec.confidence === 'number' ? rawRec.confidence : 0;
              const strategy = rawRec.strategy || 'topic_decomposition';
              const isAdvisory = recConf < 0.70;
              const confidencePercent = Math.round(recConf * 100);
              const isHybrid = strategy === 'hybrid' || Boolean(rawRec.isHybrid || rawRec.is_hybrid);
              const synergy = rawRec.synergyRationale || rawRec.synergy_rationale;
              const effort = rawRec.estimatedEffort || rawRec.estimated_effort || 'Low';
              const benefits: string[] = Array.isArray(rawRec.expectedBenefits)
                ? rawRec.expectedBenefits
                : (Array.isArray(rawRec.expected_benefits) ? rawRec.expected_benefits : []);
              const tradeOffs: string[] = Array.isArray(rawRec.tradeOffs)
                ? rawRec.tradeOffs
                : (Array.isArray(rawRec.trade_offs) ? rawRec.trade_offs : []);
              const benefitsList = benefits.map((b) => `<li>↑ ${b}</li>`).join('');
              const tradeOffsList = tradeOffs.map((t) => `<li>↓ ${t}</li>`).join('');

              return `
                <div class="recommendation-card${isHybrid ? ' hybrid-recommendation-card' : ''}" data-rec-id="${recId}">
                  <div class="rec-card-header">
                    <span class="rec-strategy-title">${isHybrid ? 'Recommended Hybrid Approach: ' : ''}${getStrategyLabel(strategy)}</span>
                    <div class="rec-header-badges">
                      ${isHybrid ? '<span class="hybrid-badge">Recommended Hybrid Approach</span>' : ''}
                      ${isAdvisory ? '<span class="rec-advisory-badge">Advisory</span>' : ''}
                      <span class="rec-confidence-badge">${confidencePercent}% Fit ${renderInfoBubbleHtml('fit_score')}</span>
                    </div>
                  </div>
                  ${synergy ? `
                    <div class="rec-synergy-banner">
                      <span class="synergy-label">Synergy Rationale:</span>
                      <span class="synergy-text">${synergy}</span>
                    </div>
                  ` : ''}
                  <div class="trade-off-table">
                    <div class="trade-off-col trade-off-benefits">
                      <span class="col-title">Expected Benefits</span>
                      <ul>${benefitsList}</ul>
                    </div>
                    <div class="trade-off-col trade-off-costs">
                      <span class="col-title">Countervailing Trade-offs</span>
                      <ul>${tradeOffsList}</ul>
                    </div>
                  </div>
                </div>
              `;
            }).join('')}
          </div>
        `;
      }

      // Primary Action Button (Driving the user to inspect plan preview and apply resolution)
      const oppId = f.opportunity_id || (sig ? (sig.signatureId || sig.signature_id) : undefined);
      let primaryActionBtn = '';

      if (oppId) {
        const isGenerating = generatingPlanRecId === oppId;
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-generate-multi-plan${isGenerating ? ' is-loading' : ''}"
                  data-action="generate-multi-plan"
                  data-opp-id="${oppId}"
                  title="Generate transformation plan to preview diff and apply resolution"
                  ${isGenerating ? 'disabled' : ''}>
            ${isGenerating
              ? '<span class="btn-spinner"></span> Generating Transformation Plan...'
              : '⚡ Generate Transformation Plan'}
          </button>
        `;
      } else if (f.recommendation_id) {
        const isGenerating = generatingPlanRecId === f.recommendation_id;
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-generate-plan${isGenerating ? ' is-loading' : ''}"
                  data-action="generate-plan"
                  data-rec-id="${f.recommendation_id}"
                  title="Generate transformation plan to preview diff and apply resolution"
                  ${isGenerating ? 'disabled' : ''}>
            ${isGenerating
              ? '<span class="btn-spinner"></span> Generating Transformation Plan...'
              : '⚡ Generate Transformation Plan'}
          </button>
        `;
      } else if (isBrokenRef) {
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-fix-broken"
                  data-action="fix-broken"
                  data-file="${f.file_path}"
                  data-target="${f.target || ''}"
                  ${f.line_number ? `data-line="${f.line_number}"` : ''}
                  title="Select valid target note and fix reference">
            🔗 Fix Reference
          </button>
        `;
      } else if (isSecondary) {
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-gov-eliminate-secondary"
                  data-target="${f.file_path}"
                  title="Safely remove secondary routing layer">
            ⚡ Safely Remove Secondary Route
          </button>
        `;
      } else if (isHarnessDrift) {
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-gov-sync-harnesses"
                  data-target="${f.file_path}"
                  title="Review and synchronize harness routes">
            ⚡ Review & Sync Harnesses
          </button>
        `;
      } else if (
        f.category === 'review_required' ||
        f.category === 'document_review' ||
        f.category === 'unreviewed' ||
        f.obligation !== undefined ||
        f.review_status !== undefined
      ) {
        primaryActionBtn = `
          <button type="button"
                  class="btn btn-primary btn-finding-action btn-mark-reviewed"
                  data-action="mark-reviewed"
                  data-doc-id="${f.file_path || f.doc_id || f.target || ''}"
                  title="Mark this document as reviewed for pending changes">
            ✓ Mark Reviewed
          </button>
        `;
      } else {
        primaryActionBtn = '';
      }

      const kosBadgeHtml = fKosMeta
        ? `<span class="badge-kos badge-kos-${fKosMeta.id}" title="${fKosMeta.discipline}: ${fKosMeta.description}">${fKosMeta.shortLabel}</span>`
        : '';
      const pillarBadgeHtml = f.pillar
        ? `<span class="badge-pillar badge-pillar-${String(f.pillar).toLowerCase()}">${formatPillarName(f.pillar)}</span>`
        : '';
      const reviewBadgeHtml = (f.review_status || f.obligation || f.category === 'review_required')
        ? `<span class="badge-review badge-review-${String(f.review_status || 'pending').toLowerCase()}" style="font-size: 10px; padding: 2px 6px; border-radius: 4px; font-weight: 600; text-transform: uppercase; background: var(--vscode-badge-background, rgba(56, 189, 248, 0.2)); color: var(--vscode-badge-foreground, #38bdf8);">Review: ${f.review_status || 'Required'}</span>`
        : '';
      const probBadgeHtml = problemBadge
        ? `<span class="sig-problem-badge ${problemBadge.cssClass}">${problemBadge.label}</span>`
        : '';
      const confHtml = confidence !== undefined
        ? `<span class="sig-confidence badge-confidence">${Math.round(confidence * 100)}% Confidence</span>`
        : '';

      const lineSuffix = f.line_number ? `:${f.line_number}` : '';

      return `
        <div class="evolution-finding-item" ${sigId ? `data-signature-id="${sigId}"` : ''}>
          <div class="finding-header-row">
            <div class="finding-tags-row">
              ${kosBadgeHtml}
              ${pillarBadgeHtml}
              ${reviewBadgeHtml}
              ${probBadgeHtml}
              ${confHtml}
            </div>
            <div class="finding-loc" data-file="${f.file_path}" ${f.line_number ? `data-line="${f.line_number}"` : ''} title="Click to open file in editor">
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="flex-shrink: 0; opacity: 0.7;">
                <path d="M9 1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V5L9 1z"/>
                <polyline points="9 1 9 5 13 5"/>
              </svg>
              <span class="finding-path">${f.file_path}${lineSuffix}</span>
            </div>
          </div>

          <div class="finding-desc">${f.description}</div>

          ${evidenceGridHtml ? `
            <div class="sig-evidence-section" style="margin-top: 4px;">
              <span class="section-micro-title">Empirical Metric Evidence:</span>
              ${evidenceGridHtml}
            </div>
          ` : ''}

          ${recsHtml ? `
            <div class="sig-recommendations-section" style="margin-top: 6px;">
              ${recsHtml}
            </div>
          ` : ''}

          ${f.remediation_hint ? `
            <div class="finding-hint">
              💡 Suggested Resolution: ${f.remediation_hint}
            </div>
          ` : ''}

          <div class="finding-actions-row">
            ${primaryActionBtn}
            <button type="button"
                    class="btn ${primaryActionBtn ? 'btn-secondary' : 'btn-primary'} btn-finding-open-file"
                    data-action="open-file"
                    data-file="${f.file_path}"
                    ${f.line_number ? `data-line="${f.line_number}"` : ''}
                    title="Open file in editor">
              Open File ↗
            </button>
          </div>
        </div>
      `;
    }).join('');
  }

  const hasOnlyStructure =
    allUnifiedFindings.length > 0 &&
    allUnifiedFindings.every((f) => !f.pillar || f.pillar.toLowerCase() === 'structure');

  return `
    <div class="evolution-tab-container">
      <div class="evolution-header">
        <h2>Structural Evolution & Knowledge Strategies</h2>
        <p class="evolution-subtitle">Empirical metric evidence, problem signature detection, and transparent trade-off matrices.</p>
      </div>

      ${kosMenuHtml}

      <div class="evolution-findings-card">
        <div class="findings-header">
          <div class="findings-title-box">
            <span class="findings-icon">🏛️</span>
            <span class="findings-title">${hasOnlyStructure ? 'Pillar 2 Structural Findings' : 'Vault Knowledge & KOS Findings'} (${allUnifiedFindings.length})</span>
          </div>
          <span class="findings-badge-warn">${hasOnlyStructure ? `Pillar 2 Structural Debt Active (${allUnifiedFindings.length})` : `${allUnifiedFindings.length} Active Issues`}</span>
        </div>
        <p class="findings-desc">
          The diagnostic engine identified the following findings categorized across 18 Knowledge Organization System (KOS) disciplines under your <strong>${orientationLabel}</strong> evolution strategy:
        </p>
        <div class="findings-list">
          ${itemsHtml}
        </div>
        ${bottomPaginationHtml}
      </div>
    </div>
  `;
}
