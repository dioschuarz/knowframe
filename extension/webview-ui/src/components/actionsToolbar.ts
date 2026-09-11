import {
  TokenSummary,
  PhysicsParameters,
  DEFAULT_PHYSICS_PARAMS,
  PHYSICS_PRESETS,
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
}

export interface ActionsToolbarController {
  updateTokenSummary: (summary: TokenSummary) => void;
  setPhysicsParameters: (params: PhysicsParameters) => void;
  updatePinnedCount: (count: number) => void;
  isStickyDrag: () => boolean;
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
        <button id="btn-token-toggle" class="tb-icon-btn tb-token-btn" title="Token Economics & Context Budget">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M7 1L1 9h5l-1 6 7-8h-5l1-6z"/>
          </svg>
          <span id="tb-token-pill" class="tb-pill">0 tok</span>
        </button>

        <div class="tb-divider"></div>

        <!-- Gravity & Spacing Settings Icon Button -->
        <button id="btn-physics-toggle" class="tb-icon-btn" title="Graph Gravity & Spacing Settings">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <line x1="2" y1="4" x2="14" y2="4"/>
            <line x1="2" y1="8" x2="14" y2="8"/>
            <line x1="2" y1="12" x2="14" y2="12"/>
            <circle cx="5" cy="4" r="1.6" fill="currentColor"/>
            <circle cx="11" cy="8" r="1.6" fill="currentColor"/>
            <circle cx="7" cy="12" r="1.6" fill="currentColor"/>
          </svg>
        </button>

        <!-- Set Hub Icon Button -->
        <button id="btn-set-hub" class="tb-icon-btn" title="Set Selected Note as Central Hub">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <circle cx="8" cy="8" r="6"/>
            <circle cx="8" cy="8" r="2" fill="currentColor"/>
            <line x1="8" y1="1" x2="8" y2="3"/>
            <line x1="8" y1="13" x2="8" y2="15"/>
            <line x1="1" y1="8" x2="3" y2="8"/>
            <line x1="13" y1="8" x2="15" y2="8"/>
          </svg>
        </button>

        <!-- Clusters Icon Button -->
        <button id="btn-toggle-colors" class="tb-icon-btn" title="Manage Chromatic Cluster Rules">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M8 0a8 8 0 1 0 8 8 8 8 0 0 0-8-8zm-4.5 9.5a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm2-4a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm5 0a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5zm2 4a1.5 1.5 0 1 1 1.5-1.5 1.5 1.5 0 0 1-1.5 1.5z"/>
          </svg>
        </button>

        <!-- Reset Zoom Icon Button -->
        <button id="btn-reset-zoom" class="tb-icon-btn" title="Reset Zoom & Center View">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M1 5V1h4M15 5V1h-4M1 11v4h4M15 11v4h-4"/>
          </svg>
        </button>

        <div class="tb-divider"></div>

        <!-- Actions Menu Dropdown Button -->
        <button id="btn-actions-menu" class="tb-icon-btn" title="Vault Actions & Tools">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <circle cx="8" cy="3" r="1.5"/>
            <circle cx="8" cy="8" r="1.5"/>
            <circle cx="8" cy="13" r="1.5"/>
          </svg>
        </button>
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
            <button id="btn-reset-physics" class="physics-action-btn" title="Reset all sliders to defaults">↺ Reset Defaults</button>
          </div>
        </div>
      </div>

      <!-- Actions Dropdown Menu Window -->
      <div id="actions-dropdown" class="floating-popover actions-menu-popover" style="display: none;">
        <div class="menu-item" id="menu-act-physics">
          <span class="m-icon">🎛️</span>
          <div class="m-text">
            <div class="m-title">Gravity & Spacing</div>
            <div class="m-desc">Tune node repulsion, link distance & center pull</div>
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
      </div>
    </div>
  `;

  const tokenPopover = container.querySelector('#token-popover') as HTMLElement;
  const physicsPopover = container.querySelector('#physics-popover') as HTMLElement;
  const actionsDropdown = container.querySelector('#actions-dropdown') as HTMLElement;

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

  function closeAllPopovers(): void {
    tokenPopover.style.display = 'none';
    physicsPopover.style.display = 'none';
    actionsDropdown.style.display = 'none';
    activePopover = 'none';
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
  container.querySelector('#btn-physics-toggle')?.addEventListener('click', togglePhysicsPopover);
  container.querySelector('#btn-set-hub')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onSetHub();
  });
  container.querySelector('#btn-toggle-colors')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onToggleColorManager();
  });
  container.querySelector('#btn-reset-zoom')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetZoom?.();
  });
  container.querySelector('#btn-actions-menu')?.addEventListener('click', toggleActionsDropdown);

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

  // Click outside to dismiss
  document.addEventListener('click', (e) => {
    const target = e.target as HTMLElement;
    if (!container.contains(target)) {
      closeAllPopovers();
    }
  });

  // Escape key to dismiss
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      closeAllPopovers();
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
  };
}
