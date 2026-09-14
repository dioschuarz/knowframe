import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { generateNonce, buildCspPolicy } from './cspHelper';
import { IpcClient } from '../ipc/client';
import { ScanResultPayload, WebviewInboundMessage } from '../types/protocol';
import type { EvolutionManager } from '../evolution/evolutionManager';

export class GraphPanelManager implements vscode.Disposable {
  public static currentPanel: GraphPanelManager | undefined;
  public static readonly viewType = 'llmWiki.knowledgeGraph';

  private readonly panel: vscode.WebviewPanel;
  private readonly extensionUri: vscode.Uri;
  private disposables: vscode.Disposable[] = [];
  private ipcClient: IpcClient | null = null;
  private evolutionManager: EvolutionManager | null = null;
  private currentScanResult: ScanResultPayload | null = null;
  private static cachedIssuesReport: unknown | null = null;
  private isFetchingIssues: boolean = false;
  private pendingOpenTab: string | null = null;

  public static setCachedIssuesReport(report: unknown): void {
    GraphPanelManager.cachedIssuesReport = report;
  }

  public static getCachedIssuesReport(): unknown | null {
    return GraphPanelManager.cachedIssuesReport;
  }

  public static createOrShow(
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null,
    scanResult: ScanResultPayload | null
  ): GraphPanelManager {
    const column = vscode.window.activeTextEditor
      ? vscode.window.activeTextEditor.viewColumn
      : undefined;

    if (GraphPanelManager.currentPanel) {
      GraphPanelManager.currentPanel.panel.reveal(column);
      if (scanResult && !GraphPanelManager.currentPanel.currentScanResult) {
        GraphPanelManager.currentPanel.updateData(scanResult);
      }
      return GraphPanelManager.currentPanel;
    }

    const panel = vscode.window.createWebviewPanel(
      GraphPanelManager.viewType,
      'LLM Wiki - Graph',
      column || vscode.ViewColumn.One,
      {
        enableScripts: true,
        retainContextWhenHidden: true, // Keep Webview state and canvas simulation intact when navigating away
        localResourceRoots: [
          vscode.Uri.joinPath(extensionUri, 'dist'),
          vscode.Uri.joinPath(extensionUri, 'media'),
        ],
      }
    );

    GraphPanelManager.currentPanel = new GraphPanelManager(panel, extensionUri, ipcClient, scanResult);
    return GraphPanelManager.currentPanel;
  }

  public static revive(
    panel: vscode.WebviewPanel,
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null
  ): GraphPanelManager {
    panel.title = 'LLM Wiki - Graph';
    GraphPanelManager.currentPanel = new GraphPanelManager(panel, extensionUri, ipcClient, null);
    return GraphPanelManager.currentPanel;
  }

  private constructor(
    panel: vscode.WebviewPanel,
    extensionUri: vscode.Uri,
    ipcClient: IpcClient | null,
    scanResult: ScanResultPayload | null
  ) {
    this.panel = panel;
    this.extensionUri = extensionUri;
    this.ipcClient = ipcClient;
    this.currentScanResult = scanResult;

    this.panel.title = 'LLM Wiki - Graph';
    this.panel.iconPath = {
      light: vscode.Uri.joinPath(extensionUri, 'media', 'tab-icon-light.svg'),
      dark: vscode.Uri.joinPath(extensionUri, 'media', 'tab-icon-dark.svg'),
    };

    this.updateHtml();

    this.panel.onDidDispose(() => this.dispose(), null, this.disposables);

    this.panel.onDidChangeViewState(
      (e) => {
        if (e.webviewPanel.visible && this.currentScanResult) {
          this.updateData(this.currentScanResult);
        }
      },
      null,
      this.disposables
    );

    this.panel.webview.onDidReceiveMessage(
      (message: WebviewInboundMessage) => this.handleWebviewMessage(message),
      null,
      this.disposables
    );

    if (scanResult) {
      // Small delay to allow the webview script to mount event listeners
      setTimeout(() => {
        this.updateData(scanResult);
      }, 250);
    }
  }

  private currentLoadingProgress: { stage: string; percent: number; message: string } | null = null;

  public getCurrentScanResult(): ScanResultPayload | null {
    return this.currentScanResult;
  }

