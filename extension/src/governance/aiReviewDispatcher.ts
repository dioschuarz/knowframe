import type * as vscode from 'vscode';
import { HarnessToolService } from './harnessToolService.ts';
import type { GovernanceManager } from './governanceManager.ts';
import type { HarnessToolStatus } from '../types/governance.ts';

export interface AiReviewOptions {
  filePath?: string;
  issue?: {
    category?: string;
    line_number?: number;
    description?: string;
    target?: string;
    pillar?: string;
  };
  all?: boolean;
}

export class AiReviewDispatcher {
  private readonly toolService: HarnessToolService;
  private readonly governanceManager?: GovernanceManager;

  constructor(
    toolService: HarnessToolService = new HarnessToolService(),
    governanceManager?: GovernanceManager
  ) {
    this.toolService = toolService;
    this.governanceManager = governanceManager;
  }

  /**
   * Resolves the target AI agent CLI based on active harness configuration and installed tools.
   */
  public async getTargetAgent(): Promise<HarnessToolStatus | null> {
    const candidateIds = ['agy', 'codex', 'claude', 'cursor', 'gh'];
    const detected = await this.toolService.detectAllTools();
    const installed = detected.filter((t) => candidateIds.includes(t.id) && t.is_installed);

    if (installed.length === 0) {
      return null;
    }

    // 1. If active harness matches a specific agent, prioritize it
    if (this.governanceManager) {
      try {
        const status = await this.governanceManager.getHarnessStatus();
        const activeType = status.active_harness?.harness_type;
        if (activeType === 'gemini_md') {
          const agy = installed.find((t) => t.id === 'agy');
          if (agy) return agy;
        } else if (activeType === 'claude_md') {
          const claude = installed.find((t) => t.id === 'claude');
          if (claude) return claude;
        } else if (activeType === 'copilot_instructions') {
          const gh = installed.find((t) => t.id === 'gh');
          if (gh) return gh;
        } else if (activeType === 'agents_md') {
          const codex = installed.find((t) => t.id === 'codex');
          if (codex) return codex;
          const agy = installed.find((t) => t.id === 'agy');
          if (agy) return agy;
        }
      } catch {
        // Fall back to default priority
      }
    }

    // 2. Default priority: agy -> codex -> claude -> cursor -> gh
    for (const id of candidateIds) {
      const match = installed.find((t) => t.id === id);
      if (match) return match;
    }

    return installed[0];
  }

  /**
   * Constructs a targeted instruction prompt adhering to the principle:
   * "Fix only this file and this issue, recheck until fixed."
   */
  public buildPrompt(options: AiReviewOptions): string {
    if (options.all) {
      return 'Review all knowledge base health and integrity issues reported by KnowFrame. Fix broken links, unanchored references, and review obligations across the workspace. Verify and re-check until all issues are resolved.';
    }

    const filePath = options.filePath || options.issue?.target || 'target file';
    const cat = options.issue?.category ? `${options.issue.category}` : 'integrity';
    const line = options.issue?.line_number ? ` at line ${options.issue.line_number}` : '';
    const desc = options.issue?.description ? `: ${options.issue.description}` : '';

    return `Review and fix the ${cat} issue in '${filePath}'${line}${desc}. IMPORTANT: Modify ONLY '${filePath}' and fix ONLY this issue. Re-check until the ${cat} issue in '${filePath}' is completely resolved.`;
  }

  /**
   * Formats the CLI invocation command for the target agent.
   */
  public buildCommand(agentId: string, prompt: string): string {
    const escaped = prompt.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
    switch (agentId) {
      case 'agy':
        return `agy -i "${escaped}"`;
      case 'codex':
        return `codex "${escaped}"`;
      case 'claude':
        return `claude "${escaped}"`;
      case 'cursor':
        return `cursor --agent "${escaped}"`;
      case 'gh':
        return `gh copilot suggest "${escaped}"`;
      default:
        return `${agentId} "${escaped}"`;
    }
  }

  /**
   * Dispatches the review command in a dedicated VS Code terminal.
   */
  public async dispatchReview(options: AiReviewOptions): Promise<boolean> {
    const vscode = await import('vscode');
    const agent = await this.getTargetAgent();
    if (!agent) {
      const choice = await vscode.window.showWarningMessage(
        'No supported AI agent CLI (agy, codex, claude) detected on your PATH.',
        'Open AI Harness Settings'
      );
      if (choice === 'Open AI Harness Settings') {
        vscode.commands.executeCommand('llmWiki.openHarness');
      }
      return false;
    }

    const prompt = this.buildPrompt(options);
    const cmd = this.buildCommand(agent.id, prompt);

    const termName = 'KnowFrame: AI Review';
    let terminal = vscode.window.terminals?.find((t) => t.name === termName);
    if (!terminal) {
      terminal = vscode.window.createTerminal({ name: termName });
    }
    terminal.show(true);
    terminal.sendText(cmd);

    const targetLabel = options.all ? 'all issues' : (options.filePath || 'selected issue');
    vscode.window.showInformationMessage(
      `KnowFrame: Dispatched review for ${targetLabel} to ${agent.name}.`
    );
    return true;
  }
}
