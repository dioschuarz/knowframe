/**
 * JSON-RPC 2.0 stdio client for llm-wiki-core deterministic engine sidecar.
 * Implements FR-041.
 */

import { ChildProcess, spawn } from 'child_process';
import * as readline from 'readline';

export interface JsonRpcRequest {
  jsonrpc: '2.0';
  id: number;
  method: string;
  params: any;
}

export interface JsonRpcResponse<T = any> {
  jsonrpc: '2.0';
  id: number;
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: any;
  };
}

export class EngineRpcClient {
  private childProcess: ChildProcess | null = null;
  private requestId = 1;
  private pending = new Map<number, { resolve: (res: any) => void; reject: (err: any) => void; timer: NodeJS.Timeout }>();
  private rl: readline.Interface | null = null;
  private isDisposed = false;

  private readonly binaryPath?: string;

  constructor(binaryPath?: string) {
    this.binaryPath = binaryPath;
  }

  public start(binaryPath?: string): void {
    const bin = binaryPath || this.binaryPath || 'llm-wiki-core';
    this.childProcess = spawn(bin, ['engine'], {
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    if (this.childProcess.stdout) {
      this.rl = readline.createInterface({ input: this.childProcess.stdout });
      this.rl.on('line', (line: string) => {
        this.handleMessage(line);
      });
    }

    this.childProcess.on('error', (err) => {
      this.rejectAll(new Error(`Sidecar process error: ${err.message}`));
    });

    this.childProcess.on('exit', (code) => {
      if (!this.isDisposed) {
        this.rejectAll(new Error(`Sidecar process exited with code ${code}`));
      }
    });
  }

  public handleMessage(line: string): void {
    try {
      const response: JsonRpcResponse = JSON.parse(line);
      const pendingReq = this.pending.get(response.id);
      if (pendingReq) {
        clearTimeout(pendingReq.timer);
        this.pending.delete(response.id);
        if (response.error) {
          pendingReq.reject(new Error(`RPC Error (${response.error.code}): ${response.error.message}`));
        } else {
          pendingReq.resolve(response.result);
        }
      }
    } catch {
      // Ignore malformed non-JSON lines
    }
  }

  public request<T = any>(method: string, params: any = {}): Promise<T> {
    return new Promise((resolve, reject) => {
      const id = this.requestId++;
      const payload: JsonRpcRequest = {
        jsonrpc: '2.0',
        id,
        method,
        params,
      };

      const timer = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`RPC request '${method}' (id ${id}) timed out after 10000ms`));
      }, 10000);

      this.pending.set(id, { resolve, reject, timer });

      if (this.childProcess && this.childProcess.stdin && !this.childProcess.stdin.destroyed) {
        this.childProcess.stdin.write(JSON.stringify(payload) + '\n');
      } else {
        // If not running as process (e.g. mock / in-memory unit tests), keep pending for manual handleMessage
      }
    });
  }

  public analyzeRepository(params: { root?: string; mode?: string } = {}): Promise<any> {
    return this.request('engine/analyzeRepository', params);
  }

  public getGraphTopology(params: { root?: string } = {}): Promise<any> {
    return this.request('engine/getGraphTopology', params);
  }

  public setProfileMode(params: { mode: string }): Promise<any> {
    return this.request('engine/setProfileMode', params);
  }

  public updateCustomProfile(params: { weights?: any; thresholds?: any; constraints?: any }): Promise<any> {
    return this.request('engine/updateCustomProfile', params);
  }

  public generateTransformationPlan(params: {
    strategy_id: string;
    target_files: string[];
    actions?: any[];
    projected_score_delta?: number;
  }): Promise<any> {
    return this.request('engine/generateTransformationPlan', params);
  }

  public applyTransformationPlan(params: {
    plan_id: string;
    approval_token: string;
    actions: any[];
  }): Promise<any> {
    return this.request('engine/applyTransformationPlan', params);
  }

  public getAffectedDocuments(
    params: import('../types/engine').GetAffectedDocumentsParams = {}
  ): Promise<import('../types/engine').AffectedDocumentsReport> {
    return this.request('trace/getAffectedDocuments', params);
  }

  public resolveDocumentReview(
    params: import('../types/engine').ResolveDocumentReviewParams
  ): Promise<import('../types/engine').ResolveDocumentReviewResult> {
    return this.request('review/resolveDocumentReview', params);
  }

  private rejectAll(err: Error): void {
    for (const [id, req] of this.pending.entries()) {
      clearTimeout(req.timer);
      req.reject(err);
    }
    this.pending.clear();
  }

  public dispose(): void {
    this.isDisposed = true;
    this.rejectAll(new Error('RPC Client disposed'));
    if (this.rl) {
      this.rl.close();
      this.rl = null;
    }
    if (this.childProcess) {
      this.childProcess.kill();
      this.childProcess = null;
    }
  }
}
