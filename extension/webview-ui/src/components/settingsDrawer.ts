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
import type { KnowledgeIssue, KnowledgeIssuesReport } from '../../../src/types/admission.ts';
import type {
  DocumentationOrientation,
  Tier1Objectives,
  Tier2Parameters,
  ConstraintRule,
  ConstraintConflict,
  StructuralProblemSignature,
  StructuralRecommendation,
  ObjectiveLevel,
  ConstraintLevel,
} from '../../../src/types/evolution.ts';
import { renderEvolutionTabHtml } from './evolutionTab.ts';
import { renderOrientationSelectorHtml } from './orientationSelector.ts';
import {
  renderCustomSettingsDrawerHtml,
  TIER1_KNOB_DEFS,
  TIER2_SLIDER_DEFS,
} from './customSettingsDrawer.ts';
import {
  discoverVaultSuggestions,
  countMatchingNotes,
  DEFAULT_PALETTE,
  CLUSTER_PALETTE_SWATCHES,
} from './colorManager.ts';

export const DEFAULT_TIER1_OBJECTIVES: Tier1Objectives = {
  aiDiscoverability: 'high',
  contextEfficiency: 'high',
  humanReadability: 'medium',
  knowledgeReuse: 'medium',
  narrativeCoherence: 'low',
  maintenanceSimplicity: 'medium',
};

export const DEFAULT_TIER2_PARAMETERS: Tier2Parameters = {
  cohesionThreshold: 0.7,
  semanticDiversityThreshold: 0.75,
  referenceConcentrationThreshold: 0.7,
  fragmentSizeTarget: 3000,
  maxTraceDepth: 3,
  candidateExpansionFactor: 3,
  maxContextSurfaceTokens: 12000,
  splitSensitivity: 0.7,
  canonicalOwnershipSensitivity: 0.8,
  duplicateTolerance: 0.1,
  fragmentationTolerance: 0.2,
};

export type SettingsTab = 'knowledge' | 'exclusions' | 'clusters' | 'git_sync' | 'diagnostics' | 'evolution';

export interface SettingsDrawerProps {
  activeTab: SettingsTab;
  primaryHub?: string | null;
  brokenLinksCount?: number;
  scanExclusions: string[];
  graphExclusions: string[];
  colorGroups: ClusterColorGroup[];
  nodes?: NoteNode[];
  gitHooksInstalled?: boolean;
  preCommitHookInstalled?: boolean;
  postCommitHookInstalled?: boolean;
  gitCommitLatencyMs?: number;
  blockOnBrokenLinks?: boolean;
  blockOnDuplicateCollision?: boolean;
  blockOnTechnicalDrift?: boolean;
  baselineCommit?: string;
  baselineDebtCount?: number;
  totalNotes?: number;
  connectedNotes?: number;
  orphanNotes?: number;
  excludedNotes?: number;
  issuesReport?: KnowledgeIssuesReport | null;
  selectedPillarFilter?: string;
  structuralSignatures?: StructuralProblemSignature[];
  structuralRecommendations?: StructuralRecommendation[];
  activeOrientation?: DocumentationOrientation;
  tier1?: Tier1Objectives;
  tier2?: Tier2Parameters;
  constraints?: ConstraintRule[];
  conflicts?: ConstraintConflict[];
  isCustomDrawerOpen?: boolean;
  isSavingCustom?: boolean;
  isEvolutionLoading?: boolean;
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
  onCaptureBaseline?: () => void;
  onToggleThreshold?: (key: string, enabled: boolean) => void;
  onOpenFile?: (filePath: string, line?: number) => void;
  onFixBrokenLink?: (source: string, target: string, line?: number) => void;
  onRequestIssues?: (pillar?: string) => void;
  onRequestEvolution?: () => void;
  onSelectOrientation?: (orientation: DocumentationOrientation) => void;
  onToggleCustomDrawer?: (isOpen: boolean) => void;
  onChangeTier1?: (tier1: Tier1Objectives) => void;
  onChangeTier2?: (tier2: Tier2Parameters) => void;
  onSaveCustomConfig?: (tier1: Tier1Objectives, tier2: Tier2Parameters, constraints: ConstraintRule[]) => void;
  onResetCustomConfig?: () => void;
  onValidateConstraints?: (constraints: ConstraintRule[]) => void;
  onGeneratePlan?: (recommendationId: string) => void;
  onReviewPlan?: (planId: string) => void;
  onApplyPlan?: (planId: string) => void;
  onClose?: () => void;
}

export interface SettingsDrawerController {
  open: (tab?: SettingsTab) => void;
  close: () => void;
  isOpen: () => boolean;
  updateProps: (newProps: Partial<SettingsDrawerProps>) => void;
}

