import { describe, it } from 'node:test';
import assert from 'node:assert';
import { EngineRpcClient } from '../../src/rpc/client.ts';

describe('JSON-RPC Client Test Suite (T057, FR-041)', () => {
  it('correctly dispatches and resolves JSON-RPC request by id', async () => {
    const client = new EngineRpcClient();
    const reqPromise = client.request('engine/getGraphTopology', { root: '.' });

    // Simulate stdout response from sidecar
    client.handleMessage(JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      result: { nodes: [], edges: [] },
    }));

    const result = await reqPromise;
    assert.deepStrictEqual(result, { nodes: [], edges: [] });
    client.dispose();
  });

  it('correctly rejects on JSON-RPC error response', async () => {
    const client = new EngineRpcClient();
    const reqPromise = client.request('engine/unknownMethod', {});

    client.handleMessage(JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      error: { code: -32601, message: 'Method not found' },
    }));

    await assert.rejects(reqPromise, /Method not found/);
    client.dispose();
  });

  it('rejects pending requests upon disposal', async () => {
    const client = new EngineRpcClient();
    const reqPromise = client.request('engine/slowMethod', {});
    client.dispose();
    await assert.rejects(reqPromise, /disposed/i);
  });
});
