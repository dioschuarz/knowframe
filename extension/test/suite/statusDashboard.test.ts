import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderStatusDashboardHtml } from '../../webview-ui/src/components/statusDashboard.ts';
import type { ObservatorySummaryPayload } from '../../src/types/protocol.ts';

suite('StatusDashboard MVP Test Suite', () => {
  const sampleSummary: ObservatorySummaryPayload = {
    totalNotes: 238,
    connectedNotes: 142,
    orphanNotes: 12,
    excludedNotes: 84,
    brokenLinksCount: 5,
    activeTokens: 4920,
    reachableTokens: 18240,
    syncStatus: 'synced',
    lastSyncTimestamp: 1726080000000,
  };

  test('Renders 3 pillars: Knowledge, AI Context, and Central Hub & Health', () => {
    const html = renderStatusDashboardHtml({
      summary: sampleSummary,
      primaryHub: '00_meta/wiki_index.md',
      totalLinks: 343,
    });

    // Pillar 1: Knowledge
    assert.ok(html.includes('KNOWLEDGE'), 'Must contain KNOWLEDGE header');
    assert.ok(html.includes('142'), 'Must show connected notes count');
    assert.ok(html.includes('12'), 'Must show orphan notes count');
    assert.ok(html.includes('84'), 'Must show excluded notes count');
    assert.ok(html.includes('343'), 'Must show total links count');

    // Pillar 2: AI Context
    assert.ok(html.includes('AI CONTEXT'), 'Must contain AI CONTEXT header');
    assert.ok(html.includes('4.9k') || html.includes('4,920'), 'Must show active prompt load');
    assert.ok(html.includes('18.2k') || html.includes('18,240'), 'Must show reachable context load');

    // Pillar 3: Central Hub & Health
    assert.ok(html.includes('CENTRAL HUB'), 'Must contain CENTRAL HUB header');
    assert.ok(html.includes('00_meta/wiki_index.md'), 'Must display primary hub name');
    assert.ok(html.includes('Change'), 'Must provide [Change] link');
    assert.ok(html.includes('Synced'), 'Must provide Synced status indicator');
  });

  test('Conditionally renders broken links attention card when broken links > 0', () => {
    const htmlWithBroken = renderStatusDashboardHtml({
      summary: sampleSummary,
      primaryHub: '00_meta/wiki_index.md',
      totalLinks: 343,
    });
    assert.ok(htmlWithBroken.includes('5 broken links'), 'Must show broken links count');
    assert.ok(htmlWithBroken.includes('Review issues'), 'Must show [Review issues] action');

    const cleanSummary: ObservatorySummaryPayload = {
      ...sampleSummary,
      brokenLinksCount: 0,
    };
    const htmlClean = renderStatusDashboardHtml({
      summary: cleanSummary,
      primaryHub: '00_meta/wiki_index.md',
      totalLinks: 343,
    });
    assert.ok(!htmlClean.includes('Review issues'), 'Must NOT show attention card when 0 broken links');
  });

  test('Strictly omits all 5 obsolete duplicate action buttons', () => {
    const html = renderStatusDashboardHtml({
      summary: sampleSummary,
      primaryHub: '00_meta/wiki_index.md',
      totalLinks: 343,
    });

    // Zero Action Duplication verification (Constitution Principle VI)
    assert.ok(!html.includes('Open Knowledge Graph'), 'Duplicate "Open Knowledge Graph" button must be gone');
    assert.ok(!html.includes('Set Central Hub'), 'Duplicate "Set Central Hub" button must be gone');
    assert.ok(!html.includes('Rescan Vault'), 'Duplicate "Rescan Vault" button must be gone');
    assert.ok(!html.includes('Fix Broken Links'), 'Duplicate "Fix Broken Links" button must be gone');
    assert.ok(!html.includes('Install Git Hooks'), 'Duplicate "Install Git Hooks" button must be gone');
  });
});
