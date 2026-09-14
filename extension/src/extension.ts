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
import { AdmissionManager } from './services/admissionManager';
import { EvolutionManager } from './evolution/evolutionManager';
import { KnowledgeQualityScore } from './types/admission';

let processManager: ProcessManager | null = null;
let ipcClient: IpcClient | null = null;
let statusBarItem: vscode.StatusBarItem | null = null;
let latestScanResult: ScanResultPayload | null = null;
let sidebarProvider: SidebarViewProvider | null = null;
let admissionManager: AdmissionManager | null = null;
let evolutionManager: EvolutionManager | null = null;
let latestQualityScore: KnowledgeQualityScore | null = null;

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
  sidebarProvider = new SidebarViewProvider(context.extensionUri, () => latestScanResult, () => latestQualityScore);
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(SidebarViewProvider.viewType, sidebarProvider)
  );

  const refreshQualityScore = async (): Promise<void> => {
    if (!admissionManager) {return;}
    try {
      latestQualityScore = await admissionManager.getQualityScore();
      outputChannel.appendLine(`[EXTENSION] Quality score updated: ${latestQualityScore.overall_score.toFixed(1)}/100`);
      sidebarProvider?.updateQuality(latestQualityScore);
      updateStatusBarWithBudget(latestScanResult, latestQualityScore);
    } catch (err: any) {
      outputChannel.appendLine(`[EXTENSION] Failed to compute quality score: ${err.message}`);
    }
  };

  const onScanResultUpdated = (newResult: ScanResultPayload) => {
    latestScanResult = newResult;
    updateStatusBarWithBudget(newResult, latestQualityScore);
    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.updateData(newResult);
    }
    sidebarProvider?.updateData(newResult, latestQualityScore);
    refreshQualityScore();
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
    registerShowGraphCommand(context, () => ipcClient, () => latestScanResult, () => evolutionManager),
    registerSetHubCommand(() => ipcClient, () => latestScanResult, onScanResultUpdated),
    registerRescanVaultCommand(() => ipcClient, onScanResultUpdated),
    registerFixBrokenLinksCommand(() => latestScanResult),
    registerInstallHooksCommand(outputChannel),
    vscode.commands.registerCommand('llmWiki.refreshQualityScore', async () => {
      await refreshQualityScore();
      if (latestQualityScore) {
        const q = latestQualityScore;
        vscode.window.showInformationMessage(
          `LLM Wiki Knowledge Quality: ${q.overall_score.toFixed(0)}/100 (Integrity: ${q.integrity_score.toFixed(0)}%, Consistency: ${q.consistency_score.toFixed(0)}%, Discoverability: ${q.discoverability_score.toFixed(0)}%, Freshness: ${q.freshness_score.toFixed(0)}%)`
        );
      }
    }),
    vscode.commands.registerCommand('llmWiki.reviewIssues', async () => {
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      await panel.openDiagnosticsTab();
    }),
    vscode.commands.registerCommand('llmWiki.reviewEvolution', async () => {
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      await panel.openEvolutionTab();
    })
  );

  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (workspaceFolders && workspaceFolders.length > 0) {
    const rootPath = workspaceFolders[0].uri.fsPath;

    try {
      statusBarItem.text = '$(sync~spin) Wiki: 10% Starting Daemon';
      const port = await processManager.start(rootPath);
      const token = secretManager.getToken() || '';

      ipcClient = new IpcClient(port, token, outputChannel);
      await ipcClient.connect();
      context.subscriptions.push(ipcClient);

      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.setIpcClient(ipcClient);
      }

      admissionManager = new AdmissionManager(ipcClient);
      const uiBridge = {
        confirmPlanApproval: async (planId: string) => {
          const choice = await vscode.window.showWarningMessage(
            `Apply structural transformation plan '${planId}'? This will modify documents atomically.`,
            { modal: true },
            'Apply Changes'
          );
          return choice === 'Apply Changes';
        },
        stageFiles: async (root: string, files: string[]) => {
          try {
            const { exec } = await import('child_process');
            const cmd = `git add ${files.map((f) => `"${f}"`).join(' ')}`;
            await new Promise<void>((resolve) => {
              exec(cmd, { cwd: root }, () => resolve());
            });
          } catch (e: any) {
            outputChannel.appendLine(`[EVOLUTION GIT STAGE] Warning: ${e.message}`);
          }
        },
        log: (msg: string) => outputChannel.appendLine(msg),
      };
      evolutionManager = new EvolutionManager(ipcClient, uiBridge);
      context.subscriptions.push({ dispose: () => evolutionManager?.dispose() });

      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.setEvolutionManager(evolutionManager);
      }
      evolutionManager.onDidChangeOrientation((orient) => {
        sidebarProvider?.updateOrientation(orient);
      });
      evolutionManager.getOrientation().then((res) => {
        sidebarProvider?.updateOrientation(res.orientation);
      }).catch(() => {});

      // Phase 1: Fast Graph Topology Scan (PRIORITY 1 - Render Graph First and Quickly!)
      statusBarItem.text = '$(sync~spin) Wiki: 30% Scanning Graph...';
      GraphPanelManager.currentPanel?.sendLoadingProgress('graph', 30, 'Scanning graph topology...');
      outputChannel.appendLine(`[EXTENSION] Sidecar connected on port ${port}. Requesting fast workspace/scan...`);

      const scanStart = Date.now();
      latestScanResult = await ipcClient.sendRequest<ScanResultPayload>('workspace/scan', undefined, 30000);
      outputChannel.appendLine(`[EXTENSION] Fast scan completed in ${Date.now() - scanStart}ms. Nodes: ${latestScanResult.nodes.length}, Edges: ${latestScanResult.edges.length}`);

      // Render Graph and Sidebar immediately
      updateStatusBarWithBudget(latestScanResult, latestQualityScore);
      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.updateData(latestScanResult);
      }
      sidebarProvider?.updateData(latestScanResult, latestQualityScore);

      // Phase 2 & 3: Background Staged Quality Evaluation & Issues Auditing
      (async () => {
        try {
          statusBarItem.text = '$(sync~spin) Wiki: 60% Checking Quality...';
          GraphPanelManager.currentPanel?.sendLoadingProgress('quality', 60, 'Evaluating 5-pillar health score...');

          latestQualityScore = await admissionManager!.getQualityScore();
          outputChannel.appendLine(`[EXTENSION] Quality score updated: ${latestQualityScore.overall_score.toFixed(1)}/100`);
          sidebarProvider?.updateQuality(latestQualityScore);
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);

          statusBarItem.text = '$(sync~spin) Wiki: 85% Auditing Issues...';
          GraphPanelManager.currentPanel?.sendLoadingProgress('issues', 85, 'Gathering diagnostic issues...');

          if (ipcClient) {
            try {
              const issuesReport = await ipcClient.sendRequest('workspace/getIssues', {});
              GraphPanelManager.setCachedIssuesReport(issuesReport);
              if (GraphPanelManager.currentPanel) {
                GraphPanelManager.currentPanel.sendIssuesReport(issuesReport);
              }
            } catch (err: any) {
              outputChannel.appendLine(`[EXTENSION] Non-fatal issues fetch warning: ${err.message}`);
            }
          }

          GraphPanelManager.currentPanel?.sendLoadingProgress('complete', 100, 'Ready');
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);
          outputChannel.appendLine('[EXTENSION] Background knowledge pipeline fully ready.');
        } catch (err: any) {
          outputChannel.appendLine(`[EXTENSION] Quality check warning: ${err.message}`);
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);
        }
      })();
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

