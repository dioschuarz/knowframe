import * as fs from 'fs';
import * as path from 'path';
import type { IpcClient } from '../ipc/client.ts';
import type {
  AIOrientedCapabilityMatrix,
  DirectoryStructureMetrics,
  HarnessEntryPoint,
  HarnessEvolutionRecommendation,
  ProcedureCandidate,
  SkillDefinition,
} from '../types/governance.ts';

export interface HarnessStatusResponse {
  harnesses: HarnessEntryPoint[];
  active_harness: HarnessEntryPoint | null;
  capability_matrix: AIOrientedCapabilityMatrix;
}

export interface SetActiveHarnessResponse {
  active_harness: string;
  previous_harness: string | null;
}

export interface SkillCatalogResponse {
  skills: SkillDefinition[];
  candidates: ProcedureCandidate[];
  total_skills_count: number;
  duplicate_candidates_count: number;
}

export interface ApplyGovernancePlanResponse {
  success: boolean;
  applied_plan_id: string;
  re_measurement_report: {
    delta_tokens: number;
    delta_hops: number;
    delta_score: number;
  };
}

export type Listener<T> = (data: T) => void;

/**
 * GovernanceManager coordinates AI Harness Entry Points, Knowledge Index routing,
 * Skill auditing, Procedure extraction, and User-Approved Transformations.
 */
export class GovernanceManager {
  private readonly ipcClient: IpcClient;
  private cachedStatus: HarnessStatusResponse | null = null;
  private statusListeners: Set<Listener<HarnessStatusResponse>> = new Set();

  constructor(ipcClient: IpcClient) {
    this.ipcClient = ipcClient;
  }

  public subscribeStatus(listener: Listener<HarnessStatusResponse>): () => void {
    this.statusListeners.add(listener);
    if (this.cachedStatus) {
      listener(this.cachedStatus);
    }
    return () => this.statusListeners.delete(listener);
  }

  private notifyStatus(status: HarnessStatusResponse): void {
    this.cachedStatus = status;
    for (const listener of this.statusListeners) {
      try {
        listener(status);
      } catch (err) {
        console.error('[GovernanceManager] Error in status listener:', err);
      }
    }
  }

  /**
   * Retrieves the current harness status, detected harnesses, and capability matrix.
   */
  public async getHarnessStatus(): Promise<HarnessStatusResponse> {
    const response = await this.ipcClient.sendRequest<HarnessStatusResponse>(
      'governance/getHarnessStatus',
      {}
    );
    this.notifyStatus(response);
    return response;
  }

  /**
   * Toggles the active primary AI harness entry point.
   */
  public async setActiveHarness(harnessFile: string): Promise<SetActiveHarnessResponse> {
    const response = await this.ipcClient.sendRequest<SetActiveHarnessResponse>(
      'governance/setActiveHarness',
      { harness_file: harnessFile }
    );
    await this.getHarnessStatus();
    return response;
  }

