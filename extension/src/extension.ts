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
import { KnowledgeQualityScore, KnowledgeIssuesReport } from './types/admission';
import { GovernanceManager } from './governance/governanceManager';
import { AIOrientedCapabilityMatrix } from './types/governance';
import { AiReviewDispatcher } from './governance/aiReviewDispatcher';

let processManager: ProcessManager | null = null;
let ipcClient: IpcClient | null = null;
let statusBarItem: vscode.StatusBarItem | null = null;
let latestScanResult: ScanResultPayload | null = null;
let sidebarProvider: SidebarViewProvider | null = null;
let admissionManager: AdmissionManager | null = null;
let evolutionManager: EvolutionManager | null = null;
let governanceManager: GovernanceManager | null = null;
let latestQualityScore: KnowledgeQualityScore | null = null;
let latestIssuesReport: KnowledgeIssuesReport | null = null;
let latestGovernanceMatrix: AIOrientedCapabilityMatrix | null = null;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const outputChannel = vscode.window.createOutputChannel('KnowFrame');
  outputChannel.appendLine('[EXTENSION] Activating KnowFrame extension...');

  const secretManager = new SecretManager(context);
  processManager = new ProcessManager(context, secretManager, outputChannel);
  context.subscriptions.push(processManager);

  // Status Bar Item
  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBarItem.command = 'llmWiki.showKnowledgeGraph';
  statusBarItem.text = '$(llm-wiki-graph) Initializing';
  statusBarItem.tooltip = 'Click to open KnowFrame - Graph';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  // Register Sidebar Webview View Provider (Activity Bar)
  sidebarProvider = new SidebarViewProvider(
    context.extensionUri,
    () => latestScanResult,
    () => latestQualityScore,
    undefined,
    () => latestIssuesReport
  );
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(SidebarViewProvider.viewType, sidebarProvider)
  );

  const refreshQualityScore = async (): Promise<void> => {
    if (!admissionManager) {return;}
    try {
      latestQualityScore = await admissionManager.getQualityScore();
      outputChannel.appendLine(`[EXTENSION] Quality score updated: ${latestQualityScore.overall_score.toFixed(1)}/100`);
      if (governanceManager) {
        try {
          const harnessStatus = await governanceManager.getHarnessStatus();
          latestGovernanceMatrix = harnessStatus.capability_matrix || null;
        } catch {
          // ignore
        }
      }
      sidebarProvider?.updateQuality(latestQualityScore);
      updateStatusBarWithBudget(latestScanResult, latestQualityScore, latestGovernanceMatrix);
      if (ipcClient) {
        latestIssuesReport = await ipcClient.sendRequest<KnowledgeIssuesReport>('workspace/getIssues', {});
        GraphPanelManager.setCachedIssuesReport(latestIssuesReport);
        GraphPanelManager.currentPanel?.sendIssuesReport(latestIssuesReport);
        sidebarProvider?.updateIssues(latestIssuesReport);
      }
    } catch (err: any) {
      outputChannel.appendLine(`[EXTENSION] Failed to compute quality score: ${err.message}`);
    }
  };

  const onScanResultUpdated = (newResult: ScanResultPayload) => {
    latestScanResult = newResult;
    updateStatusBarWithBudget(newResult, latestQualityScore, latestGovernanceMatrix);
    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.updateData(newResult);
    }
    sidebarProvider?.updateData(newResult, latestQualityScore);
    refreshQualityScore();
  };

  const ensureIpcClient = async (): Promise<IpcClient | null> => {
    const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!rootPath) {
      return null;
    }
    if (!processManager || !processManager.isRunning()) {
      outputChannel.appendLine('[EXTENSION] Daemon not running, starting via ProcessManager...');
      try {
        const port = await processManager!.ensureRunning(rootPath);
        if (!ipcClient) {
          const token = secretManager.getToken() || '';
          ipcClient = new IpcClient(() => processManager?.port || port, token, outputChannel, rootPath);
          context.subscriptions.push(ipcClient);
        } else {
          ipcClient.setPort(port);
        }
      } catch (err: any) {
        outputChannel.appendLine(`[EXTENSION ERROR] Failed to start daemon: ${err.message}`);
        return null;
      }
    }

    if (ipcClient) {
      try {
        await ipcClient.ensureConnected();
      } catch (err: any) {
        outputChannel.appendLine(`[EXTENSION ERROR] Failed to ensure IPC client connected: ${err.message}`);
      }
    }
    return ipcClient;
  };

  processManager.onDidRestart(async (newPort) => {
    outputChannel.appendLine(`[EXTENSION] Daemon restarted on port ${newPort}. Updating IPC connection...`);
    if (ipcClient) {
      ipcClient.setPort(newPort);
      try {
        await ipcClient.ensureConnected();
        const scan = await ipcClient.sendRequest<ScanResultPayload>('workspace/scan');
        onScanResultUpdated(scan);
      } catch (err: any) {
        outputChannel.appendLine(`[EXTENSION] Background recovery after daemon restart warning: ${err.message}`);
      }
    }
  });

  // Register Webview Panel Serializer
  context.subscriptions.push(
    vscode.window.registerWebviewPanelSerializer(
      GraphPanelManager.viewType,
      new GraphPanelSerializer(context.extensionUri, () => ipcClient, () => latestScanResult)
    )
  );

  // Register modular commands
  context.subscriptions.push(
    registerShowGraphCommand(context, () => ipcClient, () => latestScanResult, () => evolutionManager, () => governanceManager),
    registerSetHubCommand(() => ipcClient, () => latestScanResult, onScanResultUpdated),
    registerRescanVaultCommand(ensureIpcClient, onScanResultUpdated),
    registerFixBrokenLinksCommand(() => latestScanResult),
    registerInstallHooksCommand(outputChannel),
    vscode.commands.registerCommand('llmWiki.openFolder', () => {
      vscode.commands.executeCommand('vscode.openFolder');
    }),
    vscode.commands.registerCommand('llmWiki.initializeHarness', async () => {
      const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
      if (rootPath && governanceManager) {
        const created = await governanceManager.initializeHarness(rootPath);
        if (created) {
          vscode.window.showInformationMessage('Successfully initialized AGENTS.md template with embedded Knowledge Index.');
        } else {
          vscode.window.showInformationMessage('AGENTS.md already exists in workspace.');
        }
      }
    }),
    vscode.commands.registerCommand('llmWiki.refreshQualityScore', async () => {
      await vscode.commands.executeCommand('llmWiki.rescanVault');
    }),
    vscode.commands.registerCommand('llmWiki.restartDaemon', async () => {
      const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
      if (rootPath) {
        outputChannel.appendLine('[EXTENSION] User triggered manual daemon restart');
        vscode.window.showInformationMessage('Restarting KnowFrame Knowledge Engine daemon...');
        try {
          await processManager.stop();
          const port = await processManager.start(rootPath, 0);
          await ensureIpcClient();
          await vscode.commands.executeCommand('llmWiki.rescanVault');
          vscode.window.showInformationMessage(`KnowFrame daemon restarted successfully on port ${port}.`);
        } catch (err: any) {
          vscode.window.showErrorMessage(`Failed to restart KnowFrame daemon: ${err.message}`);
        }
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
    }),
    vscode.commands.registerCommand('llmWiki.navigateToDetail', async (intent?: any) => {
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      if (intent) {
        await panel.navigateToDetail(intent);
      }
    }),
    vscode.commands.registerCommand('llmWiki.openSettings', async (tab?: string) => {
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      if (governanceManager) {
        panel.setGovernanceManager(governanceManager);
      }
      await panel.openSettingsDrawer(typeof tab === 'string' ? tab : 'knowledge');
    }),
    vscode.commands.registerCommand('llmWiki.openHarness', async () => {
      await vscode.commands.executeCommand('llmWiki.openSettings', 'harness');
    }),
    vscode.commands.registerCommand('llmWiki.resolveDocumentReview', async (payload?: any) => {
      if (ipcClient && payload) {
        try {
          const docIds = payload.doc_ids || (payload.doc_id ? [payload.doc_id] : (payload.file_path ? [payload.file_path] : []));
          if (docIds.length > 0) {
            await ipcClient.sendRequest('review/resolveDocumentReview', { doc_ids: docIds });
            vscode.window.showInformationMessage(`Document review resolved for ${docIds.join(', ')}.`);
            await vscode.commands.executeCommand('llmWiki.rescanVault');
          }
        } catch (err: any) {
          vscode.window.showErrorMessage(`Failed to resolve document review: ${err.message}`);
        }
      }
    }),
    vscode.commands.registerCommand('llmWiki.reviewWithAi', async (payload?: any) => {
      const dispatcher = new AiReviewDispatcher(undefined, governanceManager || undefined);
      const filePath = payload?.filePath || payload?.doc_id || payload?.file_path || (typeof payload === 'string' ? payload : undefined);
      const issue = payload?.issue || (payload?.category ? payload : undefined);
      await dispatcher.dispatchReview({ filePath, issue });
    }),
    vscode.commands.registerCommand('llmWiki.reviewAllWithAi', async () => {
      const dispatcher = new AiReviewDispatcher(undefined, governanceManager || undefined);
      await dispatcher.dispatchReview({ all: true });
    })
  );

  async function initWorkspace(rootPath: string): Promise<void> {
    try {
      if (statusBarItem) {
        statusBarItem.text = '$(sync~spin) 10% Starting Daemon';
        statusBarItem.command = 'llmWiki.showKnowledgeGraph';
        statusBarItem.tooltip = 'Click to open KnowFrame - Graph';
      }
      const port = await processManager!.start(rootPath);
      const token = secretManager.getToken() || '';

      ipcClient = new IpcClient(() => processManager?.port || port, token, outputChannel, rootPath);
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
      governanceManager = new GovernanceManager(ipcClient);

      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.setEvolutionManager(evolutionManager);
        GraphPanelManager.currentPanel.setGovernanceManager(governanceManager);
      }
      evolutionManager.onDidChangeOrientation((orient) => {
        sidebarProvider?.updateOrientation(orient);
      });
      evolutionManager.getOrientation().then((res) => {
        sidebarProvider?.updateOrientation(res.orientation);
      }).catch(() => {});

      // Phase 1: Fast Graph Topology Scan (PRIORITY 1 - Render Graph First and Quickly!)
      if (statusBarItem) {
        statusBarItem.text = '$(sync~spin) 30% Scanning Graph...';
      }
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
          if (statusBarItem) {
            statusBarItem.text = '$(sync~spin) 60% Checking Quality...';
          }
          GraphPanelManager.currentPanel?.sendLoadingProgress('quality', 60, 'Evaluating 5-pillar health score...');

          latestQualityScore = await admissionManager!.getQualityScore();
          outputChannel.appendLine(`[EXTENSION] Quality score updated: ${latestQualityScore.overall_score.toFixed(1)}/100`);
          sidebarProvider?.updateQuality(latestQualityScore);
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);

          if (statusBarItem) {
            statusBarItem.text = '$(sync~spin) 85% Auditing Issues...';
          }
          GraphPanelManager.currentPanel?.sendLoadingProgress('issues', 85, 'Gathering diagnostic issues...');

          if (ipcClient) {
            try {
              latestIssuesReport = await ipcClient.sendRequest<KnowledgeIssuesReport>('workspace/getIssues', {});
              GraphPanelManager.setCachedIssuesReport(latestIssuesReport);
              if (GraphPanelManager.currentPanel) {
                GraphPanelManager.currentPanel.sendIssuesReport(latestIssuesReport);
              }
              sidebarProvider?.updateIssues(latestIssuesReport);
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
      if (statusBarItem) {
        statusBarItem.text = '$(error) Offline';
        statusBarItem.tooltip = `Sidecar error: ${err.message}`;
      }
      GraphPanelManager.currentPanel?.sendEngineOffline(err.message);
    }
  }

  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (workspaceFolders && workspaceFolders.length > 0) {
    await initWorkspace(workspaceFolders[0].uri.fsPath);
  } else {
    outputChannel.appendLine('[EXTENSION] No open workspace folder detected.');
    if (statusBarItem) {
      statusBarItem.text = '$(folder-opened) KnowFrame: Open Folder';
      statusBarItem.tooltip = 'Click to open a workspace folder or Obsidian vault';
      statusBarItem.command = 'llmWiki.openFolder';
    }
    GraphPanelManager.currentPanel?.sendNoWorkspace();
  }

  context.subscriptions.push(
    vscode.workspace.onDidChangeWorkspaceFolders(async (event) => {
      if (event.added.length > 0) {
        const rootPath = event.added[0].uri.fsPath;
        outputChannel.appendLine(`[EXTENSION] Workspace folder opened: ${rootPath}`);
        await initWorkspace(rootPath);
      } else if (!vscode.workspace.workspaceFolders || vscode.workspace.workspaceFolders.length === 0) {
        outputChannel.appendLine('[EXTENSION] All workspace folders closed.');
        latestScanResult = null;
        latestQualityScore = null;
        latestIssuesReport = null;
        if (statusBarItem) {
          statusBarItem.text = '$(folder-opened) KnowFrame: Open Folder';
          statusBarItem.tooltip = 'Click to open a workspace folder or Obsidian vault';
          statusBarItem.command = 'llmWiki.openFolder';
        }
        GraphPanelManager.currentPanel?.sendNoWorkspace();
        sidebarProvider?.updateData(null as any, null);
      }
    })
  );
}

