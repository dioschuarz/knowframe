import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderSidebarHtml } from '../../src/webview/sidebarRenderer.ts';
import { normalizeTab, type NavigationIntent } from '../../webview-ui/src/components/settingsDrawer.ts';

suite('NavigationRouting Test Suite (T031)', () => {
  test('sidebar HTML contains clickable pillar mini rows with data-dimension attributes', () => {
    const html = renderSidebarHtml('mock-nonce');

    const expectedDimensions = ['integrity', 'structure', 'consistency', 'discoverability', 'traceability'];
    for (const dim of expectedDimensions) {
      assert.ok(
        html.includes(`data-dimension="${dim}"`) || html.includes(`data-legacy-dimension="${dim}"`),
        `Sidebar HTML must contain mini row for dimension: ${dim}`,
      );
    }
  });

  test('sidebar HTML contains evolution navigation buttons', () => {
    const html = renderSidebarHtml('mock-nonce');

    assert.ok(html.includes('id="dash-btn-configure-evolution"'), 'Must have evolution change button');
    assert.ok(html.includes('id="evolution-mode-badge"'), 'Must have evolution mode badge');
    assert.ok(html.includes('id="evolution-opportunity-row"'), 'Must have evolution opportunity row');
  });

  test('validates NavigationIntent structure for Health dimension routing', () => {
    const intent: NavigationIntent = {
      primaryTab: 'insights',
      subTab: 'health',
      targetSection: 'integrity',
    };

    assert.strictEqual(intent.primaryTab, 'insights');
    assert.strictEqual(intent.subTab, 'health');
    assert.strictEqual(intent.targetSection, 'integrity');
  });

  test('validates NavigationIntent structure for Evolution routing', () => {
    const intent: NavigationIntent = {
      primaryTab: 'insights',
      subTab: 'evolution',
      targetId: 'opportunity-1',
    };

    assert.strictEqual(intent.primaryTab, 'insights');
    assert.strictEqual(intent.subTab, 'evolution');
    assert.strictEqual(intent.targetId, 'opportunity-1');
  });

  test('normalizeTab handles backwards-compatibility aliases', () => {
    assert.strictEqual(normalizeTab('diagnostics'), 'health');
    assert.strictEqual(normalizeTab('git_sync'), 'git');
    assert.strictEqual(normalizeTab('health'), 'health');
    assert.strictEqual(normalizeTab('structure'), 'structure');
    assert.strictEqual(normalizeTab('evolution'), 'evolution');
    assert.strictEqual(normalizeTab('advanced'), 'git');
    assert.strictEqual(normalizeTab('git_harness'), 'harness');
    assert.strictEqual(normalizeTab('git'), 'git');
    assert.strictEqual(normalizeTab('harness'), 'harness');
    assert.strictEqual(normalizeTab('knowledge'), 'knowledge');
  });
});
