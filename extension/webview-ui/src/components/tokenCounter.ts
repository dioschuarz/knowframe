import { TokenSummary } from '../../../src/types/graph';
import { formatCompactNumber } from '../utils/numberFormat';

export function renderTokenCounter(container: HTMLElement, summary: TokenSummary): void {
  const activeFormatted = formatCompactNumber(summary.activeHarnessTokens);
  const subcallFormatted = formatCompactNumber(summary.subcallAccessibleTokens);
  const warnFormatted = formatCompactNumber(summary.warningThreshold);
  const critFormatted = formatCompactNumber(summary.criticalThreshold);

  let statusBadgeColor = '#10B981'; // Green
  let statusText = 'NORMAL';
  if (summary.status === 'warning') {
    statusBadgeColor = '#F59E0B'; // Amber
    statusText = 'WARNING';
  } else if (summary.status === 'critical') {
    statusBadgeColor = '#EF4444'; // Red
    statusText = 'CRITICAL OVERLOAD';
  }

  container.innerHTML = `
    <div class="token-counter-card">
      <div class="token-header">
        <span class="token-title">Token Economics</span>
        <span class="token-badge" style="background-color: ${statusBadgeColor};">${statusText}</span>
      </div>
      <div class="token-metrics">
        <div class="metric">
          <span class="metric-label">Active Prompt Load (L0):</span>
          <span class="metric-value">${activeFormatted} tokens</span>
        </div>
        <div class="metric">
          <span class="metric-label">Subcall Accessible (L1+):</span>
          <span class="metric-value">${subcallFormatted} tokens</span>
        </div>
      </div>
      <div class="token-thresholds">
        <span>Ceilings: Soft ${warnFormatted} | Hard ${critFormatted}</span>
      </div>
    </div>
  `;
}
