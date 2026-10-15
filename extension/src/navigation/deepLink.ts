/**
 * Bidirectional editor deep-linking navigation.
 * Implements FR-044: opens document jumped to exact line and column.
 */

import * as vscode from 'vscode';
import * as path from 'path';

export async function openDocumentAtPosition(
  filePath: string,
  line: number = 1,
  column: number = 1,
  workspaceRoot?: string,
): Promise<vscode.TextEditor | undefined> {
  let resolvedPath = filePath;
  if (!path.isAbsolute(resolvedPath) && workspaceRoot) {
    resolvedPath = path.resolve(workspaceRoot, filePath);
  }

  const uri = vscode.Uri.file(resolvedPath);
  try {
    const doc = await vscode.workspace.openTextDocument(uri);
    const zeroBasedLine = Math.max(0, line - 1);
    const zeroBasedCol = Math.max(0, column - 1);
    const pos = new vscode.Position(zeroBasedLine, zeroBasedCol);
    const range = new vscode.Range(pos, pos);

    return await vscode.window.showTextDocument(doc, {
      selection: range,
      viewColumn: vscode.ViewColumn.Beside,
    });
  } catch (err) {
    console.error(`Failed to open document at ${resolvedPath}:`, err);
    return undefined;
  }
}
