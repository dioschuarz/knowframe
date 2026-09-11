import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { generateNonce, buildCspPolicy } from '../../src/webview/cspHelper.ts';

suite('Webview Security & CSP Test Suite', () => {
  test('Generated nonce has length 32 and alphanumeric characters', () => {
    const nonce = generateNonce();
    assert.strictEqual(nonce.length, 32);
    assert.match(nonce, /^[a-zA-Z0-9]{32}$/);
  });

  test('CSP policy includes cryptographic nonce and restricts script execution', () => {
    const nonce = generateNonce();
    const csp = buildCspPolicy(nonce, 'vscode-webview:');
    assert.ok(csp.includes(`script-src 'nonce-${nonce}'`));
    assert.ok(csp.includes("default-src 'none'"));
  });
});
