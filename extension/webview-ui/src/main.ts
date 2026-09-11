import { CanvasRenderer } from './graph/canvasRenderer';
import { createPhysicsSimulation } from './graph/physics';
import { WebviewRpcBridge } from './rpc/webviewRpc';
import { renderActionsToolbar } from './components/actionsToolbar';
import { renderColorManager } from './components/colorManager';
import { renderInspectorDrawer, InspectorDrawerController } from './components/inspectorDrawer';
import { renderExclusionModal, ExclusionModalController } from './components/exclusionModal';
import { renderSettingsDrawer, SettingsDrawerController, type SettingsTab } from './components/settingsDrawer';
import { filterNodesByQuery, SearchMatchOptions } from './graph/searchMatcher';
import { filterGraphByFocus } from './graph/focusEngine';
import {
  NoteNode,
  NoteEdge,
  TokenSummary,
  ClusterColorGroup,
  PhysicsParameters,
  DEFAULT_PHYSICS_PARAMS,
  LabelsMode,
  FocusMode,
} from '../../src/types/graph';

const rpc = new WebviewRpcBridge();

const canvas = document.getElementById('graph-canvas') as HTMLCanvasElement;
const toolbarContainer = document.getElementById('toolbar-container') as HTMLElement;
const colorModalContainer = document.getElementById('color-modal-container') as HTMLElement;
const inspectorContainer = document.getElementById('inspector-container') as HTMLElement;
const exclusionModalContainer = document.getElementById('exclusion-modal-container') as HTMLElement;
const settingsContainer = document.getElementById('settings-container') as HTMLElement;
const tooltip = document.getElementById('node-tooltip') as HTMLElement;

let rawNodes: NoteNode[] = [];
let rawEdges: NoteEdge[] = [];
let nodes: NoteNode[] = [];
let edges: NoteEdge[] = [];
let colorGroups: ClusterColorGroup[] = [];
let selectedNode: NoteNode | null = null;
let lastTokenSummary: TokenSummary | null = null;
let currentPreviewPattern = '';

const savedState = rpc.getState<any>();
let currentFocusMode: FocusMode = (savedState?.focusMode as FocusMode) || 'all';
let currentShowOrphans: boolean = savedState?.showOrphans !== undefined ? savedState.showOrphans : true;

const exclusionModal = renderExclusionModal(exclusionModalContainer, {
  onApply: (pattern, scope) => {
    rpc.excludePath(pattern, scope);
  },
});

const settingsDrawer = renderSettingsDrawer(
  settingsContainer,
  {
    activeTab: 'knowledge',
    primaryHub: null,
    brokenLinksCount: 0,
    scanExclusions: [],
    graphExclusions: [],
    colorGroups: colorGroups,
    gitHooksInstalled: true,
    gitCommitLatencyMs: 15,
  },
  {
    onChangeHub: () => {
      rpc.sendMessage({ type: 'setHub', payload: {} });
    },
    onFixBrokenLinks: () => {
      rpc.fixBrokenLinks();
    },
    onRescanVault: () => {
      rpc.rescanVault();
    },
    onAddExclusion: (pattern, scope) => {
      rpc.excludePath(pattern, scope);
    },
    onInstallGitHooks: () => {
      rpc.sendMessage({ type: 'installHooks' });
    },
  },
);

function inspectNode(node: NoteNode): void {
  selectedNode = node;
  const inLinks: string[] = [];
  const outLinks: string[] = [];
  for (const edge of edges) {
    const srcId = typeof edge.source === 'object' ? (edge.source as any).id : edge.source;
    const tgtId = typeof edge.target === 'object' ? (edge.target as any).id : edge.target;
    if (tgtId === node.id || tgtId === node.label) {
      inLinks.push(srcId);
    }
    if (srcId === node.id || srcId === node.label) {
      outLinks.push(tgtId);
    }
  }
  const isHub = node.hopDistance === 0;
  inspectorDrawer.inspectNode(node, inLinks, outLinks, isHub);
}

