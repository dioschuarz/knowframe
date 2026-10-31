import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { getScoreColor, renderQualityPillarHtml } from '../../webview-ui/src/components/qualityPillar.ts';
import { renderStatusDashboardHtml } from '../../webview-ui/src/components/statusDashboard.ts';
import type { KnowledgeQualityScore } from '../../src/types/admission.ts';
import type { ObservatorySummaryPayload } from '../../src/types/protocol.ts';

suite('QualityPillar & 4-Pillar Dashboard Test Suite', () => {
  const sampleQuality: KnowledgeQualityScore = {
    overall_score: 88.4,
    integrity_score: 95.0,
    structure_score: 90.0,
    consistency_score: 82.0,
    discoverability_score: 90.0,
    freshness_score: 85.0,
    legacy_debt_count: 5,
    regressions_count: 1,
    resolved_debt_count: 2,
    central_hub_id: 'AGENTS.md',
    top_degradation_factors: [
      {
        category: 'broken_link',
        affected_node: 'AGENTS.md',
        betweenness_weight: 3.5,
        deduction_points: 17.5,
        description: "Broken link to 'missing.md'",
      },
    ],
  };

  test('getScoreColor returns green for >=90, yellow for >=75, red for <75', () => {
    assert.ok(getScoreColor(95).includes('green'));
    assert.ok(getScoreColor(80).includes('yellow'));
    assert.ok(getScoreColor(60).includes('red'));
  });

  test('renderQualityPillarHtml renders overall score and 5 pillar meters', () => {
    const html = renderQualityPillarHtml({ qualityScore: sampleQuality });

    assert.ok(html.includes('QUALITY & ADMISSION'), 'Must display Quality title');
    assert.ok(html.includes('88'), 'Must show overall score');
    assert.ok(html.includes('Integrity'), 'Must display Integrity pillar');
    assert.ok(html.includes('Structure'), 'Must display Structure pillar (Pillar 2)');
    assert.ok(html.includes('Consistency'), 'Must display Consistency pillar');
    assert.ok(html.includes('Discoverability'), 'Must display Discoverability pillar');
    assert.ok(html.includes('Traceability') || html.includes('Freshness'), 'Must display Traceability pillar');

    // Debt accounting
    assert.ok(html.includes('Legacy'), 'Must show Legacy label');
    assert.ok(html.includes('New Regr.'), 'Must show New Regr. label');
    assert.ok(html.includes('Resolved'), 'Must show Resolved label');
    assert.ok(html.includes('5'), 'Must show legacy count 5');
    assert.ok(html.includes('1'), 'Must show new regressions count 1');
    assert.ok(html.includes('2'), 'Must show resolved count 2');

    // Degradation factors
    assert.ok(html.includes('-17.5'), 'Must display top degradation deduction');
    assert.ok(html.includes('AGENTS.md'), 'Must display affected node');
  });

  test('renderStatusDashboardHtml seamlessly renders Fourth Pillar alongside Knowledge, AI Context, Hub', () => {
    const summary: ObservatorySummaryPayload = {
      totalNotes: 100,
      connectedNotes: 80,
      orphanNotes: 5,
      excludedNotes: 15,
      brokenLinksCount: 1,
      activeTokens: 2000,
      reachableTokens: 8000,
      syncStatus: 'synced',
      lastSyncTimestamp: 1726080000000,
    };

    const html = renderStatusDashboardHtml({
      summary,
      primaryHub: 'AGENTS.md',
      qualityScore: sampleQuality,
    });

    assert.ok(html.includes('KNOWLEDGE'), 'Must include Pillar 1 Knowledge');
    assert.ok(html.includes('AI CONTEXT'), 'Must include Pillar 2 AI Context');
    assert.ok(html.includes('CENTRAL HUB & HEALTH'), 'Must include Pillar 3 Central Hub');
    assert.ok(html.includes('QUALITY & ADMISSION'), 'Must include Pillar 4 Quality & Admission');
  });
});
