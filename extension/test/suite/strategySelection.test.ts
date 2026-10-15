import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderStrategySetHtml,
  renderStrategyRecommendationItemHtml,
  renderEvolutionTabHtml,
} from '../../webview-ui/src/components/evolutionTab.ts';
import type { StrategySet, StructuralProblemSignature } from '../../src/types/evolution.ts';

suite('Strategy Selection & Combination Test Suite (Feature 007 / T015, T030)', () => {
  const mockStrategySet: StrategySet = {
    setId: 'set_opp_001',
    opportunityId: 'opp_001',
    targetDocument: 'architecture/monolith.md',
    primary: {
      recommendationId: 'rec_opp_001_primary',
      strategy: 'section_addressing',
      fitScore: 88,
      role: 'primary',
      rationale: 'Primary section-level addressing strategy eliminates context waste.',
      expectedBenefits: ['Zero-explosion section navigation', 'Context efficiency up +35%'],
      countervailingTradeOffs: ['Frontmatter overhead'],
      transformationCost: 5,
    },
    supporting: [
      {
        recommendationId: 'rec_opp_001_LightweightIndex',
        strategy: 'lightweight_index',
        fitScore: 78,
        role: 'supporting',
        rationale: 'Complementary index routing synergizing with primary section addressing.',
        expectedBenefits: ['Curated discovery paths', 'Reduced hop distance'],
        countervailingTradeOffs: ['New index node overhead'],
        transformationCost: 10,
      },
    ],
    alternatives: [
      {
        recommendationId: 'rec_opp_001_FacetedClassification',
        strategy: 'faceted_classification',
        fitScore: 68,
        role: 'alternative',
        rationale: 'Alternative taxonomy tagging strategy.',
        expectedBenefits: ['Thematic queries'],
        countervailingTradeOffs: ['Tag drift overhead'],
        transformationCost: 15,
      },
      {
        recommendationId: 'rec_opp_001_PreserveDocument',
        strategy: 'preserve_document',
        fitScore: 62,
        role: 'alternative',
        rationale: 'Preserve unbroken document as healthy monolith.',
        expectedBenefits: ['100% narrative preservation'],
        countervailingTradeOffs: ['Token context overhead'],
        transformationCost: 0,
      },
    ],
    compositePlanValue: {
      baseFitSum: 166, // 88 + 78
      synergyBonus: 15,
      overlapPenalty: 5,
      transformationCost: 15, // 5 + 10
      netValue: 161, // 166 + 15 - 5 - 15
      narrative: 'Combined plan yields net value 161 with +15 synergy bonus exceeding 5 overlap overhead.',
    },
    activeSelection: ['section_addressing', 'lightweight_index'],
  };

  test('T019: Renders PRIMARY, SUPPORTING, and ALTERNATIVES visual groupings with badges and checkboxes', () => {
    const html = renderStrategySetHtml(mockStrategySet, 'opp_001');

    // Group headers
    assert.ok(html.includes('Primary Solution'), 'Must have Primary Solution section');
    assert.ok(html.includes('Supporting Strategies'), 'Must have Supporting section');
    assert.ok(html.includes('Alternative Options'), 'Must have Alternatives section');

    // Role Badges
    assert.ok(html.includes('role-badge-primary'), 'Must render Primary role badge');
    assert.ok(html.includes('role-badge-supporting'), 'Must render Supporting role badge');
    assert.ok(html.includes('role-badge-alt'), 'Must render Alternative role badge');

    // Strategy Labels
    assert.ok(html.includes('Section Addressing'), 'Must render Section Addressing');
    assert.ok(html.includes('Lightweight Index'), 'Must render Lightweight Index');
    assert.ok(html.includes('Faceted Classification'), 'Must render Faceted Classification');
    assert.ok(html.includes('Preserve Document'), 'Must render Preserve Document');

    // Default Pre-checked state: Primary and Supporting are checked, Alternatives are unchecked
    assert.ok(html.includes('data-strategy="section_addressing"'), 'Primary strategy present');
    assert.ok(html.includes('data-role="primary"'), 'Primary role present');
    assert.ok(/data-strategy="section_addressing"[^>]*checked/.test(html), 'Primary must be pre-checked');
    assert.ok(/data-strategy="lightweight_index"[^>]*checked/.test(html), 'Supporting must be pre-checked');
    assert.ok(html.includes('data-strategy="faceted_classification"'), 'Alternative must be rendered');
    assert.ok(!/data-strategy="faceted_classification"[^>]*checked/.test(html), 'Alternative must NOT be pre-checked');
  });

  test('T019 & T027: Renders Composite Plan Value Card with Base Fit, Synergy Bonus, Overlap Penalty, and Net Value without arbitrary Effort Cost', () => {
    const html = renderStrategySetHtml(mockStrategySet, 'opp_001');

    assert.ok(html.includes('Composite Plan Value & Trade-Off Valuation'), 'Must render Composite Plan Value header');
    assert.ok(html.includes('Net Value: 176'), 'Must render Net Value badge');
    assert.ok(html.includes('Base Fit Sum'), 'Must render Base Fit Sum label');
    assert.ok(html.includes('166'), 'Must display 166 Base Fit Sum');
    assert.ok(html.includes('+15'), 'Must display +15 Synergy Bonus');
    assert.ok(html.includes('-5'), 'Must display -5 Overlap Penalty');
    assert.ok(!html.includes('Effort Cost'), 'Must NOT render arbitrary Effort Cost label');
    assert.ok(!html.includes('rec-effort-badge'), 'Must NOT render arbitrary Cost badge');
    assert.ok(html.includes('Combined plan yields net value 176 with +15 synergy bonus exceeding 5 overlap overhead.'), 'Must render valuation narrative');
  });

  test('T020 & T021: Renders Incompatibility conflict banner and empty selection warning hooks', () => {
    const html = renderStrategySetHtml(mockStrategySet, 'opp_001');

    assert.ok(html.includes('id="conflict-banner-opp_001"'), 'Must have conflict notification banner');
    assert.ok(html.includes('id="empty-warning-opp_001"'), 'Must have empty selection warning element');
    assert.ok(html.includes('btn-generate-multi-plan'), 'Must have multi-strategy plan button');
    assert.ok(html.includes('data-action="generate-multi-plan"'), 'Must have action attribute');
  });

  test('T030: Evolution tab integration renders StrategySet when available on signature (Click budget <= 2 clicks)', () => {
    const mockSig: StructuralProblemSignature & { strategySet: StrategySet } = {
      signatureId: 'opp_001',
      documentPath: 'architecture/monolith.md',
      problemType: 'god_document',
      confidence: 0.92,
      rationale: 'Severe context waste in monolith.',
      evidence: {
        cohesion: 0.2,
        coupling: 0.8,
        referenceConcentration: 0.85,
        granularity: 45000,
        contextEfficiency: 0.25,
        discoverability: 0.4,
        reusability: 0.3,
        semanticDiversity: 0.88,
        navigationalComplexity: 4.2,
        maintenanceCoupling: 0.75,
      },
      strategySet: mockStrategySet,
    };

    const tabHtml = renderEvolutionTabHtml({
      signatures: [mockSig],
      recommendations: [],
      activeOrientation: 'ai_oriented',
    });

    // Validates that within the drawer (Click 1), the user directly inspects Primary, Supporting, and Alternatives
    // and can click to toggle or generate (Click 2), satisfying the <= 2 click budget (SC-005).
    assert.ok(tabHtml.includes('architecture/monolith.md'), 'Must display document path');
    assert.ok(tabHtml.includes('Multi-Strategy Selection:'), 'Must render multi-strategy section');
    assert.ok(tabHtml.includes('role-badge-primary'), 'Primary solution visible directly');
    assert.ok(tabHtml.includes('role-badge-supporting'), 'Supporting solution visible directly');
    assert.ok(tabHtml.includes('btn-generate-multi-plan'), 'Generate multi-strategy plan button immediately accessible');
  });
});
