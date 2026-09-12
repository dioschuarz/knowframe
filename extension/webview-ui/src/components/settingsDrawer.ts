/**
 * Settings Drawer Component
 *
 * Implements Constitution Principle VI (Progressive Disclosure).
 * Houses administrative tasks in a slide-over drawer accessed via toolbar:
 * - Knowledge (Hub configuration, broken links, vault rescan)
 * - Excluded Paths (Two-tier rule manager, test preview, add/delete rules)
 * - Cluster Groups (Color clustering rules by path or tag)
 * - Git Sync (prek git hooks status, latency, reinstall)
 * - Diagnostics (Node counts, protocol metrics, cache stats)
 */

import type { ClusterColorGroup, NoteNode } from '../../../src/types/graph.ts';
import {
  discoverVaultSuggestions,
  countMatchingNotes,
  DEFAULT_PALETTE,
} from './colorManager.ts';

export type SettingsTab = 'knowledge' | 'exclusions' | 'clusters' | 'git_sync' | 'diagnostics';

export interface SettingsDrawerProps {
  activeTab: SettingsTab;
  primaryHub?: string | null;
  brokenLinksCount?: number;
  scanExclusions: string[];
  graphExclusions: string[];
  colorGroups: ClusterColorGroup[];
  nodes?: NoteNode[];
  gitHooksInstalled?: boolean;
  gitCommitLatencyMs?: number;
  totalNotes?: number;
  connectedNotes?: number;
  orphanNotes?: number;
  excludedNotes?: number;
}

export interface SettingsDrawerCallbacks {
  onChangeTab?: (tab: SettingsTab) => void;
  onChangeHub?: () => void;
  onFixBrokenLinks?: () => void;
  onRescanVault?: () => void;
  onAddExclusion?: (pattern: string, scope: 'scan' | 'graph') => void;
  onDeleteExclusion?: (pattern: string, scope: 'scan' | 'graph') => void;
  onTestPreview?: (pattern: string) => void;
  onSaveColorGroups?: (groups: ClusterColorGroup[]) => void;
  onInstallGitHooks?: () => void;
  onClose?: () => void;
}

export interface SettingsDrawerController {
  open: (tab?: SettingsTab) => void;
  close: () => void;
  isOpen: () => boolean;
  updateProps: (newProps: Partial<SettingsDrawerProps>) => void;
}

