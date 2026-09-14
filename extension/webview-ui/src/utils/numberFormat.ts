/**
 * Universal Compact Number Formatter.
 * Invariant X: Presentation Separation.
 * Strict Invariant: Output MUST NOT exceed 5 characters across all scales.
 *
 * Mathematical Length Verification:
 *   [0 .. 999]              -> value.toString()           (1..3 chars) e.g. "543"
 *   [1,000 .. 9,999]        -> (val/1k).toFixed(2) + 'k'  (max 5 chars) e.g. "4.23k", "9.99k"
 *   [10,000 .. 99,999]      -> (val/1k).toFixed(1) + 'k'  (max 5 chars) e.g. "13.3k", "99.9k"
 *   [100,000 .. 999,999]    -> Math.round(val/1k) + 'k'   (max 4 chars) e.g. "100k", "543k"
 *   [1M .. 9,999,999]       -> (val/1M).toFixed(2) + 'M'  (max 5 chars) e.g. "1.04M", "9.99M"
 *   [10M .. 99,999,999]     -> (val/1M).toFixed(1) + 'M'  (max 5 chars) e.g. "10M", "12.5M"
 *   [100M .. 999,999,999]   -> Math.round(val/1M) + 'M'   (max 4 chars) e.g. "100M", "123M"
 *   [1B .. 9,999,999,999]   -> (val/1B).toFixed(2) + 'B'  (max 5 chars) e.g. "1B", "1.04B"
 *   [10B .. 99,999,999,999] -> (val/1B).toFixed(1) + 'B'  (max 5 chars) e.g. "10B", "12.5B"
 *   [>= 100B]               -> Math.round(val/1B) + 'B'   (max 4..5 chars) e.g. "100B", "999B"
 */
export function formatCompactNumber(value: number | null | undefined): string {
  if (value === null || value === undefined || isNaN(value) || value <= 0) {
    return '0';
  }

  let res: string;
  if (value < 1000) {
    res = Math.round(value).toString();
  } else if (value < 10000) {
    res = (value / 1000).toFixed(2).replace(/\.?0+$/, '') + 'k';
  } else if (value < 100000) {
    res = (value / 1000).toFixed(1).replace(/\.?0+$/, '') + 'k';
  } else if (value < 1000000) {
    res = Math.round(value / 1000) + 'k';
  } else if (value < 10000000) {
    res = (value / 1000000).toFixed(2).replace(/\.?0+$/, '') + 'M';
  } else if (value < 100000000) {
    res = (value / 1000000).toFixed(1).replace(/\.?0+$/, '') + 'M';
  } else if (value < 1000000000) {
    res = Math.round(value / 1000000) + 'M';
  } else if (value < 10000000000) {
    res = (value / 1000000000).toFixed(2).replace(/\.?0+$/, '') + 'B';
  } else if (value < 100000000000) {
    res = (value / 1000000000).toFixed(1).replace(/\.?0+$/, '') + 'B';
  } else {
    res = Math.round(value / 1000000000) + 'B';
  }

  // Strict invariant guarantee: length <= 5
  if (res.length > 5) {
    if (res.endsWith('k')) {
      const v = parseFloat(res);
      res = (v / 1000).toFixed(1).replace(/\.?0+$/, '') + 'M';
    } else if (res.endsWith('M')) {
      const v = parseFloat(res);
      res = (v / 1000).toFixed(1).replace(/\.?0+$/, '') + 'B';
    }
  }

  return res.length <= 5 ? res : res.slice(0, 5);
}
