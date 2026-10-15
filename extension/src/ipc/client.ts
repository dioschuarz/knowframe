import WebSocket from 'ws';
import type * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import type { JsonRpcRequest, JsonRpcResponse, JsonRpcNotification } from '../types/protocol.ts';

import type { FragmentMetadata, FivePillarQualityScore, ContextTraceSectionResponse } from '../types/fragment.ts';

interface PendingRequest {
  resolve: (value: any) => void;
  reject: (reason: any) => void;
  timer: NodeJS.Timeout;
}

export class IpcClient implements vscode.Disposable {
  private ws: WebSocket | null = null;
  private requestIdCounter = 1;
  private pendingRequests = new Map<string | number, PendingRequest>();
  private notificationHandlers = new Map<string, Set<(params: any) => void>>();
  private isDisposed = false;
  private connectPromise: Promise<void> | null = null;
  private pingInterval: NodeJS.Timeout | null = null;

  private portOrGetter: number | (() => number | null);
  private readonly token: string;
  private readonly outputChannel?: vscode.OutputChannel;
  private readonly workspaceRoot?: string;

  constructor(
    portOrGetter: number | (() => number | null),
    token: string,
    outputChannel?: vscode.OutputChannel,
    workspaceRoot?: string,
  ) {
    this.portOrGetter = portOrGetter;
    this.token = token;
    this.outputChannel = outputChannel;
    this.workspaceRoot = workspaceRoot;
  }

  public getActivePort(): number | null {
    let p = typeof this.portOrGetter === 'function' ? this.portOrGetter() : this.portOrGetter;
    if ((!p || p <= 0) && this.workspaceRoot) {
      try {
        const portFile = path.join(this.workspaceRoot, '.git', 'llm-wiki.port');
        if (fs.existsSync(portFile)) {
          const parsed = parseInt(fs.readFileSync(portFile, 'utf8').trim(), 10);
          if (!isNaN(parsed) && parsed > 0) {
            p = parsed;
          }
        }
      } catch {}
    }
    return p && p > 0 ? p : null;
  }

  public setPort(newPort: number): void {
    this.portOrGetter = newPort;
  }

