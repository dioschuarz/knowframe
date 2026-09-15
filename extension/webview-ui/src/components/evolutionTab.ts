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
} from '../../../src/types/evolution.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';

export interface EvolutionTabProps {
  signatures: StructuralProblemSignature[];
  recommendations: StructuralRecommendation[];
  selectedSignatureId?: string | null;
  isLoading?: boolean;
  activeOrientation?: string;
  structureFindings?: Array<{ file_path: string; description: string; remediation_hint?: string }>;
  generatingPlanRecId?: string | null;   // recId whose plan is currently being generated
  planGenerationError?: string | null;   // error message for the failed plan generation
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

export function getStrategyLabel(strategy: KnowledgeOrganizationStrategy): string {
  switch (strategy) {
    case 'section_addressing':
      return 'Section Addressing';
    case 'topic_decomposition':
      return 'Topic Decomposition';
    case 'lightweight_index':
      return 'Lightweight Index';
    case 'faceted_classification':
      return 'Faceted Classification';
    case 'preserve_document':
      return 'Preserve Document';
    case 'hybrid':
      return 'Hybrid Architecture';
  }
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

  // Render Pillar 2 Structural Debt findings if any exist
  let structureFindingsHtml = '';
  if (props.structureFindings && props.structureFindings.length > 0) {
    const itemsHtml = props.structureFindings.map((f) => `
      <div class="evolution-finding-item">
        <div class="finding-loc">
          <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="flex-shrink: 0; opacity: 0.7;">
            <path d="M9 1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V5L9 1z"/>
            <polyline points="9 1 9 5 13 5"/>
          </svg>
          <span class="finding-path">${f.file_path}</span>
        </div>
        <div class="finding-desc">${f.description}</div>
        ${f.remediation_hint ? `<div class="finding-hint">💡 ${f.remediation_hint}</div>` : ''}
      </div>
    `).join('');

    structureFindingsHtml = `
      <div class="evolution-findings-card">
        <div class="findings-header">
          <div class="findings-title-box">
            <span class="findings-icon">📐</span>
            <span class="findings-title">Pillar 2 Structural Findings (${props.structureFindings.length})</span>
          </div>
          <span class="findings-badge-warn">Structure Debt Active</span>
        </div>
        <p class="findings-desc">
          The diagnostic engine identified the following structural debt findings in the vault. These directly impact context retrieval efficiency under your <strong>${orientationLabel}</strong> evolution strategy:
        </p>
        <div class="findings-list">
          ${itemsHtml}
        </div>
      </div>
    `;
  }

  if (!props.signatures || props.signatures.length === 0) {
    const emptyMessage = props.structureFindings && props.structureFindings.length > 0
      ? `
        <div class="evolution-structural-notice">
          <span class="empty-icon">📐</span>
          <div class="notice-content">
            <h3>Pillar 2 Structural Debt Active (${props.structureFindings.length})</h3>
            <p>While no high-severity God Documents or isolated silos were detected, the vault contains ${props.structureFindings.length} structural debt item(s) in Pillar 2 (Structure). Address these findings using section addressing or materialized frontmatter to achieve full structural compliance.</p>
          </div>
        </div>
      `
      : `
        <div class="evolution-empty-state">
          <span class="empty-icon">🌱</span>
          <h3>Vault Structure is Balanced & Clean</h3>
          <p>No structural problem signatures (God Documents, Implicit Routers, or Isolated Silos) or debt findings were detected under the active <strong>${orientationLabel}</strong> optimization profile.</p>
        </div>
      `;

    return `
      <div class="evolution-tab-container">
        ${structureFindingsHtml}
        ${emptyMessage}
      </div>
    `;
  }

  const signatureCardsHtml = props.signatures.map((rawSig: any) => {
    const sigId = rawSig.signatureId || rawSig.signature_id || '';
    const docPath = rawSig.documentPath || rawSig.document_path || '';
    const problemType = rawSig.problemType || rawSig.problem_type || 'god_document';
    const confidence = typeof rawSig.confidence === 'number' ? rawSig.confidence : 0;
    const rationale = rawSig.rationale || '';

    const badge = getProblemTypeBadge(problemType);
    const recsForSig = (props.recommendations || []).filter((r: any) => {
      const rSigId = r.signatureId || r.signature_id;
      return rSigId === sigId;
    });

    // Render 10D evidence metrics (strictly <= 5 chars via formatMetricValue)
    const e = rawSig.evidence || {};
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

    const evidenceGridHtml = `
      <div class="evidence-metrics-grid">
        <div class="evidence-pill" title="Cohesion"><span class="ep-label">Coh</span><span class="ep-val">${formatMetricValue(coh)}</span></div>
        <div class="evidence-pill" title="Coupling"><span class="ep-label">Coup</span><span class="ep-val">${formatMetricValue(coup)}</span></div>
        <div class="evidence-pill" title="Ref Concentration"><span class="ep-label">Conc</span><span class="ep-val">${formatMetricValue(conc)}</span></div>
        <div class="evidence-pill" title="Granularity"><span class="ep-label">Gran</span><span class="ep-val">${formatMetricValue(gran)}</span></div>
        <div class="evidence-pill" title="Context Efficiency"><span class="ep-label">Eff</span><span class="ep-val">${formatMetricValue(eff)}</span></div>
        <div class="evidence-pill" title="Discoverability"><span class="ep-label">Disc</span><span class="ep-val">${formatMetricValue(disc)}</span></div>
        <div class="evidence-pill" title="Reusability"><span class="ep-label">Reus</span><span class="ep-val">${formatMetricValue(reus)}</span></div>
        <div class="evidence-pill" title="Semantic Diversity"><span class="ep-label">Div</span><span class="ep-val">${formatMetricValue(div)}</span></div>
        <div class="evidence-pill" title="Navigational Complexity"><span class="ep-label">Nav</span><span class="ep-val">${formatMetricValue(nav)}</span></div>
        <div class="evidence-pill" title="Maintenance Coupling"><span class="ep-label">Maint</span><span class="ep-val">${formatMetricValue(maint)}</span></div>
      </div>
    `;

    // Render Recommendations
    const recsHtml = recsForSig.map((rawRec: any) => {
      const recId = rawRec.recommendationId || rawRec.recommendation_id || '';
      const recConf = typeof rawRec.confidence === 'number' ? rawRec.confidence : 0;
      const strategy = rawRec.strategy || 'topic_decomposition';
      const isAdvisory = recConf < 0.70;
      const confidencePercent = Math.round(recConf * 100);

      const benefits: string[] = Array.isArray(rawRec.expectedBenefits)
        ? rawRec.expectedBenefits
        : (Array.isArray(rawRec.expected_benefits) ? rawRec.expected_benefits : []);
      const tradeOffs: string[] = Array.isArray(rawRec.tradeOffs)
        ? rawRec.tradeOffs
        : (Array.isArray(rawRec.trade_offs) ? rawRec.trade_offs : []);

      const benefitsList = benefits.map((b) => `<li>✔ ${b}</li>`).join('');
      const tradeOffsList = tradeOffs.map((t) => `<li>⚠ ${t}</li>`).join('');

      const txPlan = rawRec.transformationPlan || rawRec.transformation_plan;
      const planId = txPlan ? (txPlan.planId || txPlan.plan_id) : null;

      const isGenerating = generatingPlanRecId === recId;
      const errorMsg = (typeof planGenerationError === 'object' && planGenerationError && (planGenerationError as any).recId === recId)
        ? (planGenerationError as any).message
        : (typeof planGenerationError === 'string' && isGenerating ? planGenerationError : null);

      return `
            <div class="recommendation-card" data-rec-id="${recId}">
              <div class="rec-card-header">
                <span class="rec-strategy-title">${getStrategyLabel(strategy)}</span>
                <div class="rec-header-badges">
                  ${isAdvisory ? '<span class="rec-advisory-badge">Advisory</span>' : ''}
                  <span class="rec-confidence-badge">${confidencePercent}% Fit</span>
                </div>
              </div>

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

              <div class="rec-actions-bar">
                <button type="button"
                        class="btn btn-primary btn-generate-plan${isGenerating ? ' is-loading' : ''}"
                        data-action="generate-plan"
                        data-rec-id="${recId}"
                        ${isGenerating ? 'disabled' : ''}>
                  ${isGenerating
                    ? '<span class="btn-spinner"></span> Generating Plan...'
                    : 'Generate Transformation Plan'}
                </button>
                ${planId ? `
                  <button type="button" class="btn btn-secondary btn-review-plan" data-action="review-plan" data-plan-id="${planId}">
                    Review Plan (${planId})
                  </button>
                ` : ''}
              </div>
              ${errorMsg ? `
                <div class="plan-gen-error-banner">
                  <span class="error-icon">⚠️</span>
                  <span class="error-msg">${errorMsg}</span>
                </div>
              ` : ''}
            </div>
          `;
        }).join('');

    return `
      <div class="signature-card" data-signature-id="${sigId}">
        <div class="signature-header">
          <span class="sig-doc-path" title="${docPath}">${docPath}</span>
          <div class="sig-meta-row">
            <span class="sig-problem-badge ${badge.cssClass}">${badge.label}</span>
            <span class="sig-confidence">Confidence: ${Math.round(confidence * 100)}%</span>
          </div>
        </div>

        <p class="sig-rationale">${rationale}</p>

        <div class="sig-evidence-section">
          <span class="section-micro-title">Empirical Metric Evidence:</span>
          ${evidenceGridHtml}
        </div>

        <div class="sig-recommendations-section">
          <span class="section-micro-title">Candidate KOS Strategies (${recsForSig.length}):</span>
          <div class="recs-list">
            ${recsHtml}
          </div>
        </div>
      </div>
    `;
  }).join('');

  return `
    <div class="evolution-tab-container">
      <div class="evolution-header">
        <h2>Structural Evolution & Knowledge Strategies</h2>
        <p class="evolution-subtitle">Empirical metric evidence, problem signature detection, and transparent trade-off matrices.</p>
      </div>

      ${structureFindingsHtml}

      <div class="signatures-list">
        ${signatureCardsHtml}
      </div>
    </div>
  `;
}
