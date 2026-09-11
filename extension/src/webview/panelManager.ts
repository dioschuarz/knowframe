import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { generateNonce, buildCspPolicy } from './cspHelper';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload, WebviewInboundMessage } from '../types/protocol';

export class GraphPanelManager implements vscode.Disposable {
  public static currentPanel: GraphPanelManager | undefined;
  public static readonly viewType = 'llmWiki.knowledgeGraph';

  private readonly panel: vscode.WebviewPanel;
  private readonly extensionUri: vscode.Uri;
  private disposables: vscode.Disposable[] = [];
  private ipcClient: IpcClient | null = null;
  private currentScanResult: ScanResultPayload | null = null;

  public static createOrShow(
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null,
    scanResult: ScanResultPayload | null
  ): GraphPanelManager {
    const column = vscode.window.activeTextEditor
      ? vscode.window.activeTextEditor.viewColumn
      : undefined;

    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.panel.reveal(column);
      if (scanResult) {
        GraphPanelManager.currentPanel.updateData(scanResult);
      }
      return GraphPanelManager.currentPanel;
    }

    const panel = vscode.window.createWebviewPanel(
      GraphPanelManager.viewType,
      'LLM Wiki Knowledge Graph',
      column || vscode.ViewColumn.One,
      {
        enableScripts: true,
        retainContextWhenHidden: true, // Keep Webview state and canvas simulation intact when navigating away
        localResourceRoots: [
          vscode.Uri.joinPath(extensionUri, 'dist'),
          vscode.Uri.joinPath(extensionUri, 'media'),
        ],
      }
    );

