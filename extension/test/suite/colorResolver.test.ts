import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { resolveNodeColor } from '../../webview-ui/src/graph/colorResolver.ts';

suite('Hue + Opacity Color Resolver Test Suite', () => {
  test('Level 0 active hub has 100% solid opacity', () => {
    const color = resolveNodeColor('#3B82F6', 0, false);
    assert.strictEqual(color, 'rgba(59, 130, 246, 1)');
  });

  test('Level 1 direct hop has 60% opacity', () => {
    const color = resolveNodeColor('#8B5CF6', 1, true);
    assert.strictEqual(color, 'rgba(139, 92, 246, 0.6)');
  });

  test('Level 2 hop has 30% opacity', () => {
    const color = resolveNodeColor('#10B981', 2, true);
    assert.strictEqual(color, 'rgba(16, 185, 129, 0.3)');
  });

  test('Level 3+ deep hops have 15% opacity', () => {
    const color = resolveNodeColor('#EF4444', 3, true);
    assert.strictEqual(color, 'rgba(239, 68, 68, 0.15)');
  });

  test('Unstyled orphan node has 40% muted opacity', () => {
    const color = resolveNodeColor('#6B7280', -1, false);
    assert.strictEqual(color, 'rgba(107, 114, 128, 0.4)');
  });

  test('Orphan matching cluster rule has 15% peripheral opacity', () => {
    const color = resolveNodeColor('#EF4444', -1, true);
    assert.strictEqual(color, 'rgba(239, 68, 68, 0.15)');
  });
});
