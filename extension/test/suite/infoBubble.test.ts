import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  INFO_BUBBLE_DEFINITIONS,
  renderInfoBubbleHtml,
  adjustTooltipPosition,
  resetTooltipPosition,
  wireInfoBubbleListeners,
} from '../../webview-ui/src/components/infoBubble.ts';

suite('Informational Bubble (ⓘ) Component Test Suite (T035)', () => {
  const canonicalDimensions = [
    'cohesion',
    'coupling',
    'reference_concentration',
    'granularity',
    'context_efficiency',
    'discoverability',
    'knowledge_reuse',
    'semantic_diversity',
    'navigational_complexity',
    'maintenance_coupling',
  ];

  test('defines complete metadata answering all 5 mandatory questions for 10 structural dimensions', () => {
    for (const key of canonicalDimensions) {
      const def = INFO_BUBBLE_DEFINITIONS[key];
      assert.ok(def, `Missing definition for structural dimension: ${key}`);
      assert.ok(def.title && def.title.length > 0, `${key} must have human-readable title`);
      assert.ok(def.meaning && def.meaning.length > 10, `${key} must answer 'Meaning'`);
      assert.ok(def.implications && def.implications.length > 10, `${key} must answer 'Implications'`);
      assert.ok(def.importance && def.importance.length > 10, `${key} must answer 'Importance'`);
      assert.ok(def.affects && def.affects.length > 10, `${key} must answer 'Affects'`);
      assert.ok(def.tradeOff && def.tradeOff.length > 10, `${key} must answer 'Trade-off'`);
    }
  });

  test('defines strategy fit score informational bubble', () => {
    const def = INFO_BUBBLE_DEFINITIONS.fit_score;
    assert.ok(def, 'Missing fit_score definition');
    assert.ok(def.meaning.includes('orientation'), 'Fit score explains orientation alignment');
    assert.ok(def.tradeOff.length > 10, 'Fit score explains trade-offs');
  });

  test('renders accessible tooltip HTML answering 5 questions', () => {
    const html = renderInfoBubbleHtml('cohesion');

    assert.ok(html.includes('info-bubble-wrapper'), 'Must have wrapper');
    assert.ok(html.includes('ⓘ'), 'Must render info icon');
    assert.ok(html.includes('Cohesion'), 'Must render title');
    assert.ok(html.includes('Meaning:'), 'Must label Meaning');
    assert.ok(html.includes('Implications:'), 'Must label Implications');
    assert.ok(html.includes('Importance:'), 'Must label Importance');
    assert.ok(html.includes('Affects:'), 'Must label Affects');
    assert.ok(html.includes('Trade-off:'), 'Must label Trade-off');
  });

  test('supports camelCase aliases for metrics', () => {
    const html1 = renderInfoBubbleHtml('contextEfficiency');
    assert.ok(html1.includes('Context Efficiency'), 'Must support camelCase contextEfficiency');

    const html2 = renderInfoBubbleHtml('referenceConcentration');
    assert.ok(html2.includes('Reference Concentration'), 'Must support camelCase referenceConcentration');
  });

  test('returns empty string for unrecognized keys', () => {
    const html = renderInfoBubbleHtml('non_existent_key');
    assert.strictEqual(html, '');
  });

  test('adjusts tooltip position rightward when overflowing left drawer border (Granularity case)', () => {
    // Container: drawer content located from X: 1000 to X: 1444 (width: 444)
    const containerMock = {
      getBoundingClientRect: () => ({
        left: 1000,
        right: 1444,
        top: 100,
        bottom: 800,
        width: 444,
        height: 700,
      }),
    };

    // Tooltip: width: 280, height: 240
    // Centered on wrapper at X: 1070 would give left: 1070 - 140 = 930 (70px outside left border 1000)
    let tooltipTransform = '';
    const tooltipMock: any = {
      style: {
        set transform(v: string) { tooltipTransform = v; },
        get transform() { return tooltipTransform; },
      },
      classList: {
        classes: new Set<string>(),
        add(c: string) { this.classes.add(c); },
        remove(c: string) { this.classes.delete(c); },
      },
      getBoundingClientRect: () => ({
        left: 930,
        right: 1210,
        top: 360,
        bottom: 600,
        width: 280,
        height: 240,
      }),
    };

    // Wrapper: icon at X: 1070, Y: 600
    const wrapperMock: any = {
      querySelector: (sel: string) => sel.includes('info-bubble-tooltip') ? tooltipMock : null,
      closest: (sel: string) => sel.includes('settings-content') ? containerMock : null,
      getBoundingClientRect: () => ({
        left: 1070,
        right: 1083,
        top: 600,
        bottom: 613,
        width: 13,
        height: 13,
      }),
    };

    adjustTooltipPosition(wrapperMock);

    // minLeft = 1000 + 12 = 1012. Tooltip left was 930. Overflow = 1012 - 930 = 82px.
    assert.ok(tooltipMock.style.transform.includes('translateX(calc(-50% + 82px))'),
      `Tooltip must shift rightward to clear left border: ${tooltipMock.style.transform}`);
    assert.strictEqual(tooltipMock.classList.classes.has('tooltip-flipped'), false,
      'Must not flip when space above is ample');
  });

  test('adjusts tooltip position leftward when overflowing right drawer border (Discoverability case)', () => {
    const containerMock = {
      getBoundingClientRect: () => ({
        left: 1000,
        right: 1444,
        top: 100,
        bottom: 800,
        width: 444,
        height: 700,
      }),
    };

    // Tooltip: centered at X: 1380 gives right: 1380 + 140 = 1520 (76px outside right border 1444)
    let tooltipTransform = '';
    const tooltipMock: any = {
      style: {
        set transform(v: string) { tooltipTransform = v; },
        get transform() { return tooltipTransform; },
      },
      classList: {
        classes: new Set<string>(),
        add(c: string) { this.classes.add(c); },
        remove(c: string) { this.classes.delete(c); },
      },
      getBoundingClientRect: () => ({
        left: 1240,
        right: 1520,
        top: 360,
        bottom: 600,
        width: 280,
        height: 240,
      }),
    };

    const wrapperMock: any = {
      querySelector: (sel: string) => sel.includes('info-bubble-tooltip') ? tooltipMock : null,
      closest: (sel: string) => sel.includes('settings-content') ? containerMock : null,
      getBoundingClientRect: () => ({
        left: 1380,
        right: 1393,
        top: 600,
        bottom: 613,
        width: 13,
        height: 13,
      }),
    };

    adjustTooltipPosition(wrapperMock);

    // maxRight = 1444 - 12 = 1432. Tooltip right was 1520. Overflow = 1520 - 1432 = 88px.
    assert.ok(tooltipMock.style.transform.includes('translateX(calc(-50% - 88px))'),
      `Tooltip must shift leftward to clear right border: ${tooltipMock.style.transform}`);
  });

  test('flips tooltip below icon when space above is insufficient', () => {
    const containerMock = {
      getBoundingClientRect: () => ({
        left: 1000,
        right: 1444,
        top: 100,
        bottom: 800,
        width: 444,
        height: 700,
      }),
    };

    // Wrapper is near the top of the container: top is 150 (only 50px from container top 100)
    // Tooltip height is 240, so it cannot fit above
    const tooltipMock: any = {
      style: {},
      classList: {
        classes: new Set<string>(),
        add(c: string) { this.classes.add(c); },
        remove(c: string) { this.classes.delete(c); },
      },
      getBoundingClientRect: () => ({
        left: 1100,
        right: 1380,
        top: -90,
        bottom: 150,
        width: 280,
        height: 240,
      }),
    };

    const wrapperMock: any = {
      querySelector: (sel: string) => sel.includes('info-bubble-tooltip') ? tooltipMock : null,
      closest: (sel: string) => sel.includes('settings-content') ? containerMock : null,
      getBoundingClientRect: () => ({
        left: 1200,
        right: 1213,
        top: 150,
        bottom: 163,
        width: 13,
        height: 13,
      }),
    };

    adjustTooltipPosition(wrapperMock);

    assert.ok(tooltipMock.classList.classes.has('tooltip-flipped'), 'Must add tooltip-flipped class');
    assert.strictEqual(tooltipMock.style.bottom, 'auto', 'bottom must be set to auto');
    assert.strictEqual(tooltipMock.style.top, 'calc(100% + 6px)', 'top must open below icon');
  });

  test('resets inline styles on resetTooltipPosition', () => {
    const tooltipMock: any = {
      style: {
        display: 'flex',
        visibility: 'visible',
        transform: 'translateX(calc(-50% + 40px))',
        top: 'calc(100% + 6px)',
        bottom: 'auto',
      },
      classList: {
        classes: new Set<string>(['tooltip-flipped']),
        remove(c: string) { this.classes.delete(c); },
      },
    };

    const wrapperMock: any = {
      querySelector: (sel: string) => tooltipMock,
    };

    resetTooltipPosition(wrapperMock);

    assert.strictEqual(tooltipMock.style.display, '');
    assert.strictEqual(tooltipMock.style.visibility, '');
    assert.strictEqual(tooltipMock.style.transform, '');
    assert.strictEqual(tooltipMock.style.top, '');
    assert.strictEqual(tooltipMock.style.bottom, '');
    assert.strictEqual(tooltipMock.classList.classes.has('tooltip-flipped'), false);
  });

  test('wireInfoBubbleListeners attaches delegated listeners and unbinds cleanly', () => {
    const listeners: Record<string, Function[]> = {};
    const containerMock: any = {
      addEventListener: (event: string, cb: Function) => {
        listeners[event] = listeners[event] || [];
        listeners[event].push(cb);
      },
      removeEventListener: (event: string, cb: Function) => {
        if (listeners[event]) {
          listeners[event] = listeners[event].filter((f) => f !== cb);
        }
      },
      querySelector: () => null,
    };

    const unbind = wireInfoBubbleListeners(containerMock);

    assert.ok(listeners.mouseover && listeners.mouseover.length > 0, 'Must register mouseover');
    assert.ok(listeners.mouseout && listeners.mouseout.length > 0, 'Must register mouseout');
    assert.ok(listeners.focusin && listeners.focusin.length > 0, 'Must register focusin');
    assert.ok(listeners.focusout && listeners.focusout.length > 0, 'Must register focusout');
    assert.ok(listeners.keydown && listeners.keydown.length > 0, 'Must register keydown');
    assert.ok(listeners.scroll && listeners.scroll.length > 0, 'Must register scroll');

    // Unbind cleanly
    unbind();
    assert.strictEqual(listeners.mouseover.length, 0, 'Must remove mouseover');
    assert.strictEqual(listeners.mouseout.length, 0, 'Must remove mouseout');
    assert.strictEqual(listeners.focusin.length, 0, 'Must remove focusin');
    assert.strictEqual(listeners.focusout.length, 0, 'Must remove focusout');
    assert.strictEqual(listeners.keydown.length, 0, 'Must remove keydown');
    assert.strictEqual(listeners.scroll.length, 0, 'Must remove scroll');
  });
});
