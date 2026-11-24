import * as vscode from 'vscode';
import { ScanResultPayload } from '../types/protocol';
import { extractWikilinksInMarkdown, resolveWikilinkTarget } from './markdownLinkResolver';

/**
 * VS Code DocumentLinkProvider for Markdown files.
 * Makes every valid `[[target#fragment|label]]` Obsidian wikilink immediately Ctrl+Click / Cmd+Click
 * navigable directly inside the VS Code Markdown editor, jumping to the exact target file and heading line.
 */
export class KnowFrameMarkdownDocumentLinkProvider implements vscode.DocumentLinkProvider {
  constructor(private readonly getScanResult: () => ScanResultPayload | null) {}

  public provideDocumentLinks(
    document: vscode.TextDocument,
    _token: vscode.CancellationToken
  ): vscode.DocumentLink[] {
    const config = vscode.workspace.getConfiguration('llmWiki');
    if (config.get<boolean>('enableEditorWikilinkNavigation', true) === false) {
      return [];
    }

    const workspaceFolder =
      vscode.workspace.getWorkspaceFolder(document.uri) ||
      vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
      return [];
    }

    const workspaceRoot = workspaceFolder.uri.fsPath;
    const sourcePath = document.uri.fsPath;
    const content = document.getText();
    const scanResult = this.getScanResult();

    const wikilinks = extractWikilinksInMarkdown(content);
    const links: vscode.DocumentLink[] = [];

    for (const wl of wikilinks) {
      if (wl.isEmbed) {
        continue;
      }

      const resolved = resolveWikilinkTarget(
        workspaceRoot,
        sourcePath,
        wl.rawTarget,
        wl.anchor,
        scanResult
      );
      if (!resolved) {
        continue;
      }

      const startPos = document.positionAt(wl.startOffset);
      const endPos = document.positionAt(wl.endOffset);
      const range = new vscode.Range(startPos, endPos);

      let targetUri = vscode.Uri.file(resolved.absolutePath);
      if (resolved.targetLine && resolved.targetLine > 0) {
        targetUri = targetUri.with({ fragment: `L${resolved.targetLine}` });
      }

      const docLink = new vscode.DocumentLink(range, targetUri);
      docLink.tooltip = resolved.fragmentSlug
        ? `KnowFrame: Open ${resolved.canonicalId}#${resolved.fragmentSlug}`
        : `KnowFrame: Open ${resolved.canonicalId}`;
      links.push(docLink);
    }

    return links;
  }
}
