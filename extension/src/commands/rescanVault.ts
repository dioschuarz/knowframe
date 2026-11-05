import * as vscode from 'vscode';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload } from '../types/protocol';
import { GraphPanelManager } from '../webview/panelManager';

export function registerRescanVaultCommand(
  getOrEnsureIpcClient: () => Promise<IpcClient | null> | (IpcClient | null),
  onScanResultUpdated: (newResult: ScanResultPayload) => Promise<void> | void
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.rescanVault', async () => {
    await vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Notification,
        title: 'KnowFrame: Rescanning Vault...',
        cancellable: false,
      },
      async (progress) => {
        try {
          progress.report({ message: 'Connecting to daemon...' });
          const ipc = await getOrEnsureIpcClient();
          if (!ipc) {
            vscode.window.showWarningMessage('KnowFrame: Daemon is not running and could not be started.');
            return;
          }

          GraphPanelManager.setCachedIssuesReport(null);
          GraphPanelManager.setPipelineRunning(true);
          progress.report({ message: 'Scanning markdown documents & graph topology...' });
          GraphPanelManager.reportLoadingProgress('graph', 30, 'Rescanning vault...', true);

          const scan = await ipc.sendRequest<ScanResultPayload>('workspace/scan');

          progress.report({ message: 'Reconciling baseline debt & quality metrics...' });
          await onScanResultUpdated(scan);

          const docCount = scan.nodes?.length ?? 0;
          const refCount = scan.edges?.length ?? 0;
          vscode.window.showInformationMessage(
            `KnowFrame: Vault scan complete (${docCount} documents, ${refCount} references).`
          );
        } catch (err: any) {
          GraphPanelManager.setPipelineRunning(false);
          GraphPanelManager.reportLoadingProgress('complete', 100, 'Ready');
          vscode.window.showErrorMessage(`KnowFrame: Rescan failed: ${err.message}`);
        }
      }
    );
  });
}
