/**
 * Graph domain entities and types for LLM Wiki Extension.
 */

export type NodeStatus = 'active' | 'connected_inactive' | 'orphan';
export type LinkType = 'markdown' | 'wikilink';
export type TokenBudgetStatus = 'normal' | 'warning' | 'critical';
export type ClusterQueryType = 'tag' | 'path';

export interface NoteNode {
  id: string;
  label: string;
  filePath: string;
  tags: string[];
  color: string;
  status: NodeStatus;
  tokens: number;
  hopDistance: number;
  inDegree: number;
  outDegree: number;
  betweenness: number;
  brokenLinks: number;

  // d3-force visual coordinates
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
}

export interface NoteEdge {
  id: string;
  source: string | NoteNode;
  target: string | NoteNode;
  isBroken: boolean;
  linkType: LinkType;
  anchor?: string | null;
  line?: number | null;
}

export interface TokenSummary {
  activeHarnessTokens: number;
  subcallAccessibleTokens: number;
  warningThreshold: number;
  criticalThreshold: number;
  status: TokenBudgetStatus;
}

export interface ClusterColorGroup {
  query: string;
  color: string;
  label: string;
  enabled: boolean;
}

export interface GraphData {
  nodes: NoteNode[];
  edges: NoteEdge[];
  tokenSummary: TokenSummary;
  recommendedHub: string | null;
  activeHub: string | null;
}
