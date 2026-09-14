import { WebviewInboundMessage, WebviewOutboundMessage } from '../../../src/types/protocol';

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

  public applyTransformationPlan(planId: string): void {
    this.sendMessage({
      type: 'applyTransformationPlan',
      payload: { planId },
    });
  }

  public setState(state: any): void {
    this.vscodeApi?.setState(state);
  }

  public getState<T>(): T | null {
    return (this.vscodeApi?.getState() as T) || null;
  }
}
