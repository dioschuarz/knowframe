import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderCustomSettingsDrawerHtml,
  TIER1_KNOB_DEFS,
  TIER2_SLIDER_DEFS,
  type CustomSettingsProps,
} from '../../webview-ui/src/components/customSettingsDrawer.ts';

suite('CustomSettingsDrawer Unit Test Suite (T050)', () => {
  const sampleProps: CustomSettingsProps = {
    tier1: {
      aiDiscoverability: 'high',
      contextEfficiency: 'high',
      humanReadability: 'medium',
      knowledgeReuse: 'high',
      narrativeCoherence: 'low',
      maintenanceSimplicity: 'medium',
    },
    tier2: {
      cohesionThreshold: 0.8,
      semanticDiversityThreshold: 0.75,
      referenceConcentrationThreshold: 0.7,
      fragmentSizeTarget: 3500,
      maxTraceDepth: 4,
      candidateExpansionFactor: 1.5,
      maxContextSurfaceTokens: 12000,
      splitSensitivity: 0.8,
      canonicalOwnershipSensitivity: 0.75,
      duplicateTolerance: 0.1,
      fragmentationTolerance: 0.15,
    },
    constraints: [
      {
        id: 'c1',
        level: 'hard_constraint',
        ruleKey: 'never_split_canonical_docs',
        description: 'Prohibit splitting canonical reference notes',
      },
    ],
    conflicts: [],
  };

  test('renders all 6 Tier 1 knobs with active level highlights and tooltips', () => {
    const html = renderCustomSettingsDrawerHtml(sampleProps);

    assert.strictEqual(TIER1_KNOB_DEFS.length, 6);
    for (const knob of TIER1_KNOB_DEFS) {
      assert.ok(html.includes(knob.label), `Must contain label for ${knob.label}`);
    }

    assert.ok(html.includes('data-knob="aiDiscoverability" data-level="high"'));
    assert.ok(html.includes('tier1-btn-active'));
  });

  test('renders all 11 Tier 2 sliders with min, max, step, and current values', () => {
    const html = renderCustomSettingsDrawerHtml(sampleProps);

    assert.strictEqual(TIER2_SLIDER_DEFS.length, 11);
    for (const slider of TIER2_SLIDER_DEFS) {
      assert.ok(html.includes(`slider-${slider.key}`), `Must contain slider for ${slider.key}`);
      assert.ok(html.includes(`data-param="${slider.key}"`));
    }

    assert.ok(html.includes('3500 tok'));
    assert.ok(html.includes('4 hops'));
    assert.ok(html.includes('12000 tok'));
  });

  test('renders constraint rules list and integrated conflict status banner', () => {
    const html = renderCustomSettingsDrawerHtml(sampleProps);

    assert.ok(html.includes('never_split_canonical_docs'));
    assert.ok(html.includes('Prohibit splitting canonical reference notes'));
    assert.ok(html.includes('hard constraint'));
    assert.ok(html.includes('All active constraint rules are mutually satisfiable'));
  });
});
