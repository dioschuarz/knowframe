import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { AiReviewDispatcher } from '../../src/governance/aiReviewDispatcher.ts';
import { HarnessToolService } from '../../src/governance/harnessToolService.ts';
import type { HarnessToolDefinition } from '../../src/governance/harnessToolService.ts';

suite('AiReviewDispatcher Unit Test Suite', () => {
  test('buildPrompt generates single-file, single-issue targeted prompt adhering to isolation invariant', () => {
    const dispatcher = new AiReviewDispatcher();
    const prompt = dispatcher.buildPrompt({
      filePath: 'notes/01_projects/core.md',
      issue: {
        category: 'broken_link',
        line_number: 42,
        description: "Target 'notes/missing.md' not found",
      },
    });

    assert.ok(prompt.includes("Review and fix the broken_link issue in 'notes/01_projects/core.md' at line 42"));
    assert.ok(prompt.includes("IMPORTANT: Modify ONLY 'notes/01_projects/core.md' and fix ONLY this issue"));
    assert.ok(prompt.includes("Re-check until the broken_link issue in 'notes/01_projects/core.md' is completely resolved"));
  });

  test('buildPrompt generates full workspace review prompt when all=true', () => {
    const dispatcher = new AiReviewDispatcher();
    const prompt = dispatcher.buildPrompt({ all: true });

    assert.ok(prompt.includes('Review all knowledge base health and integrity issues'));
    assert.ok(prompt.includes('Fix broken links, unanchored references, and review obligations'));
  });

  test('buildCommand correctly formats agent-specific CLI commands and escapes quotes', () => {
    const dispatcher = new AiReviewDispatcher();
    const prompt = 'Fix "link" in file.md';

    const agyCmd = dispatcher.buildCommand('agy', prompt);
    assert.strictEqual(agyCmd, 'agy -i "Fix \\"link\\" in file.md"');

    const codexCmd = dispatcher.buildCommand('codex', prompt);
    assert.strictEqual(codexCmd, 'codex "Fix \\"link\\" in file.md"');

    const claudeCmd = dispatcher.buildCommand('claude', prompt);
    assert.strictEqual(claudeCmd, 'claude "Fix \\"link\\" in file.md"');

    const cursorCmd = dispatcher.buildCommand('cursor', prompt);
    assert.strictEqual(cursorCmd, 'cursor --agent "Fix \\"link\\" in file.md"');

    const ghCmd = dispatcher.buildCommand('gh', prompt);
    assert.strictEqual(ghCmd, 'gh copilot suggest "Fix \\"link\\" in file.md"');
  });

  test('getTargetAgent detects installed agent CLI based on mock toolService', async () => {
    const mockTools: HarnessToolDefinition[] = [
      {
        id: 'agy',
        name: 'Antigravity CLI (Google DeepMind)',
        category: 'agent',
        binary: 'agy',
        description: 'Antigravity CLI',
        installCommand: 'uv tool install antigravity-cli',
      },
      {
        id: 'codex',
        name: 'Codex CLI (OpenAI)',
        category: 'agent',
        binary: 'codex',
        description: 'Codex CLI',
        installCommand: 'npm install -g @openai/codex',
      },
    ];

    const mockExecutor = async (cmd: string) => {
      if (cmd.includes('which agy')) return { stdout: '/usr/local/bin/agy\n', stderr: '' };
      if (cmd.includes('which codex')) return { stdout: '/usr/local/bin/codex\n', stderr: '' };
      throw new Error('Not found');
    };

    const toolService = new HarnessToolService(mockTools, mockExecutor);
    const dispatcher = new AiReviewDispatcher(toolService);

    const agent = await dispatcher.getTargetAgent();
    assert.ok(agent !== null, 'Must find an installed agent');
    assert.strictEqual(agent.id, 'agy', 'Must pick agy based on priority order');
  });
});
