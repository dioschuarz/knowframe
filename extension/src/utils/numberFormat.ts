/**
 * Universal Compact Number Formatter.
 * Enforces: Maximum 3 digits and maximum 5 characters total.
 *
 * Requirements:
 * - Maximum 3 digits (e.g., 543, 13.3k, 0.11M)
 * - Maximum 5 characters total (string length <= 5)
 *
 * Examples:
 *   543      -> 543     (3 digits, 3 chars)
 *   13300    -> 13.3k   (3 digits, 5 chars)
 *   110000   -> 0.11M   (3 digits, 5 chars)
 *   1234567  -> 1.23M   (3 digits, 5 chars)
 *   13340000 -> 13.3M   (3 digits, 5 chars)
 *   100000000-> 100M    (3 digits, 4 chars)
 */
export function formatCompactNumber(n: number | null | undefined): string {
  if (n === null || n === undefined || isNaN(n) || n <= 0) {
    return '0';
  }

  if (n < 1000) {
    return Math.round(n).toString();
  }

  if (n < 10000) {
    const val = (n / 1000).toFixed(2);
    if (parseFloat(val) >= 10) {
      return (n / 1000).toFixed(1) + 'k';
    }
    return val + 'k';
  }

  if (n < 100000) {
    const val = (n / 1000).toFixed(1);
    if (parseFloat(val) >= 100) {
      return '0.10M';
    }
    return val + 'k';
  }

  if (n < 1000000) {
    const val = (n / 1000000).toFixed(2);
    if (parseFloat(val) >= 1) {
      return '1.00M';
    }
    return val + 'M';
  }

  if (n < 10000000) {
    const val = (n / 1000000).toFixed(2);
    if (parseFloat(val) >= 10) {
      return (n / 1000000).toFixed(1) + 'M';
    }
    return val + 'M';
  }

  if (n < 100000000) {
    const val = (n / 1000000).toFixed(1);
    if (parseFloat(val) >= 100) {
      return '100M';
    }
    return val + 'M';
  }

  if (n < 1000000000) {
    return Math.round(n / 1000000) + 'M';
  }

  return (n / 1000000000).toFixed(1) + 'B';
}
