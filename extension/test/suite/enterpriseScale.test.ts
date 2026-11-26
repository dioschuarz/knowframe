import { describe, it } from 'node:test';
import * as assert from 'node:assert/strict';
import {
  buildSlimSidebarScanPayload,
  buildSlimSidebarIssuesPayload,
} from '../../src/webview/sidebarRenderer.ts';
import { buildGraphWebviewPayload } from '../../src/webview/graphPayloadProjector.ts';
import type { ScanResultPayload } from '../../src/types/protocol.ts';

describe('Enterprise Scale Payload Projection Test Suite (20,000+ docs)', () => {
  function createEnterpriseScanPayload(nodeCount: number): ScanResultPayload {
    const nodes: any[] = [];
    const edges: any[] = [];

    for (let i = 0; i < nodeCount; i++) {
      const id = i === 0 ? 'AGENTS.md' : `docs/section_${Math.floor(i / 500)}/doc_${i}.md`;
      nodes.push({
        id,
        file_path: `/enterprise/${id}`,
        label: i === 0 ? 'AGENTS' : `Doc ${i}`,
        tags: ['enterprise'],
        tokens: 800,
        in_degree: i === 0 ? 100 : (i < 50 ? 20 : 1),
        out_degree: i === 0 ? 50 : 2,
        betweenness: i === 0 ? 0.95 : (i < 50 ? 0.2 : 0.001),
        hop_distance: i === 0 ? 0 : (i <= 50 ? 1 : (i <= 600 ? 2 : -1)),
        status: i <= 600 ? 'active' : 'orphan',
        color: '#64748B',
        broken_links: 0,
        fragments: [
          { anchor: 'intro', heading: 'Intro', start_line: 1, end_line: 20, tokens: 400, inbound: 0, outbound: 1 },
          { anchor: 'details', heading: 'Details', start_line: 21, end_line: 60, tokens: 400, inbound: 0, outbound: 1 },
        ],
      });
      if (i > 0 && i <= 600) {
        const src = i <= 50 ? 'AGENTS.md' : `docs/section_0/doc_${(i % 50) + 1}.md`;
        edges.push({
          id: `e-${i}`,
          source: src,
          target: id,
          is_broken: false,
          link_type: 'markdown',
        });
      }
    }

    return {
      nodes,
      edges,
      recommended_hub: 'AGENTS.md',
      token_summary: {
        active_harness_tokens: 800,
        subcall_accessible_tokens: 480000,
        warning_threshold: 60000,
        critical_threshold: 100000,
        status: 'healthy',
      },
      observatory_summary: {
        total_notes: nodeCount,
        connected_notes: 601,
        orphan_notes: nodeCount - 601,
        excluded_notes: 0,
        broken_links_count: 0,
        active_tokens: 800,
        reachable_tokens: 480000,
        last_sync_timestamp: Date.now(),
      } as any,
    };
  }

  it('compacts a 20,000-node scan payload to < 2 KB for the Sidebar while preserving totals and top-5 hubs', () => {
    const scan = createEnterpriseScanPayload(20_000);
    const slim = buildSlimSidebarScanPayload(scan)!;
    const serializedBytes = Buffer.byteLength(JSON.stringify(slim), 'utf8');

    assert.equal(slim.total_nodes, 20_000);
    assert.equal(slim.total_edges, 600);
    assert.equal(slim.connected_nodes, 601);
    assert.equal(slim.orphan_nodes, 20_000 - 601);
    assert.equal(slim.recommended_hub, 'AGENTS.md');
    assert.equal(slim.top_nodes.length, 5);
    assert.equal(slim.top_nodes[0].id, 'AGENTS.md');
    assert.ok(serializedBytes < 2048, `Expected slim sidebar payload < 2 KB, got ${serializedBytes} bytes`);
  });

  it('strips raw issue arrays from Sidebar issues payload to prevent 200MB HTML serialization', () => {
    const hugeIssues = Array.from({ length: 50_000 }, (_, i) => ({
      id: `issue-${i}`,
      pillar: 'integrity',
      category: 'non_clickable_link',
      file_path: `docs/doc_${i}.md`,
      deduction_points: 0.5,
      weight: 1.0,
      description: 'Non-clickable link',
      remediation_hint: 'Convert to Markdown',
      is_legacy: false,
    }));
    const slim = buildSlimSidebarIssuesPayload({
      overall_score: 72.5,
      total_issues: 50_000,
      active_issues_count: 50_000,
      legacy_debt_count: 0,
      integrity_count: 50_000,
      issues: hugeIssues,
    } as any)!;

    assert.equal((slim as any).issues, undefined);
    assert.equal(slim.total_issues, 50_000);
    assert.equal(slim.active_issues_count, 50_000);
    assert.ok(Buffer.byteLength(JSON.stringify(slim), 'utf8') < 1024);
  });

  it('applies Enterprise Focus Windowing at 2,000 nodes and strips fragments for the Graph Webview', () => {
    const scan = createEnterpriseScanPayload(20_000);
    const projected = buildGraphWebviewPayload(scan, 2000);

    assert.equal(projected.enterpriseWindow.truncated, true);
    assert.equal(projected.enterpriseWindow.totalNodes, 20_000);
    assert.equal(projected.enterpriseWindow.displayedNodes, 2000);
    assert.equal(projected.nodes.length, 2000);
    // Central hub (hop 0) MUST be first
    assert.equal(projected.nodes[0].id, 'AGENTS.md');
    // Fragments MUST be stripped from canvas node payloads
    assert.equal((projected.nodes[0] as any).fragments, undefined);
    // All 600 connected edges (within hop 0..2) MUST be preserved
    assert.equal(projected.edges.length, 600);
  });
});
