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
      hub_path: 'AGENTS.md',
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

  test('T014: Explore graph button moved into Health card header with no duplicate blue refresh/settings buttons', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    assert.ok(html.includes('<title>KnowFrame - Manager</title>'), 'Title must be KnowFrame - Manager');
    assert.ok(html.includes('id="dash-btn-explore-graph"'), 'Must have Explore graph button');
    assert.ok(html.includes('Explore graph'), 'Button text must be Explore graph');
    assert.ok(html.includes('id="dash-btn-review-all-ai"'), 'Must have Review all with AI button');
    assert.ok(html.includes('data-tooltip="Review all with AI"'), 'Review all with AI must have tooltip');
    // SVG icon matching sidebar-icon.svg
    assert.ok(html.includes('viewBox="0 0 24 24"'), 'Must contain 24x24 SVG graph icon matching sidebar-icon.svg');

    // Verify duplicate top buttons are removed from cards
    assert.ok(!html.includes('id="dash-btn-open-settings"'), 'Duplicate settings button in Health header must be removed');
    assert.ok(!html.includes('id="dash-btn-refresh-quality"'), 'Duplicate refresh button in Health header must be removed');
    assert.ok(!html.includes('id="dash-btn-rescan-vault"'), 'Duplicate rescan button in Structure header must be removed');

    // Verify Review all with AI and Explore graph are inside Health card, with Review all to the left of Explore graph
    const healthIdx = html.indexOf('id="pillar-health"');
    const reviewAllIdx = html.indexOf('id="dash-btn-review-all-ai"');
    const exploreIdx = html.indexOf('id="dash-btn-explore-graph"');
    const structureIdx = html.indexOf('id="pillar-structure"');
    assert.ok(healthIdx !== -1 && reviewAllIdx !== -1 && exploreIdx !== -1 && structureIdx !== -1, 'Health, Review all with AI, Explore graph, and Structure must exist');
    assert.ok(healthIdx < reviewAllIdx && reviewAllIdx < exploreIdx && exploreIdx < structureIdx, 'Review all with AI must be to the left of Explore graph inside Health card');
  });

  test('T015 & T016: Evolution card (no objective sentence) and Sync card invariants with gear icon-only buttons & hover tooltips', () => {
    const html = renderSidebarHtml({
      cspPolicy: "default-src 'none'",
      nonce: 'test-nonce',
      initialSummary: sampleSummary,
    });

    // Evolution
    const bodyHtml = html.split('<script')[0];
    assert.ok(bodyHtml.includes('evolution-mode-badge'), 'Must contain orientation badge');
    assert.ok(bodyHtml.includes('dash-btn-configure-evolution'), 'Must contain evolution configure gear button');
    assert.ok(bodyHtml.includes('structural opportunities detected') || bodyHtml.includes('structural opportunity detected'), 'Must contain opportunities text');
    assert.ok(!bodyHtml.includes('id="evolution-objective-text"'), 'Must NOT render evolution objective sentence');
    assert.ok(!bodyHtml.includes('Optimizing AI discovery, context efficiency & reuse'), 'Must NOT include mode description sentence in Evolution card');

    // Gear icon-only buttons with tooltip bubbles
    assert.ok(html.includes('class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-change-hub"'), 'Change hub must be a gear icon-only button');
    assert.ok(html.includes('data-tooltip="Change central hub note"'), 'Change hub gear button must have hover tooltip bubble');
    const structureHeaderIdx = html.indexOf('<div class="pillar-title">STRUCTURE</div>');
    const changeHubIdx = html.indexOf('id="dash-btn-change-hub"');
    const statsPillsIdx = html.indexOf('class="stats-pills-row"');
    assert.ok(structureHeaderIdx < changeHubIdx && changeHubIdx < statsPillsIdx, 'Change hub gear button must be in the upper right header of the STRUCTURE box');
    assert.ok(html.includes('class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-configure-evolution"'), 'Configure evolution must be a gear icon-only button');
    assert.ok(html.includes('data-tooltip="Configure evolution mode & strategies"'), 'Configure evolution gear button must have hover tooltip bubble');
    assert.ok(html.includes('class="dash-action-btn icon-only-btn gear-action-btn" id="dash-btn-open-harness"'), 'Harness tools button must be a gear icon-only button');
    assert.ok(html.includes('data-tooltip="Configure AI harness & agent tools"'), 'Harness tools gear button must have hover tooltip bubble');
    assert.ok(!html.includes('>Change</button>'), 'Must not render text "Change" inside buttons');
    assert.ok(!html.includes('Harness Tools</button>'), 'Must not render text "Harness Tools" inside button');

    // Sync
    assert.ok(html.includes('sync-indicator-dot'), 'Must contain sync status dot');
    assert.ok(html.includes('Synced ≠ Healthy'), 'Must contain Invariant I reminder (Synced != Healthy)');
  });
});
