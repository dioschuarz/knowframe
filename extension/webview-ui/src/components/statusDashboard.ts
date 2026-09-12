/**
 * Calm Status Dashboard Component
 *
 * Implements Constitution Principle VI (Zero Action Duplication & State-Driven Emergence).
 * Presents 3 focused pillars:
 * 1. KNOWLEDGE: Connected, Orphan, Excluded counts, and total links.
 * 2. AI CONTEXT: Active Prompt Load (L0) vs Reachable Context (L1-L2), with budget progress bar.
 * 3. CENTRAL HUB & HEALTH: Designated Hub note with [Change] action, ● Synced status.
 * State-driven attention card conditionally emerges if broken links > 0.
 */

import type { ObservatorySummaryPayload } from '../../../src/types/protocol.ts';
import type { KnowledgeQualityScore, QualityDegradationFactor } from '../../../src/types/admission.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';
import { renderQualityPillarHtml } from './qualityPillar.ts';

export interface StatusDashboardProps {
  summary: ObservatorySummaryPayload | {
    totalNotes: number;
    connectedNotes: number;
    orphanNotes: number;
    excludedNotes: number;
    brokenLinksCount: number;
    activeTokens: number;
    reachableTokens: number;
    syncStatus: string;
    lastSyncTimestamp?: number;
  };
  primaryHub?: string | null;
  totalLinks?: number;
  criticalThreshold?: number;
  suggestionsCount?: number;
  qualityScore?: KnowledgeQualityScore | null;
  onOpenHub?: (hubPath: string) => void;
  onChangeHub?: () => void;
  onReviewIssues?: () => void;
  onReviewSuggestions?: () => void;
  onRefreshQuality?: () => void;
  onFactorClick?: (factor: QualityDegradationFactor) => void;
}

export function formatTokenCount(tokens: number): string {
  return formatCompactNumber(tokens);
}

