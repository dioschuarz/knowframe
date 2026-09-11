import { TokenSummary } from '../../../src/types/graph';

export interface ActionsToolbarCallbacks {
  onSetHub: () => void;
  onRescanVault: () => void;
  onFixBrokenLinks: () => void;
  onInstallHooks?: () => void;
  onToggleColorManager: () => void;
  onResetZoom?: () => void;
}

export interface ActionsToolbarController {
  updateTokenSummary: (summary: TokenSummary) => void;
}

export function renderActionsToolbar(
  container: HTMLElement,
  callbacks: ActionsToolbarCallbacks,
): ActionsToolbarController {
  let activePopover: 'none' | 'token' | 'actions' = 'none';

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

      <!-- Actions Dropdown Menu Window -->
      <div id="actions-dropdown" class="floating-popover actions-menu-popover" style="display: none;">
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
  const actionsDropdown = container.querySelector('#actions-dropdown') as HTMLElement;

  function closeAllPopovers(): void {
    tokenPopover.style.display = 'none';
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

  // Bind Toolbar Buttons
  container.querySelector('#btn-token-toggle')?.addEventListener('click', toggleTokenPopover);
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

  // Close buttons & popover events
  container.querySelector('#close-token-popover')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
  });

  // Action Menu Items
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
  };
}