  public setIpcClient(client: IpcClient | null): void {
    this.ipcClient = client;
  }

  public sendLoadingProgress(stage: string, percent: number, message: string): void {
    this.currentLoadingProgress = { stage, percent, message };
    this.panel.webview.postMessage({
      type: 'loadingProgress',
      payload: this.currentLoadingProgress,
    });
  }

  public sendIssuesReport(issuesReport: unknown): void {
    GraphPanelManager.cachedIssuesReport = issuesReport;
    this.panel.webview.postMessage({
      type: 'issuesReportUpdate',
      payload: issuesReport,
    });
  }

  public updateData(scanResult: ScanResultPayload): void {
    this.currentScanResult = scanResult;
    this.currentLoadingProgress = { stage: 'complete', percent: 100, message: 'Ready' };
    this.panel.webview.postMessage({
      type: 'graphDataUpdate',
      payload: {
        nodes: scanResult.nodes.map((n) => ({
          ...n,
          hopDistance: n.hop_distance,
          inDegree: n.in_degree,
          outDegree: n.out_degree,
          brokenLinks: n.broken_links,
          filePath: n.file_path,
        })),
        edges: scanResult.edges.map((e) => ({
          ...e,
          isBroken: e.is_broken,
          linkType: e.link_type,
        })),
        tokenSummary: {
          activeHarnessTokens: scanResult.token_summary.active_harness_tokens,
          subcallAccessibleTokens: scanResult.token_summary.subcall_accessible_tokens,
          warningThreshold: scanResult.token_summary.warning_threshold,
          criticalThreshold: scanResult.token_summary.critical_threshold,
          status: scanResult.token_summary.status,
        },
        recommendedHub: scanResult.recommended_hub,
      },
    });
    this.panel.webview.postMessage({
      type: 'loadingProgress',
      payload: { stage: 'complete', percent: 100, message: 'Ready' },
    });
  }

  public async openDiagnosticsTab(): Promise<void> {
    this.panel.reveal();
    this.pendingOpenTab = 'diagnostics';
    this.panel.webview.postMessage({
      type: 'openSettingsDrawer',
      tab: 'diagnostics',
    });

    if (GraphPanelManager.cachedIssuesReport) {
      this.panel.webview.postMessage({
        type: 'issuesReportUpdate',
        payload: GraphPanelManager.cachedIssuesReport,
      });
    }

    if (this.isFetchingIssues) {
      return;
    }

    if (this.ipcClient) {
      try {
        this.isFetchingIssues = true;
        const issuesReport = await this.ipcClient.sendRequest('workspace/getIssues', {});
        GraphPanelManager.cachedIssuesReport = issuesReport;
        this.panel.webview.postMessage({
          type: 'issuesReportUpdate',
          payload: issuesReport,
        });
      } catch (err: any) {
        console.error('Failed to fetch issues report:', err);
      } finally {
        this.isFetchingIssues = false;
      }
    }
  }

  public setEvolutionManager(manager: EvolutionManager | null): void {
    this.evolutionManager = manager;
    if (manager) {
      manager.onDidChangeOrientation(() => {
        this.sendEvolutionState();
      });
      manager.onDidUpdateProblems(() => {
        this.sendEvolutionState();
      });
      manager.onDidUpdateRecommendations(() => {
        this.sendEvolutionState();
      });
    }
  }

