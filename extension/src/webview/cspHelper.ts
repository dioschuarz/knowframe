
/**
 * Generates a cryptographically random 32-character alphanumeric nonce.
 */
export function generateNonce(): string {
  let text = '';
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

/**
 * Builds a strict Content Security Policy (CSP) string for the Webview.
 */
export function buildCspPolicy(nonce: string, webviewCspSource: string): string {
  return [
    `default-src 'none'`,
    `img-src ${webviewCspSource} https: data:`,
    `style-src ${webviewCspSource} 'unsafe-inline'`,
    `script-src 'nonce-${nonce}'`,
    `font-src ${webviewCspSource}`,
  ].join('; ');
}
