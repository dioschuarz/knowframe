import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { filterGraphByFocus } from '../../webview-ui/src/graph/focusEngine.ts';
import type { NoteNode, NoteEdge, FocusMode } from '../../src/types/graph.ts';

suite('FocusEngine Subgraph Filtering & Hop Horizon Test Suite', () => {
  const sampleNodes: NoteNode[] = [
    { id: 'hub', label: 'Hub Note', filePath: 'hub.md', tokens: 1000, hopDistance: 0, inDegree: 2, outDegree: 2, brokenLinks: 0 },
    { id: 'hop1', label: 'Hop 1 Note', filePath: 'hop1.md', tokens: 500, hopDistance: 1, inDegree: 1, outDegree: 1, brokenLinks: 0 },
    { id: 'hop2', label: 'Hop 2 Note', filePath: 'hop2.md', tokens: 300, hopDistance: 2, inDegree: 1, outDegree: 1, brokenLinks: 0 },
    { id: 'hop3', label: 'Hop 3 Note', filePath: 'hop3.md', tokens: 200, hopDistance: 3, inDegree: 1, outDegree: 1, brokenLinks: 0 },
    { id: 'orphan', label: 'Orphan Note', filePath: 'orphan.md', tokens: 150, hopDistance: -1, inDegree: 0, outDegree: 0, brokenLinks: 0 },
  ];

  const sampleEdges: NoteEdge[] = [
    { source: 'hub', target: 'hop1', isBroken: false },
    { source: 'hop1', target: 'hop2', isBroken: false },
    { source: 'hop2', target: 'hop3', isBroken: false },
  ];

  test('FocusMode "hub" keeps only Level 0 Hub node', () => {
    const res = filterGraphByFocus(sampleNodes, sampleEdges, 'hub', true);
    assert.strictEqual(res.nodes.length, 1);
    assert.strictEqual(res.nodes[0].id, 'hub');
    assert.strictEqual(res.edges.length, 0);
  });

  test('FocusMode "1_hop" includes Hub (0) and direct neighbors (1)', () => {
    const res = filterGraphByFocus(sampleNodes, sampleEdges, '1_hop', false);
    const ids = res.nodes.map((n) => n.id);
    assert.deepStrictEqual(ids.sort(), ['hop1', 'hub']);
    assert.strictEqual(res.edges.length, 1);
    assert.strictEqual(res.edges[0].source, 'hub');
    assert.strictEqual(res.edges[0].target, 'hop1');
  });

  test('FocusMode "2_hops" includes Hub (0), Level 1, and Level 2', () => {
    const res = filterGraphByFocus(sampleNodes, sampleEdges, '2_hops', false);
    const ids = res.nodes.map((n) => n.id);
    assert.deepStrictEqual(ids.sort(), ['hop1', 'hop2', 'hub']);
    assert.strictEqual(res.edges.length, 2);
  });

  test('FocusMode "all" with showOrphans=true includes all nodes and edges', () => {
    const res = filterGraphByFocus(sampleNodes, sampleEdges, 'all', true);
    assert.strictEqual(res.nodes.length, 5);
    assert.strictEqual(res.edges.length, 3);
  });

  test('showOrphans=false excludes unlinked orphan notes in "all" mode', () => {
    const res = filterGraphByFocus(sampleNodes, sampleEdges, 'all', false);
    const ids = res.nodes.map((n) => n.id);
    assert.ok(!ids.includes('orphan'), 'Orphan must be excluded when showOrphans=false');
    assert.strictEqual(res.nodes.length, 4);
    assert.strictEqual(res.edges.length, 3);
  });
});
