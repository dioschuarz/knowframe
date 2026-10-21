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
import { renderTransformationPlanModalHtml } from './components/transformationPlanModal';
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
const transformationModalContainer = document.getElementById('transformation-modal-container') as HTMLElement;
const tooltip = document.getElementById('node-tooltip') as HTMLElement;

let rawNodes: NoteNode[] = [];
let rawEdges: NoteEdge[] = [];
let nodes: NoteNode[] = [];
let edges: NoteEdge[] = [];
let colorGroups: ClusterColorGroup[] = [];
let selectedNode: NoteNode | null = null;
let lastTokenSummary: TokenSummary | null = null;
let currentPreviewPattern = '';

// Transformation Plan Modal State
let currentTransformationPlan: any = null;
let isTransformationModalOpen = false;
let isApplyingTransformationPlan = false;
let isTransformationPlanApplied = false;
let actualTransformationDelta: any = null;
let applyTransformationPlanError: string | null = null;

function renderTransformationModal(): void {
  if (!transformationModalContainer) return;
  transformationModalContainer.innerHTML = renderTransformationPlanModalHtml({
    plan: currentTransformationPlan,
    isOpen: isTransformationModalOpen,
    isApplying: isApplyingTransformationPlan,
    isApplied: isTransformationPlanApplied,
    actualDelta: actualTransformationDelta,
    applyError: applyTransformationPlanError,
  });

  if (isTransformationModalOpen) {
    const closeTransformationModal = (e?: Event) => {
      e?.stopPropagation();
      isTransformationModalOpen = false;
      renderTransformationModal();
    };

    transformationModalContainer.querySelector('#btn-close-plan-modal')?.addEventListener('click', closeTransformationModal);
    transformationModalContainer.querySelector('#btn-cancel-plan')?.addEventListener('click', closeTransformationModal);

    const overlay = transformationModalContainer.querySelector('#transformation-modal');
    overlay?.addEventListener('click', (e) => {
      if (e.target === overlay) {
        closeTransformationModal(e);
      }
    });

    transformationModalContainer.querySelector('#btn-apply-plan-changes')?.addEventListener('click', (e) => {
      e.stopPropagation();
      if (currentTransformationPlan && !isApplyingTransformationPlan && !isTransformationPlanApplied) {
        isApplyingTransformationPlan = true;
        applyTransformationPlanError = null;
        renderTransformationModal();
        const plan = currentTransformationPlan;
        const approvalToken = plan.approval_token || plan.approvalToken;
        const planId = plan.planId || plan.plan_id;
        if (approvalToken && planId) {
          rpc.applyGovernancePlan(planId, approvalToken);
        } else {
          rpc.applyTransformationPlan(currentTransformationPlan);
        }
      }
    });
  }
}