    GraphPanelManager.currentPanel = new GraphPanelManager(panel, extensionUri, ipcClient, scanResult);
    return GraphPanelManager.currentPanel;
  }

  public static revive(
    panel: vscode.WebviewPanel,
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null
  ): GraphPanelManager {
    panel.title = 'LLM Wiki Knowledge Graph';
    GraphPanelManager.currentPanel = new GraphPanelManager(panel, extensionUri, ipcClient, null);
    return GraphPanelManager.currentPanel;
  }

  private constructor(
    panel: vscode.WebviewPanel,
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null,
    scanResult: ScanResultPayload | null
  ) {
    this.panel = panel;
    this.extensionUri = extensionUri;
    this.ipcClient = ipcClient;
    this.currentScanResult = scanResult;

    this.panel.title = 'LLM Wiki Knowledge Graph';
    this.panel.iconPath = {
      light: vscode.Uri.joinPath(extensionUri, 'media', 'sidebar-icon.svg'),
      dark: vscode.Uri.joinPath(extensionUri, 'media', 'sidebar-icon.svg'),
    };

    this.updateHtml();

    this.panel.onDidDispose(() => this.dispose(), null, this.disposables);

    this.panel.onDidChangeViewState(
      (e) => {
        if (e.webviewPanel.visible && this.currentScanResult) {
          this.updateData(this.currentScanResult);
        }
      },
      null,
      this.disposables
    );

    this.panel.webview.onDidReceiveMessage(
      (message: WebviewInboundMessage) => this.handleWebviewMessage(message),
      null,
      this.disposables
    );

    if (scanResult) {
      // Small delay to allow the webview script to mount event listeners
      setTimeout(() => {
        this.updateData(scanResult);
      }, 250);
    }
  }

  public getCurrentScanResult(): ScanResultPayload | null {
    return this.currentScanResult;
  }

  public setIpcClient(client: IpcClient | null): void {
    this.ipcClient = client;
  }

  public updateData(scanResult: ScanResultPayload): void {
    this.currentScanResult = scanResult;
    this.panel.webview.postMessage({
      type: 'graphDataUpdate',
      payload: {
        nodes: scanResult.nodes.map((n) => ({
          ...n,
          hopDistance: n.hop_distance,
          inDegree: n.in_degree,
          outDegree: n.out_degree,
          brokenLinks: n.broken_links,
          filePath: n.file_path,
        })),
        edges: scanResult.edges.map((e) => ({
          ...e,
          isBroken: e.is_broken,
          linkType: e.link_type,
        })),
        tokenSummary: {
          activeHarnessTokens: scanResult.token_summary.active_harness_tokens,
          subcallAccessibleTokens: scanResult.token_summary.subcall_accessible_tokens,
          warningThreshold: scanResult.token_summary.warning_threshold,
          criticalThreshold: scanResult.token_summary.critical_threshold,
          status: scanResult.token_summary.status,
        },
        recommendedHub: scanResult.recommended_hub,
      },
    });
  }

  private async handleWebviewMessage(message: WebviewInboundMessage): Promise<void> {
    if (!message || !message.type) return;

    switch (message.type) {
      case 'openFile': {
        const payload = message.payload as { file_path: string; line?: number };
        if (payload?.file_path) {
          await this.openDocument(payload.file_path, payload.line);
        }
        break;
      }
      case 'setHub': {
        const payload = message.payload as { hub_id: string };
        if (payload?.hub_id && this.ipcClient) {
          await this.ipcClient.sendRequest('graph/setHub', { hub_id: payload.hub_id });
          vscode.commands.executeCommand('llmWiki.rescanVault');
        }
        break;
      }
      case 'rescanVault': {
        vscode.commands.executeCommand('llmWiki.rescanVault');
        break;
      }
      case 'fixBrokenLinks': {
        vscode.commands.executeCommand('llmWiki.fixBrokenLinks');
        break;
      }
      case 'updateColorGroups': {
        const payload = message.payload as { color_groups: any[] };
        if (payload?.color_groups && this.ipcClient) {
          vscode.window.showInformationMessage(`Updated ${payload.color_groups.length} cluster color rules.`);
        }
        break;
      }
      case 'requestGraphData': {
        if (this.currentScanResult) {
          this.updateData(this.currentScanResult);
        }
        break;
      }
      case 'triggerSetHubQuickPick': {
        await vscode.commands.executeCommand('llmWiki.setHub');
        break;
      }
    }
  }

  private async openDocument(filePath: string, line?: number): Promise<void> {
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
      const editor = await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.Beside,
        preview: true,
      });

      if (line && line > 0) {
        const pos = new vscode.Position(line - 1, 0);
        editor.selection = new vscode.Selection(pos, pos);
        editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
      }
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to open note: ${err.message}`);
    }
  }

  private updateHtml(): void {
    const webview = this.panel.webview;
    const nonce = generateNonce();
    const cspPolicy = buildCspPolicy(nonce, webview.cspSource);

    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this.extensionUri, 'dist', 'webview.js')
    );
    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this.extensionUri, 'dist', 'theme.css')
    );

    const templatePath = path.join(this.extensionUri.fsPath, 'dist', 'index.html');
    let html = '';

    if (fs.existsSync(templatePath)) {
      html = fs.readFileSync(templatePath, 'utf8');
      html = html
        .replace('{{CSP_POLICY}}', cspPolicy)
        .replace('{{NONCE}}', nonce)
        .replace('{{STYLE_URI}}', styleUri.toString())
        .replace('{{SCRIPT_URI}}', scriptUri.toString());
    } else {
      // Fallback in case template file isn't ready
      html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="${cspPolicy}">
  <title>LLM Wiki Knowledge Graph</title>
  <link rel="stylesheet" href="${styleUri}">
</head>
<body>
  <div id="toolbar-container"></div>
  <div id="token-container"></div>
  <canvas id="graph-canvas"></canvas>
  <div id="node-tooltip"></div>
  <div id="color-modal-container"></div>
  <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
    }

    this.panel.webview.html = html;
  }

  public dispose(): void {
    GraphPanelManager.currentPanel = undefined;
    this.panel.dispose();
    while (this.disposables.length) {
      const d = this.disposables.pop();
      if (d) {
        d.dispose();
      }
    }
  }
}

/**
 * Serializer to revive Webview when VS Code restarts
 */
export class GraphPanelSerializer implements vscode.WebviewPanelSerializer {
  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly getIpcClient: () => IpcClient | null
  ) {}

  async deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, _state: unknown): Promise<void> {
    webviewPanel.webview.options = {
      enableScripts: true,
      localResourceRoots: [vscode.Uri.joinPath(this.extensionUri, 'dist')],
    };
    GraphPanelManager.revive(webviewPanel, this.extensionUri, this.getIpcClient());
  }
}
