import { CanvasRenderer } from './graph/canvasRenderer';
import { createPhysicsSimulation } from './graph/physics';
import { WebviewRpcBridge } from './rpc/webviewRpc';
import { renderTokenCounter } from './components/tokenCounter';
import { renderActionsToolbar } from './components/actionsToolbar';
import { renderColorManager } from './components/colorManager';
import { NoteNode, NoteEdge, TokenSummary, ClusterColorGroup } from '../../src/types/graph';

const rpc = new WebviewRpcBridge();

const canvas = document.getElementById('graph-canvas') as HTMLCanvasElement;
const tokenContainer = document.getElementById('token-container') as HTMLElement;
const toolbarContainer = document.getElementById('toolbar-container') as HTMLElement;
const colorModalContainer = document.getElementById('color-modal-container') as HTMLElement;
const tooltip = document.getElementById('node-tooltip') as HTMLElement;

let nodes: NoteNode[] = [];
let edges: NoteEdge[] = [];
let colorGroups: ClusterColorGroup[] = [];
let selectedNode: NoteNode | null = null;

const renderer = new CanvasRenderer({
  canvas,
  onNodeClick: (node) => {
    selectedNode = node;
    rpc.openFile(node.filePath);
  },
  onNodeHover: (node, x, y) => {
    if (node) {
      tooltip.style.display = 'block';
      tooltip.style.left = `${x + 12}px`;
      tooltip.style.top = `${y + 12}px`;
      tooltip.innerHTML = `
        <div class="tooltip-title">${node.label}</div>
        <div class="tooltip-meta">Tokens: ${node.tokens.toLocaleString()}</div>
        <div class="tooltip-meta">Hop Depth: ${node.hopDistance >= 0 ? 'Level ' + node.hopDistance : 'Orphan'}</div>
        <div class="tooltip-meta">Degree: In ${node.inDegree} / Out ${node.outDegree}</div>
        ${node.brokenLinks > 0 ? `<div class="tooltip-warn">Broken links: ${node.brokenLinks}</div>` : ''}
      `;
    } else {
      tooltip.style.display = 'none';
    }
  },
});

const physics = createPhysicsSimulation(window.innerWidth, window.innerHeight, () => {
  renderer.draw();
});

window.addEventListener('resize', () => {
  renderer.resize(window.innerWidth, window.innerHeight);
});

// Render Actions Toolbar
renderActionsToolbar(toolbarContainer, {
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

// Listen for updates from VS Code Extension Host
rpc.on('graphDataUpdate', (data: any) => {
  nodes = data.nodes || [];
  edges = data.edges || [];
  colorGroups = data.color_groups || [];

  renderer.setData(nodes, edges);
  physics.updateData(nodes, edges);

  if (data.tokenSummary) {
    renderTokenCounter(tokenContainer, data.tokenSummary);
  }

  rpc.setState({ nodes, edges, tokenSummary: data.tokenSummary });
});

// Recover state on tab restoration
const savedState = rpc.getState<any>();
if (savedState) {
  nodes = savedState.nodes || [];
  edges = savedState.edges || [];
  renderer.setData(nodes, edges);
  physics.updateData(nodes, edges);
  if (savedState.tokenSummary) {
    renderTokenCounter(tokenContainer, savedState.tokenSummary);
  }
}
