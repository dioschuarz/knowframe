import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderOrientationSelectorHtml,
  ORIENTATION_CARDS,
} from '../../webview-ui/src/components/orientationSelector.ts';

suite('OrientationSelector Unit Test Suite (T049)', () => {
  test('renders 4 selectable radio cards with canonical target sizes', () => {
    const html = renderOrientationSelectorHtml({ activeOrientation: 'ai_oriented' });

    assert.strictEqual(ORIENTATION_CARDS.length, 4);
    assert.ok(html.includes('AI-Oriented'));
    assert.ok(html.includes('Human-Oriented'));
    assert.ok(html.includes('Balanced'));
    assert.ok(html.includes('Custom'));
    assert.ok(!html.includes('Custom (Advanced)'));

    assert.ok(html.includes('3k tokens'));
    assert.ok(html.includes('8k tokens'));
    assert.ok(html.includes('5k tokens'));
    assert.ok(html.includes('User Defined'));
  });

  test('marks the active orientation as selected with checked radio', () => {
    const htmlAi = renderOrientationSelectorHtml({ activeOrientation: 'ai_oriented' });
    assert.ok(htmlAi.includes('orientation-card-selected'));
    assert.ok(htmlAi.includes('value="ai_oriented" checked'));

    const htmlHuman = renderOrientationSelectorHtml({ activeOrientation: 'human_oriented' });
    assert.ok(htmlHuman.includes('value="human_oriented" checked'));

    const htmlCustom = renderOrientationSelectorHtml({ activeOrientation: 'custom' });
    assert.ok(htmlCustom.includes('value="custom" checked'));
  });

  test('renders transparent trade-offs for each orientation card', () => {
    const html = renderOrientationSelectorHtml({ activeOrientation: 'balanced' });

    assert.ok(html.includes('Higher fragmentation; splits narrative chapters'));
    assert.ok(html.includes('Lower context efficiency; agents load larger documents'));
    assert.ok(html.includes('Moderate balance; intermediate section sizes'));
    assert.ok(html.includes('Configurable directly in the Custom Settings drawer'));
  });
});
