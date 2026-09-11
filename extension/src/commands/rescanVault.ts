import * as vscode from 'vscode';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload } from '../types/protocol';
import { GraphPanelManager } from '../webview/panelManager';

export function registerRescanVaultCommand(
  getIpcClient: () => IpcClient | null,
  onScanResultUpdated: (newResult: ScanResultPayload) => void
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.rescanVault', async () => {
    const ipc = getIpcClient();
    if (!ipc) {
      vscode.window.showWarningMessage('LLM Wiki: Daemon is not running.');
      return;
    }

    try {
      const scan = await ipc.sendRequest<ScanResultPayload>('workspace/scan');
      onScanResultUpdated(scan);
      if (GraphPanelManager.currentPanel) {
        GraphPanelManager.currentPanel.updateData(scan);
      }
      vscode.window.showInformationMessage('LLM Wiki: Vault scan complete.');
    } catch (err: any) {
      vscode.window.showErrorMessage(`LLM Wiki: Rescan failed: ${err.message}`);
    }
  });
}
