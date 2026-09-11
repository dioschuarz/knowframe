import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';
import type * as vscode from 'vscode';
import {
  generateFallbackPostCommitScript,
  generatePreCommitConfigContent,
  installPrek,
} from './prekInstaller';

const execAsync = promisify(exec);

export type HookInstallerTier = 'prek' | 'pre-commit' | 'script';

export interface HookInstallResult {
  tier: HookInstallerTier;
  success: boolean;
  message: string;
}

/**
 * Executes 3-tier fallback hook installation:
 * Tier 1: prek (Rust-native orchestrator)
 * Tier 2: pre-commit (Python/standard orchestrator)
 * Tier 3: Direct .git/hooks/post-commit shell script fallback
 */
export async function installGitHooksWithFallback(
  workspaceRoot: string,
  preferredEngine?: string,
  outputChannel?: vscode.OutputChannel
): Promise<HookInstallResult> {
  const gitDir = path.join(workspaceRoot, '.git');
  if (!fs.existsSync(gitDir)) {
    return {
      tier: 'script',
      success: false,
      message: 'Not a Git repository (no .git directory found).',
    };
  }

  // Tier 1: Try prek if preferred or auto-detected
  if (!preferredEngine || preferredEngine === 'prek') {
    outputChannel?.appendLine('[HOOK INSTALLER] Attempting Tier 1: prek...');
    const prekInstalled = await checkCommandExists('prek');
    if (prekInstalled) {
      const ok = await installPrek(workspaceRoot, outputChannel);
      if (ok) {
        return {
          tier: 'prek',
          success: true,
          message: 'Installed Git hooks using Rust-native prek orchestrator.',
        };
      }
    } else {
      outputChannel?.appendLine('[HOOK INSTALLER] `prek` executable not found in PATH.');
    }
  }

  // Tier 2: Try pre-commit
  if (!preferredEngine || preferredEngine === 'pre-commit' || preferredEngine === 'prek') {
    outputChannel?.appendLine('[HOOK INSTALLER] Attempting Tier 2: pre-commit...');
    const preCommitInstalled = await checkCommandExists('pre-commit');
    if (preCommitInstalled) {
      try {
        const configPath = path.join(workspaceRoot, '.pre-commit-config.yaml');
        if (!fs.existsSync(configPath)) {
          fs.writeFileSync(configPath, generatePreCommitConfigContent(), 'utf8');
        }
        await execAsync('pre-commit install --hook-type post-commit', { cwd: workspaceRoot });
        outputChannel?.appendLine('[HOOK INSTALLER] Installed via pre-commit.');
        return {
          tier: 'pre-commit',
          success: true,
          message: 'Installed Git hooks using standard pre-commit orchestrator.',
        };
      } catch (err: any) {
        outputChannel?.appendLine(`[HOOK INSTALLER] pre-commit install failed: ${err.message}`);
      }
    } else {
      outputChannel?.appendLine('[HOOK INSTALLER] `pre-commit` executable not found in PATH.');
    }
  }

  // Tier 3: Shell script fallback
  outputChannel?.appendLine('[HOOK INSTALLER] Falling back to Tier 3: direct .git/hooks/post-commit script...');
  try {
    const hooksDir = path.join(gitDir, 'hooks');
    fs.mkdirSync(hooksDir, { recursive: true });
    const postCommitPath = path.join(hooksDir, 'post-commit');

    fs.writeFileSync(postCommitPath, generateFallbackPostCommitScript(), {
      encoding: 'utf8',
      mode: 0o755,
    });

    outputChannel?.appendLine(`[HOOK INSTALLER] Created executable script at ${postCommitPath}`);
    return {
      tier: 'script',
      success: true,
      message: 'Installed standalone zero-dependency .git/hooks/post-commit hook.',
    };
  } catch (err: any) {
    return {
      tier: 'script',
      success: false,
      message: `Failed to write .git/hooks/post-commit: ${err.message}`,
    };
  }
}

async function checkCommandExists(cmd: string): Promise<boolean> {
  try {
    await execAsync(`which ${cmd}`);
    return true;
  } catch {
    return false;
  }
}
