import * as vscode from 'vscode';
import { SecretManager } from './lifecycle/secretManager';
import { ProcessManager } from './lifecycle/processManager';
import { IpcClient } from './ipc/client';
import { ScanResultPayload } from './types/protocol';
import { GraphPanelManager, GraphPanelSerializer } from './webview/panelManager';
import { registerShowGraphCommand } from './commands/showGraph';
import { registerSetHubCommand } from './commands/setHub';
import { registerRescanVaultCommand } from './commands/rescanVault';
import { registerFixBrokenLinksCommand } from './commands/fixBrokenLinks';
import { registerInstallHooksCommand } from './commands/installHooks';
import { SidebarViewProvider } from './webview/sidebarViewProvider';

let processManager: ProcessManager | null = null;
let ipcClient: IpcClient | null = null;
let statusBarItem: vscode.StatusBarItem | null = null;
let latestScanResult: ScanResultPayload | null = null;
let sidebarProvider: SidebarViewProvider | null = null;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const outputChannel = vscode.window.createOutputChannel('LLM Wiki');
  outputChannel.appendLine('[EXTENSION] Activating LLM Wiki extension...');

  const secretManager = new SecretManager(context);
  processManager = new ProcessManager(context, secretManager, outputChannel);
  context.subscriptions.push(processManager);

  // Status Bar Item
  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBarItem.command = 'llmWiki.showKnowledgeGraph';
  statusBarItem.text = '$(graph) LLM Wiki: Initializing';
  statusBarItem.tooltip = 'Click to open LLM Wiki - Graph';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  // Register Sidebar Webview View Provider (Activity Bar)
  sidebarProvider = new SidebarViewProvider(context.extensionUri, () => latestScanResult);
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(SidebarViewProvider.viewType, sidebarProvider)
  );

  const onScanResultUpdated = (newResult: ScanResultPayload) => {
    latestScanResult = newResult;
    updateStatusBarWithBudget(newResult);
    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.updateData(newResult);
    }
    sidebarProvider?.updateData(newResult);
  };

  // Register Webview Panel Serializer
  context.subscriptions.push(
    vscode.window.registerWebviewPanelSerializer(
      GraphPanelManager.viewType,
      new GraphPanelSerializer(context.extensionUri, () => ipcClient)
    )
  );

  // Register modular commands
  context.subscriptions.push(
    registerShowGraphCommand(context, () => ipcClient, () => latestScanResult),
    registerSetHubCommand(() => ipcClient, () => latestScanResult, onScanResultUpdated),
    registerRescanVaultCommand(() => ipcClient, onScanResultUpdated),
    registerFixBrokenLinksCommand(() => latestScanResult),
    registerInstallHooksCommand(outputChannel)
  );

  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (workspaceFolders && workspaceFolders.length > 0) {
    const rootPath = workspaceFolders[0].uri.fsPath;

    try {
      statusBarItem.text = '$(sync~spin) LLM Wiki: Starting Daemon';
      const port = await processManager.start(rootPath);
      const token = secretManager.getToken() || '';

      ipcClient = new IpcClient(port, token, outputChannel);
      await ipcClient.connect();
      context.subscriptions.push(ipcClient);

      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.setIpcClient(ipcClient);
      }

      statusBarItem.text = `$(sync~spin) LLM Wiki: Scanning Vault...`;
      outputChannel.appendLine(`[EXTENSION] Sidecar connected on port ${port}. Requesting initial workspace/scan...`);

      // Initial scan (120s timeout for large vaults)
      latestScanResult = await ipcClient.sendRequest<ScanResultPayload>('workspace/scan', undefined, 120000);
      outputChannel.appendLine(`[EXTENSION] Scan completed. Nodes: ${latestScanResult.nodes.length}, Edges: ${latestScanResult.edges.length}`);
      onScanResultUpdated(latestScanResult);
      outputChannel.appendLine(`[EXTENSION] Knowledge graph and status bar ready.`);
    } catch (err: any) {
      outputChannel.appendLine(`[EXTENSION ERROR] Sidecar initialization failed: ${err.message}`);
      statusBarItem.text = '$(error) LLM Wiki: Offline';
      statusBarItem.tooltip = `Sidecar error: ${err.message}`;
    }
  } else {
    outputChannel.appendLine('[EXTENSION] No open workspace folder detected.');
    statusBarItem.text = '$(graph) LLM Wiki: No Workspace';
  }
}

function updateStatusBarWithBudget(scan: ScanResultPayload): void {
  if (!statusBarItem) {return;}
  const activeTokens = scan.token_summary.active_harness_tokens.toLocaleString();
  const subcallTokens = scan.token_summary.subcall_accessible_tokens.toLocaleString();
  const status = scan.token_summary.status;

  let icon = '$(graph)';
  if (status === 'warning') {icon = '$(warning)';}
  if (status === 'critical') {icon = '$(error)';}

  statusBarItem.text = `${icon} Wiki: ${activeTokens} tok (subcall: ${subcallTokens})`;
  statusBarItem.tooltip = `Active Prompt Load: ${activeTokens} tokens\nAccessible Subcall: ${subcallTokens} tokens\nStatus: ${status.toUpperCase()}`;
}

export async function deactivate(): Promise<void> {
  if (ipcClient) {
    ipcClient.dispose();
    ipcClient = null;
  }
  if (processManager) {
    await processManager.stop();
    processManager = null;
  }
}
