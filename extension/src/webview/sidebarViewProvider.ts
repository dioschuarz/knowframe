import * as vscode from 'vscode';
import * as path from 'path';
import { generateNonce, buildCspPolicy } from './cspHelper';
import {
  ScanResultPayload,
  ManagerSummaryState,
  DocumentationOrientation,
} from '../types/evolution';
import { KnowledgeQualityScore, KnowledgeIssuesReport } from '../types/admission';

import { renderSidebarHtml, type SidebarRenderOptions } from './sidebarRenderer.js';
export { renderSidebarHtml, type SidebarRenderOptions };

export class SidebarViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = 'llmWiki.sidebarView';
  private _view?: vscode.WebviewView;
  private currentScanResult: ScanResultPayload | null = null;
  private currentQualityScore: KnowledgeQualityScore | null = null;
  private currentIssuesReport: KnowledgeIssuesReport | null = null;
  private currentOrientation: DocumentationOrientation = 'ai_oriented';
  private currentSummary: ManagerSummaryState | null = null;

  private readonly extensionUri: vscode.Uri;
  private readonly getScanResult: () => ScanResultPayload | null;
  private readonly getQualityScore?: () => KnowledgeQualityScore | null;
  private readonly getManagerSummary?: () => ManagerSummaryState | null;
  private readonly getIssuesReport?: () => KnowledgeIssuesReport | null;

  constructor(
    extensionUri: vscode.Uri,
    getScanResult: () => ScanResultPayload | null,
    getQualityScore?: () => KnowledgeQualityScore | null,
    getManagerSummary?: () => ManagerSummaryState | null,
    getIssuesReport?: () => KnowledgeIssuesReport | null
  ) {
    this.extensionUri = extensionUri;
    this.getScanResult = getScanResult;
    this.getQualityScore = getQualityScore;
    this.getManagerSummary = getManagerSummary;
    this.getIssuesReport = getIssuesReport;
  }

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken
  ): void {
    this._view = webviewView;
    webviewView.title = '';

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.extensionUri],
    };

    this.currentScanResult = this.getScanResult();
    this.currentQualityScore = this.getQualityScore ? this.getQualityScore() : null;
    this.currentSummary = this.getManagerSummary ? this.getManagerSummary() : null;
    this.currentIssuesReport = this.getIssuesReport ? this.getIssuesReport() : null;
    this.updateHtml();

    webviewView.webview.onDidReceiveMessage(async (data) => {
      switch (data.type) {
        case 'openGraph':
          vscode.commands.executeCommand('llmWiki.showKnowledgeGraph');
          break;
        case 'openFolder':
          vscode.commands.executeCommand('vscode.openFolder');
          break;
        case 'changeHub':
        case 'setHub':
          vscode.commands.executeCommand('llmWiki.setHub');
          break;
        case 'rescanVault':
          vscode.commands.executeCommand('llmWiki.rescanVault');
          break;
        case 'refreshQuality':
          vscode.commands.executeCommand('llmWiki.rescanVault');
          break;
        case 'openSettings':
          vscode.commands.executeCommand('llmWiki.openSettings', data.tab);
          break;
        case 'openEvolution':
        case 'reviewEvolution':
          vscode.commands.executeCommand('llmWiki.reviewEvolution');
          break;
        case 'navigateToDetail':
          vscode.commands.executeCommand('llmWiki.navigateToDetail', data);
          break;
        case 'openFile':
          if (data.filePath) {
            this.openDocument(data.filePath);
          }
          break;
        case 'openHub':
          if (data.hubPath) {
            this.openDocument(data.hubPath);
          }
          break;
        case 'resolveDocumentReview':
          vscode.commands.executeCommand('llmWiki.resolveDocumentReview', data);
          break;
        case 'reviewWithAi':
          vscode.commands.executeCommand('llmWiki.reviewWithAi', data?.payload || data);
          break;
        case 'reviewAllWithAi':
          vscode.commands.executeCommand('llmWiki.reviewAllWithAi');
          break;
      }
    });

    if (this.currentSummary) {
      this.updateSummary(this.currentSummary);
    } else if (this.currentScanResult) {
      this.updateData(this.currentScanResult, this.currentQualityScore);
    }
    if (this.currentIssuesReport) {
      this.updateIssues(this.currentIssuesReport);
    }
  }

  public updateSummary(summary: ManagerSummaryState): void {
    this.currentSummary = summary;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'summaryUpdate',
        payload: summary,
      });
    }
  }

  public updateOrientation(orientation: DocumentationOrientation): void {
    this.currentOrientation = orientation;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'orientationUpdate',
        payload: orientation,
      });
    }
  }

  public updateData(scanResult: ScanResultPayload, qualityScore?: KnowledgeQualityScore | null): void {
    this.currentScanResult = scanResult;
    if (qualityScore !== undefined) {
      this.currentQualityScore = qualityScore;
    }
    if (this._view) {
      this._view.webview.postMessage({
        type: 'graphDataUpdate',
        payload: scanResult,
        qualityScore: this.currentQualityScore,
      });
    }
  }

  public updateQuality(qualityScore: KnowledgeQualityScore): void {
    this.currentQualityScore = qualityScore;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'qualityUpdate',
        payload: qualityScore,
      });
    }
  }

  public updateIssues(issuesReport: KnowledgeIssuesReport): void {
    this.currentIssuesReport = issuesReport;
    if (this._view) {
      this._view.webview.postMessage({
        type: 'issuesUpdate',
        payload: issuesReport,
      });
    }
  }

  private async openDocument(filePath: string): Promise<void> {
    try {
      let uri: vscode.Uri;
      if (path.isAbsolute(filePath)) {
        uri = vscode.Uri.file(filePath);
      } else {
        const root = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!root) {
          return;
        }
        uri = vscode.Uri.joinPath(root, filePath);
      }
      const doc = await vscode.workspace.openTextDocument(uri);
      await vscode.window.showTextDocument(doc, { preview: true });
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to open note: ${err.message}`);
    }
  }

  private updateHtml(): void {
    if (!this._view) {
      return;
    }
    const webview = this._view.webview;
    const nonce = generateNonce();
    const cspPolicy = buildCspPolicy(nonce, webview.cspSource);

    this._view.webview.html = renderSidebarHtml({
      cspPolicy,
      nonce,
      initialData: this.currentScanResult,
      initialQuality: this.currentQualityScore,
      initialSummary: this.currentSummary,
      initialIssues: this.currentIssuesReport,
      orientation: this.currentOrientation,
    });
  }
}
