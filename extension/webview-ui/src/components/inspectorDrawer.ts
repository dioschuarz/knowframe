/**
 * Contextual Selection Inspector Component
 *
 * Implements Constitution Principle VI (Zero Action Duplication & State-Driven Emergence).
 * Provides a slide-over Contextual Inspector drawer that displays relevant metadata
 * and actionable operations whenever a note node, edge, or token status metric is selected.
 */

import type { NoteNode, NoteEdge, TokenSummary } from '../../../src/types/graph.ts';
import { formatCompactNumber } from '../utils/numberFormat.ts';

export function formatHopTier(hopDistance: number): string {
  if (hopDistance === 0) return 'Level 0 (Central Hub)';
  if (hopDistance === 1) return 'Level 1 (Direct Reference)';
  if (hopDistance === 2) return 'Level 2 (Transitive Reference)';
  if (hopDistance >= 3) return `Level ${hopDistance} (Deep Reference)`;
  return 'Orphan (Disconnected)';
}

function formatTokens(tokens: number): string {
  return formatCompactNumber(tokens);
}

export interface NodeInspectionProps {
  node: NoteNode;
  inLinks: string[];
  outLinks: string[];
  isHub: boolean;
}

export interface EdgeInspectionProps {
  edge: NoteEdge;
  sourceLabel?: string;
}

export interface TokenEconomicsProps {
  summary: TokenSummary;
  topNodes: Array<{ label: string; filePath: string; tokens: number }>;
}

export interface InspectorCallbacks {
  onOpenFile?: (filePath: string) => void;
  onOpenSource?: (sourcePath: string, line?: number) => void;
  onSetHub?: (hubId: string) => void;
  onExclude?: (pattern: string) => void;
  onFixReference?: (edge: NoteEdge) => void;
  onNavigateLink?: (target: string) => void;
  onClose?: () => void;
}

export interface InspectorDrawerController {
  inspectNode: (node: NoteNode, inLinks: string[], outLinks: string[], isHub: boolean) => void;
  inspectEdge: (edge: NoteEdge, sourceLabel?: string) => void;
  inspectTokenEconomics: (summary: TokenSummary, topNodes: Array<{ label: string; filePath: string; tokens: number }>) => void;
  open: () => void;
  close: () => void;
  isOpen: () => boolean;
}

