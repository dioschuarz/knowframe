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

  const signatureCardsHtml = props.signatures.map((sig) => {
    const badge = getProblemTypeBadge(sig.problemType);
    const recsForSig = props.recommendations.filter((r) => r.signatureId === sig.signatureId);

    // Render 10D evidence metrics (strictly <= 5 chars via formatMetricValue)
    const e = sig.evidence;
    const evidenceGridHtml = `
      <div class="evidence-metrics-grid">
        <div class="evidence-pill" title="Cohesion"><span class="ep-label">Coh</span><span class="ep-val">${formatMetricValue(e.cohesion)}</span></div>
        <div class="evidence-pill" title="Coupling"><span class="ep-label">Coup</span><span class="ep-val">${formatMetricValue(e.coupling)}</span></div>
        <div class="evidence-pill" title="Ref Concentration"><span class="ep-label">Conc</span><span class="ep-val">${formatMetricValue(e.referenceConcentration)}</span></div>
        <div class="evidence-pill" title="Granularity"><span class="ep-label">Gran</span><span class="ep-val">${formatMetricValue(e.granularity)}</span></div>
        <div class="evidence-pill" title="Context Efficiency"><span class="ep-label">Eff</span><span class="ep-val">${formatMetricValue(e.contextEfficiency)}</span></div>
        <div class="evidence-pill" title="Discoverability"><span class="ep-label">Disc</span><span class="ep-val">${formatMetricValue(e.discoverability)}</span></div>
        <div class="evidence-pill" title="Reusability"><span class="ep-label">Reus</span><span class="ep-val">${formatMetricValue(e.reusability)}</span></div>
        <div class="evidence-pill" title="Semantic Diversity"><span class="ep-label">Div</span><span class="ep-val">${formatMetricValue(e.semanticDiversity)}</span></div>
        <div class="evidence-pill" title="Navigational Complexity"><span class="ep-label">Nav</span><span class="ep-val">${formatMetricValue(e.navigationalComplexity)}</span></div>
        <div class="evidence-pill" title="Maintenance Coupling"><span class="ep-label">Maint</span><span class="ep-val">${formatMetricValue(e.maintenanceCoupling)}</span></div>
      </div>
    `;

    // Render Recommendations
    const recsHtml = recsForSig.map((rec) => {
      const isAdvisory = rec.confidence < 0.70;
      const confidencePercent = Math.round(rec.confidence * 100);

      const benefitsList = rec.expectedBenefits.map((b) => `<li>✔ ${b}</li>`).join('');
      const tradeOffsList = rec.tradeOffs.map((t) => `<li>⚠ ${t}</li>`).join('');

      return `
        <div class="recommendation-card" data-rec-id="${rec.recommendationId}">
          <div class="rec-card-header">
            <span class="rec-strategy-title">${getStrategyLabel(rec.strategy)}</span>
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
            <button type="button" class="btn btn-primary btn-generate-plan" data-action="generate-plan" data-rec-id="${rec.recommendationId}">
              Generate Transformation Plan
            </button>
            ${rec.transformationPlan ? `
              <button type="button" class="btn btn-secondary btn-review-plan" data-action="review-plan" data-plan-id="${rec.transformationPlan.planId}">
                Review Plan (${rec.transformationPlan.planId})
              </button>
            ` : ''}
          </div>
        </div>
      `;
    }).join('');

    return `
      <div class="signature-card" data-signature-id="${sig.signatureId}">
        <div class="signature-header">
          <div class="sig-title-row">
            <span class="sig-doc-path" title="${sig.documentPath}">${sig.documentPath}</span>
            <span class="sig-problem-badge ${badge.cssClass}">${badge.label}</span>
          </div>
          <span class="sig-confidence">Confidence: ${Math.round(sig.confidence * 100)}%</span>
        </div>

        <p class="sig-rationale">${sig.rationale}</p>

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
