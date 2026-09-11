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
  },
  onNodeDoubleClick: (node) => {
    physics.unpinNode(node);
    toolbarController?.updatePinnedCount(physics.getPinnedCount());
  },
});

window.addEventListener('resize', () => {
  renderer.resize(window.innerWidth, window.innerHeight);
  physics.resize(window.innerWidth, window.innerHeight);
});

function persistState(): void {
  rpc.setState({
    tokenSummary: lastTokenSummary,
    physicsParams: currentPhysicsParams,
    labelsMode: currentLabelsMode,
  });
}

// Render Actions Toolbar with Icon Popovers and Physics Controls
const toolbarController = renderActionsToolbar(toolbarContainer, {
  onSetHub: () => {
    if (selectedNode) {
      rpc.setHub(selectedNode.id);
    } else {
      alert('Please click a note on the graph first to select it as Hub.');
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
  },
  onToggleColorManager: () => {
    renderColorManager(colorModalContainer, colorGroups, {
      onSaveGroups: (newGroups) => {
        colorGroups = newGroups;
        rpc.updateColorGroups(newGroups);
      },
      onClose: () => {
        colorModalContainer.innerHTML = '';
      },
    });
  },
});

// Set initial physics parameters and labels mode in toolbar UI
toolbarController.setPhysicsParameters(currentPhysicsParams);
toolbarController.setLabelsMode(currentLabelsMode);

// Listen for updates from VS Code Extension Host
rpc.on('graphDataUpdate', (data: any) => {
  nodes = data.nodes || [];
  edges = data.edges || [];
  colorGroups = data.color_groups || [];

  renderer.setData(nodes, edges);
  physics.updateData(nodes, edges);

  if (data.tokenSummary) {
    lastTokenSummary = data.tokenSummary;
    toolbarController.updateTokenSummary(data.tokenSummary);
  }

  persistState();
});

// Recover state on tab restoration
if (savedState && savedState.tokenSummary) {
  lastTokenSummary = savedState.tokenSummary;
  toolbarController.updateTokenSummary(savedState.tokenSummary);
}
