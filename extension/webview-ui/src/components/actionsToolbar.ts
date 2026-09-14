import {
  DEFAULT_PHYSICS_PARAMS,
  PHYSICS_PRESETS,
} from '../../../src/types/graph.ts';
import type {
  TokenSummary,
  PhysicsParameters,
  LabelsMode,
  FocusMode,
} from '../../../src/types/graph.ts';
import type { SearchMatchOptions, SearchMode } from '../graph/searchMatcher.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';

export interface ActionsToolbarCallbacks {
  onSetHub?: () => void;
  onRescanVault?: () => void;
  onFixBrokenLinks?: () => void;
  onInstallHooks?: () => void;
  onToggleColorManager?: () => void;
  onResetZoom?: () => void;
  onPhysicsChange?: (params: PhysicsParameters) => void;
  onReheat?: () => void;
  onUnpinAll?: () => void;
  onStickyToggle?: (enabled: boolean) => void;
  onToggleLabelsMode?: (mode: LabelsMode) => void;
  onSearchChange?: (query: string, options: SearchMatchOptions) => void;
  onFocusModeChange?: (mode: FocusMode) => void;
  onToggleOrphans?: (show: boolean) => void;
  onToggleExcluded?: (show: boolean) => void;
  onOpenSettings?: (tab?: string) => void;
  onOpenTokenBreakdown?: () => void;
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
  getSearchOptions: () => SearchMatchOptions;
  setFocusMode?: (mode: FocusMode) => void;
  setShowOrphans?: (show: boolean) => void;
  setShowExcluded?: (show: boolean) => void;
}

