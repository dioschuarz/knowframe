import { describe, it } from 'node:test';
import assert from 'node:assert';
import { EngineRpcClient } from '../src/rpc/client.ts';

describe('JSON-RPC Client Test Suite (T057, FR-041)', () => {
  it('correctly dispatches and resolves JSON-RPC request by id', async () => {
    const client = new EngineRpcClient();
    const reqPromise = client.request('engine/getGraphTopology', { root: '.' });

    client.handleMessage(JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      result: { nodes: [], edges: [] },
    }));

    const result = await reqPromise;
    assert.deepStrictEqual(result, { nodes: [], edges: [] });
    client.dispose();
  });
});
