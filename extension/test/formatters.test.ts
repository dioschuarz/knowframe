import { describe, it } from 'node:test';
import assert from 'node:assert';
import { formatCompactNumber } from '../src/webview/formatters.ts';

describe('Formatters Unit Test Suite (T056, FR-030, NFR-007)', () => {
  it('strictly limits string length to <= 5 characters across all scales', () => {
    const numbers = [
      0, 1, 9, 99, 543, 999, 1000, 4230, 13300, 543000,
      1043256, 123456789, 999900000, 1200000000, 12500000000
    ];

    for (const n of numbers) {
      const res = formatCompactNumber(n);
      assert.ok(
        res.length <= 5,
        `Formatted string "${res}" for ${n} exceeds 5 characters (length=${res.length})`
      );
    }
  });
});
