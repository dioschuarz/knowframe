import * as vscode from 'vscode';
import { GraphPanelManager } from '../webview/panelManager';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload } from '../types/protocol';

export function registerShowGraphCommand(
  context: vscode.ExtensionContext,
  getIpcClient: () => IpcClient | null,
  getScanResult: () => ScanResultPayload | null
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.showKnowledgeGraph', () => {
    const ipc = getIpcClient();
    const scan = getScanResult();
    GraphPanelManager.createOrShow(context.extensionUri, ipc, scan);
  });
}
