import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderActionsToolbar } from '../../webview-ui/src/components/actionsToolbar.ts';
import type { TokenSummary } from '../../src/types/graph.ts';

function createMockElement(tagName = 'div'): any {
  const children: any[] = [];
  const attributes: Record<string, string> = {};
  const listeners: Record<string, Function[]> = {};

  const element: any = {
    tagName: tagName.toUpperCase(),
    style: {},
    classList: {
      contains: () => false,
      add: () => {},
      remove: () => {},
      toggle: () => {},
    },
    getAttribute: (attr: string) => attributes[attr] || null,
    setAttribute: (attr: string, val: string) => { attributes[attr] = val; },
    addEventListener: (evt: string, fn: Function) => {
      listeners[evt] = listeners[evt] || [];
      listeners[evt].push(fn);
    },
    removeEventListener: () => {},
    dispatchEvent: () => true,
    querySelector: (selector: string) => findSelector(element, selector),
    querySelectorAll: (selector: string) => findAllSelectors(element, selector),
    appendChild: (child: any) => { children.push(child); return child; },
    textContent: '',
    value: '',
  };

  let innerHtmlContent = '';
  Object.defineProperty(element, 'innerHTML', {
    get() {
      return innerHtmlContent;
    },
    set(val: string) {
      innerHtmlContent = val;
      parseHtmlToMock(val, element);
    },
  });

  return element;
}

function parseHtmlToMock(html: string, parent: any): void {
  const idRegex = /id="([^"]+)"/g;
  let match;
  parent._elementsById = parent._elementsById || {};
  while ((match = idRegex.exec(html)) !== null) {
    const id = match[1];
    parent._elementsById[id] = createMockElement('div');
    parent._elementsById[id].id = id;
  }
}

function findSelector(parent: any, selector: string): any {
  if (selector.startsWith('#')) {
    const id = selector.slice(1);
    if (parent._elementsById && parent._elementsById[id]) {
      return parent._elementsById[id];
    }
  }
  return createMockElement('div');
}

function findAllSelectors(_parent: any, _selector: string): any[] {
  return [];
}

suite('ActionsToolbar & Token Economics Test Suite (T048)', () => {
  test('renders toolbar with Token Economics popover including context reduction (FR-033)', () => {
    const container = createMockElement('div');
    const controller = renderActionsToolbar(container, {});

    assert.ok(container.innerHTML.includes('Token Economics'), 'Must include Token Economics title');
    assert.ok(container.innerHTML.includes('Context Reduction:'), 'Must include Context Reduction row');
    assert.ok(container.innerHTML.includes('popover-context-reduction'), 'Must include reduction element id');

    const summary: TokenSummary = {
      activeHarnessTokens: 4920,
      subcallAccessibleTokens: 18240,
      warningThreshold: 60000,
      criticalThreshold: 100000,
      status: 'normal',
      contextReductionPercent: 88.5,
    };

    controller.updateTokenSummary(summary);

    const reductionEl = container.querySelector('#popover-context-reduction');
    assert.strictEqual(reductionEl.textContent, '88.5%', 'Must render exact formatted context reduction');
  });

  test('defaults context reduction to ~90% when contextReductionPercent is omitted', () => {
    const container = createMockElement('div');
    const controller = renderActionsToolbar(container, {});

    const summary: TokenSummary = {
      activeHarnessTokens: 5000,
      subcallAccessibleTokens: 20000,
      warningThreshold: 60000,
      criticalThreshold: 100000,
      status: 'normal',
    };

    controller.updateTokenSummary(summary);

    const reductionEl = container.querySelector('#popover-context-reduction');
    assert.strictEqual(reductionEl.textContent, '~90%', 'Must default to ~90% context reduction');
  });

  test('renders Rescan Vault menu item in actions dropdown', () => {
    const container = createMockElement('div');
    renderActionsToolbar(container, {});

    assert.ok(container.innerHTML.includes('id="menu-act-rescan"'), 'Must include menu-act-rescan element');
    assert.ok(container.innerHTML.includes('Rescan Vault'), 'Must include Rescan Vault title in actions menu');
  });
});
