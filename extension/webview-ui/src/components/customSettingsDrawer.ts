/**
 * CustomSettingsDrawer Component (T050)
 *
 * Implements FR-009, FR-010, FR-012, FR-013, US2.
 * Renders:
 * - 6 Tier 1 business knobs (Low / Med / High) with informational tooltips (T051)
 * - 11 Tier 2 technical sliders with dependency mapping
 * - Constraint rules with conflict banner (T052)
 */

import type {
  Tier1Objectives,
  Tier2Parameters,
  ConstraintRule,
  ConstraintConflict,
  ObjectiveLevel,
  ConstraintLevel,
} from '../../../src/types/evolution.ts';
import { renderKnobTooltipHtml } from './tooltip.ts';
import { renderConflictBannerHtml } from './conflictBanner.ts';

export interface CustomSettingsProps {
  tier1: Tier1Objectives;
  tier2: Tier2Parameters;
  constraints: ConstraintRule[];
  conflicts: ConstraintConflict[];
  isSaving?: boolean;
}

export const TIER1_KNOB_DEFS: Array<{ key: keyof Tier1Objectives; label: string }> = [
  { key: 'aiDiscoverability', label: 'AI Discoverability' },
  { key: 'contextEfficiency', label: 'Context Efficiency' },
  { key: 'humanReadability', label: 'Human Readability' },
  { key: 'knowledgeReuse', label: 'Knowledge Reuse' },
  { key: 'narrativeCoherence', label: 'Narrative Coherence' },
  { key: 'maintenanceSimplicity', label: 'Maintenance Simplicity' },
];

export const TIER2_SLIDER_DEFS: Array<{
  key: keyof Tier2Parameters;
  label: string;
  min: number;
  max: number;
  step: number;
  unit: string;
}> = [
  { key: 'cohesionThreshold', label: 'Cohesion Threshold', min: 0.1, max: 1.0, step: 0.05, unit: '' },
  { key: 'semanticDiversityThreshold', label: 'Semantic Diversity', min: 0.1, max: 1.0, step: 0.05, unit: '' },
  { key: 'referenceConcentrationThreshold', label: 'Ref. Concentration', min: 0.1, max: 1.0, step: 0.05, unit: '' },
  { key: 'fragmentSizeTarget', label: 'Fragment Size Target', min: 1000, max: 15000, step: 500, unit: 'tok' },
  { key: 'maxTraceDepth', label: 'Max Trace Depth', min: 1, max: 10, step: 1, unit: 'hops' },
  { key: 'candidateExpansionFactor', label: 'Candidate Expansion', min: 1.0, max: 3.0, step: 0.1, unit: 'x' },
  { key: 'maxContextSurfaceTokens', label: 'Max Context Surface', min: 2000, max: 50000, step: 1000, unit: 'tok' },
  { key: 'splitSensitivity', label: 'Split Sensitivity', min: 0.0, max: 1.0, step: 0.05, unit: '' },
  { key: 'canonicalOwnershipSensitivity', label: 'Canonical Ownership', min: 0.0, max: 1.0, step: 0.05, unit: '' },
  { key: 'duplicateTolerance', label: 'Duplicate Tolerance', min: 0.0, max: 1.0, step: 0.05, unit: '' },
  { key: 'fragmentationTolerance', label: 'Fragmentation Tolerance', min: 0.0, max: 1.0, step: 0.05, unit: '' },
];

