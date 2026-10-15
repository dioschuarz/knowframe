/**
 * Evolution UI controls rendering strategy cards, trade-off matrices,
 * diff preview, and explicit approval button.
 * Implements FR-043 and Constitution Invariants XV & XVI.
 */

import { StrategyRecommendation, TransformationPlan } from '../types/engine';

export interface EvolutionViewProps {
  recommendations: StrategyRecommendation[];
  activePlan: TransformationPlan | null;
  onSelectStrategy: (strategyId: string) => void;
  onApprovePlan: (planId: string, approvalToken: string) => void;
}

export function renderStrategyRecommendationCard(rec: StrategyRecommendation): string {
  const benefitsHtml = rec.expected_benefits
    .map((b) => `<li class="benefit-item">✓ ${escapeHtml(b)}</li>`)
    .join('');
  const tradeoffsHtml = rec.countervailing_trade_offs
    .map((t) => `<li class="tradeoff-item">⚠️ ${escapeHtml(t)}</li>`)
    .join('');

  return `
    <div class="strategy-card" data-strategy-id="${escapeHtml(rec.strategy_id)}">
      <div class="strategy-header">
        <span class="strategy-type">${escapeHtml(rec.strategy_type)}</span>
        <span class="strategy-role role-${escapeHtml(rec.role)}">${escapeHtml(rec.role)}</span>
        <span class="strategy-fit">Fit: ${(rec.fit_score * 100).toFixed(0)}%</span>
      </div>
      <div class="strategy-targets">
        Target: ${rec.target_documents.map((d) => `<code>${escapeHtml(d)}</code>`).join(', ')}
      </div>
      <div class="strategy-tradeoffs-matrix">
        <div class="benefits-col">
          <h4>Expected Benefits</h4>
          <ul>${benefitsHtml}</ul>
        </div>
        <div class="tradeoffs-col">
          <h4>Countervailing Trade-Offs (Invariant XV)</h4>
          <ul>${tradeoffsHtml}</ul>
        </div>
      </div>
      <button class="btn-select-strategy" data-id="${escapeHtml(rec.strategy_id)}">
        Select Strategy
      </button>
    </div>
  `;
}

export function renderTransformationApprovalGate(plan: TransformationPlan): string {
  return `
    <div class="plan-approval-modal">
      <div class="modal-header">
        <h3>Transformation Plan: ${escapeHtml(plan.plan_id)}</h3>
        <span class="approval-badge">Requires User Approval (Invariant XVI)</span>
      </div>
      <div class="modal-diff">
        <pre><code>${escapeHtml(plan.unified_diff)}</code></pre>
      </div>
      <div class="modal-actions">
        <input type="hidden" id="approval-token" value="${escapeHtml(plan.approval_token)}" />
        <button id="btn-approve-transformation" class="btn-primary">
          Approve & Apply Changes
        </button>
      </div>
    </div>
  `;
}

function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}
