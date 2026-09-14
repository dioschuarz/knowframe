/**
 * Transformation Plan Modal Component (T054)
 *
 * Implements FR-037, FR-038, FR-040, NFR-009, SC-005, Invariant XVI, US6.
 * Renders modal dialog for reviewing and approving structural transformations:
 * - Atomic action list
 * - Unified diff preview
 * - Projected vs actual Before/After metric deltas
 * - Strict user-controlled [Apply Changes] button
 */

import type {
  TransformationPlan,
  MetricDeltaReport,
} from '../../../src/types/evolution.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';

export interface TransformationPlanModalProps {
  plan: TransformationPlan | null;
  isOpen: boolean;
  isApplying?: boolean;
  actualDelta?: MetricDeltaReport | null;
  onApply?: (planId: string) => void;
  onClose?: () => void;
}

export function formatDeltaNumber(val: number): string {
  if (typeof val !== 'number' || isNaN(val)) {
    return '0.0';
  }
  const prefix = val > 0 ? '+' : '';
  if (Math.abs(val) < 1.0) {
    return prefix + val.toFixed(2);
  }
  return prefix + formatCompactNumber(Math.abs(val));
}

export function renderDiffPreviewHtml(rawDiff: string): string {
  if (!rawDiff) return '<div class="diff-empty">No diff content generated.</div>';

  const lines = rawDiff.split('\n');
  const highlighted = lines.map((line) => {
    let cssClass = 'diff-context';
    if (line.startsWith('+') && !line.startsWith('+++')) {
      cssClass = 'diff-addition';
    } else if (line.startsWith('-') && !line.startsWith('---')) {
      cssClass = 'diff-deletion';
    } else if (line.startsWith('@@')) {
      cssClass = 'diff-hunk';
    } else if (line.startsWith('---') || line.startsWith('+++')) {
      cssClass = 'diff-file-header';
    }
    const escaped = line
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    return `<div class="diff-line ${cssClass}">${escaped}</div>`;
  }).join('');

  return `<div class="diff-viewer"><pre><code>${highlighted}</code></pre></div>`;
}

export function renderTransformationPlanModalHtml(props: TransformationPlanModalProps): string {
  if (!props.isOpen || !props.plan) {
    return '';
  }

  const { plan, isApplying, actualDelta } = props;
  const d = actualDelta || plan.projectedDelta;
  const deltaTitle = actualDelta ? 'Actual Before-vs-After Deltas' : 'Projected Structural Deltas';

  // Actions list
  const actionsListHtml = plan.actions.map((act) => {
    const typeLabel = act.type.replace(/_/g, ' ');
    const p = act.payload as any;
    const targetFile = p.filePath || p.sourcePath || '';
    return `
      <div class="modal-action-row">
        <span class="action-type-badge">${typeLabel}</span>
        <span class="action-target-file"><code>${targetFile}</code></span>
      </div>
    `;
  }).join('');

  const diffHtml = renderDiffPreviewHtml(plan.diffPreview);

  return `
    <div class="transformation-modal-overlay" id="transformation-modal" role="dialog" aria-modal="true" aria-labelledby="modal-plan-title">
      <div class="transformation-modal-card">
        <div class="modal-header">
          <div class="modal-title-group">
            <h2 id="modal-plan-title">Transformation Plan Preview</h2>
            <span class="plan-id-badge">${plan.planId}</span>
          </div>
          <button type="button" class="modal-close-btn" id="btn-close-plan-modal" aria-label="Close">✕</button>
        </div>

        <div class="modal-body">
          <div class="invariant-xvi-callout">
            <span class="callout-icon">🛡</span>
            <div class="callout-content">
              <strong>Invariant XVI Protection:</strong> Auto-Apply is OFF. No changes will be made to your workspace documents until you explicitly click <em>Apply Changes</em> below.
            </div>
          </div>

          <div class="modal-section">
            <h3 class="modal-section-title">${deltaTitle}</h3>
            <div class="delta-cards-grid">
              <div class="delta-card">
                <span class="delta-label">Δ Efficiency</span>
                <span class="delta-value ${d.efficiencyDelta >= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(d.efficiencyDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Trace Depth</span>
                <span class="delta-value ${d.traceDepthDelta <= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(d.traceDepthDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Coupling</span>
                <span class="delta-value ${d.couplingDelta <= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(d.couplingDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Document Count</span>
                <span class="delta-value">${d.docCountDelta >= 0 ? '+' : ''}${d.docCountDelta}</span>
              </div>
            </div>
          </div>

          <div class="modal-section">
            <h3 class="modal-section-title">Atomic File System Actions (${plan.actions.length})</h3>
            <div class="modal-actions-list">
              ${actionsListHtml}
            </div>
          </div>

          <div class="modal-section">
            <h3 class="modal-section-title">Unified Diff Preview</h3>
            ${diffHtml}
          </div>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" id="btn-cancel-plan">Cancel</button>
          <button type="button" class="btn btn-primary btn-apply-plan" id="btn-apply-plan-changes" data-plan-id="${plan.planId}">
            ${isApplying ? 'Applying Atomic Changes...' : 'Apply Changes'}
          </button>
        </div>
      </div>
    </div>
  `;
}