export function renderSettingsDrawerHtml(props: SettingsDrawerProps, isEntering = false): string {
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
                <div class="cluster-color-btn-wrapper" data-idx="${idx}">
                  <button type="button" class="cluster-color-swatch-btn btn-rule-color-swatch" data-idx="${idx}" style="background-color: ${g.color};" title="Click to choose color from palette" aria-label="Change color for ${label}"></button>
                </div>
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
            <div class="cluster-color-btn-wrapper" id="add-cluster-color-wrapper">
              <button type="button" class="cluster-color-swatch-btn" id="btn-add-cluster-color" style="background-color: ${nextColor};" title="Click to choose color from palette" aria-label="Select cluster color"></button>
              <input type="hidden" id="input-cluster-color" value="${nextColor}" />
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

          <!-- Floating Palette Popover -->
          <div class="cluster-palette-popover" id="cluster-palette-popover" style="display: none;">
            <div class="palette-popover-header">
              <span class="palette-popover-title">Choose Color</span>
              <button type="button" class="palette-close-btn" id="btn-close-palette" aria-label="Close palette">✕</button>
            </div>
            <div class="palette-grid">
              ${CLUSTER_PALETTE_SWATCHES.map((c) => `
                <button type="button" class="palette-swatch-item" data-color="${c}" style="background-color: ${c};" title="${c}"></button>
              `).join('')}
            </div>
            <div class="palette-hex-row">
              <span class="palette-hex-prefix">#</span>
              <input type="text" class="palette-hex-input" id="palette-hex-input" placeholder="3B82F6" maxlength="7" />
              <button type="button" class="btn-tiny palette-hex-apply-btn" id="btn-palette-apply-hex">Set</button>
            </div>
          </div>
        </div>
      `;
      break;
    }

    case 'git_sync': {
      const preActive = props.preCommitHookInstalled ?? gitHooksInstalled;
      const postActive = props.postCommitHookInstalled ?? gitHooksInstalled;
      const bBroken = props.blockOnBrokenLinks ?? true;
      const bDup = props.blockOnDuplicateCollision ?? true;
      const bDrift = props.blockOnTechnicalDrift ?? false;
      const baseCommitStr = props.baselineCommit || 'Initial (HEAD)';
      const baseDebtNum = props.baselineDebtCount ?? 0;

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Git Hook Integration & Dual Hooks (prek)</div>
          <p class="settings-desc">
            Dual-process enforcement boundary: pre-commit validates in ephemeral RAM; post-commit incorporates git deltas into persistent knowledge graph.
          </p>

          <div class="settings-hook-card">
            <div class="settings-hook-row">
              <span class="hook-label">1. Pre-Commit Validation:</span>
              <span class="hook-status-badge ${preActive ? 'status-clean' : 'status-warn'}">
                ${preActive ? '● Active (Admission Gate)' : '○ Not Installed'}
              </span>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">2. Post-Commit Graph Sync:</span>
              <span class="hook-status-badge ${postActive ? 'status-clean' : 'status-warn'}">
                ${postActive ? '● Active (Ratchet Sync)' : '○ Not Installed'}
              </span>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">Last Hook Latency:</span>
              <span class="hook-val">${gitCommitLatencyMs} ms (&le; 500ms ceiling)</span>
            </div>
          </div>

          <div class="settings-subheading" style="margin-top: 16px;">Admission Blocking Thresholds</div>
          <p class="settings-desc">Configure which new regressions abort commits with exit code 1:</p>
          <div class="threshold-toggles-list">
            <label class="threshold-toggle-item">
              <input type="checkbox" id="toggle-block-broken" ${bBroken ? 'checked' : ''} />
              <span>Block on Broken Links / Nonexistent Anchor Targets</span>
            </label>
            <label class="threshold-toggle-item">
              <input type="checkbox" id="toggle-block-duplicate" ${bDup ? 'checked' : ''} />
              <span>Block on High Semantic Duplication (&ge; 85% overlap)</span>
            </label>
            <label class="threshold-toggle-item">
              <input type="checkbox" id="toggle-block-drift" ${bDrift ? 'checked' : ''} />
              <span>Block on Technical Drift (Code target deleted/renamed)</span>
            </label>
          </div>

          <div class="settings-subheading" style="margin-top: 16px;">Knowledge Debt Baseline</div>
          <p class="settings-desc">
            Accepted brownfield debt in <code>.context-vault/baseline.json</code>. Regressions only block if they exceed this baseline.
          </p>
          <div class="baseline-management-card">
            <div class="baseline-info-row">
              <span class="baseline-label">Baseline Commit:</span>
              <code class="baseline-commit-val">${baseCommitStr}</code>
            </div>
            <div class="baseline-info-row">
              <span class="baseline-label">Baseline Debt Count:</span>
              <span class="baseline-debt-count">${baseDebtNum} finding(s)</span>
            </div>
            <div class="baseline-actions-row">
              <button class="btn-primary" id="btn-capture-baseline">Capture New Baseline</button>
            </div>
          </div>

          <button class="btn-secondary" id="btn-reinstall-hooks" style="margin-top: 16px;">
            <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;"><path d="M2.5 8a5.5 5.5 0 1 1 1.6 3.9L1.5 14M1.5 9.5v4.5h4.5"/></svg>
            Reinstall Dual Git Hooks
          </button>
        </div>
      `;
      break;
    }

    case 'diagnostics': {
      const report = props.issuesReport;
      const issues = report?.issues ?? [];
      const overallScore = report ? Math.round(report.overall_score) : 100;

      const integFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'integrity').length;
      const integrityCount = (report?.integrity_count !== undefined && report.integrity_count >= integFilterCount)
        ? report.integrity_count
        : (integFilterCount || brokenLinksCount);

      const structFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'structure').length;
      const structureCount = (report?.structure_count !== undefined && report.structure_count >= structFilterCount)
        ? report.structure_count
        : structFilterCount;

      const consFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'consistency').length;
      const consistencyCount = (report?.consistency_count !== undefined && report.consistency_count >= consFilterCount)
        ? report.consistency_count
        : consFilterCount;

      const discFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'discoverability').length;
      const discoverabilityCount = (report?.discoverability_count !== undefined && report.discoverability_count >= discFilterCount)
        ? report.discoverability_count
        : (discFilterCount || orphanNotes);

      const freshFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'freshness').length;
      const freshnessCount = (report?.freshness_count !== undefined && report.freshness_count >= freshFilterCount)
        ? report.freshness_count
        : freshFilterCount;

      const computedTotal = integrityCount + structureCount + consistencyCount + discoverabilityCount + freshnessCount;
      const totalIssues = (report?.total_issues !== undefined && report.total_issues >= issues.length)
        ? report.total_issues
        : computedTotal;

      const currentPillar = props.selectedPillarFilter || 'all';

      const scoreClass = overallScore >= 90 ? 'diag-score-good' : overallScore >= 75 ? 'diag-score-warn' : 'diag-score-bad';

      const pillarDefs = [
        { id: 'integrity', label: 'Integrity', count: integrityCount },
        { id: 'structure', label: 'Structure', count: structureCount },
        { id: 'consistency', label: 'Consistency', count: consistencyCount },
        { id: 'discoverability', label: 'Discoverability', count: discoverabilityCount },
        { id: 'freshness', label: 'Freshness', count: freshnessCount },
      ];

      const visiblePillars = currentPillar === 'all'
        ? pillarDefs
        : pillarDefs.filter((p) => p.id === currentPillar);

      let accordionsHtml = '';
      if (issues.length === 0 && !report) {
        accordionsHtml = `
          <div class="diag-empty-box">
            <svg viewBox="0 0 16 16" width="24" height="24" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom: 8px;">
              <circle cx="8" cy="8" r="7"/>
              <line x1="8" y1="4" x2="8" y2="9"/>
              <circle cx="8" cy="12" r="0.5" fill="#38bdf8"/>
            </svg>
            <div>Loading knowledge issues from engine...</div>
          </div>
        `;
      } else if (totalIssues === 0) {
        accordionsHtml = `
          <div class="diag-empty-box" style="border-color: #22c55e; color: #22c55e;">
            <svg viewBox="0 0 16 16" width="24" height="24" fill="none" stroke="#22c55e" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom: 8px;">
              <circle cx="8" cy="8" r="7"/>
              <polyline points="5 8 7.5 10.5 11 5.5"/>
            </svg>
            <div style="font-weight: 600; font-size: 13px;">Zero Defects Detected</div>
            <div style="font-size: 11px; opacity: 0.8; margin-top: 4px;">Knowledge graph is in full architectural compliance.</div>
          </div>
        `;
      } else {
        for (const p of visiblePillars) {
          const pillarIssues = issues.filter((i) => i.pillar.toLowerCase() === p.id);
          if (currentPillar === 'all' && pillarIssues.length === 0) continue;

          const issuesHtml = pillarIssues.map((issue) => {
            const catClass = `cat-${issue.category.toLowerCase().replace(/[^a-z0-9_]/g, '')}`;
            const targetHtml = issue.target ? `<span class="diag-arrow">→</span><span class="diag-target-text" title="${issue.target}">${issue.target}</span>` : '';
            const lineStr = issue.line_number ? `:${issue.line_number}` : '';
            const fixBtnHtml = issue.category === 'broken_link'
              ? `<button type="button" class="btn-tiny btn-primary diag-fix-ref-btn" data-file="${issue.file_path}" data-target="${issue.target || ''}" data-line="${issue.line_number ?? ''}">Fix Reference</button>`
              : '';

            return `
              <div class="diag-issue-card">
                <div class="diag-issue-top">
                  <span class="diag-cat-badge ${catClass}">${issue.category}</span>
                  <span class="diag-penalty-badge">-${issue.deduction_points.toFixed(1)} pts (${issue.weight.toFixed(2)}x)</span>
                </div>
                <div class="diag-issue-loc">
                  <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="flex-shrink: 0; opacity: 0.7;">
                    <path d="M9 1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V5L9 1z"/>
                    <polyline points="9 1 9 5 13 5"/>
                  </svg>
                  <span class="diag-file-clickable" data-file="${issue.file_path}" data-line="${issue.line_number ?? ''}" title="Click to open ${issue.file_path}${lineStr}">
                    ${issue.file_path}${lineStr}
                  </span>
                  ${targetHtml}
                </div>
                <div class="diag-issue-desc">${issue.description}</div>
                <div class="diag-issue-hint">💡 ${issue.remediation_hint}</div>
                <div class="diag-issue-actions">
                  <button type="button" class="btn-tiny diag-open-doc-btn" data-file="${issue.file_path}" data-line="${issue.line_number ?? ''}">Open Note</button>
                  ${fixBtnHtml}
                </div>
              </div>
            `;
          }).join('');

          accordionsHtml += `
            <div class="diag-accordion-group">
              <div class="diag-accordion-header" data-pillar="${p.id}">
                <div class="diag-accordion-title">
                  <span>${p.label.toUpperCase()}</span>
                  <span class="diag-count-badge">${pillarIssues.length}</span>
                </div>
                <span class="diag-accordion-chevron">
                  <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="4 6 8 10 12 6"></polyline>
                  </svg>
                </span>
              </div>
              <div class="diag-accordion-body" id="diag-body-${p.id}">
                ${issuesHtml || '<div style="color:var(--vscode-descriptionForeground); font-size:11px; padding:4px;">No issues in this pillar.</div>'}
              </div>
            </div>
          `;
        }
      }

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Knowledge Quality Diagnostics</div>
          <p class="settings-desc">
            Itemized findings detected from last sync across all 5 quality pillars. Inspect issues one-by-one and resolve them in editor.
          </p>

          <div class="diag-overview-card">
            <div class="diag-overview-header">
              <span class="diag-overview-title">Quality & Defect Overview</span>
              <span class="diag-score-badge ${scoreClass}">${overallScore}/100</span>
            </div>
            <div class="diag-pillar-chips">
              <div class="diag-pillar-chip" title="Integrity defects (broken links)">
                <span class="diag-chip-val" style="color: ${integrityCount > 0 ? '#ef4444' : 'inherit'};">${integrityCount}</span>
                <span class="diag-chip-lbl">Integrity</span>
              </div>
              <div class="diag-pillar-chip" title="Structure defects (oversized docs/missing metadata)">
                <span class="diag-chip-val" style="color: ${structureCount > 0 ? '#38bdf8' : 'inherit'};">${structureCount}</span>
                <span class="diag-chip-lbl">Structure</span>
              </div>
              <div class="diag-pillar-chip" title="Consistency defects (collisions)">
                <span class="diag-chip-val" style="color: ${consistencyCount > 0 ? '#f472b6' : 'inherit'};">${consistencyCount}</span>
                <span class="diag-chip-lbl">Consistency</span>
              </div>
              <div class="diag-pillar-chip" title="Discoverability defects (orphans/high hops)">
                <span class="diag-chip-val" style="color: ${discoverabilityCount > 0 ? '#eab308' : 'inherit'};">${discoverabilityCount}</span>
                <span class="diag-chip-lbl">Discovery</span>
              </div>
              <div class="diag-pillar-chip" title="Freshness defects (technical drift)">
                <span class="diag-chip-val" style="color: ${freshnessCount > 0 ? '#c084fc' : 'inherit'};">${freshnessCount}</span>
                <span class="diag-chip-lbl">Freshness</span>
              </div>
            </div>

            <div class="diag-filter-bar">
              <button type="button" class="diag-filter-btn ${currentPillar === 'all' ? 'active' : ''}" data-filter="all">
                All (${totalIssues})
              </button>
              <button type="button" class="diag-filter-btn ${currentPillar === 'integrity' ? 'active' : ''}" data-filter="integrity">
                Integrity (${integrityCount})
              </button>
              <button type="button" class="diag-filter-btn ${currentPillar === 'structure' ? 'active' : ''}" data-filter="structure">
                Structure (${structureCount})
              </button>
              <button type="button" class="diag-filter-btn ${currentPillar === 'consistency' ? 'active' : ''}" data-filter="consistency">
                Consistency (${consistencyCount})
              </button>
              <button type="button" class="diag-filter-btn ${currentPillar === 'discoverability' ? 'active' : ''}" data-filter="discoverability">
                Discovery (${discoverabilityCount})
              </button>
              <button type="button" class="diag-filter-btn ${currentPillar === 'freshness' ? 'active' : ''}" data-filter="freshness">
                Freshness (${freshnessCount})
              </button>
            </div>
          </div>

          <div class="diag-issues-list">
            ${accordionsHtml}
          </div>
        </div>

        <div class="settings-section" style="margin-top: 12px;">
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

    case 'evolution': {
      try {
        // Extract structure debt findings from issuesReport (Pillar 2: Structure)
        const structureFindings = (props.issuesReport?.issues || [])
          .filter((i) => i.pillar && i.pillar.toLowerCase() === 'structure')
          .map((i) => ({
            file_path: i.file_path,
            description: i.description,
            remediation_hint: i.remediation_hint,
          }));

        const activeOrientation = props.activeOrientation || 'ai_oriented';
        const isCustomOpen = props.isCustomDrawerOpen !== undefined
          ? props.isCustomDrawerOpen
          : activeOrientation === 'custom';

        const orientationHtml = renderOrientationSelectorHtml({
          activeOrientation,
        });

        const customToggleBtnHtml = `
          <div class="evolution-custom-toggle-bar">
            <button type="button" class="btn btn-secondary evolution-custom-toggle-btn" id="btn-toggle-custom-params">
              ${isCustomOpen ? '▲ Hide Custom Parameters (Tier 1 & Tier 2)' : '⚙️ Custom Parameters (Tier 1 & Tier 2)'}
            </button>
          </div>
        `;

        let customDrawerHtml = '';
        if (isCustomOpen) {
          customDrawerHtml = renderCustomSettingsDrawerHtml({
            tier1: props.tier1 || DEFAULT_TIER1_OBJECTIVES,
            tier2: props.tier2 || DEFAULT_TIER2_PARAMETERS,
            constraints: props.constraints || [],
            conflicts: props.conflicts || [],
            isSaving: props.isSavingCustom,
          });
        }

        const evolutionTabHtml = renderEvolutionTabHtml({
          signatures: props.structuralSignatures || [],
          recommendations: props.structuralRecommendations || [],
          activeOrientation,
          structureFindings,
          isLoading: props.isEvolutionLoading,
        });

        tabContentHtml = `
          <div class="evolution-settings-wrapper">
            ${orientationHtml}
            ${customToggleBtnHtml}
            ${customDrawerHtml}
            <div class="evolution-section-divider"></div>
            ${evolutionTabHtml}
          </div>
        `;
      } catch (err) {
        console.error('[SettingsDrawer] Error rendering evolution tab:', err);
        tabContentHtml = `
          <div class="evolution-settings-wrapper">
            <div class="evolution-empty-state">
              <span class="empty-icon">⚠️</span>
              <h3>Error Rendering Structural Evolution</h3>
              <p>${String(err)}</p>
            </div>
          </div>
        `;
      }
      break;
    }
  }

  return `
    <div class="settings-drawer${isEntering ? ' drawer-entering' : ''}" id="settings-drawer" style="display: none;">
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
        <button class="settings-tab-btn ${activeTab === 'evolution' ? 'active' : ''}" data-tab="evolution">Structural Evolution</button>
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
  let isEntering = false;
  let enterTimer: any = null;

  let _lastRenderedTab: SettingsTab | null = null;
  let _pendingOrientation: string | null = null;   // optimistic lock while awaiting IPC confirm
  let _lastKnownScrollTop = 0;                     // updated by scroll listener; used in rAF restore
  let _renderPending = false;                      // debounce: only one rAF render in flight

  function scheduleRender(): void {
    if (_renderPending) return;
    _renderPending = true;
    requestAnimationFrame(() => {
      _renderPending = false;
      render();
    });
  }

  function render(): void {
    // Only preserve scroll when staying on the same tab
    const isTabChange = _lastRenderedTab !== state.activeTab;
    const savedScrollTop = isTabChange ? 0 : _lastKnownScrollTop;

    container.innerHTML = renderSettingsDrawerHtml(state, isEntering);
    _lastRenderedTab = state.activeTab;

    const drawer = container.querySelector('#settings-drawer') as HTMLElement;
    if (drawer) {
      drawer.style.display = isVisible ? 'flex' : 'none';
      drawer.addEventListener('mousedown', (e) => e.stopPropagation());
      drawer.addEventListener('mouseup', (e) => e.stopPropagation());
      drawer.addEventListener('click', (e) => e.stopPropagation());
    }

    // Track scroll position via event so we always have the real user position
    const newContentEl = container.querySelector('#settings-content') as HTMLElement | null;
    if (newContentEl) {
      if (savedScrollTop > 0) {
        // Restore after layout via rAF
        requestAnimationFrame(() => {
          const el = container.querySelector('#settings-content') as HTMLElement | null;
          if (el) {
            el.scrollTop = savedScrollTop;
          }
        });
      }
      newContentEl.addEventListener('scroll', () => {
        _lastKnownScrollTop = newContentEl.scrollTop;
      }, { passive: true });
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
          if (tab === 'diagnostics') {
            callbacks.onRequestIssues?.(state.selectedPillarFilter);
          }
          if (tab === 'evolution') {
            if (!state.issuesReport) {
              callbacks.onRequestIssues?.();
            }
            callbacks.onRequestEvolution?.();
          }
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

    container.querySelector('#btn-capture-baseline')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onCaptureBaseline?.();
    });

    const toggleBroken = container.querySelector('#toggle-block-broken') as HTMLInputElement | null;
    toggleBroken?.addEventListener('change', (e) => {
      e.stopPropagation();
      state.blockOnBrokenLinks = toggleBroken.checked;
      callbacks.onToggleThreshold?.('blockOnBrokenLinks', toggleBroken.checked);
    });

    const toggleDup = container.querySelector('#toggle-block-duplicate') as HTMLInputElement | null;
    toggleDup?.addEventListener('change', (e) => {
      e.stopPropagation();
      state.blockOnDuplicateCollision = toggleDup.checked;
      callbacks.onToggleThreshold?.('blockOnDuplicateCollision', toggleDup.checked);
    });

    const toggleDrift = container.querySelector('#toggle-block-drift') as HTMLInputElement | null;
    toggleDrift?.addEventListener('change', (e) => {
      e.stopPropagation();
      state.blockOnTechnicalDrift = toggleDrift.checked;
      callbacks.onToggleThreshold?.('blockOnTechnicalDrift', toggleDrift.checked);
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

    // Color Palette Popover Logic
    const palettePopover = container.querySelector('#cluster-palette-popover') as HTMLElement;
    const paletteHexInput = container.querySelector('#palette-hex-input') as HTMLInputElement;
    const btnPaletteApplyHex = container.querySelector('#btn-palette-apply-hex');
    const btnClosePalette = container.querySelector('#btn-close-palette');

    let paletteTargetType: 'add' | 'rule' | null = null;
    let paletteTargetIdx: number = -1;

    const closePalette = () => {
      if (palettePopover) {
        palettePopover.style.display = 'none';
        paletteTargetType = null;
        paletteTargetIdx = -1;
      }
    };

    btnClosePalette?.addEventListener('click', (e) => {
      e.stopPropagation();
      closePalette();
    });

    const openPaletteFor = (wrapper: HTMLElement, type: 'add' | 'rule', idx: number = -1) => {
      if (!palettePopover) return;
      paletteTargetType = type;
      paletteTargetIdx = idx;

      // Move popover inside this wrapper so absolute positioning anchors to it
      wrapper.appendChild(palettePopover);

      // Check vertical room
      const rect = wrapper.getBoundingClientRect();
      const spaceBelow = window.innerHeight - rect.bottom;
      if (spaceBelow < 180 && rect.top > 180) {
        palettePopover.style.top = 'auto';
        palettePopover.style.bottom = 'calc(100% + 4px)';
      } else {
        palettePopover.style.top = 'calc(100% + 4px)';
        palettePopover.style.bottom = 'auto';
      }

      // Pre-fill current color in hex input
      const currentColor = type === 'add'
        ? (container.querySelector('#input-cluster-color') as HTMLInputElement)?.value || '#3B82F6'
        : state.colorGroups[idx]?.color || '#3B82F6';
      if (paletteHexInput) {
        paletteHexInput.value = currentColor.replace(/^#/, '');
      }

      // Highlight active swatch in grid
      palettePopover.querySelectorAll('.palette-swatch-item').forEach((swatch) => {
        const c = swatch.getAttribute('data-color');
        if (c?.toLowerCase() === currentColor.toLowerCase()) {
          swatch.classList.add('selected');
        } else {
          swatch.classList.remove('selected');
        }
      });

      palettePopover.style.display = 'block';
    };

    const applyChosenColor = (color: string) => {
      if (!color) return;
      const cleanHex = color.trim().replace(/^#/, '');
      if (!/^[0-9A-Fa-f]{3,6}$/.test(cleanHex)) return;
      const normalizedColor = `#${cleanHex}`;

      if (paletteTargetType === 'add') {
        const swatchBtn = container.querySelector('#btn-add-cluster-color') as HTMLElement;
        const hiddenInp = container.querySelector('#input-cluster-color') as HTMLInputElement;
        if (swatchBtn) swatchBtn.style.backgroundColor = normalizedColor;
        if (hiddenInp) hiddenInp.value = normalizedColor;
        closePalette();
      } else if (paletteTargetType === 'rule' && paletteTargetIdx >= 0 && state.colorGroups[paletteTargetIdx]) {
        state.colorGroups[paletteTargetIdx].color = normalizedColor;
        closePalette();
        callbacks.onSaveColorGroups?.(state.colorGroups);
        render();
      }
    };

    // Open palette for Add Rule color swatch
    const btnAddColor = container.querySelector('#btn-add-cluster-color');
    const addColorWrapper = container.querySelector('#add-cluster-color-wrapper') as HTMLElement;
    btnAddColor?.addEventListener('click', (e) => {
      e.stopPropagation();
      if (palettePopover && palettePopover.style.display === 'block' && paletteTargetType === 'add') {
        closePalette();
      } else if (addColorWrapper) {
        openPaletteFor(addColorWrapper, 'add');
      }
    });

    // Open palette for Configured Rules swatches
    container.querySelectorAll('.btn-rule-color-swatch').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const idx = Number((e.target as HTMLElement).getAttribute('data-idx'));
        const wrapper = (e.target as HTMLElement).closest('.cluster-color-btn-wrapper') as HTMLElement;
        if (palettePopover && palettePopover.style.display === 'block' && paletteTargetType === 'rule' && paletteTargetIdx === idx) {
          closePalette();
        } else if (wrapper && !isNaN(idx)) {
          openPaletteFor(wrapper, 'rule', idx);
        }
      });
    });

    // Swatch click inside palette
    palettePopover?.querySelectorAll('.palette-swatch-item').forEach((swatch) => {
      swatch.addEventListener('click', (e) => {
        e.stopPropagation();
        const color = (e.target as HTMLElement).getAttribute('data-color');
        if (color) applyChosenColor(color);
      });
    });

    // Hex apply button inside palette
    btnPaletteApplyHex?.addEventListener('click', (e) => {
      e.stopPropagation();
      const val = paletteHexInput?.value.trim();
      if (val) applyChosenColor(val);
    });

    paletteHexInput?.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        e.stopPropagation();
        const val = paletteHexInput?.value.trim();
        if (val) applyChosenColor(val);
      }
    });

    palettePopover?.addEventListener('click', (e) => e.stopPropagation());

    document.addEventListener('click', (e) => {
      if (palettePopover && palettePopover.style.display === 'block') {
        const target = e.target as HTMLElement;
        if (!palettePopover.contains(target) && !target.closest('.cluster-color-btn-wrapper')) {
          closePalette();
        }
      }
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

    // Diagnostics pillar filter buttons
    container.querySelectorAll('.diag-filter-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const filter = btn.getAttribute('data-filter') || 'all';
        state.selectedPillarFilter = filter;
        render();
      });
    });

    // Diagnostics pillar accordions
    container.querySelectorAll('.diag-accordion-header').forEach((header) => {
      header.addEventListener('click', (e) => {
        e.stopPropagation();
        const pillar = header.getAttribute('data-pillar');
        const body = container.querySelector(`#diag-body-${pillar}`);
        if (body) {
          const isCollapsed = header.classList.contains('collapsed');
          if (isCollapsed) {
            header.classList.remove('collapsed');
            body.classList.remove('collapsed');
          } else {
            header.classList.add('collapsed');
            body.classList.add('collapsed');
          }
        }
      });
    });

    // Diagnostics Open Note buttons & clickable file links
    container.querySelectorAll('.diag-open-doc-btn, .diag-file-clickable').forEach((el) => {
      el.addEventListener('click', (e) => {
        e.stopPropagation();
        const file = el.getAttribute('data-file');
        const lineAttr = el.getAttribute('data-line');
        const line = lineAttr ? parseInt(lineAttr, 10) : undefined;
        if (file) {
          callbacks.onOpenFile?.(file, line);
        }
      });
    });

    // Diagnostics Fix Reference buttons
    container.querySelectorAll('.diag-fix-ref-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const file = btn.getAttribute('data-file');
        const target = btn.getAttribute('data-target') || '';
        const lineAttr = btn.getAttribute('data-line');
        const line = lineAttr ? parseInt(lineAttr, 10) : undefined;
        if (file) {
          callbacks.onFixBrokenLink?.(file, target, line);
        }
      });
    });

    // --- Evolution Tab Event Wiring ---

    // 1. Orientation Cards Selection
    container.querySelectorAll('.orientation-card').forEach((card) => {
      card.addEventListener('click', (e) => {
        e.stopPropagation();
        const orient = card.getAttribute('data-orientation') as DocumentationOrientation;
        if (orient) {
          state.activeOrientation = orient;
          _pendingOrientation = orient;  // lock out IPC overwrites until confirmed
          if (orient === 'custom') {
            state.isCustomDrawerOpen = true;
          }
          callbacks.onSelectOrientation?.(orient);
          render();
        }
      });
    });

    // 2. Custom Parameters Toggle Button
    container.querySelector('#btn-toggle-custom-params')?.addEventListener('click', (e) => {
      e.stopPropagation();
      state.isCustomDrawerOpen = !state.isCustomDrawerOpen;
      callbacks.onToggleCustomDrawer?.(state.isCustomDrawerOpen);
      render();
    });

    // 3. Tier 1 Knobs (Low / Med / High)
    container.querySelectorAll('.tier1-level-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const knob = btn.getAttribute('data-knob') as keyof Tier1Objectives;
        const level = btn.getAttribute('data-level') as ObjectiveLevel;
        if (knob && level) {
          state.tier1 = { ...(state.tier1 || DEFAULT_TIER1_OBJECTIVES), [knob]: level };
          callbacks.onChangeTier1?.(state.tier1);
          render();
        }
      });
    });

    // 4. Tier 2 Sliders (Real-time updates)
    container.querySelectorAll('.tier2-slider-input').forEach((input) => {
      const slider = input as HTMLInputElement;
      slider.addEventListener('input', (e) => {
        e.stopPropagation();
        const param = slider.getAttribute('data-param') as keyof Tier2Parameters;
        const val = parseFloat(slider.value);
        if (param && !isNaN(val)) {
          state.tier2 = { ...(state.tier2 || DEFAULT_TIER2_PARAMETERS), [param]: val };
          const badge = container.querySelector(`#val-${param}`);
          const def = TIER2_SLIDER_DEFS.find((d) => d.key === param);
          if (badge) {
            badge.textContent = `${val}${def?.unit ? ' ' + def.unit : ''}`;
          }
          callbacks.onChangeTier2?.(state.tier2);
        }
      });
    });

    // 5. Reset Custom Parameters
    container.querySelector('#btn-reset-custom')?.addEventListener('click', (e) => {
      e.stopPropagation();
      state.tier1 = { ...DEFAULT_TIER1_OBJECTIVES };
      state.tier2 = { ...DEFAULT_TIER2_PARAMETERS };
      state.constraints = [];
      state.conflicts = [];
      callbacks.onResetCustomConfig?.();
      render();
    });

    // 6. Save & Compile Custom Profile
    container.querySelector('#btn-save-custom')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onSaveCustomConfig?.(
        state.tier1 || DEFAULT_TIER1_OBJECTIVES,
        state.tier2 || DEFAULT_TIER2_PARAMETERS,
        state.constraints || [],
      );
    });

    // 7. Hard & Soft Constraints Level Select
    container.querySelectorAll('.rule-level-select').forEach((sel) => {
      sel.addEventListener('change', (e) => {
        e.stopPropagation();
        const ruleId = sel.getAttribute('data-rule-id');
        const level = (sel as HTMLSelectElement).value as ConstraintLevel;
        if (ruleId && state.constraints) {
          const rule = state.constraints.find((r) => r.id === ruleId);
          if (rule) {
            rule.level = level;
            callbacks.onValidateConstraints?.(state.constraints);
          }
        }
      });
    });

    // 8. Transformation Plan Actions (Generate / Review / Apply)
    container.querySelectorAll('[data-action="generate-plan"]').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const recId = btn.getAttribute('data-rec-id');
        if (recId) callbacks.onGeneratePlan?.(recId);
      });
    });

    container.querySelectorAll('[data-action="review-plan"]').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const planId = btn.getAttribute('data-plan-id');
        if (planId) callbacks.onReviewPlan?.(planId);
      });
    });

    container.querySelectorAll('[data-action="apply-plan"]').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const planId = btn.getAttribute('data-plan-id');
        if (planId) callbacks.onApplyPlan?.(planId);
      });
    });

    // 9. Finding File Link Navigation in Structural Debt Findings
    container.querySelectorAll('.finding-loc').forEach((el) => {
      el.addEventListener('click', (e) => {
        e.stopPropagation();
        const pathSpan = el.querySelector('.finding-path');
        const file = pathSpan?.textContent?.trim();
        if (file) {
          callbacks.onOpenFile?.(file);
        }
      });
    });
  }

  function open(tab?: SettingsTab): void {
    const wasVisible = isVisible;
    const prevTab = state.activeTab;
    if (tab) state.activeTab = tab;
    isVisible = true;

    // Skip full re-render if already visible on the same tab (deduplicate rapid open calls)
    if (wasVisible && state.activeTab === prevTab) {
      return;
    }

    // Reset scroll when switching tabs
    if (state.activeTab !== prevTab) {
      _lastKnownScrollTop = 0;
    }

    if (!wasVisible) {
      isEntering = true;
      if (enterTimer) {
        clearTimeout(enterTimer);
      }
      enterTimer = setTimeout(() => {
        isEntering = false;
        const drawer = container.querySelector('#settings-drawer');
        drawer?.classList.remove('drawer-entering');
      }, 250);
    }

    if (state.activeTab === 'diagnostics' || state.activeTab === 'evolution') {
      if (!state.issuesReport) {
        callbacks.onRequestIssues?.(state.selectedPillarFilter);
      }
      if (state.activeTab === 'evolution') {
        callbacks.onRequestEvolution?.();
      }
    }
    render();
  }

  function close(): void {
    isVisible = false;
    isEntering = false;
    _lastKnownScrollTop = 0;
    if (enterTimer) {
      clearTimeout(enterTimer);
      enterTimer = null;
    }
    const drawer = container.querySelector('#settings-drawer') as HTMLElement;
    if (drawer) {
      drawer.style.display = 'none';
      drawer.classList.remove('drawer-entering');
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
      // Pending orientation lock: if user just clicked an orientation card,
      // don't let an in-flight IPC response overwrite it until the sidecar
      // echoes back the same orientation (confirming the change was persisted).
      if (_pendingOrientation && 'activeOrientation' in newProps) {
        if (newProps.activeOrientation === _pendingOrientation) {
          _pendingOrientation = null; // IPC confirmed — release the lock
        } else {
          // IPC returned stale state — keep our local selection
          const { activeOrientation: _ignored, ...rest } = newProps as any;
          newProps = rest;
        }
      }
      state = { ...state, ...newProps };
      // Batch rapid updates (IPC often fires multiple messages in quick succession)
      scheduleRender();
    },
  };
}
