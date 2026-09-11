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
  fx?: number | null;
  fy?: number | null;
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

export interface PhysicsParameters {
  centerGravity: number;
  linkDistance: number;
  repulsion: number;
  collisionRadius: number;
}

export const DEFAULT_PHYSICS_PARAMS: PhysicsParameters = {
  centerGravity: 0.15,
  linkDistance: 80,
  repulsion: 250,
  collisionRadius: 18,
};

export const PHYSICS_PRESETS: Record<'compact' | 'balanced' | 'spacious', PhysicsParameters> = {
  compact: {
    centerGravity: 0.35,
    linkDistance: 50,
    repulsion: 160,
    collisionRadius: 14,
  },
  balanced: {
    centerGravity: 0.15,
    linkDistance: 80,
    repulsion: 250,
    collisionRadius: 18,
  },
  spacious: {
    centerGravity: 0.08,
    linkDistance: 160,
    repulsion: 450,
    collisionRadius: 24,
  },
};

export interface GraphData {
  nodes: NoteNode[];
  edges: NoteEdge[];
  tokenSummary: TokenSummary;
  recommendedHub: string | null;
  activeHub: string | null;
  physicsParams?: PhysicsParameters;
}
