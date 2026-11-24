import * as vscode from 'vscode';
import * as path from 'path';
import { IpcClient } from '../ipc/client';

interface WorkspaceLinkNormalizationReport {
  applied: boolean;
  files_scanned: number;
  files_modified: number;
  total_rewrites: number;
  total_broken_skipped: number;
  total_already_clickable: number;
  documents: Array<{
    file_path: string;
    unified_diff: string;
    rewrites: Array<{
      line: number;
      original: string;
      replacement: string;
      resolved_target: string;
      resolved_fragment?: string;
    }>;
  }>;
}

export function registerMakeLinksClickableCommand(
  ensureIpcClient: () => Promise<IpcClient | null>
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.makeLinksClickable', async (targetFileArg?: string) => {
    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceRoot) {
      vscode.window.showWarningMessage('KnowFrame: Open a workspace folder first.');
      return;
    }

    const client = await ensureIpcClient();
    if (!client) {
      vscode.window.showErrorMessage('KnowFrame: Background daemon is not connected.');
      return;
    }

    let targetFile: string | undefined = targetFileArg;

    if (!targetFile) {
      const activeEditor = vscode.window.activeTextEditor;
      const activeIsMd =
        activeEditor &&
        (activeEditor.document.languageId === 'markdown' ||
          activeEditor.document.uri.fsPath.endsWith('.md'));

      const picks: Array<vscode.QuickPickItem & { scope: 'current' | 'workspace' }> = [];
      if (activeIsMd && activeEditor) {
        const rel = path.relative(workspaceRoot, activeEditor.document.uri.fsPath).replace(/\\/g, '/');
        picks.push({
          label: '$(file-code) Current Markdown File',
          description: rel,
          detail: 'Convert valid wikilinks & non-portable paths in this file to universally clickable Markdown links',
          scope: 'current',
        });
      }
      picks.push({
        label: '$(root-folder) Entire Workspace',
        description: path.basename(workspaceRoot),
        detail: 'Convert valid non-clickable links across all Markdown documents in the workspace',
        scope: 'workspace',
      });

      const chosen = await vscode.window.showQuickPick(picks, {
        placeHolder: 'Choose scope for converting links to universally clickable standard Markdown',
      });
      if (!chosen) {
        return;
      }
      if (chosen.scope === 'current' && activeEditor) {
        targetFile = path.relative(workspaceRoot, activeEditor.document.uri.fsPath).replace(/\\/g, '/');
      }
    }

    try {
      const preview = await vscode.window.withProgress(
        {
          location: vscode.ProgressLocation.Notification,
          title: 'KnowFrame: Analyzing link clickability...',
          cancellable: false,
        },
        () =>
          client.sendRequest<WorkspaceLinkNormalizationReport>('workspace/normalizeLinks', {
            file: targetFile,
            apply: false,
          })
      );

      if (preview.total_rewrites === 0) {
        const msg =
          preview.total_broken_skipped > 0
            ? `KnowFrame: All valid links are already clickable (${preview.total_broken_skipped} broken link(s) skipped).`
            : 'KnowFrame: All valid links are already universally clickable!';
        vscode.window.showInformationMessage(msg);
        return;
      }

      const scopeDesc = targetFile || `${preview.documents.length} file(s)`;
      const choice = await vscode.window.showInformationMessage(
        `KnowFrame found ${preview.total_rewrites} valid non-clickable link(s) in ${scopeDesc} (${preview.total_broken_skipped} broken skipped). Convert to universally clickable standard Markdown links?`,
        { modal: true },
        'Apply Changes',
        'Preview Diff'
      );

      if (choice === 'Preview Diff') {
        const fullDiff = preview.documents
          .map((d) => d.unified_diff)
          .filter(Boolean)
          .join('\n');
        const diffDoc = await vscode.workspace.openTextDocument({
          content: fullDiff || '# No diff generated',
          language: 'diff',
        });
        await vscode.window.showTextDocument(diffDoc, { preview: true });

        const applyAfterPreview = await vscode.window.showInformationMessage(
          `Apply ${preview.total_rewrites} link conversion(s) across ${preview.documents.length} file(s)?`,
          'Apply Changes',
          'Cancel'
        );
        if (applyAfterPreview !== 'Apply Changes') {
          return;
        }
      } else if (choice !== 'Apply Changes') {
        return;
      }

      const applied = await client.sendRequest<WorkspaceLinkNormalizationReport>(
        'workspace/normalizeLinks',
        {
          file: targetFile,
          apply: true,
        }
      );

      vscode.window.showInformationMessage(
        `KnowFrame: Converted ${applied.total_rewrites} link(s) across ${applied.files_modified} file(s) to clickable standard Markdown!`
      );
      vscode.commands.executeCommand('llmWiki.rescanVault');
    } catch (err: any) {
      vscode.window.showErrorMessage(`KnowFrame: Failed to normalize links: ${err.message}`);
    }
  });
}
