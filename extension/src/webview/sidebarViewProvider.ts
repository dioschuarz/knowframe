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

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.extensionUri],
    };

    this.currentScanResult = this.getScanResult();
    this.updateHtml();

    webviewView.webview.onDidReceiveMessage(async (data) => {
      switch (data.type) {
        case 'openGraph':
          vscode.commands.executeCommand('llmWiki.showKnowledgeGraph');
          break;
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
        if (!root) return;
        uri = vscode.Uri.joinPath(root, filePath);
      }
      const doc = await vscode.workspace.openTextDocument(uri);
      await vscode.window.showTextDocument(doc, { preview: true });
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to open note: ${err.message}`);
    }
  }

  private updateHtml(): void {
    if (!this._view) return;
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
  <title>LLM Wiki Sidebar</title>
  <style>
    body {
      font-family: var(--vscode-font-family);
      font-size: var(--vscode-font-size);
      color: var(--vscode-foreground);
      background-color: var(--vscode-sideBar-background);
      padding: 12px;
      margin: 0;
      box-sizing: border-box;
    }
    .btn-primary {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      padding: 8px 12px;
      background: var(--vscode-button-background);
      color: var(--vscode-button-foreground);
      border: none;
      border-radius: 4px;
      font-weight: 600;
      cursor: pointer;
      margin-bottom: 14px;
      font-size: 13px;
    }
    .btn-primary:hover {
      background: var(--vscode-button-hoverBackground);
    }
    .card {
      background: var(--vscode-editor-background);
      border: 1px solid var(--vscode-widget-border, #333333);
      border-radius: 6px;
      padding: 10px;
      margin-bottom: 12px;
    }
    .card-header {
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      color: var(--vscode-descriptionForeground);
      margin-bottom: 6px;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .status-badge {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      font-size: 11px;
      padding: 2px 6px;
      border-radius: 10px;
      background: rgba(16, 185, 129, 0.15);
      color: #10b981;
    }
    .status-dot {
      width: 6px;
      height: 6px;
      border-radius: 50%;
      background: #10b981;
    }
    .hub-title {
      font-size: 13px;
      font-weight: 600;
      color: var(--vscode-textLink-foreground);
      cursor: pointer;
      word-break: break-all;
    }
    .hub-title:hover {
      text-decoration: underline;
    }
    .stats-grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 8px;
      margin-bottom: 12px;
    }
    .stat-box {
      background: var(--vscode-editor-background);
      border: 1px solid var(--vscode-widget-border, #333333);
      border-radius: 4px;
      padding: 8px;
      text-align: center;
    }
    .stat-val {
      font-size: 16px;
      font-weight: bold;
      color: var(--vscode-editor-foreground);
    }
    .stat-lbl {
      font-size: 10px;
      color: var(--vscode-descriptionForeground);
      margin-top: 2px;
    }
    .token-bar {
      height: 4px;
      background: var(--vscode-widget-border, #333333);
      border-radius: 2px;
      margin-top: 6px;
      overflow: hidden;
    }
    .token-fill {
      height: 100%;
      background: #3b82f6;
      width: 10%;
      transition: width 0.3s ease;
    }
    .section-title {
      font-size: 11px;
      text-transform: uppercase;
      font-weight: 600;
      color: var(--vscode-descriptionForeground);
      margin: 12px 0 6px 0;
    }
    .action-links {
      display: flex;
      flex-direction: column;
      gap: 6px;
      margin-bottom: 12px;
    }
    .action-btn {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 10px;
      background: var(--vscode-editor-background);
      border: 1px solid var(--vscode-widget-border, #333333);
      color: var(--vscode-foreground);
      border-radius: 4px;
      font-size: 12px;
      cursor: pointer;
      text-align: left;
    }
    .action-btn:hover {
      background: var(--vscode-list-hoverBackground);
      color: var(--vscode-list-hoverForeground);
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
      font-size: 12px;
    }
    .note-item:hover {
      background: var(--vscode-list-hoverBackground);
    }
    .note-name {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 160px;
      color: var(--vscode-editor-foreground);
    }
    .note-tokens {
      font-size: 10px;
      color: var(--vscode-descriptionForeground);
    }
    .broken-badge {
      background: rgba(239, 68, 68, 0.2);
      color: #ef4444;
      padding: 1px 5px;
      border-radius: 8px;
      font-size: 10px;
      font-weight: bold;
    }
  </style>
</head>
<body>
  <button id="btn-open-graph" class="btn-primary">
    <span>📊 Open Knowledge Graph</span>
  </button>

  <div class="card">
    <div class="card-header">
      <span>Central Knowledge Hub</span>
      <span class="status-badge"><span class="status-dot"></span> Online</span>
    </div>
    <div id="hub-name" class="hub-title">Scanning vault...</div>
  </div>

  <div class="stats-grid">
    <div class="stat-box">
      <div id="stat-notes" class="stat-val">-</div>
      <div class="stat-lbl">Notes</div>
    </div>
    <div class="stat-box">
      <div id="stat-links" class="stat-val">-</div>
      <div class="stat-lbl">Links</div>
    </div>
    <div class="stat-box">
      <div id="stat-tokens" class="stat-val">-</div>
      <div class="stat-lbl">Prompt Tokens</div>
      <div class="token-bar"><div id="token-fill" class="token-fill"></div></div>
    </div>
    <div class="stat-box">
      <div id="stat-broken" class="stat-val">-</div>
      <div class="stat-lbl">Broken Links</div>
    </div>
  </div>

  <div class="section-title">Quick Actions</div>
  <div class="action-links">
    <button class="action-btn" id="act-set-hub">🎯 Set Central Hub</button>
    <button class="action-btn" id="act-rescan">🔄 Rescan Vault</button>
    <button class="action-btn" id="act-fix-links">🛠️ Fix Broken Links</button>
    <button class="action-btn" id="act-hooks">⚙️ Install Git Hooks</button>
  </div>

  <div class="section-title">Top Connected Notes</div>
  <ul id="top-notes-list" class="top-notes">
    <li style="color: var(--vscode-descriptionForeground); font-size: 11px; padding: 4px;">Loading top hubs...</li>
  </ul>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();

    document.getElementById('btn-open-graph').addEventListener('click', () => {
      vscode.postMessage({ type: 'openGraph' });
    });
    document.getElementById('act-set-hub').addEventListener('click', () => {
      vscode.postMessage({ type: 'setHub' });
    });
    document.getElementById('act-rescan').addEventListener('click', () => {
      vscode.postMessage({ type: 'rescanVault' });
    });
    document.getElementById('act-fix-links').addEventListener('click', () => {
      vscode.postMessage({ type: 'fixBrokenLinks' });
    });
    document.getElementById('act-hooks').addEventListener('click', () => {
      vscode.postMessage({ type: 'installHooks' });
    });

    let currentData = ${initialData};
    renderData(currentData);

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.type === 'graphDataUpdate') {
        currentData = msg.payload;
        renderData(currentData);
      }
    });

    function renderData(data) {
      if (!data || !data.nodes) return;

      const nodes = data.nodes || [];
      const edges = data.edges || [];
      const summary = data.token_summary || {};

      document.getElementById('stat-notes').textContent = nodes.length.toLocaleString();
      document.getElementById('stat-links').textContent = edges.length.toLocaleString();

      const brokenCount = edges.filter(e => e.is_broken).length;
      const brokenEl = document.getElementById('stat-broken');
      brokenEl.textContent = brokenCount;
      if (brokenCount > 0) {
        brokenEl.innerHTML = '<span class="broken-badge">' + brokenCount + '</span>';
      }

      const activeTokens = summary.active_harness_tokens || 0;
      document.getElementById('stat-tokens').textContent = activeTokens.toLocaleString();
      const pct = Math.min(100, Math.round((activeTokens / (summary.critical_threshold || 100000)) * 100));
      const fill = document.getElementById('token-fill');
      fill.style.width = pct + '%';
      if (pct > 80) fill.style.background = '#ef4444';
      else if (pct > 50) fill.style.background = '#f59e0b';
      else fill.style.background = '#3b82f6';

      const hubEl = document.getElementById('hub-name');
      const hubId = data.recommended_hub || (nodes[0] ? nodes[0].id : 'No Hub');
      hubEl.textContent = hubId;
      const hubNode = nodes.find(n => n.id === hubId);
      if (hubNode) {
        hubEl.onclick = () => {
          vscode.postMessage({ type: 'openFile', filePath: hubNode.file_path });
        };
      }

      // Render top 6 notes by betweenness centrality or degree
      const sorted = [...nodes].sort((a, b) => (b.betweenness || 0) - (a.betweenness || 0) || (b.in_degree || 0) - (a.in_degree || 0)).slice(0, 6);
      const listEl = document.getElementById('top-notes-list');
      listEl.innerHTML = '';
      for (const node of sorted) {
        const li = document.createElement('li');
        li.className = 'note-item';
        li.innerHTML = '<span class="note-name" title="' + node.id + '">' + node.label + '</span><span class="note-tokens">' + (node.tokens || 0).toLocaleString() + ' tok</span>';
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