function updateStatusBarWithBudget(
  scan: ScanResultPayload | null,
  quality: KnowledgeQualityScore | null,
  governance?: AIOrientedCapabilityMatrix | null
): void {
  if (!statusBarItem) {return;}
  if (!scan) {
    statusBarItem.text = '$(llm-wiki-graph)';
    return;
  }
  const activeTokens = scan.token_summary.active_harness_tokens.toLocaleString();
  const subcallTokens = scan.token_summary.subcall_accessible_tokens.toLocaleString();
  const status = scan.token_summary.status;

  let icon = '$(llm-wiki-graph)';
  if (status === 'warning') {icon = '$(warning)';}
  if (status === 'critical') {icon = '$(error)';}

  let qualitySuffix = '';
  if (quality) {
    const qScore = Math.round(quality.overall_score);
    qualitySuffix = ` | Q: ${qScore}/100`;
  }

  statusBarItem.text = `${icon} ${activeTokens} tok${qualitySuffix}`;
  let tooltipText = `Active Prompt Load: ${activeTokens} tokens\nAccessible Subcall: ${subcallTokens} tokens\nStatus: ${status.toUpperCase()}`;
  if (quality) {
    const regrCount = quality.regressions_count ?? (quality as any).new_regressions_count ?? 0;
    const traceScore = (quality as any).traceability_score != null ? (quality as any).traceability_score : quality.freshness_score;
    tooltipText += `\n\nKnowledge Quality: ${quality.overall_score.toFixed(0)}/100\n- Integrity: ${quality.integrity_score.toFixed(0)}%\n- Consistency: ${quality.consistency_score.toFixed(0)}%\n- Discoverability: ${quality.discoverability_score.toFixed(0)}%\n- Traceability: ${traceScore.toFixed(0)}%\n- Issues: ${regrCount} active regressions, ${quality.legacy_debt_count} baselined`;
  }
  if (governance) {
    tooltipText += `\n\nAI Governance: ${governance.overall_governance_score.toFixed(0)}/100\n- Entry: ${governance.entry_score.toFixed(0)}%\n- Discovery: ${governance.discovery_score.toFixed(0)}%\n- Retrieval: ${governance.retrieval_score.toFixed(0)}%\n- Reuse: ${governance.reuse_score.toFixed(0)}%`;
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
