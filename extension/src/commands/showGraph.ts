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
  getGovernanceManager?: () => GovernanceManager | null
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
        'LLM Wiki: No open workspace folder detected. Please open a folder to explore the knowledge graph.',
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
    } else if (ipc) {
      panel.sendLoadingProgress('graph', 30, 'Scanning vault topology...');
      vscode.commands.executeCommand('llmWiki.rescanVault');
    } else {
      panel.sendLoadingProgress('graph', 15, 'Starting LLM Wiki engine daemon...');
    }
  });
}

