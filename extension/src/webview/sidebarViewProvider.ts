import * as vscode from 'vscode';
import * as path from 'path';
import { generateNonce, buildCspPolicy } from './cspHelper';
import { ScanResultPayload } from '../types/protocol';
import { KnowledgeQualityScore } from '../types/admission';

export class SidebarViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = 'llmWiki.sidebarView';
  private _view?: vscode.WebviewView;
  private currentScanResult: ScanResultPayload | null = null;
  private currentQualityScore: KnowledgeQualityScore | null = null;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly getScanResult: () => ScanResultPayload | null,
    private readonly getQualityScore?: () => KnowledgeQualityScore | null
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
    this.currentQualityScore = this.getQualityScore ? this.getQualityScore() : null;
    this.updateHtml();

    webviewView.webview.onDidReceiveMessage(async (data) => {
      switch (data.type) {
        case 'openGraph':
        case 'reviewSuggestions':
          vscode.commands.executeCommand('llmWiki.showKnowledgeGraph');
          break;
        case 'reviewIssues':
          vscode.commands.executeCommand('llmWiki.reviewIssues');
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
        case 'refreshQuality':
          vscode.commands.executeCommand('llmWiki.refreshQualityScore');
          break;
        case 'openFactorNode':
          if (data.nodeId) {
            this.openDocument(data.nodeId);
          }
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

  public updateData(scanResult: ScanResultPayload, qualityScore?: KnowledgeQualityScore | null): void {
    this.currentScanResult = scanResult;
    if (qualityScore !== undefined) {
      this.currentQualityScore = qualityScore;
    }
    if (this._view) {
      this._view.webview.postMessage({
        type: 'graphDataUpdate',
        payload: scanResult,
        qualityScore: this.currentQualityScore,
      });
    }
  }

  public updateQuality(qualityScore: KnowledgeQualityScore): void {
    this.currentQualityScore = qualityScore;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'qualityUpdate',
        payload: qualityScore,
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
    const initialQuality = JSON.stringify(this.currentQualityScore || null);

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
    .dash-action-link:disabled,
    .dash-action-link.loading {
      opacity: 0.6;
      cursor: wait;
      pointer-events: none;
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
    .quality-pills-row {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 4px;
      background: rgba(255, 255, 255, 0.02);
      border: 1px solid rgba(255, 255, 255, 0.05);
      border-radius: 4px;
      padding: 5px 3px;
      text-align: center;
      margin-bottom: 6px;
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
    .factors-section {
      margin-top: 8px;
      border-top: 1px solid rgba(255, 255, 255, 0.06);
      padding-top: 6px;
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
    .pillar-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 8px;
    }
    .pillar-header .pillar-title {
      margin-bottom: 0;
    }
    .quality-refresh-btn {
      background: transparent;
      border: none;
      color: var(--vscode-descriptionForeground, #888);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 2px;
      border-radius: 3px;
    }
    .quality-refresh-btn:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.1));
      color: var(--vscode-foreground, #fff);
    }
    .quality-gauge-row {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 10px;
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
    .quality-pillars-mini {
      display: flex;
      flex-direction: column;
      gap: 4px;
      flex: 1;
    }
    .pillar-mini-row {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 10px;
    }
    .mini-label {
      width: 68px;
      min-width: 68px;
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
    .debt-accounting-row {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 6px;
      background: rgba(255, 255, 255, 0.02);
      border: 1px solid rgba(255, 255, 255, 0.05);
      border-radius: 4px;
      padding: 6px 4px;
      text-align: center;
      margin-bottom: 6px;
    }
    .debt-stat {
      display: flex;
      flex-direction: column;
      align-items: center;
    }
    .debt-value {
      font-size: 13px;
      font-weight: 700;
      color: var(--vscode-editor-foreground, #fff);
    }
    .debt-label {
      font-size: 9px;
      color: var(--vscode-descriptionForeground, #888);
      margin-top: 1px;
    }
    .quality-factors-list {
      margin-top: 8px;
      border-top: 1px solid rgba(255, 255, 255, 0.06);
      padding-top: 6px;
    }
    .factors-title {
      font-size: 9.5px;
      font-weight: 600;
      color: var(--vscode-descriptionForeground, #888);
      margin-bottom: 4px;
      text-transform: uppercase;
      letter-spacing: 0.4px;
    }
    .quality-factor-item {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 6px;
      font-size: 10.5px;
      padding: 3px 4px;
      border-radius: 3px;
      cursor: pointer;
    }
    .quality-factor-item:hover {
      background: var(--vscode-list-hoverBackground, rgba(255, 255, 255, 0.05));
    }
    .factor-badge {
      font-size: 9px;
      font-weight: 700;
      padding: 1px 4px;
      border-radius: 2px;
      background: rgba(239, 68, 68, 0.15);
      color: #ef4444;
      border: 1px solid rgba(239, 68, 68, 0.3);
    }
    .factor-node {
      flex: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      color: var(--vscode-textLink-foreground, #38bdf8);
    }
    .factor-node:hover {
      text-decoration: underline;
    }
    .factor-weight {
      font-size: 9px;
      color: var(--vscode-descriptionForeground, #888);
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

    <!-- Pillar 1: Quality (At the top) -->
    <div class="dashboard-pillar pillar-quality" id="pillar-quality">
      <div class="pillar-header">
        <div class="pillar-title">QUALITY</div>
        <button class="quality-refresh-btn" id="dash-btn-refresh-quality" title="Recalculate Knowledge Quality Score">
          <svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <path d="M1.5 8a6.5 6.5 0 1 0 1.9-4.6L1.5 5.5"/>
            <polyline points="1.5 1.5 1.5 5.5 5.5 5.5"/>
          </svg>
        </button>
      </div>

      <div class="quality-gauge-row">
        <div class="quality-score-circle" id="quality-circle" style="border-color: var(--vscode-charts-green, #22c55e);">
          <span class="score-number" id="val-quality-score">--</span>
          <span class="score-max">/100</span>
        </div>
        <div class="quality-pillars-mini">
          <div class="pillar-mini-row" title="Integrity (25%) - Broken links and target anchors">
            <span class="mini-label">Integrity</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" id="fill-integrity" style="width: 0%; background-color: var(--vscode-charts-green, #22c55e);"></div>
            </div>
            <span class="mini-val" id="val-integrity">--%</span>
          </div>
          <div class="pillar-mini-row" title="Structure (20%) - Missing fragment metadata & oversized documents">
            <span class="mini-label">Structure</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" id="fill-structure" style="width: 0%; background-color: var(--vscode-charts-green, #22c55e);"></div>
            </div>
            <span class="mini-val" id="val-structure">--%</span>
          </div>
          <div class="pillar-mini-row" title="Consistency (20%) - Semantic collisions & duplicates">
            <span class="mini-label">Consistency</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" id="fill-consistency" style="width: 0%; background-color: var(--vscode-charts-green, #22c55e);"></div>
            </div>
            <span class="mini-val" id="val-consistency">--%</span>
          </div>
          <div class="pillar-mini-row" title="Discoverability (20%) - Orphans & hop reachability">
            <span class="mini-label">Discoverability</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" id="fill-discoverability" style="width: 0%; background-color: var(--vscode-charts-green, #22c55e);"></div>
            </div>
            <span class="mini-val" id="val-discoverability">--%</span>
          </div>
          <div class="pillar-mini-row" title="Freshness (15%) - Technical drift & stale code citations">
            <span class="mini-label">Freshness</span>
            <div class="mini-bar-track">
              <div class="mini-bar-fill" id="fill-freshness" style="width: 0%; background-color: var(--vscode-charts-green, #22c55e);"></div>
            </div>
            <span class="mini-val" id="val-freshness">--%</span>
          </div>
        </div>
      </div>

      <!-- Knowledge Stats Row (Compact Pills) -->
      <div class="quality-pills-row" title="Knowledge Base Network Metrics">
        <div class="pill-stat" title="Connected Notes">
          <span class="pill-value" id="val-connected">0</span>
          <span class="pill-label">Connected</span>
        </div>
        <div class="pill-stat" title="Orphan Notes">
          <span class="pill-value" id="val-orphans">0</span>
          <span class="pill-label">Orphans</span>
        </div>
        <div class="pill-stat" title="Excluded Notes">
          <span class="pill-value" id="val-excluded">0</span>
          <span class="pill-label">Excluded</span>
        </div>
        <div class="pill-stat" title="Total Links">
          <span class="pill-value" id="val-links">0</span>
          <span class="pill-label">Links</span>
        </div>
      </div>

      <!-- Debt & Broken Stats Row (Compact Pills) -->
      <div class="quality-pills-row" title="Knowledge Debt & Link Integrity">
        <div class="pill-stat" id="box-broken" title="Broken References">
          <span class="pill-value" id="val-broken-count" style="color: var(--vscode-charts-red, #ef4444);">0</span>
          <span class="pill-label">Broken</span>
        </div>
        <div class="pill-stat" title="Baseline accepted brownfield debt">
          <span class="pill-value" id="val-debt-legacy">0</span>
          <span class="pill-label">Legacy</span>
        </div>
        <div class="pill-stat" id="box-debt-regr" title="New regressions exceeding baseline">
          <span class="pill-value" id="val-debt-regr">0</span>
          <span class="pill-label">New Regr.</span>
        </div>
        <div class="pill-stat" title="Resolved legacy defects">
          <span class="pill-value" id="val-debt-resolved" style="color: var(--vscode-charts-green, #22c55e);">0</span>
          <span class="pill-label">Resolved</span>
        </div>
      </div>

      <!-- Top Degradation Factors (Collapsible, closed by default) -->
      <div class="factors-section" id="factors-section" style="display: none;">
        <div class="collapsible-header collapsed" id="factors-header" title="Toggle top degradation factors">
          <span class="factors-title" style="margin: 0; cursor: pointer;">Top Degradation Factors</span>
          <span class="collapsible-chevron" id="factors-chevron">
            <svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="4 6 8 10 12 6"></polyline>
            </svg>
          </span>
        </div>
        <div class="quality-factors-list" id="quality-factors-container" style="display: none;">
          <div id="quality-factors-items"></div>
        </div>
      </div>
    </div>

    <!-- Pillar 2: Central Hub & Health -->
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


    <div class="section-title collapsible-header collapsed" id="top-notes-header" title="Toggle collapse">
      <span>Top Connected Notes</span>
      <span class="collapsible-chevron" id="top-notes-chevron">
        <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="4 6 8 10 12 6"></polyline>
        </svg>
      </span>
    </div>
    <ul id="top-notes-list" class="top-notes" style="display: none;">
      <li style="color: var(--vscode-descriptionForeground); font-size: 11px; padding: 4px;">Loading top hubs...</li>
    </ul>
  </div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();

    const btnReviewBroken = document.getElementById('dash-btn-review-broken');
    btnReviewBroken?.addEventListener('click', () => {
      btnReviewBroken.classList.add('loading');
      btnReviewBroken.textContent = 'Opening...';
      vscode.postMessage({ type: 'reviewIssues' });
      setTimeout(() => {
        btnReviewBroken.classList.remove('loading');
        btnReviewBroken.textContent = 'Review issues';
      }, 2000);
    });

    const btnReviewSuggestions = document.getElementById('dash-btn-review-suggestions');
    btnReviewSuggestions?.addEventListener('click', () => {
      btnReviewSuggestions.classList.add('loading');
      btnReviewSuggestions.textContent = 'Opening...';
      vscode.postMessage({ type: 'reviewSuggestions' });
      setTimeout(() => {
        btnReviewSuggestions.classList.remove('loading');
        btnReviewSuggestions.textContent = 'Review suggestions';
      }, 2000);
    });
    document.getElementById('dash-btn-change-hub')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'changeHub' });
    });
    document.getElementById('dash-btn-refresh-quality')?.addEventListener('click', () => {
      vscode.postMessage({ type: 'refreshQuality' });
    });

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

    let isFactorsCollapsed = true;
    const factorsHeader = document.getElementById('factors-header');
    const factorsContainer = document.getElementById('quality-factors-container');
    if (factorsHeader && factorsContainer) {
      factorsHeader.addEventListener('click', () => {
        isFactorsCollapsed = !isFactorsCollapsed;
        if (isFactorsCollapsed) {
          factorsHeader.classList.add('collapsed');
          factorsContainer.style.display = 'none';
        } else {
          factorsHeader.classList.remove('collapsed');
          factorsContainer.style.display = 'block';
        }
      });
    }

    let currentData = ${initialData};
    let currentQuality = ${initialQuality};
    renderData(currentData);
    if (currentQuality) {
      renderQuality(currentQuality);
    }

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.type === 'graphDataUpdate') {
        currentData = msg.payload;
        renderData(currentData);
        if (msg.qualityScore) {
          currentQuality = msg.qualityScore;
          renderQuality(currentQuality);
        }
      } else if (msg.type === 'qualityUpdate') {
        currentQuality = msg.payload;
        renderQuality(currentQuality);
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

      const brokenPill = document.getElementById('val-broken-count');
      if (brokenPill) brokenPill.textContent = formatCompactNumber(brokenCount);

      const brokenAlert = document.getElementById('dash-broken-alert');
      const brokenText = document.getElementById('dash-broken-text');
      if (brokenCount > 0) {
        brokenAlert.style.display = 'flex';
        brokenText.textContent = formatCompactNumber(brokenCount) + (brokenCount === 1 ? ' broken link' : ' broken links');
      } else {
        brokenAlert.style.display = 'none';
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

    function getScoreColor(score) {
      if (score >= 90) return 'var(--vscode-charts-green, #22c55e)';
      if (score >= 75) return 'var(--vscode-charts-yellow, #eab308)';
      return 'var(--vscode-charts-red, #ef4444)';
    }

    function renderQuality(q) {
      if (!q) return;

      const overall = Math.round(q.overall_score || 0);
      const color = getScoreColor(overall);

      const circle = document.getElementById('quality-circle');
      const scoreNum = document.getElementById('val-quality-score');
      if (circle) circle.style.borderColor = color;
      if (scoreNum) {
        scoreNum.textContent = overall.toString();
        scoreNum.style.color = color;
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

      updateBar('fill-integrity', 'val-integrity', q.integrity_score);
      updateBar('fill-structure', 'val-structure', q.structure_score != null ? q.structure_score : 100);
      updateBar('fill-consistency', 'val-consistency', q.consistency_score);
      updateBar('fill-discoverability', 'val-discoverability', q.discoverability_score);
      updateBar('fill-freshness', 'val-freshness', q.freshness_score);

      const legEl = document.getElementById('val-debt-legacy');
      const regrEl = document.getElementById('val-debt-regr');
      const resEl = document.getElementById('val-debt-resolved');

      if (legEl) legEl.textContent = formatCompactNumber(q.legacy_debt_count || 0);

      const regrCount = q.regressions_count != null ? q.regressions_count : (q.new_regressions_count || 0);
      if (regrEl) {
        regrEl.textContent = formatCompactNumber(regrCount);
        regrEl.style.color = regrCount > 0 ? 'var(--vscode-charts-red, #ef4444)' : 'inherit';
      }
      if (resEl) resEl.textContent = formatCompactNumber(q.resolved_debt_count || 0);

      const factorsSection = document.getElementById('factors-section');
      const factorsContainer = document.getElementById('quality-factors-container');
      const factorsItems = document.getElementById('quality-factors-items');
      const factors = q.top_degradation_factors || [];

      if (factorsSection && factorsContainer && factorsItems) {
        if (factors.length > 0) {
          factorsSection.style.display = 'block';
          factorsContainer.style.display = isFactorsCollapsed ? 'none' : 'block';
          factorsItems.innerHTML = '';
          for (const f of factors.slice(0, 5)) {
            const item = document.createElement('div');
            item.className = 'quality-factor-item';
            item.title = (f.affected_node || '') + ': ' + (f.description || '');
            item.innerHTML =
              '<span class="factor-badge">-' + (f.deduction_points || 0).toFixed(1) + '</span>' +
              '<span class="factor-node" title="' + f.affected_node + '">' + f.affected_node + '</span>' +
              '<span class="factor-weight">' + (f.betweenness_weight || 1.0).toFixed(1) + 'x</span>';
            item.onclick = () => {
              vscode.postMessage({ type: 'openFactorNode', nodeId: f.affected_node });
            };
            factorsItems.appendChild(item);
          }
        } else {
          factorsSection.style.display = 'none';
        }
      }
    }

  </script>
</body>
</html>`;
  }
}
