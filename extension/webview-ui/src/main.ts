import { CanvasRenderer } from './graph/canvasRenderer';
import { createPhysicsSimulation } from './graph/physics';
import { WebviewRpcBridge } from './rpc/webviewRpc';
import { renderActionsToolbar } from './components/actionsToolbar';
import { renderColorManager } from './components/colorManager';
import {
  NoteNode,
  NoteEdge,
  TokenSummary,
  ClusterColorGroup,
  PhysicsParameters,
  DEFAULT_PHYSICS_PARAMS,
  LabelsMode,
} from '../../src/types/graph';

const rpc = new WebviewRpcBridge();

const canvas = document.getElementById('graph-canvas') as HTMLCanvasElement;
const toolbarContainer = document.getElementById('toolbar-container') as HTMLElement;
const colorModalContainer = document.getElementById('color-modal-container') as HTMLElement;
const tooltip = document.getElementById('node-tooltip') as HTMLElement;

let nodes: NoteNode[] = [];
let edges: NoteEdge[] = [];
let colorGroups: ClusterColorGroup[] = [];
let selectedNode: NoteNode | null = null;
let lastTokenSummary: TokenSummary | null = null;

// Initialize physics parameters and labels mode from saved state or default
const savedState = rpc.getState<any>();
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
    selectedNode = node;
    rpc.openFile(node.filePath);
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

function persistState(): void {
  const pinnedNodes = nodes
    .filter((n) => n.fx !== null && n.fx !== undefined)
    .map((n) => ({ id: n.id, fx: n.fx, fy: n.fy }));

  rpc.setState({
    tokenSummary: lastTokenSummary,
    physicsParams: currentPhysicsParams,
    labelsMode: currentLabelsMode,
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

// Render Actions Toolbar with Icon Popovers and Physics Controls
const toolbarController = renderActionsToolbar(toolbarContainer, {
  onSetHub: () => {
    if (selectedNode) {
      rpc.setHub(selectedNode.id);
    } else {
      rpc.send('triggerSetHubQuickPick');
    }
  },
  onRescanVault: () => {
    rpc.rescanVault();
  },
  onFixBrokenLinks: () => {
    rpc.fixBrokenLinks();
  },
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
  onReheat: () => {
    physics.restart(0.5);
  },
  onUnpinAll: () => {
    physics.unpinAll();
    toolbarController.updatePinnedCount(0);
    persistState();
  },
  onToggleColorManager: () => {
    renderColorManager(
      colorModalContainer,
      colorGroups,
      {
        onSaveGroups: (newGroups) => {
          colorGroups = newGroups;
          rpc.updateColorGroups(newGroups);
          applyClientSideColorGroups(nodes, newGroups);
          renderer.draw();
          persistState();
        },
        onClose: () => {
          colorModalContainer.innerHTML = '';
        },
      },
      nodes,
    );
  },
});

// Set initial physics parameters and labels mode in toolbar UI
toolbarController.setPhysicsParameters(currentPhysicsParams);
toolbarController.setLabelsMode(currentLabelsMode);

// Listen for updates from VS Code Extension Host
rpc.on('graphDataUpdate', (data: any) => {
  nodes = data.nodes || [];
  edges = data.edges || [];
  if (data.color_groups && data.color_groups.length > 0) {
    colorGroups = data.color_groups;
  }

  // Restore any saved pinned positions if not already set
  if (savedState?.pinnedNodes && Array.isArray(savedState.pinnedNodes)) {
    const pinMap = new Map<string, any>(savedState.pinnedNodes.map((p: any) => [p.id, p]));
    for (const n of nodes) {
      if (pinMap.has(n.id) && (n.fx === undefined || n.fx === null)) {
        const pin = pinMap.get(n.id);
        n.fx = pin.fx;
        n.fy = pin.fy;
      }
    }
  }

  if (colorGroups.length > 0) {
    applyClientSideColorGroups(nodes, colorGroups);
  }

  renderer.setData(nodes, edges);
  physics.updateData(nodes, edges);

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

// Request initial or refreshed graph data from Extension Host
rpc.send('requestGraphData');

