/**
 * Hue + Opacity composition engine for LLM Wiki knowledge graph.
 * Enforces Constitution Principle IV:
 * Base hue is defined by cluster rules or defaults.
 * Alpha (opacity) is strictly dictated by BFS hop distance from active Hub:
 * - Level 0: 1.0 (100% solid)
 * - Level 1: 0.6 (60%)
 * - Level 2: 0.3 (30%)
 * - Level 3+: 0.15 (15%)
 * - Orphan: 0.15 if cluster rule match, or 0.4 with default #6B7280
 */

export function resolveNodeColor(
  baseHex: string,
  hopDistance: number,
  isCustomCluster: boolean,
): string {
  const cleanHex = baseHex.replace('#', '');
  const r = parseInt(cleanHex.substring(0, 2), 16) || 0;
  const g = parseInt(cleanHex.substring(2, 4), 16) || 0;
  const b = parseInt(cleanHex.substring(4, 6), 16) || 0;

  let alpha = 0.15;
  if (hopDistance === 0) {
    alpha = 1.0;
  } else if (hopDistance === 1) {
    alpha = 0.6;
  } else if (hopDistance === 2) {
    alpha = 0.3;
  } else if (hopDistance >= 3) {
    alpha = 0.15;
  } else if (hopDistance === -1) {
    alpha = isCustomCluster ? 0.15 : 0.4;
  }

  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}
