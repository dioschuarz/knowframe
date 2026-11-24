/**
 * Settings Drawer Component
 *
 * Implements Constitution Principle VI (Progressive Disclosure).
 * Houses administrative tasks in a slide-over drawer accessed via toolbar:
 * - Knowledge (Hub configuration, broken links, vault rescan)
 * - Ignoreds (Two-tier rule manager, test preview, add/delete rules)
 * - Clusters (Color clustering rules by path or tag)
 * - Harness (prek git hooks status, latency, reinstall, LLM-Wiki skill)
 * - Issues (Node counts, protocol metrics, cache stats)
 */

import type { ClusterColorGroup, NoteNode, TokenSummary } from '../../../src/types/graph.ts';
import type { KnowledgeIssue, KnowledgeIssuesReport } from '../../../src/types/admission.ts';
import { isSameFilePath } from '../utils/pathUtils.ts';
import type {
  HarnessEntryPoint,
  AIOrientedCapabilityMatrix,
  SkillDefinition,
  ProcedureCandidate,
  DirectoryStructureMetrics,
  HarnessToolStatus,
} from '../../../src/types/governance.ts';

export const FALLBACK_HARNESS_TOOLS: HarnessToolStatus[] = [
  {
    id: 'prek',
    name: 'prek (Rust Git Hook Orchestrator)',
    category: 'core',
    binary: 'prek',
    description: 'Rust-native Git hook orchestrator enforcing sub-500ms pre-commit admission control & post-commit sync.',
    is_installed: false,
    install_command: 'cargo install prek',
    required_for: 'Dual Git Hooks (Tier 1)',
  },
  {
    id: 'uv',
    name: 'uv (Fast Python Environment & Tools)',
    category: 'runtime',
    binary: 'uv',
    description: 'Fast Python tool & virtualenv manager mandated by Mandatory Rule 10 for script execution and transcripts.',
    is_installed: false,
    install_command: 'curl -LsSf https://astral.sh/uv/install.sh | sh',
    required_for: 'Python Skills & Scripts (Rule 10)',
  },
  {
    id: 'ai_memory',
    name: 'ai-memory (Cross-Session Memory CLI)',
    category: 'core',
    binary: 'ai-memory',
    description: 'Cross-session long-term memory engine and handoffs manager mandated in root AGENTS.md.',
    is_installed: false,
    install_command: 'cargo install ai-memory',
    required_for: 'Persistent Agent Memory',
  },
  {
    id: 'notebooklm',
    name: 'notebooklm (NotebookLM CLI & Research)',
    category: 'core',
    binary: 'notebooklm',
    description: 'Zero-token grounded RAG research engine for gap closing and technical video studies.',
    is_installed: false,
    install_command: 'uv tool install notebooklm-py',
    required_for: 'Grounded RAG & Gap Closing',
  },
  {
    id: 'claude',
    name: 'Claude Code (Anthropic CLI Agent)',
    category: 'agent',
    binary: 'claude',
    description: 'Anthropic agentic CLI for code modifications, subagents, and CLAUDE.md execution.',
    is_installed: false,
    install_command: 'npm install -g @anthropic-ai/claude-code',
    required_for: 'CLAUDE.md Agent Harness',
  },
  {
    id: 'agy',
    name: 'Antigravity CLI (Google DeepMind)',
    category: 'agent',
    binary: 'agy',
    description: 'Antigravity pair-programming agent CLI and skills runner for GEMINI.md.',
    is_installed: false,
    install_command: 'uv tool install antigravity-cli',
    required_for: 'GEMINI.md Agent Harness',
  },
  {
    id: 'codex',
    name: 'Codex CLI (OpenAI)',
    category: 'agent',
    binary: 'codex',
    description: 'OpenAI Codex CLI agent executing instructions in AGENTS.md.',
    is_installed: false,
    install_command: 'npm install -g @openai/codex',
    required_for: 'AGENTS.md Agent Harness',
  },
  {
    id: 'cursor',
    name: 'Cursor CLI',
    category: 'agent',
    binary: 'cursor',
    description: 'Cursor AI code editor CLI and agent rule evaluator (.cursor/rules).',
    is_installed: false,
    install_command: 'curl -fsSL https://cursor.com -o cursor.AppImage',
    required_for: 'Cursor Rules Harness',
  },
  {
    id: 'gh',
    name: 'GitHub CLI & Copilot (gh)',
    category: 'agent',
    binary: 'gh',
    description: 'GitHub CLI with Copilot extension for command assistance & PR reviews.',
    is_installed: false,
    install_command: 'gh extension install github/gh-copilot',
    required_for: 'GitHub Copilot Harness',
  },
  {
    id: 'graphify',
    name: 'graphify (Code Architecture Graph Extractor)',
    category: 'core',
    binary: 'graphify',
    description: 'Optional offline diagnostic tool for observed code relations and AST dependency extraction. Passively supplements KnowFrame without mutating canonical markdown.',
    is_installed: false,
    install_command: 'uv tool install graphifyy',
    required_for: 'Optional Code Diagnostics',
  },
];
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
import {
  renderEvolutionTabHtml,
  renderEvolutionMethodologyCatalogHtml,
  KOS_18_METHODOLOGIES,
  inferFindingKos,
  getKosMethodologyItem,
  formatPillarName,
} from './evolutionTab.ts';
import { wireInfoBubbleListeners } from './infoBubble.ts';
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
import { formatCompactNumber } from '../utils/numberFormat.ts';

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

export type DrawerPrimaryTab = 'insights' | 'settings';
export type InsightsSubTab = 'health' | 'structure' | 'evolution' | 'load';
export type SettingsSubTab = 'exclusions' | 'clusters' | 'git' | 'harness' | 'git_harness';
export type SettingsTab =
  | InsightsSubTab
  | SettingsSubTab
  | 'knowledge'   // backwards-compatibility alias; maps to health
  | 'diagnostics' // backwards-compatibility alias for health
  | 'git_sync'    // backwards-compatibility alias for git
  | 'advanced';   // backwards-compatibility alias for git

export interface SettingsDrawerProps {
  activeTab: SettingsTab;
  tokenSummary?: TokenSummary | null;
  primaryHub?: string | null;
  brokenLinksCount?: number;
  brokenEdges?: Array<{
    source: string;
    target: string;
    line?: number;
    anchor?: string;
    resolutionStatus?: string;
  }>;
  scanExclusions: string[];
  graphExclusions: string[];
  colorGroups: ClusterColorGroup[];
  nodes?: NoteNode[];
  gitHooksInstalled?: boolean;
  preCommitHookInstalled?: boolean;
  postCommitHookInstalled?: boolean;
  llmWikiSkillAvailable?: boolean;
  gitCommitLatencyMs?: number;
  blockOnBrokenLinks?: boolean;
  blockOnDuplicateCollision?: boolean;
  blockOnTechnicalDrift?: boolean;
  blockOnMissingFragments?: boolean;
  baselineCommit?: string;
  baselineDebtCount?: number;
  totalNotes?: number;
  connectedNotes?: number;
  orphanNotes?: number;
  excludedNotes?: number;
  issuesReport?: KnowledgeIssuesReport | null;
  selectedPillarFilter?: string;
  autoExpandIssues?: boolean;
  selectedDebtScope?: 'active' | 'legacy' | 'all';
  includeExcluded?: boolean;
  selectedRuleScope?: 'scan' | 'graph';
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
  generatingPlanRecId?: string | null;   // recId currently being planned (shows spinner)
  planGenerationError?: { recId: string; message: string } | string | null;   // inline error message to display near the button
  evolutionFindingsPage?: number;
  evolutionFindingsPageSize?: number;
  selectedKosFilter?: string;
  evolutionPillarFilter?: string;
  evolutionSignaturesPage?: number;
  evolutionSignaturesPageSize?: number;
  selectedSignatureTypeFilter?: string;
  harnessStatus?: {
    harnesses: HarnessEntryPoint[];
    active_harness: HarnessEntryPoint | null;
    capability_matrix?: AIOrientedCapabilityMatrix;
  } | null;
  skills?: SkillDefinition[];
  procedureCandidates?: ProcedureCandidate[];
  directoryMetrics?: DirectoryStructureMetrics | null;
  selectedPrepActiveHarness?: string;
  toolSelection?: {
    cursor?: boolean;
    codex?: boolean;
    agy?: boolean;
    claude?: boolean;
    copilot?: boolean;
  };
  harnessTools?: HarnessToolStatus[];
}

export interface SettingsDrawerCallbacks {
  onChangeTab?: (tab: SettingsTab) => void;
  onChangeHub?: () => void;
  onFixBrokenLinks?: () => void;
  onRescanVault?: () => void;
  onSelectActiveHarness?: (harnessFile: string) => void;
  onInitializeHarness?: (targetHarness?: string) => void;
  onConfigureHarnessBase?: (options: {
    active_harness?: string;
    enabled_harnesses: string[];
    create_folders?: string[];
    consolidate_index?: boolean;
    clean_unused?: boolean;
  }) => void;
  onSyncHarnesses?: () => void;
  onRequestSkillCatalog?: () => void;
  onAddExclusion?: (pattern: string, scope: 'scan' | 'graph') => void;
  onDeleteExclusion?: (pattern: string, scope: 'scan' | 'graph') => void;
  onRequestExclusions?: () => void;
  onTestPreview?: (pattern: string) => void;
  onSaveColorGroups?: (groups: ClusterColorGroup[]) => void;
  onInstallGitHooks?: () => void;
  onInstallLLMWikiSkill?: () => void;
  onInstallHarnessTool?: (toolId: string) => void;
  onRequestHarnessTools?: () => void;
  onRequestHarnessStatus?: () => void;
  onCaptureBaseline?: () => void;
  onToggleThreshold?: (key: string, enabled: boolean) => void;
  onOpenFile?: (filePath: string, line?: number) => void;
  onFixBrokenLink?: (source: string, target: string, line?: number) => void;
  onRequestIssues?: (pillar?: string) => void;
  onRequestIssuesWithOptions?: (options: {
    include_debt?: boolean;
    include_excluded?: boolean;
    filter_path?: string;
    pillar?: string;
    limit?: number;
  }) => void;
  onRequestEvolution?: () => void;
  onSelectOrientation?: (orientation: DocumentationOrientation) => void;
  onToggleCustomDrawer?: (isOpen: boolean) => void;
  onChangeTier1?: (tier1: Tier1Objectives) => void;
  onChangeTier2?: (tier2: Tier2Parameters) => void;
  onSaveCustomConfig?: (tier1: Tier1Objectives, tier2: Tier2Parameters, constraints: ConstraintRule[]) => void;
  onResetCustomConfig?: () => void;
  onValidateConstraints?: (constraints: ConstraintRule[]) => void;
  onGeneratePlan?: (recommendationId: string) => void;
  onGeneratePlanWithStrategies?: (opportunityId: string, selectedStrategies: string[]) => void;
  onGenerateGovernancePlan?: (strategyType: string, targetFileOrDir: string) => void;
  onReviewPlan?: (planId: string) => void;
  onApplyPlan?: (planId: string) => void;
  onMarkReviewed?: (docId: string) => void;
  onReviewWithAi?: (filePath: string, issue?: unknown) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

export interface SettingsDrawerController {
  open: (tab?: SettingsTab) => void;
  close: () => void;
  isOpen: () => boolean;
  updateProps: (newProps: Partial<SettingsDrawerProps>) => void;
}

export function normalizeTab(tab: SettingsTab): InsightsSubTab | SettingsSubTab {
  if (tab === 'diagnostics' || tab === 'knowledge') return 'health';
  if (tab === 'git_sync' || tab === 'advanced') return 'git';
  if (tab === 'git_harness') return 'harness';
  return tab as InsightsSubTab | SettingsSubTab;
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
    selectedRuleScope = 'graph',
  } = props;

  let tabContentHtml = '';
  const normalizedTab = normalizeTab(activeTab);

