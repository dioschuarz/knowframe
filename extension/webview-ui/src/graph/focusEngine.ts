/**
 * Focus Engine
 *
 * Implements hop-constrained subgraph filtering and horizon controls.
 * Supported Focus Modes:
 * - 'hub': Level 0 Central Hub only
 * - '1_hop': Central Hub + direct neighbors (hop <= 1)
 * - '2_hops': Extended 2-hop neighborhood (hop <= 2)
 * - 'all': Entire graph
 */

import type { NoteNode, NoteEdge, FocusMode } from '../../../src/types/graph.ts';

export interface FilterResult {
  nodes: NoteNode[];
  edges: NoteEdge[];
}

export function filterGraphByFocus(
  nodes: NoteNode[],
  edges: NoteEdge[],
  focusMode: FocusMode,
  showOrphans: boolean = true,
): FilterResult {
  let maxHop: number;
  switch (focusMode) {
    case 'hub':
      maxHop = 0;
      break;
    case '1_hop':
      maxHop = 1;
      break;
    case '2_hops':
      maxHop = 2;
      break;
    case 'all':
    default:
      maxHop = Infinity;
      break;
  }

  const filteredNodes = nodes.filter((n) => {
    // If it's an orphan (hopDistance < 0)
    if (n.hopDistance < 0) {
      if (focusMode !== 'all') {
        return false;
      }
      return showOrphans;
    }

    // Connected note: test hop horizon
    return n.hopDistance <= maxHop;
  });

  const nodeIds = new Set<string>(filteredNodes.map((n) => n.id));

  const filteredEdges = edges.filter((e) => {
    const srcId = typeof e.source === 'object' ? (e.source as any).id : e.source;
    const tgtId = typeof e.target === 'object' ? (e.target as any).id : e.target;
    return nodeIds.has(srcId) && nodeIds.has(tgtId);
  });

  return {
    nodes: filteredNodes,
    edges: filteredEdges,
  };
}