  public isConnected(): boolean {
    return !this.isDisposed && this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  private startKeepalive(): void {
    this.stopKeepalive();
    this.pingInterval = setInterval(() => {
      if (this.isConnected() && this.ws) {
        try {
          this.ws.ping();
        } catch {
          // Socket might have dropped
        }
      }
    }, 25000);
  }

  private stopKeepalive(): void {
    if (this.pingInterval) {
      clearInterval(this.pingInterval);
      this.pingInterval = null;
    }
  }

  public ensureConnected(): Promise<void> {
    if (this.isConnected()) {
      return Promise.resolve();
    }
    if (this.connectPromise) {
      return this.connectPromise;
    }
    this.connectPromise = this.connect().finally(() => {
      this.connectPromise = null;
    });
    return this.connectPromise;
  }

  public connect(): Promise<void> {
    if (this.isDisposed) {
      return Promise.reject(new Error('IpcClient is disposed'));
    }
    if (this.isConnected()) {
      return Promise.resolve();
    }

    const port = this.getActivePort();
    if (!port) {
      return Promise.reject(new Error('Sidecar daemon port is not available'));
    }

    if (this.ws) {
      try {
        this.ws.removeAllListeners();
        this.ws.close();
      } catch {}
      this.ws = null;
    }

    return new Promise((resolve, reject) => {
      const url = `ws://127.0.0.1:${port}/ws`;
      this.outputChannel?.appendLine(`[IPC CLIENT] Connecting to ${url}...`);

      const headers = {
        Authorization: `Bearer ${this.token}`,
      };

      let wsInstance: WebSocket;
      try {
        wsInstance = new WebSocket(url, { headers });
      } catch (err) {
        return reject(err);
      }
      this.ws = wsInstance;

      let settled = false;

      const connectionTimeout = setTimeout(() => {
        if (!settled) {
          settled = true;
          try {
            wsInstance.close();
          } catch {}
          if (this.ws === wsInstance) {
            this.ws = null;
          }
          reject(new Error(`Connection to daemon timed out after 10000ms on port ${port}`));
        }
      }, 10000);

      wsInstance.on('open', () => {
        this.outputChannel?.appendLine('[IPC CLIENT] Connected successfully.');
        if (!settled) {
          settled = true;
          clearTimeout(connectionTimeout);
          this.startKeepalive();
          resolve();
        }
      });

      wsInstance.on('message', (data: WebSocket.RawData) => {
        const text = data.toString();
        this.handleIncomingMessage(text);
      });

      wsInstance.on('error', (err) => {
        this.outputChannel?.appendLine(`[IPC CLIENT ERROR] ${err.message}`);
        if (!settled) {
          settled = true;
          clearTimeout(connectionTimeout);
          if (this.ws === wsInstance) {
            this.ws = null;
          }
          reject(err);
        }
      });

      wsInstance.on('close', (code, reason) => {
        this.outputChannel?.appendLine(`[IPC CLIENT CLOSED] Code: ${code}, Reason: ${reason}`);
        this.stopKeepalive();
        if (this.ws === wsInstance) {
          this.ws = null;
        }
        // Reject all pending requests
        for (const [id, req] of this.pendingRequests.entries()) {
          clearTimeout(req.timer);
          req.reject(new Error('WebSocket connection closed'));
          this.pendingRequests.delete(id);
        }
      });
    });
  }

  public async sendRequest<T = any>(method: string, params?: unknown, timeoutMs = 90000): Promise<T> {
    if (this.isDisposed) {
      throw new Error('IPC WebSocket client is disposed');
    }

    if (!this.isConnected()) {
      this.outputChannel?.appendLine(`[IPC CLIENT] Socket not connected. Auto-reconnecting before ${method}...`);
      await this.ensureConnected();
    }

    const id = this.requestIdCounter++;
    const request: JsonRpcRequest = {
      jsonrpc: '2.0',
      id,
      method,
      params,
    };

    this.outputChannel?.appendLine(`[IPC CLIENT] Sending request: id=${id}, method=${method}`);

    return new Promise<T>((resolve, reject) => {
      const timer = setTimeout(() => {
        this.pendingRequests.delete(id);
        reject(new Error(`Request timed out after ${timeoutMs}ms: method=${method}, id=${id}`));
      }, timeoutMs);

      this.pendingRequests.set(id, { resolve, reject, timer });
      this.ws!.send(JSON.stringify(request), (err) => {
        if (err) {
          clearTimeout(timer);
          this.pendingRequests.delete(id);
          reject(err);
        }
      });
    });
  }

  public getDocumentFragments(path: string): Promise<{ path: string; total_tokens: number; fragments: FragmentMetadata[] }> {
    return this.sendRequest('graph/getDocumentFragments', { path });
  }

  public materializeFrontmatter(path: string, dryRun = false): Promise<{ path: string; updated: boolean; fragment_count: number; total_tokens: number; frontmatter_yaml?: string }> {
    return this.sendRequest('graph/materializeFrontmatter', { path, dry_run: dryRun });
  }

  public traceContextSection(query: string, entrypoint?: string, maxDepth = 4, beamWidth = 5): Promise<ContextTraceSectionResponse> {
    return this.sendRequest('trace/contextTraceSection', { query, entrypoint, max_depth: maxDepth, beam_width: beamWidth });
  }

  public getFivePillarScore(forceRecompute = false): Promise<FivePillarQualityScore> {
    return this.sendRequest('quality/getFivePillarScore', { force_recompute: forceRecompute });
  }

  public sendNotification(method: string, params: unknown): void {
    if (this.isDisposed || !this.ws || this.ws.readyState !== WebSocket.OPEN) {
      return;
    }

    const notif: JsonRpcNotification = {
      jsonrpc: '2.0',
      method,
      params,
    };

    this.ws.send(JSON.stringify(notif));
  }

  public onNotification(method: string, handler: (params: any) => void): vscode.Disposable {
    let set = this.notificationHandlers.get(method);
    if (!set) {
      set = new Set();
      this.notificationHandlers.set(method, set);
    }
    set.add(handler);

    return {
      dispose: () => {
        set?.delete(handler);
      },
    };
  }

  private handleIncomingMessage(text: string): void {
    try {
      const parsed = JSON.parse(text);
      this.outputChannel?.appendLine(`[IPC CLIENT] Incoming message id=${parsed.id}, method=${parsed.method}`);

      // 1. Check if it's a response
      if (parsed.id !== undefined && this.pendingRequests.has(parsed.id)) {
        const req = this.pendingRequests.get(parsed.id)!;
        this.pendingRequests.delete(parsed.id);
        clearTimeout(req.timer);

        const rpcRes = parsed as JsonRpcResponse;
        if (rpcRes.error) {
          req.reject(new Error(`JSON-RPC Error ${rpcRes.error.code}: ${rpcRes.error.message}`));
        } else {
          req.resolve(rpcRes.result);
        }
        return;
      }

      // 2. Check if it's a notification
      if (parsed.method) {
        const handlers = this.notificationHandlers.get(parsed.method);
        if (handlers) {
          for (const h of handlers) {
            try {
              h(parsed.params);
            } catch (err) {
              this.outputChannel?.appendLine(`[NOTIFICATION HANDLER ERROR] ${err}`);
            }
          }
        }
      }
    } catch (e) {
      this.outputChannel?.appendLine(`[IPC PARSE ERROR] ${e}`);
    }
  }

  public dispose(): void {
    this.isDisposed = true;
    this.stopKeepalive();
    if (this.ws) {
      try {
        this.ws.close();
      } catch {}
      this.ws = null;
    }
    for (const req of this.pendingRequests.values()) {
      clearTimeout(req.timer);
      req.reject(new Error('IpcClient disposed'));
    }
    this.pendingRequests.clear();
    this.notificationHandlers.clear();
  }
}
