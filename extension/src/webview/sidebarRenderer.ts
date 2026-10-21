import type {
  ScanResultPayload,
  ManagerSummaryState,
  DocumentationOrientation,
} from '../types/protocol.ts';
import type { KnowledgeQualityScore, KnowledgeIssuesReport } from '../types/admission.ts';

export interface SidebarRenderOptions {
  cspPolicy: string;
  nonce: string;
  initialData?: ScanResultPayload | null;
  initialQuality?: KnowledgeQualityScore | null;
  initialSummary?: ManagerSummaryState | null;
  initialIssues?: KnowledgeIssuesReport | null;
  orientation?: DocumentationOrientation;
}

export function renderSidebarHtml(options: SidebarRenderOptions): string {
  const {
    cspPolicy,
    nonce,
    initialData = null,
    initialQuality = null,
    initialSummary = null,
    initialIssues = null,
    orientation = 'ai_oriented',
  } = options;

  let initialFindings = 0;
  let initialBaselined = 0;
  let initialRegressions = 0;

  if (initialSummary && initialSummary.health) {
    initialFindings = initialSummary.health.active_issues_count != null
      ? initialSummary.health.active_issues_count
      : (initialSummary.health.new_regressions != null
          ? initialSummary.health.new_regressions
          : (initialSummary.health.findings_count != null ? initialSummary.health.findings_count : 0));
    initialBaselined = initialSummary.health.legacy_debt != null
      ? initialSummary.health.legacy_debt
      : (initialSummary.health.legacy_debt_count || 0);
    initialRegressions = initialSummary.health.new_regressions != null ? initialSummary.health.new_regressions : 0;
  } else if (initialIssues) {
    initialFindings = initialIssues.active_issues_count != null
      ? initialIssues.active_issues_count
      : (initialIssues.new_regressions != null
          ? initialIssues.new_regressions
          : (initialIssues.issues ? initialIssues.issues.filter((i: any) => !i.is_legacy).length : 0));
    initialBaselined = initialIssues.legacy_debt_count != null
      ? initialIssues.legacy_debt_count
      : (initialIssues.legacy_debt != null
          ? initialIssues.legacy_debt
          : (initialIssues.issues ? initialIssues.issues.filter((i: any) => !!i.is_legacy).length : 0));
    initialRegressions = initialFindings;
  } else if (initialQuality) {
    initialFindings = initialQuality.active_issues_count != null
      ? initialQuality.active_issues_count
      : (initialQuality.new_regressions_count != null
          ? initialQuality.new_regressions_count
          : (initialQuality.findings_count != null ? initialQuality.findings_count : 0));
    initialBaselined = initialQuality.legacy_debt_count != null ? initialQuality.legacy_debt_count : 0;
    initialRegressions = initialQuality.regressions_count != null ? initialQuality.regressions_count : initialFindings;
  }

  const dataJson = JSON.stringify(initialData || {});
  const qualityJson = JSON.stringify(initialQuality || null);
  const summaryJson = JSON.stringify(initialSummary || null);
  const issuesJson = JSON.stringify(initialIssues || null);

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="${cspPolicy}">
  <title>KnowFrame - Manager</title>
  <style>
    body {
      font-family: var(--vscode-font-family, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif);
      font-size: var(--vscode-font-size, 12px);
      color: var(--vscode-foreground, #d4d4d4);
      background-color: var(--vscode-sideBar-background, #252526);
      padding: 10px;
      margin: 0;
      box-sizing: border-box;
    }
    .status-dashboard {
      display: flex;
      flex-direction: column;
      gap: 12px;
    }
    .dashboard-pillar {
      background: var(--vscode-editor-background, #1e1e1e);
      border: 1px solid var(--vscode-widget-border, #333333);
      border-radius: 6px;
      padding: 10px;
    }
    .pillar-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 8px;
    }
    .pillar-header-actions {
      display: inline-flex;
      align-items: center;
      gap: 4px;
    }
    .pillar-title {
      font-size: 10px;
      font-weight: 700;
      letter-spacing: 0.6px;
      color: var(--vscode-descriptionForeground, #888);
      text-transform: uppercase;
    }
    .dash-action-btn {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 5px;
      background: transparent;
      border: 1px solid var(--vscode-widget-border, #333);
      color: var(--vscode-textLink-foreground, #38bdf8);
      padding: 3px 8px;
      border-radius: 4px;
      font-size: 11px;
      cursor: pointer;
      white-space: nowrap;
      transition: background 0.12s ease;
    }
    .dash-action-btn.icon-only-btn {
      padding: 3px 6px;
      min-width: 22px;
      height: 22px;
    }
    .dash-action-btn:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.1));
      text-decoration: underline;
    }
    .dash-action-btn.primary-action {
      background: rgba(56, 189, 248, 0.1);
      border-color: rgba(56, 189, 248, 0.3);
      font-weight: 600;
    }
    .dash-action-btn.primary-action:hover {
      background: rgba(56, 189, 248, 0.2);
    }
    .gear-action-btn {
      position: relative;
      color: var(--vscode-textLink-foreground, #38bdf8);
      border-color: rgba(56, 189, 248, 0.3);
      background: rgba(56, 189, 248, 0.1);
    }
    .gear-action-btn:hover {
      background: rgba(56, 189, 248, 0.2);
      text-decoration: none;
    }
    .gear-action-btn[data-tooltip]::after {
      content: attr(data-tooltip);
      position: absolute;
      right: 0;
      bottom: calc(100% + 6px);
      background: var(--vscode-editorHoverWidget-background, #252526);
      color: var(--vscode-editorHoverWidget-foreground, #cccccc);
      border: 1px solid var(--vscode-editorHoverWidget-border, #454545);
      border-radius: 4px;
      padding: 4px 8px;
      font-size: 10.5px;
      font-weight: 500;
      white-space: nowrap;
      pointer-events: none;
      opacity: 0;
      transform: translateY(2px);
      transition: opacity 0.14s ease, transform 0.14s ease;
      z-index: 100;
      box-shadow: 0 4px 10px rgba(0, 0, 0, 0.35);
    }
    .gear-action-btn[data-tooltip]:hover::after {
      opacity: 1;
      transform: translateY(0);
    }
    .quality-gauge-row {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 8px;
    }
    .quality-score-circle {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      width: 52px;
      height: 52px;
      min-width: 52px;
      border-radius: 50%;
      border: 3px solid var(--vscode-charts-green, #22c55e);
      background: rgba(255, 255, 255, 0.02);
      box-sizing: border-box;
    }
    .score-number {
      font-size: 16px;
      font-weight: 800;
      line-height: 1.1;
      color: var(--vscode-charts-green, #22c55e);
    }
    .score-max {
      font-size: 8.5px;
      color: var(--vscode-descriptionForeground, #888);
    }
    .findings-container {
      display: flex;
      flex-direction: column;
      gap: 2px;
      flex: 1;
    }
    .findings-summary-text {
      font-size: 11.5px;
      font-weight: 600;
      color: var(--vscode-editor-foreground, #fff);
    }
    .findings-debt-detail {
      font-size: 10px;
      color: var(--vscode-descriptionForeground, #888);
    }
    .quality-pillars-mini {
      display: flex;
      flex-direction: column;
      gap: 4px;
      margin-top: 6px;
      border-top: 1px solid rgba(255, 255, 255, 0.06);
      padding-top: 6px;
    }
    .pillar-mini-row {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 10px;
      cursor: pointer;
      padding: 2px 3px;
      border-radius: 3px;
      transition: background 0.12s ease;
    }
    .pillar-mini-row:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.06));
    }
    .mini-label {
      width: 78px;
      min-width: 78px;
      color: var(--vscode-descriptionForeground, #aaa);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .mini-bar-track {
      flex: 1;
      height: 4px;
      background: rgba(255, 255, 255, 0.1);
      border-radius: 2px;
      overflow: hidden;
    }
    .mini-bar-fill {
      height: 100%;
      border-radius: 2px;
      transition: width 0.3s ease, background-color 0.3s ease;
    }
    .mini-val {
      font-weight: 600;
      width: 28px;
      min-width: 28px;
      text-align: right;
      font-variant-numeric: tabular-nums;
      color: var(--vscode-editor-foreground, #fff);
    }
    .stats-pills-row {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 4px;
      background: rgba(255, 255, 255, 0.02);
      border: 1px solid rgba(255, 255, 255, 0.05);
      border-radius: 4px;
      padding: 5px 3px;
      text-align: center;
      margin-bottom: 8px;
    }
    .pill-stat {
      display: flex;
      flex-direction: column;
      align-items: center;
      min-width: 0;
    }
    .pill-value {
      font-size: 12px;
      font-weight: 700;
      color: var(--vscode-editor-foreground, #fff);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
    .pill-label {
      font-size: 8px;
      color: var(--vscode-descriptionForeground, #888);
      margin-top: 1px;
      text-transform: uppercase;
      letter-spacing: 0.3px;
      white-space: nowrap;
    }
    .structure-actions-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 6px;
    }
    .hub-info-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 6px;
    }
    .hub-details {
      display: flex;
      align-items: center;
      gap: 6px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 100%;
    }
    .hub-icon {
      font-size: 12px;
    }
    .hub-path-link {
      font-size: 12px;
      font-weight: 600;
      color: var(--vscode-textLink-foreground, #38bdf8);
      cursor: pointer;
      overflow: hidden;
      text-overflow: ellipsis;
    }
    .hub-path-link:hover {
      text-decoration: underline;
    }
    .opportunity-count-row {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 11.5px;
      color: var(--vscode-foreground, #d4d4d4);
      background: rgba(255, 255, 255, 0.02);
      border: 1px solid rgba(255, 255, 255, 0.05);
      border-radius: 4px;
      padding: 4px 6px;
    }
    .opportunity-count-badge {
      font-weight: 700;
      color: var(--vscode-textLink-foreground, #38bdf8);
    }
    .sync-status-row {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 11.5px;
      margin-bottom: 4px;
    }
    .sync-indicator-dot {
      width: 7px;
      height: 7px;
      border-radius: 50%;
      background: #10b981;
      display: inline-block;
    }
    .sync-commit-text {
      font-size: 10.5px;
      color: var(--vscode-descriptionForeground, #888);
      font-family: var(--vscode-editor-font-family, monospace);
    }
    .sync-disclaimer {
      font-size: 9.5px;
      color: var(--vscode-descriptionForeground, #777);
      margin-top: 4px;
      font-style: italic;
    }
    .collapsible-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      cursor: pointer;
      user-select: none;
      padding: 3px 2px;
      border-radius: 4px;
      transition: background 0.12s ease;
    }
    .collapsible-header:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.05));
    }
    .collapsible-chevron {
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--vscode-descriptionForeground, #888);
      transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    }
    .collapsible-header.collapsed .collapsible-chevron {
      transform: rotate(-90deg);
    }
    .top-notes {
      list-style: none;
      padding: 0;
      margin: 0;
    }
    .note-item {
      padding: 4px 6px;
      border-radius: 4px;
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 11px;
      gap: 6px;
      transition: background 0.12s ease;
    }
    .note-item:hover {
      background: var(--vscode-list-hoverBackground);
    }
    .note-name {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      flex: 1;
      color: var(--vscode-editor-foreground);
    }
    .note-meta-badges {
      display: flex;
      align-items: center;
      gap: 4px;
      flex-shrink: 0;
    }
    .note-conns-badge {
      font-size: 9.5px;
      font-weight: 600;
      padding: 1px 5px;
      border-radius: 3px;
      background: rgba(56, 189, 248, 0.12);
      color: #38bdf8;
      border: 1px solid rgba(56, 189, 248, 0.25);
    }
    .note-tokens {
      font-size: 9.5px;
      color: var(--vscode-descriptionForeground);
    }
    .factors-section, .top-notes-section {
      margin-top: 6px;
      border-top: 1px solid rgba(255, 255, 255, 0.06);
      padding-top: 4px;
    }
    .factors-title {
      font-size: 9.5px;
      font-weight: 600;
      color: var(--vscode-descriptionForeground, #888);
      text-transform: uppercase;
      letter-spacing: 0.4px;
    }
  </style>
</head>
<body>
  <div class="status-dashboard">
    <!-- Card 1: HEALTH -->
    <div class="dashboard-pillar pillar-health" id="pillar-health">
      <div class="pillar-header">
        <div class="pillar-title">HEALTH</div>
        <div class="pillar-header-actions">
          <button class="dash-action-btn primary-action" id="dash-btn-explore-graph" title="Open Knowledge Graph">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" style="vertical-align: -1px;">
              <circle cx="12" cy="12" r="3" fill="currentColor"/>
              <circle cx="5" cy="6" r="2" fill="currentColor"/>
              <circle cx="19" cy="6" r="2" fill="currentColor"/>
              <circle cx="5" cy="18" r="2" fill="currentColor"/>
              <circle cx="19" cy="18" r="2" fill="currentColor"/>
              <line x1="6.5" y1="7.5" x2="10" y2="10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <line x1="17.5" y1="7.5" x2="14" y2="10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <line x1="6.5" y1="16.5" x2="10" y2="13.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <line x1="17.5" y1="16.5" x2="14" y2="13.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
            Explore graph
          </button>
        </div>
      </div>

      <div class="quality-gauge-row">
        <div class="quality-score-circle" id="quality-circle" style="border-color: var(--vscode-charts-green, #22c55e);">
          <span class="score-number" id="val-quality-score">100</span>
          <span class="score-max">/100</span>
        </div>
        <div class="findings-container">
          <div class="findings-summary-text" id="dash-findings-count" style="color: ${initialFindings > 0 ? 'var(--vscode-charts-yellow, #eab308)' : 'var(--vscode-charts-green, #22c55e)'};">${initialFindings === 1 ? '1 finding needs attention' : (initialFindings + ' findings need attention')}</div>
          <div class="findings-debt-detail" id="dash-debt-counts">${initialRegressions} regressions · ${initialBaselined} baselined</div>
        </div>
      </div>

      <!-- 5-Pillar Breakdown Bars -->
      <div class="quality-pillars-mini">
        <div class="pillar-mini-row" data-dimension="integrity" title="Integrity - Broken links and target anchors">
          <span class="mini-label">Integrity</span>
          <div class="mini-bar-track">
            <div class="mini-bar-fill" id="fill-integrity" style="width: 100%; background-color: var(--vscode-charts-green, #22c55e);"></div>
          </div>
          <span class="mini-val" id="val-integrity">100%</span>
        </div>
        <div class="pillar-mini-row" data-dimension="structure" title="Structure - Missing fragment metadata & oversized documents">
          <span class="mini-label">Structure</span>
          <div class="mini-bar-track">
            <div class="mini-bar-fill" id="fill-structure" style="width: 100%; background-color: var(--vscode-charts-green, #22c55e);"></div>
          </div>
          <span class="mini-val" id="val-structure">100%</span>
        </div>
        <div class="pillar-mini-row" data-dimension="consistency" title="Consistency - Semantic collisions & duplicates">
          <span class="mini-label">Consistency</span>
          <div class="mini-bar-track">
            <div class="mini-bar-fill" id="fill-consistency" style="width: 100%; background-color: var(--vscode-charts-green, #22c55e);"></div>
          </div>
          <span class="mini-val" id="val-consistency">100%</span>
        </div>
        <div class="pillar-mini-row" data-dimension="discoverability" title="Discoverability - Isolated documents & hop reachability">
          <span class="mini-label">Discoverability</span>
          <div class="mini-bar-track">
            <div class="mini-bar-fill" id="fill-discoverability" style="width: 100%; background-color: var(--vscode-charts-green, #22c55e);"></div>
          </div>
          <span class="mini-val" id="val-discoverability">100%</span>
        </div>
        <div class="pillar-mini-row" data-dimension="freshness" title="Freshness - Technical drift & stale code citations">
          <span class="mini-label">Freshness</span>
          <div class="mini-bar-track">
            <div class="mini-bar-fill" id="fill-freshness" style="width: 100%; background-color: var(--vscode-charts-green, #22c55e);"></div>
          </div>
          <span class="mini-val" id="val-freshness">100%</span>
        </div>
      </div>
    </div>

    <!-- Card 2: STRUCTURE -->
    <div class="dashboard-pillar pillar-structure" id="pillar-structure">
      <div class="pillar-header">
        <div class="pillar-title">STRUCTURE</div>
        <button class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-change-hub" title="Change central hub note" data-tooltip="Change central hub note" aria-label="Change central hub note">
          <svg viewBox="0 0 16 16" width="11" height="11" fill="currentColor">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.3h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.3-.4V7.4l-2.4-.5-.3-.7 1.3-2-.8-.9-2 1.3-.7-.2zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
          </svg>
        </button>
      </div>

      <!-- Standardized Terminology 4-Pill Grid -->
      <div class="stats-pills-row" title="Knowledge Base Document & Reference Metrics">
        <div class="pill-stat" title="Total active Markdown documents in knowledge base">
          <span class="pill-value" id="val-documents">0</span>
          <span class="pill-label">Documents</span>
        </div>
        <div class="pill-stat" title="Documents participating in references">
          <span class="pill-value" id="val-connected">0</span>
          <span class="pill-label">Connected</span>
        </div>
        <div class="pill-stat" title="Documents with zero references">
          <span class="pill-value" id="val-isolated">0</span>
          <span class="pill-label">Isolated</span>
        </div>
        <div class="pill-stat" title="Total references in graph">
          <span class="pill-value" id="val-references">0</span>
          <span class="pill-label">References</span>
        </div>
      </div>

      <div class="hub-info-row">
        <div class="hub-details">
          <span class="hub-icon">
            <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="8" cy="8" r="6"/>
              <circle cx="8" cy="8" r="2.5" fill="#38bdf8"/>
            </svg>
          </span>
          <span class="hub-path-link" id="dash-hub-link" title="Designated primary hub">00_meta/wiki_index.md</span>
        </div>
      </div>

      <!-- Top Connected Notes (Collapsible) -->
      <div class="top-notes-section" id="top-notes-section">
        <div class="collapsible-header collapsed" id="top-notes-header" title="Toggle top connected notes">
          <span class="factors-title" style="margin: 0;">Top Connected Notes</span>
          <span class="collapsible-chevron" id="top-notes-chevron">
            <svg viewBox="0 0 16 16" width="10" height="10" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="4 6 8 10 12 6"></polyline>
            </svg>
          </span>
        </div>
        <ul id="top-notes-list" class="top-notes" style="display: none; margin-top: 4px;">
          <li style="color: var(--vscode-descriptionForeground); font-size: 11px; padding: 4px;">No documents found</li>
        </ul>
      </div>
    </div>

    <!-- Card 3: EVOLUTION -->
    <div class="dashboard-pillar pillar-evolution" id="pillar-evolution">
      <div class="pillar-header">
        <div class="pillar-title">EVOLUTION</div>
        <button class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-configure-evolution" title="Configure evolution mode & strategies" data-tooltip="Configure evolution mode & strategies" aria-label="Configure evolution mode & strategies">
          <svg viewBox="0 0 16 16" width="11" height="11" fill="currentColor">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.3h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.3-.4V7.4l-2.4-.5-.3-.7 1.3-2-.8-.9-2 1.3-.7-.2zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
          </svg>
        </button>
      </div>

      <div class="hub-info-row">
        <div class="hub-details">
          <span class="hub-icon" id="evolution-mode-icon">🤖</span>
          <span class="hub-path-link" id="evolution-mode-badge" title="Click to view evolution details">AI-Oriented</span>
        </div>
      </div>
      <div class="opportunity-count-row" id="evolution-opportunity-row">
        <span class="opportunity-count-badge" id="val-opportunity-count">0</span>
        <span id="evolution-opportunity-text">structural opportunities detected</span>
      </div>
    </div>

    <!-- Card 4: SYNC -->
    <div class="dashboard-pillar pillar-sync" id="pillar-sync">
      <div class="pillar-header">
        <div class="pillar-title">SYNC & HARNESS</div>
        <button class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-open-harness" title="Configure AI harness & agent tools" data-tooltip="Configure AI harness & agent tools" aria-label="Configure AI harness & agent tools">
          <svg viewBox="0 0 16 16" width="11" height="11" fill="currentColor">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.3h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.3-.4V7.4l-2.4-.5-.3-.7 1.3-2-.8-.9-2 1.3-.7-.2zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
          </svg>
        </button>
      </div>
      <div class="sync-status-row">
        <span class="sync-indicator-dot" id="sync-indicator-dot"></span>
        <span class="sync-status-text" id="sync-status-text" style="font-weight: 600;">Synced</span>
      </div>
      <div class="sync-commit-text" id="sync-commit-text">Last commit: #uninitialized</div>
      <div class="sync-disclaimer">Synced ≠ Healthy (Git persistence active)</div>
    </div>
  </div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();

    // Wire buttons
    document.getElementById('dash-btn-explore-graph')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'openGraph' });
    });
    document.getElementById('dash-btn-change-hub')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'changeHub' });
    });
    document.getElementById('dash-btn-open-harness')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'openSettings', tab: 'harness' });
    });
    document.getElementById('dash-btn-configure-evolution')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'navigateToDetail', primaryTab: 'insights', subTab: 'evolution' });
    });
    document.getElementById('evolution-mode-badge')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'navigateToDetail', primaryTab: 'insights', subTab: 'evolution' });
    });
    document.getElementById('evolution-opportunity-row')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'navigateToDetail', primaryTab: 'insights', subTab: 'evolution' });
    });

    // Deep-linking clicks on Health dimension rows
    document.querySelectorAll('.pillar-mini-row').forEach(row => {
      row.addEventListener('click', () => {
        const dimension = row.getAttribute('data-dimension');
        vscode.postMessage({
          type: 'navigateToDetail',
          primaryTab: 'insights',
          subTab: 'health',
          targetSection: dimension || undefined
        });
      });
    });

    // Top notes collapsible
    let isTopNotesCollapsed = true;
    const topNotesHeader = document.getElementById('top-notes-header');
    const topNotesList = document.getElementById('top-notes-list');
    if (topNotesHeader && topNotesList) {
      topNotesHeader.addEventListener('click', () => {
        isTopNotesCollapsed = !isTopNotesCollapsed;
        if (isTopNotesCollapsed) {
          topNotesHeader.classList.add('collapsed');
          topNotesList.style.display = 'none';
        } else {
          topNotesHeader.classList.remove('collapsed');
          topNotesList.style.display = 'block';
        }
      });
    }

    function formatCompactNumber(n) {
      if (n == null || isNaN(n) || n <= 0) return '0';
      if (n < 1000) return Math.round(n).toString();
      if (n < 10000) return (n / 1000).toFixed(1) + 'k';
      if (n < 100000) return Math.round(n / 1000) + 'k';
      if (n < 1000000) return Math.round(n / 1000) + 'k';
      if (n < 10000000) return (n / 1000000).toFixed(1) + 'M';
      return Math.round(n / 1000000) + 'M';
    }

    function getScoreColor(score) {
      if (score >= 90) return 'var(--vscode-charts-green, #22c55e)';
      if (score >= 75) return 'var(--vscode-charts-yellow, #eab308)';
      return 'var(--vscode-charts-red, #ef4444)';
    }

    function updateBar(fillId, valId, val) {
      const v = Math.round(val != null ? val : 0);
      const f = document.getElementById(fillId);
      const l = document.getElementById(valId);
      if (f) {
        f.style.width = v + '%';
        f.style.backgroundColor = getScoreColor(v);
      }
      if (l) l.textContent = v + '%';
    }

    function partialUpdateHealth(health) {
      if (!health) return;
      const score = Math.round(health.quality_score != null ? health.quality_score : (health.overall_score || 0));
      const color = getScoreColor(score);

      const circle = document.getElementById('quality-circle');
      const scoreNum = document.getElementById('val-quality-score');
      if (circle) circle.style.borderColor = color;
      if (scoreNum) {
        scoreNum.textContent = score.toString();
        scoreNum.style.color = color;
      }

      const activeFindings = health.active_issues_count != null
        ? health.active_issues_count
        : (health.new_regressions != null
            ? health.new_regressions
            : (health.findings_count != null ? health.findings_count : 0));
      const findingsEl = document.getElementById('dash-findings-count');
      if (findingsEl) {
        findingsEl.textContent = activeFindings === 1 ? '1 finding needs attention' : (activeFindings + ' findings need attention');
        findingsEl.style.color = activeFindings > 0 ? 'var(--vscode-charts-yellow, #eab308)' : 'var(--vscode-charts-green, #22c55e)';
      }

      const baselined = health.legacy_debt != null ? health.legacy_debt : (health.legacy_debt_count || 0);
      const regressions = health.new_regressions != null ? health.new_regressions : (health.regressions_count || 0);
      const debtDetailEl = document.getElementById('dash-debt-counts');
      if (debtDetailEl) {
        debtDetailEl.textContent = regressions + ' regressions · ' + baselined + ' baselined';
      }

      updateBar('fill-integrity', 'val-integrity', health.integrity != null ? health.integrity : health.integrity_score);
      updateBar('fill-structure', 'val-structure', health.structure != null ? health.structure : (health.structure_score != null ? health.structure_score : 100));
      updateBar('fill-consistency', 'val-consistency', health.consistency != null ? health.consistency : health.consistency_score);
      updateBar('fill-discoverability', 'val-discoverability', health.discoverability != null ? health.discoverability : health.discoverability_score);
      updateBar('fill-freshness', 'val-freshness', health.freshness != null ? health.freshness : health.freshness_score);
    }

    function partialUpdateStructure(structure, nodes) {
      if (!structure) return;
      const totalDocs = structure.total_documents != null ? structure.total_documents : (structure.totalNotes || (nodes ? nodes.length : 0));
      const connected = structure.connected_documents != null ? structure.connected_documents : (structure.connectedNotes || 0);
      const isolated = structure.isolated_documents != null ? structure.isolated_documents : (structure.orphanNotes || 0);
      const refs = structure.total_references != null ? structure.total_references : (structure.totalLinks || 0);

      const docsEl = document.getElementById('val-documents');
      if (docsEl) docsEl.textContent = formatCompactNumber(totalDocs);

      const connEl = document.getElementById('val-connected');
      if (connEl) connEl.textContent = formatCompactNumber(connected);

      const isoEl = document.getElementById('val-isolated');
      if (isoEl) isoEl.textContent = formatCompactNumber(isolated);

      const refEl = document.getElementById('val-references');
      if (refEl) refEl.textContent = formatCompactNumber(refs);

      if (nodes && nodes.length > 0) {
        const sorted = [...nodes]
          .map(n => ({
            ...n,
            totalConns: (n.in_degree || 0) + (n.out_degree || 0)
          }))
          .sort((a, b) => b.totalConns - a.totalConns || (b.tokens || 0) - (a.tokens || 0))
          .slice(0, 5);

        const listEl = document.getElementById('top-notes-list');
        if (listEl) {
          listEl.innerHTML = '';
          for (const node of sorted) {
            const li = document.createElement('li');
            li.className = 'note-item';
            li.innerHTML = '<span class="note-name" title="' + node.id + '">' + node.label + '</span>' +
              '<span class="note-meta-badges">' +
                '<span class="note-conns-badge" title="' + (node.in_degree || 0) + ' incoming, ' + (node.out_degree || 0) + ' outgoing">' + formatCompactNumber(node.totalConns) + ' refs</span>' +
                '<span class="note-tokens">' + formatCompactNumber(node.tokens || 0) + ' tok</span>' +
              '</span>';
            li.onclick = () => {
              vscode.postMessage({ type: 'openFile', filePath: node.file_path });
            };
            listEl.appendChild(li);
          }
        }
      }
    }

    function partialUpdateEvolution(evo) {
      if (!evo) return;
      const orientation = evo.orientation || 'ai_oriented';
      const iconEl = document.getElementById('evolution-mode-icon');
      const badgeEl = document.getElementById('evolution-mode-badge');
      const oppCountEl = document.getElementById('val-opportunity-count');
      const oppTextEl = document.getElementById('evolution-opportunity-text');

      let icon = '🤖';
      let title = 'AI-Oriented';

      switch (orientation) {
        case 'ai_oriented':
          icon = '🤖';
          title = 'AI-Oriented';
          break;
        case 'human_oriented':
          icon = '👤';
          title = 'Human-Oriented';
          break;
        case 'balanced':
          icon = '⚖️';
          title = 'Balanced';
          break;
        case 'custom':
          icon = '⚙️';
          title = 'Custom';
          break;
      }

      if (iconEl) iconEl.textContent = icon;
      if (badgeEl) badgeEl.textContent = title;

      const opps = evo.opportunity_count != null ? evo.opportunity_count : 0;
      if (oppCountEl) oppCountEl.textContent = opps.toString();
      if (oppTextEl) {
        oppTextEl.textContent = opps === 1 ? 'structural opportunity detected' : 'structural opportunities detected';
      }
    }

    function partialUpdateSync(sync) {
      if (!sync) return;
      const dotEl = document.getElementById('sync-indicator-dot');
      const textEl = document.getElementById('sync-status-text');
      const shaEl = document.getElementById('sync-commit-text');

      let dotColor = '#10b981';
      let statusText = 'Synced';

      switch (sync.status) {
        case 'synced':
          dotColor = '#10b981';
          statusText = 'Synced';
          break;
        case 'syncing':
          dotColor = '#f59e0b';
          statusText = 'Syncing...';
          break;
        case 'error':
          dotColor = '#ef4444';
          statusText = 'Error';
          break;
        case 'uninitialized':
        default:
          dotColor = '#888888';
          statusText = 'Uninitialized';
          break;
      }

      if (dotEl) dotEl.style.background = dotColor;
      if (textEl) textEl.textContent = statusText;
      if (shaEl) {
        if (sync.last_commit_sha) {
          shaEl.textContent = 'Last commit: #' + sync.last_commit_sha;
        } else {
          shaEl.textContent = 'Last commit: #uninitialized';
        }
      }
    }

    function renderSummary(summary) {
      if (!summary) return;
      partialUpdateHealth(summary.health);
      partialUpdateStructure(summary.structure, null);
      partialUpdateEvolution(summary.evolution);
      partialUpdateSync(summary.sync);
    }

    // Initial hydration
    const initialSummary = ${summaryJson};
    if (initialSummary) {
      renderSummary(initialSummary);
    } else {
      const initData = ${dataJson};
      const initQuality = ${qualityJson};
      const initIssues = ${issuesJson};
      if (initQuality) partialUpdateHealth(initQuality);
      if (initIssues) {
        const activeCount = initIssues.active_issues_count != null
          ? initIssues.active_issues_count
          : (initIssues.new_regressions != null
              ? initIssues.new_regressions
              : (initIssues.issues ? initIssues.issues.filter(i => !i.is_legacy).length : 0));
        const baselined = initIssues.legacy_debt_count != null
          ? initIssues.legacy_debt_count
          : (initIssues.legacy_debt != null ? initIssues.legacy_debt : (initIssues.issues ? initIssues.issues.filter(i => !!i.is_legacy).length : 0));
        const fEl = document.getElementById('dash-findings-count');
        if (fEl) {
          fEl.textContent = activeCount === 1 ? '1 finding needs attention' : (activeCount + ' findings need attention');
          fEl.style.color = activeCount > 0 ? 'var(--vscode-charts-yellow, #eab308)' : 'var(--vscode-charts-green, #22c55e)';
        }
        const dEl = document.getElementById('dash-debt-counts');
        if (dEl) {
          dEl.textContent = activeCount + ' regressions · ' + baselined + ' baselined';
        }
      }
      if (initData && initData.nodes) {
        const obs = initData.observatory_summary;
        const totalDocs = initData.nodes.length;
        const connected = obs ? obs.connectedNotes : initData.nodes.filter(n => n.status === 'active' || n.status === 'connected_inactive').length;
        const isolated = obs ? obs.orphanNotes : initData.nodes.filter(n => n.status === 'orphan').length;
        const refs = initData.edges ? initData.edges.length : 0;
        partialUpdateStructure({
          total_documents: totalDocs,
          connected_documents: connected,
          isolated_documents: isolated,
          total_references: refs,
        }, initData.nodes);

        const hubId = initData.recommended_hub || (initData.nodes[0] ? initData.nodes[0].id : '00_meta/wiki_index.md');
        const hubEl = document.getElementById('dash-hub-link');
        if (hubEl) hubEl.textContent = hubId;
      }
      partialUpdateEvolution({ orientation: '${orientation}', opportunity_count: 0 });
      partialUpdateSync({ status: 'synced', last_commit_sha: null });
    }

    // Message listener
    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.type === 'summaryUpdate') {
        renderSummary(msg.payload);
      } else if (msg.type === 'graphDataUpdate') {
        const data = msg.payload;
        if (data && data.nodes) {
          const obs = data.observatory_summary;
          const totalDocs = data.nodes.length;
          const connected = obs ? obs.connectedNotes : data.nodes.filter(n => n.status === 'active' || n.status === 'connected_inactive').length;
          const isolated = obs ? obs.orphanNotes : data.nodes.filter(n => n.status === 'orphan').length;
          const refs = data.edges ? data.edges.length : 0;
          partialUpdateStructure({
            total_documents: totalDocs,
            connected_documents: connected,
            isolated_documents: isolated,
            total_references: refs,
          }, data.nodes);

          const hubId = data.recommended_hub || (data.nodes[0] ? data.nodes[0].id : '00_meta/wiki_index.md');
          const hubEl = document.getElementById('dash-hub-link');
          if (hubEl) hubEl.textContent = hubId;
        }
        if (msg.qualityScore) {
          partialUpdateHealth(msg.qualityScore);
        }
      } else if (msg.type === 'qualityUpdate') {
        partialUpdateHealth(msg.payload);
      } else if (msg.type === 'issuesUpdate') {
        const report = msg.payload;
        if (report) {
          const activeCount = report.active_issues_count != null
            ? report.active_issues_count
            : (report.new_regressions != null
                ? report.new_regressions
                : (report.issues ? report.issues.filter(i => !i.is_legacy).length : 0));
          const baselined = report.legacy_debt_count != null
            ? report.legacy_debt_count
            : (report.legacy_debt != null ? report.legacy_debt : (report.issues ? report.issues.filter(i => !!i.is_legacy).length : 0));
          const findingsEl = document.getElementById('dash-findings-count');
          if (findingsEl) {
            findingsEl.textContent = activeCount === 1 ? '1 finding needs attention' : (activeCount + ' findings need attention');
            findingsEl.style.color = activeCount > 0 ? 'var(--vscode-charts-yellow, #eab308)' : 'var(--vscode-charts-green, #22c55e)';
          }
          const debtDetailEl = document.getElementById('dash-debt-counts');
          if (debtDetailEl) {
            debtDetailEl.textContent = activeCount + ' regressions · ' + baselined + ' baselined';
          }
        }
      } else if (msg.type === 'orientationUpdate') {
        partialUpdateEvolution({ orientation: msg.payload });
      }
    });
  </script>
</body>
</html>`;
}