  public async sendEvolutionState(): Promise<void> {
    if (!this.evolutionManager) {
      return;
    }
    try {
      const orientRes = await this.evolutionManager.getOrientation();
      let customRes: any = { tier1: null, tier2: null, constraints: [] };
      try {
        customRes = await this.evolutionManager.getCustomConfig();
      } catch {
        // Fallback if not custom
      }
      const rawSignatures = await this.evolutionManager.getStructuralProblems();
      const rawRecommendations = await this.evolutionManager.getRecommendations();

      // Normalize signatures for webview
      const signatures = (rawSignatures || []).map((s: any) => ({
        signatureId: s.signature_id || s.signatureId,
        signature_id: s.signature_id || s.signatureId,
        documentPath: s.document_path || s.documentPath,
        document_path: s.document_path || s.documentPath,
        problemType: s.problem_type || s.problemType,
        problem_type: s.problem_type || s.problemType,
        confidence: s.confidence,
        rationale: s.rationale,
        evidence: {
          cohesion: s.evidence?.cohesion ?? 0,
          coupling: s.evidence?.coupling ?? 0,
          referenceConcentration: s.evidence?.reference_concentration ?? s.evidence?.referenceConcentration ?? 0,
          reference_concentration: s.evidence?.reference_concentration ?? s.evidence?.referenceConcentration ?? 0,
          granularity: s.evidence?.granularity ?? 0,
          contextEfficiency: s.evidence?.context_efficiency ?? s.evidence?.contextEfficiency ?? 0,
          context_efficiency: s.evidence?.context_efficiency ?? s.evidence?.contextEfficiency ?? 0,
          discoverability: s.evidence?.discoverability ?? 0,
          reusability: s.evidence?.reusability ?? 0,
          semanticDiversity: s.evidence?.semantic_diversity ?? s.evidence?.semanticDiversity ?? 0,
          semantic_diversity: s.evidence?.semantic_diversity ?? s.evidence?.semanticDiversity ?? 0,
          navigationalComplexity: s.evidence?.navigational_complexity ?? s.evidence?.navigationalComplexity ?? 0,
          navigational_complexity: s.evidence?.navigational_complexity ?? s.evidence?.navigationalComplexity ?? 0,
          maintenanceCoupling: s.evidence?.maintenance_coupling ?? s.evidence?.maintenanceCoupling ?? 0,
          maintenance_coupling: s.evidence?.maintenance_coupling ?? s.evidence?.maintenanceCoupling ?? 0,
        },
      }));

      // Normalize recommendations for webview
      const recommendations = (rawRecommendations || []).map((r: any) => ({
        recommendationId: r.recommendation_id || r.recommendationId,
        recommendation_id: r.recommendation_id || r.recommendationId,
        signatureId: r.signature_id || r.signatureId,
        signature_id: r.signature_id || r.signatureId,
        strategy: r.strategy,
        confidence: r.confidence,
        expectedBenefits: r.expected_benefits || r.expectedBenefits || [],
        expected_benefits: r.expected_benefits || r.expectedBenefits || [],
        tradeOffs: r.trade_offs || r.tradeOffs || [],
        trade_offs: r.trade_offs || r.tradeOffs || [],
        transformationPlan: r.transformation_plan || r.transformationPlan,
      }));

      let tier1 = customRes?.tier1;
      if (tier1) {
        tier1 = {
          aiDiscoverability: tier1.ai_discoverability || tier1.aiDiscoverability,
          ai_discoverability: tier1.ai_discoverability || tier1.aiDiscoverability,
          contextEfficiency: tier1.context_efficiency || tier1.contextEfficiency,
          context_efficiency: tier1.context_efficiency || tier1.contextEfficiency,
          humanReadability: tier1.human_readability || tier1.humanReadability,
          human_readability: tier1.human_readability || tier1.humanReadability,
          knowledgeReuse: tier1.knowledge_reuse || tier1.knowledgeReuse,
          knowledge_reuse: tier1.knowledge_reuse || tier1.knowledgeReuse,
          narrativeCoherence: tier1.narrative_coherence || tier1.narrativeCoherence,
          narrative_coherence: tier1.narrative_coherence || tier1.narrativeCoherence,
          maintenanceSimplicity: tier1.maintenance_simplicity || tier1.maintenanceSimplicity,
          maintenance_simplicity: tier1.maintenance_simplicity || tier1.maintenanceSimplicity,
        };
      }

      let tier2 = customRes?.tier2;
      if (tier2) {
        tier2 = {
          cohesionThreshold: tier2.cohesion_threshold ?? tier2.cohesionThreshold,
          cohesion_threshold: tier2.cohesion_threshold ?? tier2.cohesionThreshold,
          semanticDiversityThreshold: tier2.semantic_diversity_threshold ?? tier2.semanticDiversityThreshold,
          semantic_diversity_threshold: tier2.semantic_diversity_threshold ?? tier2.semanticDiversityThreshold,
          referenceConcentrationThreshold: tier2.reference_concentration_threshold ?? tier2.referenceConcentrationThreshold,
          reference_concentration_threshold: tier2.reference_concentration_threshold ?? tier2.referenceConcentrationThreshold,
          fragmentSizeTarget: tier2.fragment_size_target ?? tier2.fragmentSizeTarget,
          fragment_size_target: tier2.fragment_size_target ?? tier2.fragmentSizeTarget,
          maxTraceDepth: tier2.max_trace_depth ?? tier2.maxTraceDepth,
          max_trace_depth: tier2.max_trace_depth ?? tier2.maxTraceDepth,
          candidateExpansionFactor: tier2.candidate_expansion_factor ?? tier2.candidateExpansionFactor,
          candidate_expansion_factor: tier2.candidate_expansion_factor ?? tier2.candidateExpansionFactor,
          maxContextSurfaceTokens: tier2.max_context_surface_tokens ?? tier2.maxContextSurfaceTokens,
          max_context_surface_tokens: tier2.max_context_surface_tokens ?? tier2.maxContextSurfaceTokens,
          splitSensitivity: tier2.split_sensitivity ?? tier2.splitSensitivity,
          split_sensitivity: tier2.split_sensitivity ?? tier2.splitSensitivity,
          canonicalOwnershipSensitivity: tier2.canonical_ownership_sensitivity ?? tier2.canonicalOwnershipSensitivity,
          canonical_ownership_sensitivity: tier2.canonical_ownership_sensitivity ?? tier2.canonicalOwnershipSensitivity,
          duplicateTolerance: tier2.duplicate_tolerance ?? tier2.duplicateTolerance,
          duplicate_tolerance: tier2.duplicate_tolerance ?? tier2.duplicateTolerance,
          fragmentationTolerance: tier2.fragmentation_tolerance ?? tier2.fragmentationTolerance,
          fragmentation_tolerance: tier2.fragmentation_tolerance ?? tier2.fragmentationTolerance,
        };
      }

      this.panel.webview.postMessage({
        type: 'evolutionStateUpdate',
        payload: {
          orientation: orientRes.orientation,
          profile: orientRes.profile,
          tier1,
          tier2,
          constraints: customRes.constraints,
          signatures,
          recommendations,
        },
      });
    } catch (err: any) {
      console.error('[GraphPanelManager] Error sending evolution state:', err);
    }
  }

