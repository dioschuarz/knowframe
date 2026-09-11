/**
 * Strongly typed JSON-RPC 2.0 protocol definitions matching contracts/
 */

export interface JsonRpcRequest<T = unknown> {
  jsonrpc: '2.0';
  id: string | number;
  method: string;
  params?: T;
}

export interface JsonRpcResponse<T = unknown> {
  jsonrpc: '2.0';
  id: string | number;
  result?: T;
  error?: JsonRpcError;
}

export interface JsonRpcError {
  code: number;
  message: string;
  data?: unknown;
}

export interface JsonRpcNotification<T = unknown> {
  jsonrpc: '2.0';
  method: string;
  params: T;
}

export interface NodePayload {
  id: string;
  label: string;
  file_path: string;
  tags: string[];
  color: string;
  status: 'active' | 'connected_inactive' | 'orphan';
  tokens: number;
  hop_distance: number;
  in_degree: number;
  out_degree: number;
  betweenness: number;
  broken_links: number;
}

export interface EdgePayload {
  id: string;
  source: string;
  target: string;
  is_broken: boolean;
  link_type: 'markdown' | 'wikilink';
  anchor?: string | null;
  line?: number | null;
}

export interface TokenSummaryPayload {
  active_harness_tokens: number;
  subcall_accessible_tokens: number;
  warning_threshold: number;
  critical_threshold: number;
  status: 'normal' | 'warning' | 'critical';
}

export interface ScanResultPayload {
  nodes: NodePayload[];
  edges: EdgePayload[];
  token_summary: TokenSummaryPayload;
  recommended_hub: string | null;
}

export interface ChangedFileItem {
  path: string;
  status: 'added' | 'modified' | 'deleted';
}

export interface GitCommitDeltaPayload {
  repo_path: string;
  commit_hash?: string;
  branch?: string;
  changed_files: ChangedFileItem[];
}

export interface OpenFilePayload {
  file_path: string;
  line?: number;
}

export interface SetHubPayload {
  hub_id: string;
}

export interface UpdateColorGroupsPayload {
  color_groups: {
    query: string;
    color: string;
    label: string;
    enabled?: boolean;
  }[];
}

export type WebviewInboundType =
  | 'openFile'
  | 'setHub'
  | 'updateColorGroups'
  | 'fixBrokenLinks'
  | 'rescanVault';

export interface WebviewInboundMessage {
  requestId?: string;
  type: WebviewInboundType;
  payload?: unknown;
}

export type WebviewOutboundType =
  | 'graphDataUpdate'
  | 'deltaUpdate'
  | 'tokenBudgetAlert'
  | 'sidecarStatus'
  | 'error';

export interface WebviewOutboundMessage {
  requestId?: string;
  type: WebviewOutboundType;
  payload: unknown;
}