export function renderStatusDashboardHtml(props: StatusDashboardProps): string {
  const { summary, primaryHub, totalLinks = 0, criticalThreshold = 100000, suggestionsCount = 0 } = props;

  const activeTokens = summary.activeTokens || 0;
  const reachableTokens = summary.reachableTokens || 0;
  const brokenCount = summary.brokenLinksCount || 0;
  const hubName = primaryHub || 'No Hub Selected';

  const budgetPct = Math.min(100, Math.round((activeTokens / criticalThreshold) * 100));
  let barColor = 'var(--vscode-charts-blue, #3b82f6)';
  if (budgetPct >= 90) {
    barColor = 'var(--vscode-charts-red, #ef4444)';
  } else if (budgetPct >= 60) {
    barColor = 'var(--vscode-charts-yellow, #f59e0b)';
  }

  const attentionCardHtml = brokenCount > 0
    ? `
    <div class="dashboard-attention-card broken-links-alert" id="dash-broken-alert">
      <div class="attention-header">
        <span class="attention-icon">
          <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <path d="M8 1.5L14.5 13.5H1.5L8 1.5Z"/>
            <line x1="8" y1="6" x2="8" y2="9.5"/>
            <circle cx="8" cy="11.5" r="0.75" fill="currentColor"/>
          </svg>
        </span>
        <span class="attention-title">${formatCompactNumber(brokenCount)} broken links</span>
      </div>
      <button class="dash-action-link" id="dash-btn-review-broken">Review issues</button>
    </div>`
    : '';

  const suggestionsCardHtml = suggestionsCount > 0
    ? `
    <div class="dashboard-attention-card suggestions-alert" id="dash-suggestions-alert">
      <div class="attention-header">
        <span class="attention-icon">
          <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M6 13h4M6.5 15h3M4.5 6.5a3.5 3.5 0 1 1 7 0c0 1.5-1 2.5-1.5 3.5h-4C5.5 9 4.5 8 4.5 6.5Z"/>
          </svg>
        </span>
        <span class="attention-title">${formatCompactNumber(suggestionsCount)} noise patterns</span>
      </div>
      <button class="dash-action-link" id="dash-btn-review-suggestions">Review suggestions</button>
    </div>`
    : '';

  return `
<div class="status-dashboard">
  ${attentionCardHtml}
  ${suggestionsCardHtml}

  <!-- Pillar 1: Knowledge Base -->
  <div class="dashboard-pillar pillar-knowledge">
    <div class="pillar-title">KNOWLEDGE</div>
    <div class="metrics-grid">
      <div class="metric-item">
        <span class="metric-value" id="val-connected">${formatCompactNumber(summary.connectedNotes)}</span>
        <span class="metric-label">Connected</span>
      </div>
      <div class="metric-item">
        <span class="metric-value" id="val-orphans">${formatCompactNumber(summary.orphanNotes)}</span>
        <span class="metric-label">Orphans</span>
      </div>
      <div class="metric-item">
        <span class="metric-value" id="val-excluded">${formatCompactNumber(summary.excludedNotes)}</span>
        <span class="metric-label">Excluded</span>
      </div>
      <div class="metric-item">
        <span class="metric-value" id="val-links">${formatCompactNumber(totalLinks)}</span>
        <span class="metric-label">Total Links</span>
      </div>
    </div>
  </div>

  <!-- Pillar 2: AI Context -->
  <div class="dashboard-pillar pillar-ai-context">
    <div class="pillar-title">AI CONTEXT</div>
    <div class="token-metrics">
      <div class="token-row">
        <span class="token-row-label">Active Prompt Load (L0)</span>
        <span class="token-row-value" id="val-l0-tokens">${formatTokenCount(activeTokens)} tok</span>
      </div>
      <div class="token-progress-track">
        <div class="token-progress-fill" style="width: ${budgetPct}%; background-color: ${barColor};"></div>
      </div>
      <div class="token-row reachable-row">
        <span class="token-row-label">Reachable (L1-L2)</span>
        <span class="token-row-value" id="val-reachable-tokens">${formatTokenCount(reachableTokens)} tok</span>
      </div>
    </div>
  </div>

  <!-- Pillar 3: Central Hub & Health -->
  <div class="dashboard-pillar pillar-hub-health">
    <div class="pillar-title">CENTRAL HUB & HEALTH</div>
    <div class="hub-info-row">
      <div class="hub-details">
        <span class="hub-icon">
          <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="8" cy="8" r="6"/>
            <circle cx="8" cy="8" r="2.5" fill="#38bdf8"/>
          </svg>
        </span>
        <span class="hub-path-link" id="dash-hub-link" title="${hubName}">${hubName}</span>
      </div>
      <button class="dash-action-link" id="dash-btn-change-hub">Change</button>
    </div>
    <div class="sync-status-row">
      <span class="sync-indicator-dot"></span>
      <span class="sync-status-text">Synced</span>
    </div>
  </div>

  <!-- Pillar 4: Quality & Admission -->
  ${props.qualityScore ? renderQualityPillarHtml({ qualityScore: props.qualityScore, onRefreshQuality: props.onRefreshQuality, onFactorClick: props.onFactorClick }) : ''}
</div>
`;
}

export class StatusDashboard {
  private container: HTMLElement | null = null;
  private currentProps: StatusDashboardProps | null = null;

  constructor() {}

  public mount(container: HTMLElement, props: StatusDashboardProps): void {
    this.container = container;
    this.update(props);
  }

  public update(props: StatusDashboardProps): void {
    this.currentProps = props;
    if (!this.container) return;

    this.container.innerHTML = renderStatusDashboardHtml(props);
    this.bindEvents();
  }

  private bindEvents(): void {
    if (!this.container || !this.currentProps) return;

    const hubLink = this.container.querySelector('#dash-hub-link');
    if (hubLink && this.currentProps.primaryHub) {
      hubLink.addEventListener('click', () => {
        this.currentProps?.onOpenHub?.(this.currentProps.primaryHub!);
      });
    }

    const changeBtn = this.container.querySelector('#dash-btn-change-hub');
    if (changeBtn) {
      changeBtn.addEventListener('click', () => {
        this.currentProps?.onChangeHub?.();
      });
    }

    const reviewBrokenBtn = this.container.querySelector('#dash-btn-review-broken');
    if (reviewBrokenBtn) {
      reviewBrokenBtn.addEventListener('click', () => {
        this.currentProps?.onReviewIssues?.();
      });
    }

    const reviewSuggestionsBtn = this.container.querySelector('#dash-btn-review-suggestions');
    if (reviewSuggestionsBtn) {
      reviewSuggestionsBtn.addEventListener('click', () => {
        this.currentProps?.onReviewSuggestions?.();
      });
    }

    const refreshQualityBtn = this.container.querySelector('#quality-refresh-btn');
    if (refreshQualityBtn) {
      refreshQualityBtn.addEventListener('click', () => {
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
