import {
  TokenSummary,
  PhysicsParameters,
  DEFAULT_PHYSICS_PARAMS,
  PHYSICS_PRESETS,
  LabelsMode,
} from '../../../src/types/graph';

export interface ActionsToolbarCallbacks {
  onSetHub: () => void;
  onRescanVault: () => void;
  onFixBrokenLinks: () => void;
  onInstallHooks?: () => void;
  onToggleColorManager: () => void;
  onResetZoom?: () => void;
  onPhysicsChange?: (params: PhysicsParameters) => void;
  onReheat?: () => void;
  onUnpinAll?: () => void;
  onStickyToggle?: (enabled: boolean) => void;
  onToggleLabelsMode?: (mode: LabelsMode) => void;
  onSearchChange?: (query: string, exact: boolean) => void;
  onResetLayout?: () => void;
}

export interface ActionsToolbarController {
  updateTokenSummary: (summary: TokenSummary) => void;
  setPhysicsParameters: (params: PhysicsParameters) => void;
  updatePinnedCount: (count: number) => void;
  isStickyDrag: () => boolean;
  setLabelsMode: (mode: LabelsMode) => void;
  setSearchMatchCount: (matchingCount: number, totalCount: number) => void;
  openSearch: () => void;
  closeSearch: () => void;
}