window.addEventListener('keydown', (e: KeyboardEvent) => {
  if (e.key === 'Escape' && isTransformationModalOpen) {
    isTransformationModalOpen = false;
    renderTransformationModal();
  }
});

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
    nodes: rawNodes,
    gitHooksInstalled: true,
    llmWikiSkillAvailable: false,
    gitCommitLatencyMs: 15,
  },
  {
    onChangeHub: () => {
      // An empty setHub payload is ignored by the panel bridge. Use the
      // dedicated message that opens the extension-host Quick Pick instead.
      rpc.sendMessage({ type: 'triggerSetHubQuickPick' });
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
    onDeleteExclusion: (pattern, scope) => {
      rpc.removeExclusion(pattern, scope);
    },
    onRequestExclusions: () => {
      rpc.requestExclusions();
    },
    onSaveColorGroups: (newGroups) => {
      colorGroups = newGroups;
      rpc.updateColorGroups(newGroups);
      applyClientSideColorGroups(rawNodes, newGroups);
      applyFocusFilter(0.2);
      persistState();
    },
    onInstallGitHooks: () => {
      rpc.sendMessage({ type: 'installHooks' });
    },
    onInstallLLMWikiSkill: () => {
      rpc.sendMessage({ type: 'installLLMWikiSkill' });
    },
    onOpenFile: (filePath, line) => {
      rpc.openFile(filePath, line);
    },
    onSelectActiveHarness: (harnessFile) => {
      rpc.setActiveHarness(harnessFile);
    },
    onInitializeHarness: (targetHarness) => {
      rpc.initializeHarness(targetHarness);
    },
    onConfigureHarnessBase: (options) => {
      rpc.configureHarnessBase(options);
    },
    onSyncHarnesses: () => {
      rpc.syncHarnesses();
    },
    onRequestSkillCatalog: () => {
      rpc.getSkillCatalog();
    },
    onFixBrokenLink: (source, target, line) => {
      rpc.fixBrokenLink(source, line, target);
    },
    onRequestIssues: (pillar) => {
      rpc.sendMessage({ type: 'requestIssues', payload: { pillar } });
    },
    onRequestIssuesWithOptions: (options) => {
      if (options?.include_debt) {
        renderer.setShowAllUnanchored(true);
      } else {
        renderer.setShowAllUnanchored(false);
      }
      rpc.sendMessage({ type: 'requestIssues', payload: options });
    },
    onRequestEvolution: () => {
      rpc.getEvolutionState();
    },
    onInstallHarnessTool: (toolId) => {
      rpc.installHarnessTool(toolId);
    },
    onRequestHarnessTools: () => {
      rpc.requestHarnessTools();
    },
    onRequestHarnessStatus: () => {
      rpc.getHarnessStatus();
    },
    onChangeTab: (tab) => {
      if (tab === 'evolution') {
        rpc.getEvolutionState();
      }
      if (tab === 'exclusions') {
        rpc.requestExclusions();
      }
      if (tab === 'git_harness' || tab === 'git_sync' || tab === 'harness' || tab === 'git') {
        rpc.getHarnessStatus();
        rpc.getSkillCatalog();
        rpc.requestHarnessTools();
      }
    },
    onSelectOrientation: (orientation) => {
      rpc.setOrientation(orientation);
    },
    onSaveCustomConfig: (tier1, tier2, constraints) => {
      settingsDrawer.updateProps({ isSavingCustom: true });
      rpc.updateCustomConfig(tier1, tier2, constraints);
    },
    onResetCustomConfig: () => {
      rpc.getEvolutionState();
    },
    onValidateConstraints: (constraints) => {
      rpc.validateConstraints(constraints);
    },
    onGeneratePlan: (recId) => {
      settingsDrawer.updateProps({
        generatingPlanRecId: recId,
        planGenerationError: null,
      });
      rpc.generateTransformationPlan(recId);
    },
    onGeneratePlanWithStrategies: (opportunityId, selectedStrategies) => {
      settingsDrawer.updateProps({
        generatingPlanRecId: opportunityId,
        planGenerationError: null,
      });
      rpc.generateMultiStrategyPlan(opportunityId, selectedStrategies);
    },
    onGenerateGovernancePlan: (strategyType, targetFileOrDir) => {
      rpc.generateGovernancePlan(strategyType, targetFileOrDir);
    },
    onReviewPlan: (planId) => {
      if (
        currentTransformationPlan &&
        (currentTransformationPlan.planId === planId || currentTransformationPlan.plan_id === planId)
      ) {
        isTransformationModalOpen = true;
        renderTransformationModal();
      }
    },
    onApplyPlan: (planId) => {
      const planToApply =
        currentTransformationPlan &&
        (currentTransformationPlan.planId === planId || currentTransformationPlan.plan_id === planId)
          ? currentTransformationPlan
          : planId;
      rpc.applyTransformationPlan(planToApply);
    },
    onMarkReviewed: (docId) => {
      rpc.resolveDocumentReview([docId]);
    },
    onOpen: () => {
      if (typeof inspectorDrawer !== 'undefined' && inspectorDrawer) {
        inspectorDrawer.close();
      }
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
    const p = typeof filePath === 'object' ? (filePath as any).filePath || (filePath as any).id : filePath;
    rpc.openFile(p);
  },
  onOpenSource: (sourcePath, line) => {
    const p = typeof sourcePath === 'object' ? (sourcePath as any).filePath || (sourcePath as any).id : sourcePath;
    rpc.openFile(p, line);
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
  onMarkReviewed: (docId) => {
    rpc.resolveDocumentReview([docId]);
  },
  onClose: () => {
    selectedNode = null;
  },
  onOpen: () => {
    if (typeof settingsDrawer !== 'undefined' && settingsDrawer) {
      settingsDrawer.close();
    }
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
        ${isPinned ? '<div class="tooltip-meta" style="color: #F59E0B; font-weight: 600; display: flex; align-items: center; gap: 4px;"><svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="#F59E0B" stroke-width="1.5"><line x1="8" y1="1" x2="8" y2="10"/><circle cx="8" cy="4" r="3"/><line x1="8" y1="10" x2="8" y2="15"/></svg>Position Fixed (Double-click to reset)</div>' : ''}
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
          if (nodePath.startsWith(prefix) || nodePath.replace(/^notes\//, '').startsWith(prefix)) {
            matchedColor = group.color;
            break;
          }
        } else {
          try {
            const regexStr = '^' + pat.replace(/\./g, '\\.').replace(/\*\*/g, '.*').replace(/\*/g, '[^/]*') + '$';
            const re = new RegExp(regexStr);
            if (re.test(nodePath) || re.test(nodePath.replace(/^notes\//, ''))) {
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
    } else if (node.status === 'excluded') {
      node.color = '#475569';
    } else if (node.hopDistance < 0) {
      node.color = '#6B7280';
    } else {
      node.color = '#64748B'; // Default grey: indicates no custom color rule set
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
  onRescanVault: () => {
    rpc.rescanVault();
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
    inspectorDrawer.close();
    settingsDrawer.updateProps({
      nodes: rawNodes,
      colorGroups,
      tokenSummary: lastTokenSummary,
    });
    settingsDrawer.open((tab as SettingsTab) || 'knowledge');
  },
  onOpenTokenBreakdown: () => {
    inspectorDrawer.close();
    settingsDrawer.updateProps({
      nodes: rawNodes,
      colorGroups,
      tokenSummary: lastTokenSummary,
    });
    settingsDrawer.open('load');
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

const loadingOverlay = document.getElementById('graph-loading-overlay') as HTMLElement | null;
const loadingCard = document.getElementById('loading-card') as HTMLElement | null;
const noWorkspaceCard = document.getElementById('no-workspace-card') as HTMLElement | null;
const loadingTimeoutActions = document.getElementById('loading-timeout-actions') as HTMLElement | null;
const loadingPercent = document.getElementById('loading-percent') as HTMLElement | null;
const loadingStage = document.getElementById('loading-stage') as HTMLElement | null;
const loadingBarFill = document.getElementById('loading-bar-fill') as HTMLElement | null;

let loadingTimeoutTimer: any = null;

function startLoadingTimeoutGuard(): void {
  clearLoadingTimeoutGuard();
  loadingTimeoutTimer = setTimeout(() => {
    if (loadingOverlay && !loadingOverlay.classList.contains('hidden') && loadingTimeoutActions) {
      loadingTimeoutActions.classList.remove('hidden');
    }
  }, 8000);
}

function clearLoadingTimeoutGuard(): void {
  if (loadingTimeoutTimer) {
    clearTimeout(loadingTimeoutTimer);
    loadingTimeoutTimer = null;
  }
}

function updateLoadingProgress(percent: number, stage: string): void {
  if (loadingPercent) loadingPercent.textContent = `${Math.round(percent)}%`;
  if (loadingStage) loadingStage.textContent = stage;
  if (loadingBarFill) loadingBarFill.style.width = `${Math.round(percent)}%`;
}

function hideLoadingOverlay(): void {
  clearLoadingTimeoutGuard();
  if (loadingOverlay) {
    loadingOverlay.classList.add('hidden');
  }
}

function showNoWorkspace(): void {
  clearLoadingTimeoutGuard();
  if (loadingOverlay) loadingOverlay.classList.remove('hidden');
  if (loadingCard) loadingCard.classList.add('hidden');
  if (noWorkspaceCard) noWorkspaceCard.classList.remove('hidden');
}

// Wire no-workspace and timeout actions
document.getElementById('btn-open-workspace')?.addEventListener('click', () => {
  rpc.sendMessage({ type: 'openFolder' });
});
document.getElementById('btn-open-folder-loading')?.addEventListener('click', () => {
  rpc.sendMessage({ type: 'openFolder' });
});
document.getElementById('btn-retry-loading')?.addEventListener('click', () => {
  if (loadingTimeoutActions) loadingTimeoutActions.classList.add('hidden');
  updateLoadingProgress(15, 'Retrying vault scan...');
  startLoadingTimeoutGuard();
  rpc.sendMessage({ type: 'rescanVault' });
});

startLoadingTimeoutGuard();


// Listen for updates from VS Code Extension Host
rpc.on('graphDataUpdate', (data: any) => {
  const isFirstLoad = nodes.length === 0;
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

  // Apply cluster color rules, or ensure default grey for unassigned nodes
  applyClientSideColorGroups(rawNodes, colorGroups);

  hideLoadingOverlay();
  applyFocusFilter(0.4);

  if (isFirstLoad && rawNodes.length > 0 && !savedState?.viewTransform) {
    const hub = rawNodes.find((n) => n.hopDistance === 0) || rawNodes[0];
    if (hub) {
      setTimeout(() => renderer.centerOnNode(hub.id), 120);
    }
  }

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
    nodes: rawNodes,
    tokenSummary: data.tokenSummary || lastTokenSummary,
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
  settingsDrawer.updateProps({ tokenSummary: savedState.tokenSummary });
}
if (savedState?.showExcluded !== undefined) {
  renderer.setShowExcluded(savedState.showExcluded);
}

rpc.on('noWorkspaceOpen', () => {
  showNoWorkspace();
});

rpc.on('engineOffline', (data: any) => {
  clearLoadingTimeoutGuard();
  if (loadingStage) loadingStage.textContent = data?.message || 'Engine daemon is offline.';
  if (loadingTimeoutActions) loadingTimeoutActions.classList.remove('hidden');
});

rpc.on('loadingProgress', (data: any) => {
  if (data) {
    if (loadingCard) loadingCard.classList.remove('hidden');
    if (noWorkspaceCard) noWorkspaceCard.classList.add('hidden');
    const p = typeof data.percent === 'number' ? data.percent : 0;
    const msg = data.message || 'Loading...';
    updateLoadingProgress(p, msg);
    if (data.stage === 'complete' || p >= 100) {
      clearLoadingTimeoutGuard();
      setTimeout(hideLoadingOverlay, 300);
    } else {
      startLoadingTimeoutGuard();
    }
  }
});


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

rpc.on('openSettingsDrawer', (data: any) => {
  inspectorDrawer.close();
  settingsDrawer.updateProps({
    nodes: rawNodes,
    colorGroups,
    tokenSummary: lastTokenSummary,
  });
  const tab = data?.tab || 'knowledge';
  if (tab === 'harness' || tab === 'git_harness' || tab === 'git') {
    rpc.getHarnessStatus();
    rpc.requestHarnessTools();
    rpc.getSkillCatalog();
  }
  settingsDrawer.open(tab);
});

rpc.on('navigateToDetail', (data: any) => {
  inspectorDrawer.close();
  const subTab = data?.subTab || 'health';
  const targetSection = data?.targetSection;
  if (targetSection) {
    settingsDrawer.updateProps({
      selectedPillarFilter: targetSection,
      tokenSummary: lastTokenSummary,
    });
  } else {
    settingsDrawer.updateProps({
      tokenSummary: lastTokenSummary,
    });
  }
  settingsDrawer.open(subTab);
});

rpc.on('issuesReportUpdate', (data: any) => {
  const activeUnanchored = new Set<string>();
  if (data?.issues && Array.isArray(data.issues)) {
    for (const issue of data.issues) {
      if ((issue.pillar === 'structure' || issue.category?.includes('unanchored')) && !issue.is_legacy) {
        if (issue.file_path && issue.target) {
          activeUnanchored.add(`${issue.file_path}->${issue.target}`);
        }
      }
    }
  }
  renderer.setActiveUnanchoredEdges(activeUnanchored);
  settingsDrawer.updateProps({
    issuesReport: data,
  });
});

rpc.on('exclusionsUpdate', (data: any) => {
  if (data) {
    settingsDrawer.updateProps({
      scanExclusions: data.scanExclusions || [],
      graphExclusions: data.graphExclusions || [],
    });
  }
});

rpc.on('evolutionStateUpdate', (data: any) => {
  if (data) {
    settingsDrawer.updateProps({
      activeOrientation: data.orientation,
      tier1: data.tier1,
      tier2: data.tier2,
      constraints: data.constraints,
      structuralSignatures: data.signatures || [],
      structuralRecommendations: data.recommendations || [],
      isSavingCustom: false,
      isEvolutionLoading: false,
    });
  }
});

rpc.on('harnessStatus', (data: any) => {
  const propsToUpdate: any = {
    llmWikiSkillAvailable: Boolean(data?.llmWikiSkillAvailable),
    harnessStatus: data,
  };
  if (data?.harnessTools || data?.tools) {
    propsToUpdate.harnessTools = data.harnessTools || data.tools;
  }
  settingsDrawer.updateProps(propsToUpdate);
});

rpc.on('harnessToolsUpdate', (data: any) => {
  settingsDrawer.updateProps({
    harnessTools: data?.tools || [],
  });
});

rpc.on('configureHarnessBaseResult', (res: any) => {
  const feedback = document.querySelector('#prep-base-feedback') as HTMLElement | null;
  if (feedback) {
    feedback.style.display = 'block';
    feedback.style.background = 'rgba(34, 197, 94, 0.15)';
    feedback.style.color = '#22c55e';
    feedback.style.border = '1px solid rgba(34, 197, 94, 0.3)';
    feedback.textContent = `✓ Base configured: ${res?.enabled_harnesses?.join(', ') || 'Ready'}.`;
  }
});

rpc.on('skillCatalogUpdate', (data: any) => {
  settingsDrawer.updateProps({
    skills: data?.skills || [],
    procedureCandidates: data?.procedure_candidates || [],
  });
});

rpc.on('directoryMetricsUpdate', (data: any) => {
  settingsDrawer.updateProps({
    directoryMetrics: data,
  });
});

rpc.on('directoryStructureMetrics', (data: any) => {
  settingsDrawer.updateProps({
    directoryMetrics: data,
  });
});

rpc.on('transformationPlanReady', (plan: any) => {
  if (plan) {
    settingsDrawer.updateProps({
      generatingPlanRecId: null,
      planGenerationError: null,
    });
    currentTransformationPlan = plan;
    isTransformationModalOpen = true;
    isApplyingTransformationPlan = false;
    isTransformationPlanApplied = false;
    applyTransformationPlanError = null;
    actualTransformationDelta = null;
    renderTransformationModal();
  }
});

rpc.on('transformationPlanError', (data: any) => {
  settingsDrawer.updateProps({
    generatingPlanRecId: null,
    planGenerationError: {
      recId: data?.recommendationId,
      message: data?.error || 'Failed to generate transformation plan',
    },
  });
});

rpc.on('transformationPlanApplied', (data: any) => {
  if (data) {
    isApplyingTransformationPlan = false;
    isTransformationPlanApplied = true;
    applyTransformationPlanError = null;
    actualTransformationDelta = data.actualDelta || data.actual_delta;
    renderTransformationModal();
  }
});

rpc.on('governancePlanApplied', (data: any) => {
  if (data) {
    isApplyingTransformationPlan = false;
    isTransformationPlanApplied = true;
    applyTransformationPlanError = null;
    actualTransformationDelta = data.actualDelta || data.actual_delta || data.re_measurement;
    renderTransformationModal();
    rpc.send('requestHarnessStatus');
    rpc.getSkillCatalog();
    rpc.analyzeDirectoryStructure();
  }
});

rpc.on('transformationPlanApplyError', (data: any) => {
  isApplyingTransformationPlan = false;
  isTransformationPlanApplied = false;
  applyTransformationPlanError = data?.error || 'Failed to apply transformation plan';
  console.error('[TransformationPlan] Apply failed:', applyTransformationPlanError);
  renderTransformationModal();
});

rpc.on('constraintValidationResult', (data: any) => {
  if (data) {
    settingsDrawer.updateProps({
      conflicts: data.conflicts || [],
    });
  }
});

// Request initial or refreshed graph data, harness, skills, and directory metrics from Extension Host
rpc.send('requestGraphData');
rpc.send('requestHarnessStatus');
rpc.getSkillCatalog();
rpc.analyzeDirectoryStructure();
rpc.requestExclusions();
rpc.getEvolutionState();
rpc.requestHarnessTools();
