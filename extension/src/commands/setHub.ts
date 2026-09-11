import * as vscode from 'vscode';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload } from '../types/protocol';

export function registerSetHubCommand(
  getIpcClient: () => IpcClient | null,
  getScanResult: () => ScanResultPayload | null,
  onScanResultUpdated: (newResult: ScanResultPayload) => void
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.setHub', async (targetHubId?: string) => {
    const ipc = getIpcClient();
    if (!ipc) {
      vscode.window.showWarningMessage('LLM Wiki: Daemon is not running.');
      return;
    }

    let hubId = targetHubId;

    if (!hubId) {
      const scan = getScanResult();
      if (!scan || scan.nodes.length === 0) {
        vscode.window.showInformationMessage('LLM Wiki: No notes available to select as Hub.');
        return;
      }

      const items = scan.nodes.map((n) => ({
        label: n.label,
        description: n.id,
        detail: `Tokens: ${n.tokens.toLocaleString()} | Centrality: ${n.betweenness.toFixed(3)}`,
      }));

      const selected = await vscode.window.showQuickPick(items, {
        placeHolder: 'Select a Markdown note to designate as central Level 0 Hub',
      });

      if (!selected) return;
      hubId = selected.description;
    }

    try {
      await ipc.sendRequest('graph/setHub', { hub_id: hubId });
      const newScan = await ipc.sendRequest<ScanResultPayload>('workspace/scan');
      onScanResultUpdated(newScan);
      vscode.window.showInformationMessage(`LLM Wiki: Designated Hub set to ${hubId}`);
    } catch (err: any) {
      vscode.window.showErrorMessage(`LLM Wiki: Failed to set Hub: ${err.message}`);
    }
  });
}
