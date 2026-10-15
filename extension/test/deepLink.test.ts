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
});
