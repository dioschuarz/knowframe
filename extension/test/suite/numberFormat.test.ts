import { describe, it } from 'node:test';
import assert from 'node:assert';
import { formatCompactNumber } from '../../webview-ui/src/utils/numberFormat.ts';

describe('Universal Compact Number Formatter Test Suite', () => {
  it('strictly satisfies user-specified examples: 543, 13.3k, 0.11M', () => {
    assert.strictEqual(formatCompactNumber(543), '543');
    assert.strictEqual(formatCompactNumber(13300), '13.3k');
    assert.strictEqual(formatCompactNumber(110000), '0.11M');
  });

  it('handles null, undefined, 0, and negative numbers safely', () => {
    assert.strictEqual(formatCompactNumber(null), '0');
    assert.strictEqual(formatCompactNumber(undefined), '0');
    assert.strictEqual(formatCompactNumber(0), '0');
    assert.strictEqual(formatCompactNumber(-50), '0');
  });

  it('enforces maximum 3 digits and maximum 5 characters across all test ranges', () => {
    const testCases = [
      0, 1, 9, 12, 99, 100, 543, 999,
      1000, 1234, 4250, 9990, 9999,
      10000, 13300, 13340, 50000, 99900, 99990,
      100000, 110000, 543000, 999000, 999900,
      1000000, 1234567, 9990000,
      10000000, 13340000, 99900000,
      100000000, 543000000, 999000000,
      1000000000, 2500000000,
    ];

    for (const num of testCases) {
      const formatted = formatCompactNumber(num);
      const digitsCount = (formatted.match(/\d/g) || []).length;
      assert.ok(
        formatted.length <= 5,
        `Formatted string "${formatted}" for ${num} exceeds 5 characters (length=${formatted.length})`
      );
      assert.ok(
        digitsCount <= 3,
        `Formatted string "${formatted}" for ${num} exceeds 3 digits (digits=${digitsCount})`
      );
    }
  });

  it('guarantees invariant across 50,000 stepped values', () => {
    for (let val = 0; val <= 500000; val += 73) {
      const res = formatCompactNumber(val);
      const digits = (res.match(/\d/g) || []).length;
      assert.ok(res.length <= 5, `Value ${val} formatted as "${res}" exceeded 5 chars`);
      assert.ok(digits <= 3, `Value ${val} formatted as "${res}" exceeded 3 digits`);
    }
  });
});
