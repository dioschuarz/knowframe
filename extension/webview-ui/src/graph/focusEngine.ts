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
  issueNodeIds?: Set<string>,
): FilterResult {
  if (focusMode === 'issues_only') {
    const targetSet = issueNodeIds || new Set<string>();
    const filteredNodes = nodes.filter(
      (n) => targetSet.has(n.id) || (n.filePath && targetSet.has(n.filePath)) || n.brokenLinks > 0,
    );
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
    case '3_hops':
      maxHop = 3;
      break;
    case 'all':
    default:
      maxHop = Infinity;
      break;
  }

  const filteredNodes = nodes.filter((n) => {
    const isOrphan = n.hopDistance < 0 && (n.inDegree || 0) === 0 && (n.outDegree || 0) === 0;
    if (isOrphan) {
      if (focusMode !== 'all') {
        return false;
      }
      return showOrphans;
    }

    if (n.hopDistance < 0) {
      // Connected via inbound or outbound reference, shown in 'all' view regardless of showOrphans
      return focusMode === 'all';
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
