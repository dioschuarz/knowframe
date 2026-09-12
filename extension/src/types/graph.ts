/**
 * Graph domain entities and types for LLM Wiki Extension.
 */

export type NodeStatus = 'active' | 'connected_inactive' | 'orphan' | 'excluded';
export type LinkResolutionStatus =
  | 'resolved'
  | 'resolved_excluded'
  | 'broken_missing_file'
  | 'broken_missing_anchor'
  | 'broken_wikilink';
export type LinkType = 'markdown' | 'wikilink';
export type TokenBudgetStatus = 'normal' | 'warning' | 'critical';
export type ClusterQueryType = 'tag' | 'path';
export type LabelsMode = 'hover' | 'all';
export type FocusMode = 'hub' | '1_hop' | '2_hops' | '3_hops' | 'all';

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
  resolutionStatus?: LinkResolutionStatus;
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

export interface ExclusionRule {
  id: string;
  pattern: string;
  scope: 'scan' | 'graph';
  matchingCount?: number;
  createdAt: string;
}

export interface NoiseSuggestion {
  pattern: string;
  matchingCount: number;
  reason: string;
  recommended: boolean;
}

export interface ObservatorySummary {
  totalNotes: number;
  connectedNotes: number;
  orphanNotes: number;
  excludedNotes: number;
  brokenLinksCount: number;
  activeTokens: number;
  reachableTokens: number;
  syncStatus: 'synced' | 'syncing' | 'error';
  lastSyncTimestamp: number;
}

export interface ScanSettings {
  include: string[];
  exclude: string[];
}

export interface GraphSettings {
  exclude: string[];
  focusMode?: FocusMode;
  showOrphans?: boolean;
  showExcluded?: boolean;
}

export interface ContextVaultConfig {
  version: string;
  vault: {
    name: string;
    mode: 'single-repo' | 'multi-repo' | 'vault';
    autoDiscoveredHub?: string | null;
    hubs: string[];
  };
  include?: string[];
  exclude?: string[];
  scan?: ScanSettings;
  graph?: GraphSettings;
  tokenBudget?: {
    warningThreshold: number;
    criticalThreshold: number;
  };
  colorGroups?: ClusterColorGroup[];
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
  observatorySummary?: ObservatorySummary;
  focusMode?: FocusMode;
  showOrphans?: boolean;
  showExcluded?: boolean;
}
