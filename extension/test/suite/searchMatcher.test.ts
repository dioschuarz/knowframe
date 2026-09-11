import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { filterNodesByQuery, matchesQuery } from '../../webview-ui/src/graph/searchMatcher.ts';
import type { NoteNode } from '../../src/types/graph.ts';

suite('Graph Search Matcher Test Suite', () => {
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
      id: '01_projects/client_app.md',
      label: 'Client App Architecture',
      filePath: '01_projects/client_app.md',
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

  test('Empty or whitespace query returns null (no filtering)', () => {
    assert.strictEqual(filterNodesByQuery(sampleNodes, ''), null);
    assert.strictEqual(filterNodesByQuery(sampleNodes, '   '), null);
  });

  test('Substring search matches folder path', () => {
    const matched = filterNodesByQuery(sampleNodes, '06_studies');
    assert.ok(matched);
    assert.strictEqual(matched.size, 1);
    assert.ok(matched.has('06_studies/k8s/01_index.md'));
  });

  test('Substring search matches file title and name', () => {
    const matched = filterNodesByQuery(sampleNodes, 'context');
    assert.ok(matched);
    assert.strictEqual(matched.size, 1);
    assert.ok(matched.has('02_memory/active_context.md'));
  });

  test('Substring search matches tags', () => {
    const matched = filterNodesByQuery(sampleNodes, 'architecture');
    assert.ok(matched);
    assert.strictEqual(matched.size, 2);
    assert.ok(matched.has('01_projects/client_app.md'));
    assert.ok(matched.has('06_studies/k8s/01_index.md'));
  });

  test('Exact match distinguishes specific folder or tag', () => {
    const exactFolder = filterNodesByQuery(sampleNodes, '00_meta', { exact: true });
    assert.ok(exactFolder);
    assert.strictEqual(exactFolder.size, 1);
    assert.ok(exactFolder.has('00_meta/wiki_index.md'));

    const nonExact = filterNodesByQuery(sampleNodes, '00_met', { exact: true });
    assert.ok(nonExact);
    assert.strictEqual(nonExact.size, 0);
  });
});
