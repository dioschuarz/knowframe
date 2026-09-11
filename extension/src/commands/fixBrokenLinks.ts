import * as vscode from 'vscode';
import * as path from 'path';
import { ScanResultPayload } from '../types/protocol';

export function registerFixBrokenLinksCommand(
  getScanResult: () => ScanResultPayload | null
): vscode.Disposable {
  return vscode.commands.registerCommand('llmWiki.fixBrokenLinks', async () => {
    const scan = getScanResult();
    if (!scan) {
      vscode.window.showInformationMessage('LLM Wiki: No graph data available.');
      return;
    }

    const brokenEdges = scan.edges.filter((e) => e.is_broken);
    if (brokenEdges.length === 0) {
      vscode.window.showInformationMessage('LLM Wiki: No broken links found in workspace!');
      return;
    }

    const items = brokenEdges.map((e) => ({
      label: `$(alert) ${e.source} -> ${e.target}`,
      description: e.line ? `Line ${e.line}` : 'Unknown line',
      detail: e.anchor ? `Target anchor: #${e.anchor}` : undefined,
      edge: e,
    }));

    const selected = await vscode.window.showQuickPick(items, {
      placeHolder: `Found ${brokenEdges.length} broken links. Select to navigate to location in editor:`,
    });

    if (selected) {
      const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri;
      if (!workspaceRoot) {return;}

      const sourceFile = selected.edge.source;
      const fileUri = path.isAbsolute(sourceFile)
        ? vscode.Uri.file(sourceFile)
        : vscode.Uri.joinPath(workspaceRoot, sourceFile);

      try {
        const doc = await vscode.workspace.openTextDocument(fileUri);
        const editor = await vscode.window.showTextDocument(doc, { preview: true });

        if (selected.edge.line && selected.edge.line > 0) {
          const lineIndex = Math.max(0, selected.edge.line - 1);
          const pos = new vscode.Position(lineIndex, 0);
          editor.selection = new vscode.Selection(pos, pos);
          editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
        }
      } catch (err: any) {
        vscode.window.showErrorMessage(`Failed to open document: ${err.message}`);
      }
    }
  });
}
