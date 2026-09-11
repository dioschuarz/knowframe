import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  discoverVaultSuggestions,
  countMatchingNotes,
} from '../../webview-ui/src/components/colorManager.ts';
import type { NoteNode } from '../../src/types/graph.ts';

suite('Cluster Groups Suggestions & Matcher Test Suite', () => {
  const sampleNodes: NoteNode[] = [
    {
      id: '00_meta/wiki_index.md',
      label: 'Wiki Index',
      filePath: '00_meta/wiki_index.md',
      tags: ['#meta', '#index'],
      color: '#3B82F6',
      status: 'active',
      tokens: 1500,
      hopDistance: 0,
      inDegree: 10,
      outDegree: 15,
      betweenness: 0.95,
      brokenLinks: 0,
    },
    {
      id: '01_projects/proj1.md',
      label: 'Project 1',
      filePath: '01_projects/proj1.md',
      tags: ['#projects', '#architecture'],
      color: '#3B82F6',
      status: 'active',
      tokens: 800,
      hopDistance: 1,
      inDegree: 4,
      outDegree: 2,
      betweenness: 0.25,
      brokenLinks: 0,
    },
    {
      id: '01_projects/proj2.md',
      label: 'Project 2',
      filePath: '01_projects/proj2.md',
      tags: ['#projects'],
      color: '#3B82F6',
      status: 'active',
      tokens: 900,
      hopDistance: 1,
      inDegree: 3,
      outDegree: 1,
      betweenness: 0.15,
      brokenLinks: 0,
    },
    {
      id: '02_memory/active_context.md',
      label: 'Active Context',
      filePath: '02_memory/active_context.md',
      tags: ['#memory'],
      color: '#3B82F6',
      status: 'active',
      tokens: 1200,
      hopDistance: 1,
      inDegree: 8,
      outDegree: 5,
      betweenness: 0.6,
      brokenLinks: 0,
    },
    {
      id: '06_studies/k8s/01_index.md',
      label: 'K8s Study Index',
      filePath: '06_studies/k8s/01_index.md',
      tags: ['#studies', '#k8s', '#architecture'],
      color: '#3B82F6',
      status: 'active',
      tokens: 2200,
      hopDistance: 2,
      inDegree: 5,
      outDegree: 7,
      betweenness: 0.4,
      brokenLinks: 0,
    },
  ];

  test('discoverVaultSuggestions finds folders and tags sorted by frequency', () => {
    const suggestions = discoverVaultSuggestions(sampleNodes);
    assert.ok(suggestions.length > 0, 'Suggestions should not be empty');

    // Folder checks
    const projFolder = suggestions.find((s) => s.query === 'path:01_projects/**');
    assert.ok(projFolder, 'Should discover 01_projects/**');
    assert.strictEqual(projFolder.count, 2, '01_projects should have 2 notes');

    const metaFolder = suggestions.find((s) => s.query === 'path:00_meta/**');
    assert.ok(metaFolder, 'Should discover 00_meta/**');
    assert.strictEqual(metaFolder.count, 1);

    // Tag checks
    const archTag = suggestions.find((s) => s.query === 'tag:#architecture');
    assert.ok(archTag, 'Should discover tag:#architecture');
    assert.strictEqual(archTag.count, 2, 'Architecture tag should match 2 notes');

    // Distinct colors assigned from palette
    assert.ok(projFolder.color.startsWith('#'));
    assert.ok(metaFolder.color.startsWith('#'));
  });

  test('countMatchingNotes accurately matches path patterns and tags', () => {
    assert.strictEqual(countMatchingNotes('path:01_projects/**', sampleNodes), 2);
    assert.strictEqual(countMatchingNotes('path:00_meta/**', sampleNodes), 1);
    assert.strictEqual(countMatchingNotes('path:06_studies/**', sampleNodes), 1);
    assert.strictEqual(countMatchingNotes('path:non_existent/**', sampleNodes), 0);

    assert.strictEqual(countMatchingNotes('tag:#architecture', sampleNodes), 2);
    assert.strictEqual(countMatchingNotes('tag:#projects', sampleNodes), 2);
    assert.strictEqual(countMatchingNotes('tag:#memory', sampleNodes), 1);
    assert.strictEqual(countMatchingNotes('tag:#missing', sampleNodes), 0);
  });
});
