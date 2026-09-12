import type { ClusterColorGroup, NoteNode } from '../../../src/types/graph.ts';

export interface ColorManagerCallbacks {
  onSaveGroups: (groups: ClusterColorGroup[]) => void;
  onClose: () => void;
}

export interface ClusterSuggestion {
  query: string;
  color: string;
  label: string;
  count: number;
  type: 'path' | 'tag';
}

export const DEFAULT_PALETTE = [
  '#3B82F6', // Blue
  '#10B981', // Emerald
  '#F59E0B', // Amber
  '#8B5CF6', // Purple
  '#EC4899', // Pink
  '#06B6D4', // Cyan
  '#F97316', // Orange
  '#14B8A6', // Teal
  '#84CC16', // Lime
  '#E11D48', // Rose
  '#6366F1', // Indigo
  '#A855F7', // Violet
];

export function countMatchingNotes(query: string, nodes: NoteNode[]): number {
  if (!query || !nodes || nodes.length === 0) return 0;
  const cleanQuery = query.trim();

  if (cleanQuery.startsWith('tag:')) {
    const pat = cleanQuery.slice(4).trim().toLowerCase().replace(/^#/, '');
    return nodes.filter((n) =>
      n.tags?.some((t) => t.toLowerCase().replace(/^#/, '') === pat),
    ).length;
  }

  if (cleanQuery.startsWith('path:')) {
    const pat = cleanQuery.slice(5).trim();
    if (pat.endsWith('/**')) {
      const prefix = pat.slice(0, -3);
      return nodes.filter((n) => {
        const p = n.filePath || n.id;
        return p.startsWith(prefix) || p.replace(/^notes\//, '').startsWith(prefix);
      }).length;
    }
    const regexStr =
      '^' +
      pat
        .replace(/\./g, '\\.')
        .replace(/\*\*/g, '.*')
        .replace(/\*/g, '[^/]*') +
      '$';
    try {
      const re = new RegExp(regexStr);
      return nodes.filter((n) => {
        const p = n.filePath || n.id;
        return re.test(p) || re.test(p.replace(/^notes\//, ''));
      }).length;
    } catch {
      return 0;
    }
  }

  return 0;
}

export function discoverVaultSuggestions(nodes: NoteNode[]): ClusterSuggestion[] {
  if (!nodes || nodes.length === 0) return [];

  const folderCounts = new Map<string, number>();
  const tagCounts = new Map<string, number>();

  for (const n of nodes) {
    // 1. Folder path discovery
    const path = (n.filePath || n.id).replace(/^\/+/, '');
    if (path) {
      const parts = path.split('/');
      if (parts.length > 1) {
        const topFolder = parts[0];
        folderCounts.set(topFolder, (folderCounts.get(topFolder) || 0) + 1);

        if (parts.length > 2) {
          const subFolder = `${parts[0]}/${parts[1]}`;
          folderCounts.set(subFolder, (folderCounts.get(subFolder) || 0) + 1);
        }
      }
    }

    // 2. Tag discovery
    if (n.tags && Array.isArray(n.tags)) {
      for (const t of n.tags) {
        const cleanTag = t.trim().replace(/^#/, '');
        if (cleanTag) {
          tagCounts.set(cleanTag, (tagCounts.get(cleanTag) || 0) + 1);
        }
      }
    }
  }

  const suggestions: ClusterSuggestion[] = [];
  let colorIdx = 0;

  // Folder suggestions
  const sortedFolders = Array.from(folderCounts.entries())
    .filter(([folder, count]) => {
      if (count < 1 || folder.startsWith('.')) return false;
      // If a single top-level folder contains all nodes and has subfolders (e.g. 'notes/'), skip the umbrella
      if (nodes.length > 3 && count === nodes.length && !folder.includes('/')) {
        const hasSubfolders = Array.from(folderCounts.keys()).some((k) => k.startsWith(`${folder}/`));
        if (hasSubfolders) return false;
      }
      return true;
    })
    .sort((a, b) => b[1] - a[1]);

  for (const [folder, count] of sortedFolders) {
    suggestions.push({
      query: `path:${folder}/**`,
      color: DEFAULT_PALETTE[colorIdx % DEFAULT_PALETTE.length],
      label: folder,
      count,
      type: 'path',
    });
    colorIdx++;
  }

  // Tag suggestions
  const sortedTags = Array.from(tagCounts.entries())
    .filter(([_, count]) => count >= 1)
    .sort((a, b) => b[1] - a[1]);

  for (const [tag, count] of sortedTags) {
    suggestions.push({
      query: `tag:#${tag}`,
      color: DEFAULT_PALETTE[colorIdx % DEFAULT_PALETTE.length],
      label: `#${tag}`,
      count,
      type: 'tag',
    });
    colorIdx++;
  }

  return suggestions;
}

export function renderColorManager(
  container: HTMLElement,
  currentGroups: ClusterColorGroup[],
  callbacks: ColorManagerCallbacks,
  nodes: NoteNode[] = [],
): void {
  const suggestions = discoverVaultSuggestions(nodes);

  // If no current groups exist, auto-populate with top detected folder suggestions
  let groups: ClusterColorGroup[] = [...currentGroups];
  if (groups.length === 0 && suggestions.length > 0) {
    const initialSuggestions = suggestions.slice(0, 6);
    groups = initialSuggestions.map((s) => ({
      query: s.query,
      color: s.color,
      label: s.label,
      enabled: true,
    }));
  }

  const render = () => {
    // Determine which suggestions are not yet added
    const existingQueries = new Set(groups.map((g) => g.query.trim()));
    const unaddedSuggestions = suggestions.filter((s) => !existingQueries.has(s.query.trim()));

    container.innerHTML = `
      <div class="color-manager-overlay">
        <div class="color-manager-panel">
          <div class="color-manager-header">
            <h3>Chromatic Cluster Groups</h3>
            <button id="btn-close-colors" class="btn-icon" title="Close">&times;</button>
          </div>
          <p class="color-manager-subtitle">
            Color nodes by folder path (<code>path:folder/**</code>) or hashtag (<code>tag:#tag</code>).
            Flag checkboxes to apply or unapply rules. Opacity follows hop depth from hub.
          </p>

          <div class="color-manager-tools">
            <button id="btn-select-all" class="btn-tiny"><svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" style="vertical-align: -1px; margin-right: 3px;"><polyline points="3 8 6.5 11.5 13 4.5"/></svg>Select All</button>
            <button id="btn-deselect-all" class="btn-tiny"><svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.5" style="vertical-align: -1px; margin-right: 3px;"><rect x="2.5" y="2.5" width="11" height="11" rx="1.5"/></svg>Deselect All</button>
            <button id="btn-populate-vault" class="btn-tiny"><svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="vertical-align: -1px; margin-right: 3px;"><polygon points="8.5 1.5 3 9 7.5 9 6.5 14.5 13 6.5 8.5 6.5 8.5 1.5"/></svg>Auto-Populate</button>
            <button id="btn-add-group" class="btn-tiny">+ Add Custom Rule</button>
          </div>

          <div class="color-groups-list">
            ${
              groups.length === 0
                ? '<div style="color: var(--vscode-descriptionForeground); font-size: 11px; padding: 12px; text-align: center;">No cluster rules defined yet. Click suggestions below or add a rule.</div>'
                : groups
                    .map((g, idx) => {
                      const matchCount = countMatchingNotes(g.query, nodes);
                      return `
                <div class="color-group-item" data-idx="${idx}">
                  <input type="checkbox" class="chk-group-enabled" ${g.enabled ? 'checked' : ''} title="Flag rule to apply to graph" />
                  <input type="color" class="inp-group-color" value="${g.color}" title="Choose cluster hue" />
                  <input type="text" class="inp-group-label" value="${g.label}" placeholder="Label" title="Cluster Name" />
                  <input type="text" class="inp-group-query" value="${g.query}" placeholder="tag:#... or path:..." title="Pattern Query" />
                  <span class="rule-match-count" title="${matchCount} notes currently match this rule">${matchCount} note${matchCount === 1 ? '' : 's'}</span>
                  <button class="btn-icon btn-remove-group" title="Delete rule">&times;</button>
                </div>
              `;
                    })
                    .join('')
            }
          </div>

          ${
            unaddedSuggestions.length > 0
              ? `
            <div class="suggestions-section">
              <div class="suggestions-header">
                <span class="suggestions-title"><svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="#3b82f6" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="vertical-align: -2px; margin-right: 4px;"><path d="M8 1.5a5 5 0 0 0-3.5 8.5c.7.7 1 1.5 1 2.5h5c0-1 .3-1.8 1-2.5A5 5 0 0 0 8 1.5z"/><line x1="6.5" y1="14.5" x2="9.5" y2="14.5"/></svg>Detected Vault Suggestions</span>
                <span class="suggestions-subtitle">Click to add as rule</span>
              </div>
              <div class="suggestions-list">
                ${unaddedSuggestions
                  .map(
                    (s) => `
                  <button class="suggestion-chip" data-query="${s.query}" data-label="${s.label}" data-color="${s.color}" title="Add ${s.query} (${s.count} notes)">
                    <span class="chip-dot" style="background-color: ${s.color};"></span>
                    <span>${s.label}</span>
                    <span class="chip-count">${s.count}</span>
                  </button>
                `,
                  )
                  .join('')}
              </div>
            </div>
          `
              : ''
          }

          <div class="color-manager-actions">
            <span style="font-size: 11px; color: var(--vscode-descriptionForeground);">
              ${groups.filter((g) => g.enabled).length} of ${groups.length} active
            </span>
            <div class="actions-right">
              <button id="btn-cancel-colors" class="btn-secondary">Cancel</button>
              <button id="btn-save-colors" class="btn-primary">Apply & Save</button>
            </div>
          </div>
        </div>
      </div>
    `;

    // Hook listeners
    container.querySelector('#btn-close-colors')?.addEventListener('click', callbacks.onClose);
    container.querySelector('#btn-cancel-colors')?.addEventListener('click', callbacks.onClose);

    // Select All
    container.querySelector('#btn-select-all')?.addEventListener('click', () => {
      groups.forEach((g) => (g.enabled = true));
      render();
    });

    // Deselect All
    container.querySelector('#btn-deselect-all')?.addEventListener('click', () => {
      groups.forEach((g) => (g.enabled = false));
      render();
    });

    // Auto-Populate from Vault
    container.querySelector('#btn-populate-vault')?.addEventListener('click', () => {
      if (suggestions.length > 0) {
        groups = suggestions.map((s) => ({
          query: s.query,
          color: s.color,
          label: s.label,
          enabled: true,
        }));
        render();
      }
    });

    // Add Custom Rule
    container.querySelector('#btn-add-group')?.addEventListener('click', () => {
      const nextColor = DEFAULT_PALETTE[groups.length % DEFAULT_PALETTE.length];
      groups.push({
        query: 'path:new_folder/**',
        color: nextColor,
        label: 'New Cluster',
        enabled: true,
      });
      render();
    });

    // Suggestion chips
    container.querySelectorAll('.suggestion-chip').forEach((chip) => {
      chip.addEventListener('click', () => {
        const query = chip.getAttribute('data-query') || '';
        const label = chip.getAttribute('data-label') || '';
        const color = chip.getAttribute('data-color') || '#3B82F6';
        groups.push({
          query,
          color,
          label,
          enabled: true,
        });
        render();
      });
    });

    // Individual rule controls
    container.querySelectorAll('.color-group-item').forEach((item) => {
      const idx = Number(item.getAttribute('data-idx'));
      const chk = item.querySelector('.chk-group-enabled') as HTMLInputElement;
      const colorInp = item.querySelector('.inp-group-color') as HTMLInputElement;
      const labelInp = item.querySelector('.inp-group-label') as HTMLInputElement;
      const queryInp = item.querySelector('.inp-group-query') as HTMLInputElement;
      const removeBtn = item.querySelector('.btn-remove-group');

      chk?.addEventListener('change', () => {
        groups[idx].enabled = chk.checked;
      });
      colorInp?.addEventListener('input', () => {
        groups[idx].color = colorInp.value;
      });
      labelInp?.addEventListener('change', () => {
        groups[idx].label = labelInp.value;
      });
      queryInp?.addEventListener('change', () => {
        groups[idx].query = queryInp.value;
        render();
      });
      removeBtn?.addEventListener('click', () => {
        groups.splice(idx, 1);
        render();
      });
    });

    container.querySelector('#btn-save-colors')?.addEventListener('click', () => {
      callbacks.onSaveGroups(groups);
      callbacks.onClose();
    });
  };

  render();
}
