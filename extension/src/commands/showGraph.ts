import * as vscode from 'vscode';
import { GraphPanelManager } from '../webview/panelManager';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload } from '../types/protocol';
import type { EvolutionManager } from '../evolution/evolutionManager';
import type { GovernanceManager } from '../governance/governanceManager';

export function registerShowGraphCommand(
  context: vscode.ExtensionContext,
  getIpcClient: () => IpcClient | null,
  getScanResult: () => ScanResultPayload | null,
  getEvolutionManager?: () => EvolutionManager | null,
  getGovernanceManager?: () => GovernanceManager | null,
  onEnsureAnalysis?: () => Promise<void>
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.showKnowledgeGraph', () => {
    const ipc = getIpcClient();
    const scan = getScanResult();
    const hasWorkspace = !!(vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0);

    const panel = GraphPanelManager.createOrShow(context.extensionUri, ipc, scan);
    if (getEvolutionManager) {
      const em = getEvolutionManager();
      if (em) {
        panel.setEvolutionManager(em);
      }
    }
    if (getGovernanceManager) {
      const gm = getGovernanceManager();
      if (gm) {
        panel.setGovernanceManager(gm);
      }
    }

    if (!hasWorkspace) {
      panel.sendNoWorkspace();
      vscode.window.showInformationMessage(
        'KnowFrame: No open workspace folder detected. Please open a folder to explore the knowledge graph.',
        'Open Folder'
      ).then((choice) => {
        if (choice === 'Open Folder') {
          vscode.commands.executeCommand('vscode.openFolder');
        }
      });
      return;
    }

    if (scan) {
      panel.updateData(scan);
    } else if (onEnsureAnalysis) {
      if (!GraphPanelManager.isPipelineActive()) {
        panel.sendLoadingProgress('init', 15, 'Starting KnowFrame engine on demand...', true);
        void onEnsureAnalysis();
      }
    } else if (ipc) {
      if (!GraphPanelManager.isPipelineActive()) {
        panel.sendLoadingProgress('graph', 30, 'Scanning vault topology...', true);
        vscode.commands.executeCommand('llmWiki.rescanVault');
      }
    } else {
      panel.sendLoadingProgress('init', 15, 'Starting KnowFrame engine daemon...');
    }
  });
}

