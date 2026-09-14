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

  const { plan: rawPlan, isApplying, actualDelta: rawActualDelta } = props;
  const plan: any = rawPlan;
  const planId = plan.planId || plan.plan_id || '';
  const diffPreview = plan.diffPreview || plan.diff_preview || '';
  const projectedDelta = plan.projectedDelta || plan.projected_delta || {};
  const actualDelta = rawActualDelta || plan.actualDelta || plan.actual_delta || null;
  const d = actualDelta || projectedDelta;
  const deltaTitle = actualDelta ? 'Actual Before-vs-After Deltas' : 'Projected Structural Deltas';

  const effDelta = typeof d.efficiencyDelta === 'number' ? d.efficiencyDelta : (typeof d.efficiency_delta === 'number' ? d.efficiency_delta : 0);
  const traceDelta = typeof d.traceDepthDelta === 'number' ? d.traceDepthDelta : (typeof d.trace_depth_delta === 'number' ? d.trace_depth_delta : 0);
  const coupDelta = typeof d.couplingDelta === 'number' ? d.couplingDelta : (typeof d.coupling_delta === 'number' ? d.coupling_delta : 0);
  const docDelta = typeof d.docCountDelta === 'number' ? d.docCountDelta : (typeof d.doc_count_delta === 'number' ? d.doc_count_delta : 0);

  // Actions list
  const actions: any[] = Array.isArray(plan.actions) ? plan.actions : [];
  const actionsListHtml = actions.map((act) => {
    const typeLabel = (act.type || '').replace(/_/g, ' ');
    const p = act.payload || {};
    const targetFile = p.filePath || p.file_path || p.sourcePath || p.source_path || '';
    return `
      <div class="modal-action-row">
        <span class="action-type-badge">${typeLabel}</span>
        <span class="action-target-file"><code>${targetFile}</code></span>
      </div>
    `;
  }).join('');

  const diffHtml = renderDiffPreviewHtml(diffPreview);

  return `
    <div class="transformation-modal-overlay" id="transformation-modal" role="dialog" aria-modal="true" aria-labelledby="modal-plan-title">
      <div class="transformation-modal-card">
        <div class="modal-header">
          <div class="modal-title-group">
            <h2 id="modal-plan-title">Transformation Plan Preview</h2>
            <span class="plan-id-badge">${planId}</span>
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
                <span class="delta-value ${effDelta >= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(effDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Trace Depth</span>
                <span class="delta-value ${traceDelta <= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(traceDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Coupling</span>
                <span class="delta-value ${coupDelta <= 0 ? 'delta-positive' : 'delta-negative'}">${formatDeltaNumber(coupDelta)}</span>
              </div>
              <div class="delta-card">
                <span class="delta-label">Δ Document Count</span>
                <span class="delta-value">${docDelta >= 0 ? '+' : ''}${docDelta}</span>
              </div>
            </div>
          </div>

          <div class="modal-section">
            <h3 class="modal-section-title">Atomic File System Actions (${actions.length})</h3>
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