export function renderActionsToolbar(
  container: HTMLElement,
  callbacks: ActionsToolbarCallbacks,
): ActionsToolbarController {
  let activePopover: 'none' | 'token' | 'physics' | 'actions' = 'none';

  container.innerHTML = `
    <div class="floating-toolbar-wrapper">
      <div class="floating-toolbar">
        <!-- Token Economics Icon Button -->
        <button id="btn-token-toggle" class="tb-icon-btn tb-token-btn" aria-label="Token Economics" data-tooltip-title="Token Economics" data-tooltip-desc="Active prompt tokens (L0) and subcall context (L1+) against budget">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M7 1L1 9h5l-1 6 7-8h-5l1-6z"/>
          </svg>
          <span id="tb-token-pill" class="tb-pill">0 tok</span>
        </button>

        <div class="tb-divider"></div>

        <!-- Search Notes Button -->
        <button id="btn-search-toggle" class="tb-icon-btn" aria-label="Search Graph" data-tooltip-title="Search Graph (Ctrl+F)" data-tooltip-desc="Filter nodes by title, filename, folder path, or tag with match highlighting">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="7" cy="7" r="4.5"/>
            <line x1="10.5" y1="10.5" x2="14.5" y2="14.5"/>
          </svg>
        </button>

        <!-- Gravity & Spacing Settings Button -->
        <button id="btn-physics-toggle" class="tb-icon-btn" aria-label="Gravity & Spacing" data-tooltip-title="Gravity & Spacing" data-tooltip-desc="Tune center gravity, node distance, repulsion force, and presets">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <line x1="2" y1="4" x2="14" y2="4"/>
            <line x1="2" y1="8" x2="14" y2="8"/>
            <line x1="2" y1="12" x2="14" y2="12"/>
            <circle cx="5" cy="4" r="1.6" fill="currentColor"/>
            <circle cx="11" cy="8" r="1.6" fill="currentColor"/>
            <circle cx="7" cy="12" r="1.6" fill="currentColor"/>
          </svg>
        </button>

        <!-- Set Hub Button -->
        <button id="btn-set-hub" class="tb-icon-btn" aria-label="Set Central Hub" data-tooltip-title="Set Central Hub" data-tooltip-desc="Select a note on graph first, then click here to set as Level 0 hub">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="8" cy="8" r="6"/>
            <circle cx="8" cy="8" r="2" fill="currentColor"/>
            <line x1="8" y1="1" x2="8" y2="3"/>
            <line x1="8" y1="13" x2="8" y2="15"/>
            <line x1="1" y1="8" x2="3" y2="8"/>
            <line x1="13" y1="8" x2="15" y2="8"/>
          </svg>
        </button>

        <!-- Clusters Button -->
        <button id="btn-toggle-colors" class="tb-icon-btn" aria-label="Manage Clusters" data-tooltip-title="Cluster Colors" data-tooltip-desc="Configure chromatic cluster rules mapped to #tags or directory paths">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 0a8 8 0 1 0 8 8 8 8 0 0 0-8-8zm-4.5 9.5a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm2-4a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm5 0a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm2 4a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5z"/>
          </svg>
        </button>

        <!-- Labels Mode Toggle Button -->
        <button id="btn-toggle-labels" class="tb-icon-btn" aria-label="File Names" data-tooltip-title="File Names: Hover Only (Clean)" data-tooltip-desc="Click to switch to Show All mode and display all note names">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M2.5 3A1.5 1.5 0 0 0 1 4.5v7A1.5 1.5 0 0 0 2.5 13h11a1.5 1.5 0 0 0 1.5-1.5v-7A1.5 1.5 0 0 0 13.5 3h-11zm2.854 7.354l1.5-4a.5.5 0 0 1 .94 0l1.5 4a.5.5 0 0 1-.936.352L7.886 9.5H6.468l-.472 1.206a.5.5 0 1 1-.936-.352zM7.177 7.7l-.469 1.2h.944l-.475-1.2zM11 7a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-1 0v-3A.5.5 0 0 1 11 7z"/>
          </svg>
        </button>

        <!-- Reset Zoom Button -->
        <button id="btn-reset-zoom" class="tb-icon-btn" aria-label="Reset Zoom & Center View" data-tooltip-title="Reset View" data-tooltip-desc="Reset pan coordinates and zoom scale back to centered 100%">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M1 5V1h4M15 5V1h-4M1 11v4h4M15 11v4h-4"/>
          </svg>
        </button>

        <!-- Reset Graph Disposition Button -->
        <button id="btn-reset-layout" class="tb-icon-btn" aria-label="Reset Graph Disposition" data-tooltip-title="Reset Graph Disposition" data-tooltip-desc="Return all nodes to default positions, unpin all notes, restore balanced gravity & re-center view">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <path d="M8 2.5a5.5 5.5 0 1 1-3.9 1.6L2 2"/>
            <path d="M2 6V2h4"/>
          </svg>
        </button>

        <div class="tb-divider"></div>

        <!-- Actions Menu Dropdown Button -->
        <button id="btn-actions-menu" class="tb-icon-btn" aria-label="Vault Actions" data-tooltip-title="Vault Actions & Tools" data-tooltip-desc="Rescan vault, fix broken links, install git hooks, and unpin notes">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <circle cx="8" cy="3" r="1.5"/>
            <circle cx="8" cy="8" r="1.5"/>
            <circle cx="8" cy="13" r="1.5"/>
          </svg>
        </button>
      </div>

      <!-- Instant Hover Tooltip -->
      <div id="toolbar-tooltip" class="tb-hover-tooltip" style="display: none;">
        <div class="tb-tt-title" id="tb-tt-title"></div>
        <div class="tb-tt-desc" id="tb-tt-desc"></div>
      </div>

      <!-- Search Bar Popover Window -->
      <div id="search-popover" class="floating-popover search-popover" style="display: none;">
        <div class="search-bar-inner">
          <svg class="search-input-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="7" cy="7" r="4.5"/>
            <line x1="10.5" y1="10.5" x2="14.5" y2="14.5"/>
          </svg>
          <input
            id="tb-search-input"
            type="text"
            class="search-input"
            placeholder="Search title, filename, folder, or #tag..."
            autocomplete="off"
            spellcheck="false"
          />
          <span id="tb-search-count" class="search-match-badge" style="display: none;">0 matches</span>
          <button id="btn-search-exact" class="search-option-btn" title="Toggle: Substring contains (default) vs Exact match">Aa</button>
          <button id="btn-search-clear" class="search-clear-btn" title="Clear search (Esc)" style="display: none;">✕</button>
        </div>
      </div>

      <!-- Token Economics Popover Window -->
      <div id="token-popover" class="floating-popover" style="display: none;">
        <div class="popover-header">
          <div class="popover-title-box">
            <span class="popover-title">Token Economics</span>
            <span id="popover-token-badge" class="popover-badge">NORMAL</span>
          </div>
          <button id="close-token-popover" class="popover-close-btn" title="Close">✕</button>
        </div>
        <div class="popover-body">
          <div class="popover-metric-row">
            <span class="p-lbl">Active Prompt Load (L0):</span>
            <strong id="popover-active-tokens" class="p-val">0</strong>
          </div>
          <div class="popover-metric-row">
            <span class="p-lbl">Accessible Subcall (L1+):</span>
            <strong id="popover-subcall-tokens" class="p-val">0</strong>
          </div>
          <div class="popover-bar-bg">
            <div id="popover-token-bar-fill" class="popover-bar-fill"></div>
          </div>
          <div class="popover-limits-row">
            <span>Warning: <strong id="popover-warn-limit">60,000</strong></span>
            <span>Critical: <strong id="popover-crit-limit">100,000</strong></span>
          </div>
        </div>
      </div>

      <!-- Physics & Gravity Popover Window -->
      <div id="physics-popover" class="floating-popover physics-popover" style="display: none;">
        <div class="popover-header">
          <div class="popover-title-box">
            <span class="popover-title">Graph Gravity & Spacing</span>
          </div>
          <button id="close-physics-popover" class="popover-close-btn" title="Close">✕</button>
        </div>
        <div class="popover-body">
          <!-- Presets -->
          <div class="physics-preset-section">
            <span class="p-lbl">Presets:</span>
            <div class="preset-btn-group">
              <button id="preset-compact" class="preset-btn" title="High gravity, compact clusters — best for whole vault overview">Compact</button>
              <button id="preset-balanced" class="preset-btn active" title="Standard balanced gravity and spacing">Balanced</button>
              <button id="preset-spacious" class="preset-btn" title="Low gravity, wide spacing — best for reading labels">Spacious</button>
            </div>
          </div>

          <!-- Labels Mode Toggle -->
          <div class="physics-toggle-row">
            <span class="p-lbl">File Names:</span>
            <div class="preset-btn-group">
              <button id="btn-lbl-hover" class="preset-btn active" title="Clean mode: names only appear when passing mouse over node">Hover Only (Clean)</button>
              <button id="btn-lbl-all" class="preset-btn" title="Display names for all notes">Show All</button>
            </div>
          </div>

          <!-- Sliders -->
          <div class="physics-row">
            <div class="physics-row-header">
              <span class="p-lbl">Center Gravity</span>
              <span id="badge-gravity" class="physics-num-badge">0.15</span>
            </div>
            <input type="range" id="slider-gravity" class="physics-slider" min="0" max="0.5" step="0.01" value="0.15">
            <div class="physics-hint">Pulls all nodes inward to prevent outer sprawl</div>
          </div>

          <div class="physics-row">
            <div class="physics-row-header">
              <span class="p-lbl">Node Distance (Spacing)</span>
              <span id="badge-distance" class="physics-num-badge">80px</span>
            </div>
            <input type="range" id="slider-distance" class="physics-slider" min="30" max="260" step="5" value="80">
            <div class="physics-hint">Resting link length between connected notes</div>
          </div>

          <div class="physics-row">
            <div class="physics-row-header">
              <span class="p-lbl">Node Repulsion (Charge)</span>
              <span id="badge-repulsion" class="physics-num-badge">250</span>
            </div>
            <input type="range" id="slider-repulsion" class="physics-slider" min="50" max="700" step="10" value="250">
            <div class="physics-hint">Pushes neighboring nodes apart to avoid overlap</div>
          </div>

          <div class="physics-row">
            <div class="physics-row-header">
              <span class="p-lbl">Collision Barrier</span>
              <span id="badge-collision" class="physics-num-badge">18px</span>
            </div>
            <input type="range" id="slider-collision" class="physics-slider" min="6" max="40" step="1" value="18">
            <div class="physics-hint">Minimum clearance padding around nodes</div>
          </div>

          <div class="physics-toggle-row">
            <label class="physics-checkbox-lbl" title="Keep moved nodes pinned in place until released">
              <input type="checkbox" id="chk-sticky-drag" checked>
              <span>Sticky Placement</span>
            </label>
            <button id="btn-unpin-all" class="physics-action-btn" title="Unpin all manually positioned notes">
              📌 Unpin (<span id="badge-pinned-count">0</span>)
            </button>
          </div>

          <div class="physics-footer">
            <button id="btn-reheat-sim" class="physics-action-btn" title="Re-simulate node positions">⚡ Reheat</button>
            <button id="btn-reset-physics" class="physics-action-btn" title="Reset all sliders to defaults">⚙️ Defaults</button>
            <button id="btn-reset-disposition" class="physics-action-btn physics-action-primary" title="Restore default node layout, unpin all notes, and re-center view">↺ Reset Layout</button>
          </div>
        </div>
      </div>

      <!-- Actions Dropdown Menu Window -->
      <div id="actions-dropdown" class="floating-popover actions-menu-popover" style="display: none;">
        <div class="menu-item" id="menu-act-search">
          <span class="m-icon">🔍</span>
          <div class="m-text">
            <div class="m-title">Search Graph</div>
            <div class="m-desc">Filter nodes by title, folder, or tag (Ctrl+F)</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-physics">
          <span class="m-icon">🎛️</span>
          <div class="m-text">
            <div class="m-title">Gravity & Spacing</div>
            <div class="m-desc">Tune node repulsion, link distance & center pull</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-toggle-labels">
          <span class="m-icon">🏷️</span>
          <div class="m-text">
            <div class="m-title" id="menu-title-labels">File Names: Hover Only</div>
            <div class="m-desc">Toggle between clean hover-only and showing all names</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-unpin">
          <span class="m-icon">📌</span>
          <div class="m-text">
            <div class="m-title">Unpin All Nodes</div>
            <div class="m-desc">Release all manually positioned notes</div>
          </div>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" id="menu-act-rescan">
          <span class="m-icon">🔄</span>
          <div class="m-text">
            <div class="m-title">Rescan Vault</div>
            <div class="m-desc">Re-run AST parsing & compute graph topology</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-fix-links">
          <span class="m-icon">🛠️</span>
          <div class="m-text">
            <div class="m-title">Fix Broken Links</div>
            <div class="m-desc">Inspect broken references & create missing notes</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-hooks">
          <span class="m-icon">⚙️</span>
          <div class="m-text">
            <div class="m-title">Install Git Hooks</div>
            <div class="m-desc">Configure prek / commit-delta background sync</div>
          </div>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" id="menu-act-reset-zoom">
          <span class="m-icon">⛶</span>
          <div class="m-text">
            <div class="m-title">Reset Canvas View</div>
            <div class="m-desc">Center graph and reset zoom level</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-reset-layout">
          <span class="m-icon">↺</span>
          <div class="m-text">
            <div class="m-title">Reset Graph Disposition</div>
            <div class="m-desc">Restore default node positions, unpin all, and re-center</div>
          </div>
        </div>
      </div>
    </div>
  `;

  const tokenPopover = container.querySelector('#token-popover') as HTMLElement;
  const physicsPopover = container.querySelector('#physics-popover') as HTMLElement;
  const actionsDropdown = container.querySelector('#actions-dropdown') as HTMLElement;
  const searchPopover = container.querySelector('#search-popover') as HTMLElement;

  const searchInput = container.querySelector('#tb-search-input') as HTMLInputElement;
  const searchMatchBadge = container.querySelector('#tb-search-count') as HTMLElement;
  const btnSearchToggle = container.querySelector('#btn-search-toggle') as HTMLElement;
  const btnSearchExact = container.querySelector('#btn-search-exact') as HTMLElement;
  const btnSearchClear = container.querySelector('#btn-search-clear') as HTMLElement;

  const sliderGravity = container.querySelector('#slider-gravity') as HTMLInputElement;
  const sliderDistance = container.querySelector('#slider-distance') as HTMLInputElement;
  const sliderRepulsion = container.querySelector('#slider-repulsion') as HTMLInputElement;
  const sliderCollision = container.querySelector('#slider-collision') as HTMLInputElement;

  const badgeGravity = container.querySelector('#badge-gravity') as HTMLElement;
  const badgeDistance = container.querySelector('#badge-distance') as HTMLElement;
  const badgeRepulsion = container.querySelector('#badge-repulsion') as HTMLElement;
  const badgeCollision = container.querySelector('#badge-collision') as HTMLElement;

  const btnCompact = container.querySelector('#preset-compact') as HTMLButtonElement;
  const btnBalanced = container.querySelector('#preset-balanced') as HTMLButtonElement;
  const btnSpacious = container.querySelector('#preset-spacious') as HTMLButtonElement;

  // Tooltip elements
  const tooltipEl = container.querySelector('#toolbar-tooltip') as HTMLElement;
  const ttTitle = container.querySelector('#tb-tt-title') as HTMLElement;
  const ttDesc = container.querySelector('#tb-tt-desc') as HTMLElement;
  const toolbarWrapper = container.querySelector('.floating-toolbar-wrapper') as HTMLElement;

  function showTooltip(btn: HTMLElement): void {
    if (activePopover !== 'none') return;
    const title = btn.getAttribute('data-tooltip-title');
    const desc = btn.getAttribute('data-tooltip-desc');
    if (!title || !tooltipEl) return;

    ttTitle.textContent = title;
    ttDesc.textContent = desc || '';

    // Render offscreen/hidden first to measure actual rendered bounding box
    tooltipEl.style.visibility = 'hidden';
    tooltipEl.style.display = 'block';

    const btnRect = btn.getBoundingClientRect();
    const tooltipRect = tooltipEl.getBoundingClientRect();

    // Calculate ideal horizontal center below button
    const btnCenterX = btnRect.left + (btnRect.width / 2);
    let left = btnCenterX - (tooltipRect.width / 2);

    // Dynamically clamp horizontally within viewport with 12px safety padding
    const padding = 12;
    const maxLeft = window.innerWidth - tooltipRect.width - padding;
    const minLeft = padding;
    left = Math.max(minLeft, Math.min(left, maxLeft));

    // Vertical positioning: place below button, flip above if overflowing bottom
    let top = btnRect.bottom + 8;
    if (top + tooltipRect.height > window.innerHeight - padding) {
      top = btnRect.top - tooltipRect.height - 8;
    }

    tooltipEl.style.left = `${Math.round(left)}px`;
    tooltipEl.style.top = `${Math.round(top)}px`;
    tooltipEl.style.visibility = 'visible';
  }

  function hideTooltip(): void {
    if (tooltipEl) {
      tooltipEl.style.display = 'none';
      tooltipEl.style.visibility = 'hidden';
    }
  }

  function closeAllPopovers(): void {
    tokenPopover.style.display = 'none';
    physicsPopover.style.display = 'none';
    actionsDropdown.style.display = 'none';
    searchPopover.style.display = 'none';
    activePopover = 'none';
    btnSearchToggle?.classList.toggle('active', !!searchInput?.value.trim());
    hideTooltip();
  }

  let isExactMatch = false;

  function emitSearch(): void {
    const query = searchInput.value;
    callbacks.onSearchChange?.(query, isExactMatch);
    if (query.trim().length > 0) {
      btnSearchClear.style.display = 'inline-block';
      btnSearchToggle.classList.add('active');
    } else {
      btnSearchClear.style.display = 'none';
      searchMatchBadge.style.display = 'none';
      btnSearchToggle.classList.remove('active');
    }
  }

  function openSearch(): void {
    closeAllPopovers();
    searchPopover.style.display = 'block';
    btnSearchToggle.classList.add('active');
    activePopover = 'search';
    setTimeout(() => {
      searchInput.focus();
      searchInput.select();
    }, 40);
  }

  function closeSearch(): void {
    searchPopover.style.display = 'none';
    if (activePopover === 'search') {
      activePopover = 'none';
    }
    btnSearchToggle.classList.toggle('active', !!searchInput.value.trim());
  }

  function toggleSearchPopover(e: Event): void {
    e.stopPropagation();
    if (activePopover === 'search') {
      closeAllPopovers();
    } else {
      openSearch();
    }
  }

  function toggleTokenPopover(e: Event): void {
    e.stopPropagation();
    if (activePopover === 'token') {
      closeAllPopovers();
    } else {
      closeAllPopovers();
      tokenPopover.style.display = 'block';
      activePopover = 'token';
    }
  }

  function togglePhysicsPopover(e: Event): void {
    e.stopPropagation();
    if (activePopover === 'physics') {
      closeAllPopovers();
    } else {
      closeAllPopovers();
      physicsPopover.style.display = 'block';
      activePopover = 'physics';
    }
  }

  function toggleActionsDropdown(e: Event): void {
    e.stopPropagation();
    if (activePopover === 'actions') {
      closeAllPopovers();
    } else {
      closeAllPopovers();
      actionsDropdown.style.display = 'block';
      activePopover = 'actions';
    }
  }

  function updatePresetButtons(activePreset: 'compact' | 'balanced' | 'spacious' | 'custom'): void {
    btnCompact?.classList.toggle('active', activePreset === 'compact');
    btnBalanced?.classList.toggle('active', activePreset === 'balanced');
    btnSpacious?.classList.toggle('active', activePreset === 'spacious');
  }

  function applyPhysicsToUi(params: PhysicsParameters): void {
    if (sliderGravity) sliderGravity.value = params.centerGravity.toString();
    if (sliderDistance) sliderDistance.value = params.linkDistance.toString();
    if (sliderRepulsion) sliderRepulsion.value = params.repulsion.toString();
    if (sliderCollision) sliderCollision.value = params.collisionRadius.toString();

    if (badgeGravity) badgeGravity.textContent = params.centerGravity.toFixed(2);
    if (badgeDistance) badgeDistance.textContent = `${params.linkDistance}px`;
    if (badgeRepulsion) badgeRepulsion.textContent = params.repulsion.toString();
    if (badgeCollision) badgeCollision.textContent = `${params.collisionRadius}px`;

    if (
      params.centerGravity === PHYSICS_PRESETS.compact.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.compact.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.compact.repulsion
    ) {
      updatePresetButtons('compact');
    } else if (
      params.centerGravity === PHYSICS_PRESETS.spacious.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.spacious.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.spacious.repulsion
    ) {
      updatePresetButtons('spacious');
    } else if (
      params.centerGravity === PHYSICS_PRESETS.balanced.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.balanced.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.balanced.repulsion
    ) {
      updatePresetButtons('balanced');
    } else {
      updatePresetButtons('custom');
    }
  }

  function handleSliderInput(): void {
    const params: PhysicsParameters = {
      centerGravity: parseFloat(sliderGravity.value),
      linkDistance: parseInt(sliderDistance.value, 10),
      repulsion: parseInt(sliderRepulsion.value, 10),
      collisionRadius: parseInt(sliderCollision.value, 10),
    };

    if (badgeGravity) badgeGravity.textContent = params.centerGravity.toFixed(2);
    if (badgeDistance) badgeDistance.textContent = `${params.linkDistance}px`;
    if (badgeRepulsion) badgeRepulsion.textContent = params.repulsion.toString();
    if (badgeCollision) badgeCollision.textContent = `${params.collisionRadius}px`;

    // Check if matches preset
    if (
      params.centerGravity === PHYSICS_PRESETS.compact.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.compact.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.compact.repulsion
    ) {
      updatePresetButtons('compact');
    } else if (
      params.centerGravity === PHYSICS_PRESETS.balanced.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.balanced.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.balanced.repulsion
    ) {
      updatePresetButtons('balanced');
    } else if (
      params.centerGravity === PHYSICS_PRESETS.spacious.centerGravity &&
      params.linkDistance === PHYSICS_PRESETS.spacious.linkDistance &&
      params.repulsion === PHYSICS_PRESETS.spacious.repulsion
    ) {
      updatePresetButtons('spacious');
    } else {
      updatePresetButtons('custom');
    }

    callbacks.onPhysicsChange?.(params);
  }

  // Bind Toolbar Buttons
  container.querySelector('#btn-token-toggle')?.addEventListener('click', toggleTokenPopover);
  container.querySelector('#btn-search-toggle')?.addEventListener('click', toggleSearchPopover);
  container.querySelector('#btn-physics-toggle')?.addEventListener('click', togglePhysicsPopover);
  container.querySelector('#btn-set-hub')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onSetHub();
  });
  container.querySelector('#btn-toggle-colors')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onToggleColorManager();
  });

  // Bind Search Box controls
  searchInput?.addEventListener('input', () => {
    emitSearch();
  });

  btnSearchExact?.addEventListener('click', (e) => {
    e.stopPropagation();
    isExactMatch = !isExactMatch;
    btnSearchExact.classList.toggle('active', isExactMatch);
    btnSearchExact.title = isExactMatch ? 'Exact match (active)' : 'Substring contains (default)';
    emitSearch();
  });

  btnSearchClear?.addEventListener('click', (e) => {
    e.stopPropagation();
    searchInput.value = '';
    emitSearch();
    searchInput.focus();
  });

  container.querySelector('#menu-act-search')?.addEventListener('click', (e) => {
    e.stopPropagation();
    openSearch();
  });

  // Labels mode state and wiring
  let currentLabelsMode: LabelsMode = 'hover';

  function updateLabelsModeUi(mode: LabelsMode) {
    currentLabelsMode = mode;
    const btnHover = container.querySelector('#btn-lbl-hover') as HTMLElement;
    const btnAll = container.querySelector('#btn-lbl-all') as HTMLElement;
    const tbBtn = container.querySelector('#btn-toggle-labels') as HTMLElement;
    const menuTitle = container.querySelector('#menu-title-labels') as HTMLElement;

    if (mode === 'hover') {
      btnHover?.classList.add('active');
      btnAll?.classList.remove('active');
      tbBtn?.classList.remove('active');
      tbBtn?.setAttribute('data-tooltip-title', 'File Names: Hover Only (Clean)');
      tbBtn?.setAttribute('data-tooltip-desc', 'Click to switch to Show All mode and display names for all notes.');
      if (menuTitle) menuTitle.textContent = 'File Names: Hover Only (Clean)';
    } else {
      btnHover?.classList.remove('active');
      btnAll?.classList.add('active');
      tbBtn?.classList.add('active');
      tbBtn?.setAttribute('data-tooltip-title', 'File Names: Show All');
      tbBtn?.setAttribute('data-tooltip-desc', 'Click to switch to Hover Only mode for a clean noise-free graph.');
      if (menuTitle) menuTitle.textContent = 'File Names: Show All';
    }
  }

  container.querySelector('#btn-toggle-labels')?.addEventListener('click', () => {
    closeAllPopovers();
    const nextMode: LabelsMode = currentLabelsMode === 'hover' ? 'all' : 'hover';
    updateLabelsModeUi(nextMode);
    callbacks.onToggleLabelsMode?.(nextMode);
  });

  container.querySelector('#btn-lbl-hover')?.addEventListener('click', () => {
    updateLabelsModeUi('hover');
    callbacks.onToggleLabelsMode?.('hover');
  });

  container.querySelector('#btn-lbl-all')?.addEventListener('click', () => {
    updateLabelsModeUi('all');
    callbacks.onToggleLabelsMode?.('all');
  });

  container.querySelector('#menu-act-toggle-labels')?.addEventListener('click', () => {
    closeAllPopovers();
    const nextMode: LabelsMode = currentLabelsMode === 'hover' ? 'all' : 'hover';
    updateLabelsModeUi(nextMode);
    callbacks.onToggleLabelsMode?.(nextMode);
  });

  container.querySelector('#btn-reset-zoom')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetZoom?.();
  });
  container.querySelector('#btn-reset-layout')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetLayout?.();
  });
  container.querySelector('#btn-actions-menu')?.addEventListener('click', toggleActionsDropdown);

  // Instant hover tooltips for all toolbar buttons
  container.querySelectorAll('.tb-icon-btn').forEach((btn) => {
    btn.addEventListener('mouseenter', () => showTooltip(btn as HTMLElement));
    btn.addEventListener('mouseleave', hideTooltip);
    btn.addEventListener('click', hideTooltip);
  });

  // Close buttons
  container.querySelector('#close-token-popover')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
  });
  container.querySelector('#close-physics-popover')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
  });

  // Slider inputs
  sliderGravity?.addEventListener('input', handleSliderInput);
  sliderDistance?.addEventListener('input', handleSliderInput);
  sliderRepulsion?.addEventListener('input', handleSliderInput);
  sliderCollision?.addEventListener('input', handleSliderInput);

  // Preset buttons
  btnCompact?.addEventListener('click', () => {
    const preset = PHYSICS_PRESETS.compact;
    applyPhysicsToUi(preset);
    updatePresetButtons('compact');
    callbacks.onPhysicsChange?.(preset);
  });

  btnBalanced?.addEventListener('click', () => {
    const preset = PHYSICS_PRESETS.balanced;
    applyPhysicsToUi(preset);
    updatePresetButtons('balanced');
    callbacks.onPhysicsChange?.(preset);
  });

  btnSpacious?.addEventListener('click', () => {
    const preset = PHYSICS_PRESETS.spacious;
    applyPhysicsToUi(preset);
    updatePresetButtons('spacious');
    callbacks.onPhysicsChange?.(preset);
  });

  // Physics action buttons
  container.querySelector('#btn-reheat-sim')?.addEventListener('click', () => {
    callbacks.onReheat?.();
  });

  container.querySelector('#btn-reset-physics')?.addEventListener('click', () => {
    const preset = DEFAULT_PHYSICS_PARAMS;
    applyPhysicsToUi(preset);
    updatePresetButtons('balanced');
    callbacks.onPhysicsChange?.(preset);
  });

  container.querySelector('#btn-reset-disposition')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetLayout?.();
  });

  // Sticky placement and unpin controls
  const chkStickyDrag = container.querySelector('#chk-sticky-drag') as HTMLInputElement;
  const badgePinnedCount = container.querySelector('#badge-pinned-count') as HTMLElement;

  chkStickyDrag?.addEventListener('change', () => {
    callbacks.onStickyToggle?.(chkStickyDrag.checked);
  });

  container.querySelector('#btn-unpin-all')?.addEventListener('click', () => {
    callbacks.onUnpinAll?.();
  });

  container.querySelector('#menu-act-unpin')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onUnpinAll?.();
  });

  // Action Menu Items
  container.querySelector('#menu-act-physics')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
    physicsPopover.style.display = 'block';
    activePopover = 'physics';
  });
  container.querySelector('#menu-act-rescan')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onRescanVault();
  });
  container.querySelector('#menu-act-fix-links')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onFixBrokenLinks();
  });
  container.querySelector('#menu-act-hooks')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onInstallHooks?.();
  });
  container.querySelector('#menu-act-reset-zoom')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetZoom?.();
  });
  container.querySelector('#menu-act-reset-layout')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetLayout?.();
  });

  // Click outside to dismiss
  document.addEventListener('click', (e) => {
    const target = e.target as HTMLElement;
    if (!container.contains(target)) {
      closeAllPopovers();
    }
  });

  // Keyboard shortcuts (Ctrl+F for search, Escape to close/clear)
  window.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      openSearch();
      return;
    }
    if (e.key === 'Escape') {
      if (activePopover === 'search') {
        if (searchInput.value.trim().length > 0) {
          searchInput.value = '';
          emitSearch();
        } else {
          closeAllPopovers();
        }
      } else {
        closeAllPopovers();
      }
    }
  });

  return {
    updateTokenSummary(summary: TokenSummary) {
      const activeFormatted = summary.activeHarnessTokens.toLocaleString();
      const subcallFormatted = summary.subcallAccessibleTokens.toLocaleString();
      const warnFormatted = summary.warningThreshold.toLocaleString();
      const critFormatted = summary.criticalThreshold.toLocaleString();

      let badgeColor = '#10B981';
      let statusText = 'NORMAL';
      if (summary.status === 'warning') {
        badgeColor = '#F59E0B';
        statusText = 'WARNING';
      } else if (summary.status === 'critical') {
        badgeColor = '#EF4444';
        statusText = 'CRITICAL';
      }

      // Update Toolbar Pill
      const pill = container.querySelector('#tb-token-pill') as HTMLElement;
      if (pill) {
        pill.textContent = `${activeFormatted} tok`;
        pill.style.borderColor = badgeColor;
      }

      // Update Popover contents
      const badge = container.querySelector('#popover-token-badge') as HTMLElement;
      if (badge) {
        badge.textContent = statusText;
        badge.style.backgroundColor = badgeColor;
      }

      const activeEl = container.querySelector('#popover-active-tokens');
      if (activeEl) activeEl.textContent = `${activeFormatted} tok`;

      const subcallEl = container.querySelector('#popover-subcall-tokens');
      if (subcallEl) subcallEl.textContent = `${subcallFormatted} tok`;

      const warnEl = container.querySelector('#popover-warn-limit');
      if (warnEl) warnEl.textContent = warnFormatted;

      const critEl = container.querySelector('#popover-crit-limit');
      if (critEl) critEl.textContent = critFormatted;

      const fill = container.querySelector('#popover-token-bar-fill') as HTMLElement;
      if (fill) {
        const pct = Math.min(100, Math.round((summary.activeHarnessTokens / (summary.criticalThreshold || 100000)) * 100));
        fill.style.width = `${pct}%`;
        fill.style.backgroundColor = badgeColor;
      }
    },

    setPhysicsParameters(params: PhysicsParameters) {
      applyPhysicsToUi(params);
    },

    updatePinnedCount(count: number) {
      if (badgePinnedCount) {
        badgePinnedCount.textContent = count.toString();
      }
    },

    isStickyDrag() {
      return chkStickyDrag ? chkStickyDrag.checked : true;
    },

    setLabelsMode(mode: LabelsMode) {
      updateLabelsModeUi(mode);
    },

    setSearchMatchCount(matchingCount: number, totalCount: number) {
      if (!searchInput || !searchInput.value.trim()) {
        searchMatchBadge.style.display = 'none';
        return;
      }
      searchMatchBadge.style.display = 'inline-block';
      if (matchingCount === 0) {
        searchMatchBadge.textContent = '0 matches';
        searchMatchBadge.style.color = '#ef4444';
        searchMatchBadge.style.backgroundColor = 'rgba(239, 68, 68, 0.15)';
      } else {
        searchMatchBadge.textContent = `${matchingCount} / ${totalCount}`;
        searchMatchBadge.style.color = '#38bdf8';
        searchMatchBadge.style.backgroundColor = 'rgba(56, 189, 248, 0.15)';
      }
    },

    openSearch() {
      openSearch();
    },

    closeSearch() {
      closeSearch();
    },
  };
}
