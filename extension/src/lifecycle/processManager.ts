import { ChildProcess, spawn } from 'child_process';
import * as vscode from 'vscode';
import { resolveBinaryPath } from './binaryResolver';
import { SecretManager } from './secretManager';

export class ProcessManager implements vscode.Disposable {
  private process: ChildProcess | null = null;
  private activePort: number | null = null;
  private isIntentionallyStopped = false;
  private restartAttempts = 0;
  private readonly maxRestartAttempts = 3;
  private readonly onDidRestartEmitter = new vscode.EventEmitter<number>();
  public readonly onDidRestart = this.onDidRestartEmitter.event;

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly secretManager: SecretManager,
    private readonly outputChannel: vscode.OutputChannel,
  ) {}

  public get port(): number | null {
    return this.activePort;
  }

  public isRunning(): boolean {
    return this.process !== null && !this.process.killed && this.activePort !== null;
  }

  public async ensureRunning(workspaceRoot: string, desiredPort = 0): Promise<number> {
    if (this.isRunning() && this.activePort) {
      return this.activePort;
    }
    return this.start(workspaceRoot, desiredPort);
  }

  public async start(workspaceRoot: string, desiredPort = 0): Promise<number> {
    this.isIntentionallyStopped = false;
    const binaryPath = resolveBinaryPath(this.context.extensionPath);
    const token = await this.secretManager.initializeToken();

    return new Promise<number>((resolve, reject) => {
      this.outputChannel.appendLine(`[PROCESS] Spawning sidecar daemon: ${binaryPath} (root=${workspaceRoot})`);

      const args = ['daemon', '--root', workspaceRoot, '--port', desiredPort.toString()];
      const env = {
        ...process.env,
        LLM_WIKI_AUTH_TOKEN: token,
      };

      try {
        this.process = spawn(binaryPath, args, {
          env,
          cwd: workspaceRoot,
          stdio: ['ignore', 'pipe', 'pipe'],
        });
      } catch (err) {
        return reject(new Error(`Failed to spawn daemon: ${err}`));
      }

      let resolved = false;

      this.process.stdout?.on('data', (chunk: Buffer) => {
        const text = chunk.toString();
        this.outputChannel.append(`[DAEMON OUT] ${text}`);

        for (const line of text.split('\n')) {
          const trimmed = line.trim();
          if (!trimmed) {continue;}
          try {
            const data = JSON.parse(trimmed);
            if (data.event === 'daemon_ready' && data.port) {
              this.activePort = Number(data.port);
              this.restartAttempts = 0;
              this.onDidRestartEmitter.fire(this.activePort);
              if (!resolved) {
                resolved = true;
                resolve(this.activePort);
              }
            }
          } catch {
            // Non-JSON log line
          }
        }
      });

      this.process.stderr?.on('data', (chunk: Buffer) => {
        this.outputChannel.append(`[DAEMON ERR] ${chunk.toString()}`);
      });

      this.process.on('error', (err) => {
        this.outputChannel.appendLine(`[PROCESS ERROR] ${err.message}`);
        if (!resolved) {
          resolved = true;
          reject(err);
        }
      });

      this.process.on('exit', (code, signal) => {
        this.outputChannel.appendLine(`[PROCESS EXIT] Code: ${code}, Signal: ${signal}`);
        this.activePort = null;
        this.process = null;

        if (!this.isIntentionallyStopped && this.restartAttempts < this.maxRestartAttempts) {
          this.restartAttempts++;
          const delay = Math.pow(2, this.restartAttempts) * 500;
          this.outputChannel.appendLine(
            `[PROCESS RESTART] Restarting daemon in ${delay}ms (Attempt ${this.restartAttempts}/${this.maxRestartAttempts})...`,
          );
          setTimeout(() => {
            this.start(workspaceRoot, desiredPort).catch((e) => {
              this.outputChannel.appendLine(`[PROCESS RESTART FAILED] ${e.message}`);
            });
          }, delay);
        }
      });

      // Fail-safe timeout
      setTimeout(() => {
        if (!resolved) {
          resolved = true;
          reject(new Error('Timed out waiting for daemon to report ready port (15s)'));
        }
      }, 15000);
    });
  }

  public async stop(): Promise<void> {
    this.isIntentionallyStopped = true;
    if (!this.process) {
      return;
    }

    const proc = this.process;
    this.process = null;
    this.activePort = null;

    return new Promise<void>((resolve) => {
      const timer = setTimeout(() => {
        try {
          proc.kill('SIGKILL');
        } catch {}
        resolve();
      }, 2000);

      proc.once('exit', () => {
        clearTimeout(timer);
        resolve();
      });

      try {
        proc.kill('SIGTERM');
      } catch {
        clearTimeout(timer);
        resolve();
      }
    });
  }

  public dispose(): void {
    this.onDidRestartEmitter.dispose();
    this.stop().catch(() => {});
  }
}
