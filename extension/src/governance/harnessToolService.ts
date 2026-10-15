import { exec } from 'child_process';
import { promisify } from 'util';
import type * as vscode from 'vscode';
import type { HarnessToolCategory, HarnessToolStatus } from '../types/governance.ts';

const execAsync = promisify(exec);

export interface HarnessToolDefinition {
  id: string;
  name: string;
  category: HarnessToolCategory;
  description: string;
  binary: string;
  versionCommand?: string;
  installCommand: string;
  requiredFor?: string;
  docsUrl?: string;
}

export interface ToolInstallOptions {
  terminalLauncher?: (name: string, command: string) => void;
  outputChannel?: vscode.OutputChannel;
  commandExecutor?: (cmd: string) => Promise<{ stdout: string; stderr: string }>;
}

export const CANONICAL_HARNESS_TOOLS: HarnessToolDefinition[] = [
  // 1. Core Git & Hook Admission Control
  {
    id: 'prek',
    name: 'prek (Rust Git Hook Orchestrator)',
    category: 'core',
    binary: 'prek',
    description: 'Rust-native Git hook orchestrator enforcing sub-500ms pre-commit admission control & post-commit sync.',
    installCommand: 'cargo install prek',
    requiredFor: 'Dual Git Hooks (Tier 1)',
    docsUrl: 'https://github.com/jdx/prek',
  },
  // 2. Runtime & Python Environment Management
  {
    id: 'uv',
    name: 'uv (Fast Python Environment & Tools)',
    category: 'runtime',
    binary: 'uv',
    description: 'Fast Python tool & virtualenv manager mandated by Mandatory Rule 10 for script execution and transcripts.',
    installCommand: 'curl -LsSf https://astral.sh/uv/install.sh | sh',
    requiredFor: 'Python Skills & Scripts (Rule 10)',
    docsUrl: 'https://docs.astral.sh/uv/',
  },
  // 3. Persistent Agent Memory
  {
    id: 'ai_memory',
    name: 'ai-memory (Cross-Session Memory CLI)',
    category: 'core',
    binary: 'ai-memory',
    description: 'Cross-session long-term memory engine and handoffs manager mandated in root AGENTS.md.',
    installCommand: 'cargo install ai-memory',
    requiredFor: 'Persistent Agent Memory',
    docsUrl: 'https://github.com/akitaonrails/ai-memory',
  },
  // 4. Grounded Research & Studies
  {
    id: 'notebooklm',
    name: 'notebooklm (NotebookLM CLI & Research)',
    category: 'core',
    binary: 'notebooklm',
    description: 'Zero-token grounded RAG research engine for gap closing and technical video studies.',
    installCommand: 'uv tool install notebooklm-py',
    requiredFor: 'Grounded RAG & Gap Closing',
    docsUrl: 'https://github.com/tomaarsen/notebooklm-py',
  },
  // 5. Claude Code AI Agent
  {
    id: 'claude',
    name: 'Claude Code (Anthropic CLI Agent)',
    category: 'agent',
    binary: 'claude',
    description: 'Anthropic agentic CLI for code modifications, subagents, and CLAUDE.md execution.',
    installCommand: 'npm install -g @anthropic-ai/claude-code',
    requiredFor: 'CLAUDE.md Agent Harness',
    docsUrl: 'https://docs.anthropic.com/en/docs/claude-code',
  },
  // 6. Antigravity AI Agent
  {
    id: 'agy',
    name: 'Antigravity CLI (Google DeepMind)',
    category: 'agent',
    binary: 'agy',
    description: 'Antigravity pair-programming agent CLI and skills runner for GEMINI.md.',
    installCommand: 'uv tool install antigravity-cli',
    requiredFor: 'GEMINI.md Agent Harness',
  },
  // 7. OpenAI Codex CLI Agent
  {
    id: 'codex',
    name: 'Codex CLI (OpenAI)',
    category: 'agent',
    binary: 'codex',
    description: 'OpenAI Codex CLI agent executing instructions in AGENTS.md.',
    installCommand: 'npm install -g @openai/codex',
    requiredFor: 'AGENTS.md Agent Harness',
  },
  // 8. Cursor AI Editor CLI
  {
    id: 'cursor',
    name: 'Cursor CLI',
    category: 'agent',
    binary: 'cursor',
    description: 'Cursor AI code editor CLI and agent rule evaluator (.cursor/rules).',
    installCommand: 'curl -fsSL https://cursor.com -o cursor.AppImage',
    requiredFor: 'Cursor Rules Harness',
    docsUrl: 'https://cursor.com',
  },
  // 9. GitHub CLI & Copilot
  {
    id: 'gh',
    name: 'GitHub CLI & Copilot (gh)',
    category: 'agent',
    binary: 'gh',
    description: 'GitHub CLI with Copilot extension for command assistance & PR reviews.',
    installCommand: 'gh extension install github/gh-copilot',
    requiredFor: 'GitHub Copilot Harness',
    docsUrl: 'https://github.com/github/gh-copilot',
  },
  // 10. Code Diagnostics & Observed Architecture
  {
    id: 'graphify',
    name: 'graphify (Code Architecture Graph Extractor)',
    category: 'core',
    binary: 'graphify',
    description: 'Optional offline diagnostic tool for observed code relations and AST dependency extraction. Passively supplements LLM Wiki without mutating canonical markdown.',
    installCommand: 'uv tool install graphifyy',
    requiredFor: 'Optional Code Diagnostics',
    docsUrl: 'https://github.com/Graphify-Labs/graphify',
  },
];

