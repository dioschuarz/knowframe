import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderKnobTooltipHtml, TIER1_TOOLTIPS } from '../../webview-ui/src/components/tooltip.ts';

suite('Tier 1 Knob Tooltip Unit Test Suite (T051)', () => {
  test('defines complete tooltip metadata for all 6 Tier 1 knobs', () => {
    const keys = Object.keys(TIER1_TOOLTIPS);
    assert.strictEqual(keys.length, 6);
    assert.ok(keys.includes('aiDiscoverability'));
    assert.ok(keys.includes('contextEfficiency'));
    assert.ok(keys.includes('humanReadability'));
    assert.ok(keys.includes('knowledgeReuse'));
    assert.ok(keys.includes('narrativeCoherence'));
    assert.ok(keys.includes('maintenanceSimplicity'));
  });

  test('renders plain-language descriptions and higher/lower impacts', () => {
    const html = renderKnobTooltipHtml('contextEfficiency');

    assert.ok(html.includes('Context Efficiency'));
    assert.ok(html.includes('Minimizes irrelevant token retrieval'));
    assert.ok(html.includes('▲ Higher:'));
    assert.ok(html.includes('▼ Lower:'));
    assert.ok(html.includes('Trade-off:'));
  });

  test('surfaces associated problem signatures and countervailing trade-offs', () => {
    const htmlGod = renderKnobTooltipHtml('contextEfficiency');
    assert.ok(htmlGod.includes('fragmentSizeTarget'));
    assert.ok(htmlGod.includes('maxContextSurfaceTokens'));

    const htmlNarrative = renderKnobTooltipHtml('narrativeCoherence');
    assert.ok(htmlNarrative.includes('splitSensitivity'));
    assert.ok(htmlNarrative.includes('cohesionThreshold'));
  });

  test('returns empty string for unrecognized knob keys', () => {
    const html = renderKnobTooltipHtml('unknownKey');
    assert.strictEqual(html, '');
  });
});