  switch (normalizedTab) {
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
              <button type="button" class="settings-scope-btn" id="btn-scope-toggle" aria-haspopup="listbox" aria-expanded="false" title="Exclusion tier">
                <span id="label-scope-selected">${selectedRuleScope === 'scan' ? 'Scan' : 'Graph'}</span>
                <span class="scope-dropdown-arrow">▾</span>
              </button>
              <div class="settings-scope-menu" id="menu-scope-options" style="display: none;" role="listbox">
                <div class="scope-menu-item ${selectedRuleScope === 'graph' ? 'active' : ''}" data-value="graph" role="option" aria-selected="${selectedRuleScope === 'graph'}">
                  <span class="scope-item-check" style="${selectedRuleScope === 'graph' ? '' : 'visibility: hidden;'}">✓</span>
                  <span class="scope-item-title">Graph</span>
                </div>
                <div class="scope-menu-item ${selectedRuleScope === 'scan' ? 'active' : ''}" data-value="scan" role="option" aria-selected="${selectedRuleScope === 'scan'}">
                  <span class="scope-item-check" style="${selectedRuleScope === 'scan' ? '' : 'visibility: hidden;'}">✓</span>
                  <span class="scope-item-title">Scan</span>
                </div>
              </div>
              <input type="hidden" id="select-rule-scope" value="${selectedRuleScope}" />
            </div>
            <button class="btn-primary" id="btn-add-rule">+ Add Rule</button>
          </div>

          <div class="settings-subheading">Graph Exclusions</div>
          <div class="settings-rules-list">${graphRulesHtml}</div>

          <div class="settings-subheading" style="margin-top: 14px;">Scan Exclusions</div>
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

    case 'git': {
      const preActive = props.preCommitHookInstalled ?? gitHooksInstalled;
      const postActive = props.postCommitHookInstalled ?? gitHooksInstalled;
      const bBroken = props.blockOnBrokenLinks ?? true;
      const bDup = props.blockOnDuplicateCollision ?? true;
      const bDrift = props.blockOnTechnicalDrift ?? false;
      const bMissingFrag = props.blockOnMissingFragments ?? true;
      const baseCommitStr = props.baselineCommit || 'Initial (HEAD)';
      const baseDebtNum = props.baselineDebtCount ?? 0;
      const effectiveToolsList = (props.harnessTools && props.harnessTools.length > 0)
        ? props.harnessTools
        : FALLBACK_HARNESS_TOOLS;
      const prekTool = effectiveToolsList.find(t => t.id === 'prek');
      const isPrekActive = prekTool?.is_installed ?? false;

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Git Hook Integration & Dual Hooks</div>
          <p class="settings-desc">
            Dual-process enforcement boundary: pre-commit validates in ephemeral RAM; post-commit incorporates git deltas into persistent knowledge graph.
          </p>

          <div class="settings-hook-card">
            <div class="settings-hook-row">
              <span class="hook-label">1. Pre-Commit Validation:</span>
              <span class="hook-status-badge ${preActive ? 'status-clean' : 'status-warn'}">
                ${preActive ? '● Active' : '○ Not Installed'}
              </span>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">2. Post-Commit Graph Sync:</span>
              <span class="hook-status-badge ${postActive ? 'status-clean' : 'status-warn'}">
                ${postActive ? '● Active' : '○ Not Installed'}
              </span>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">3. prek Orchestrator (Rust):</span>
              <div style="display: flex; align-items: center; gap: 8px;">
                <span class="hook-status-badge ${isPrekActive ? 'status-clean' : 'status-warn'}">
                  ${isPrekActive ? '● Active' : '○ Not Installed'}
                </span>
                ${!isPrekActive ? `
                  <button class="btn-tiny btn-install-harness-tool" data-tool-id="prek" style="padding: 1px 7px; font-size: 10px; background: rgba(56, 189, 248, 0.15); border: 1px solid rgba(56, 189, 248, 0.35); color: #38bdf8; border-radius: 3px; cursor: pointer;">
                    Install prek
                  </button>
                ` : ''}
              </div>
            </div>
            <div class="settings-hook-row">
              <span class="hook-label">Last Hook Latency:</span>
              <span class="hook-val">${gitCommitLatencyMs} ms — &le; 500ms ceiling</span>
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
            <label class="threshold-toggle-item">
              <input type="checkbox" id="toggle-block-missing-fragments" ${bMissingFrag ? 'checked' : ''} />
              <span>Block on Missing Section Fragments (Directs agent attention to sections)</span>
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

    case 'harness':
    case 'git_harness': {
      const skillAvailable = props.llmWikiSkillAvailable ?? false;
      const harnessStatus = props.harnessStatus;
      let activeHarness = harnessStatus?.active_harness;
      let harnesses = harnessStatus?.harnesses ? [...harnessStatus.harnesses] : [];

      // Resilient fallback: if backend governance status has not yet arrived or activeHarness is null,
      // inspect primaryHub or nodes in the graph for recognized harness entries (AGENTS.md, notes/AGENTS.md, CLAUDE.md, GEMINI.md).
      if (!activeHarness) {
        const hubPath = props.primaryHub || '';
        const matchingNode = props.nodes?.find(n =>
          n.id === 'AGENTS.md' || n.id === 'notes/AGENTS.md' || n.id.endsWith('AGENTS.md') ||
          n.filePath === 'AGENTS.md' || n.filePath === 'notes/AGENTS.md' || n.filePath?.endsWith('AGENTS.md') ||
          n.label?.includes('AGENTS.md') ||
          n.id === 'CLAUDE.md' || n.id === 'notes/CLAUDE.md' || n.id.endsWith('CLAUDE.md') ||
          n.filePath === 'CLAUDE.md' || n.filePath === 'notes/CLAUDE.md' || n.filePath?.endsWith('CLAUDE.md') ||
          n.id === 'GEMINI.md' || n.id === 'notes/GEMINI.md' || n.id.endsWith('GEMINI.md') ||
          n.filePath === 'GEMINI.md' || n.filePath === 'notes/GEMINI.md' || n.filePath?.endsWith('GEMINI.md') ||
          (hubPath && (n.id === hubPath || n.filePath === hubPath || n.label === hubPath))
        );
        if (matchingNode || (hubPath && (hubPath.endsWith('AGENTS.md') || hubPath.endsWith('CLAUDE.md') || hubPath.endsWith('GEMINI.md')))) {
          const filePath = matchingNode?.filePath || matchingNode?.id || hubPath || 'AGENTS.md';
          activeHarness = {
            file_path: filePath,
            harness_type: filePath.toLowerCase().includes('claude') ? 'claude_md' : filePath.toLowerCase().includes('gemini') ? 'gemini_md' : 'agents_md',
            is_primary: true,
            total_tokens: matchingNode?.tokens || 3270,
            heading_count: matchingNode?.fragments?.length || 13,
            embedded_index: null,
            secondary_layers_detected: [],
            quality_ratings: {
              integrity_score: 90,
              structure_score: 95,
              consistency_score: 90,
              discoverability_score: 85,
              freshness_score: 90,
              context_efficiency: 85,
            },
            last_modified: null,
          };
          if (harnesses.length === 0) {
            harnesses.push(activeHarness);
          }
        }
      }

      const effectiveToolsList = (props.harnessTools && props.harnessTools.length > 0)
        ? props.harnessTools
        : FALLBACK_HARNESS_TOOLS;

      const harnessToolsCardHtml = `
        <div class="harness-tools-catalog-card" id="harness-tools-catalog-card" style="margin-bottom: 16px; border: 1px solid rgba(56, 189, 248, 0.35); background: rgba(56, 189, 248, 0.03); padding: 12px; border-radius: 4px;">
          <div class="harness-header-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <div style="display: flex; align-items: center; gap: 8px;">
              <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#38bdf8" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 11.5a2.5 2.5 0 0 1-5 0V9H5.5a2.5 2.5 0 0 1 0-5H9V1.5a2.5 2.5 0 0 1 5 0v10z"/>
                <path d="M2 9h4M2 13h4"/>
              </svg>
              <span style="font-weight: 600; font-size: 13px;">AI Harness & Agent Tooling (Optional Accelerators)</span>
            </div>
            <button class="btn-tiny" id="btn-refresh-harness-tools" title="Scan installed tools" style="padding: 2px 8px; font-size: 11px; background: rgba(56, 189, 248, 0.12); border: 1px solid rgba(56, 189, 248, 0.3); color: #38bdf8; border-radius: 3px; cursor: pointer; display: flex; align-items: center; gap: 4px;">
              <svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M2.5 8a5.5 5.5 0 1 1 1.6 3.9L1.5 14M1.5 9.5v4.5h4.5"/></svg>
              Scan Tools
            </button>
          </div>
          <p class="settings-desc" style="margin-top: 4px; margin-bottom: 12px;">
            KnowFrame operates completely autonomously out of the box with zero external dependencies. The tools below are optional complementary accelerators for agent memory, dual Git hooks, and multi-agent execution if installed. None are required for KnowFrame to organize and govern your documentation.
          </p>
          <div class="harness-tools-list" style="display: flex; flex-direction: column; gap: 8px;">
            ${effectiveToolsList.map((tool) => `
              <div class="harness-tool-item-card" data-tool-id="${tool.id}" style="padding: 10px; border-radius: 4px; border: 1px solid var(--vscode-widget-border, #333333); background: rgba(0, 0, 0, 0.15);">
                <div class="harness-tool-header-row" style="display: flex; justify-content: space-between; align-items: center; gap: 10px; width: 100%;">
                  <strong style="font-size: 12px; color: var(--vscode-foreground);">${tool.name}</strong>
                  <span class="hook-status-badge ${tool.is_installed ? 'status-clean' : 'status-warn'}" style="margin-left: auto; text-align: right; white-space: nowrap; flex-shrink: 0;">
                    ${tool.is_installed ? '● Active' : '○ Not Installed'}
                  </span>
                </div>
                ${tool.required_for ? `<div style="margin-top: 4px;"><span style="font-size: 9.5px; padding: 1px 5px; border-radius: 3px; background: rgba(255,255,255,0.06); color: var(--vscode-descriptionForeground); display: inline-block;">${tool.required_for}</span></div>` : ''}
                <div style="font-size: 11px; color: var(--vscode-descriptionForeground); margin-top: 4px; line-height: 1.4;">
                  ${tool.description}
                </div>
                <div style="display: flex; justify-content: space-between; align-items: center; gap: 10px; flex-wrap: wrap; margin-top: 6px;">
                  <div style="font-size: 10.5px; display: flex; align-items: center; gap: 8px; flex-wrap: wrap;">
                    <code style="color: #38bdf8;">${tool.binary}</code>
                    ${tool.is_installed && tool.version ? `<span style="opacity: 0.85;">Version: <strong>${tool.version}</strong></span>` : ''}
                    ${tool.is_installed && tool.bin_path ? `<span style="opacity: 0.65; font-size: 10px;">(${tool.bin_path})</span>` : ''}
                    ${!tool.is_installed ? `<span style="opacity: 0.75;">Install: <code>${tool.install_command}</code></span>` : ''}
                  </div>
                  <div style="display: flex; align-items: center; gap: 6px; flex-shrink: 0; margin-left: auto;">
                    ${!tool.is_installed ? `
                      <button class="btn-primary btn-install-harness-tool" data-tool-id="${tool.id}" style="padding: 4px 10px; font-size: 11px; white-space: nowrap; border-radius: 3px; cursor: pointer;">
                        Install ${tool.binary}
                      </button>
                    ` : `
                      <button class="btn-secondary btn-install-harness-tool" data-tool-id="${tool.id}" style="padding: 3px 8px; font-size: 10.5px; white-space: nowrap; border-radius: 3px; opacity: 0.85; cursor: pointer;" title="Reinstall or update tool via terminal">
                        Reinstall
                      </button>
                    `}
                  </div>
                </div>
              </div>
            `).join('')}
          </div>
        </div>
      `;

      const hasAgents = harnesses.some(h => h.file_path === 'AGENTS.md' || h.file_path.endsWith('AGENTS.md'));
      const hasGemini = harnesses.some(h => h.file_path === 'GEMINI.md' || h.file_path.endsWith('GEMINI.md'));
      const hasClaude = harnesses.some(h => h.file_path === 'CLAUDE.md' || h.file_path.endsWith('CLAUDE.md'));
      const hasCopilot = harnesses.some(h => h.file_path === '.github/copilot-instructions.md' || h.file_path.endsWith('copilot-instructions.md'));

      const isCursorChecked = props.toolSelection?.cursor ?? (hasAgents || harnesses.length === 0);
      const isCodexChecked = props.toolSelection?.codex ?? (hasAgents || harnesses.length === 0);
      const isAgyChecked = props.toolSelection?.agy ?? (hasGemini || harnesses.length === 0);
      const isClaudeChecked = props.toolSelection?.claude ?? hasClaude;
      const isCopilotChecked = props.toolSelection?.copilot ?? hasCopilot;

      const candidateHarnesses = new Set<string>();
      if (isCursorChecked || isCodexChecked) candidateHarnesses.add(hasAgents ? harnesses.find(h => h.file_path.endsWith('AGENTS.md'))!.file_path : 'AGENTS.md');
      if (isAgyChecked) candidateHarnesses.add(hasGemini ? harnesses.find(h => h.file_path.endsWith('GEMINI.md'))!.file_path : 'GEMINI.md');
      if (isClaudeChecked) candidateHarnesses.add(hasClaude ? harnesses.find(h => h.file_path.endsWith('CLAUDE.md'))!.file_path : 'CLAUDE.md');
      if (isCopilotChecked) candidateHarnesses.add(hasCopilot ? harnesses.find(h => h.file_path.endsWith('copilot-instructions.md'))!.file_path : '.github/copilot-instructions.md');
      harnesses.forEach(h => candidateHarnesses.add(h.file_path));
      if (candidateHarnesses.size === 0) candidateHarnesses.add('AGENTS.md');

      const preferredActive = props.selectedPrepActiveHarness
        || activeHarness?.file_path
        || (candidateHarnesses.has('AGENTS.md') ? 'AGENTS.md' : Array.from(candidateHarnesses)[0]);
      const currentActive = candidateHarnesses.has(preferredActive)
        ? preferredActive
        : (candidateHarnesses.has('AGENTS.md') ? 'AGENTS.md' : Array.from(candidateHarnesses)[0]);

      const basePrepCardHtml = `
        <div class="harness-entry-card" id="harness-base-prep-card" style="margin-bottom: 16px; border: 1px solid rgba(56, 189, 248, 0.35); background: rgba(56, 189, 248, 0.03);">
          <div class="harness-header-row">
            <div style="display: flex; align-items: center; gap: 8px;">
              <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#38bdf8" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
                <rect x="2" y="3" width="12" height="10" rx="1.5"/>
                <path d="M5 8h6M8 5v6"/>
              </svg>
              <span style="font-weight: 600; font-size: 13px;">AI Tool & Base Preparation</span>
            </div>
            <span class="harness-status-pill primary" style="background: rgba(56, 189, 248, 0.15); color: #38bdf8; border: 1px solid rgba(56, 189, 248, 0.3);">Harness Manager</span>
          </div>
          <p class="settings-desc" style="margin-top: 6px; margin-bottom: 10px;">
            Select which AI harnesses and agent tooling environments are used in this project. The system provisions corresponding directories, initializes entry points with embedded <code>## Knowledge Index</code> routing, and removes unselected harness files to eliminate collision debt.
          </p>

          <div class="harness-tools-selection-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px; background: rgba(0,0,0,0.15); padding: 10px; border-radius: 4px; border: 1px solid var(--vscode-widget-border, #333333);">
            <label style="display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer;">
              <input type="checkbox" id="chk-tool-cursor" ${isCursorChecked ? 'checked' : ''} />
              <span><strong>Cursor</strong> <small style="opacity: 0.75;">· AGENTS.md + .cursor/rules/</small></span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer;">
              <input type="checkbox" id="chk-tool-codex" ${isCodexChecked ? 'checked' : ''} />
              <span><strong>Codex</strong> <small style="opacity: 0.75;">· AGENTS.md</small></span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer;">
              <input type="checkbox" id="chk-tool-agy" ${isAgyChecked ? 'checked' : ''} />
              <span><strong>Agy / Antigravity</strong> <small style="opacity: 0.75;">· GEMINI.md + .agents/</small></span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer;">
              <input type="checkbox" id="chk-tool-claude" ${isClaudeChecked ? 'checked' : ''} />
              <span><strong>Claude Code</strong> <small style="opacity: 0.75;">· CLAUDE.md</small></span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer; grid-column: 1 / -1;">
              <input type="checkbox" id="chk-tool-copilot" ${isCopilotChecked ? 'checked' : ''} />
              <span><strong>GitHub Copilot</strong> <small style="opacity: 0.75;">· .github/copilot-instructions.md</small></span>
            </label>
          </div>

          <div class="harness-prep-options" style="display: flex; flex-direction: column; gap: 6px; margin-bottom: 12px; font-size: 11.5px; opacity: 0.9;">
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
              <input type="checkbox" id="chk-prep-create-dirs" checked />
              <span>Provision directory structures: <code>.agents/skills</code>, <code>.cursor/rules</code></span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
              <input type="checkbox" id="chk-prep-consolidate-index" checked />
              <span>Consolidate pre-existing <code>index.md</code> into active harness</span>
            </label>
            <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
              <input type="checkbox" id="chk-prep-clean-unused" checked />
              <span>Clean & remove unselected harness files from workspace</span>
            </label>
          </div>

          <div style="display: flex; align-items: center; justify-content: space-between; gap: 10px; margin-top: 8px;">
            <div style="display: flex; align-items: center; gap: 8px; flex: 1;">
              <label style="font-size: 11px; white-space: nowrap; color: var(--vscode-descriptionForeground);">Primary Active:</label>
              <div class="settings-scope-dropdown" id="prep-active-dropdown">
                <button type="button" class="settings-scope-btn" id="btn-prep-active-toggle" aria-haspopup="listbox" aria-expanded="false" style="max-width: 180px; padding: 3px 8px; font-size: 11.5px;">
                  <span id="label-prep-active-selected">${currentActive}</span>
                  <span class="scope-dropdown-arrow">▾</span>
                </button>
                <div class="settings-scope-menu" id="menu-prep-active-options" style="display: none; min-width: 160px;" role="listbox">
                  ${Array.from(candidateHarnesses).map((cand) => `
                    <div class="scope-menu-item ${cand === currentActive ? 'active' : ''}" data-value="${cand}" role="option" aria-selected="${cand === currentActive}">
                      <span class="scope-item-check" style="${cand === currentActive ? '' : 'visibility: hidden;'}">✓</span>
                      <span class="scope-item-title">${cand}</span>
                    </div>
                  `).join('')}
                </div>
                <input type="hidden" id="select-prep-active-harness" value="${currentActive}" />
              </div>
            </div>
            <button class="btn-primary" id="btn-prepare-harness-base" style="padding: 4px 12px; font-size: 12px; display: flex; align-items: center; gap: 6px; white-space: nowrap;">
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <polygon points="9 1 3 9 8 9 7 15 13 7 8 7 9 1"/>
              </svg>
              Prepare & Configure Base
            </button>
          </div>
          <div id="prep-base-feedback" style="display: none; margin-top: 8px; font-size: 11px; padding: 6px 10px; border-radius: 4px;"></div>
        </div>
      `;

      let harnessCardHtml = '';
      if (activeHarness) {
        const ratings = activeHarness.quality_ratings;
        const secondaryLayers = activeHarness.secondary_layers_detected || [];
        const embeddedIdx = activeHarness.embedded_index;

        harnessCardHtml = `
          <div class="harness-entry-card" id="harness-entry-point-card">
            <div class="harness-header-row">
              <div style="display: flex; align-items: center; gap: 8px;">
                <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#38bdf8" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M2 3h12a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z"/>
                  <path d="M5 6l2 2-2 2M9 10h2"/>
                </svg>
                <span style="font-weight: 600; font-size: 13px;">AI Harness Entry Point</span>
              </div>
              <span class="harness-status-pill primary">Primary Entry</span>
            </div>
            <div style="font-size: 12px; margin-bottom: 8px; display: flex; align-items: center; gap: 8px;">
              <code style="color: #38bdf8; font-weight: 600;">${activeHarness.file_path}</code>
              <span style="color: var(--vscode-descriptionForeground);">•</span>
              <span>${activeHarness.total_tokens.toLocaleString()} tokens</span>
              <span style="color: var(--vscode-descriptionForeground);">•</span>
              <span>${activeHarness.heading_count} headings</span>
            </div>
            <div class="harness-actions-bar" style="margin-top: 8px; margin-bottom: 10px; display: flex; flex-direction: column; gap: 6px;">
              ${harnesses.length > 1 ? `
                <div style="margin-bottom: 6px;">
                  <label style="font-size: 11px; color: var(--vscode-descriptionForeground); display: block; margin-bottom: 4px;">Primary Active Harness:</label>
                  <div class="settings-scope-dropdown" id="active-harness-dropdown" style="width: 100%;">
                    <button type="button" class="settings-scope-btn" id="btn-active-harness-toggle" aria-haspopup="listbox" aria-expanded="false" style="width: 100%; justify-content: space-between; padding: 4px 8px; font-size: 12px;">
                      <span id="label-active-harness-selected">${currentActive} · ${harnesses.find(h => h.file_path === currentActive)?.harness_type || 'custom'}</span>
                      <span class="scope-dropdown-arrow">▾</span>
                    </button>
                    <div class="settings-scope-menu" id="menu-active-harness-options" style="display: none; width: 100%; min-width: 200px;" role="listbox">
                      ${harnesses.map(h => `
                        <div class="scope-menu-item ${h.file_path === currentActive ? 'active' : ''}" data-value="${h.file_path}" role="option" aria-selected="${h.file_path === currentActive}">
                          <span class="scope-item-check" style="${h.file_path === currentActive ? '' : 'visibility: hidden;'}">✓</span>
                          <span class="scope-item-title">${h.file_path} <span style="opacity: 0.65; font-size: 11px;">· ${h.harness_type}</span></span>
                        </div>
                      `).join('')}
                    </div>
                    <input type="hidden" id="select-active-harness" value="${currentActive}" />
                  </div>
                </div>
              ` : ''}
              <div style="display: flex; gap: 6px; align-items: center; flex-wrap: wrap;">
                ${['AGENTS.md', 'CLAUDE.md', 'GEMINI.md']
                  .filter(cand => !harnesses.some(h => h.file_path === cand))
                  .map(cand => `<button class="btn-tiny btn-init-specific-harness" data-harness="${cand}" style="padding: 2px 8px; font-size: 10.5px; background: rgba(56, 189, 248, 0.12); border: 1px solid rgba(56, 189, 248, 0.3); color: #38bdf8; border-radius: 4px; cursor: pointer;">+ ${cand}</button>`)
                  .join('')}
                ${harnesses.length > 1 ? `
                  <button class="btn-tiny" id="btn-sync-all-harnesses" style="padding: 2px 8px; font-size: 10.5px; background: rgba(168, 85, 247, 0.15); border: 1px solid rgba(168, 85, 247, 0.35); color: #c084fc; border-radius: 4px; cursor: pointer;" title="Review and synchronize Knowledge Index across configured AI harnesses">⚡ Review & Sync Harnesses</button>
                ` : ''}
              </div>
            </div>
            ${ratings ? `
            <div class="harness-ratings-grid">
              <div class="harness-rating-item">
                <div class="harness-rating-label">Integrity</div>
                <div class="harness-rating-value">${Math.round(ratings.integrity_score)}%</div>
              </div>
              <div class="harness-rating-item">
                <div class="harness-rating-label">Structure</div>
                <div class="harness-rating-value">${Math.round(ratings.structure_score)}%</div>
              </div>
              <div class="harness-rating-item">
                <div class="harness-rating-label">Consistency</div>
                <div class="harness-rating-value">${Math.round(ratings.consistency_score)}%</div>
              </div>
              <div class="harness-rating-item">
                <div class="harness-rating-label">Discoverability</div>
                <div class="harness-rating-value">${Math.round(ratings.discoverability_score)}%</div>
              </div>
              <div class="harness-rating-item">
                <div class="harness-rating-label">Freshness</div>
                <div class="harness-rating-value">${Math.round(ratings.freshness_score)}%</div>
              </div>
              <div class="harness-rating-item">
                <div class="harness-rating-label">Context Efficiency</div>
                <div class="harness-rating-value">${Math.round(ratings.context_efficiency)}%</div>
              </div>
            </div>
            ` : ''}
            ${embeddedIdx ? `
              <div style="font-size: 11px; color: var(--vscode-descriptionForeground); margin-top: 6px;">
                Embedded Knowledge Index: <strong>${embeddedIdx.routes.length} routes</strong> (${Math.round(embeddedIdx.domain_coverage_ratio * 100)}% domain coverage)
              </div>
            ` : `
              <div style="font-size: 11px; color: #f59e0b; margin-top: 6px; display: flex; align-items: center; justify-content: space-between; gap: 8px;">
                <span>⚠️ Missing embedded <code>## Knowledge Index</code> section in entry point</span>
                <button class="btn-tiny btn-gov-embed-index" data-harness="${activeHarness.file_path}" style="padding: 2px 8px; font-size: 10.5px; background: rgba(56, 189, 248, 0.15); border: 1px solid rgba(56, 189, 248, 0.35); color: #38bdf8; border-radius: 4px; cursor: pointer; white-space: nowrap;">Embed Index</button>
              </div>
            `}
            ${harnessStatus?.capability_matrix ? `
              <div class="harness-capability-matrix-card" id="governance-capability-scorecard" style="margin-top: 10px; padding: 10px; background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); border-radius: 4px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                  <span style="font-size: 11.5px; font-weight: 700; color: #38bdf8;">4-Capability Governance Model</span>
                  <span style="font-size: 12px; font-weight: 800; color: #38bdf8;">Overall: ${Math.round(harnessStatus.capability_matrix.overall_governance_score)}/100</span>
                </div>
                <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 6px; text-align: center;">
                  <div style="background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.07); border-radius: 4px; padding: 4px;">
                    <div style="font-size: 9px; color: var(--vscode-descriptionForeground);">Entry</div>
                    <div style="font-size: 12px; font-weight: 700; margin-top: 2px;">${Math.round(harnessStatus.capability_matrix.entry_score)}%</div>
                  </div>
                  <div style="background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.07); border-radius: 4px; padding: 4px;">
                    <div style="font-size: 9px; color: var(--vscode-descriptionForeground);">Discovery</div>
                    <div style="font-size: 12px; font-weight: 700; margin-top: 2px;">${Math.round(harnessStatus.capability_matrix.discovery_score)}%</div>
                  </div>
                  <div style="background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.07); border-radius: 4px; padding: 4px;">
                    <div style="font-size: 9px; color: var(--vscode-descriptionForeground);">Retrieval</div>
                    <div style="font-size: 12px; font-weight: 700; margin-top: 2px;">${Math.round(harnessStatus.capability_matrix.retrieval_score)}%</div>
                  </div>
                  <div style="background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.07); border-radius: 4px; padding: 4px;">
                    <div style="font-size: 9px; color: var(--vscode-descriptionForeground);">Reuse</div>
                    <div style="font-size: 12px; font-weight: 700; margin-top: 2px;">${Math.round(harnessStatus.capability_matrix.reuse_score)}%</div>
                  </div>
                </div>
              </div>
            ` : ''}
            ${secondaryLayers.map(sec => `
              <div class="secondary-layer-alert-card" style="min-width: 0; max-width: 100%; box-sizing: border-box;">
                <div class="secondary-layer-alert-title" style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 8px; min-width: 0; max-width: 100%; box-sizing: border-box;">
                  <span style="display: flex; align-items: flex-start; gap: 6px; min-width: 0; max-width: 100%; flex: 1 1 auto; word-break: break-all; overflow-wrap: anywhere;">
                    <svg viewBox="0 0 16 16" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.6" style="flex-shrink: 0; margin-top: 2px;"><path d="M8 2l6 11H2L8 2zM8 6v3M8 11h.01"/></svg>
                    <span>Secondary Routing Layer: <code style="word-break: break-all; overflow-wrap: anywhere; white-space: normal;">${sec.file_path}</code></span>
                  </span>
                  <div style="display: flex; gap: 6px; flex-wrap: wrap;">
                    <button class="btn-tiny btn-gov-eliminate-secondary" data-target="${sec.file_path}" style="padding: 4px 8px; font-size: 10px; background: rgba(239, 68, 68, 0.15); border: 1px solid rgba(239, 68, 68, 0.35); color: #f87171; border-radius: 4px; cursor: pointer; white-space: normal; word-break: break-all; overflow-wrap: anywhere; text-align: left; max-width: 100%; line-height: 1.35; display: inline-block; box-sizing: border-box;">Safely Remove ${sec.file_path}</button>
                    <button type="button" class="btn-tiny btn-secondary diag-mark-reviewed-btn" data-action="review-with-ai" data-file="${sec.file_path}" data-doc-id="${sec.file_path}" data-issue="${encodeURIComponent(JSON.stringify({ category: 'secondary_routing_layer', file_path: sec.file_path, description: sec.description, remediation_hint: sec.recommendation_hint }))}" title="Review and resolve with AI agent">Review with AI</button>
                  </div>
                </div>
                <div class="secondary-layer-alert-desc" style="word-break: break-word; overflow-wrap: anywhere; white-space: normal; min-width: 0;">
                  ${sec.description} (${sec.outgoing_routes_count} outgoing routes, +${sec.intermediate_hops} hops, -${sec.penalty_points} penalty points).
                  <br/><em>${sec.recommendation_hint}</em>
                </div>
              </div>
            `).join('')}
          </div>
        `;
      } else {
        harnessCardHtml = `
          <div class="harness-entry-card" id="harness-entry-point-card">
            <div class="harness-header-row">
              <div style="display: flex; align-items: center; gap: 8px;">
                <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#f59e0b" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M2 3h12a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z"/>
                  <path d="M5 6l2 2-2 2M9 10h2"/>
                </svg>
                <span style="font-weight: 600; font-size: 13px;">AI Harness Entry Point</span>
              </div>
              <span class="harness-status-pill" style="background: rgba(245, 158, 11, 0.2); color: #f59e0b; border: 1px solid rgba(245, 158, 11, 0.4);">Not Detected</span>
            </div>
            <p class="settings-desc" style="margin-top: 6px; margin-bottom: 12px;">
              No standard AI Harness Entry Point (<code>AGENTS.md</code>, <code>CLAUDE.md</code>, <code>GEMINI.md</code>) detected. AI agents lack a deterministic root navigation index.
            </p>
            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
              <button class="btn-primary btn-init-specific-harness" id="btn-init-harness" data-harness="AGENTS.md">
                Initialize AGENTS.md
              </button>
              <button class="btn-secondary btn-init-specific-harness" data-harness="CLAUDE.md" style="padding: 4px 10px; font-size: 12px; background: rgba(255, 255, 255, 0.08); border: 1px solid var(--vscode-dropdown-border); border-radius: 4px; cursor: pointer;">
                Initialize CLAUDE.md
              </button>
              <button class="btn-secondary btn-init-specific-harness" data-harness="GEMINI.md" style="padding: 4px 10px; font-size: 12px; background: rgba(255, 255, 255, 0.08); border: 1px solid var(--vscode-dropdown-border); border-radius: 4px; cursor: pointer;">
                Initialize GEMINI.md
              </button>
            </div>
          </div>
        `;
      }

      const skills = props.skills || [];
      const procedureCandidates = props.procedureCandidates || [];

      const skillsCatalogHtml = `
        <div class="skills-catalog-card" id="skills-catalog-card">
          <div class="skill-header-row">
            <div style="display: flex; align-items: center; gap: 8px;">
              <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#a855f7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 2v12l4-2 4 2V2z"/>
              </svg>
              <span style="font-weight: 600; font-size: 13px;">Agent Skills Catalog</span>
            </div>
            <button class="btn-tiny" id="btn-refresh-skills" title="Rescan skills">
              <svg viewBox="0 0 16 16" width="11" height="11" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M2.5 8a5.5 5.5 0 1 1 1.6 3.9L1.5 14M1.5 9.5v4.5h4.5"/></svg>
              Scan
            </button>
          </div>
          <p class="settings-desc" style="margin-top: 4px; margin-bottom: 8px;">
            Reusable operational capabilities defined in <code>.agents/skills/</code> conforming to the Agent Skill Standard.
          </p>

          ${skills.length === 0 ? `
            <div style="font-size: 11px; color: var(--vscode-descriptionForeground); font-style: italic; padding: 6px 0;">
              No packaged skills discovered in repository.
            </div>
          ` : `
            <div class="skills-list">
              ${skills.map(skill => {
                let badgeClass = 'valid';
                let badgeLabel = 'Valid';
                if (skill.status === 'broken_asset_reference') {
                  badgeClass = 'broken';
                  badgeLabel = 'Broken Asset';
                } else if (skill.status === 'missing_manifest') {
                  badgeClass = 'missing';
                  badgeLabel = 'Missing Skill.md';
                } else if (skill.status === 'duplicate_capability') {
                  badgeClass = 'duplicate';
                  badgeLabel = 'Duplicate Capability';
                }

                return `
                  <div class="skill-item-card">
                    <div class="skill-header-row">
                      <div class="skill-name">${skill.name || skill.skill_id}</div>
                      <span class="skill-badge ${badgeClass}">${badgeLabel}</span>
                    </div>
                    <div style="font-size: 11px; color: var(--vscode-descriptionForeground); margin-bottom: 6px;">
                      ${skill.description || 'No description provided.'}
                    </div>
                    <div style="font-size: 10.5px; display: flex; flex-wrap: wrap; gap: 6px; align-items: center;">
                      <code style="color: #a855f7;">${skill.dir_path}</code>
                      ${skill.unreferenced ? '<span style="color: #f59e0b; background: rgba(245, 158, 11, 0.1); padding: 1px 5px; border-radius: 4px;">Unreferenced</span>' : ''}
                      ${skill.overlap_ids && skill.overlap_ids.length > 0 ? `<span style="color: #c084fc; background: rgba(192, 132, 252, 0.1); padding: 1px 5px; border-radius: 4px;">Overlap: ${skill.overlap_ids.join(', ')}</span>` : ''}
                    </div>
                    ${skill.assets && skill.assets.length > 0 ? `
                      <div style="margin-top: 6px; padding-top: 6px; border-top: 1px solid rgba(255,255,255,0.05); font-size: 10px;">
                        ${skill.assets.map(a => `
                          <div style="display: flex; align-items: center; gap: 4px; color: ${a.exists_on_disk ? 'var(--vscode-descriptionForeground)' : '#f87171'};">
                            <span>${a.exists_on_disk ? '✓' : '✗'}</span>
                            <span>[${a.asset_type}] ${a.file_path}</span>
                            ${!a.exists_on_disk ? '<strong>· missing on disk</strong>' : ''}
                          </div>
                        `).join('')}
                      </div>
                    ` : ''}
                  </div>
                `;
              }).join('')}
            </div>
          `}

          ${procedureCandidates.length > 0 ? `
            <div style="margin-top: 14px; padding-top: 10px; border-top: 1px solid var(--vscode-widget-border, #333333);">
              <div style="font-size: 12px; font-weight: 600; color: #38bdf8; display: flex; align-items: center; gap: 6px; margin-bottom: 6px;">
                <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="8" cy="8" r="6"/><path d="M8 5v3l2 2"/></svg>
                Procedure Candidates for Skill Promotion (${procedureCandidates.length})
              </div>
              <p class="settings-desc" style="margin-bottom: 8px;">
                Repeated procedural sequences detected across multiple documents meeting the &ge; 70% recurrence threshold (FR-023).
              </p>
              ${procedureCandidates.map(cand => `
                <div class="procedure-candidate-card">
                  <div style="display: flex; justify-content: space-between; align-items: center; gap: 8px;">
                    <strong style="font-size: 12px; color: var(--vscode-editor-foreground);">${cand.recommended_skill_name}</strong>
                    <div style="display: flex; align-items: center; gap: 6px;">
                      <span style="font-size: 10px; font-weight: 600; color: #38bdf8; background: rgba(56, 189, 248, 0.15); padding: 2px 6px; border-radius: 3px;">
                        ${Math.round(cand.recurrence_score * 100)}% Recurrence · ${cand.occurrence_count} docs
                      </span>
                      <button class="btn-tiny btn-gov-promote-skill" data-target="${cand.source_documents[0] || ''}" data-skill-name="${cand.recommended_skill_name}" style="padding: 2px 8px; font-size: 10px; background: rgba(168, 85, 247, 0.15); border: 1px solid rgba(168, 85, 247, 0.35); color: #c084fc; border-radius: 4px; cursor: pointer; white-space: nowrap;">Promote to Skill</button>
                    </div>
                  </div>
                  <ol class="procedure-step-list">
                    ${cand.step_signatures.map(step => `<li>${step}</li>`).join('')}
                  </ol>
                  <div class="procedure-meta-row">
                    <span>Sources: ${cand.source_documents.map(d => `<code>${d}</code>`).join(', ')}</span>
                    ${cand.associated_scripts && cand.associated_scripts.length > 0 ? `<span>• Scripts: ${cand.associated_scripts.map(s => `<code>${s}</code>`).join(', ')}</span>` : ''}
                  </div>
                </div>
              `).join('')}
            </div>
          ` : ''}
        </div>
      `;

      tabContentHtml = `
        <div class="settings-section">
          ${harnessToolsCardHtml}
          ${basePrepCardHtml}
          ${harnessCardHtml}
          ${skillsCatalogHtml}
          <div class="settings-skill-card" style="margin-top: 0;">
            <div class="settings-subheading">KnowFrame Skill</div>
            <p class="settings-desc">Install the workspace skill that documents the complete KnowFrame CLI and its UI-equivalent workflows.</p>
            <div class="settings-hook-row" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
              <span class="hook-label">Skill availability:</span>
              <span class="hook-status-badge ${skillAvailable ? 'status-clean' : 'status-warn'}" style="margin-left: auto; text-align: right; white-space: nowrap; flex-shrink: 0;">
                ${skillAvailable ? '● Available' : '○ Not Installed'}
              </span>
            </div>
            <button class="btn-secondary" id="btn-install-llm-wiki-skill" style="margin-top: 12px;">
              ${skillAvailable ? 'Reinstall KnowFrame Skill' : 'Install KnowFrame Skill'}
            </button>
          </div>
        </div>
      `;
      break;
    }

    case 'health':
    case 'diagnostics': {
      const report = props.issuesReport;
      const rawReportIssues = report?.issues ?? [];
      const overallScore = report ? Math.round(report.overall_score) : 100;
      const rawScore = report?.raw_score !== undefined && report.raw_score !== null ? Math.round(report.raw_score) : null;
      const selectedScope = props.selectedDebtScope || 'active';
      const includeExcluded = Boolean(props.includeExcluded);

      // Synthesize missing integrity issues from brokenEdges if report lacks them
      const hasIntegrityInIssues = rawReportIssues.some((i) => i.pillar?.toLowerCase() === 'integrity');
      let allIssues = [...rawReportIssues];
      if (!hasIntegrityInIssues && props.brokenEdges && props.brokenEdges.length > 0) {
        for (const edge of props.brokenEdges) {
          allIssues.push({
            id: `edge:${edge.source}:${edge.target}:${edge.line || 0}`,
            pillar: 'integrity',
            category: edge.anchor ? 'invalid_anchor' : 'broken_link',
            file_path: edge.source,
            line_number: edge.line || null,
            target: edge.target + (edge.anchor ? `#${edge.anchor}` : ''),
            deduction_points: 5.0,
            weight: 1.0,
            description: `Broken reference to missing target '${edge.target}'${edge.anchor ? ` (anchor: ${edge.anchor})` : ''}`,
            remediation_hint: `Create target file '${edge.target}' or update link in '${edge.source}'`,
            is_legacy: false,
            is_excluded: false,
          });
        }
      }

      // Synthesize structural problem signatures if report lacks them
      if (props.structuralSignatures && props.structuralSignatures.length > 0) {
        for (const s of props.structuralSignatures) {
          const docPath = s.documentPath || (s as any).document_path;
          const problemType = s.problemType || (s as any).problem_type || 'god_document';
          const already = allIssues.some((i) => isSameFilePath(i.file_path, docPath) && i.category === problemType);
          if (!already) {
            const pillar = (problemType === 'isolated_silo' || problemType === 'implicit_router') ? 'discoverability' : 'structure';
            allIssues.push({
              id: `sig:${s.signatureId || (s as any).signature_id || docPath}`,
              pillar,
              category: problemType,
              file_path: docPath,
              line_number: null,
              target: null,
              deduction_points: 5.0,
              weight: 1.0,
              description: s.rationale || `${problemType.replace(/_/g, ' ')} detected on ${docPath}.`,
              remediation_hint: 'Review structural balance and modular boundaries in note frontmatter.',
              is_legacy: false,
              is_excluded: false,
            });
          }
        }
      }

      // Synthesize directory organizational signals into Health -> Structure
      if (props.directoryMetrics && props.directoryMetrics.organizational_signals && props.directoryMetrics.organizational_signals.length > 0) {
        for (const sig of props.directoryMetrics.organizational_signals) {
          const targetDir = sig.target_directory || props.primaryHub || 'AGENTS.md';
          const cat = sig.signal_type || 'organizational_signal';
          const already = allIssues.some((i) => i.category === cat && isSameFilePath(i.file_path, targetDir));
          if (!already) {
            allIssues.push({
              id: `org:${cat}:${targetDir}`,
              pillar: 'structure',
              category: cat,
              file_path: targetDir,
              line_number: null,
              target: null,
              deduction_points: 5.0,
              weight: 1.0,
              description: `${sig.message} (${sig.cost_evidence})`,
              remediation_hint: 'Rebalance directory depth or cross-folder navigation structure.',
              is_legacy: false,
              is_excluded: false,
            });
          }
        }
      }

      // Determine active vs legacy counts
      const activeCount = report?.active_issues_count !== undefined
        ? Math.max(report.active_issues_count, allIssues.filter((i) => !i.is_legacy).length)
        : allIssues.filter((i) => !i.is_legacy).length;
      const legacyDebtCount = report?.legacy_debt_count !== undefined
        ? report.legacy_debt_count
        : allIssues.filter((i) => !!i.is_legacy).length || (props.baselineDebtCount ?? 0);
      const allCount = activeCount + legacyDebtCount;

      // Filter issues by scope
      let issues = allIssues;
      if (selectedScope === 'active') {
        issues = allIssues.filter((i) => !i.is_legacy);
      } else if (selectedScope === 'legacy') {
        issues = allIssues.filter((i) => !!i.is_legacy);
      }

      // Compute KOS methodology counts for the current scope
      const selectedKosFilter = props.selectedKosFilter || 'all';
      const kosCounts: Record<string, number> = {};
      for (const k of KOS_18_METHODOLOGIES) {
        kosCounts[k.id] = 0;
      }
      for (const issue of issues) {
        const kosId = inferFindingKos(issue);
        kosCounts[kosId] = (kosCounts[kosId] || 0) + 1;
      }

      // Compute pillar counts for displayed scope
      const integFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'integrity').length;
      const integrityCount = report
        ? (selectedScope === 'all' && report.integrity_count !== undefined && report.integrity_count >= integFilterCount
            ? report.integrity_count
            : Math.max(integFilterCount, selectedScope !== 'legacy' ? (props.brokenLinksCount ?? 0) : 0))
        : (selectedScope !== 'legacy' ? Math.max(integFilterCount, props.brokenLinksCount ?? 0) : 0);

      const structFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'structure').length;
      const structureCount = report
        ? (selectedScope === 'all' && report.structure_count !== undefined && report.structure_count >= structFilterCount
            ? report.structure_count
            : structFilterCount)
        : 0;

      const consFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'consistency').length;
      const consistencyCount = report
        ? (selectedScope === 'all' && report.consistency_count !== undefined && report.consistency_count >= consFilterCount
            ? report.consistency_count
            : consFilterCount)
        : 0;