  public async openEvolutionTab(): Promise<void> {
    this.panel.reveal();
    this.pendingOpenTab = 'evolution';
    this.panel.webview.postMessage({
      type: 'openSettingsDrawer',
      tab: 'evolution',
    });

    if (GraphPanelManager.cachedIssuesReport) {
      this.panel.webview.postMessage({
        type: 'issuesReportUpdate',
        payload: GraphPanelManager.cachedIssuesReport,
      });
    }

    if (this.ipcClient && !GraphPanelManager.cachedIssuesReport) {
      try {
        const issuesReport = await this.ipcClient.sendRequest('workspace/getIssues', {});
        GraphPanelManager.cachedIssuesReport = issuesReport;
        this.panel.webview.postMessage({
          type: 'issuesReportUpdate',
          payload: issuesReport,
        });
      } catch {
        // ignore
      }
    }

    await this.sendEvolutionState();
  }

  private async handleWebviewMessage(message: WebviewInboundMessage): Promise<void> {
    if (!message || !message.type) {return;}

    switch (message.type) {
      case 'openFile': {
        const payload = message.payload as { file_path: string; line?: number };
        if (payload?.file_path) {
          await this.openDocument(payload.file_path, payload.line);
        }
        break;
      }
      case 'setHub': {
        const payload = message.payload as { hub_id: string };
        if (payload?.hub_id && this.ipcClient) {
          try {
            await this.ipcClient.sendRequest('graph/setCentralHub', { hubPath: payload.hub_id });
          } catch {
            await this.ipcClient.sendRequest('graph/setHub', { hub_id: payload.hub_id });
          }
          vscode.commands.executeCommand('llmWiki.rescanVault');
        }
        break;
      }
      case 'fixBrokenLink': {
        const payload = message.payload as { source?: string; filePath?: string; line?: number; target: string };
        const source = payload?.source || payload?.filePath;
        if (source && payload.target && this.currentScanResult) {
          const notes = this.currentScanResult.nodes.map((n) => n.file_path || n.id);
          const picked = await vscode.window.showQuickPick(notes, {
            placeHolder: `Select replacement target for "${payload.target}" in ${source}`,
          });
          if (picked) {
            await this.replaceLinkInFile(source, payload.line, payload.target, picked);
            vscode.window.showInformationMessage(`Updated link in ${source} to [[${picked}]]`);
            vscode.commands.executeCommand('llmWiki.rescanVault');
          }
        }
        break;
      }
      case 'previewExclusion': {
        const payload = message.payload as { pattern: string };
        if (payload?.pattern && this.ipcClient) {
          try {
            const res = await this.ipcClient.sendRequest('graph/previewExclusion', { pattern: payload.pattern });
            this.panel.webview.postMessage({
              type: 'exclusionPreviewResult',
              payload: res,
            });
          } catch (err: any) {
            vscode.window.showErrorMessage(`Exclusion preview failed: ${err.message}`);
          }
        }
        break;
      }
      case 'excludePath': {
        const payload = message.payload as { pattern: string; scope: 'scan' | 'graph' };
        if (payload?.pattern && this.ipcClient) {
          try {
            await this.ipcClient.sendRequest('graph/excludePath', {
              pattern: payload.pattern,
              scope: payload.scope || 'graph',
            });
            vscode.window.showInformationMessage(`Excluded pattern: ${payload.pattern}`);
            vscode.commands.executeCommand('llmWiki.rescanVault');
          } catch (err: any) {
            vscode.window.showErrorMessage(`Exclusion failed: ${err.message}`);
          }
        }
        break;
      }
      case 'suggestExclusions': {
        if (this.ipcClient) {
          try {
            const res = await this.ipcClient.sendRequest('graph/suggestExclusions', {});
            this.panel.webview.postMessage({
              type: 'noiseSuggestions',
              payload: res,
            });
          } catch (err: any) {
            console.error('Suggest exclusions error:', err);
          }
        }
        break;
      }
      case 'rescanVault': {
        vscode.commands.executeCommand('llmWiki.rescanVault');
        break;
      }
      case 'fixBrokenLinks': {
        vscode.commands.executeCommand('llmWiki.fixBrokenLinks');
        break;
      }
      case 'updateColorGroups': {
        const payload = message.payload as { color_groups: any[] };
        if (payload?.color_groups && this.ipcClient) {
          vscode.window.showInformationMessage(`Updated ${payload.color_groups.length} cluster color rules.`);
        }
        break;
      }
      case 'requestGraphData': {
        if (this.currentScanResult) {
          this.updateData(this.currentScanResult);
        } else if (this.currentLoadingProgress) {
          this.panel.webview.postMessage({
            type: 'loadingProgress',
            payload: this.currentLoadingProgress,
          });
        } else {
          this.sendLoadingProgress('graph', 20, 'Scanning vault topology...');
        }

        if (this.pendingOpenTab) {
          const tabToOpen = this.pendingOpenTab;
          this.pendingOpenTab = null;
          this.panel.webview.postMessage({
            type: 'openSettingsDrawer',
            tab: tabToOpen,
          });
          if (GraphPanelManager.cachedIssuesReport) {
            this.panel.webview.postMessage({
              type: 'issuesReportUpdate',
              payload: GraphPanelManager.cachedIssuesReport,
            });
          }
        }
        break;
      }
      case 'triggerSetHubQuickPick': {
        await vscode.commands.executeCommand('llmWiki.setHub');
        break;
      }
      case 'requestIssues': {
        if (GraphPanelManager.cachedIssuesReport) {
          this.panel.webview.postMessage({
            type: 'issuesReportUpdate',
            payload: GraphPanelManager.cachedIssuesReport,
          });
        }
        if (this.isFetchingIssues) {
          break;
        }
        if (this.ipcClient) {
          try {
            this.isFetchingIssues = true;
            const payload = (message.payload || {}) as { pillar?: string; limit?: number };
            const issuesReport = await this.ipcClient.sendRequest('workspace/getIssues', payload);
            GraphPanelManager.cachedIssuesReport = issuesReport;
            this.panel.webview.postMessage({
              type: 'issuesReportUpdate',
              payload: issuesReport,
            });
          } catch (err: any) {
            console.error('Failed to get issues report:', err);
          } finally {
            this.isFetchingIssues = false;
          }
        }
        break;
      }
      case 'getEvolutionState': {
        await this.sendEvolutionState();
        break;
      }
      case 'setOrientation': {
        const payload = message.payload as { orientation: any };
        if (this.evolutionManager && payload?.orientation) {
          try {
            await this.evolutionManager.setOrientation(payload.orientation);
            vscode.window.showInformationMessage(
              `Documentation orientation set to: ${payload.orientation.replace(/_/g, ' ')}`
            );
            await this.sendEvolutionState();
          } catch (err: any) {
            vscode.window.showErrorMessage(`Failed to set orientation: ${err.message}`);
          }
        }
        break;
      }
      case 'updateCustomConfig': {
        const payload = message.payload as { tier1: any; tier2: any; constraints: any[] };
        if (this.evolutionManager && payload?.tier1 && payload?.tier2) {
          try {
            await this.evolutionManager.updateCustomConfig(
              payload.tier1,
              payload.tier2,
              payload.constraints || []
            );
            vscode.window.showInformationMessage('Custom KOS profile updated and compiled successfully.');
            await this.sendEvolutionState();
          } catch (err: any) {
            vscode.window.showErrorMessage(`Failed to update custom configuration: ${err.message}`);
          }
        }
        break;
      }
      case 'validateConstraints': {
        const payload = message.payload as { constraints: any[] };
        if (this.evolutionManager && payload?.constraints) {
          try {
            const valRes = await this.evolutionManager.validateConstraints(payload.constraints);
            this.panel.webview.postMessage({
              type: 'constraintValidationResult',
              payload: valRes,
            });
          } catch (err: any) {
            console.error('Failed to validate constraints:', err);
          }
        }
        break;
      }
      case 'generateTransformationPlan': {
        const payload = message.payload as { recommendationId: string };
        if (this.evolutionManager && payload?.recommendationId) {
          try {
            const plan = await this.evolutionManager.generatePlan(payload.recommendationId);
            this.panel.webview.postMessage({
              type: 'transformationPlanReady',
              payload: plan,
            });
          } catch (err: any) {
            vscode.window.showErrorMessage(`Failed to generate transformation plan: ${err.message}`);
          }
        }
        break;
      }
      case 'applyTransformationPlan': {
        const payload = message.payload as { planId: string };
        if (this.evolutionManager && payload?.planId) {
          try {
            const rootPath = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
            const res = await this.evolutionManager.applyPlan(payload.planId, { rootPath });
            vscode.window.showInformationMessage(
              `Transformation plan applied: efficiency delta ${res.actualDelta.efficiencyDelta >= 0 ? '+' : ''}${res.actualDelta.efficiencyDelta.toFixed(1)}%`
            );
            this.panel.webview.postMessage({
              type: 'transformationPlanApplied',
              payload: res,
            });
            vscode.commands.executeCommand('llmWiki.rescanVault');
            await this.sendEvolutionState();
          } catch (err: any) {
            vscode.window.showErrorMessage(`Failed to apply transformation plan: ${err.message}`);
          }
        }
        break;
      }
    }
  }