export class HarnessToolService {
  private readonly tools: HarnessToolDefinition[];
  private customExecutor?: (cmd: string) => Promise<{ stdout: string; stderr: string }>;

  constructor(
    tools: HarnessToolDefinition[] = CANONICAL_HARNESS_TOOLS,
    customExecutor?: (cmd: string) => Promise<{ stdout: string; stderr: string }>
  ) {
    this.tools = tools;
    this.customExecutor = customExecutor;
  }

  public getRegisteredTools(): HarnessToolDefinition[] {
    return [...this.tools];
  }

  private async execCmd(cmd: string): Promise<{ stdout: string; stderr: string }> {
    if (this.customExecutor) {
      return this.customExecutor(cmd);
    }
    return execAsync(cmd, { timeout: 2500 });
  }

  /**
   * Checks whether a single tool is installed and retrieves its version.
   */
  public async detectTool(tool: HarnessToolDefinition): Promise<HarnessToolStatus> {
    try {
      const whichRes = await this.execCmd(`which ${tool.binary}`);
      const binPath = whichRes.stdout.trim().split('\n')[0];

      let version: string | null = null;
      try {
        const verCmd = tool.versionCommand || `${tool.binary} --version`;
        const verRes = await this.execCmd(verCmd);
        const firstLine = (verRes.stdout || verRes.stderr).trim().split('\n')[0];
        if (firstLine) {
          version = firstLine.replace(/^[^0-9]*([0-9]+(\.[0-9]+)+(-[a-zA-Z0-9.]+)?).*$/, '$1').trim();
          if (!version || version.length > 25) {
            version = firstLine.slice(0, 30).trim();
          }
        }
      } catch {
        version = 'Installed';
      }

      return {
        id: tool.id,
        name: tool.name,
        category: tool.category,
        description: tool.description,
        binary: tool.binary,
        is_installed: true,
        version: version || 'Installed',
        bin_path: binPath,
        install_command: tool.installCommand,
        required_for: tool.requiredFor,
        docs_url: tool.docsUrl,
      };
    } catch {
      return {
        id: tool.id,
        name: tool.name,
        category: tool.category,
        description: tool.description,
        binary: tool.binary,
        is_installed: false,
        version: null,
        bin_path: null,
        install_command: tool.installCommand,
        required_for: tool.requiredFor,
        docs_url: tool.docsUrl,
      };
    }
  }

  /**
   * Concurrently checks the installation status of all registered tools.
   */
  public async detectAllTools(): Promise<HarnessToolStatus[]> {
    return Promise.all(this.tools.map((t) => this.detectTool(t)));
  }

  /**
   * Executes the installation command for the requested tool.
   * Under Invariant XVI, launches in terminal so user can monitor progress and handle any prompts.
   */
  public async installTool(toolId: string, options?: ToolInstallOptions): Promise<boolean> {
    const tool = this.tools.find((t) => t.id === toolId);
    if (!tool) {
      options?.outputChannel?.appendLine(`[TOOL INSTALLER] Tool not found: ${toolId}`);
      return false;
    }

    options?.outputChannel?.appendLine(`[TOOL INSTALLER] Initiating install for ${tool.name}: ${tool.installCommand}`);

    if (options?.terminalLauncher) {
      options.terminalLauncher(`LLM Wiki: Install ${tool.name}`, tool.installCommand);
      return true;
    }

    try {
      const vscode = await import('vscode');
      const termName = `LLM Wiki: Install ${tool.name}`;
      const existingTerminal = vscode.window.terminals?.find((t: any) => t.name === termName);
      const terminal = existingTerminal || vscode.window.createTerminal({
        name: termName,
      });
      terminal.show(true);
      terminal.sendText(tool.installCommand);
      vscode.window.showInformationMessage(
        `LLM Wiki: Installing ${tool.name} in terminal (${tool.installCommand}).`
      );
      return true;
    } catch (err: any) {
      options?.outputChannel?.appendLine(`[TOOL INSTALLER] Terminal launch failed: ${err?.message}`);
      return false;
    }
  }

  /**
   * Quick check for prek specifically (used by Git hook status).
   */
  public async isPrekInstalled(): Promise<boolean> {
    const status = await this.detectTool(this.tools.find((t) => t.id === 'prek') || CANONICAL_HARNESS_TOOLS[0]);
    return status.is_installed;
  }
}