  /**
   * Generates a template AI harness entry point at the workspace root if absent (FR-002).
   */
  public async initializeHarness(workspaceRoot: string, targetHarness: string = 'AGENTS.md'): Promise<boolean> {
    const targetPath = path.join(workspaceRoot, targetHarness);
    if (fs.existsSync(targetPath)) {
      return false; // Already exists
    }

    const routes: string[] = [];
    const agentsPath = path.join(workspaceRoot, 'AGENTS.md');
    const indexPath = path.join(workspaceRoot, 'index.md');
    const constPath = path.join(workspaceRoot, 'CONSTITUTION.md');

    // 1. If AGENTS.md exists and has routes, borrow them directly
    if (fs.existsSync(agentsPath) && targetHarness !== 'AGENTS.md') {
      try {
        const agContent = fs.readFileSync(agentsPath, 'utf-8');
        const lines = agContent.split('\n');
        let inIndex = false;
        for (const line of lines) {
          if (line.trim().startsWith('## ') && line.toLowerCase().includes('knowledge index')) {
            inIndex = true;
            continue;
          }
          if (inIndex && line.trim().startsWith('## ') && !line.toLowerCase().includes('knowledge index')) {
            inIndex = false;
          }
          if (inIndex && (line.trim().startsWith('- [') || line.trim().startsWith('### '))) {
            routes.push(line);
          }
        }
      } catch {}
    }

    // 2. If index.md exists and no routes found yet, extract routes directly from index.md
    if (routes.length === 0 && fs.existsSync(indexPath)) {
      try {
        const idxContent = fs.readFileSync(indexPath, 'utf-8');
        const lines = idxContent.split('\n');
        for (const line of lines) {
          if ((line.trim().startsWith('- [') || line.trim().startsWith('### ')) && !line.includes('(index.md)')) {
            routes.push(line);
          }
        }
      } catch {}
    }

    if (routes.length === 0 && fs.existsSync(constPath)) {
      routes.push('- [Constitution & Invariants](CONSTITUTION.md): Core architectural invariants and quality principles.');
    }

    // If no root documents were detected, scan for top-level markdown files
    if (routes.length === 0) {
      try {
        const files = fs.readdirSync(workspaceRoot);
        for (const file of files) {
          if (file.endsWith('.md') && file !== targetHarness && !file.startsWith('.')) {
            const label = file.replace(/\.md$/, '').replace(/_/g, ' ');
            routes.push(`- [${label}](${file})`);
          }
        }
      } catch {
        // Fallback
      }
    }

    if (routes.length === 0) {
      routes.push('- [Documentation](README.md): Project knowledge and guidelines.');
    }

    let titleHeader = `# ${targetHarness} — AI Harness Entry Point`;
    if (targetHarness === 'CLAUDE.md') {
      titleHeader = '# CLAUDE.md — Claude Code Instructions & Knowledge Index';
    } else if (targetHarness === 'GEMINI.md') {
      titleHeader = '# GEMINI.md — Gemini CLI & Agent Instructions';
    }

    const template = `${titleHeader}

Welcome to this repository. This file serves as the deterministic knowledge and instructions entry point for AI agents.

## 1. Naming Standards & Conventions
- **Files & Notes**: Use \`snake_case\` for markdown notes and documentation.
- **Code Symbols**: Use \`CamelCase\` for types, classes, and components.

## 2. Knowledge Index
${routes.join('\n')}
`;

    fs.writeFileSync(targetPath, template, 'utf-8');
    await this.getHarnessStatus();
    return true;
  }

  /**
   * Synchronizes Knowledge Index across all configured/detected AI harnesses.
   */
  public async syncHarnesses(workspaceRoot: string): Promise<number> {
    try {
      const resp = await this.ipcClient.sendRequest<{ synced_count: number }>(
        'governance/syncHarnesses',
        {}
      );
      await this.getHarnessStatus();
      return resp.synced_count;
    } catch {
      return 0;
    }
  }

  /**
   * Prepares and configures the AI harness base: creates requested directories,
   * configures enabled harnesses with embedded knowledge index, removes unselected
   * harnesses, consolidates pre-existing index.md, and persists settings.
   */
  public async configureHarnessBase(options: {
    active_harness?: string;
    enabled_harnesses: string[];
    create_folders?: string[];
    consolidate_index?: boolean;
    clean_unused?: boolean;
  }): Promise<any> {
    const response = await this.ipcClient.sendRequest<any>(
      'governance/configureHarnessBase',
      options
    );
    await this.getHarnessStatus();
    return response;
  }

  /**
   * Retrieves the skill catalog and procedure candidates (FR-021..FR-024).
   */
  public async getSkillCatalog(): Promise<SkillCatalogResponse> {
    return this.ipcClient.sendRequest<SkillCatalogResponse>(
      'governance/getSkillCatalog',
      {}
    );
  }

  /**
   * Analyzes physical directory structure metrics and organizational signals (FR-025, FR-026).
   */
  public async analyzeDirectoryStructure(): Promise<DirectoryStructureMetrics> {
    return this.ipcClient.sendRequest<DirectoryStructureMetrics>(
      'governance/analyzeDirectoryStructure',
      {}
    );
  }

  /**
   * Generates a structural governance transformation plan with deterministic approval token (FR-031).
   */
  public async generateGovernancePlan(
    strategyType: string,
    targetFileOrDir: string
  ): Promise<HarnessEvolutionRecommendation> {
    return this.ipcClient.sendRequest<HarnessEvolutionRecommendation>(
      'governance/generateGovernancePlan',
      {
        strategy_type: strategyType,
        target_file_or_dir: targetFileOrDir,
      }
    );
  }

  /**
   * Applies a structural governance plan with approval token validation (Invariant XVI).
   */
  public async applyGovernancePlan(
    planId: string,
    approvalToken: string
  ): Promise<ApplyGovernancePlanResponse> {
    return this.ipcClient.sendRequest<ApplyGovernancePlanResponse>(
      'governance/applyGovernancePlan',
      {
        plan_id: planId,
        approval_token: approvalToken,
      }
    );
  }
}
