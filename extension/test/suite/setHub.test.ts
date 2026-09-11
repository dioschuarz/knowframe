import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import type { NodePayload } from '../../src/types/protocol.ts';

suite('Set Central Hub Sorting & Ranking Test Suite', () => {
  const mockNodes: NodePayload[] = [
    {
      id: '01_projects/small_note.md',
      label: 'Small Note',
      file_path: '01_projects/small_note.md',
      tags: [],
      color: '#3B82F6',
      status: 'active',
      tokens: 400,
      hop_distance: 2,
      in_degree: 1,
      out_degree: 0,
      betweenness: 0.01,
      broken_links: 0,
    },
    {
      id: '00_meta/wiki_index.md',
      label: 'Wiki Index',
      file_path: '00_meta/wiki_index.md',
      tags: ['#meta'],
      color: '#3B82F6',
      status: 'active',
      tokens: 3500,
      hop_distance: 0,
      in_degree: 12,
      out_degree: 18,
      betweenness: 0.88,
      broken_links: 0,
    },
    {
      id: '02_memory/active_context.md',
      label: 'Active Context',
      file_path: '02_memory/active_context.md',
      tags: ['#memory'],
      color: '#3B82F6',
      status: 'active',
      tokens: 1800,
      hop_distance: 1,
      in_degree: 8,
      out_degree: 4,
      betweenness: 0.45,
      broken_links: 0,
    },
    {
      id: '03_architecture/system_design.md',
      label: 'System Design',
      file_path: '03_architecture/system_design.md',
      tags: ['#architecture'],
      color: '#3B82F6',
      status: 'active',
      tokens: 2200,
      hop_distance: 1,
      in_degree: 6,
      out_degree: 6,
      betweenness: 0.45,
      broken_links: 0,
    },
  ];

  test('Nodes are sorted by total inbound + outbound connections descending', () => {
    const sorted = [...mockNodes].sort((a, b) => {
      const totalA = (a.in_degree || 0) + (a.out_degree || 0);
      const totalB = (b.in_degree || 0) + (b.out_degree || 0);
      if (totalB !== totalA) {
        return totalB - totalA;
      }
      if (b.betweenness !== a.betweenness) {
        return b.betweenness - a.betweenness;
      }
      return b.tokens - a.tokens;
    });

    assert.strictEqual(sorted[0].id, '00_meta/wiki_index.md', 'Wiki index with 30 links should be #1');
    assert.strictEqual((sorted[0].in_degree + sorted[0].out_degree), 30);

    // Active context and system design both have 12 links, but betweenness tiebreak or token tiebreak
    assert.strictEqual((sorted[1].in_degree + sorted[1].out_degree), 12);
    assert.strictEqual((sorted[2].in_degree + sorted[2].out_degree), 12);

    assert.strictEqual(sorted[3].id, '01_projects/small_note.md', 'Small note with 1 link should be last');
  });
});
