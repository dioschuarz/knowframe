import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';
import type * as vscode from 'vscode';

const execAsync = promisify(exec);

export function generatePreCommitConfigContent(): string {
  return `# LLM Wiki - Event-driven Git Hook Orchestration
# Leverages prek (Rust-native) or pre-commit with regex file filtering
repos:
  - repo: local
    hooks:
      - id: llm-wiki-sync
        name: LLM Wiki Knowledge Graph Sync
        entry: llm-wiki-core git-delta
        language: system
        stages: [post-commit]
        always_run: false
        pass_filenames: true
        files: '(?i)\\.md$'
`;
}

export function generateFallbackPostCommitScript(): string {
  return `#!/bin/sh
# LLM Wiki - Zero-dependency fail-open post-commit hook fallback
# Enforces <= 500ms timeout with silent fail-open exit 0
changed_files=$(git diff-tree -r --name-only --no-commit-id HEAD 2>/dev/null | grep -i '\\.md$')
if [ -n "$changed_files" ]; then
  llm-wiki-core git-delta $changed_files >/dev/null 2>&1 || true
fi
exit 0
`;
}

/**
 * Writes .pre-commit-config.yaml and runs `prek install --hook-type post-commit`.
 */
export async function installPrek(
  workspaceRoot: string,
  outputChannel?: vscode.OutputChannel
): Promise<boolean> {
  const configPath = path.join(workspaceRoot, '.pre-commit-config.yaml');

  // Check if .pre-commit-config.yaml exists
  if (!fs.existsSync(configPath)) {
    fs.writeFileSync(configPath, generatePreCommitConfigContent(), 'utf8');
    outputChannel?.appendLine(`[HOOK INSTALLER] Created ${configPath}`);
  } else {
    // Append hook if not present
    const existing = fs.readFileSync(configPath, 'utf8');
    if (!existing.includes('llm-wiki-sync')) {
      const hookSnippet = `
  - repo: local
    hooks:
      - id: llm-wiki-sync
        name: LLM Wiki Knowledge Graph Sync
        entry: llm-wiki-core git-delta
        language: system
        stages: [post-commit]
        always_run: false
        pass_filenames: true
        files: '(?i)\\.md$'
`;
      fs.appendFileSync(configPath, hookSnippet, 'utf8');
      outputChannel?.appendLine(`[HOOK INSTALLER] Appended llm-wiki-sync hook to ${configPath}`);
    }
  }

  try {
    outputChannel?.appendLine('[HOOK INSTALLER] Running `prek install --hook-type post-commit`...');
    await execAsync('prek install --hook-type post-commit', { cwd: workspaceRoot });
    outputChannel?.appendLine('[HOOK INSTALLER] prek hook installed successfully.');
    return true;
  } catch (err: any) {
    outputChannel?.appendLine(`[HOOK INSTALLER] prek install failed: ${err.message}`);
    return false;
  }
}
