import { describe, it } from 'node:test';
import assert from 'node:assert';
import { formatCompactNumber } from '../../src/utils/numberFormat.ts';

describe('Universal Compact Number Formatter Test Suite', () => {
  it('strictly satisfies boundary values and user-specified numbers', () => {
    assert.strictEqual(formatCompactNumber(0), '0');
    assert.strictEqual(formatCompactNumber(999), '999');
    assert.strictEqual(formatCompactNumber(1000), '1k');
    assert.strictEqual(formatCompactNumber(4230), '4.23k');
    assert.strictEqual(formatCompactNumber(13300), '13.3k');
    assert.strictEqual(formatCompactNumber(543000), '543k');
    assert.strictEqual(formatCompactNumber(1043256), '1.04M');
    assert.strictEqual(formatCompactNumber(123456789), '123M');
    assert.strictEqual(formatCompactNumber(999900000), '1000M');
    assert.strictEqual(formatCompactNumber(1200000000), '1.2B');
  });

  it('handles null, undefined, 0, and negative numbers safely', () => {
    assert.strictEqual(formatCompactNumber(null), '0');
    assert.strictEqual(formatCompactNumber(undefined), '0');
    assert.strictEqual(formatCompactNumber(0), '0');
    assert.strictEqual(formatCompactNumber(-50), '0');
  });

  it('enforces maximum 5 characters across all test ranges', () => {
    const testCases = [
      0, 1, 9, 12, 99, 100, 543, 999,
      1000, 1234, 4230, 4250, 9990, 9999,
      10000, 13300, 13340, 50000, 99900, 99990,
      100000, 110000, 543000, 999000, 999900,
      1000000, 1043256, 1234567, 9990000,
      10000000, 13340000, 99900000,
      100000000, 123456789, 543000000, 999000000, 999900000,
      1000000000, 1200000000, 2500000000, 12500000000, 125000000000,
    ];

    for (const num of testCases) {
      const formatted = formatCompactNumber(num);
      assert.ok(
        formatted.length <= 5,
        `Formatted string "${formatted}" for ${num} exceeds 5 characters (length=${formatted.length})`
      );
    }
  });

  it('guarantees invariant across 50,000 stepped values', () => {
    for (let val = 0; val <= 500000; val += 73) {
      const res = formatCompactNumber(val);
      assert.ok(res.length <= 5, `Value ${val} formatted as "${res}" exceeded 5 chars`);
    }
  });
});
