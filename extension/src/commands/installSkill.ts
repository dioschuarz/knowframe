import * as fs from 'fs';
import * as path from 'path';
import * as vscode from 'vscode';

const SKILL_RELATIVE_PATH = path.join('.agents', 'skills', 'llm_wiki_extension', 'SKILL.md');

export function getLLMWikiSkillPath(workspaceRoot: string): string {
  return path.join(workspaceRoot, SKILL_RELATIVE_PATH);
}

export function isLLMWikiSkillAvailable(workspaceRoot: string): boolean {
  return fs.existsSync(getLLMWikiSkillPath(workspaceRoot));
}

export function installLLMWikiSkill(
  workspaceRoot: string,
  extensionPath: string,
  outputChannel?: vscode.OutputChannel,
): void {
  const sourcePath = path.join(extensionPath, 'skills', 'llm_wiki_extension', 'SKILL.md');
  const destinationPath = getLLMWikiSkillPath(workspaceRoot);
  if (!fs.existsSync(sourcePath)) {
    throw new Error(`Bundled LLM-Wiki skill not found at ${sourcePath}`);
  }
  fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
  fs.copyFileSync(sourcePath, destinationPath);
  outputChannel?.appendLine(`[SKILL INSTALLER] Installed llm-wiki-extension skill at ${destinationPath}`);
}
