/**
 * Fourth Pillar UI: Quality & Admission Component
 *
 * Implements Constitution Principle IX (Network-Aware Knowledge Quality Scoring).
 * Displays:
 * 1. Overall Quality Score gauge (0–100) with color coding (Green >= 90, Yellow >= 75, Red < 75).
 * 2. 4 Pillar meters: Integrity (30%), Consistency (25%), Discoverability (25%), Freshness (20%).
 * 3. Debt Accounting counters: Legacy Debt, New Regressions, Resolved Debt.
 * 4. Interactive degradation factors list with topological impact.
 */

import type { KnowledgeQualityScore, QualityDegradationFactor } from '../../src/types/admission.ts';

export interface QualityPillarProps {
  qualityScore?: KnowledgeQualityScore | null;
  onFactorClick?: (factor: QualityDegradationFactor) => void;
  onRefreshQuality?: () => void;
}

export function getScoreColor(score: number): string {
  if (score >= 90) return 'var(--vscode-charts-green, #22c55e)';
  if (score >= 75) return 'var(--vscode-charts-yellow, #eab308)';
  return 'var(--vscode-charts-red, #ef4444)';
}

export function renderQualityPillarHtml(props: QualityPillarProps): string {
  const score = props.qualityScore;

  if (!score) {
    return `
      <div class="dashboard-pillar pillar-quality" id="pillar-quality">
        <div class="pillar-title">QUALITY & ADMISSION</div>
        <div class="quality-loading">Computing quality score...</div>
      </div>
    `;
  }

  const overallColor = getScoreColor(score.overall_score);

  const factorItemsHtml = score.top_degradation_factors.length > 0
    ? `
      <div class="quality-factors-list">
        <div class="factors-title">Top Degradation Factors:</div>
        ${score.top_degradation_factors
          .slice(0, 3)
          .map(
            (f, idx) => `
          <div class="quality-factor-item" data-index="${idx}" title="${f.description}">
            <span class="factor-badge factor-${f.category}">-${f.deduction_points.toFixed(1)}</span>
            <span class="factor-node">${f.affected_node}</span>
            <span class="factor-weight">${f.betweenness_weight.toFixed(1)}x</span>
          </div>
        `
          )
          .join('')}
      </div>
    `
    : '';

  return `
    <div class="dashboard-pillar pillar-quality" id="pillar-quality">
      <div class="pillar-header">
        <div class="pillar-title">QUALITY & ADMISSION</div>
        <button class="quality-refresh-btn" id="quality-refresh-btn" title="Recalculate Knowledge Quality">↻</button>
      </div>

      <div class="quality-gauge-row">
        <div class="quality-score-circle" style="border-color: ${overallColor};">
          <span class="score-number" style="color: ${overallColor};">${score.overall_score.toFixed(0)}</span>
          <span class="score-max">/100</span>
        </div>
        <div class="quality-pillars-mini">
          <div class="pillar-mini-row" title="Broken links & anchor targets (30%)">
            <span class="mini-label">Integrity</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" style="width: ${score.integrity_score}%; background: ${getScoreColor(score.integrity_score)};"></div>
            </div>
            <span class="mini-val">${score.integrity_score.toFixed(0)}%</span>
          </div>
          <div class="pillar-mini-row" title="Semantic collisions & conflicts (25%)">
            <span class="mini-label">Consistency</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" style="width: ${score.consistency_score}%; background: ${getScoreColor(score.consistency_score)};"></div>
            </div>
            <span class="mini-val">${score.consistency_score.toFixed(0)}%</span>
          </div>
          <div class="pillar-mini-row" title="Orphans & hop reachability (25%)">
            <span class="mini-label">Discoverability</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" style="width: ${score.discoverability_score}%; background: ${getScoreColor(score.discoverability_score)};"></div>
            </div>
            <span class="mini-val">${score.discoverability_score.toFixed(0)}%</span>
          </div>
          <div class="pillar-mini-row" title="Technical drift & unvalidated notes (20%)">
            <span class="mini-label">Freshness</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" style="width: ${score.freshness_score}%; background: ${getScoreColor(score.freshness_score)};"></div>
            </div>
            <span class="mini-val">${score.freshness_score.toFixed(0)}%</span>
          </div>
        </div>
      </div>

      <div class="debt-accounting-row">
        <div class="debt-stat" title="Baseline accepted brownfield debt">
          <span class="debt-value">${score.legacy_debt_count}</span>
          <span class="debt-label">Legacy</span>
        </div>
        <div class="debt-stat ${score.regressions_count > 0 ? 'has-regressions' : ''}" title="New regressions exceeding baseline">
          <span class="debt-value" style="color: ${score.regressions_count > 0 ? 'var(--vscode-charts-red, #ef4444)' : 'inherit'};">${score.regressions_count}</span>
          <span class="debt-label">New Regr.</span>
        </div>
        <div class="debt-stat" title="Resolved legacy defects">
          <span class="debt-value" style="color: var(--vscode-charts-green, #22c55e);">${score.resolved_debt_count ?? 0}</span>
          <span class="debt-label">Resolved</span>
        </div>
      </div>

      ${factorItemsHtml}
    </div>
  `;
}

export class QualityPillar {
  private container: HTMLElement | null = null;
  private currentProps: QualityPillarProps | null = null;

  public mount(container: HTMLElement, props: QualityPillarProps): void {
    this.container = container;
    this.update(props);
  }

  public update(props: QualityPillarProps): void {
    this.currentProps = props;
    if (!this.container) return;

    this.container.innerHTML = renderQualityPillarHtml(props);
    this.bindEvents();
  }

  private bindEvents(): void {
    if (!this.container || !this.currentProps) return;

    const refreshBtn = this.container.querySelector('#quality-refresh-btn');
    if (refreshBtn) {
      refreshBtn.addEventListener('click', () => {
        this.currentProps?.onRefreshQuality?.();
      });
    }

    const factorItems = this.container.querySelectorAll('.quality-factor-item');
    factorItems.forEach((el) => {
      el.addEventListener('click', () => {
        const idx = parseInt(el.getAttribute('data-index') || '0', 10);
        const factor = this.currentProps?.qualityScore?.top_degradation_factors[idx];
        if (factor) {
          this.currentProps?.onFactorClick?.(factor);
        }
      });
    });
  }
}
