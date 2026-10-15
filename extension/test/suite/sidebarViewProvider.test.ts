import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderSidebarHtml } from '../../src/webview/sidebarRenderer.ts';
import type { ManagerSummaryState } from '../../src/types/protocol.ts';

suite('SidebarViewProvider 4-Card Redesign Test Suite', () => {
  const sampleSummary: ManagerSummaryState = {
    health: {
      quality_score: 88,
      integrity: 92,
      structure: 85,
      consistency: 90,
      discoverability: 82,
      freshness: 91,
      findings_count: 3,
      broken_references: 2,
      invalid_anchors: 1,
      legacy_debt: 5,
      new_regressions: 0,
    },
    structure: {
      total_documents: 51,
      connected_documents: 4,
      isolated_documents: 47,
      total_references: 72,
    },
    evolution: {
      orientation: 'ai_oriented',
      opportunity_count: 2,
      primary_objective: 'Optimizing AI discovery, context efficiency & reuse',
      conflicting_constraints: false,
    },
    sync: {
      status: 'synced',
      last_commit_sha: 'a1b2c3d',
      last_commit_time: null,
    },
  };

  test('T010 & T020: Renders exactly 4 calm cards: Health, Structure, Evolution, Sync', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('pillar-health'), 'Must contain Health card');
    assert.ok(html.includes('pillar-structure'), 'Must contain Structure card');
    assert.ok(html.includes('pillar-evolution'), 'Must contain Evolution card');
    assert.ok(html.includes('pillar-sync'), 'Must contain Sync card');

    assert.ok(html.includes('HEALTH'), 'Must have HEALTH title');
    assert.ok(html.includes('STRUCTURE'), 'Must have STRUCTURE title');
    assert.ok(html.includes('EVOLUTION'), 'Must have EVOLUTION title');
    assert.ok(html.includes('SYNC'), 'Must have SYNC title');
  });

  test('T011 & T021: Visual separation of Findings Count vs Quality Score (No duplicate broken alert)', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    // Score is present
    assert.ok(html.includes('val-quality-score'), 'Quality score element must exist');
    // Findings count is separated
    assert.ok(html.includes('dash-findings-count'), 'Findings count element must exist');
    assert.ok(html.includes('findings need attention'), 'Findings description must exist');

    // Broken links and invalid anchors must NOT be a separate floating alert badge
    assert.ok(!html.includes('broken-links-alert'), 'Floating broken links alert badge must be eliminated per T012/SC-001');
    assert.ok(!html.includes('dash-broken-alert'), 'dash-broken-alert must be removed');
  });

  test('T013: Standardized Structure Terminology (documents, references, isolated, never orphans)', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('Documents'), 'Must use Documents terminology');
    assert.ok(html.includes('Connected'), 'Must use Connected terminology');
    assert.ok(html.includes('Isolated'), 'Must use Isolated terminology');
    assert.ok(html.includes('References'), 'Must use References terminology');

    // Ensure the label says "Isolated" and never "Orphans"
    assert.ok(!html.includes('>Orphans<'), 'Must NEVER use the label "Orphans" in structure pills per SC-004');
    assert.ok(!html.includes('>Links<'), 'Must use References instead of generic Links label in pills');
  });

  test('T014: Explore graph button with LLM Wiki graph icon', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('dash-btn-explore-graph'), 'Must have Explore graph button');
    assert.ok(html.includes('Explore graph'), 'Button text must be Explore graph');
    // SVG icon matching sidebar-icon.svg
    assert.ok(html.includes('viewBox="0 0 24 24"'), 'Must contain 24x24 SVG graph icon matching sidebar-icon.svg');
  });

  test('T015 & T016: Evolution card and Sync card invariants', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    // Evolution
    assert.ok(html.includes('evolution-mode-badge'), 'Must contain orientation badge');
    assert.ok(html.includes('dash-btn-configure-evolution'), 'Must contain evolution configure/change button');
    assert.ok(html.includes('structural opportunities detected') || html.includes('structural opportunity detected'), 'Must contain opportunities text');

    // Sync
    assert.ok(html.includes('sync-indicator-dot'), 'Must contain sync status dot');
    assert.ok(html.includes('Synced ≠ Healthy'), 'Must contain Invariant I reminder (Synced != Healthy)');
  });

  test('Renders Management & Settings gear button in Health card header left of refresh button', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('id="dash-btn-open-settings"'), 'Must contain Management & Settings button');
    assert.ok(html.includes('title="Management & Settings"'), 'Must have title Management & Settings');
    assert.ok(html.includes('id="dash-btn-refresh-quality"'), 'Must contain Refresh Quality button');

    // Verify ordering: gear button appears before refresh button in HTML
    const settingsIdx = html.indexOf('dash-btn-open-settings');
    const refreshIdx = html.indexOf('dash-btn-refresh-quality');
    assert.ok(settingsIdx !== -1, 'Gear button must exist');
    assert.ok(refreshIdx !== -1, 'Refresh button must exist');
    assert.ok(settingsIdx < refreshIdx, 'Gear button must be positioned to the left of the refresh button');
  });

  test('Renders Rescan Vault button in Structure card header alongside Explore graph', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('id="dash-btn-rescan-vault"'), 'Must contain Rescan Vault button in Structure card');
    assert.ok(html.includes('title="Rescan Vault & Rebuild Graph"'), 'Must have title Rescan Vault & Rebuild Graph');
  });
});
