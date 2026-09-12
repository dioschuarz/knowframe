import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { AdmissionManager } from '../../src/services/admissionManager.ts';
import type { IpcClient } from '../../src/ipc/client.ts';

suite('AdmissionManager Unit Test Suite', () => {
  function createMockIpcClient(responses: Record<string, unknown>): IpcClient {
    const sentRequests: { method: string; params: unknown }[] = [];
    return {
      sendRequest: async (method: string, params?: unknown) => {
        sentRequests.push({ method, params });
        if (method in responses) {
          return responses[method];
        }
        throw new Error(`Unexpected method: ${method}`);
      },
      getSentRequests: () => sentRequests,
    } as unknown as IpcClient;
  }

  test('validateCandidate invokes workspace/validateCandidate RPC', async () => {
    const mockReport = {
      base_commit: 'abc1234',
      decision: 'pass',
      exit_code: 0,
      summary: { total_debt: 0, legacy_debt: 0, new_debt: 0, resolved_debt: 0, regressions: 0 },
      regressions: [],
      resolved_findings: [],
      warnings: [],
    };
    const client = createMockIpcClient({ 'workspace/validateCandidate': mockReport });
    const manager = new AdmissionManager(client);

    const result = await manager.validateCandidate({
      staged_files: [{ path: 'docs/test.md', status: 'modified' }],
    });

    assert.strictEqual(result.decision, 'pass');
    assert.strictEqual(result.exit_code, 0);
  });

  test('getBaseline invokes workspace/getBaseline RPC', async () => {
    const mockBaseline = {
      baseline_commit: 'abc1234',
      timestamp: '2026-09-12T00:00:00Z',
      broken_links_count: 2,
      orphan_nodes_count: 1,
      collisions_count: 0,
      drifts_count: 0,
      total_debt: 3,
    };
    const client = createMockIpcClient({ 'workspace/getBaseline': mockBaseline });
    const manager = new AdmissionManager(client);

    const result = await manager.getBaseline();
    assert.strictEqual(result.total_debt, 3);
    assert.strictEqual(result.broken_links_count, 2);
  });

  test('updateBaseline invokes workspace/updateBaseline RPC', async () => {
    const mockUpdate = {
      baseline_commit: 'def5678',
      total_debt: 1,
      previous_total_debt: 3,
    };
    const client = createMockIpcClient({ 'workspace/updateBaseline': mockUpdate });
    const manager = new AdmissionManager(client);

    const result = await manager.updateBaseline({ force_new_commit: 'def5678' });
    assert.strictEqual(result.baseline_commit, 'def5678');
    assert.strictEqual(result.total_debt, 1);
  });

  test('traceContext invokes workspace/traceContext RPC', async () => {
    const mockTrace = {
      query: 'Auth Architecture',
      entrypoint: 'AGENTS.md',
      target_node: 'auth.md',
      path: ['AGENTS.md', 'security.md', 'auth.md'],
      depth: 2,
      path_quality_score: 95.0,
      cumulative_bpe_tokens: 650,
      relevant_tokens: 300,
      weak_intermediate_nodes: [],
      latency_ms: 12,
    };
    const client = createMockIpcClient({ 'workspace/traceContext': mockTrace });
    const manager = new AdmissionManager(client);

    const result = await manager.traceContext({ query: 'Auth Architecture' });
    assert.strictEqual(result.target_node, 'auth.md');
    assert.strictEqual(result.depth, 2);
    assert.strictEqual(result.path_quality_score, 95.0);
  });

  test('getQualityScore invokes workspace/getQualityScore RPC', async () => {
    const mockQuality = {
      overall_score: 92.5,
      integrity_score: 90.0,
      consistency_score: 95.0,
      discoverability_score: 92.0,
      freshness_score: 94.0,
      legacy_debt_count: 2,
      regressions_count: 0,
      resolved_debt_count: 1,
      top_degradation_factors: [],
    };
    const client = createMockIpcClient({ 'workspace/getQualityScore': mockQuality });
    const manager = new AdmissionManager(client);

    const result = await manager.getQualityScore();
    assert.strictEqual(result.overall_score, 92.5);
    assert.strictEqual(result.regressions_count, 0);
  });
});
