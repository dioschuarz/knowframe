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
});
