import WebSocket from 'ws';
import * as vscode from 'vscode';
import { JsonRpcRequest, JsonRpcResponse, JsonRpcNotification } from '../types/protocol';

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

  constructor(
    private readonly port: number,
    private readonly token: string,
    private readonly outputChannel?: vscode.OutputChannel,
  ) {}

  public connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      const url = `ws://127.0.0.1:${this.port}/ws`;
      this.outputChannel?.appendLine(`[IPC CLIENT] Connecting to ${url}...`);

      const headers = {
        Authorization: `Bearer ${this.token}`,
      };

      this.ws = new WebSocket(url, { headers });

      let resolved = false;

      this.ws.on('open', () => {
        this.outputChannel?.appendLine('[IPC CLIENT] Connected successfully.');
        if (!resolved) {
          resolved = true;
          resolve();
        }
      });

      this.ws.on('message', (data: WebSocket.RawData) => {
        const text = data.toString();
        this.handleIncomingMessage(text);
      });

      this.ws.on('error', (err) => {
        this.outputChannel?.appendLine(`[IPC CLIENT ERROR] ${err.message}`);
        if (!resolved) {
          resolved = true;
          reject(err);
        }
      });

      this.ws.on('close', (code, reason) => {
        this.outputChannel?.appendLine(`[IPC CLIENT CLOSED] Code: ${code}, Reason: ${reason}`);
        this.ws = null;
        // Reject all pending requests
        for (const [id, req] of this.pendingRequests.entries()) {
          clearTimeout(req.timer);
          req.reject(new Error('WebSocket connection closed'));
          this.pendingRequests.delete(id);
        }
      });
    });
  }

  public sendRequest<T = any>(method: string, params?: unknown, timeoutMs = 15000): Promise<T> {
    if (this.isDisposed || !this.ws || this.ws.readyState !== WebSocket.OPEN) {
      return Promise.reject(new Error('IPC WebSocket is not connected or client is disposed'));
    }

    const id = this.requestIdCounter++;
    const request: JsonRpcRequest = {
      jsonrpc: '2.0',
      id,
      method,
      params,
    };

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

    return new vscode.Disposable(() => {
      set?.delete(handler);
    });
  }

  private handleIncomingMessage(text: string): void {
    try {
      const parsed = JSON.parse(text);

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
    if (this.ws) {
      this.ws.close();
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
