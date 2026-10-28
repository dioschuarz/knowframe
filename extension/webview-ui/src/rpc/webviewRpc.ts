import type { WebviewInboundMessage, WebviewOutboundMessage, NavigationIntent } from '../../../src/types/protocol.ts';

declare function acquireVsCodeApi(): {
  postMessage: (msg: unknown) => void;
  setState: (state: unknown) => void;
  getState: () => unknown;
};

export class WebviewRpcBridge {
  private vscodeApi = typeof acquireVsCodeApi === 'function' ? acquireVsCodeApi() : null;
  private messageHandlers = new Map<string, Set<(payload: any) => void>>();

  constructor() {
    window.addEventListener('message', (event) => {
      const msg: WebviewOutboundMessage = event.data;
      if (!msg || !msg.type) return;

      const handlers = this.messageHandlers.get(msg.type);
      if (handlers) {
        for (const h of handlers) {
          h(msg.payload);
        }
      }
    });
  }

  public sendMessage(msg: WebviewInboundMessage): void {
    if (this.vscodeApi) {
      this.vscodeApi.postMessage(msg);
    } else {
      console.warn('[WebviewRpcBridge] vscodeApi not available in standalone browser mode');
    }
  }

  public send(type: WebviewInboundType, payload?: unknown): void {
    this.sendMessage({
      type,
      payload,
    });
  }

  public on(type: string, handler: (payload: any) => void): () => void {
    let set = this.messageHandlers.get(type);
    if (!set) {
      set = new Set();
      this.messageHandlers.set(type, set);
    }
    set.add(handler);

    return () => {
      set?.delete(handler);
    };
  }

  public openFile(filePath: string, line?: number): void {
    this.sendMessage({
      type: 'openFile',
      payload: { file_path: filePath, line },
    });
  }

  public setHub(hubId: string): void {
    this.sendMessage({
      type: 'setHub',
      payload: { hub_id: hubId },
    });
  }

  public rescanVault(): void {
    this.sendMessage({
      type: 'rescanVault',
    });
  }

  public fixBrokenLinks(): void {
    this.sendMessage({
      type: 'fixBrokenLinks',
    });
  }

  public fixBrokenLink(source: string, line?: number, target?: string): void {
    this.sendMessage({
      type: 'fixBrokenLink',
      payload: { source, line, target },
    });
  }

  public previewExclusion(pattern: string): void {
    this.sendMessage({
      type: 'previewExclusion',
      payload: { pattern },
    });
  }

  public excludePath(pattern: string, scope: 'scan' | 'graph' = 'graph'): void {
    this.sendMessage({
      type: 'excludePath',
      payload: { pattern, scope },
    });
  }

  public removeExclusion(pattern: string, scope: 'scan' | 'graph' = 'graph'): void {
    this.sendMessage({
      type: 'removeExclusion',
      payload: { pattern, scope },
    });
  }

  public requestExclusions(): void {
    this.sendMessage({
      type: 'requestExclusions',
    });
  }

  public navigateToDetail(intent: NavigationIntent): void {
    this.sendMessage({
      type: 'navigateToDetail',
      payload: intent,
    });
  }

  public updateColorGroups(colorGroups: any[]): void {
    this.sendMessage({
      type: 'updateColorGroups',
      payload: { color_groups: colorGroups },
    });
  }

  public getEvolutionState(): void {
    this.sendMessage({
      type: 'getEvolutionState',
    });
  }

  public setOrientation(orientation: string): void {
    this.sendMessage({
      type: 'setOrientation',
      payload: { orientation },
    });
  }

  public updateCustomConfig(tier1: any, tier2: any, constraints: any[]): void {
    this.sendMessage({
      type: 'updateCustomConfig',
      payload: { tier1, tier2, constraints },
    });
  }

  public validateConstraints(constraints: any[]): void {
    this.sendMessage({
      type: 'validateConstraints',
      payload: { constraints },
    });
  }

