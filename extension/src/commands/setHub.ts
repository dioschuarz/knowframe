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

      // Sort nodes by total connections (in_degree + out_degree) desc, then betweenness desc, then tokens desc
      const sortedNodes = [...scan.nodes].sort((a, b) => {
        const totalA = (a.in_degree || 0) + (a.out_degree || 0);
        const totalB = (b.in_degree || 0) + (b.out_degree || 0);
        if (totalB !== totalA) {
          return totalB - totalA;
        }
        if (b.betweenness !== a.betweenness) {
          return b.betweenness - a.betweenness;
        }
        return b.tokens - a.tokens;
      });

      const items = sortedNodes.map((n, idx) => {
        const totalLinks = (n.in_degree || 0) + (n.out_degree || 0);
        const isCurrentHub = n.hop_distance === 0 || n.id === scan.recommended_hub;
        const prefix = isCurrentHub ? '$(star-full) [Current Hub] ' : (idx < 3 && totalLinks > 0 ? '$(pinned) ' : '');
        return {
          label: `${prefix}${n.label}`,
          description: `Links: ${totalLinks} (In: ${n.in_degree || 0} / Out: ${n.out_degree || 0}) • ${n.id}`,
          detail: `Centrality: ${(n.betweenness || 0).toFixed(4)} | Tokens: ${n.tokens.toLocaleString()}${n.tags && n.tags.length > 0 ? ' | Tags: ' + n.tags.join(', ') : ''}`,
          hubId: n.id,
        };
      });

      const selected = await vscode.window.showQuickPick(items, {
        placeHolder: 'Select a Markdown note to designate as central Level 0 Hub (ordered by connectivity)',
        matchOnDescription: true,
        matchOnDetail: true,
      });

      if (!selected) {return;}
      hubId = selected.hubId;
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
