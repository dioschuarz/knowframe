import * as vscode from 'vscode';
import * as path from 'path';
import { generateNonce, buildCspPolicy } from './cspHelper';
import { ScanResultPayload } from '../types/protocol';

export class SidebarViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = 'llmWiki.sidebarView';
  private _view?: vscode.WebviewView;
  private currentScanResult: ScanResultPayload | null = null;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly getScanResult: () => ScanResultPayload | null
  ) {}

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken
  ): void {
    this._view = webviewView;
    webviewView.title = '';

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.extensionUri],
    };

    this.currentScanResult = this.getScanResult();
    this.updateHtml();

    webviewView.webview.onDidReceiveMessage(async (data) => {
      switch (data.type) {
        case 'openGraph':
        case 'reviewIssues':
        case 'reviewSuggestions':
          vscode.commands.executeCommand('llmWiki.showKnowledgeGraph');
          break;
        case 'changeHub':
        case 'setHub':
          vscode.commands.executeCommand('llmWiki.setHub');
          break;
        case 'rescanVault':
          vscode.commands.executeCommand('llmWiki.rescanVault');
          break;
        case 'fixBrokenLinks':
          vscode.commands.executeCommand('llmWiki.fixBrokenLinks');
          break;
        case 'installHooks':
          vscode.commands.executeCommand('llmWiki.installGitHooks');
          break;
        case 'openFile':
          if (data.filePath) {
            this.openDocument(data.filePath);
          }
          break;
        case 'openHub':
          if (data.hubPath) {
            this.openDocument(data.hubPath);
          }
          break;
      }
    });

    if (this.currentScanResult) {
      this.updateData(this.currentScanResult);
    }
  }

  public updateData(scanResult: ScanResultPayload): void {
    this.currentScanResult = scanResult;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'graphDataUpdate',
        payload: scanResult,
      });
    }
  }

  private async openDocument(filePath: string): Promise<void> {
    try {
      let uri: vscode.Uri;
      if (path.isAbsolute(filePath)) {
        uri = vscode.Uri.file(filePath);
      } else {
        const root = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!root) {return;}
        uri = vscode.Uri.joinPath(root, filePath);
      }
      const doc = await vscode.workspace.openTextDocument(uri);
      await vscode.window.showTextDocument(doc, { preview: true });
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to open note: ${err.message}`);
    }
  }

  private updateHtml(): void {
    if (!this._view) {return;}
    const webview = this._view.webview;
    const nonce = generateNonce();
    const cspPolicy = buildCspPolicy(nonce, webview.cspSource);

    const initialData = JSON.stringify(this.currentScanResult || {});

    this._view.webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="${cspPolicy}">
  <title>LLM Wiki - Manager</title>
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
      gap: 10px;
    }
    .dashboard-attention-card {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 8px 10px;
      border-radius: 6px;
      font-size: 12px;
    }
    .broken-links-alert {
      background: rgba(239, 68, 68, 0.12);
      border: 1px solid rgba(239, 68, 68, 0.4);
    }
    .suggestions-alert {
      background: rgba(59, 130, 246, 0.12);
      border: 1px solid rgba(59, 130, 246, 0.4);
    }
    .attention-header {
      display: flex;
      align-items: center;
      gap: 6px;
    }
    .attention-icon {
      font-size: 14px;
    }
    .broken-links-alert .attention-title {
      color: #ef4444;
      font-weight: 600;
    }
    .suggestions-alert .attention-title {
      color: #3b82f6;
      font-weight: 600;
    }
    .dash-action-link {
      background: transparent;
      border: 1px solid var(--vscode-widget-border, #333);
      color: var(--vscode-textLink-foreground, #38bdf8);
      padding: 3px 8px;
      border-radius: 4px;
      font-size: 11px;
      cursor: pointer;
      white-space: nowrap;
    }
    .dash-action-link:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.1));
      text-decoration: underline;
    }
    .dashboard-pillar {
      background: var(--vscode-editor-background, #1e1e1e);
      border: 1px solid var(--vscode-widget-border, #333333);
      border-radius: 6px;
      padding: 10px;
    }
    .pillar-title {
      font-size: 10px;
      font-weight: 700;
      letter-spacing: 0.6px;
      color: var(--vscode-descriptionForeground, #888);
      margin-bottom: 8px;
      text-transform: uppercase;
    }
    .metrics-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 6px;
    }
    .metric-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      background: rgba(255, 255, 255, 0.03);
      border: 1px solid rgba(255, 255, 255, 0.05);
      border-radius: 4px;
      padding: 6px 4px;
      text-align: center;
    }
    .metric-value {
      font-size: 15px;
      font-weight: 700;
      color: var(--vscode-editor-foreground, #fff);
    }
    .metric-label {
      font-size: 9.5px;
      color: var(--vscode-descriptionForeground, #888);
      margin-top: 2px;
    }
    .token-metrics {
      display: flex;
      flex-direction: column;
      gap: 6px;
    }
    .token-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 11.5px;
    }
    .token-row-label {
      color: var(--vscode-descriptionForeground, #aaa);
    }
    .token-row-value {
      font-weight: 600;
      color: var(--vscode-editor-foreground, #fff);
    }
    .reachable-row {
      font-size: 11px;
      opacity: 0.85;
    }
    .token-progress-track {
      height: 4px;
      background: rgba(255, 255, 255, 0.1);
      border-radius: 2px;
      overflow: hidden;
      margin: 2px 0;
    }
    .token-progress-fill {
      height: 100%;
      border-radius: 2px;
      transition: width 0.3s ease;
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
      max-width: 70%;
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
    .sync-status-row {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 11px;
      color: var(--vscode-descriptionForeground, #888);
    }
    .sync-indicator-dot {
      width: 6px;
      height: 6px;
      border-radius: 50%;
      background: #10b981;
    }
    .section-title {
      font-size: 10px;
      font-weight: 700;
      letter-spacing: 0.6px;
      color: var(--vscode-descriptionForeground, #888);
      margin: 12px 0 6px 0;
      text-transform: uppercase;
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
      padding: 6px 8px;
      border-radius: 4px;
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 11.5px;
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
      gap: 6px;
      flex-shrink: 0;
    }
    .note-conns-badge {
      font-size: 10px;
      font-weight: 600;
      padding: 1px 6px;
      border-radius: 3px;
      background: rgba(56, 189, 248, 0.12);
      color: #38bdf8;
      border: 1px solid rgba(56, 189, 248, 0.25);
      width: 62px;
      min-width: 62px;
      text-align: right;
      display: inline-block;
      box-sizing: border-box;
      font-variant-numeric: tabular-nums;
    }
    .note-tokens {
      font-size: 10.5px;
      color: var(--vscode-descriptionForeground);
      white-space: nowrap;
      width: 52px;
      min-width: 52px;
      text-align: right;
      display: inline-block;
      font-variant-numeric: tabular-nums;
    }
  </style>
</head>
<body>
  <div class="status-dashboard">
    <!-- Attention Cards (Conditionally shown) -->
    <div class="dashboard-attention-card broken-links-alert" id="dash-broken-alert" style="display: none;">
      <div class="attention-header">
        <span class="attention-icon">
          <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <path d="M8 1.5L14.5 13.5H1.5L8 1.5Z"/>
            <line x1="8" y1="6" x2="8" y2="9.5"/>
            <circle cx="8" cy="11.5" r="0.75" fill="currentColor"/>
          </svg>
        </span>
        <span class="attention-title" id="dash-broken-text">0 broken links</span>
      </div>
      <button class="dash-action-link" id="dash-btn-review-broken">Review issues</button>
    </div>

    <div class="dashboard-attention-card suggestions-alert" id="dash-suggestions-alert" style="display: none;">
      <div class="attention-header">
        <span class="attention-icon">
          <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M6 13h4M6.5 15h3M4.5 6.5a3.5 3.5 0 1 1 7 0c0 1.5-1 2.5-1.5 3.5h-4C5.5 9 4.5 8 4.5 6.5Z"/>
          </svg>
        </span>
        <span class="attention-title" id="dash-suggestions-text">Noise detected</span>
      </div>
      <button class="dash-action-link" id="dash-btn-review-suggestions">Review suggestions</button>
    </div>

    <!-- Pillar 1: Knowledge Base -->
    <div class="dashboard-pillar pillar-knowledge">
      <div class="pillar-title">KNOWLEDGE</div>
      <div class="metrics-grid">
        <div class="metric-item">
          <span class="metric-value" id="val-connected">0</span>
          <span class="metric-label">Connected</span>
        </div>
        <div class="metric-item">
          <span class="metric-value" id="val-orphans">0</span>
          <span class="metric-label">Orphans</span>
        </div>
        <div class="metric-item">
          <span class="metric-value" id="val-excluded">0</span>
          <span class="metric-label">Excluded</span>
        </div>
        <div class="metric-item">
          <span class="metric-value" id="val-links">0</span>
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
          <span class="token-row-value" id="val-l0-tokens">0 tok</span>
        </div>
        <div class="token-progress-track">
          <div class="token-progress-fill" id="val-token-fill" style="width: 0%; background-color: var(--vscode-charts-blue, #3b82f6);"></div>
        </div>
        <div class="token-row reachable-row">
          <span class="token-row-label">Reachable (L1-L2)</span>
          <span class="token-row-value" id="val-reachable-tokens">0 tok</span>
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
          <span class="hub-path-link" id="dash-hub-link" title="Scanning vault...">Scanning vault...</span>
        </div>
        <button class="dash-action-link" id="dash-btn-change-hub">Change</button>
      </div>
      <div class="sync-status-row">
        <span class="sync-indicator-dot"></span>
        <span class="sync-status-text">Synced</span>
      </div>
    </div>

    <div class="section-title collapsible-header" id="top-notes-header" title="Toggle collapse">
      <span>Top Connected Notes</span>
      <span class="collapsible-chevron" id="top-notes-chevron">
        <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="4 6 8 10 12 6"></polyline>
        </svg>
      </span>
    </div>
    <ul id="top-notes-list" class="top-notes">
      <li style="color: var(--vscode-descriptionForeground); font-size: 11px; padding: 4px;">Loading top hubs...</li>
    </ul>
  </div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();

    document.getElementById('dash-btn-review-broken')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'reviewIssues' });
    });
    document.getElementById('dash-btn-review-suggestions')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'reviewSuggestions' });
    });
    document.getElementById('dash-btn-change-hub')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'changeHub' });
    });

    let isTopNotesCollapsed = false;
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

    let currentData = ${initialData};
    renderData(currentData);

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.type === 'graphDataUpdate') {
        currentData = msg.payload;
        renderData(currentData);
      }
    });

    function formatCompactNumber(n) {
      if (n == null || isNaN(n) || n <= 0) return '0';
      if (n < 1000) return Math.round(n).toString();
      if (n < 10000) {
        const val = (n / 1000).toFixed(2);
        if (parseFloat(val) >= 10) return (n / 1000).toFixed(1) + 'k';
        return val + 'k';
      }
      if (n < 100000) {
        const val = (n / 1000).toFixed(1);
        if (parseFloat(val) >= 100) return '0.10M';
        return val + 'k';
      }
      if (n < 1000000) {
        const val = (n / 1000000).toFixed(2);
        if (parseFloat(val) >= 1) return '1.00M';
        return val + 'M';
      }
      if (n < 10000000) {
        const val = (n / 1000000).toFixed(2);
        if (parseFloat(val) >= 10) return (n / 1000000).toFixed(1) + 'M';
        return val + 'M';
      }
      if (n < 100000000) {
        const val = (n / 1000000).toFixed(1);
        if (parseFloat(val) >= 100) return '100M';
        return val + 'M';
      }
      if (n < 1000000000) {
        return Math.round(n / 1000000) + 'M';
      }
      return (n / 1000000000).toFixed(1) + 'B';
    }

    function renderData(data) {
      if (!data || !data.nodes) return;

      const nodes = data.nodes || [];
      const edges = data.edges || [];
      const summary = data.token_summary || {};
      const obsSummary = data.observatory_summary;

      const totalLinks = edges.length;
      let connectedNotes = 0;
      let orphanNotes = 0;
      let excludedNotes = 0;
      let brokenCount = 0;
      let activeTokens = 0;
      let reachableTokens = 0;

      if (obsSummary) {
        connectedNotes = obsSummary.connectedNotes || 0;
        orphanNotes = obsSummary.orphanNotes || 0;
        excludedNotes = obsSummary.excludedNotes || 0;
        brokenCount = obsSummary.brokenLinksCount || 0;
        activeTokens = obsSummary.activeTokens || 0;
        reachableTokens = obsSummary.reachableTokens || 0;
      } else {
        connectedNotes = nodes.filter(n => n.status === 'active' || n.status === 'connected_inactive').length;
        orphanNotes = nodes.filter(n => n.status === 'orphan').length;
        excludedNotes = nodes.filter(n => n.status === 'excluded').length;
        brokenCount = edges.filter(e => e.is_broken).length;
        activeTokens = summary.active_harness_tokens || 0;
        reachableTokens = summary.subcall_accessible_tokens || 0;
      }

      document.getElementById('val-connected').textContent = formatCompactNumber(connectedNotes);
      document.getElementById('val-orphans').textContent = formatCompactNumber(orphanNotes);
      document.getElementById('val-excluded').textContent = formatCompactNumber(excludedNotes);
      document.getElementById('val-links').textContent = formatCompactNumber(totalLinks);

      const brokenAlert = document.getElementById('dash-broken-alert');
      const brokenText = document.getElementById('dash-broken-text');
      if (brokenCount > 0) {
        brokenAlert.style.display = 'flex';
        brokenText.textContent = formatCompactNumber(brokenCount) + (brokenCount === 1 ? ' broken link' : ' broken links');
      } else {
        brokenAlert.style.display = 'none';
      }

      document.getElementById('val-l0-tokens').textContent = formatCompactNumber(activeTokens) + ' tok';
      document.getElementById('val-reachable-tokens').textContent = formatCompactNumber(reachableTokens) + ' tok';

      const critThreshold = summary.critical_threshold || 100000;
      const budgetPct = Math.min(100, Math.round((activeTokens / critThreshold) * 100));
      const fill = document.getElementById('val-token-fill');
      fill.style.width = budgetPct + '%';
      if (budgetPct >= 90) {
        fill.style.backgroundColor = 'var(--vscode-charts-red, #ef4444)';
      } else if (budgetPct >= 60) {
        fill.style.backgroundColor = 'var(--vscode-charts-yellow, #f59e0b)';
      } else {
        fill.style.backgroundColor = 'var(--vscode-charts-blue, #3b82f6)';
      }

      const hubEl = document.getElementById('dash-hub-link');
      const hubId = data.recommended_hub || (nodes[0] ? nodes[0].id : 'No Hub Selected');
      hubEl.textContent = hubId;
      hubEl.title = hubId;
      const hubNode = nodes.find(n => n.id === hubId);
      if (hubNode) {
        hubEl.onclick = () => {
          vscode.postMessage({ type: 'openHub', hubPath: hubNode.file_path });
        };
      }

      // Sort nodes strictly from highest number of connections to lower, presenting also number of tokens
      const sorted = [...nodes]
        .map(n => ({
          ...n,
          totalConns: (n.in_degree || 0) + (n.out_degree || 0)
        }))
        .sort((a, b) => b.totalConns - a.totalConns || (b.tokens || 0) - (a.tokens || 0))
        .slice(0, 5);

      const listEl = document.getElementById('top-notes-list');
      listEl.innerHTML = '';
      for (const node of sorted) {
        const li = document.createElement('li');
        li.className = 'note-item';
        li.innerHTML = '<span class="note-name" title="' + node.id + '">' + node.label + '</span>' +
          '<span class="note-meta-badges">' +
            '<span class="note-conns-badge" title="' + (node.in_degree || 0) + ' incoming, ' + (node.out_degree || 0) + ' outgoing">' + formatCompactNumber(node.totalConns) + ' links</span>' +
            '<span class="note-tokens">' + formatCompactNumber(node.tokens || 0) + ' tok</span>' +
          '</span>';
        li.onclick = () => {
          vscode.postMessage({ type: 'openFile', filePath: node.file_path });
        };
        listEl.appendChild(li);
      }
    }
  </script>
</body>
</html>`;
  }
}
