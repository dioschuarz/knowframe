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

import type { ClusterColorGroup } from '../../../src/types/graph.ts';

export type SettingsTab = 'knowledge' | 'exclusions' | 'clusters' | 'git_sync' | 'diagnostics';

export interface SettingsDrawerProps {
  activeTab: SettingsTab;
  primaryHub?: string | null;
  brokenLinksCount?: number;
  scanExclusions: string[];
  graphExclusions: string[];
  colorGroups: ClusterColorGroup[];
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
            <span class="settings-hub-icon">⭐</span>
            <span class="settings-hub-path" title="${primaryHub}">${primaryHub}</span>
            <button class="btn-secondary settings-hub-btn" id="btn-settings-change-hub">Change Hub</button>
          </div>
        </div>

        <div class="settings-section">
          <div class="settings-sec-title">Link Integrity</div>
          <p class="settings-desc">Broken links pointing to nonexistent notes or invalid section anchors.</p>
          <div class="settings-health-row">
            <span class="settings-health-status ${brokenLinksCount === 0 ? 'status-clean' : 'status-warn'}">
              ${brokenLinksCount === 0 ? '✓ No broken links detected' : `⚠ ${brokenLinksCount} broken references`}
            </span>
            ${brokenLinksCount > 0 ? `<button class="btn-primary" id="btn-settings-fix-broken">Fix References</button>` : ''}
          </div>
        </div>

        <div class="settings-section">
          <div class="settings-sec-title">Vault Re-indexing</div>
          <p class="settings-desc">Trigger a full AST parse and graph topology rebuild from workspace files.</p>
          <button class="btn-secondary" id="btn-settings-rescan">🔄 Rescan Vault Now</button>
        </div>
      `;
      break;

    case 'exclusions':
      const graphRulesHtml = graphExclusions.length > 0
        ? graphExclusions.map((rule) => `
            <div class="settings-rule-item">
              <span class="rule-pattern" title="${rule}">🚫 ${rule}</span>
              <span class="rule-tier-tag">graph</span>
              <button class="rule-del-btn" data-rule="${rule}" data-scope="graph" title="Remove rule">✕</button>
            </div>
          `).join('')
        : '<div class="settings-empty-msg">No graph exclusion rules configured</div>';

      const scanRulesHtml = scanExclusions.length > 0
        ? scanExclusions.map((rule) => `
            <div class="settings-rule-item">
              <span class="rule-pattern" title="${rule}">⛔ ${rule}</span>
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
            <select id="select-rule-scope" class="settings-select">
              <option value="graph">Graph Tier</option>
              <option value="scan">Scan Tier</option>
            </select>
            <button class="btn-primary" id="btn-add-rule">+ Add Rule</button>
          </div>

          <div class="settings-subheading">Graph Exclusions (Visual & Context Hiding)</div>
          <div class="settings-rules-list">${graphRulesHtml}</div>

          <div class="settings-subheading" style="margin-top: 14px;">Scan Exclusions (Completely Ignored)</div>
          <div class="settings-rules-list">${scanRulesHtml}</div>
        </div>
      `;
      break;

    case 'clusters':
      const colorRulesHtml = colorGroups.length > 0
        ? colorGroups.map((g) => `
            <div class="settings-rule-item">
              <span class="color-swatch-dot" style="background-color: ${g.color};"></span>
              <span class="rule-name">${g.name}</span>
              <span class="rule-pattern">${g.query}</span>
              <span class="rule-tier-tag">${g.enabled ? 'Active' : 'Disabled'}</span>
            </div>
          `).join('')
        : '<div class="settings-empty-msg">No custom cluster color groups defined</div>';

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Cluster Color Groups</div>
          <p class="settings-desc">Semantic color groups categorizing notes by tag (#tag) or folder path (path:folder/**).</p>
          <div class="settings-rules-list">${colorRulesHtml}</div>
        </div>
      `;
      break;

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
            ${gitHooksInstalled ? '🔄 Reinstall Git Hooks' : '⚡ Install Git Hooks'}
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
          <span class="settings-icon">⚙️</span>
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
    }

    container.querySelector('#settings-btn-close')?.addEventListener('click', () => {
      close();
    });

    container.querySelectorAll('.settings-tab-btn').forEach((btn) => {
      btn.addEventListener('click', () => {
        const tab = btn.getAttribute('data-tab') as SettingsTab;
        if (tab && tab !== state.activeTab) {
          state.activeTab = tab;
          callbacks.onChangeTab?.(tab);
          render();
        }
      });
    });

    container.querySelector('#btn-settings-change-hub')?.addEventListener('click', () => {
      callbacks.onChangeHub?.();
    });

    container.querySelector('#btn-settings-fix-broken')?.addEventListener('click', () => {
      callbacks.onFixBrokenLinks?.();
    });

    container.querySelector('#btn-settings-rescan')?.addEventListener('click', () => {
      callbacks.onRescanVault?.();
    });

    container.querySelector('#btn-reinstall-hooks')?.addEventListener('click', () => {
      callbacks.onInstallGitHooks?.();
    });

    container.querySelector('#btn-add-rule')?.addEventListener('click', () => {
      const input = container.querySelector('#input-new-rule') as HTMLInputElement;
      const select = container.querySelector('#select-rule-scope') as HTMLSelectElement;
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
