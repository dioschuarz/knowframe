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
import { registerMakeLinksClickableCommand } from './commands/makeLinksClickable';
import { registerInstallHooksCommand } from './commands/installHooks';
import { KnowFrameMarkdownDocumentLinkProvider } from './providers/markdownDocumentLinkProvider';
import { registerKnowFrameWikilinkPlugin } from './providers/markdownItWikilinks';
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

export async function activate(
  context: vscode.ExtensionContext
): Promise<{ extendMarkdownIt: (md: any) => any }> {
  const outputChannel = vscode.window.createOutputChannel('KnowFrame');
  outputChannel.appendLine('[EXTENSION] Activating KnowFrame extension...');

  const secretManager = new SecretManager(context);
  processManager = new ProcessManager(context, secretManager, outputChannel);
  context.subscriptions.push(processManager);

  const getExecutionMode = (): 'on_demand' | 'live_watch' => {
    const raw = vscode.workspace.getConfiguration('llmWiki').get<string>('executionMode', 'on_demand');
    return raw === 'live_watch' ? 'live_watch' : 'on_demand';
  };

  const getIdleTimeoutSeconds = (): number => {
    const val = vscode.workspace.getConfiguration('llmWiki').get<number>('daemonIdleTimeoutSeconds', 60);
    return typeof val === 'number' && val >= 0 ? val : 60;
  };

  processManager.setAutoRestart(getExecutionMode() === 'live_watch');

  let idleShutdownTimer: ReturnType<typeof setTimeout> | null = null;
  let inFlightInitPromise: Promise<void> | null = null;

  const clearIdleShutdownTimer = (): void => {
    if (idleShutdownTimer) {
      clearTimeout(idleShutdownTimer);
      idleShutdownTimer = null;
    }
  };

  const stopDaemonAndReleaseMemory = async (reason: string): Promise<void> => {
    clearIdleShutdownTimer();
    if (ipcClient) {
      ipcClient.dispose();
      ipcClient = null;
    }
    admissionManager = null;
    if (evolutionManager) {
      evolutionManager.dispose();
      evolutionManager = null;
    }
    governanceManager = null;
    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.setIpcClient(null);
      GraphPanelManager.currentPanel.setEvolutionManager(null);
      GraphPanelManager.currentPanel.setGovernanceManager(null);
    }
    if (processManager && processManager.isRunning()) {
      outputChannel.appendLine(`[EXTENSION] Stopping KnowFrame daemon (${reason}) to release memory...`);
      await processManager.stop();
    }
    if (statusBarItem && !latestScanResult) {
      statusBarItem.text = '$(llm-wiki-graph) KnowFrame';
      statusBarItem.tooltip = 'KnowFrame (On-Demand — click to analyze workspace)';
    }
  };

  const scheduleIdleShutdown = (): void => {
    clearIdleShutdownTimer();
    if (getExecutionMode() !== 'on_demand') {
      return;
    }
    const timeoutSec = getIdleTimeoutSeconds();
    if (timeoutSec <= 0) {
      return;
    }
    idleShutdownTimer = setTimeout(() => {
      idleShutdownTimer = null;
      if (GraphPanelManager.currentPanel) {
        return;
      }
      if (GraphPanelManager.isPipelineActive() || ipcClient?.hasPendingRequests()) {
        scheduleIdleShutdown();
        return;
      }
      void stopDaemonAndReleaseMemory(`idle timeout ${timeoutSec}s after panel close`);
    }, timeoutSec * 1000);
  };

  // Status Bar Item
  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBarItem.command = 'llmWiki.showKnowledgeGraph';
  statusBarItem.text = getExecutionMode() === 'on_demand' ? '$(llm-wiki-graph) KnowFrame' : '$(llm-wiki-graph) Initializing';
  statusBarItem.tooltip =
    getExecutionMode() === 'on_demand'
      ? 'KnowFrame (On-Demand — click to open Graph & analyze workspace)'
      : 'Click to open KnowFrame - Graph';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  const ensureWorkspaceAnalyzed = async (): Promise<void> => {
    const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!rootPath) {
      return;
    }
    clearIdleShutdownTimer();
    if (inFlightInitPromise) {
      return inFlightInitPromise;
    }
    if (latestScanResult && processManager?.isRunning() && ipcClient?.isConnected()) {
      return;
    }
    inFlightInitPromise = initWorkspace(rootPath).finally(() => {
      inFlightInitPromise = null;
    });
    return inFlightInitPromise;
  };

  GraphPanelManager.setOnPanelDisposed(() => {
    scheduleIdleShutdown();
  });
  GraphPanelManager.setOnPanelReadyNeedsScan(() => {
    void ensureWorkspaceAnalyzed();
  });

  // Register Sidebar Webview View Provider (Activity Bar)
  sidebarProvider = new SidebarViewProvider(
    context.extensionUri,
    () => latestScanResult,
    () => latestQualityScore,
    undefined,
    () => latestIssuesReport,
    () => {
      // Trigger a single on-demand analysis only when user explicitly opens the KnowFrame Sidebar
      if (!latestScanResult && !GraphPanelManager.isPipelineActive()) {
        void ensureWorkspaceAnalyzed();
      }
    }
  );
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(SidebarViewProvider.viewType, sidebarProvider)
  );

  const refreshQualityScore = async (): Promise<void> => {
    if (!admissionManager) {
      GraphPanelManager.setPipelineRunning(false);
      return;
    }
    try {
      GraphPanelManager.reportLoadingProgress('quality', 65, 'Evaluating 5-pillar health score...');
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
        GraphPanelManager.reportLoadingProgress('issues', 85, 'Gathering diagnostic issues...');
        latestIssuesReport = await ipcClient.sendRequest<KnowledgeIssuesReport>('workspace/getIssues', { limit: 500 });
        GraphPanelManager.setCachedIssuesReport(latestIssuesReport);
        if (GraphPanelManager.currentPanel) {
          GraphPanelManager.currentPanel.sendIssuesReport(latestIssuesReport);
        } else {
          GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
        }
        sidebarProvider?.updateIssues(latestIssuesReport);
      } else {
        GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
      }
    } catch (err: any) {
      outputChannel.appendLine(`[EXTENSION] Failed to compute quality score: ${err.message}`);
      GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
    } finally {
      GraphPanelManager.setPipelineRunning(false);
    }
  };

  const onScanResultUpdated = async (newResult: ScanResultPayload) => {
    latestScanResult = newResult;
    GraphPanelManager.setPipelineRunning(true);
    updateStatusBarWithBudget(newResult, latestQualityScore, latestGovernanceMatrix);
    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.updateData(newResult);
    } else {
      GraphPanelManager.reportLoadingProgress('layout', 50, 'Preparing graph topology & layout...');
    }
    sidebarProvider?.updateData(newResult, latestQualityScore);
    await refreshQualityScore();
  };

  const ensureIpcClient = async (): Promise<IpcClient | null> => {
    const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!rootPath) {
      return null;
    }
    clearIdleShutdownTimer();
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
        await ipcClient.ensureConnected();
        if (!admissionManager) {
          admissionManager = new AdmissionManager(ipcClient);
        }
        if (!governanceManager) {
          governanceManager = new GovernanceManager(ipcClient);
        }
        if (GraphPanelManager.currentPanel) {
          GraphPanelManager.currentPanel.setIpcClient(ipcClient);
          GraphPanelManager.currentPanel.setGovernanceManager(governanceManager);
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
        const scan = await ipcClient.sendRequest<ScanResultPayload>(
          'workspace/scan',
          { include_fragments: false },
          180000
        );
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

  // Register Markdown Editor Ctrl+Click DocumentLinkProvider for wikilinks & fragments
  context.subscriptions.push(
    vscode.languages.registerDocumentLinkProvider(
      { language: 'markdown' },
      new KnowFrameMarkdownDocumentLinkProvider(() => latestScanResult)
    )
  );

  // Register modular commands
  context.subscriptions.push(
    registerShowGraphCommand(
      context,
      () => ipcClient,
      () => latestScanResult,
      () => evolutionManager,
      () => governanceManager,
      ensureWorkspaceAnalyzed
    ),
    registerSetHubCommand(() => ipcClient, () => latestScanResult, onScanResultUpdated),
    registerRescanVaultCommand(ensureIpcClient, onScanResultUpdated),
    registerFixBrokenLinksCommand(() => latestScanResult),
    registerMakeLinksClickableCommand(ensureIpcClient),
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
          await stopDaemonAndReleaseMemory('manual restart');
          await ensureWorkspaceAnalyzed();
          vscode.window.showInformationMessage('KnowFrame daemon restarted successfully.');
        } catch (err: any) {
          vscode.window.showErrorMessage(`Failed to restart KnowFrame daemon: ${err.message}`);
        }
      }
    }),
    vscode.commands.registerCommand('llmWiki.stopDaemon', async () => {
      await stopDaemonAndReleaseMemory('manual user stop');
      vscode.window.showInformationMessage('KnowFrame engine daemon stopped — memory freed.');
    }),
    vscode.commands.registerCommand('llmWiki.reviewIssues', async () => {
      await ensureWorkspaceAnalyzed();
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      await panel.openDiagnosticsTab();
    }),
    vscode.commands.registerCommand('llmWiki.reviewEvolution', async () => {
      await ensureWorkspaceAnalyzed();
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      await panel.openEvolutionTab();
    }),
    vscode.commands.registerCommand('llmWiki.navigateToDetail', async (intent?: any) => {
      await ensureWorkspaceAnalyzed();
      const panel = GraphPanelManager.createOrShow(context.extensionUri, ipcClient, latestScanResult);
      if (evolutionManager) {
        panel.setEvolutionManager(evolutionManager);
      }
      if (intent) {
        await panel.navigateToDetail(intent);
      }
    }),
    vscode.commands.registerCommand('llmWiki.openSettings', async (tab?: string) => {
      await ensureWorkspaceAnalyzed();
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
      const issue = Array.isArray(payload?.issue) ? undefined : (payload?.issue || (payload?.category ? payload : undefined));
      const issues = Array.isArray(payload?.issue) ? payload.issue : (Array.isArray(payload?.issues) ? payload.issues : undefined);
      await dispatcher.dispatchReview({ filePath, issue, issues });
    }),
    vscode.commands.registerCommand('llmWiki.reviewAllWithAi', async () => {
      const dispatcher = new AiReviewDispatcher(undefined, governanceManager || undefined);
      await dispatcher.dispatchReview({ all: true });
    })
  );

  async function initWorkspace(rootPath: string): Promise<void> {
    try {
      GraphPanelManager.setPipelineRunning(true);
      GraphPanelManager.reportLoadingProgress('init', 15, 'Starting KnowFrame engine daemon...', true);
      if (statusBarItem) {
        statusBarItem.text = '$(sync~spin) 15% Starting Daemon';
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
      GraphPanelManager.reportLoadingProgress('graph', 30, 'Scanning graph topology...');
      outputChannel.appendLine(`[EXTENSION] Sidecar connected on port ${port}. Requesting fast workspace/scan...`);

      const scanStart = Date.now();
      latestScanResult = await ipcClient.sendRequest<ScanResultPayload>(
        'workspace/scan',
        { include_fragments: false },
        180000
      );
      outputChannel.appendLine(`[EXTENSION] Fast scan completed in ${Date.now() - scanStart}ms. Nodes: ${latestScanResult.nodes.length}, Edges: ${latestScanResult.edges.length}`);

      // Render Graph and Sidebar immediately
      updateStatusBarWithBudget(latestScanResult, latestQualityScore);
      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.updateData(latestScanResult);
      } else {
        GraphPanelManager.reportLoadingProgress('layout', 50, 'Preparing graph topology & layout...');
      }
      sidebarProvider?.updateData(latestScanResult, latestQualityScore);

      // Phase 2 & 3: Background Staged Quality Evaluation & Issues Auditing (cached in Rust engine)
      (async () => {
        try {
          if (statusBarItem) {
            statusBarItem.text = '$(sync~spin) 65% Checking Quality...';
          }
          GraphPanelManager.reportLoadingProgress('quality', 65, 'Evaluating 5-pillar health score...');

          latestQualityScore = await admissionManager!.getQualityScore();
          outputChannel.appendLine(`[EXTENSION] Quality score updated: ${latestQualityScore.overall_score.toFixed(1)}/100`);
          sidebarProvider?.updateQuality(latestQualityScore);
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);

          if (statusBarItem) {
            statusBarItem.text = '$(sync~spin) 85% Auditing Issues...';
          }
          GraphPanelManager.reportLoadingProgress('issues', 85, 'Gathering diagnostic issues...');

          if (ipcClient) {
            try {
              latestIssuesReport = await ipcClient.sendRequest<KnowledgeIssuesReport>('workspace/getIssues', { limit: 500 });
              GraphPanelManager.setCachedIssuesReport(latestIssuesReport);
              if (GraphPanelManager.currentPanel) {
                GraphPanelManager.currentPanel.sendIssuesReport(latestIssuesReport);
              }
              sidebarProvider?.updateIssues(latestIssuesReport);
            } catch (err: any) {
              outputChannel.appendLine(`[EXTENSION] Non-fatal issues fetch warning: ${err.message}`);
            }
          }

          GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);
          outputChannel.appendLine('[EXTENSION] Background knowledge pipeline fully ready.');
        } catch (err: any) {
          outputChannel.appendLine(`[EXTENSION] Quality check warning: ${err.message}`);
          GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
          updateStatusBarWithBudget(latestScanResult, latestQualityScore);
        } finally {
          GraphPanelManager.setPipelineRunning(false);
          if (!GraphPanelManager.currentPanel) {
            scheduleIdleShutdown();
          }
        }
      })();
    } catch (err: any) {
      GraphPanelManager.setPipelineRunning(false);
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
    if (getExecutionMode() === 'live_watch') {
      await initWorkspace(workspaceFolders[0].uri.fsPath);
    } else {
      outputChannel.appendLine('[EXTENSION] Running in on_demand mode — 0 background daemon RAM until user opens KnowFrame.');
    }
  } else {
    outputChannel.appendLine('[EXTENSION] No open workspace folder detected.');
    if (statusBarItem) {
      statusBarItem.text = '$(folder-opened) KnowFrame: Open Folder';
      statusBarItem.tooltip = 'Click to open a workspace folder or Obsidian vault';
      statusBarItem.command = 'llmWiki.openFolder';
    }
    GraphPanelManager.currentPanel?.sendNoWorkspace();
  }

  let mdWatchDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let isBackgroundRescanning = false;
  let mdWatcher: vscode.FileSystemWatcher | null = null;

  const triggerSilentRescan = (uri?: vscode.Uri) => {
    if (getExecutionMode() !== 'live_watch') {
      return;
    }
    if (uri) {
      const p = uri.fsPath.replace(/\\/g, '/');
      if (
        p.includes('/.git/') ||
        p.includes('/node_modules/') ||
        p.includes('/target/') ||
        p.includes('/dist/') ||
        p.includes('/.gemini/')
      ) {
        return;
      }
    }
    if (mdWatchDebounceTimer) {
      clearTimeout(mdWatchDebounceTimer);
    }
    mdWatchDebounceTimer = setTimeout(async () => {
      if (isBackgroundRescanning || !ipcClient) {
        return;
      }
      isBackgroundRescanning = true;
      try {
        GraphPanelManager.setCachedIssuesReport(null);
        const scan = await ipcClient.sendRequest<ScanResultPayload>(
          'workspace/scan',
          { include_fragments: false },
          180000
        );
        await onScanResultUpdated(scan);
      } catch (err: any) {
        outputChannel.appendLine(`[EXTENSION] Background file-watch rescan warning: ${err.message}`);
      } finally {
        isBackgroundRescanning = false;
      }
    }, 1200);
  };

  const syncFileWatcherWithExecutionMode = (): void => {
    const isLive = getExecutionMode() === 'live_watch';
    processManager?.setAutoRestart(isLive);
    if (isLive && !mdWatcher) {
      mdWatcher = vscode.workspace.createFileSystemWatcher('**/*.md');
      mdWatcher.onDidChange(triggerSilentRescan);
      mdWatcher.onDidCreate(triggerSilentRescan);
      mdWatcher.onDidDelete(triggerSilentRescan);
      outputChannel.appendLine('[EXTENSION] Live watch enabled: monitoring **/*.md changes.');
    } else if (!isLive && mdWatcher) {
      mdWatcher.dispose();
      mdWatcher = null;
      if (mdWatchDebounceTimer) {
        clearTimeout(mdWatchDebounceTimer);
        mdWatchDebounceTimer = null;
      }
      outputChannel.appendLine('[EXTENSION] On-demand mode active: markdown file watcher disabled.');
    }
  };

  syncFileWatcherWithExecutionMode();
  context.subscriptions.push({
    dispose: () => {
      clearIdleShutdownTimer();
      mdWatcher?.dispose();
      mdWatcher = null;
    },
  });

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('llmWiki.executionMode')) {
        syncFileWatcherWithExecutionMode();
        if (getExecutionMode() === 'live_watch' && vscode.workspace.workspaceFolders?.[0]) {
          void ensureWorkspaceAnalyzed();
        } else if (getExecutionMode() === 'on_demand' && !GraphPanelManager.currentPanel) {
          scheduleIdleShutdown();
        }
      }
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeWorkspaceFolders(async (event) => {
      if (event.added.length > 0) {
        const rootPath = event.added[0].uri.fsPath;
        outputChannel.appendLine(`[EXTENSION] Workspace folder opened: ${rootPath}`);
        if (getExecutionMode() === 'live_watch') {
          await initWorkspace(rootPath);
        }
      } else if (!vscode.workspace.workspaceFolders || vscode.workspace.workspaceFolders.length === 0) {
        outputChannel.appendLine('[EXTENSION] All workspace folders closed.');
        latestScanResult = null;
        latestQualityScore = null;
        latestIssuesReport = null;
        await stopDaemonAndReleaseMemory('all workspace folders closed');
        if (statusBarItem) {
          statusBarItem.text = '$(folder-opened) KnowFrame: Open Folder';
          statusBarItem.tooltip = 'Click to open a workspace folder or Obsidian vault';
          statusBarItem.command = 'llmWiki.openFolder';
        }
        GraphPanelManager.currentPanel?.sendNoWorkspace();
        sidebarProvider?.updateData(null, null);
      }
    })
  );

  // Export markdown-it plugin so VS Code's built-in Markdown Preview (Ctrl+Shift+V)
  // renders every valid [[wikilink#fragment|Label]] as a real clickable hyperlink.
  return {
    extendMarkdownIt(md: any) {
      return registerKnowFrameWikilinkPlugin(md, {
        isEnabled: () =>
          vscode.workspace
            .getConfiguration('llmWiki')
            .get<boolean>('enableMarkdownPreviewWikilinks', true) !== false,
        getWorkspaceRoot: (sourcePath?: string) => {
          if (sourcePath) {
            const folder = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(sourcePath));
            if (folder) {
              return folder.uri.fsPath;
            }
          }
          return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || null;
        },
        getFallbackDocumentPath: () => {
          const active = vscode.window.activeTextEditor?.document;
          if (active && (active.languageId === 'markdown' || active.uri.fsPath.endsWith('.md'))) {
            return active.uri.fsPath;
          }
          return null;
        },
        getScanResult: () => latestScanResult,
      });
    },
  };
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