const inspectorDrawer = renderInspectorDrawer(inspectorContainer, {
  onOpenFile: (filePath) => {
    rpc.openFile(filePath);
  },
  onOpenSource: (sourcePath, line) => {
    rpc.openFile(sourcePath, line);
  },
  onSetHub: (hubId) => {
    rpc.setHub(hubId);
  },
  onExclude: (filePath) => {
    currentPreviewPattern = filePath;
    rpc.previewExclusion(filePath);
  },
  onFixReference: (edge) => {
    const srcId = typeof edge.source === 'object' ? (edge.source as any).id : edge.source;
    const tgtId = typeof edge.target === 'object' ? (edge.target as any).id : edge.target;
    rpc.fixBrokenLink(srcId, edge.line, tgtId);
  },
  onNavigateLink: (target) => {
    const targetNode = nodes.find((n) => n.id === target || n.label === target || n.filePath === target);
    if (targetNode) {
      inspectNode(targetNode);
      renderer.centerOnNode(targetNode.id);
    } else {
      rpc.openFile(target);
    }
  },
  onClose: () => {
    selectedNode = null;
  },
});

// Initialize physics parameters and labels mode from saved state or default
let currentPhysicsParams: PhysicsParameters = savedState?.physicsParams
  ? { ...DEFAULT_PHYSICS_PARAMS, ...savedState.physicsParams }
  : { ...DEFAULT_PHYSICS_PARAMS };
let currentLabelsMode: LabelsMode = savedState?.labelsMode === 'all' ? 'all' : 'hover';

const physics = createPhysicsSimulation(
  window.innerWidth,
  window.innerHeight,
  () => {
    renderer.draw();
  },
  currentPhysicsParams,
);

const renderer = new CanvasRenderer({
  canvas,
  initialLabelsMode: currentLabelsMode,
  onNodeClick: (node) => {
    inspectNode(node);
  },
  onEdgeClick: (edge) => {
    const srcId = typeof edge.source === 'object' ? (edge.source as any).id : edge.source;
    const srcNode = nodes.find((n) => n.id === srcId);
    inspectorDrawer.inspectEdge(edge, srcNode?.label || (typeof srcId === 'string' ? srcId : ''));
  },
  onNodeHover: (node, x, y) => {
    if (node) {
      const isPinned = node.fx !== null && node.fx !== undefined;
      tooltip.style.display = 'block';
      tooltip.style.left = `${x + 12}px`;
      tooltip.style.top = `${y + 12}px`;
      tooltip.innerHTML = `
        <div class="tooltip-title">${node.label}</div>
        <div class="tooltip-meta">Tokens: ${node.tokens.toLocaleString()}</div>
        <div class="tooltip-meta">Hop Depth: ${node.hopDistance >= 0 ? 'Level ' + node.hopDistance : 'Orphan'}</div>
        <div class="tooltip-meta">Degree: In ${node.inDegree} / Out ${node.outDegree}</div>
        ${isPinned ? '<div class="tooltip-meta" style="color: #F59E0B; font-weight: bold;">📌 Position Fixed (Double-click to unpin)</div>' : ''}
        ${node.brokenLinks > 0 ? `<div class="tooltip-warn">Broken links: ${node.brokenLinks}</div>` : ''}
      `;
    } else {
      tooltip.style.display = 'none';
    }
  },
  onNodeDragStart: (node) => {
    tooltip.style.display = 'none';
    physics.onDragStart(node);
  },
  onNodeDrag: (node, x, y) => {
    physics.onDrag(node, x, y);
  },
  onNodeDragEnd: (node, x, y) => {
    const isSticky = toolbarController ? toolbarController.isStickyDrag() : true;
    physics.onDragEnd(node, x, y, isSticky);
    toolbarController?.updatePinnedCount(physics.getPinnedCount());
    persistState();
  },
  onNodeDoubleClick: (node) => {
    physics.unpinNode(node);
    toolbarController?.updatePinnedCount(physics.getPinnedCount());
    persistState();
  },
  onViewTransformChange: () => {
    persistState();
  },
});

// Restore saved pan/zoom transform
if (savedState?.viewTransform) {
  renderer.setViewTransform(savedState.viewTransform);
}
if (savedState?.colorGroups && Array.isArray(savedState.colorGroups) && savedState.colorGroups.length > 0) {
  colorGroups = savedState.colorGroups;
}