function updateStatusBarWithBudget(scan: ScanResultPayload | null, quality: KnowledgeQualityScore | null): void {
  if (!statusBarItem) {return;}
  if (!scan) {
    statusBarItem.text = '$(graph) LLM Wiki';
    return;
  }
  const activeTokens = scan.token_summary.active_harness_tokens.toLocaleString();
  const subcallTokens = scan.token_summary.subcall_accessible_tokens.toLocaleString();
  const status = scan.token_summary.status;

  let icon = '$(graph)';
  if (status === 'warning') {icon = '$(warning)';}
  if (status === 'critical') {icon = '$(error)';}

  let qualitySuffix = '';
  if (quality) {
    const qScore = Math.round(quality.overall_score);
    qualitySuffix = ` | Q: ${qScore}/100`;
  }

  statusBarItem.text = `${icon} Wiki: ${activeTokens} tok${qualitySuffix}`;
  let tooltipText = `Active Prompt Load: ${activeTokens} tokens\nAccessible Subcall: ${subcallTokens} tokens\nStatus: ${status.toUpperCase()}`;
  if (quality) {
    const regrCount = quality.regressions_count ?? (quality as any).new_regressions_count ?? 0;
    tooltipText += `\n\nKnowledge Quality: ${quality.overall_score.toFixed(0)}/100\n- Integrity: ${quality.integrity_score.toFixed(0)}%\n- Consistency: ${quality.consistency_score.toFixed(0)}%\n- Discoverability: ${quality.discoverability_score.toFixed(0)}%\n- Freshness: ${quality.freshness_score.toFixed(0)}%\n- Debt: ${quality.legacy_debt_count} legacy, ${regrCount} regressions`;
  }
  statusBarItem.tooltip = tooltipText;
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