export function renderSettingsDrawerHtml(props: SettingsDrawerProps): string {
  const {
    activeTab,
    primaryHub = '00_meta/wiki_index.md',
    brokenLinksCount = 0,
    scanExclusions = [],
    graphExclusions = [],
    colorGroups = [],
    nodes = [],
    gitHooksInstalled = true,
    gitCommitLatencyMs = 15,
    totalNotes = 0,
    connectedNotes = 0,
    orphanNotes = 0,
    excludedNotes = 0,
  } = props;

  let tabContentHtml = '';

  switch (activeTab) {
    case 'knowledge':
      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Central Hub Note</div>
          <p class="settings-desc">The designated anchor note from which BFS hop horizons and context decay tiers are calculated.</p>
          <div class="settings-hub-card">
            <span class="settings-hub-icon">
              <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="8" cy="8" r="6"/>
                <circle cx="8" cy="8" r="2.5" fill="#38bdf8"/>
              </svg>
            </span>
            <span class="settings-hub-path" title="${primaryHub}">${primaryHub}</span>
            <button class="btn-secondary settings-hub-btn" id="btn-settings-change-hub">Change Hub</button>
          </div>
        </div>

        <div class="settings-section">
          <div class="settings-sec-title">Link Integrity</div>
          <p class="settings-desc">Broken links pointing to nonexistent notes or invalid section anchors.</p>
          <div class="settings-health-row">
            <span class="settings-health-status ${brokenLinksCount === 0 ? 'status-clean' : 'status-warn'}">
              ${brokenLinksCount === 0
                ? `<svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="#10b981" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;"><path d="M3 8.5L6.5 12L13 4"/></svg>No broken links detected`
                : `<svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="#ef4444" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;"><path d="M8 1.5L14.5 13.5H1.5L8 1.5Z"/><line x1="8" y1="6" x2="8" y2="9.5"/><circle cx="8" cy="11.5" r="0.75" fill="#ef4444"/></svg>${brokenLinksCount} broken references`}
            </span>
            ${brokenLinksCount > 0 ? `<button class="btn-primary" id="btn-settings-fix-broken">Fix References</button>` : ''}
          </div>
        </div>

        <div class="settings-section">
          <div class="settings-sec-title">Vault Re-indexing</div>
          <p class="settings-desc">Trigger a full AST parse and graph topology rebuild from workspace files.</p>
          <button class="btn-secondary" id="btn-settings-rescan">
            <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;">
              <path d="M2.5 8a5.5 5.5 0 1 1 1.6 3.9L1.5 14M1.5 9.5v4.5h4.5"/>
            </svg>
            Rescan Vault Now
          </button>
        </div>
      `;
      break;

    case 'exclusions':
      const graphRulesHtml = graphExclusions.length > 0
        ? graphExclusions.map((rule) => `
            <div class="settings-rule-item">
              <span class="rule-pattern" title="${rule}">
                <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px; opacity:0.75;">
                  <circle cx="8" cy="8" r="6"/>
                  <line x1="3.5" y1="3.5" x2="12.5" y2="12.5"/>
                </svg>
                ${rule}
              </span>
              <span class="rule-tier-tag">graph</span>
              <button class="rule-del-btn" data-rule="${rule}" data-scope="graph" title="Remove rule">✕</button>
            </div>
          `).join('')
        : '<div class="settings-empty-msg">No graph exclusion rules configured</div>';

      const scanRulesHtml = scanExclusions.length > 0
        ? scanExclusions.map((rule) => `
            <div class="settings-rule-item">
              <span class="rule-pattern" title="${rule}">
                <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px; opacity:0.75;">
                  <circle cx="8" cy="8" r="6"/>
                  <line x1="4" y1="8" x2="12" y2="8"/>
                </svg>
                ${rule}
              </span>
              <span class="rule-tier-tag">scan</span>
              <button class="rule-del-btn" data-rule="${rule}" data-scope="scan" title="Remove rule">✕</button>
            </div>
          `).join('')
        : '<div class="settings-empty-msg">No scan exclusion rules configured</div>';

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Excluded Paths Manager</div>
          <p class="settings-desc">
            <strong>Graph Exclude:</strong> Preserves link resolution but hides from graph canvas and orphan counter.<br/>
            <strong>Scan Exclude:</strong> Completely skips parsing (e.g. node_modules, .git).
          </p>

          <div class="settings-add-rule-box">
            <input type="text" id="input-new-rule" class="settings-input" placeholder="e.g. packages/**/README.md or vendor/**" />
            <div class="settings-scope-dropdown" id="scope-dropdown">
              <button type="button" class="settings-scope-btn" id="btn-scope-toggle" aria-haspopup="listbox">
                <span id="label-scope-selected">Graph Tier</span>
                <span class="scope-dropdown-arrow">▾</span>
              </button>
              <div class="settings-scope-menu" id="menu-scope-options" style="display: none;">
                <div class="scope-menu-item active" data-value="graph">
                  <div class="scope-item-title">Graph Tier</div>
                  <div class="scope-item-desc">Hides from graph canvas, preserves link resolution</div>
                </div>
                <div class="scope-menu-item" data-value="scan">
                  <div class="scope-item-title">Scan Tier</div>
                  <div class="scope-item-desc">Completely skips parsing during vault index</div>
                </div>
              </div>
              <input type="hidden" id="select-rule-scope" value="graph" />
            </div>
            <button class="btn-primary" id="btn-add-rule">+ Add Rule</button>
          </div>

          <div class="settings-subheading">Graph Exclusions (Visual & Context Hiding)</div>
          <div class="settings-rules-list">${graphRulesHtml}</div>

          <div class="settings-subheading" style="margin-top: 14px;">Scan Exclusions (Completely Ignored)</div>
          <div class="settings-rules-list">${scanRulesHtml}</div>
        </div>
      `;
      break;

    case 'clusters': {
      const allSuggestions = discoverVaultSuggestions(nodes);
      const existingQueries = new Set(colorGroups.map((g) => g.query.trim().toLowerCase()));
      const unaddedSuggestions = allSuggestions.filter(
        (s) => !existingQueries.has(s.query.trim().toLowerCase()),
      );
      const nextColor = DEFAULT_PALETTE[colorGroups.length % DEFAULT_PALETTE.length];
      const activeCount = colorGroups.filter((g) => g.enabled).length;

      const colorRulesHtml = colorGroups.length > 0
        ? colorGroups.map((g, idx) => {
            const matchCount = countMatchingNotes(g.query, nodes);
            const label = g.label || (g as any).name || g.query;
            return `
              <div class="settings-rule-item cluster-rule-row" data-idx="${idx}">
                <input type="checkbox" class="chk-cluster-enabled" data-idx="${idx}" ${g.enabled ? 'checked' : ''} title="Toggle rule active" />
                <input type="color" class="inp-cluster-color" data-idx="${idx}" value="${g.color}" title="Change cluster color" />
                <span class="rule-name" title="${label}">${label}</span>
                <span class="rule-pattern" title="${g.query}">${g.query}</span>
                <span class="rule-match-count" title="${matchCount} notes currently match this rule">${matchCount} note${matchCount === 1 ? '' : 's'}</span>
                <button class="rule-del-btn btn-delete-cluster" data-idx="${idx}" title="Delete rule">✕</button>
              </div>
            `;
          }).join('')
        : '<div class="settings-empty-msg">No custom cluster color groups defined. Add a rule above or pick a suggestion below.</div>';

      const suggestionsChipsHtml = unaddedSuggestions.length > 0
        ? `
            <div class="suggestions-list" style="max-height: 140px; overflow-y: auto;">
              ${unaddedSuggestions.map((s) => `
                <button type="button" class="suggestion-chip btn-add-suggestion" data-query="${s.query}" data-label="${s.label}" data-color="${s.color}" title="Add ${s.query} (${s.count} notes)">
                  <span class="chip-dot" style="background-color: ${s.color};"></span>
                  <span class="chip-label">${s.label}</span>
                  <span class="chip-count">${s.count}</span>
                  <span class="chip-plus">+</span>
                </button>
              `).join('')}
            </div>
          `
        : '<div class="settings-empty-msg" style="font-size: 11px;">All detected vault folders and tags are currently configured.</div>';

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Cluster Color Groups</div>
          <p class="settings-desc">
            Categorize and color graph nodes by tag (<code>tag:#tag</code>) or folder path (<code>path:folder/**</code>).
            Active rules tint nodes on canvas; opacity decays with hop distance from the hub.
          </p>

          <div class="settings-subheading">Add Color Group</div>
          <div class="settings-add-cluster-box">
            <div class="cluster-color-picker-wrap" title="Select cluster color">
              <input type="color" id="input-cluster-color" class="cluster-color-input" value="${nextColor}" />
            </div>
            <input type="text" id="input-cluster-label" class="settings-input cluster-input-label" placeholder="Label (e.g. Memory)" />
            <input type="text" id="input-cluster-query" class="settings-input cluster-input-query" placeholder="Query: path:folder/** or tag:#tag" />
            <button class="btn-primary" id="btn-add-cluster-group">+ Add</button>
          </div>

          <div class="settings-subheading" style="margin-top: 16px; display: flex; justify-content: space-between; align-items: center;">
            <span>Configured Groups (${activeCount} of ${colorGroups.length} active)</span>
            ${colorGroups.length > 0 ? `
              <div class="cluster-quick-actions">
                <button type="button" class="btn-tiny" id="btn-clusters-select-all">Select All</button>
                <button type="button" class="btn-tiny" id="btn-clusters-deselect-all">Deselect All</button>
                <button type="button" class="btn-tiny" id="btn-clusters-clear-all" style="color: #ef4444;">Clear All</button>
              </div>
            ` : ''}
          </div>
          <div class="settings-rules-list">${colorRulesHtml}</div>

          <div class="settings-subheading" style="margin-top: 16px; display: flex; justify-content: space-between; align-items: center;">
            <span style="display: flex; align-items: center; gap: 4px;">
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M8 1.5a5 5 0 0 0-3.5 8.5c.7.7 1 1.5 1 2.5h5c0-1 .3-1.8 1-2.5A5 5 0 0 0 8 1.5z"/>
                <line x1="6.5" y1="14.5" x2="9.5" y2="14.5"/>
              </svg>
              Suggestions from Vault
            </span>
            ${unaddedSuggestions.length > 0 ? `
              <button type="button" class="btn-tiny" id="btn-cluster-autopopulate" title="Add top 6 suggestions automatically">
                ⚡ Auto-Populate
              </button>
            ` : ''}
          </div>
          <p class="settings-desc" style="margin-bottom: 8px;">
            Click any suggestion to add it as an active color group:
          </p>
          ${suggestionsChipsHtml}
        </div>
      `;
      break;
    }

    case 'git_sync':
      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Git Hook Integration (prek)</div>
          <p class="settings-desc">
            Ultra-fast non-blocking commit hooks (≤ 500ms fail-open budget) syncing incremental graph deltas.
          </p>
          <div class="settings-hook-card">
            <div class="settings-hook-row">
              <span class="hook-label">Hook Status:</span>
              <span class="hook-status-badge ${gitHooksInstalled ? 'status-clean' : 'status-warn'}">
                ${gitHooksInstalled ? '● Active & Installed' : '○ Not Installed'}
              </span>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">Last Delta Latency:</span>
              <span class="hook-val">${gitCommitLatencyMs} ms</span>
            </div>
          </div>
          <button class="btn-secondary" id="btn-reinstall-hooks" style="margin-top: 12px;">
            ${gitHooksInstalled
              ? `<svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;"><path d="M2.5 8a5.5 5.5 0 1 1 1.6 3.9L1.5 14M1.5 9.5v4.5h4.5"/></svg>Reinstall Git Hooks`
              : `<svg viewBox="0 0 16 16" width="12" height="12" fill="currentColor" style="display:inline-block; vertical-align:middle; margin-right:5px;"><path d="M7 1L1 9h5l-1 6 7-8h-5l1-6z"/></svg>Install Git Hooks`}
          </button>
        </div>
      `;
      break;

    case 'diagnostics':
      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Engine Diagnostics</div>
          <div class="settings-diag-grid">
            <div class="diag-item">
              <span class="diag-lbl">Total Notes</span>
              <span class="diag-val">${totalNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Connected</span>
              <span class="diag-val">${connectedNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Orphans</span>
              <span class="diag-val">${orphanNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Excluded</span>
              <span class="diag-val">${excludedNotes}</span>
            </div>
          </div>
          <div class="settings-diag-item" style="margin-top: 12px;">
            <span class="diag-lbl">Architecture:</span>
            <span class="diag-text">Dual-Process Sidecar (Rust IPC Core + Webview 2D Canvas)</span>
          </div>
          <div class="settings-diag-item">
            <span class="diag-lbl">Protocol:</span>
            <span class="diag-text">JSON-RPC 2.0 over Stdio + Loopback Delta Authorization</span>
          </div>
        </div>
      `;
      break;
  }

  return `
    <div class="settings-drawer" id="settings-drawer" style="display: none;">
      <div class="settings-header">
        <div class="settings-title-box">
          <span class="settings-icon">
            <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="8" cy="8" r="2.5"/>
              <path d="M8 1v2m0 10v2M1 8h2m10 0h2m-2.05-4.95l-1.41 1.41m-7.08 7.08l-1.41 1.41m0-9.9l1.41 1.41m7.08 7.08l1.41 1.41"/>
            </svg>
          </span>
          <h3 class="settings-title">Advanced Settings</h3>
        </div>
        <button class="settings-close-btn" id="settings-btn-close" aria-label="Close Settings">✕</button>
      </div>

      <div class="settings-tabs-bar">
        <button class="settings-tab-btn ${activeTab === 'knowledge' ? 'active' : ''}" data-tab="knowledge">Knowledge</button>
        <button class="settings-tab-btn ${activeTab === 'exclusions' ? 'active' : ''}" data-tab="exclusions">Excluded Paths</button>
        <button class="settings-tab-btn ${activeTab === 'clusters' ? 'active' : ''}" data-tab="clusters">Cluster Groups</button>
        <button class="settings-tab-btn ${activeTab === 'git_sync' ? 'active' : ''}" data-tab="git_sync">Git Sync</button>
        <button class="settings-tab-btn ${activeTab === 'diagnostics' ? 'active' : ''}" data-tab="diagnostics">Diagnostics</button>
      </div>

      <div class="settings-content" id="settings-content">
        ${tabContentHtml}
      </div>
    </div>
  `;
}

export function renderSettingsDrawer(
  container: HTMLElement,
  initialProps: SettingsDrawerProps,
  callbacks: SettingsDrawerCallbacks,
): SettingsDrawerController {
  let state = { ...initialProps };
  let isVisible = false;

  function render(): void {
    container.innerHTML = renderSettingsDrawerHtml(state);
    const drawer = container.querySelector('#settings-drawer') as HTMLElement;
    if (drawer) {
      drawer.style.display = isVisible ? 'flex' : 'none';
      drawer.addEventListener('mousedown', (e) => e.stopPropagation());
      drawer.addEventListener('mouseup', (e) => e.stopPropagation());
      drawer.addEventListener('click', (e) => e.stopPropagation());
    }

    container.querySelector('#settings-btn-close')?.addEventListener('click', (e) => {
      e.stopPropagation();
      close();
    });

    // Wire custom scope dropdown in Excluded Paths tab
    const scopeBtn = container.querySelector('#btn-scope-toggle') as HTMLElement;
    const scopeMenu = container.querySelector('#menu-scope-options') as HTMLElement;
    const scopeLabel = container.querySelector('#label-scope-selected') as HTMLElement;
    const scopeHidden = container.querySelector('#select-rule-scope') as HTMLInputElement;

    if (scopeBtn && scopeMenu) {
      scopeBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = scopeMenu.style.display === 'block';
        scopeMenu.style.display = isOpen ? 'none' : 'block';
      });

      scopeMenu.querySelectorAll('.scope-menu-item').forEach((item) => {
        item.addEventListener('click', (e) => {
          e.stopPropagation();
          const val = item.getAttribute('data-value') as 'scan' | 'graph';
          if (val && scopeHidden && scopeLabel) {
            scopeHidden.value = val;
            scopeLabel.textContent = val === 'graph' ? 'Graph Tier' : 'Scan Tier';
            scopeMenu.querySelectorAll('.scope-menu-item').forEach((i) => i.classList.remove('active'));
            item.classList.add('active');
            scopeMenu.style.display = 'none';
          }
        });
      });

      document.addEventListener('click', () => {
        if (scopeMenu) scopeMenu.style.display = 'none';
      });
    }

    container.querySelectorAll('.settings-tab-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const tab = btn.getAttribute('data-tab') as SettingsTab;
        if (tab && tab !== state.activeTab) {
          state.activeTab = tab;
          callbacks.onChangeTab?.(tab);
          render();
        }
      });
    });

    container.querySelector('#btn-settings-change-hub')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onChangeHub?.();
    });

    container.querySelector('#btn-settings-fix-broken')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onFixBrokenLinks?.();
    });

    container.querySelector('#btn-settings-rescan')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onRescanVault?.();
    });

    container.querySelector('#btn-reinstall-hooks')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onInstallGitHooks?.();
    });

    container.querySelector('#btn-add-rule')?.addEventListener('click', (e) => {
      e.stopPropagation();
      const input = container.querySelector('#input-new-rule') as HTMLInputElement;
      const select = container.querySelector('#select-rule-scope') as HTMLInputElement;
      const pat = input?.value.trim();
      const scope = (select?.value as 'scan' | 'graph') || 'graph';
      if (pat) {
        callbacks.onAddExclusion?.(pat, scope);
        if (scope === 'graph') {
          state.graphExclusions.push(pat);
        } else {
          state.scanExclusions.push(pat);
        }
        render();
      }
    });

    container.querySelectorAll('.rule-del-btn').forEach((btn) => {
      btn.addEventListener('click', () => {
        const pat = btn.getAttribute('data-rule');
        const scope = (btn.getAttribute('data-scope') as 'scan' | 'graph') || 'graph';
        if (pat) {
          callbacks.onDeleteExclusion?.(pat, scope);
          if (scope === 'graph') {
            state.graphExclusions = state.graphExclusions.filter((r) => r !== pat);
          } else {
            state.scanExclusions = state.scanExclusions.filter((r) => r !== pat);
          }
          render();
        }
      });
    });

    // Cluster Groups event wiring
    const addClusterBtn = container.querySelector('#btn-add-cluster-group');
    const clusterQueryInput = container.querySelector('#input-cluster-query') as HTMLInputElement;
    const clusterLabelInput = container.querySelector('#input-cluster-label') as HTMLInputElement;
    const clusterColorInput = container.querySelector('#input-cluster-color') as HTMLInputElement;

    const doAddClusterGroup = () => {
      let query = clusterQueryInput?.value.trim();
      let label = clusterLabelInput?.value.trim();
      const color = clusterColorInput?.value || DEFAULT_PALETTE[state.colorGroups.length % DEFAULT_PALETTE.length];

      if (!query) return;

      if (!query.startsWith('path:') && !query.startsWith('tag:')) {
        if (query.startsWith('#')) {
          query = `tag:${query}`;
        } else {
          query = `path:${query}`;
        }
      }

      if (!label) {
        if (query.startsWith('tag:')) {
          label = query.slice(4).trim();
        } else if (query.startsWith('path:')) {
          const p = query.slice(5).trim().replace(/\/\*\*$/, '');
          label = p.split('/').pop() || p;
        } else {
          label = query;
        }
      }

      state.colorGroups.push({
        query,
        label,
        name: label,
        color,
        enabled: true,
      });
      callbacks.onSaveColorGroups?.(state.colorGroups);
      render();
    };

    addClusterBtn?.addEventListener('click', (e) => {
      e.stopPropagation();
      doAddClusterGroup();
    });

    clusterQueryInput?.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        e.stopPropagation();
        doAddClusterGroup();
      }
    });

    clusterLabelInput?.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        e.stopPropagation();
        doAddClusterGroup();
      }
    });

    container.querySelectorAll('.btn-add-suggestion').forEach((chip) => {
      chip.addEventListener('click', (e) => {
        e.stopPropagation();
        const query = chip.getAttribute('data-query') || '';
        const label = chip.getAttribute('data-label') || '';
        const color = chip.getAttribute('data-color') || DEFAULT_PALETTE[state.colorGroups.length % DEFAULT_PALETTE.length];
        if (query) {
          state.colorGroups.push({
            query,
            label: label || query,
            name: label || query,
            color,
            enabled: true,
          });
          callbacks.onSaveColorGroups?.(state.colorGroups);
          render();
        }
      });
    });

    container.querySelector('#btn-cluster-autopopulate')?.addEventListener('click', (e) => {
      e.stopPropagation();
      const suggestions = discoverVaultSuggestions(state.nodes || []);
      const existingQueries = new Set(state.colorGroups.map((g) => g.query.trim().toLowerCase()));
      const toAdd = suggestions.filter((s) => !existingQueries.has(s.query.trim().toLowerCase())).slice(0, 6);
      if (toAdd.length > 0) {
        for (const s of toAdd) {
          state.colorGroups.push({
            query: s.query,
            label: s.label,
            name: s.label,
            color: s.color,
            enabled: true,
          });
        }
        callbacks.onSaveColorGroups?.(state.colorGroups);
        render();
      }
    });

    container.querySelector('#btn-clusters-select-all')?.addEventListener('click', (e) => {
      e.stopPropagation();
      state.colorGroups.forEach((g) => (g.enabled = true));
      callbacks.onSaveColorGroups?.(state.colorGroups);
      render();
    });

    container.querySelector('#btn-clusters-deselect-all')?.addEventListener('click', (e) => {
      e.stopPropagation();
      state.colorGroups.forEach((g) => (g.enabled = false));
      callbacks.onSaveColorGroups?.(state.colorGroups);
      render();
    });

    container.querySelector('#btn-clusters-clear-all')?.addEventListener('click', (e) => {
      e.stopPropagation();
      state.colorGroups = [];
      callbacks.onSaveColorGroups?.(state.colorGroups);
      render();
    });

    container.querySelectorAll('.chk-cluster-enabled').forEach((chk) => {
      chk.addEventListener('change', (e) => {
        e.stopPropagation();
        const idx = Number((e.target as HTMLElement).getAttribute('data-idx'));
        if (!isNaN(idx) && state.colorGroups[idx]) {
          state.colorGroups[idx].enabled = (e.target as HTMLInputElement).checked;
          callbacks.onSaveColorGroups?.(state.colorGroups);
          render();
        }
      });
    });

    container.querySelectorAll('.inp-cluster-color').forEach((inp) => {
      inp.addEventListener('change', (e) => {
        e.stopPropagation();
        const idx = Number((e.target as HTMLElement).getAttribute('data-idx'));
        if (!isNaN(idx) && state.colorGroups[idx]) {
          state.colorGroups[idx].color = (e.target as HTMLInputElement).value;
          callbacks.onSaveColorGroups?.(state.colorGroups);
          render();
        }
      });
    });

    container.querySelectorAll('.btn-delete-cluster').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const idx = Number((e.target as HTMLElement).getAttribute('data-idx'));
        if (!isNaN(idx) && state.colorGroups[idx]) {
          state.colorGroups.splice(idx, 1);
          callbacks.onSaveColorGroups?.(state.colorGroups);
          render();
        }
      });
    });
  }

  function open(tab?: SettingsTab): void {
    if (tab) state.activeTab = tab;
    isVisible = true;
    render();
  }

  function close(): void {
    isVisible = false;
    const drawer = container.querySelector('#settings-drawer') as HTMLElement;
    if (drawer) {
      drawer.style.display = 'none';
    }
    callbacks.onClose?.();
  }

  render();

  return {
    open(tab?: SettingsTab) {
      open(tab);
    },
    close() {
      close();
    },
    isOpen() {
      return isVisible;
    },
    updateProps(newProps: Partial<SettingsDrawerProps>) {
      state = { ...state, ...newProps };
      render();
    },
  };
}