      const discFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'discoverability').length;
      const discoverabilityCount = report
        ? (selectedScope === 'all' && report.discoverability_count !== undefined && report.discoverability_count >= discFilterCount
            ? report.discoverability_count
            : discFilterCount)
        : (selectedScope !== 'legacy' ? orphanNotes : 0);

      const freshFilterCount = issues.filter((i) => i.pillar?.toLowerCase() === 'freshness' || i.pillar?.toLowerCase() === 'traceability').length;
      const freshnessCount = report
        ? (selectedScope === 'all' && (report.traceability_count !== undefined || report.freshness_count !== undefined)
            ? (report.traceability_count ?? report.freshness_count)
            : freshFilterCount)
        : 0;

      const selectedPillars = new Set(
        (props.selectedPillarFilter || '')
          .split(',')
          .map((pillar) => pillar.trim())
          .filter(Boolean),
      );

      let issuesForDisplay = issues;
      if (selectedKosFilter !== 'all') {
        issuesForDisplay = issuesForDisplay.filter((issue) => inferFindingKos(issue) === selectedKosFilter);
      }
      if (selectedPillars.size > 0) {
        issuesForDisplay = issuesForDisplay.filter((issue) => {
          const p = issue.pillar.toLowerCase();
          return selectedPillars.has(p) || (selectedPillars.has('traceability') && p === 'freshness') || (selectedPillars.has('freshness') && p === 'traceability');
        });
      }

      const scoreClass = overallScore >= 90 ? 'diag-score-good' : overallScore >= 75 ? 'diag-score-warn' : 'diag-score-bad';

      const pillarDefs = [
        { id: 'integrity', label: 'Integrity', count: integrityCount },
        { id: 'structure', label: 'Structure', count: structureCount },
        { id: 'consistency', label: 'Consistency', count: consistencyCount },
        { id: 'discoverability', label: 'Discoverability', count: discoverabilityCount },
        { id: 'traceability', label: 'Traceability', count: freshnessCount },
      ];

      const visiblePillars = selectedPillars.size === 0
        ? pillarDefs
        : pillarDefs.filter((p) => selectedPillars.has(p.id) || (selectedPillars.has('freshness') && p.id === 'traceability') || (selectedPillars.has('traceability') && p.id === 'freshness'));

      let accordionsHtml = '';
      if (allIssues.length === 0 && !report) {
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
      } else if (selectedScope === 'active' && activeCount === 0 && legacyDebtCount > 0) {
        const filterNames = selectedPillars.size > 0
          ? Array.from(selectedPillars).map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join(', ')
          : '';
        accordionsHtml = `
          <div class="diag-empty-box" style="border-color: #22c55e; color: #22c55e;">
            <svg viewBox="0 0 16 16" width="28" height="28" fill="none" stroke="#22c55e" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom: 8px;">
              <circle cx="8" cy="8" r="7"/>
              <polyline points="5 8 7.5 10.5 11 5.5"/>
            </svg>
            <div style="font-weight: 600; font-size: 13px; color: #22c55e;">Zero Active Knowledge Regressions${filterNames ? ` in ${filterNames}` : ''}</div>
            <div style="font-size: 11px; opacity: 0.85; margin-top: 4px; color: var(--vscode-foreground);">
              ${filterNames ? `All active documents meet quality standards for <strong>${filterNames}</strong> (0 active regressions).` : 'All active documents meet quality standards.'} ${legacyDebtCount} finding${legacyDebtCount === 1 ? '' : 's'} acknowledged in baseline.
            </div>
            <button type="button" class="btn-tiny btn-primary diag-inspect-debt-btn" data-scope="legacy" style="margin-top: 10px;">
              Inspect Baselined (${legacyDebtCount})
            </button>
          </div>
        `;
      } else if (allCount === 0) {
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
      } else if (issuesForDisplay.length === 0) {
        accordionsHtml = `
          <div class="diag-empty-box">
            <div style="font-size: 12px; color: var(--vscode-descriptionForeground);">No issues found in the selected scope or filters.</div>
          </div>
        `;
      } else {
        for (const p of visiblePillars) {
          const pillarIssues = issuesForDisplay.filter((i) => {
            const raw = i.pillar.toLowerCase();
            if (p.id === 'traceability' || p.id === 'freshness') {
              return raw === 'traceability' || raw === 'freshness';
            }
            return raw === p.id;
          });
          if (selectedPillars.size === 0 && pillarIssues.length === 0) continue;

          const issuesHtml = pillarIssues.map((issue) => {
            const catClass = `cat-${issue.category.toLowerCase().replace(/[^a-z0-9_]/g, '')}`;
            const lineStr = issue.line_number ? `:${issue.line_number}` : '';
            const isBroken = issue.category === 'broken_link' || issue.category === 'invalid_anchor' || issue.category === 'broken_fragment_anchor';
            const isNonClickable = issue.category === 'non_clickable_link';
            const isSecondary =
              issue.category === 'secondaryroutinglayer' ||
              issue.category === 'secondary_routing_layer' ||
              (issue.description && issue.description.toLowerCase().includes('secondary routing layer'));
            const isDrift =
              issue.category === 'harness_drift' ||
              issue.category === 'harnessdrift' ||
              (issue.description && (issue.description.toLowerCase().includes('harness route drift') || issue.description.toLowerCase().includes('harness drift')));
            const isReview =
              issue.category === 'review_required' ||
              issue.category === 'document_review' ||
              issue.category === 'unreviewed' ||
              (issue as any).obligation !== undefined ||
              (issue as any).review_status !== undefined;

            let cleanResolution = issue.remediation_hint || '';
            if (isDrift) {
              cleanResolution = 'Synchronize route definitions across configured AI harness files.';
            } else if (isSecondary) {
              cleanResolution = 'Consolidate routes into primary AI Harness Entry Point.';
            } else if (isBroken) {
              cleanResolution = issue.target
                ? `Update reference to valid target note '${issue.target}'.`
                : 'Fix broken link reference in document.';
            } else if (!cleanResolution) {
              cleanResolution = 'Review and resolve finding in editor.';
            }

            const targetRow = issue.target
              ? `
                <div class="diag-loc-row">
                  <span class="diag-loc-label">Target</span>
                  <span class="diag-loc-path diag-target-clickable ${isBroken ? 'target-missing' : ''}" data-target="${issue.target}" title="Open target file">${issue.target}</span>
                </div>`
              : '';

            const sparklesSvg = `
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:5px;">
                <path d="M7.5 1.5L9 5l3.5 1.5L9 8l-1.5 3.5L6 8 2.5 6.5 6 5z" fill="currentColor" fill-opacity="0.3"/>
                <path d="M12.5 10.5l.75 1.75 1.75.75-1.75.75-.75 1.75-.75-1.75-1.75-.75 1.75-.75z" fill="currentColor"/>
              </svg>
            `;

            const hasDeterministicAction = isBroken || isNonClickable || isSecondary || isDrift || isReview;
            let actionBtnHtml = '';

            if (hasDeterministicAction) {
              let primaryActionClass = 'btn-primary diag-btn-deterministic';
              let extraAttrs = '';
              if (isBroken) {
                primaryActionClass += ' diag-fix-ref-btn';
                extraAttrs = `data-file="${issue.file_path}" data-target="${issue.target || ''}" data-line="${issue.line_number ?? ''}"`;
              } else if (isNonClickable) {
                primaryActionClass += ' btn-gov-normalize-links';
                extraAttrs = `data-target="${issue.file_path}"`;
              } else if (isSecondary) {
                primaryActionClass += ' btn-gov-eliminate-secondary';
                extraAttrs = `data-target="${issue.file_path}"`;
              } else if (isDrift) {
                primaryActionClass += ' btn-gov-sync-harnesses';
                extraAttrs = `data-target="${issue.file_path}"`;
              } else if (isReview) {
                primaryActionClass += ' diag-mark-reviewed-btn';
                extraAttrs = `data-action="mark-reviewed" data-file="${issue.file_path}" data-doc-id="${issue.file_path}"`;
              }

              actionBtnHtml = `
                <div class="diag-action-split-btn">
                  <button type="button" class="btn-tiny ${primaryActionClass} diag-split-primary" ${extraAttrs} title="Apply deterministic algorithmic fix">
                    Review
                  </button>
                  <button type="button" class="btn-tiny btn-primary diag-split-toggle" title="More review options" aria-haspopup="true" aria-expanded="false">
                    ▾
                  </button>
                  <div class="diag-split-menu" style="display: none;">
                    <button type="button"
                            class="diag-split-menu-item diag-mark-reviewed-btn"
                            data-action="review-with-ai"
                            data-file="${issue.file_path}"
                            data-doc-id="${issue.file_path}"
                            data-issue="${encodeURIComponent(JSON.stringify(issue))}"
                            title="Resolve issue with AI">
                      ${sparklesSvg}
                      <span>Review with AI</span>
                    </button>
                  </div>
                </div>
              `;
            } else {
              actionBtnHtml = `
                <div class="diag-action-direct-btn">
                  <button type="button"
                          class="btn-tiny btn-primary diag-mark-reviewed-btn diag-btn-sparkles"
                          data-action="review-with-ai"
                          data-file="${issue.file_path}"
                          data-doc-id="${issue.file_path}"
                          data-issue="${encodeURIComponent(JSON.stringify(issue))}"
                          title="Resolve issue with AI">
                    ${sparklesSvg}
                    <span>Review with AI</span>
                  </button>
                </div>
              `;
            }

            const kosDiscipline = inferFindingKos(issue);
            const kosMeta = getKosMethodologyItem(kosDiscipline);

            return `
              <div class="diag-issue-card ${issue.is_legacy ? 'diag-issue-legacy' : ''}">
                <div class="diag-issue-top">
                  <div style="display: flex; align-items: center; gap: 6px; flex-wrap: wrap;">
                    ${kosMeta ? `<span class="badge-kos badge-kos-${kosMeta.id}" title="${kosMeta.discipline}: ${kosMeta.description}">${kosMeta.shortLabel}</span>` : ''}
                    <span class="badge-pillar badge-pillar-${issue.pillar?.toLowerCase()}">${formatPillarName(issue.pillar)}</span>
                    <span class="diag-cat-badge ${catClass}">${issue.category}</span>
                    ${(issue as any).review_status ? `<span class="diag-cat-badge diag-cat-review" style="background: rgba(245, 158, 11, 0.2); color: #f59e0b;">Review: ${(issue as any).review_status}</span>` : ''}
                    ${issue.is_legacy ? '<span class="diag-legacy-badge" title="Acknowledged in debt baseline">Baselined</span>' : ''}
                    ${issue.is_excluded ? '<span class="diag-excluded-badge" title="From path excluded in .context-vault.yaml">Excluded</span>' : ''}
                  </div>
                  <span class="diag-penalty-badge">-${(issue.deduction_points ?? 5.0).toFixed(1)} pts (${(issue.weight ?? 1.0).toFixed(2)}x)</span>
                </div>
                <div class="diag-issue-locations">
                  <div class="diag-loc-row">
                    <span class="diag-loc-label">Source</span>
                    <span class="diag-loc-path diag-file-clickable" data-file="${issue.file_path}" data-line="${issue.line_number ?? ''}" title="Open source file">${issue.file_path}${lineStr}</span>
                  </div>
                  ${targetRow}
                </div>
                <div class="diag-issue-desc">${issue.description}</div>
                <div class="diag-issue-resolution">
                  <span class="diag-resolution-label">Resolution</span>
                  <span class="diag-resolution-text">${cleanResolution}</span>
                </div>
                <div class="diag-issue-actions">
                  ${actionBtnHtml}
                </div>
              </div>
            `;
          }).join('');

          const isPillarExpanded = Boolean(props.autoExpandIssues);
          accordionsHtml += `
            <div class="diag-accordion-group" data-pillar="${p.id}" ${p.id === 'traceability' ? 'data-legacy-pillar="freshness"' : ''}>
              <div class="diag-accordion-header ${isPillarExpanded ? '' : 'collapsed'}" data-pillar="${p.id}">
                <div class="diag-accordion-title">
                  <span>${p.label}</span>
                  <span class="diag-count-badge" style="background: ${p.count > 0 ? (p.id === 'integrity' ? '#ef4444' : (p.id === 'traceability' || p.id === 'freshness') ? '#a855f7' : p.id === 'consistency' ? '#ec4899' : p.id === 'structure' ? '#0ea5e9' : '#eab308') : 'var(--vscode-badge-background)'}; color: #ffffff;">${pillarIssues.length}</span>
                </div>
                <span class="diag-accordion-chevron">
                  <svg viewBox="0 0 16 16" width="12" height="12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="4 6 8 10 12 6"></polyline>
                  </svg>
                </span>
              </div>
              <div class="diag-accordion-body ${isPillarExpanded ? '' : 'collapsed'}" id="diag-body-${p.id}">
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
            Itemized findings detected from last sync across all 5 quality pillars and 18 KOS disciplines. Inspect issues one-by-one and resolve them in editor.
          </p>

          <div class="diag-overview-card">
            <div class="diag-overview-header">
              <span class="diag-overview-title">Quality & Defect Overview</span>
              <div class="diag-overview-actions">
                <button type="button" class="diag-toggle-all" id="diag-toggle-all">${props.autoExpandIssues ? 'Collapse all' : 'Expand all'}</button>
                ${rawScore !== null && rawScore !== overallScore ? `<span class="diag-score-raw-label" title="Raw unmasked audit score">Audit: ${rawScore}/100</span>` : ''}
                <span class="diag-score-badge ${scoreClass}" title="Effective Score: ${overallScore}/100${rawScore !== null ? ` | Raw Historical Audit: ${rawScore}/100` : ''}">${overallScore}/100</span>
              </div>
            </div>
            <div class="diag-scope-row">
              <div class="diag-scope-selector" role="radiogroup" aria-label="Issue debt scope">
                <button type="button" class="diag-scope-btn ${selectedScope === 'active' ? 'active' : ''}" data-scope="active" title="Show active regressions (quiet by default)">
                  Active (${activeCount})
                </button>
                <button type="button" class="diag-scope-btn ${selectedScope === 'legacy' ? 'active' : ''}" data-scope="legacy" title="Show acknowledged baseline issues">
                  Baselined (${legacyDebtCount})
                </button>
                <button type="button" class="diag-scope-btn ${selectedScope === 'all' ? 'active' : ''}" data-scope="all" title="Show all issues (active + baselined)">
                  All (${allCount})
                </button>
              </div>
              <label class="diag-include-excluded-label" title="Include issues from paths excluded via scan/graph rules">
                <input type="checkbox" id="chk-include-excluded" ${includeExcluded ? 'checked' : ''} />
                <span>Include Excluded</span>
              </label>
            </div>
            <div class="diag-pillar-filter-row">
              <div class="diag-pillar-chips" role="group" aria-label="Filter issues by quality pillar">
              <button type="button" class="diag-pillar-chip ${selectedPillars.has('integrity') ? 'active' : ''}" data-filter="integrity" aria-pressed="${selectedPillars.has('integrity')}">
                <span class="diag-chip-val" style="color: ${integrityCount > 0 ? '#ef4444' : 'inherit'};">${integrityCount}</span>
                <span class="diag-chip-lbl">Integrity</span>
              </button>
              <button type="button" class="diag-pillar-chip ${selectedPillars.has('structure') ? 'active' : ''}" data-filter="structure" aria-pressed="${selectedPillars.has('structure')}">
                <span class="diag-chip-val" style="color: ${structureCount > 0 ? '#38bdf8' : 'inherit'};">${structureCount}</span>
                <span class="diag-chip-lbl">Structure</span>
              </button>
              <button type="button" class="diag-pillar-chip ${selectedPillars.has('consistency') ? 'active' : ''}" data-filter="consistency" aria-pressed="${selectedPillars.has('consistency')}">
                <span class="diag-chip-val" style="color: ${consistencyCount > 0 ? '#f472b6' : 'inherit'};">${consistencyCount}</span>
                <span class="diag-chip-lbl">Consistency</span>
              </button>
              <button type="button" class="diag-pillar-chip ${selectedPillars.has('discoverability') ? 'active' : ''}" data-filter="discoverability" aria-pressed="${selectedPillars.has('discoverability')}">
                <span class="diag-chip-val" style="color: ${discoverabilityCount > 0 ? '#eab308' : 'inherit'};">${discoverabilityCount}</span>
                <span class="diag-chip-lbl">Discovery</span>
              </button>
              <button type="button" class="diag-pillar-chip ${selectedPillars.has('traceability') || selectedPillars.has('freshness') ? 'active' : ''}" data-filter="traceability" aria-pressed="${selectedPillars.has('traceability') || selectedPillars.has('freshness')}">
                <span class="diag-chip-val" style="color: ${freshnessCount > 0 ? '#c084fc' : 'inherit'};">${freshnessCount}</span>
                <span class="diag-chip-lbl">Traceability</span>
              </button>
              </div>
              <button type="button" class="diag-clear-filters" id="diag-clear-filters" ${selectedPillars.size === 0 && selectedKosFilter === 'all' ? 'disabled' : ''}>Clear</button>
            </div>
            <div class="findings-kos-filter-bar" style="margin-top: 10px; padding-top: 8px; border-top: 1px solid rgba(255, 255, 255, 0.08);">
              <span class="findings-filter-label" style="font-size: 11px; font-weight: 600; opacity: 0.85; margin-bottom: 6px; display: block;">Filter by KOS Discipline:</span>
              <div class="kos-chips-container">
                <button type="button" class="kos-filter-chip diag-kos-chip ${selectedKosFilter === 'all' ? 'active' : ''}" data-kos-id="all" title="All 18 KOS Methodologies (${issues.length} findings)">
                  <span class="chip-name">All 18 KOS</span>
                  <span class="chip-count">${issues.length}</span>
                </button>
                ${KOS_18_METHODOLOGIES.map((k) => {
                  const count = kosCounts[k.id] || 0;
                  const isActive = selectedKosFilter === k.id;
                  return `
                    <button type="button" class="kos-filter-chip diag-kos-chip ${isActive ? 'active' : ''} ${count > 0 ? 'has-findings' : 'zero-findings'}" data-kos-id="${k.id}" title="${k.name}: ${k.description}">
                      <span class="chip-icon">${k.icon}</span>
                      <span class="chip-name">${k.shortLabel}</span>
                      <span class="chip-count">${count}</span>
                    </button>
                  `;
                }).join('')}
              </div>
            </div>
          </div>

          <div class="diag-issues-list">
            ${accordionsHtml}
          </div>
        </div>
      `;
      break;
    }

    case 'structure': {
      const totalRefs = nodes.reduce((sum, n) => sum + (n.outDegree || 0), 0);
      const topConnected = [...nodes]
        .sort((a, b) => ((b.inDegree || 0) + (b.outDegree || 0)) - ((a.inDegree || 0) + (a.outDegree || 0)))
        .slice(0, 8);

      const topConnectedHtml = topConnected.length > 0
        ? topConnected.map((n) => `
            <div class="structure-note-item" data-file="${n.filePath}" title="Click to open ${n.filePath}">
              <span class="structure-note-title">${n.label}</span>
              <span class="structure-note-badge">${(n.inDegree || 0) + (n.outDegree || 0)} refs</span>
            </div>
          `).join('')
        : '<div class="settings-empty-msg">No connected documents in graph</div>';

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Knowledge Graph Structure</div>
          <p class="settings-desc">Document topology, reference density, and designated hub anchor.</p>

          <div class="settings-diag-grid">
            <div class="diag-item">
              <span class="diag-lbl">Total Documents</span>
              <span class="diag-val">${totalNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Connected</span>
              <span class="diag-val">${connectedNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Isolated</span>
              <span class="diag-val">${orphanNotes}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">References</span>
              <span class="diag-val">${totalRefs}</span>
            </div>
          </div>
        </div>

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
          <div class="settings-sec-title">Top Connected Notes</div>
          <p class="settings-desc">Notes with the highest inbound and outbound reference centrality.</p>
          <div class="structure-notes-list">
            ${topConnectedHtml}
          </div>
        </div>

        ${props.directoryMetrics ? `
        <div class="settings-section">
          <div class="settings-sec-title">Physical Directory & Knowledge Organization</div>
          <p class="settings-desc">Non-prescriptive directory depth, Shannon concentration entropy, and cross-folder coupling.</p>

          <div class="settings-diag-grid">
            <div class="diag-item">
              <span class="diag-lbl">Max Depth</span>
              <span class="diag-val">${props.directoryMetrics.max_directory_depth}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Avg Depth</span>
              <span class="diag-val">${props.directoryMetrics.average_directory_depth.toFixed(1)}</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Entropy</span>
              <span class="diag-val">${Math.round(props.directoryMetrics.concentration_entropy * 100)}%</span>
            </div>
            <div class="diag-item">
              <span class="diag-lbl">Cross Coupling</span>
              <span class="diag-val">${Math.round(props.directoryMetrics.cross_folder_coupling * 100)}%</span>
            </div>
          </div>
        </div>
        ` : ''}
      `;
      break;
    }

    case 'evolution': {
      try {
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
              ${isCustomOpen ? '▲ Hide Custom Parameters' : '⚙️ Custom Parameters'}
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

        const orientationLabel = activeOrientation
          .replace(/_/g, ' ')
          .replace(/\b\w/g, (c) => c.toUpperCase());

        const methodologyCatalogHtml = renderEvolutionMethodologyCatalogHtml(orientationLabel);

        tabContentHtml = `
          <div class="evolution-settings-wrapper">
            <div class="evolution-header" style="margin-bottom: 12px;">
              <h2 style="font-size: 15px; font-weight: 600; margin: 0 0 4px 0;">Structural Evolution & Knowledge Methodology</h2>
              <p class="evolution-subtitle" style="font-size: 11.5px; opacity: 0.75; margin: 0;">Configure evolutionary optimization profiles, objective trade-offs, and governance parameters for your knowledge graph.</p>
            </div>
            ${orientationHtml}
            ${customToggleBtnHtml}
            ${customDrawerHtml}
            <div class="evolution-section-divider"></div>
            ${methodologyCatalogHtml}
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

    case 'load': {
      const summary = props.tokenSummary || {
        activeHarnessTokens: props.nodes?.find((n) => n.hopDistance === 0)?.tokens || 0,
        subcallAccessibleTokens: (props.nodes || [])
          .filter((n) => n.hopDistance >= 0 && n.hopDistance <= 2)
          .reduce((acc, n) => acc + (n.tokens || 0), 0),
        warningThreshold: 60000,
        criticalThreshold: 100000,
        status: 'normal' as const,
        contextReductionPercent: 90,
      };

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

      const reductionPct = summary.contextReductionPercent !== undefined
        ? summary.contextReductionPercent
        : (reachableTokens > 0 ? Math.max(0, Math.round(((reachableTokens - activeTokens) / reachableTokens) * 100)) : 90);

      const topNodes = [...(props.nodes || [])]
        .filter((n) => (n.tokens || 0) > 0)
        .sort((a, b) => (b.tokens || 0) - (a.tokens || 0))
        .slice(0, 15);

      const topConsumersHtml = topNodes.length > 0
        ? topNodes.map((n) => `
          <li class="load-consumer-item" data-path="${n.filePath || n.id}" title="Click to open ${n.filePath || n.id}">
            <div class="load-consumer-info">
              <span class="load-consumer-title">${n.label || n.id}</span>
              <span class="load-consumer-path">${n.filePath || n.id}</span>
            </div>
            <span class="load-consumer-tokens">${formatCompactNumber(n.tokens)} tok</span>
          </li>`).join('')
        : '<li class="load-empty-item">No indexed documents found</li>';

      const statusLabel = summary.status ? summary.status.charAt(0).toUpperCase() + summary.status.slice(1).toLowerCase() : 'Normal';

      tabContentHtml = `
        <div class="settings-section">
          <div class="settings-sec-title">Active Prompt Load — L0 Budget</div>
          <p class="settings-desc">Persistent prompt token load from the primary active AI harness and designated central hub note.</p>
          <div class="load-budget-card">
            <div class="load-metric-row">
              <div class="load-metric-main">
                <span class="load-metric-number">${formatCompactNumber(activeTokens)}</span>
                <span class="load-metric-unit">tok</span>
              </div>
              <div class="load-metric-badge-group">
                <span class="load-status-badge ${badgeClass}">${statusLabel}</span>
                <span class="load-reduction-badge">~${reductionPct}% reduction</span>
              </div>
            </div>
            <div class="token-progress-track" style="margin: 10px 0 6px 0; height: 6px;">
              <div class="token-progress-fill" style="width: ${budgetPct}%; background-color: ${barColor};"></div>
            </div>
            <div class="load-budget-limits">
              <span>Budget used: <strong>${budgetPct}%</strong> of critical limit</span>
              <span class="load-limits-sub">Warning: ${formatCompactNumber(warnLimit)} tok · Critical: ${formatCompactNumber(critLimit)} tok</span>
            </div>
          </div>
        </div>

        <div class="settings-section" style="margin-top: 14px;">
          <div class="settings-sec-title">Reachable Subcall Context — L1–L2</div>
          <p class="settings-desc">On-demand knowledge accessible within 2 hops of the active hub. Subagents and LLM tools fetch these on demand without burdening the permanent prompt context.</p>
          <div class="load-reachable-card">
            <div class="load-reachable-row">
              <span class="load-reachable-label">On-demand Subcall Context:</span>
              <span class="load-reachable-val">${formatCompactNumber(reachableTokens)} tok</span>
            </div>
            <div class="load-advice-text">
              Reachable notes are within 2 hops of the active hub. They can be fetched on demand by LLM tools without burdening the permanent prompt context.
            </div>
          </div>
        </div>

        <div class="settings-section" style="margin-top: 14px;">
          <div class="settings-sec-title">Top Token Consumers</div>
          <p class="settings-desc">Heaviest documents indexed in the workspace. Click any document to open it in the editor.</p>
          <ul class="load-consumers-list">
            ${topConsumersHtml}
          </ul>
        </div>
      `;
      break;
    }
  }

  return `
    <div class="settings-drawer${isEntering ? ' drawer-entering' : ''}" id="settings-drawer" style="display: none;">
      <div class="settings-header">
        <div class="settings-title-box">
          <span class="settings-icon">
            <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor">
              <path fill-rule="evenodd" clip-rule="evenodd" d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.3h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.3-.4V7.4l-2.4-.5-.3-.7 1.3-2-.8-.9-2 1.3-.7-.2zM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
            </svg>
          </span>
          <h3 class="settings-title">Management & Settings</h3>
        </div>
        <button class="settings-close-btn" id="settings-btn-close" aria-label="Close Settings">✕</button>
      </div>

      <div class="settings-nav-tiers">
        <div class="settings-nav-group" data-tier="insights">
          <span class="settings-nav-label">Insights</span>
          <div class="settings-tabs-subgroup">
            <button class="settings-tab-btn ${normalizedTab === 'health' ? 'active' : ''}" data-tab="health">Health</button>
            <button class="settings-tab-btn ${normalizedTab === 'structure' ? 'active' : ''}" data-tab="structure">Structure</button>
            <button class="settings-tab-btn ${normalizedTab === 'evolution' ? 'active' : ''}" data-tab="evolution">Evolution</button>
            <button class="settings-tab-btn ${normalizedTab === 'load' ? 'active' : ''}" data-tab="load">Load</button>
          </div>
        </div>
        <div class="settings-nav-group" data-tier="settings">
          <span class="settings-nav-label">Settings</span>
          <div class="settings-tabs-subgroup">
            <button class="settings-tab-btn ${normalizedTab === 'exclusions' ? 'active' : ''}" data-tab="exclusions">Exclusions</button>
            <button class="settings-tab-btn ${normalizedTab === 'clusters' ? 'active' : ''}" data-tab="clusters">Clusters</button>
            <button class="settings-tab-btn ${normalizedTab === 'git' ? 'active' : ''}" data-tab="git">Git</button>
            <button class="settings-tab-btn ${normalizedTab === 'harness' || normalizedTab === 'git_harness' ? 'active' : ''}" data-tab="harness">Harness</button>
          </div>
        </div>
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
  let state = {
    selectedDebtScope: (initialProps.selectedDebtScope || 'active') as 'active' | 'legacy' | 'all',
    includeExcluded: initialProps.includeExcluded ?? false,
    evolutionFindingsPage: initialProps.evolutionFindingsPage || 1,
    evolutionFindingsPageSize: initialProps.evolutionFindingsPageSize || 10,
    selectedKosFilter: initialProps.selectedKosFilter || 'all',
    evolutionPillarFilter: initialProps.evolutionPillarFilter || 'all',
    evolutionSignaturesPage: initialProps.evolutionSignaturesPage || 1,
    evolutionSignaturesPageSize: initialProps.evolutionSignaturesPageSize || 10,
    selectedSignatureTypeFilter: initialProps.selectedSignatureTypeFilter || 'all',
    ...initialProps,
  };
  let isVisible = false;
  let isEntering = false;
  let enterTimer: any = null;

  let _lastRenderedTab: SettingsTab | null = null;
  let _pendingOrientation: string | null = null;   // optimistic lock while awaiting IPC confirm
  let _renderPending = false;                      // debounce: only one rAF render in flight

  function wireKnowledgeListeners(): void {
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
  }

  function wireExclusionsListeners(): void {
    const addBtn = container.querySelector('#btn-add-rule') as HTMLElement | null;
    const input = container.querySelector('#input-new-rule') as HTMLInputElement | null;
    const scopeBtn = container.querySelector('#btn-scope-toggle') as HTMLElement | null;
    const scopeMenu = container.querySelector('#menu-scope-options') as HTMLElement | null;
    const scopeLabel = container.querySelector('#label-scope-selected') as HTMLElement | null;
    const scopeHidden = container.querySelector('#select-rule-scope') as HTMLInputElement | null;

    if (scopeBtn && scopeMenu) {
      scopeBtn.onclick = (e) => {
        e.preventDefault();
        e.stopPropagation();
        const isHidden = scopeMenu.style.display === 'none' || !scopeMenu.style.display;
        scopeMenu.style.display = isHidden ? 'block' : 'none';
        scopeBtn.setAttribute('aria-expanded', String(isHidden));
      };

      scopeMenu.querySelectorAll<HTMLElement>('.scope-menu-item').forEach((item) => {
        item.onclick = (e) => {
          e.preventDefault();
          e.stopPropagation();
          const val = item.getAttribute('data-value') as 'scan' | 'graph';
          if (val) {
            state.selectedRuleScope = val;
            if (scopeHidden) scopeHidden.value = val;
            if (scopeLabel) scopeLabel.textContent = val === 'graph' ? 'Graph' : 'Scan';
            scopeMenu.querySelectorAll('.scope-menu-item').forEach((i) => {
              const isSelected = i === item;
              i.classList.toggle('active', isSelected);
              i.setAttribute('aria-selected', String(isSelected));
              const check = i.querySelector('.scope-item-check') as HTMLElement | null;
              if (check) check.style.visibility = isSelected ? 'visible' : 'hidden';
            });
            scopeMenu.style.display = 'none';
            scopeBtn.setAttribute('aria-expanded', 'false');
          }
        };
      });
    }

    const doAddRule = () => {
      const pat = input?.value.trim();
      const scope = state.selectedRuleScope || (scopeHidden?.value as 'scan' | 'graph') || 'graph';
      if (pat) {
        callbacks.onAddExclusion?.(pat, scope);
        if (scope === 'graph') {
          if (!state.graphExclusions.includes(pat)) {
            state.graphExclusions.push(pat);
          }
        } else {
          if (!state.scanExclusions.includes(pat)) {
            state.scanExclusions.push(pat);
          }
        }
        if (input) input.value = '';
        render();
      }
    };

    if (addBtn) {
      addBtn.onclick = (e) => {
        e.stopPropagation();
        doAddRule();
      };
    }

    if (input) {
      input.onkeydown = (e) => {
        if (e.key === 'Enter') {
          e.preventDefault();
          e.stopPropagation();
          doAddRule();
        }
      };
    }

    container.querySelectorAll<HTMLElement>('.rule-del-btn').forEach((btn) => {
      btn.onclick = (e) => {
        e.stopPropagation();
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
      };
    });
  }

  function scheduleRender(): void {
    // For IPC-driven updates, do a PARTIAL DOM update — only replace the
    // content area. The #settings-content element stays alive so its
    // scrollTop is naturally preserved (no rAF tricks needed).
    if (_renderPending) return;
    _renderPending = true;
    requestAnimationFrame(() => {
      _renderPending = false;
      const contentEl = container.querySelector('#settings-content') as HTMLElement | null;
      if (!contentEl || _lastRenderedTab !== state.activeTab) {
        // No content element yet, or tab just changed — need full render
        render();
        return;
      }
      // Partial update: parse new HTML in a temp div, extract content, inject
      const tmp = document.createElement('div');
      tmp.innerHTML = renderSettingsDrawerHtml(state, false);
      const newContent = tmp.querySelector('#settings-content');
      if (newContent) {
        contentEl.innerHTML = newContent.innerHTML;
        wireKnowledgeListeners();
        wireDiagnosticsAccordionListeners();
        wireEvolutionListeners();
        wireHealthAndStructureActions();
        wireExclusionsListeners();
        wireGitListeners();
        wireHarnessListeners();
        wireLoadListeners();
        applyDiagnosticsFilterToDom();
      }
    });
  }

  function wireGitListeners(): void {
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

    const toggleMissingFrag = container.querySelector('#toggle-block-missing-fragments') as HTMLInputElement | null;
    toggleMissingFrag?.addEventListener('change', (e) => {
      e.stopPropagation();
      state.blockOnMissingFragments = toggleMissingFrag.checked;
      callbacks.onToggleThreshold?.('blockOnMissingFragments', toggleMissingFrag.checked);
    });

    container.querySelectorAll('.settings-hook-card .btn-install-harness-tool').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const toolId = (btn as HTMLElement).getAttribute('data-tool-id');
        if (toolId) {
          callbacks.onInstallHarnessTool?.(toolId);
        }
      });
    });
  }

  function wireHarnessListeners(): void {
    const prepDropdown = container.querySelector('#prep-active-dropdown');
    const prepBtn = container.querySelector('#btn-prep-active-toggle');
    const prepMenu = container.querySelector('#menu-prep-active-options') as HTMLElement | null;
    const prepLabel = container.querySelector('#label-prep-active-selected');
    const prepHidden = container.querySelector('#select-prep-active-harness') as HTMLInputElement | null;

    const activeDropdown = container.querySelector('#active-harness-dropdown');
    const activeBtn = container.querySelector('#btn-active-harness-toggle');
    const activeMenu = container.querySelector('#menu-active-harness-options') as HTMLElement | null;
    const activeLabel = container.querySelector('#label-active-harness-selected');
    const activeHidden = container.querySelector('#select-active-harness') as HTMLInputElement | null;

    const selectHarnessChoice = (val: string) => {
      if (!val) return;
      state.selectedPrepActiveHarness = val;

      // 1. Update prep dropdown
      if (prepHidden) prepHidden.value = val;
      if (prepLabel) prepLabel.textContent = val;
      if (prepMenu) {
        prepMenu.querySelectorAll('.scope-menu-item').forEach((item) => {
          const itemVal = item.getAttribute('data-value');
          const isSelected = itemVal === val;
          item.classList.toggle('active', isSelected);
          item.setAttribute('aria-selected', String(isSelected));
          const check = item.querySelector('.scope-item-check') as HTMLElement | null;
          if (check) check.style.visibility = isSelected ? 'visible' : 'hidden';
        });
      }

      // 2. Update active dropdown
      if (activeHidden) activeHidden.value = val;
      if (activeLabel) {
        const harnesses = state.harnessStatus?.harnesses ?? [];
        const hObj = harnesses.find((h) => h.file_path === val);
        activeLabel.textContent = hObj ? `${val} (${hObj.harness_type})` : val;
      }
      if (activeMenu) {
        activeMenu.querySelectorAll('.scope-menu-item').forEach((item) => {
          const itemVal = item.getAttribute('data-value');
          const isSelected = itemVal === val;
          item.classList.toggle('active', isSelected);
          item.setAttribute('aria-selected', String(isSelected));
          const check = item.querySelector('.scope-item-check') as HTMLElement | null;
          if (check) check.style.visibility = isSelected ? 'visible' : 'hidden';
        });
      }

      // 3. Notify backend if harness exists on disk
      const harnesses = state.harnessStatus?.harnesses ?? [];
      const exists = harnesses.some((h) => h.file_path === val);
      if (exists) {
        callbacks.onSelectActiveHarness?.(val);
      }
    };

    // Toggle popover menus
    prepBtn?.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (!prepMenu) return;
      const isHidden = prepMenu.style.display === 'none' || !prepMenu.style.display;
      prepMenu.style.display = isHidden ? 'block' : 'none';
      prepBtn.setAttribute('aria-expanded', String(isHidden));
      if (activeMenu) {
        activeMenu.style.display = 'none';
        activeBtn?.setAttribute('aria-expanded', 'false');
      }
    });

    activeBtn?.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (!activeMenu) return;
      const isHidden = activeMenu.style.display === 'none' || !activeMenu.style.display;
      activeMenu.style.display = isHidden ? 'block' : 'none';
      activeBtn.setAttribute('aria-expanded', String(isHidden));
      if (prepMenu) {
        prepMenu.style.display = 'none';
        prepBtn?.setAttribute('aria-expanded', 'false');
      }
    });

    const wirePrepMenuItems = () => {
      if (!prepMenu) return;
      prepMenu.querySelectorAll<HTMLElement>('.scope-menu-item').forEach((item) => {
        item.onclick = (e) => {
          e.preventDefault();
          e.stopPropagation();
          const val = item.getAttribute('data-value');
          if (val) {
            selectHarnessChoice(val);
            prepMenu.style.display = 'none';
            prepBtn?.setAttribute('aria-expanded', 'false');
          }
        };
      });
    };
    wirePrepMenuItems();

    if (activeMenu) {
      activeMenu.querySelectorAll<HTMLElement>('.scope-menu-item').forEach((item) => {
        item.onclick = (e) => {
          e.preventDefault();
          e.stopPropagation();
          const val = item.getAttribute('data-value');
          if (val) {
            selectHarnessChoice(val);
            activeMenu.style.display = 'none';
            activeBtn?.setAttribute('aria-expanded', 'false');
          }
        };
      });
    }

    const updatePrepActiveOptions = () => {
      const chkCursor = container.querySelector('#chk-tool-cursor') as HTMLInputElement | null;
      const chkCodex = container.querySelector('#chk-tool-codex') as HTMLInputElement | null;
      const chkAgy = container.querySelector('#chk-tool-agy') as HTMLInputElement | null;
      const chkClaude = container.querySelector('#chk-tool-claude') as HTMLInputElement | null;
      const chkCopilot = container.querySelector('#chk-tool-copilot') as HTMLInputElement | null;

      const candidates = new Set<string>();
      if (chkCursor?.checked || chkCodex?.checked) candidates.add('AGENTS.md');
      if (chkAgy?.checked) candidates.add('GEMINI.md');
      if (chkClaude?.checked) candidates.add('CLAUDE.md');
      if (chkCopilot?.checked) candidates.add('.github/copilot-instructions.md');
      (state.harnessStatus?.harnesses ?? []).forEach((h) => candidates.add(h.file_path));
      if (candidates.size === 0) candidates.add('AGENTS.md');

      const currentVal = prepHidden?.value || state.selectedPrepActiveHarness || state.harnessStatus?.active_harness?.file_path || 'AGENTS.md';
      const effectiveVal = candidates.has(currentVal) ? currentVal : (candidates.has('AGENTS.md') ? 'AGENTS.md' : Array.from(candidates)[0]);

      if (prepMenu) {
        prepMenu.innerHTML = Array.from(candidates)
          .map((cand) => `
            <div class="scope-menu-item ${cand === effectiveVal ? 'active' : ''}" data-value="${cand}" role="option" aria-selected="${cand === effectiveVal}">
              <span class="scope-item-check" style="${cand === effectiveVal ? '' : 'visibility: hidden;'}">✓</span>
              <span class="scope-item-title">${cand}</span>
            </div>
          `)
          .join('');
        wirePrepMenuItems();
      }

      selectHarnessChoice(effectiveVal);
    };

    const syncToolSelection = () => {
      const chkCursor = container.querySelector('#chk-tool-cursor') as HTMLInputElement | null;
      const chkCodex = container.querySelector('#chk-tool-codex') as HTMLInputElement | null;
      const chkAgy = container.querySelector('#chk-tool-agy') as HTMLInputElement | null;
      const chkClaude = container.querySelector('#chk-tool-claude') as HTMLInputElement | null;
      const chkCopilot = container.querySelector('#chk-tool-copilot') as HTMLInputElement | null;

      state.toolSelection = {
        cursor: chkCursor?.checked ?? false,
        codex: chkCodex?.checked ?? false,
        agy: chkAgy?.checked ?? false,
        claude: chkClaude?.checked ?? false,
        copilot: chkCopilot?.checked ?? false,
      };

      updatePrepActiveOptions();
    };

    ['#chk-tool-cursor', '#chk-tool-codex', '#chk-tool-agy', '#chk-tool-claude', '#chk-tool-copilot'].forEach((sel) => {
      container.querySelector(sel)?.addEventListener('change', (e) => {
        e.stopPropagation();
        syncToolSelection();
      });
    });

    prepHidden?.addEventListener('change', (e) => {
      e.stopPropagation();
      selectHarnessChoice(prepHidden.value);
    });

    activeHidden?.addEventListener('change', (e) => {
      e.stopPropagation();
      selectHarnessChoice(activeHidden.value);
    });

    container.querySelector('#btn-prepare-harness-base')?.addEventListener('click', (e) => {
      e.stopPropagation();
      const chkCursor = container.querySelector('#chk-tool-cursor') as HTMLInputElement | null;
      const chkCodex = container.querySelector('#chk-tool-codex') as HTMLInputElement | null;
      const chkAgy = container.querySelector('#chk-tool-agy') as HTMLInputElement | null;
      const chkClaude = container.querySelector('#chk-tool-claude') as HTMLInputElement | null;
      const chkCopilot = container.querySelector('#chk-tool-copilot') as HTMLInputElement | null;

      const enabledHarnesses = new Set<string>();
      const createFolders = new Set<string>();

      if (chkCursor?.checked) {
        enabledHarnesses.add('AGENTS.md');
        createFolders.add('.cursor/rules');
      }
      if (chkCodex?.checked) {
        enabledHarnesses.add('AGENTS.md');
      }
      if (chkAgy?.checked) {
        enabledHarnesses.add('GEMINI.md');
        createFolders.add('.agents');
      }
      if (chkClaude?.checked) {
        enabledHarnesses.add('CLAUDE.md');
      }
      if (chkCopilot?.checked) {
        enabledHarnesses.add('.github/copilot-instructions.md');
        createFolders.add('.github');
      }

      if (enabledHarnesses.size === 0) {
        enabledHarnesses.add('AGENTS.md');
      }

      const createDirs = (container.querySelector('#chk-prep-create-dirs') as HTMLInputElement | null)?.checked ?? true;
      const consolidateIndex = (container.querySelector('#chk-prep-consolidate-index') as HTMLInputElement | null)?.checked ?? true;
      const cleanUnused = (container.querySelector('#chk-prep-clean-unused') as HTMLInputElement | null)?.checked ?? true;
      const selectActive = container.querySelector('#select-prep-active-harness') as HTMLSelectElement | null;
      const activeHarness = selectActive?.value || state.selectedPrepActiveHarness || (enabledHarnesses.has('AGENTS.md') ? 'AGENTS.md' : Array.from(enabledHarnesses)[0]);

      const feedback = container.querySelector('#prep-base-feedback') as HTMLElement | null;
      if (feedback) {
        feedback.style.display = 'block';
        feedback.style.background = 'rgba(56, 189, 248, 0.15)';
        feedback.style.color = '#38bdf8';
        feedback.style.border = '1px solid rgba(56, 189, 248, 0.3)';
        feedback.textContent = `Provisioning ${Array.from(enabledHarnesses).join(', ')}...`;
      }

      callbacks.onConfigureHarnessBase?.({
        active_harness: activeHarness,
        enabled_harnesses: Array.from(enabledHarnesses),
        create_folders: createDirs ? Array.from(createFolders) : [],
        consolidate_index: consolidateIndex,
        clean_unused: cleanUnused,
      });
    });

    container.querySelector('#btn-init-harness')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onInitializeHarness?.('AGENTS.md');
    });

    container.querySelectorAll('.btn-init-specific-harness').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const harness = (btn as HTMLElement).getAttribute('data-harness') || 'AGENTS.md';
        state.selectedPrepActiveHarness = harness;
        callbacks.onInitializeHarness?.(harness);
      });
    });

    container.querySelector('#btn-sync-all-harnesses')?.addEventListener('click', (e) => {
      e.stopPropagation();
      const target = state.selectedPrepActiveHarness || 'AGENTS.md';
      callbacks.onGenerateGovernancePlan?.('sync_harness_routes', target);
    });

    container.querySelector('#btn-refresh-skills')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onRequestSkillCatalog?.();
    });

    container.querySelector('#btn-install-llm-wiki-skill')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onInstallLLMWikiSkill?.();
    });

    container.querySelectorAll('.btn-gov-embed-index').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const harness = (btn as HTMLElement).getAttribute('data-harness') || 'AGENTS.md';
        callbacks.onGenerateGovernancePlan?.('embed_knowledge_index', harness);
      });
    });

    container.querySelectorAll('.btn-gov-eliminate-secondary').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('eliminate_secondary_routing_layer', target);
        }
      });
    });

    container.querySelectorAll('.btn-gov-sync-harnesses').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || 'AGENTS.md';
        callbacks.onGenerateGovernancePlan?.('sync_harness_routes', target);
      });
    });

    container.querySelectorAll('.btn-gov-normalize-links').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('normalize_links_to_markdown', target);
        }
      });
    });

    container.querySelectorAll('.btn-gov-promote-skill').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('create_skill_from_procedure', target);
        }
      });
    });

    container.querySelector('#btn-refresh-harness-tools')?.addEventListener('click', (e) => {
      e.stopPropagation();
      callbacks.onRequestHarnessTools?.();
    });

    container.querySelectorAll('.harness-tools-catalog-card .btn-install-harness-tool').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const toolId = (btn as HTMLElement).getAttribute('data-tool-id');
        if (toolId) {
          callbacks.onInstallHarnessTool?.(toolId);
        }
      });
    });
  }

  function wireLoadListeners(): void {
    container.querySelectorAll('.load-consumer-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const filePath = item.getAttribute('data-path');
        if (filePath) {
          callbacks.onOpenFile?.(filePath);
        }
      });
    });
  }

  function render(): void {
    // Full render — replaces the entire drawer shell + content
    container.innerHTML = renderSettingsDrawerHtml(state, isEntering);
    _lastRenderedTab = state.activeTab;

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

    container.querySelectorAll('.settings-tab-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const tab = btn.getAttribute('data-tab') as SettingsTab;
        if (tab && tab !== state.activeTab) {
          state.activeTab = tab;
          callbacks.onChangeTab?.(tab);
          if (tab === 'exclusions') {
            callbacks.onRequestExclusions?.();
          }
          if (tab === 'diagnostics' || tab === 'health') {
            if (callbacks.onRequestIssuesWithOptions) {
              callbacks.onRequestIssuesWithOptions({
                include_debt: state.selectedDebtScope !== 'active',
                include_excluded: state.includeExcluded,
                pillar: state.selectedPillarFilter || undefined,
              });
            } else {
              callbacks.onRequestIssues?.(state.selectedPillarFilter);
            }
          }
          if (tab === 'evolution') {
            if (!state.issuesReport) {
              callbacks.onRequestIssues?.();
            }
            callbacks.onRequestEvolution?.();
          }
          if (tab === 'git' || tab === 'harness' || tab === 'git_harness') {
            callbacks.onRequestHarnessTools?.();
            callbacks.onRequestHarnessStatus?.();
          }
          render();
        }
      });
    });

    wireKnowledgeListeners();
    wireExclusionsListeners();
    wireGitListeners();
    wireHarnessListeners();
    wireLoadListeners();

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

    wireHealthAndStructureActions();
    wireDiagnosticsAccordionListeners();

    // --- Evolution Tab Event Wiring ---
    wireEvolutionListeners();
  }

  function wireHealthAndStructureActions(): void {
    // Clickable file links & open target
    container.querySelectorAll('.diag-open-doc-btn, .diag-open-code-btn, .diag-open-target-btn, .diag-file-clickable, .diag-target-clickable, [data-action="open-target"]').forEach((el) => {
      el.addEventListener('click', (e) => {
        e.stopPropagation();
        const file = el.getAttribute('data-target') || el.getAttribute('data-file');
        const lineAttr = el.getAttribute('data-line');
        const line = lineAttr ? parseInt(lineAttr, 10) : undefined;
        if (file) {
          callbacks.onOpenFile?.(file, line);
        }
      });
    });

    function closeAllSplitMenus(): void {
      container.querySelectorAll<HTMLElement>('.diag-split-menu').forEach((m) => {
        m.style.display = 'none';
      });
      container.querySelectorAll('.diag-split-toggle').forEach((t) => {
        t.setAttribute('aria-expanded', 'false');
      });
      container.querySelectorAll('.diag-action-split-btn').forEach((p) => {
        p.classList.remove('is-open');
      });
      container.querySelectorAll('.diag-issue-card, .evolution-finding-item').forEach((c) => {
        c.classList.remove('menu-open');
      });
    }

    // Split button dropdown toggles
    container.querySelectorAll('.diag-split-toggle').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const parent = btn.closest('.diag-action-split-btn');
        const card = btn.closest('.diag-issue-card, .evolution-finding-item');
        const menu = parent?.querySelector('.diag-split-menu') as HTMLElement | null;
        if (!menu) return;
        const isShown = menu.style.display === 'block';
        closeAllSplitMenus();
        if (!isShown) {
          menu.style.display = 'block';
          btn.setAttribute('aria-expanded', 'true');
          parent?.classList.add('is-open');
          card?.classList.add('menu-open');

          // Auto-flip upward if overflowing viewport bottom
          const rect = menu.getBoundingClientRect();
          if (rect.bottom > window.innerHeight - 10) {
            menu.style.top = 'auto';
            menu.style.bottom = 'calc(100% + 4px)';
          } else {
            menu.style.top = 'calc(100% + 4px)';
            menu.style.bottom = 'auto';
          }
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

    // Exclude Path buttons (contextual inside Health findings)
    container.querySelectorAll('.diag-exclude-path-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const file = btn.getAttribute('data-file');
        if (file) {
          callbacks.onAddExclusion?.(file, 'graph');
        }
      });
    });

    // Structure Note Items (click to open note in editor)
    container.querySelectorAll('.structure-note-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const file = item.getAttribute('data-file');
        if (file) {
          callbacks.onOpenFile?.(file);
        }
      });
    });

    // Secondary Routing Layer Elimination (Safely Remove & Consolidate)
    container.querySelectorAll('.btn-gov-eliminate-secondary').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('eliminate_secondary_routing_layer', target);
        }
      });
    });

    container.querySelectorAll('.btn-gov-sync-harnesses').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || 'AGENTS.md';
        callbacks.onGenerateGovernancePlan?.('sync_harness_routes', target);
      });
    });

    container.querySelectorAll('.btn-gov-normalize-links').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('normalize_links_to_markdown', target);
        }
      });
    });
  }

  function applyDiagnosticsFilterToDom(): void {
    const selected = new Set(
      (state.selectedPillarFilter || '')
        .split(',')
        .map((pillar) => pillar.trim())
        .filter(Boolean),
    );

    container.querySelectorAll<HTMLElement>('.diag-accordion-group').forEach((group) => {
      const pillar = group.getAttribute('data-pillar');
      group.style.display = selected.size === 0 || (pillar ? selected.has(pillar) : false) ? '' : 'none';
    });

    container.querySelectorAll<HTMLElement>('.diag-pillar-chip').forEach((chip) => {
      const pillar = chip.getAttribute('data-filter');
      const active = pillar ? selected.has(pillar) : false;
      chip.classList.toggle('active', active);
      chip.setAttribute('aria-pressed', String(active));
    });

    const clearButton = container.querySelector<HTMLButtonElement>('#diag-clear-filters');
    if (clearButton) clearButton.disabled = selected.size === 0;
  }

  function wireDiagnosticsFilterListeners(): void {
    if (container.dataset.diagnosticsFiltersWired === 'true') return;
    container.dataset.diagnosticsFiltersWired = 'true';

    container.addEventListener('click', (event) => {
      const target = event.target as HTMLElement;
      const filterButton = target.closest<HTMLElement>('.diag-pillar-chip');
      const kosChip = target.closest<HTMLElement>('.diag-kos-chip');
      const clearButton = target.closest<HTMLElement>('#diag-clear-filters');
      const scopeButton = target.closest<HTMLElement>('.diag-scope-btn, .diag-inspect-debt-btn');
      if (!filterButton && !kosChip && !clearButton && !scopeButton) return;

      event.preventDefault();
      event.stopPropagation();

      if (scopeButton) {
        const scope = (scopeButton.getAttribute('data-scope') || 'active') as 'active' | 'legacy' | 'all';
        state.selectedDebtScope = scope;
        callbacks.onRequestIssuesWithOptions?.({
          include_debt: scope !== 'active',
          include_excluded: state.includeExcluded,
          pillar: state.selectedPillarFilter || undefined,
        });
        scheduleRender();
        return;
      }

      if (kosChip) {
        const kosId = kosChip.getAttribute('data-kos-id') || 'all';
        state.selectedKosFilter = kosId;
        scheduleRender();
        return;
      }

      if (clearButton) {
        state.selectedPillarFilter = '';
        state.selectedKosFilter = 'all';
        scheduleRender();
        return;
      } else if (filterButton) {
        const filter = filterButton.getAttribute('data-filter');
        if (!filter) return;
        const selected = new Set(
          (state.selectedPillarFilter || '')
            .split(',')
            .map((pillar) => pillar.trim())
            .filter(Boolean),
        );
        if (selected.has(filter)) selected.delete(filter);
        else selected.add(filter);
        state.selectedPillarFilter = Array.from(selected).join(',');
        scheduleRender();
        return;
      }

      applyDiagnosticsFilterToDom();
    }, true);

    container.addEventListener('change', (event) => {
      const target = event.target as HTMLElement;
      const chkExcluded = target.closest<HTMLInputElement>('#chk-include-excluded');
      if (chkExcluded) {
        state.includeExcluded = chkExcluded.checked;
        callbacks.onRequestIssuesWithOptions?.({
          include_debt: state.selectedDebtScope !== 'active',
          include_excluded: state.includeExcluded,
          pillar: state.selectedPillarFilter || undefined,
        });
        scheduleRender();
      }
    });
  }

  function closeAllEvolutionDropdowns(): void {
    const kosMenu = container.querySelector('#menu-kos-filter-options') as HTMLElement | null;
    if (kosMenu && kosMenu.style.display === 'block') {
      kosMenu.style.display = 'none';
      container.querySelector('#btn-kos-filter-toggle')?.setAttribute('aria-expanded', 'false');
    }
    container.querySelectorAll<HTMLElement>('.page-size-menu').forEach((m) => {
      if (m.style.display === 'block') m.style.display = 'none';
    });
    container.querySelectorAll('.btn-page-size-toggle').forEach((b) => {
      b.setAttribute('aria-expanded', 'false');
    });
    container.querySelectorAll<HTMLElement>('.signatures-page-size-menu').forEach((m) => {
      if (m.style.display === 'block') m.style.display = 'none';
    });
    container.querySelectorAll('.btn-signatures-page-size-toggle').forEach((b) => {
      b.setAttribute('aria-expanded', 'false');
    });
    container.querySelectorAll<HTMLElement>('.rule-level-menu').forEach((m) => {
      if (m.style.display === 'block') m.style.display = 'none';
    });
    container.querySelectorAll('.btn-rule-level-toggle').forEach((b) => {
      b.setAttribute('aria-expanded', 'false');
    });
  }

  function wireExclusionsDelegatedListeners(): void {
    if (container.dataset.exclusionsDelegatedWired === 'true') return;
    container.dataset.exclusionsDelegatedWired = 'true';

    container.addEventListener(
      'click',
      (event) => {
        const target = event.target as HTMLElement | null;
        if (!target) return;

        // Dismiss scope dropdown if clicked outside #scope-dropdown
        if (!target.closest('#scope-dropdown')) {
          const menu = container.querySelector('#menu-scope-options') as HTMLElement | null;
          if (menu && menu.style.display === 'block') {
            menu.style.display = 'none';
            const btn = container.querySelector('#btn-scope-toggle');
            btn?.setAttribute('aria-expanded', 'false');
          }
        }

        // Dismiss prep active dropdown if clicked outside #prep-active-dropdown
        if (!target.closest('#prep-active-dropdown')) {
          const menu = container.querySelector('#menu-prep-active-options') as HTMLElement | null;
          if (menu && menu.style.display === 'block') {
            menu.style.display = 'none';
            const btn = container.querySelector('#btn-prep-active-toggle');
            btn?.setAttribute('aria-expanded', 'false');
          }
        }

        // Dismiss active harness dropdown if clicked outside #active-harness-dropdown
        if (!target.closest('#active-harness-dropdown')) {
          const menu = container.querySelector('#menu-active-harness-options') as HTMLElement | null;
          if (menu && menu.style.display === 'block') {
            menu.style.display = 'none';
            const btn = container.querySelector('#btn-active-harness-toggle');
            btn?.setAttribute('aria-expanded', 'false');
          }
        }

        // Dismiss KOS filter dropdown if clicked outside #kos-discipline-dropdown
        if (!target.closest('#kos-discipline-dropdown')) {
          const menu = container.querySelector('#menu-kos-filter-options') as HTMLElement | null;
          if (menu && menu.style.display === 'block') {
            menu.style.display = 'none';
            container.querySelector('#btn-kos-filter-toggle')?.setAttribute('aria-expanded', 'false');
          }
        }

        // Dismiss page size dropdown if clicked outside .page-size-dropdown
        if (!target.closest('.page-size-dropdown')) {
          container.querySelectorAll<HTMLElement>('.page-size-menu').forEach((m) => {
            if (m.style.display === 'block') m.style.display = 'none';
          });
          container.querySelectorAll('.btn-page-size-toggle').forEach((b) => {
            b.setAttribute('aria-expanded', 'false');
          });
        }

        // Dismiss rule level dropdown if clicked outside .rule-level-dropdown
        if (!target.closest('.rule-level-dropdown')) {
          container.querySelectorAll<HTMLElement>('.rule-level-menu').forEach((m) => {
            if (m.style.display === 'block') m.style.display = 'none';
          });
          container.querySelectorAll('.btn-rule-level-toggle').forEach((b) => {
            b.setAttribute('aria-expanded', 'false');
          });
        }

        // Dismiss split button menus if clicked outside .diag-action-split-btn
        if (!target.closest('.diag-action-split-btn')) {
          closeAllSplitMenus();
        }

        const delBtn = target.closest<HTMLElement>('.rule-del-btn');
        if (delBtn) {
          event.stopPropagation();
          const pat = delBtn.getAttribute('data-rule');
          const scope = (delBtn.getAttribute('data-scope') as 'scan' | 'graph') || 'graph';
          if (pat) {
            callbacks.onDeleteExclusion?.(pat, scope);
            if (scope === 'graph') {
              state.graphExclusions = state.graphExclusions.filter((r) => r !== pat);
            } else {
              state.scanExclusions = state.scanExclusions.filter((r) => r !== pat);
            }
            render();
          }
        }
      },
      true,
    );

    // Also close dropdown when clicking outside container (e.g. document/canvas)
    document.addEventListener('click', (e) => {
      const target = e.target as HTMLElement | null;
      if (!target || !target.closest('#scope-dropdown')) {
        const menu = container.querySelector('#menu-scope-options') as HTMLElement | null;
        if (menu && menu.style.display === 'block') {
          menu.style.display = 'none';
          const btn = container.querySelector('#btn-scope-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
      }
      if (!target || !target.closest('#prep-active-dropdown')) {
        const menu = container.querySelector('#menu-prep-active-options') as HTMLElement | null;
        if (menu && menu.style.display === 'block') {
          menu.style.display = 'none';
          const btn = container.querySelector('#btn-prep-active-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
      }
      if (!target || !target.closest('#active-harness-dropdown')) {
        const menu = container.querySelector('#menu-active-harness-options') as HTMLElement | null;
        if (menu && menu.style.display === 'block') {
          menu.style.display = 'none';
          const btn = container.querySelector('#btn-active-harness-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
      }
      if (!target || !target.closest('#kos-discipline-dropdown')) {
        const menu = container.querySelector('#menu-kos-filter-options') as HTMLElement | null;
        if (menu && menu.style.display === 'block') {
          menu.style.display = 'none';
          container.querySelector('#btn-kos-filter-toggle')?.setAttribute('aria-expanded', 'false');
        }
      }
      if (!target || !target.closest('.page-size-dropdown')) {
        container.querySelectorAll<HTMLElement>('.page-size-menu').forEach((m) => {
          if (m.style.display === 'block') m.style.display = 'none';
        });
        container.querySelectorAll('.btn-page-size-toggle').forEach((b) => {
          b.setAttribute('aria-expanded', 'false');
        });
      }
      if (!target || !target.closest('.rule-level-dropdown')) {
        container.querySelectorAll<HTMLElement>('.rule-level-menu').forEach((m) => {
          if (m.style.display === 'block') m.style.display = 'none';
        });
        container.querySelectorAll('.btn-rule-level-toggle').forEach((b) => {
          b.setAttribute('aria-expanded', 'false');
        });
      }
      if (!target || !target.closest('.diag-action-split-btn')) {
        closeAllSplitMenus();
      }
    });

    // Close dropdown on Escape
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        closeAllSplitMenus();
        const menu = container.querySelector('#menu-scope-options') as HTMLElement | null;
        if (menu && menu.style.display === 'block') {
          menu.style.display = 'none';
          const btn = container.querySelector('#btn-scope-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
        const prepMenu = container.querySelector('#menu-prep-active-options') as HTMLElement | null;
        if (prepMenu && prepMenu.style.display === 'block') {
          prepMenu.style.display = 'none';
          const btn = container.querySelector('#btn-prep-active-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
        const activeMenu = container.querySelector('#menu-active-harness-options') as HTMLElement | null;
        if (activeMenu && activeMenu.style.display === 'block') {
          activeMenu.style.display = 'none';
          const btn = container.querySelector('#btn-active-harness-toggle');
          btn?.setAttribute('aria-expanded', 'false');
        }
        closeAllEvolutionDropdowns();
      }
    });
  }

  function wireDiagnosticsAccordionListeners(): void {
    container.querySelectorAll('.diag-accordion-header').forEach((header) => {
      header.addEventListener('click', (e) => {
        e.stopPropagation();
        const pillar = header.getAttribute('data-pillar');
        const body = pillar ? container.querySelector(`#diag-body-${pillar}`) : null;
        if (!body) return;

        const isCollapsed = header.classList.toggle('collapsed');
        body.classList.toggle('collapsed', isCollapsed);
      });
    });

    container.querySelector<HTMLButtonElement>('#diag-toggle-all')?.addEventListener('click', (e) => {
      e.stopPropagation();
      const headers = Array.from(container.querySelectorAll<HTMLElement>('.diag-accordion-header'));
      const expand = headers.some((header) => header.classList.contains('collapsed'));
      headers.forEach((header) => {
        const pillar = header.getAttribute('data-pillar');
        const body = pillar ? container.querySelector(`#diag-body-${pillar}`) : null;
        header.classList.toggle('collapsed', !expand);
        body?.classList.toggle('collapsed', !expand);
      });
      const button = e.currentTarget as HTMLButtonElement;
      button.textContent = expand ? 'Collapse all' : 'Expand all';
    });
  }

  function wireEvolutionListeners(): void {
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

    // 7. Hard & Soft Constraints Level Custom Dropdowns
    container.querySelectorAll('.btn-rule-level-toggle').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const ruleId = btn.getAttribute('data-rule-id');
        const menu = container.querySelector(`.rule-level-menu[data-rule-id="${ruleId}"]`) as HTMLElement | null;
        if (!menu) return;
        const isHidden = menu.style.display === 'none' || !menu.style.display;
        closeAllEvolutionDropdowns();
        if (isHidden) {
          menu.style.display = 'block';
          btn.setAttribute('aria-expanded', 'true');
        }
      });
    });

    container.querySelectorAll('.rule-level-menu .scope-menu-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const ruleId = item.getAttribute('data-rule-id');
        const level = item.getAttribute('data-value') as ConstraintLevel;
        if (ruleId && state.constraints) {
          const rule = state.constraints.find((r) => r.id === ruleId);
          if (rule) {
            rule.level = level;
            closeAllEvolutionDropdowns();
            callbacks.onValidateConstraints?.(state.constraints);
            render();
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

    // Strategy Selection & Incompatibility Guards (Feature 007 / T019, T020, T021, T027)
    container.querySelectorAll('.strategy-checkbox').forEach((checkbox) => {
      checkbox.addEventListener('change', (e) => {
        e.stopPropagation();
        const target = e.target as HTMLInputElement;
        const oppId = target.getAttribute('data-opp-id');
        const strategy = target.getAttribute('data-strategy');
        if (!oppId || !strategy) return;

        const parentContainer = target.closest(`.strategy-set-container[data-opp-id="${oppId}"]`);
        if (!parentContainer) return;

        const conflictBanner = parentContainer.querySelector(`#conflict-banner-${oppId}`);
        const conflictMsg = conflictBanner?.querySelector('.conflict-msg');

        if (target.checked) {
          // If PreserveDocument was checked: uncheck and disable all other strategies (T020)
          if (strategy === 'preserve_document') {
            parentContainer.querySelectorAll<HTMLInputElement>('.strategy-checkbox').forEach((cb) => {
              if (cb !== target) {
                cb.checked = false;
                cb.disabled = true;
              }
            });
            if (conflictBanner && conflictMsg) {
              conflictMsg.textContent = 'Document preservation selected: Zero transformation actions will be applied.';
              conflictBanner.classList.remove('hidden');
            }
          } else {
            // If another strategy was checked, ensure preserve_document is unchecked
            const preserveCb = parentContainer.querySelector<HTMLInputElement>(
              '.strategy-checkbox[data-strategy="preserve_document"]'
            );
            if (preserveCb && preserveCb.checked) {
              preserveCb.checked = false;
            }

            // Incompatibility resolution (Alternative guard):
            if (strategy === 'faceted_classification') {
              const secCb = parentContainer.querySelector<HTMLInputElement>(
                '.strategy-checkbox[data-strategy="section_addressing"]'
              );
              if (secCb && secCb.checked) {
                secCb.checked = false;
                if (conflictBanner && conflictMsg) {
                  conflictMsg.textContent =
                    "Swapped conflicting strategy. 'Faceted Classification' is an alternative to 'Section Addressing'.";
                  conflictBanner.classList.remove('hidden');
                }
              }
            } else if (strategy === 'section_addressing') {
              const facetCb = parentContainer.querySelector<HTMLInputElement>(
                '.strategy-checkbox[data-strategy="faceted_classification"]'
              );
              if (facetCb && facetCb.checked) {
                facetCb.checked = false;
                if (conflictBanner && conflictMsg) {
                  conflictMsg.textContent =
                    "Swapped conflicting strategy. 'Section Addressing' is an alternative to 'Faceted Classification'.";
                  conflictBanner.classList.remove('hidden');
                }
              }
            }
          }
        } else {
          // If preserve_document was unchecked, re-enable all other checkboxes
          if (strategy === 'preserve_document') {
            parentContainer.querySelectorAll<HTMLInputElement>('.strategy-checkbox').forEach((cb) => {
              cb.disabled = false;
            });
            if (conflictBanner) {
              conflictBanner.classList.add('hidden');
            }
          }
        }

        // Recalculate Composite Plan Value (T027)
        const checkedBoxes = Array.from(
          parentContainer.querySelectorAll<HTMLInputElement>('.strategy-checkbox:checked')
        );
        const selectedStrats = checkedBoxes.map((cb) => cb.getAttribute('data-strategy') || '');

        let baseFitSum = 0;
        for (const cb of checkedBoxes) {
          baseFitSum += parseInt(cb.getAttribute('data-fit') || '0', 10);
        }

        let synergyBonus = 0;
        let overlapPenalty = 0;

        const hasSec = selectedStrats.includes('section_addressing');
        const hasIdx = selectedStrats.includes('lightweight_index');
        const hasFacet = selectedStrats.includes('faceted_classification');

        if (hasSec && hasIdx) {
          synergyBonus += 15;
          overlapPenalty += 5;
        }
        if (hasFacet && hasIdx) {
          synergyBonus += 12;
          overlapPenalty += 3;
        }

        const netValue = baseFitSum + synergyBonus - overlapPenalty;

        const baseFitEl = parentContainer.querySelector(`#base-fit-${oppId}`);
        const synergyEl = parentContainer.querySelector(`#synergy-bonus-${oppId}`);
        const overlapEl = parentContainer.querySelector(`#overlap-penalty-${oppId}`);
        const netValEl = parentContainer.querySelector(`#net-val-badge-${oppId}`);
        const narrativeEl = parentContainer.querySelector(`#plan-narrative-${oppId}`);

        if (baseFitEl) baseFitEl.textContent = String(baseFitSum);
        if (synergyEl) synergyEl.textContent = `+${synergyBonus}`;
        if (overlapEl) overlapEl.textContent = `-${overlapPenalty}`;
        if (netValEl) netValEl.textContent = `Net Value: ${netValue}`;

        if (narrativeEl) {
          narrativeEl.textContent =
            selectedStrats.length > 1 && synergyBonus > overlapPenalty
              ? `Combined plan yields net value ${netValue} with +${synergyBonus} synergy bonus exceeding ${overlapPenalty} overlap overhead.`
              : `Active plan yields net value ${netValue} (fit: ${baseFitSum}).`;
        }

        // Empty selection guard (T021)
        const findingCard = target.closest('.evolution-finding-item') || parentContainer;
        const multiPlanBtn = findingCard.querySelector<HTMLButtonElement>(
          `[data-action="generate-multi-plan"][data-opp-id="${oppId}"], [data-action="generate-multi-plan"]`
        );
        const emptyWarning = parentContainer.querySelector(`#empty-warning-${oppId}`);

        if (checkedBoxes.length === 0) {
          if (multiPlanBtn) multiPlanBtn.disabled = true;
          if (emptyWarning) emptyWarning.classList.remove('hidden');
        } else {
          if (multiPlanBtn) multiPlanBtn.disabled = false;
          if (emptyWarning) emptyWarning.classList.add('hidden');
        }
      });
    });

    // Multi-strategy plan button handler
    container.querySelectorAll('[data-action="generate-multi-plan"]').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const oppId = btn.getAttribute('data-opp-id');
        if (!oppId) return;

        const findingCard = btn.closest('.evolution-finding-item') || btn.closest('.strategy-set-container') || container;
        const strategyContainer = findingCard.querySelector(`.strategy-set-container[data-opp-id="${oppId}"]`) || findingCard.querySelector('.strategy-set-container');

        let selectedStrategies: string[] = [];
        if (strategyContainer) {
          const checkedBoxes = Array.from(
            strategyContainer.querySelectorAll<HTMLInputElement>('.strategy-checkbox:checked')
          );
          selectedStrategies = checkedBoxes
            .map((cb) => cb.getAttribute('data-strategy') || '')
            .filter(Boolean);
        }

        if (selectedStrategies.length === 0) {
          const firstStrategyPill = findingCard.querySelector<HTMLInputElement>('.strategy-checkbox');
          const firstStrat = firstStrategyPill?.getAttribute('data-strategy');
          selectedStrategies = firstStrat ? [firstStrat] : ['preserve_document'];
        }

        callbacks.onGeneratePlanWithStrategies?.(oppId, selectedStrategies);
      });
    });

    // 9. Finding File Link Navigation & Actions
    container.querySelectorAll('.finding-loc, [data-action="open-file"]').forEach((el) => {
      el.addEventListener('click', (e) => {
        e.stopPropagation();
        const dataFile = el.getAttribute('data-file');
        const lineAttr = el.getAttribute('data-line');
        const line = lineAttr ? parseInt(lineAttr, 10) : undefined;
        if (dataFile) {
          callbacks.onOpenFile?.(dataFile, line);
          return;
        }
        const pathSpan = el.querySelector('.finding-path');
        const text = pathSpan?.textContent?.trim();
        if (text) {
          const [file, l] = text.split(':');
          callbacks.onOpenFile?.(file, l ? parseInt(l, 10) : undefined);
        }
      });
    });

    // 9a. Open Target Navigation
    container.querySelectorAll('[data-action="open-target"], .btn-finding-open-target, .finding-target-link').forEach((el) => {
      el.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = el.getAttribute('data-target') || el.getAttribute('data-file');
        if (target) {
          callbacks.onOpenFile?.(target);
        }
      });
    });

    // 9b. Broken Reference Fix Action
    container.querySelectorAll('[data-action="fix-broken"]').forEach((btn) => {
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

    // 9c. Review with AI Action
    container.querySelectorAll('[data-action="review-with-ai"], [data-action="mark-reviewed"], .diag-mark-reviewed-btn, .btn-mark-reviewed, .btn-finding-review-ai').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        closeAllSplitMenus();
        const docId = btn.getAttribute('data-file') || btn.getAttribute('data-doc-id');
        const issueRaw = btn.getAttribute('data-issue');
        let issue: any = undefined;
        if (issueRaw) {
          try {
            issue = JSON.parse(decodeURIComponent(issueRaw));
          } catch {}
        }
        if (callbacks.onReviewWithAi) {
          callbacks.onReviewWithAi(docId || '', issue);
        } else if (docId) {
          callbacks.onMarkReviewed?.(docId);
        }
      });
    });

    // 10. Secondary Routing Layer Elimination (Safely Remove)
    container.querySelectorAll('.btn-gov-eliminate-secondary').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('eliminate_secondary_routing_layer', target);
        }
      });
    });

    // 11. Multi-Harness Parity Synchronization
    container.querySelectorAll('.btn-gov-sync-harnesses').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || 'AGENTS.md';
        callbacks.onGenerateGovernancePlan?.('sync_harness_routes', target);
      });
    });

    container.querySelectorAll('.btn-gov-normalize-links').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const target = (btn as HTMLElement).getAttribute('data-target') || '';
        if (target) {
          callbacks.onGenerateGovernancePlan?.('normalize_links_to_markdown', target);
        }
      });
    });

    // 12. 18 KOS Methodology Filter (Custom Popover Dropdown & Chips)
    const kosBtn = container.querySelector('#btn-kos-filter-toggle');
    const kosMenu = container.querySelector('#menu-kos-filter-options') as HTMLElement | null;

    kosBtn?.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();
      if (!kosMenu) return;
      const isHidden = kosMenu.style.display === 'none' || !kosMenu.style.display;
      closeAllEvolutionDropdowns();
      if (isHidden) {
        kosMenu.style.display = 'block';
        kosBtn.setAttribute('aria-expanded', 'true');
      }
    });

    kosMenu?.querySelectorAll('.scope-menu-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const val = item.getAttribute('data-value') || 'all';
        state.selectedKosFilter = val;
        state.evolutionFindingsPage = 1;
        closeAllEvolutionDropdowns();
        scheduleRender();
      });
    });

    container.querySelectorAll<HTMLElement>('.kos-filter-chip').forEach((chip) => {
      chip.addEventListener('click', (e) => {
        e.stopPropagation();
        const kosId = chip.getAttribute('data-kos-id') || 'all';
        state.selectedKosFilter = kosId;
        state.evolutionFindingsPage = 1;
        closeAllEvolutionDropdowns();
        scheduleRender();
      });
    });

    // 13. Findings Pagination Controls (Custom Page Size Dropdown & Navigation)
    container.querySelectorAll('.btn-page-size-toggle').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const pos = btn.getAttribute('data-position') || 'top';
        const menu = container.querySelector(`.page-size-menu[data-position="${pos}"]`) as HTMLElement | null;
        if (!menu) return;
        const isHidden = menu.style.display === 'none' || !menu.style.display;
        closeAllEvolutionDropdowns();
        if (isHidden) {
          menu.style.display = 'block';
          btn.setAttribute('aria-expanded', 'true');
        }
      });
    });

    container.querySelectorAll('.page-size-menu .scope-menu-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const valStr = item.getAttribute('data-value');
        const newSize = parseInt(valStr || '25', 10);
        if (newSize && [10, 25, 50].includes(newSize)) {
          state.evolutionFindingsPageSize = newSize;
          state.evolutionFindingsPage = 1;
          closeAllEvolutionDropdowns();
          scheduleRender();
        }
      });
    });

    container.querySelectorAll<HTMLButtonElement>('.btn-page-nav').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const targetPage = parseInt(btn.getAttribute('data-page') || '1', 10);
        if (!isNaN(targetPage) && targetPage >= 1) {
          state.evolutionFindingsPage = targetPage;
          scheduleRender();
        }
      });
    });

    // 14. Quality Pillar Filter Chips (Findings)
    container.querySelectorAll<HTMLElement>('.findings-pillar-chip').forEach((chip) => {
      chip.addEventListener('click', (e) => {
        e.stopPropagation();
        const pillarId = chip.getAttribute('data-pillar-id') || 'all';
        state.evolutionPillarFilter = pillarId;
        state.evolutionFindingsPage = 1;
        closeAllEvolutionDropdowns();
        scheduleRender();
      });
    });

    // 15. Opportunity Type Filter Chips (Problem Signatures)
    container.querySelectorAll<HTMLElement>('.signature-filter-chip').forEach((chip) => {
      chip.addEventListener('click', (e) => {
        e.stopPropagation();
        const sigType = chip.getAttribute('data-sig-type') || 'all';
        state.selectedSignatureTypeFilter = sigType;
        state.evolutionSignaturesPage = 1;
        closeAllEvolutionDropdowns();
        scheduleRender();
      });
    });

    // 16. Structural Opportunities Pagination (Page Size & Nav)
    container.querySelectorAll('.btn-signatures-page-size-toggle').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const pos = btn.getAttribute('data-position') || 'top';
        const menu = container.querySelector(`.signatures-page-size-menu[data-position="${pos}"]`) as HTMLElement | null;
        if (!menu) return;
        const isHidden = menu.style.display === 'none' || !menu.style.display;
        closeAllEvolutionDropdowns();
        if (isHidden) {
          menu.style.display = 'block';
          btn.setAttribute('aria-expanded', 'true');
        }
      });
    });

    container.querySelectorAll('.signatures-page-size-menu .scope-menu-item').forEach((item) => {
      item.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const valStr = item.getAttribute('data-value');
        const newSize = parseInt(valStr || '10', 10);
        if (newSize && [10, 25, 50].includes(newSize)) {
          state.evolutionSignaturesPageSize = newSize;
          state.evolutionSignaturesPage = 1;
          closeAllEvolutionDropdowns();
          scheduleRender();
        }
      });
    });

    container.querySelectorAll<HTMLButtonElement>('.btn-signatures-page-nav').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const targetPage = parseInt(btn.getAttribute('data-page') || '1', 10);
        if (!isNaN(targetPage) && targetPage >= 1) {
          state.evolutionSignaturesPage = targetPage;
          scheduleRender();
        }
      });
    });
  }

  function open(tab?: SettingsTab): void {
    const wasVisible = isVisible;
    const prevTab = state.activeTab;
    if (tab) state.activeTab = tab;
    isVisible = true;
    callbacks.onOpen?.();

    // Skip full re-render if already visible on the same tab (deduplicate rapid open calls)
    if (wasVisible && state.activeTab === prevTab) {
      return;
    }

    // Reset scroll when switching tabs — destroy and recreate the content element
    // so the new tab starts at the top (handled naturally by full render())
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

    if (state.activeTab === 'exclusions') {
      callbacks.onRequestExclusions?.();
    }
    if (state.activeTab === 'diagnostics' || state.activeTab === 'evolution') {
      if (!state.issuesReport) {
        callbacks.onRequestIssues?.(state.selectedPillarFilter);
      }
      if (state.activeTab === 'evolution') {
        callbacks.onRequestEvolution?.();
      }
    }
    if (state.activeTab === 'git' || state.activeTab === 'harness' || state.activeTab === 'git_harness') {
      callbacks.onRequestHarnessTools?.();
      callbacks.onRequestHarnessStatus?.();
    }
    render();
  }

  function close(): void {
    isVisible = false;
    isEntering = false;
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

  wireDiagnosticsFilterListeners();
  wireExclusionsDelegatedListeners();
  render();
  wireInfoBubbleListeners(container);

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