export function renderNodeInspectionHtml(props: NodeInspectionProps): string {
  const { node, inLinks, outLinks, isHub } = props;
  const tierName = formatHopTier(node.hopDistance);
  const tokenFormatted = formatCompactNumber(node.tokens) + ' tok';

  const inLinksList = inLinks.length > 0
    ? inLinks.map((l) => `<li class="insp-link-item" data-link="${l}"><span class="insp-link-icon"><svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M10 12L6 8L10 4"/></svg></span><span class="insp-link-text" title="${l}">${l}</span></li>`).join('')
    : '<li class="insp-empty-item">No incoming links</li>';

  const outLinksList = outLinks.length > 0
    ? outLinks.map((l) => `<li class="insp-link-item" data-link="${l}"><span class="insp-link-icon"><svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M6 4L10 8L6 12"/></svg></span><span class="insp-link-text" title="${l}">${l}</span></li>`).join('')
    : '<li class="insp-empty-item">No outgoing links</li>';

  const tagsChips = (node.tags || []).length > 0
    ? (node.tags || []).map((t) => `<span class="insp-tag-chip">${t}</span>`).join('')
    : '<span class="insp-meta-empty">No tags</span>';

  return `
    <div class="insp-header">
      <div class="insp-title-box">
        <span class="insp-icon">
          ${isHub
            ? `<svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="6"/><circle cx="8" cy="8" r="2.5" fill="#38bdf8"/></svg>`
            : `<svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 2.5A1.5 1.5 0 0 1 4.5 1h7A1.5 1.5 0 0 1 13 2.5v11a1.5 1.5 0 0 1-1.5 1.5h-7A1.5 1.5 0 0 1 3 13.5v-11Z"/><line x1="5.5" y1="4.5" x2="10.5" y2="4.5"/><line x1="5.5" y1="7.5" x2="10.5" y2="7.5"/></svg>`
          }
        </span>
        <div class="insp-header-titles">
          <div class="insp-title" title="${node.label}">${node.label}</div>
          <div class="insp-path" title="${node.filePath}">${node.filePath}</div>
        </div>
      </div>
      <button class="insp-close-btn" id="insp-btn-close" title="Close Inspector">✕</button>
    </div>

    <div class="insp-body insp-body-node">
      <!-- Section: Tier & Token Weight (Fixed header) -->
      <div class="insp-section insp-section-fixed">
        <div class="insp-section-header">CONTEXT OBSERVER TIER</div>
        <div class="insp-tier-badge ${node.hopDistance === 0 ? 'tier-hub' : node.hopDistance === 1 ? 'tier-1' : node.hopDistance === 2 ? 'tier-2' : node.hopDistance > 2 ? 'tier-deep' : 'tier-orphan'}">
          ${tierName}
        </div>
        <div class="insp-metrics-row">
          <div class="insp-metric-col">
            <span class="insp-m-val">${tokenFormatted}</span>
            <span class="insp-m-lbl">BPE Tokens</span>
          </div>
          <div class="insp-metric-col">
            <span class="insp-m-val">${formatCompactNumber(node.inDegree)} / ${formatCompactNumber(node.outDegree)}</span>
            <span class="insp-m-lbl">In / Out Degree</span>
          </div>
          <div class="insp-metric-col">
            <span class="insp-m-val">${(node.betweenness || 0).toFixed(3)}</span>
            <span class="insp-m-lbl">Betweenness</span>
          </div>
        </div>
      </div>

      <!-- Section: Fragments (3-column: Fragment | Tok | I/O) -->
      <div class="insp-section insp-section-fragments" style="max-height: 180px; overflow-y: auto;">
        <div class="insp-section-header" style="cursor: pointer; display: flex; justify-content: space-between; align-items: center;" id="insp-fragments-toggle">
          <span>FRAGMENTS (${formatCompactNumber((node.fragments || []).length)})</span>
          <span class="insp-chevron-icon" id="insp-fragments-chevron">▼</span>
        </div>
        <div id="insp-fragments-content">
          ${(node.fragments || []).length > 0
            ? `
              <table class="insp-fragments-table" style="width: 100%; border-collapse: collapse; font-size: 11px;">
                <thead>
                  <tr style="border-bottom: 1px solid var(--vscode-widget-border, #333); opacity: 0.8; text-align: left;">
                    <th style="padding: 4px 6px;">Fragment</th>
                    <th style="padding: 4px 6px; text-align: right;">Tok</th>
                    <th style="padding: 4px 6px; text-align: right;">I/O</th>
                  </tr>
                </thead>
                <tbody>
                  ${(node.fragments || []).map((f) => `
                    <tr class="insp-fragment-row" data-start-line="${f.start_line}" data-end-line="${f.end_line}" data-anchor="${f.anchor}" style="cursor: pointer; border-bottom: 1px solid rgba(255,255,255,0.05);" title="Click to navigate to line ${f.start_line}">
                      <td style="padding: 4px 6px; max-width: 130px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                        <span style="font-weight: 500;">${f.heading}</span>
                        <span style="opacity: 0.6; font-size: 10px; margin-left: 4px;">#${f.anchor}</span>
                      </td>
                      <td style="padding: 4px 6px; text-align: right; font-variant-numeric: tabular-nums;">
                        ${formatCompactNumber(f.tokens)}
                      </td>
                      <td style="padding: 4px 6px; text-align: right; font-variant-numeric: tabular-nums;" title="Inbound references: ${f.inbound} · Outbound references: ${f.outbound}">
                        ${formatCompactNumber(f.inbound)}/${formatCompactNumber(f.outbound)}
                      </td>
                    </tr>
                  `).join('')}
                </tbody>
              </table>
            `
            : '<div class="insp-empty-item">No heading sections found</div>'
          }
        </div>
      </div>

      <!-- Section: Tags (1/3 of remaining space, scrollable) -->
      <div class="insp-section insp-section-third">
        <div class="insp-section-header">TAGS</div>
        <div class="insp-tags-row">${tagsChips}</div>
      </div>

      <!-- Section: Incoming References (1/3 of remaining space, scrollable) -->
      <div class="insp-section insp-section-third">
        <div class="insp-section-header">INCOMING REFERENCES (${formatCompactNumber(inLinks.length)})</div>
        <ul class="insp-links-list">${inLinksList}</ul>
      </div>

      <!-- Section: Outgoing References (1/3 of remaining space, scrollable) -->
      <div class="insp-section insp-section-third">
        <div class="insp-section-header">OUTGOING REFERENCES (${formatCompactNumber(outLinks.length)})</div>
        <ul class="insp-links-list">${outLinksList}</ul>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="insp-footer">
      <button class="btn-primary insp-action-btn" id="insp-btn-open-file">
        <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:6px;"><path d="M2.5 3.5A1.5 1.5 0 0 1 4 2h8a1.5 1.5 0 0 1 1.5 1.5v9a1.5 1.5 0 0 1-1.5 1.5H4a1.5 1.5 0 0 1-1.5-1.5v-9Z"/><line x1="5" y1="5.5" x2="11" y2="5.5"/><line x1="5" y1="8.5" x2="11" y2="8.5"/></svg>
        <span>Open file</span>
      </button>
      ${!isHub ? `
      <button class="btn-secondary insp-action-btn" id="insp-btn-set-hub">
        <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:6px;"><circle cx="8" cy="8" r="6"/><circle cx="8" cy="8" r="2.5" fill="currentColor"/></svg>
        <span>Set as hub</span>
      </button>` : ''}
      <button class="btn-secondary insp-action-btn" id="insp-btn-exclude">
        <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:6px;"><circle cx="8" cy="8" r="6"/><line x1="3.5" y1="3.5" x2="12.5" y2="12.5"/></svg>
        <span>Exclude note</span>
      </button>
    </div>
  `;
}

export function renderEdgeInspectionHtml(props: EdgeInspectionProps): string {
  const { edge, sourceLabel } = props;
  const sourceName = sourceLabel || edge.source;
  const lineCoord = edge.line ? `Line ${edge.line}` : 'Unspecified line';
  const errorType = edge.resolutionStatus || (edge.isBroken ? 'broken_link' : 'resolved');

  return `
    <div class="insp-header">
      <div class="insp-title-box">
        <span class="insp-icon">
          <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#ef4444" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M8 1.5L14.5 13.5H1.5L8 1.5Z"/><line x1="8" y1="6" x2="8" y2="9.5"/><circle cx="8" cy="11.5" r="0.75" fill="#ef4444"/></svg>
        </span>
        <div class="insp-header-titles">
          <div class="insp-title">Broken Reference</div>
          <div class="insp-path">${sourceName} : ${lineCoord}</div>
        </div>
      </div>
      <button class="insp-close-btn" id="insp-btn-close" title="Close Inspector">✕</button>
    </div>

    <div class="insp-body">
      <div class="insp-section">
        <div class="insp-section-header">REFERENCE DETAILS</div>
        <div class="insp-broken-detail-box">
          <div class="insp-detail-row">
            <span class="insp-d-lbl">Source File:</span>
            <span class="insp-d-val" title="${edge.source}">${edge.source}</span>
          </div>
          <div class="insp-detail-row">
            <span class="insp-d-lbl">Line Coordinate:</span>
            <span class="insp-d-val highlight-line">${lineCoord}</span>
          </div>
          <div class="insp-detail-row">
            <span class="insp-d-lbl">Missing Target:</span>
            <span class="insp-d-val target-missing" title="${edge.target}">${edge.target}</span>
          </div>
          ${edge.anchor ? `
          <div class="insp-detail-row">
            <span class="insp-d-lbl">Heading Anchor:</span>
            <span class="insp-d-val">#${edge.anchor}</span>
          </div>` : ''}
          <div class="insp-detail-row">
            <span class="insp-d-lbl">Error Type:</span>
            <span class="insp-error-badge">${errorType}</span>
          </div>
        </div>
      </div>

      <div class="insp-section">
        <div class="insp-section-header">RESOLUTION ADVICE</div>
        <div class="insp-advice-text">
          The link in <code>${edge.source}</code> on line ${edge.line || 1} points to <code>${edge.target}</code> which does not resolve to an existing note in this workspace. Click <strong>Fix reference</strong> to re-point the link to an existing file, or create the missing note.
        </div>
      </div>
    </div>

    <div class="insp-footer">
      <button class="btn-primary insp-action-btn" id="insp-btn-open-source">
        <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:6px;"><path d="M2.5 3.5A1.5 1.5 0 0 1 4 2h8a1.5 1.5 0 0 1 1.5 1.5v9a1.5 1.5 0 0 1-1.5 1.5H4a1.5 1.5 0 0 1-1.5-1.5v-9Z"/><line x1="5" y1="5.5" x2="11" y2="5.5"/><line x1="5" y1="8.5" x2="11" y2="8.5"/></svg>
        <span>Open source (${lineCoord})</span>
      </button>
      <button class="btn-secondary insp-action-btn" id="insp-btn-fix-reference">
        <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:6px;"><path d="M12.5 1.5a3 3 0 0 0-3.8 3.8L2 12l2 2 6.7-6.7a3 3 0 0 0 3.8-3.8l-2 2-2-2 2-2Z"/></svg>
        <span>Fix reference</span>
      </button>
    </div>
  `;
}

export function renderTokenEconomicsHtml(props: TokenEconomicsProps): string {
  const { summary, topNodes } = props;
  const activeTokens = summary.activeHarnessTokens || 0;
  const reachableTokens = summary.subcallAccessibleTokens || 0;
  const critLimit = summary.criticalThreshold || 100000;
  const warnLimit = summary.warningThreshold || 60000;

  const budgetPct = Math.min(100, Math.round((activeTokens / critLimit) * 100));
  let barColor = '#3b82f6';
  let badgeClass = 'status-normal';
  if (activeTokens >= critLimit) {
    barColor = '#ef4444';
    badgeClass = 'status-critical';
  } else if (activeTokens >= warnLimit) {
    barColor = '#f59e0b';
    badgeClass = 'status-warning';
  }

  const topConsumersHtml = topNodes.length > 0
    ? topNodes.map((n) => `
      <li class="insp-consumer-item" data-path="${n.filePath}">
        <span class="insp-consumer-label" title="${n.filePath}">${n.label}</span>
        <span class="insp-consumer-val">${formatTokens(n.tokens)} tok</span>
      </li>`).join('')
    : '<li class="insp-empty-item">No notes indexed yet</li>';

  return `
    <div class="insp-header">
      <div class="insp-title-box">
        <span class="insp-icon">
          <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor" color="#38bdf8">
            <path d="M7 1L1 9h5l-1 6 7-8h-5l1-6z"/>
          </svg>
        </span>
        <div class="insp-header-titles">
          <div class="insp-title">Token Economics</div>
          <div class="insp-path">Active Prompt vs Reachable Context</div>
        </div>
      </div>
      <button class="insp-close-btn" id="insp-btn-close" title="Close Inspector">✕</button>
    </div>

    <div class="insp-body">
      <div class="insp-section">
        <div class="insp-section-header">BUDGET ALLOCATION</div>
        <div class="insp-budget-summary">
          <div class="insp-budget-row">
            <span class="insp-b-lbl">Active Prompt Load (L0):</span>
            <span class="insp-b-val">${formatTokens(activeTokens)} tok</span>
          </div>
          <div class="token-progress-track">
            <div class="token-progress-fill" style="width: ${budgetPct}%; background-color: ${barColor};"></div>
          </div>
          <div class="insp-budget-limits">
            <span>Budget used: ${budgetPct}%</span>
            <span class="insp-badge ${badgeClass}">${(summary.status || 'NORMAL').toUpperCase()}</span>
          </div>
        </div>
      </div>

      <div class="insp-section">
        <div class="insp-section-header">REACHABLE CONTEXT (L1-L2)</div>
        <div class="insp-reachable-box">
          <div class="insp-budget-row">
            <span class="insp-b-lbl">On-demand Subcall Context:</span>
            <span class="insp-b-val">${formatTokens(reachableTokens)} tok</span>
          </div>
          <div class="insp-advice-text">
            Reachable notes are within 2 hops of the active hub. They can be fetched on demand by LLM tools without burdening the permanent prompt context.
          </div>
        </div>
      </div>

      <div class="insp-section">
        <div class="insp-section-header">TOP TOKEN CONSUMERS</div>
        <ul class="insp-consumers-list">${topConsumersHtml}</ul>
      </div>
    </div>

    <div class="insp-footer">
      <div class="insp-limits-note">
        Warning: ${formatTokens(warnLimit)} tok · Critical: ${formatTokens(critLimit)} tok
      </div>
    </div>
  `;
}

export function renderInspectorDrawer(
  container: HTMLElement,
  callbacks: InspectorCallbacks,
): InspectorDrawerController {
  container.innerHTML = `
    <div id="inspector-drawer" class="inspector-drawer" style="display: none;">
      <div id="inspector-drawer-content" class="inspector-drawer-content"></div>
    </div>
  `;

  const drawer = container.querySelector('#inspector-drawer') as HTMLElement;
  const content = container.querySelector('#inspector-drawer-content') as HTMLElement;
  let visible = false;

  if (drawer) {
    drawer.addEventListener('mousedown', (e) => e.stopPropagation());
    drawer.addEventListener('mouseup', (e) => e.stopPropagation());
    drawer.addEventListener('click', (e) => e.stopPropagation());
  }

  function open(): void {
    if (drawer) {
      drawer.style.display = 'flex';
      visible = true;
    }
  }

  function close(): void {
    if (drawer) {
      drawer.style.display = 'none';
      visible = false;
      callbacks.onClose?.();
    }
  }

  function wireContentEvents(): void {
    content.querySelector('#insp-btn-close')?.addEventListener('click', (e) => {
      e.stopPropagation();
      e.preventDefault();
      close();
    });

    content.querySelectorAll('.insp-link-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const link = item.getAttribute('data-link');
        if (link) callbacks.onNavigateLink?.(link);
      });
    });

    content.querySelectorAll('.insp-consumer-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const p = item.getAttribute('data-path');
        if (p) callbacks.onOpenFile?.(p);
      });
    });
  }

  return {
    inspectNode(node: NoteNode, inLinks: string[], outLinks: string[], isHub: boolean) {
      content.innerHTML = renderNodeInspectionHtml({ node, inLinks, outLinks, isHub });
      wireContentEvents();

      content.querySelector('#insp-btn-open-file')?.addEventListener('click', (e) => {
        e.stopPropagation();
        const p = typeof node.filePath === 'string' && node.filePath.length > 0 ? node.filePath : node.id;
        callbacks.onOpenFile?.(p);
      });
      content.querySelector('#insp-btn-set-hub')?.addEventListener('click', (e) => {
        e.stopPropagation();
        callbacks.onSetHub?.(node.id);
      });
      content.querySelector('#insp-btn-exclude')?.addEventListener('click', (e) => {
        e.stopPropagation();
        const p = typeof node.filePath === 'string' && node.filePath.length > 0 ? node.filePath : node.id;
        callbacks.onExclude?.(p);
      });

      // T044: Wire fragment row clicks to navigate to line coordinates
      content.querySelectorAll('.insp-fragment-row').forEach((row) => {
        row.addEventListener('click', (e) => {
          e.stopPropagation();
          const startLineStr = row.getAttribute('data-start-line');
          const startLine = startLineStr ? parseInt(startLineStr, 10) : undefined;
          const p = typeof node.filePath === 'string' && node.filePath.length > 0 ? node.filePath : node.id;
          callbacks.onOpenSource?.(p, startLine);
        });
      });

      // Toggle fragment collapsible accordion
      const toggleBtn = content.querySelector('#insp-fragments-toggle');
      const fragmentsContent = content.querySelector('#insp-fragments-content');
      const chevron = content.querySelector('#insp-fragments-chevron');
      if (toggleBtn && fragmentsContent) {
        toggleBtn.addEventListener('click', (e) => {
          e.stopPropagation();
          const isHidden = (fragmentsContent as HTMLElement).style.display === 'none';
          (fragmentsContent as HTMLElement).style.display = isHidden ? 'block' : 'none';
          if (chevron) {
            chevron.textContent = isHidden ? '▼' : '▶';
          }
        });
      }

      open();
    },

    inspectEdge(edge: NoteEdge, sourceLabel?: string) {
      content.innerHTML = renderEdgeInspectionHtml({ edge, sourceLabel });
      wireContentEvents();

      const srcPath = typeof edge.source === 'object'
        ? (edge.source as any).filePath || (edge.source as any).id
        : edge.source;
      content.querySelector('#insp-btn-open-source')?.addEventListener('click', (e) => {
        e.stopPropagation();
        callbacks.onOpenSource?.(srcPath, edge.line ?? undefined);
      });
      content.querySelector('#insp-btn-fix-reference')?.addEventListener('click', (e) => {
        e.stopPropagation();
        callbacks.onFixReference?.(edge);
      });

      open();
    },

    inspectTokenEconomics(summary: TokenSummary, topNodes: Array<{ label: string; filePath: string; tokens: number }>) {
      content.innerHTML = renderTokenEconomicsHtml({ summary, topNodes });
      wireContentEvents();
      open();
    },

    open() {
      open();
    },

    close() {
      close();
    },

    isOpen() {
      return visible;
    },
  };
}
