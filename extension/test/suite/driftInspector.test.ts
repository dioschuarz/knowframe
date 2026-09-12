import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderDriftInspectorHtml } from '../../webview-ui/src/components/driftInspector.ts';

suite('DriftInspector Component Test Suite', () => {
  test('Renders drift card with doc citation coordinates and code targets', () => {
    const html = renderDriftInspectorHtml({
      docPath: 'docs/architecture.md',
      codeTarget: 'crates/core/src/auth/service.rs',
      symbolName: 'AuthService',
      citationLine: 48,
      driftType: 'deleted_code_file',
      explanation: "Referenced code file 'crates/core/src/auth/service.rs' was deleted in staged changes.",
    });

    assert.ok(html.includes('Technical Drift Contradiction'), 'Must show drift title');
    assert.ok(html.includes('deleted_code_file'), 'Must show drift type');
    assert.ok(html.includes('docs/architecture.md:48'), 'Must show doc citation file and line');
    assert.ok(html.includes('crates/core/src/auth/service.rs'), 'Must show deleted code target');
    assert.ok(html.includes('AuthService'), 'Must show symbol badge');
    assert.ok(html.includes('Open Document'), 'Must render Open Document button');
    assert.ok(html.includes('Open Code Reference'), 'Must render Open Code button');
  });
});