export function renderActionsToolbar(
  container: HTMLElement,
  callbacks: ActionsToolbarCallbacks,
): ActionsToolbarController {
  let activePopover: 'none' | 'token' | 'physics' | 'actions' | 'focus' | 'search' = 'none';

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

        <!-- Focus Mode Dropdown -->
        <div class="tb-dropdown-container" id="focus-dropdown-container">
          <button id="btn-focus-toggle" class="tb-icon-btn tb-focus-btn" aria-label="Focus Mode" data-tooltip-title="Focus Mode" data-tooltip-desc="Restrict graph view to 1 hop, 2 hops, 3 hops, or All">
            <svg class="tb-svg focus-svg" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="8" cy="8" r="6"/>
              <circle cx="8" cy="8" r="2.5"/>
              <line x1="8" y1="1" x2="8" y2="3"/>
              <line x1="8" y1="13" x2="8" y2="15"/>
              <line x1="1" y1="8" x2="3" y2="8"/>
              <line x1="13" y1="8" x2="15" y2="8"/>
            </svg>
            <span id="tb-focus-label" class="tb-focus-label">All Notes</span>
            <span class="tb-dropdown-arrow">▾</span>
          </button>
          <div id="focus-dropdown-menu" class="floating-popover focus-menu-popover" style="display: none;">
            <div class="menu-item focus-item" data-focus="all">
              <span class="m-icon">
                <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="8" cy="8" r="6.5"/>
                  <ellipse cx="8" cy="8" rx="2.8" ry="6.5"/>
                  <line x1="1.5" y1="8" x2="14.5" y2="8"/>
                </svg>
              </span>
              <div class="m-text">
                <div class="m-title">All Notes</div>
                <div class="m-desc">Complete knowledge graph</div>
              </div>
              <span class="focus-check" id="check-focus-all">✓</span>
            </div>
            <div class="menu-item focus-item" data-focus="3_hops">
              <span class="m-icon">
                <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="8" cy="8" r="6.5"/>
                  <circle cx="8" cy="8" r="4.2"/>
                  <circle cx="8" cy="8" r="2" fill="currentColor"/>
                </svg>
              </span>
              <div class="m-text">
                <div class="m-title">3 Hops</div>
                <div class="m-desc">Extended community neighborhood</div>
              </div>
              <span class="focus-check" id="check-focus-3_hops" style="display: none;">✓</span>
            </div>
            <div class="menu-item focus-item" data-focus="2_hops">
              <span class="m-icon">
                <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="8" cy="8" r="5.5"/>
                  <circle cx="8" cy="8" r="2" fill="currentColor"/>
                </svg>
              </span>
              <div class="m-text">
                <div class="m-title">2 Hops</div>
                <div class="m-desc">Secondary context around Central Hub</div>
              </div>
              <span class="focus-check" id="check-focus-2_hops" style="display: none;">✓</span>
            </div>
            <div class="menu-item focus-item" data-focus="1_hop">
              <span class="m-icon">
                <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="4.5" cy="8" r="2.5" fill="currentColor"/>
                  <circle cx="11.5" cy="8" r="2"/>
                  <line x1="7" y1="8" x2="9.5" y2="8"/>
                </svg>
              </span>
              <div class="m-text">
                <div class="m-title">1 Hop</div>
                <div class="m-desc">Direct links to and from Central Hub</div>
              </div>
              <span class="focus-check" id="check-focus-1_hop" style="display: none;">✓</span>
            </div>
            <div class="menu-divider"></div>
            <label class="focus-checkbox-item">
              <input type="checkbox" id="chk-focus-orphans" checked>
              <span>Show orphans</span>
            </label>
            <label class="focus-checkbox-item">
              <input type="checkbox" id="chk-focus-excluded">
              <span>Show excluded (ghosted)</span>
            </label>
          </div>
        </div>

        <!-- Reset Disposition Button -->
        <button id="btn-reset-disposition-tb" class="tb-icon-btn" aria-label="Reset Disposition" data-tooltip-title="Reset Disposition" data-tooltip-desc="Unpin all manually moved notes and return graph to simulation layout">
          <svg class="tb-svg" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4.146.146A.5.5 0 0 1 4.5 0h7a.5.5 0 0 1 .5.5c0 .68-.342 1.174-.646 1.479-.283.282-.63.535-1.01.777C10.288 3.738 10 5.28 10 6.5c0 .35.034.65.088.91l2.42 2.42a.5.5 0 0 1-.354.854H8.5v4.5a.5.5 0 0 1-1 0V10.684H3.846a.5.5 0 0 1-.354-.854l2.42-2.42C5.966 7.15 6 6.85 6 6.5c0-1.22-.288-2.762-.344-3.744-.38-.242-.727-.495-1.01-.777C4.342 1.674 4 1.18 4 .5a.5.5 0 0 1 .146-.354z"/>
          </svg>
          <span id="tb-pinned-badge" class="tb-pin-badge" style="display: none;">0</span>
        </button>

        <div class="tb-divider"></div>

        <!-- Actions Menu Dropdown Button -->
        <button id="btn-actions-menu" class="tb-icon-btn" aria-label="Vault Actions" data-tooltip-title="Vault Actions & Tools" data-tooltip-desc="Rescan vault, fix broken links, install git hooks, and reset disposition">
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
          <button id="btn-search-clear" class="search-clear-btn" title="Clear search (Esc)" style="display: none;">✕</button>
        </div>
        <div class="search-options-row">
          <div class="search-mode-group">
            <button id="btn-mode-contains" class="search-mode-btn active" title="Contains: Match contiguous substring anywhere in note (Default)">Contains</button>
            <button id="btn-mode-keywords" class="search-mode-btn" title="Keywords: Match all separate words in any order (e.g. 'vs extension' matches 'vs code extension')">Keywords</button>
            <button id="btn-mode-exact" class="search-mode-btn" title="Exact: Match exact note title, filename stem, or folder name">Exact</button>
          </div>
          <button id="btn-search-case" class="search-option-btn" title="Match Case (Case Sensitive)">Aa</button>
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
            <span class="p-lbl">Reachable Context (L1-L2):</span>
            <strong id="popover-subcall-tokens" class="p-val">0</strong>
          </div>
          <div class="popover-metric-row">
            <span class="p-lbl">Context Reduction:</span>
            <strong id="popover-context-reduction" class="p-val" title="Context reduction achieved via section-level routing (~90% avg)">~90%</strong>
          </div>
          <div class="popover-bar-bg">
            <div id="popover-token-bar-fill" class="popover-bar-fill"></div>
          </div>
          <div class="popover-limits-row">
            <span>Warning: <strong id="popover-warn-limit">60,000</strong></span>
            <span>Critical: <strong id="popover-crit-limit">100,000</strong></span>
          </div>
          <button type="button" class="btn-primary" id="btn-token-popover-breakdown" style="width: 100%; margin-top: 10px; font-size: 11px;">View Detailed Breakdown</button>
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
            <button id="btn-unpin-all" class="physics-action-btn" title="Reset disposition by releasing all pinned notes">
              <svg viewBox="0 0 16 16" width="12" height="12" fill="currentColor" style="display: inline-block; vertical-align: middle; margin-right: 4px;">
                <path d="M4.146.146A.5.5 0 0 1 4.5 0h7a.5.5 0 0 1 .5.5c0 .68-.342 1.174-.646 1.479-.283.282-.63.535-1.01.777C10.288 3.738 10 5.28 10 6.5c0 .35.034.65.088.91l2.42 2.42a.5.5 0 0 1-.354.854H8.5v4.5a.5.5 0 0 1-1 0V10.684H3.846a.5.5 0 0 1-.354-.854l2.42-2.42C5.966 7.15 6 6.85 6 6.5c0-1.22-.288-2.762-.344-3.744-.38-.242-.727-.495-1.01-.777C4.342 1.674 4 1.18 4 .5a.5.5 0 0 1 .146-.354z"/>
              </svg>
              Reset Disposition (<span id="badge-pinned-count">0</span>)
            </button>
          </div>

          <div class="physics-footer">
            <button id="btn-reheat-sim" class="physics-action-btn" title="Re-simulate node positions">
              <svg viewBox="0 0 16 16" width="12" height="12" fill="currentColor" style="display: inline-block; vertical-align: middle; margin-right: 4px;">
                <path d="M7 1L1 9h5l-1 6 7-8h-5l1-6z"/>
              </svg>
              Reheat
            </button>
            <button id="btn-reset-physics" class="physics-action-btn" title="Reset all sliders to defaults">
              <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display: inline-block; vertical-align: middle; margin-right: 4px;">
                <path d="M2 8a6 6 0 1 1 1.76 4.24L1 15M1 10v5h5"/>
              </svg>
              Reset Defaults
            </button>
          </div>
        </div>
      </div>

      <!-- Actions Dropdown Menu Window -->
      <div id="actions-dropdown" class="floating-popover actions-menu-popover" style="display: none;">
        <div class="menu-item" id="menu-act-physics">
          <span class="m-icon">
            <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <line x1="2" y1="4" x2="14" y2="4"/>
              <line x1="2" y1="12" x2="14" y2="12"/>
              <circle cx="5" cy="4" r="2" fill="var(--vscode-editorWidget-background, #252526)"/>
              <circle cx="11" cy="12" r="2" fill="var(--vscode-editorWidget-background, #252526)"/>
            </svg>
          </span>
          <div class="m-text">
            <div class="m-title">Gravity & Spacing</div>
            <div class="m-desc">Tune node repulsion, link distance & center pull</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-toggle-labels">
          <span class="m-icon">
            <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M1.5 8.5L8.5 1.5H14.5V7.5L7.5 14.5L1.5 8.5Z"/>
              <circle cx="11.5" cy="4.5" r="1" fill="currentColor"/>
            </svg>
          </span>
          <div class="m-text">
            <div class="m-title" id="menu-title-labels">File Names: Hover Only</div>
            <div class="m-desc">Toggle between clean hover-only and showing all names</div>
          </div>
        </div>
        <div class="menu-item" id="menu-act-reset-zoom">
          <span class="m-icon">
            <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M2 5V2H5M11 2H14V5M14 11V14H11M5 14H2V11"/>
              <circle cx="8" cy="8" r="1.5" fill="currentColor"/>
            </svg>
          </span>
          <div class="m-text">
            <div class="m-title">Reset Canvas View</div>
            <div class="m-desc">Center graph and reset zoom level</div>
          </div>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" id="menu-act-settings">
          <span class="m-icon">
            <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="8" cy="8" r="2.5"/>
              <path d="M8 1v2m0 10v2M1 8h2m10 0h2m-2.05-4.95l-1.41 1.41m-7.08 7.08l-1.41 1.41m0-9.9l1.41 1.41m7.08 7.08l1.41 1.41"/>
            </svg>
          </span>
          <div class="m-text">
            <div class="m-title">Advanced Settings...</div>
            <div class="m-desc">Hub rules, exclusions, cluster colors, git hooks, diagnostics</div>
          </div>
        </div>
      </div>
    </div>
  `;

  const tokenPopover = container.querySelector('#token-popover') as HTMLElement;
  const physicsPopover = container.querySelector('#physics-popover') as HTMLElement;
  const actionsDropdown = container.querySelector('#actions-dropdown') as HTMLElement;
  const searchPopover = container.querySelector('#search-popover') as HTMLElement;
  const focusDropdownMenu = container.querySelector('#focus-dropdown-menu') as HTMLElement;
  const btnFocusToggle = container.querySelector('#btn-focus-toggle') as HTMLElement;
  const tbFocusLabel = container.querySelector('#tb-focus-label') as HTMLElement;

  const searchInput = container.querySelector('#tb-search-input') as HTMLInputElement;
  const searchMatchBadge = container.querySelector('#tb-search-count') as HTMLElement;
  const btnSearchToggle = container.querySelector('#btn-search-toggle') as HTMLElement;
  const btnModeContains = container.querySelector('#btn-mode-contains') as HTMLButtonElement;
  const btnModeKeywords = container.querySelector('#btn-mode-keywords') as HTMLButtonElement;
  const btnModeExact = container.querySelector('#btn-mode-exact') as HTMLButtonElement;
  const btnSearchCase = container.querySelector('#btn-search-case') as HTMLButtonElement;
  const btnSearchClear = container.querySelector('#btn-search-clear') as HTMLButtonElement;

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
    if (focusDropdownMenu) focusDropdownMenu.style.display = 'none';
    activePopover = 'none';
    btnSearchToggle?.classList.toggle('active', !!searchInput?.value.trim());
    hideTooltip();
  }

  let currentSearchMode: SearchMode = 'contains';
  let isCaseSensitive = false;

  function emitSearch(): void {
    const query = searchInput.value;
    callbacks.onSearchChange?.(query, {
      mode: currentSearchMode,
      caseSensitive: isCaseSensitive,
    });
    if (query.trim().length > 0) {
      btnSearchClear.style.display = 'inline-block';
      btnSearchToggle.classList.add('active');
    } else {
      btnSearchClear.style.display = 'none';
      searchMatchBadge.style.display = 'none';
      btnSearchToggle.classList.remove('active');
    }
  }

  function updateSearchModeUi(mode: SearchMode): void {
    currentSearchMode = mode;
    btnModeContains?.classList.toggle('active', mode === 'contains');
    btnModeKeywords?.classList.toggle('active', mode === 'keywords');
    btnModeExact?.classList.toggle('active', mode === 'exact');
    emitSearch();
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
  container.querySelector('#btn-token-toggle')?.addEventListener('click', () => {
    if (callbacks.onOpenTokenBreakdown) {
      callbacks.onOpenTokenBreakdown();
    } else {
      toggleTokenPopover();
    }
  });
  container.querySelector('#btn-search-toggle')?.addEventListener('click', toggleSearchPopover);

  // Focus Mode State & Wiring
  let currentFocusMode: FocusMode = 'all';

  function updateFocusUi(mode: FocusMode): void {
    currentFocusMode = mode;
    const labelMap: Record<FocusMode, string> = {
      all: 'All Notes',
      '3_hops': '3 Hops',
      '2_hops': '2 Hops',
      '1_hop': '1 Hop',
      hub: '1 Hop',
    };
    if (tbFocusLabel) {
      tbFocusLabel.textContent = labelMap[mode] || 'All Notes';
    }
    const checkAll = container.querySelector('#check-focus-all') as HTMLElement;
    const check3Hops = container.querySelector('#check-focus-3_hops') as HTMLElement;
    const check2Hops = container.querySelector('#check-focus-2_hops') as HTMLElement;
    const check1Hop = container.querySelector('#check-focus-1_hop') as HTMLElement;

    if (checkAll) checkAll.style.display = mode === 'all' ? 'inline' : 'none';
    if (check3Hops) check3Hops.style.display = mode === '3_hops' ? 'inline' : 'none';
    if (check2Hops) check2Hops.style.display = mode === '2_hops' ? 'inline' : 'none';
    if (check1Hop) check1Hop.style.display = (mode === '1_hop' || mode === 'hub') ? 'inline' : 'none';
  }

  function toggleFocusDropdown(e?: MouseEvent): void {
    e?.stopPropagation();
    if (activePopover === 'focus') {
      closeAllPopovers();
    } else {
      closeAllPopovers();
      if (focusDropdownMenu) {
        focusDropdownMenu.style.display = 'block';
        activePopover = 'focus';
      }
    }
  }

  btnFocusToggle?.addEventListener('click', toggleFocusDropdown);

  container.querySelectorAll('.focus-item').forEach((item) => {
    item.addEventListener('click', (e) => {
      e.stopPropagation();
      const mode = item.getAttribute('data-focus') as FocusMode;
      if (mode) {
        updateFocusUi(mode);
        closeAllPopovers();
        callbacks.onFocusModeChange?.(mode);
      }
    });
  });

  const chkFocusOrphans = container.querySelector('#chk-focus-orphans') as HTMLInputElement;
  chkFocusOrphans?.addEventListener('change', (e) => {
    e.stopPropagation();
    callbacks.onToggleOrphans?.(chkFocusOrphans.checked);
  });

  const chkFocusExcluded = container.querySelector('#chk-focus-excluded') as HTMLInputElement;
  chkFocusExcluded?.addEventListener('change', (e) => {
    e.stopPropagation();
    callbacks.onToggleExcluded?.(chkFocusExcluded.checked);
  });

  // Bind Search Box controls
  searchInput?.addEventListener('input', () => {
    emitSearch();
  });

  btnModeContains?.addEventListener('click', (e) => {
    e.stopPropagation();
    updateSearchModeUi('contains');
    searchInput.focus();
  });

  btnModeKeywords?.addEventListener('click', (e) => {
    e.stopPropagation();
    updateSearchModeUi('keywords');
    searchInput.focus();
  });

  btnModeExact?.addEventListener('click', (e) => {
    e.stopPropagation();
    updateSearchModeUi('exact');
    searchInput.focus();
  });

  btnSearchCase?.addEventListener('click', (e) => {
    e.stopPropagation();
    isCaseSensitive = !isCaseSensitive;
    btnSearchCase.classList.toggle('active', isCaseSensitive);
    btnSearchCase.title = isCaseSensitive ? 'Match Case (Case Sensitive: Active)' : 'Match Case (Case Sensitive)';
    emitSearch();
    searchInput.focus();
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
  container.querySelector('#btn-reset-disposition-tb')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onUnpinAll?.();
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
  container.querySelector('#btn-token-popover-breakdown')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
    callbacks.onOpenTokenBreakdown?.();
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

  // Action Menu Items
  container.querySelector('#menu-act-physics')?.addEventListener('click', (e) => {
    e.stopPropagation();
    closeAllPopovers();
    physicsPopover.style.display = 'block';
    activePopover = 'physics';
  });
  container.querySelector('#menu-act-reset-zoom')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onResetZoom?.();
  });
  container.querySelector('#menu-act-settings')?.addEventListener('click', () => {
    closeAllPopovers();
    callbacks.onOpenSettings?.();
  });

  // Click outside to dismiss
  if (typeof document !== 'undefined') {
    document.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      if (!container.contains(target)) {
        closeAllPopovers();
      }
    });
  }

  // Keyboard shortcuts (Ctrl+F for search, Escape to close/clear)
  if (typeof window !== 'undefined') {
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
}

  return {
    updateTokenSummary(summary: TokenSummary) {
      const activeFormatted = formatCompactNumber(summary.activeHarnessTokens);
      const subcallFormatted = formatCompactNumber(summary.subcallAccessibleTokens);
      const warnFormatted = formatCompactNumber(summary.warningThreshold);
      const critFormatted = formatCompactNumber(summary.criticalThreshold);

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
        pill.style.color = badgeColor;
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
      if (warnEl) warnEl.textContent = `${warnFormatted} tok`;

      const critEl = container.querySelector('#popover-crit-limit');
      if (critEl) critEl.textContent = `${critFormatted} tok`;

      const reductionEl = container.querySelector('#popover-context-reduction');
      if (reductionEl) {
        if (typeof summary.contextReductionPercent === 'number') {
          reductionEl.textContent = `${summary.contextReductionPercent.toFixed(1)}%`;
        } else {
          reductionEl.textContent = '~90%';
        }
      }

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
      const compactCount = formatCompactNumber(count);
      if (badgePinnedCount) {
        badgePinnedCount.textContent = compactCount;
      }
      const tbBadge = container.querySelector('#tb-pinned-badge') as HTMLElement;
      const tbBtn = container.querySelector('#btn-reset-disposition-tb') as HTMLElement;
      if (tbBadge) {
        if (count > 0) {
          tbBadge.textContent = compactCount;
          tbBadge.style.display = 'flex';
          tbBtn?.classList.add('has-pinned');
        } else {
          tbBadge.style.display = 'none';
          tbBtn?.classList.remove('has-pinned');
        }
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
        searchMatchBadge.textContent = `${formatCompactNumber(matchingCount)} / ${formatCompactNumber(totalCount)}`;
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

    getSearchOptions() {
      return { mode: currentSearchMode, caseSensitive: isCaseSensitive };
    },

    setFocusMode(mode: FocusMode) {
      updateFocusUi(mode);
    },

    setShowOrphans(show: boolean) {
      if (chkFocusOrphans) {
        chkFocusOrphans.checked = show;
      }
    },

    setShowExcluded(show: boolean) {
      if (chkFocusExcluded) {
        chkFocusExcluded.checked = show;
      }
    },
  };
}
