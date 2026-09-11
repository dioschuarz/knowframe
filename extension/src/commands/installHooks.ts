import * as vscode from 'vscode';
import { installGitHooksWithFallback } from '../hooks/fallbackInstaller';

export function registerInstallHooksCommand(
  outputChannel?: vscode.OutputChannel
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.installGitHooks', async () => {
    const folders = vscode.workspace.workspaceFolders;
    if (!folders || folders.length === 0) {
      vscode.window.showErrorMessage('LLM Wiki: No open workspace folder found.');
      return;
    }

    const workspaceRoot = folders[0].uri.fsPath;
    const config = vscode.workspace.getConfiguration('llmWiki');
    const preferredEngine = config.get<string>('gitHookEngine', 'prek');

    outputChannel?.appendLine(`[COMMAND] Starting Git Hook installation (preference: ${preferredEngine})...`);

    const result = await installGitHooksWithFallback(workspaceRoot, preferredEngine, outputChannel);

    if (result.success) {
      vscode.window.showInformationMessage(`LLM Wiki: ${result.message}`);
    } else {
      vscode.window.showWarningMessage(`LLM Wiki: Hook installation incomplete: ${result.message}`);
    }
  });
}
