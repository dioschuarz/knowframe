/**
 * Conflicting Constraint Warning Banner Component (T052)
 *
 * Implements FR-022, SC-002 (US3).
 * Surfaces contradictory hard constraints in real-time (<= 10ms SAT evaluation).
 */

import type { ConstraintConflict } from '../../../src/types/evolution.ts';

export interface ConflictBannerProps {
  conflicts: ConstraintConflict[];
  onResolve?: (conflictIndex: number) => void;
}

export function renderConflictBannerHtml(props: ConflictBannerProps): string {
  if (!props.conflicts || props.conflicts.length === 0) {
    return `
      <div class="constraint-status-banner constraint-status-valid">
        <span class="status-icon">✔</span>
        <span class="status-text">All active constraint rules are mutually satisfiable.</span>
      </div>
    `;
  }

  const conflictItemsHtml = props.conflicts.map((conflict, idx) => {
    const rulesList = conflict.conflictingRules.map((r) => `<code class="conflicting-rule-chip">${r}</code>`).join(' & ');

    return `
      <div class="conflict-item-card" data-conflict-index="${idx}">
        <div class="conflict-header">
          <span class="conflict-rules-label">Contradiction:</span> ${rulesList}
        </div>
        <div class="conflict-reason">${conflict.reason}</div>
        <div class="conflict-suggestion">
          <span class="suggestion-icon">💡</span>
          <span class="suggestion-text">${conflict.suggestion}</span>
        </div>
      </div>
    `;
  }).join('');

  return `
    <div class="constraint-conflict-banner" role="alert" aria-live="polite">
      <div class="banner-title-bar">
        <span class="banner-icon">⚠️</span>
        <span class="banner-title">Conflicting Hard Constraints Detected (${props.conflicts.length})</span>
      </div>
      <p class="banner-description">The following constraint rules cannot be satisfied simultaneously. Conflicting rules will be ignored or disqualified until resolved.</p>
      <div class="conflict-items-list">
        ${conflictItemsHtml}
      </div>
    </div>
  `;
}