  public generateTransformationPlan(recommendationId: string): void {
    this.sendMessage({
      type: 'generateTransformationPlan',
      payload: { recommendationId },
    });
  }

  public getStrategySet(opportunityId: string): void {
    this.sendMessage({
      type: 'getStrategySet',
      payload: { opportunityId },
    });
  }

  public validateStrategySelection(selectedStrategies: string[]): void {
    this.sendMessage({
      type: 'validateStrategySelection',
      payload: { selectedStrategies },
    });
  }

  public calculatePlanValue(opportunityId: string, selectedStrategies: string[]): void {
    this.sendMessage({
      type: 'calculatePlanValue',
      payload: { opportunityId, selectedStrategies },
    });
  }

  public generateMultiStrategyPlan(opportunityId: string, selectedStrategies: string[]): void {
    this.sendMessage({
      type: 'generateTransformationPlan',
      payload: { opportunityId, selectedStrategies },
    });
  }

  public applyTransformationPlan(plan: { planId?: string; plan_id?: string; [key: string]: unknown } | string): void {
    const payload = typeof plan === 'string'
      ? { planId: plan }
      : { planId: plan.planId || plan.plan_id, plan };
    this.sendMessage({
      type: 'applyTransformationPlan',
      payload,
    });
  }

  public getHarnessStatus(): void {
    this.sendMessage({
      type: 'getHarnessStatus',
    });
  }

  public setActiveHarness(harnessFile: string): void {
    this.sendMessage({
      type: 'setActiveHarness',
      payload: { harness_file: harnessFile },
    });
  }

  public initializeHarness(targetHarness: string = 'AGENTS.md'): void {
    this.sendMessage({
      type: 'initializeHarness',
      payload: { targetHarness },
    });
  }

  public syncHarnesses(): void {
    this.sendMessage({
      type: 'syncHarnesses',
      payload: {},
    });
  }

  public configureHarnessBase(options: {
    active_harness?: string;
    enabled_harnesses: string[];
    create_folders?: string[];
    consolidate_index?: boolean;
    clean_unused?: boolean;
  }): void {
    this.sendMessage({
      type: 'configureHarnessBase',
      payload: options,
    });
  }

  public getSkillCatalog(): void {
    this.sendMessage({
      type: 'getSkillCatalog',
    });
  }

  public analyzeDirectoryStructure(): void {
    this.sendMessage({
      type: 'analyzeDirectoryStructure',
    });
  }

  public generateGovernancePlan(strategyType: string, targetFileOrDir: string): void {
    this.sendMessage({
      type: 'generateGovernancePlan',
      payload: { strategy_type: strategyType, target_file_or_dir: targetFileOrDir },
    });
  }

  public applyGovernancePlan(planId: string, approvalToken: string): void {
    this.sendMessage({
      type: 'applyGovernancePlan',
      payload: { plan_id: planId, approval_token: approvalToken },
    });
  }

  public resolveDocumentReview(docIds: string[], reason?: string): void {
    this.sendMessage({
      type: 'resolveDocumentReview',
      payload: { doc_ids: docIds, reason },
    });
  }

  public reviewWithAi(filePath?: string, issue?: unknown): void {
    this.sendMessage({
      type: 'reviewWithAi',
      payload: { filePath, issue },
    });
  }

  public reviewAllWithAi(): void {
    this.sendMessage({
      type: 'reviewAllWithAi',
      payload: {},
    });
  }

  public getAffectedDocuments(options?: { base_ref?: string; head_ref?: string; staged?: boolean }): void {
    this.sendMessage({
      type: 'getAffectedDocuments',
      payload: options || {},
    });
  }

  public requestHarnessTools(): void {
    this.sendMessage({
      type: 'requestHarnessTools',
    });
  }

  public installHarnessTool(toolId: string): void {
    this.sendMessage({
      type: 'installHarnessTool',
      payload: { toolId },
    });
  }

  public setState(state: any): void {
    this.vscodeApi?.setState(state);
  }

  public getState<T>(): T | null {
    return (this.vscodeApi?.getState() as T) || null;
  }
}
