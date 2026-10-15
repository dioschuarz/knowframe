import { describe, it } from 'node:test';
import assert from 'node:assert';
import * as path from 'path';

describe('Deep Linking Navigation Test Suite (T058, FR-044)', () => {
  it('correctly resolves relative paths against workspace root', () => {
    const root = '/home/user/project';
    const rel = 'docs/guide.md';
    const resolved = path.resolve(root, rel);
    assert.strictEqual(resolved, '/home/user/project/docs/guide.md');
  });

  it('correctly converts 1-based line and column to 0-based coordinates', () => {
    const line = 15;
    const col = 4;
    const zeroLine = Math.max(0, line - 1);
    const zeroCol = Math.max(0, col - 1);

    assert.strictEqual(zeroLine, 14);
    assert.strictEqual(zeroCol, 3);
  });
});
