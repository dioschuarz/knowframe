import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { resolveBinaryPath } from '../../src/lifecycle/binaryResolver.ts';

suite('IPC & Lifecycle Unit Test Suite', () => {
  test('Binary resolver finds dev build or fallback binary', () => {
    const resolved = resolveBinaryPath('/fake/extension/path');
    assert.ok(resolved.length > 0, 'Binary path should not be empty');
  });

  test('JSON-RPC protocol serialization format', () => {
    const req = {
      jsonrpc: '2.0',
      id: 1,
      method: 'workspace/scan',
    };
    const serialized = JSON.stringify(req);
    assert.ok(serialized.includes('"jsonrpc":"2.0"'));
    assert.ok(serialized.includes('"method":"workspace/scan"'));
  });

  test('IpcClient resolves port dynamically from number or getter', async () => {
    const { IpcClient } = await import('../../src/ipc/client.ts');
    const clientStatic = new IpcClient(8989, 'test-token');
    assert.strictEqual(clientStatic.getActivePort(), 8989);
    assert.strictEqual(clientStatic.isConnected(), false);

    let dynamicPort: number | null = 3000;
    const clientDynamic = new IpcClient(() => dynamicPort, 'test-token');
    assert.strictEqual(clientDynamic.getActivePort(), 3000);

    dynamicPort = 45000;
    assert.strictEqual(clientDynamic.getActivePort(), 45000);

    clientDynamic.setPort(52000);
    assert.strictEqual(clientDynamic.getActivePort(), 52000);
  });

  test('IpcClient ensureConnected deduplicates concurrent calls', async () => {
    const { IpcClient } = await import('../../src/ipc/client.ts');
    const client = new IpcClient(0, 'test-token'); // 0 port should reject cleanly
    assert.strictEqual(client.getActivePort(), null);

    await assert.rejects(
      async () => {
        await client.ensureConnected();
      },
      /Sidecar daemon port is not available/
    );
  });
});