export function renderCustomSettingsDrawerHtml(props: CustomSettingsProps): string {
  const levels: ObjectiveLevel[] = ['low', 'medium', 'high'];
  const t1 = (props.tier1 || {}) as any;
  const t2 = (props.tier2 || {}) as any;

  // Tier 1 Knobs
  const tier1Html = TIER1_KNOB_DEFS.map((knob) => {
    const snakeKey = knob.key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
    const currentVal = t1[knob.key] || t1[snakeKey] || 'medium';
    const buttonsHtml = levels.map((lvl) => {
      const activeClass = currentVal === lvl ? 'tier1-btn-active' : '';
      return `
        <button type="button" class="tier1-level-btn ${activeClass}" data-knob="${knob.key}" data-level="${lvl}">
          ${lvl.charAt(0).toUpperCase() + lvl.slice(1)}
        </button>
      `;
    }).join('');

    return `
      <div class="tier1-knob-row">
        <div class="knob-label-area">
          <span class="knob-label">${knob.label}</span>
          ${renderKnobTooltipHtml(knob.key)}
        </div>
        <div class="tier1-button-group">
          ${buttonsHtml}
        </div>
      </div>
    `;
  }).join('');

  // Tier 2 Sliders
  const tier2Html = TIER2_SLIDER_DEFS.map((slider) => {
    const snakeKey = slider.key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
    let val = t2[slider.key];
    if (val === undefined || val === null) {
      val = t2[snakeKey];
    }
    if (val === undefined || val === null || isNaN(Number(val))) {
      val = (DEFAULT_TIER2_PARAMETERS as any)[slider.key] ?? slider.min;
    }
    return `
      <div class="tier2-slider-row" id="slider-container-${slider.key}">
        <div class="slider-header">
          <label for="slider-${slider.key}" class="slider-label">${slider.label}</label>
          <span class="slider-value-badge" id="val-${slider.key}">${val}${slider.unit ? ' ' + slider.unit : ''}</span>
        </div>
        <input
          type="range"
          id="slider-${slider.key}"
          class="tier2-slider-input"
          data-param="${slider.key}"
          min="${slider.min}"
          max="${slider.max}"
          step="${slider.step}"
          value="${val}"
        />
      </div>
    `;
  }).join('');

  // Constraint Rules
  const constraintLevels: ConstraintLevel[] = ['preference', 'constraint', 'hard_constraint'];
  const formatConstraintLevel = (lvl: string) =>
    lvl.split('_').map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
  const constraintsList = Array.isArray(props.constraints) ? props.constraints : [];
  const rulesHtml = constraintsList.map((rawRule: any) => {
    const ruleId = rawRule.id || '';
    const ruleKey = rawRule.ruleKey || rawRule.rule_key || '';
    const ruleDesc = rawRule.description || '';
    const currentLevel = rawRule.level || 'preference';

    const optionsHtml = constraintLevels.map((lvl) => {
      const selected = currentLevel === lvl ? 'selected' : '';
      return `<option value="${lvl}" ${selected}>${formatConstraintLevel(lvl)}</option>`;
    }).join('');

    return `
      <div class="constraint-rule-row" data-rule-id="${ruleId}">
        <div class="rule-info">
          <span class="rule-key"><code>${ruleKey}</code></span>
          <span class="rule-desc">${ruleDesc}</span>
        </div>
        <div class="rule-actions">
          <div class="settings-scope-dropdown rule-level-dropdown" data-rule-id="${ruleId}">
            <button type="button" class="settings-scope-btn rule-level-btn btn-rule-level-toggle" data-rule-id="${ruleId}" aria-haspopup="listbox" aria-expanded="false" title="Constraint Enforcement Level">
              <span class="label-rule-level-selected" data-rule-id="${ruleId}">${formatConstraintLevel(currentLevel)}</span>
              <span class="scope-dropdown-arrow">▾</span>
            </button>
            <div class="settings-scope-menu rule-level-menu" id="menu-rule-level-${ruleId}" data-rule-id="${ruleId}" style="display: none;" role="listbox">
              ${constraintLevels.map((lvl) => `
                <div class="scope-menu-item ${currentLevel === lvl ? 'active' : ''}" data-value="${lvl}" data-rule-id="${ruleId}" role="option" aria-selected="${currentLevel === lvl}">
                  <span class="scope-item-check" style="${currentLevel === lvl ? '' : 'visibility: hidden;'}">✓</span>
                  <span class="scope-item-title">${formatConstraintLevel(lvl)}</span>
                </div>
              `).join('')}
            </div>
            <input type="hidden" class="rule-level-select" data-rule-id="${ruleId}" value="${currentLevel}" />
          </div>
        </div>
      </div>
    `;
  }).join('');

  // Conflict Banner (T052)
  const conflictsList = Array.isArray(props.conflicts) ? props.conflicts : [];
  const conflictBannerHtml = renderConflictBannerHtml({ conflicts: conflictsList });

  return `
    <div class="custom-settings-drawer-container">
      <div class="custom-settings-header">
        <h2>Custom KOS Profile Configuration</h2>
        <p class="custom-subtitle">Configure 2-tier architectural objectives and SAT-checked hard constraints.</p>
      </div>

      ${conflictBannerHtml}

      <div class="custom-section">
        <h3 class="section-title">Tier 1: High-Level Objectives</h3>
        <p class="section-desc">Discrete knobs translating human organizational intent into technical parameters.</p>
        <div class="tier1-knobs-list">
          ${tier1Html}
        </div>
      </div>

      <div class="custom-section">
        <div class="section-title-with-toggle">
          <h3 class="section-title">Tier 2: Advanced Parameters</h3>
          <span class="dependency-indicator">🔗 Mapped from Tier 1</span>
        </div>
        <p class="section-desc">Fine-grained numerical weights directly governing graph analytics and KOS strategy evaluation.</p>
        <div class="tier2-sliders-list">
          ${tier2Html}
        </div>
      </div>

      <div class="custom-section">
        <h3 class="section-title">Hard & Soft Constraints</h3>
        <p class="section-desc">Rules disqualifying or penalizing candidate strategies during optimization.</p>
        <div class="constraints-list">
          ${rulesHtml}
        </div>
      </div>

      <div class="custom-actions-bar">
        <button type="button" class="btn btn-secondary" id="btn-reset-custom">Reset Defaults</button>
        <button type="button" class="btn btn-primary" id="btn-save-custom">
          ${props.isSaving ? 'Saving...' : 'Save & Compile Profile'}
        </button>
      </div>
    </div>
  `;
}
