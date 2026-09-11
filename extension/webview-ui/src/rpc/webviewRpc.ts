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

  public updateColorGroups(colorGroups: any[]): void {
    this.sendMessage({
      type: 'updateColorGroups',
      payload: { color_groups: colorGroups },
    });
  }

  public setState(state: any): void {
    this.vscodeApi?.setState(state);
  }

  public getState<T>(): T | null {
    return (this.vscodeApi?.getState() as T) || null;
  }
}
