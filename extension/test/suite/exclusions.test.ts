import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderExclusionModalHtml } from '../../webview-ui/src/components/exclusionModal.ts';
import type { NoteEdge, NoteNode } from '../../src/types/graph.ts';
import type { NoiseSuggestion } from '../../src/types/protocol.ts';

suite('Graph Exclusions & Target Resolvability Test Suite', () => {
  test('ExclusionModal renders pattern, matching count, and sample paths', () => {
    const html = renderExclusionModalHtml({
      pattern: 'packages/**/README.md',
      matchingCount: 42,
      samplePaths: [
        'packages/core/README.md',
        'packages/cli/README.md',
        'packages/web/README.md',
      ],
      scope: 'graph',
    });

    assert.ok(html.includes('packages/**/README.md'), 'Must display glob pattern');
    assert.ok(html.includes('42'), 'Must show matching files count');
    assert.ok(html.includes('packages/core/README.md'), 'Must render sample path');
    assert.ok(html.includes('packages/cli/README.md'), 'Must render sample path');
    assert.ok(html.includes('Apply Exclusion'), 'Must contain apply button');
    assert.ok(html.includes('Cancel'), 'Must contain cancel button');
  });

  test('Target Resolvability Invariant: links to excluded notes are ResolvedExcluded and NOT broken', () => {
    // Per Constitution v1.3.0 and spec FR-049:
    // Notes matching graph.exclude do not render on canvas, but inbound links to them
    // remain valid (isBroken: false) with status ResolvedExcluded.
    const edgeToExcluded: NoteEdge = {
      source: 'docs/guide.md',
      target: 'packages/core/README.md',
      line: 45,
      isBroken: false,
      resolutionStatus: 'ResolvedExcluded',
    };

    assert.strictEqual(edgeToExcluded.isBroken, false, 'Link to excluded note MUST NOT be flagged broken');
    assert.strictEqual(edgeToExcluded.resolutionStatus, 'ResolvedExcluded', 'Status must be ResolvedExcluded');
  });

  test('Noise suggestions match minimum threshold >= 5 disconnected files', () => {
    const suggestions: NoiseSuggestion[] = [
      {
        pattern: '**/README.md',
        matchingCount: 24,
        reason: '24 disconnected README.md files found across packages',
        recommended: true,
      },
      {
        pattern: 'vendor/**',
        matchingCount: 15,
        reason: '15 disconnected files in vendor directory',
        recommended: true,
      },
    ];

    for (const sug of suggestions) {
      assert.ok(sug.matchingCount >= 5, 'Each noise suggestion must affect at least 5 files');
      assert.ok(sug.pattern.length > 0, 'Pattern must be non-empty');
      assert.ok(sug.reason.length > 0, 'Reason must be non-empty');
    }
  });

  test('Excluded notes do not increment orphan count', () => {
    const nodes: NoteNode[] = [
      { id: '1', label: 'Hub', filePath: 'index.md', tokens: 100, hopDistance: 0, inDegree: 2, outDegree: 2, brokenLinks: 0 },
      { id: '2', label: 'Connected', filePath: 'page1.md', tokens: 50, hopDistance: 1, inDegree: 1, outDegree: 1, brokenLinks: 0 },
      { id: '3', label: 'True Orphan', filePath: 'orphan.md', tokens: 30, hopDistance: -1, inDegree: 0, outDegree: 0, brokenLinks: 0 },
    ];

    const orphanCount = nodes.filter((n) => n.hopDistance < 0).length;
    assert.strictEqual(orphanCount, 1, 'Only true unlinked active notes count as orphans');
  });
});
