import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  HarnessToolService,
  CANONICAL_HARNESS_TOOLS,
  type HarnessToolDefinition,
} from '../../src/governance/harnessToolService.ts';

suite('HarnessToolService Unit Test Suite', () => {
  test('registers all canonical harness tools including prek, uv, ai-memory, notebooklm, and agents', () => {
    const service = new HarnessToolService();
    const tools = service.getRegisteredTools();

    assert.strictEqual(tools.length, 10, 'Should register 10 canonical harness tools');

    const ids = tools.map((t) => t.id);
    assert.ok(ids.includes('prek'), 'Must include prek for hook admission control');
    assert.ok(ids.includes('uv'), 'Must include uv for Rule 10 Python tooling');
    assert.ok(ids.includes('ai_memory'), 'Must include ai-memory for AGENTS.md cross-session memory');
    assert.ok(ids.includes('notebooklm'), 'Must include notebooklm for zero-token grounded RAG');
    assert.ok(ids.includes('claude'), 'Must include Claude Code agent');
    assert.ok(ids.includes('agy'), 'Must include Antigravity CLI agent');
    assert.ok(ids.includes('codex'), 'Must include Codex CLI agent');
    assert.ok(ids.includes('cursor'), 'Must include Cursor CLI');
    assert.ok(ids.includes('gh'), 'Must include GitHub Copilot CLI');
    assert.ok(ids.includes('graphify'), 'Must include graphify for optional code diagnostics');
  });

  test('detectTool returns installed status, path, and parsed version when command succeeds', async () => {
    const mockExecutor = async (cmd: string) => {
      if (cmd.startsWith('which prek')) {
        return { stdout: '/home/user/.cargo/bin/prek\n', stderr: '' };
      }
      if (cmd.startsWith('prek --version')) {
        return { stdout: 'prek 0.2.4\n', stderr: '' };
      }
      throw new Error(`Command not mocked: ${cmd}`);
    };

    const service = new HarnessToolService(CANONICAL_HARNESS_TOOLS, mockExecutor);
    const prekDef = CANONICAL_HARNESS_TOOLS.find((t) => t.id === 'prek')!;
    const status = await service.detectTool(prekDef);

    assert.strictEqual(status.id, 'prek');
    assert.strictEqual(status.is_installed, true);
    assert.strictEqual(status.bin_path, '/home/user/.cargo/bin/prek');
    assert.strictEqual(status.version, '0.2.4');
  });

  test('detectTool returns not installed when which fails', async () => {
    const mockExecutor = async (cmd: string) => {
      throw new Error(`Command failed: ${cmd} (not found)`);
    };

    const service = new HarnessToolService(CANONICAL_HARNESS_TOOLS, mockExecutor);
    const claudeDef = CANONICAL_HARNESS_TOOLS.find((t) => t.id === 'claude')!;
    const status = await service.detectTool(claudeDef);

    assert.strictEqual(status.id, 'claude');
    assert.strictEqual(status.is_installed, false);
    assert.strictEqual(status.bin_path, null);
    assert.strictEqual(status.version, null);
  });

  test('detectAllTools checks all registered tools in parallel', async () => {
    const mockExecutor = async (cmd: string) => {
      if (cmd.includes('uv')) {
        if (cmd.startsWith('which')) return { stdout: '/usr/local/bin/uv\n', stderr: '' };
        return { stdout: 'uv 0.5.11 (c7b9e3a 2025-01-15)\n', stderr: '' };
      }
      throw new Error('Not found');
    };

    const service = new HarnessToolService(CANONICAL_HARNESS_TOOLS, mockExecutor);
    const statuses = await service.detectAllTools();

    assert.strictEqual(statuses.length, 10);
    const uvStatus = statuses.find((s) => s.id === 'uv')!;
    assert.strictEqual(uvStatus.is_installed, true);
    assert.strictEqual(uvStatus.version, '0.5.11');

    const prekStatus = statuses.find((s) => s.id === 'prek')!;
    assert.strictEqual(prekStatus.is_installed, false);
  });

  test('installTool launches terminal command via terminalLauncher with transparent naming', async () => {
    let launchedName = '';
    let launchedCommand = '';

    const service = new HarnessToolService();
    const success = await service.installTool('prek', {
      terminalLauncher: (name, command) => {
        launchedName = name;
        launchedCommand = command;
      },
    });

    assert.strictEqual(success, true);
    assert.ok(launchedName.includes('prek'), 'Terminal name must mention prek');
    assert.strictEqual(launchedCommand, 'cargo install prek');
  });

  test('installTool returns false for unknown tool id', async () => {
    const service = new HarnessToolService();
    const success = await service.installTool('non_existent_tool_xyz');
    assert.strictEqual(success, false);
  });

  test('isPrekInstalled delegates accurately to prek status', async () => {
    const mockExecutorSuccess = async () => ({ stdout: '/bin/prek\n', stderr: '' });
    const serviceSuccess = new HarnessToolService(CANONICAL_HARNESS_TOOLS, mockExecutorSuccess);
    assert.strictEqual(await serviceSuccess.isPrekInstalled(), true);

    const mockExecutorFail = async () => { throw new Error('Not found'); };
    const serviceFail = new HarnessToolService(CANONICAL_HARNESS_TOOLS, mockExecutorFail);
    assert.strictEqual(await serviceFail.isPrekInstalled(), false);
  });
});