window.addEventListener('resize', () => {
  renderer.resize(window.innerWidth, window.innerHeight);
  physics.resize(window.innerWidth, window.innerHeight);
});

function applyFocusFilter(reheatAlpha: number = 0.3): void {
  const result = filterGraphByFocus(rawNodes, rawEdges, currentFocusMode, currentShowOrphans);
  nodes = result.nodes;
  edges = result.edges;
  renderer.setData(nodes, edges);
  physics.updateData(nodes, edges);
  physics.restart(reheatAlpha);
  applySearchFilter();
  persistState();
}

function persistState(): void {
  const pinnedNodes = rawNodes
    .filter((n) => n.fx !== null && n.fx !== undefined)
    .map((n) => ({ id: n.id, fx: n.fx, fy: n.fy }));

  rpc.setState({
    tokenSummary: lastTokenSummary,
    physicsParams: currentPhysicsParams,
    labelsMode: currentLabelsMode,
    focusMode: currentFocusMode,
    showOrphans: currentShowOrphans,
    showExcluded: renderer.getShowExcluded(),
    colorGroups,
    viewTransform: renderer.getViewTransform(),
    pinnedNodes,
  });
}

function applyClientSideColorGroups(nodeList: NoteNode[], groups: ClusterColorGroup[]): void {
  for (const node of nodeList) {
    let matchedColor: string | null = null;
    for (const group of groups) {
      if (!group.enabled) continue;
      if (group.query.startsWith('tag:')) {
        const pat = group.query.slice(4).trim().toLowerCase().replace(/^#/, '');
        if (node.tags?.some((t) => t.toLowerCase().replace(/^#/, '') === pat)) {
          matchedColor = group.color;
          break;
        }
      } else if (group.query.startsWith('path:')) {
        let pat = group.query.slice(5).trim();
        const nodePath = node.filePath || node.id;
        if (pat.endsWith('/**')) {
          const prefix = pat.slice(0, -3);
          if (nodePath.startsWith(prefix)) {
            matchedColor = group.color;
            break;
          }
        } else {
          try {
            const regexStr = '^' + pat.replace(/\./g, '\\.').replace(/\*\*/g, '.*').replace(/\*/g, '[^/]*') + '$';
            const re = new RegExp(regexStr);
            if (re.test(nodePath)) {
              matchedColor = group.color;
              break;
            }
          } catch {
            // ignore
          }
        }
      }
    }
    if (matchedColor) {
      node.color = matchedColor;
    }
  }
}

let currentSearchQuery = '';
let currentSearchOptions: SearchMatchOptions = { mode: 'contains', caseSensitive: false };

function applySearchFilter(): void {
  const matchedIds = filterNodesByQuery(nodes, currentSearchQuery, currentSearchOptions);
  renderer.setSearchFilter(matchedIds);
  if (matchedIds !== null) {
    toolbarController.setSearchMatchCount(matchedIds.size, nodes.length);
  } else {
    toolbarController.setSearchMatchCount(0, nodes.length);
  }
  renderer.draw();
}

// Render Actions Toolbar with Icon Popovers and Physics Controls
const toolbarController = renderActionsToolbar(toolbarContainer, {
  onResetZoom: () => {
    renderer.resetZoom();
  },
  onPhysicsChange: (params) => {
    currentPhysicsParams = params;
    physics.updateParameters(params);
    persistState();
  },
  onToggleLabelsMode: (mode) => {
    currentLabelsMode = mode;
    renderer.setLabelsMode(mode);
    renderer.draw();
    persistState();
  },
  onSearchChange: (query: string, options: SearchMatchOptions) => {
    currentSearchQuery = query;
    currentSearchOptions = options;
    applySearchFilter();
  },
  onReheat: () => {
    physics.restart(0.5);
  },
  onUnpinAll: () => {
    physics.unpinAll();
    toolbarController.updatePinnedCount(0);
    persistState();
  },
  onFocusModeChange: (mode) => {
    currentFocusMode = mode;
    applyFocusFilter(0.3);
  },
  onToggleOrphans: (show) => {
    currentShowOrphans = show;
    applyFocusFilter(0.3);
  },
  onToggleExcluded: (show) => {
    renderer.setShowExcluded(show);
    persistState();
  },
  onOpenSettings: (tab) => {
    settingsDrawer.open((tab as SettingsTab) || 'knowledge');
  },
  onOpenTokenBreakdown: () => {
    if (lastTokenSummary) {
      const topNodes = [...rawNodes]
        .sort((a, b) => b.tokens - a.tokens)
        .slice(0, 10)
        .map((n) => ({ label: n.label, filePath: n.filePath, tokens: n.tokens }));
      inspectorDrawer.inspectTokenEconomics(lastTokenSummary, topNodes);
    }
  },
  onToggleColorManager: () => {
    renderColorManager(
      colorModalContainer,
      colorGroups,
      {
        onSaveGroups: (newGroups) => {
          colorGroups = newGroups;
          rpc.updateColorGroups(newGroups);
          applyClientSideColorGroups(rawNodes, newGroups);
          applyFocusFilter(0.2);
        },
        onClose: () => {
          colorModalContainer.innerHTML = '';
        },
      },
      rawNodes,
    );
  },
});

// Set initial physics parameters and labels mode in toolbar UI
toolbarController.setPhysicsParameters(currentPhysicsParams);
toolbarController.setLabelsMode(currentLabelsMode);
if (toolbarController.setFocusMode) toolbarController.setFocusMode(currentFocusMode);
if (toolbarController.setShowOrphans) toolbarController.setShowOrphans(currentShowOrphans);
if (toolbarController.setShowExcluded && savedState?.showExcluded !== undefined) {
  toolbarController.setShowExcluded(savedState.showExcluded);
}

// Listen for updates from VS Code Extension Host
rpc.on('graphDataUpdate', (data: any) => {
  rawNodes = data.nodes || [];
  rawEdges = data.edges || [];
  if (data.color_groups && data.color_groups.length > 0) {
    colorGroups = data.color_groups;
  }

  // Restore any saved pinned positions if not already set
  if (savedState?.pinnedNodes && Array.isArray(savedState.pinnedNodes)) {
    const pinMap = new Map<string, any>(savedState.pinnedNodes.map((p: any) => [p.id, p]));
    for (const n of rawNodes) {
      if (pinMap.has(n.id) && (n.fx === undefined || n.fx === null)) {
        const pin = pinMap.get(n.id);
        n.fx = pin.fx;
        n.fy = pin.fy;
      }
    }
    savedState.pinnedNodes = [];
  }

  if (colorGroups.length > 0) {
    applyClientSideColorGroups(rawNodes, colorGroups);
  }

  applyFocusFilter(0.4);

  const brokenLinksCount = rawEdges.filter((e) => e.isBroken).length;
  const connectedCount = rawNodes.filter((n) => n.hopDistance >= 0).length;
  const orphanCount = rawNodes.filter((n) => n.hopDistance < 0).length;
  const excludedCount = rawNodes.filter((n) => n.status === 'excluded').length;

  settingsDrawer.updateProps({
    primaryHub: data.recommended_hub || (rawNodes[0] ? rawNodes[0].label : null),
    brokenLinksCount,
    totalNotes: rawNodes.length,
    connectedNotes: connectedCount,
    orphanNotes: orphanCount,
    excludedNotes: excludedCount,
    colorGroups,
  });

  if (data.tokenSummary) {
    lastTokenSummary = data.tokenSummary;
    toolbarController.updateTokenSummary(data.tokenSummary);
  }

  toolbarController.updatePinnedCount(physics.getPinnedCount());
  persistState();
});

// Recover state on tab restoration
if (savedState && savedState.tokenSummary) {
  lastTokenSummary = savedState.tokenSummary;
  toolbarController.updateTokenSummary(savedState.tokenSummary);
}
if (savedState?.showExcluded !== undefined) {
  renderer.setShowExcluded(savedState.showExcluded);
}

rpc.on('exclusionPreviewResult', (data: any) => {
  const matchingCount = data?.matching_count ?? data?.matchingCount ?? 0;
  const samplePaths = data?.sample_paths ?? data?.samplePaths ?? [];
  exclusionModal.show({
    pattern: currentPreviewPattern,
    matchingCount,
    samplePaths,
    scope: 'graph',
  });
});

// Request initial or refreshed graph data from Extension Host
rpc.send('requestGraphData');