  private async replaceLinkInFile(sourcePath: string, _line: number | undefined, oldTarget: string, newTarget: string): Promise<void> {
    try {
      let uri: vscode.Uri;
      if (path.isAbsolute(sourcePath)) {
        uri = vscode.Uri.file(sourcePath);
      } else {
        const root = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!root) {return;}
        let candidateUri = vscode.Uri.joinPath(root, sourcePath);
        try {
          await vscode.workspace.fs.stat(candidateUri);
          uri = candidateUri;
        } catch {
          const notesUri = vscode.Uri.joinPath(root, 'notes', sourcePath);
          try {
            await vscode.workspace.fs.stat(notesUri);
            uri = notesUri;
          } catch {
            uri = candidateUri;
          }
        }
      }
      const doc = await vscode.workspace.openTextDocument(uri);
      const text = doc.getText();
      const targetClean = newTarget.replace(/\.md$/, '');
      const newText = text.replace(oldTarget, targetClean);
      const edit = new vscode.WorkspaceEdit();
      const fullRange = new vscode.Range(doc.positionAt(0), doc.positionAt(text.length));
      edit.replace(uri, fullRange, newText);
      await vscode.workspace.applyEdit(edit);
      await doc.save();
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to fix reference: ${err.message}`);
    }
  }

  private async openDocument(filePath: string, line?: number): Promise<void> {
    try {
      if (!filePath) { return; }
      if (typeof filePath !== 'string') {
        filePath = (filePath as any).filePath || (filePath as any).id || String(filePath);
      }
      let uri: vscode.Uri;
      if (path.isAbsolute(filePath)) {
        uri = vscode.Uri.file(filePath);
      } else {
        const root = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!root) {return;}
        let candidateUri = vscode.Uri.joinPath(root, filePath);
        try {
          await vscode.workspace.fs.stat(candidateUri);
          uri = candidateUri;
        } catch {
          const notesUri = vscode.Uri.joinPath(root, 'notes', filePath);
          try {
            await vscode.workspace.fs.stat(notesUri);
            uri = notesUri;
          } catch {
            uri = candidateUri;
          }
        }
      }

      const doc = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.Beside,
        preview: true,
      });

      if (line && line > 0) {
        const pos = new vscode.Position(line - 1, 0);
        editor.selection = new vscode.Selection(pos, pos);
        editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
      }
    } catch (err: any) {
      vscode.window.showErrorMessage(`Failed to open note: ${err.message}`);
    }
  }

  private updateHtml(): void {
    const webview = this.panel.webview;
    const nonce = generateNonce();
    const cspPolicy = buildCspPolicy(nonce, webview.cspSource);

    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this.extensionUri, 'dist', 'webview.js')
    );
    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this.extensionUri, 'dist', 'theme.css')
    );

    const templatePath = path.join(this.extensionUri.fsPath, 'dist', 'index.html');
    let html = '';

    if (fs.existsSync(templatePath)) {
      html = fs.readFileSync(templatePath, 'utf8');
      html = html
        .replace('{{CSP_POLICY}}', cspPolicy)
        .replace('{{NONCE}}', nonce)
        .replace('{{STYLE_URI}}', styleUri.toString())
        .replace('{{SCRIPT_URI}}', scriptUri.toString());
    } else {
      // Fallback in case template file isn't ready
      html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="${cspPolicy}">
  <title>LLM Wiki - Graph</title>
  <link rel="stylesheet" href="${styleUri}">
</head>
<body>
  <div id="toolbar-container"></div>
  <div id="token-container"></div>
  <canvas id="graph-canvas"></canvas>
  <div id="graph-loading-overlay" class="graph-loading-overlay">
    <div class="loading-card">
      <div class="loading-spinner"></div>
      <div class="loading-title">LLM Wiki Engine</div>
      <div class="loading-percent" id="loading-percent">0%</div>
      <div class="loading-stage" id="loading-stage">Initializing knowledge engine...</div>
      <div class="loading-bar-track">
        <div class="loading-bar-fill" id="loading-bar-fill" style="width: 0%"></div>
      </div>
    </div>
  </div>
  <div id="node-tooltip"></div>
  <div id="color-modal-container"></div>
  <div id="inspector-container"></div>
  <div id="settings-container"></div>
  <div id="exclusion-modal-container"></div>
  <div id="transformation-modal-container"></div>
  <script nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
    }

    this.panel.webview.html = html;
  }

  public dispose(): void {
    GraphPanelManager.currentPanel = undefined;
    this.panel.dispose();
    while (this.disposables.length) {
      const d = this.disposables.pop();
      if (d) {
        d.dispose();
      }
    }
  }
}

/**
 * Serializer to revive Webview when VS Code restarts
 */
export class GraphPanelSerializer implements vscode.WebviewPanelSerializer {
  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly getIpcClient: () => IpcClient | null
  ) {}

  async deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, _state: unknown): Promise<void> {
    webviewPanel.webview.options = {
      enableScripts: true,
      localResourceRoots: [vscode.Uri.joinPath(this.extensionUri, 'dist')],
    };
    GraphPanelManager.revive(webviewPanel, this.extensionUri, this.getIpcClient());
  }
}
