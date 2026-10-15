/**
 * Informational Tooltip Component (T051)
 *
 * Implements FR-011, FR-013, SC-002 (US2).
 * Displays rich metadata for Tier 1 knobs:
 * - Plain language description
 * - High vs Low impact
 * - Affected problem signatures
 * - Countervailing trade-offs
 * - Associated Tier 2 sliders
 */

export interface KnobTooltipDetails {
  key: string;
  name: string;
  description: string;
  higherImpact: string;
  lowerImpact: string;
  affectedProblems: string[];
  tradeOff: string;
  affectedTier2Keys: string[];
}

export const TIER1_TOOLTIPS: Record<string, KnobTooltipDetails> = {
  aiDiscoverability: {
    key: 'aiDiscoverability',
    name: 'AI Discoverability',
    description: 'Prioritizes rapid discovery of relevant concepts by LLM agents from entry hubs.',
    higherImpact: 'Generates structured index routers; boosts discovery scores for nested notes.',
    lowerImpact: 'Relies on existing manual link topology with fewer generated hubs.',
    affectedProblems: ['Isolated Silo', 'Fragmented Concept'],
    tradeOff: 'Increases navigational hub count and repository file footprint.',
    affectedTier2Keys: ['candidateExpansionFactor', 'maxTraceDepth'],
  },
  contextEfficiency: {
    key: 'contextEfficiency',
    name: 'Context Efficiency',
    description: 'Minimizes irrelevant token retrieval during automated Context Trace beam search.',
    higherImpact: 'Enforces smaller fragment sizes and aggressive section-level anchor targeting.',
    lowerImpact: 'Loads entire documents into prompt context without section filtering.',
    affectedProblems: ['Structural God Document'],
    tradeOff: 'Increases document fragmentation and frontmatter metadata overhead.',
    affectedTier2Keys: ['fragmentSizeTarget', 'maxContextSurfaceTokens'],
  },
  humanReadability: {
    key: 'humanReadability',
    name: 'Human Readability',
    description: 'Optimizes document layout for comfortable human reading and visual clarity.',
    higherImpact: 'Favors longer cohesive chapters with natural prose narrative continuity.',
    lowerImpact: 'Slices documents into granular micro-notes requiring frequent tab switching.',
    affectedProblems: ['Fragmented Concept'],
    tradeOff: 'Decreases agent context packing efficiency when querying specific subpoints.',
    affectedTier2Keys: ['fragmentSizeTarget', 'cohesionThreshold'],
  },
  knowledgeReuse: {
    key: 'knowledgeReuse',
    name: 'Knowledge Reuse',
    description: 'Promotes modular concepts that can be referenced across multiple contexts.',
    higherImpact: 'Introduces section anchors and faceted frontmatter taxonomy for clean re-linking.',
    lowerImpact: 'Allows monolithic inline duplication of conceptual definitions.',
    affectedProblems: ['Structural God Document', 'Fragmented Concept'],
    tradeOff: 'Anchor refactor fragility when section titles change.',
    affectedTier2Keys: ['referenceConcentrationThreshold', 'splitSensitivity'],
  },
  narrativeCoherence: {
    key: 'narrativeCoherence',
    name: 'Narrative Coherence',
    description: 'Preserves long-form authorial structure and single-document architectural narratives.',
    higherImpact: 'Strictly penalizes topic decomposition; protects healthy 50k+ monoliths.',
    lowerImpact: 'Encourages automated topic decomposition into multi-file collections.',
    affectedProblems: ['Healthy Monolith (protection)'],
    tradeOff: 'Incurs context inefficiency overhead when LLMs load complete reference guides.',
    affectedTier2Keys: ['splitSensitivity', 'cohesionThreshold'],
  },
  maintenanceSimplicity: {
    key: 'maintenanceSimplicity',
    name: 'Maintenance Simplicity',
    description: 'Minimizes friction in day-to-day authoring and Git collaboration.',
    higherImpact: 'Avoids complex metadata taxonomies and rigid link integrity constraints.',
    lowerImpact: 'Requires materialized fragment frontmatter and strict schema validation.',
    affectedProblems: ['Maintenance Coupling'],
    tradeOff: 'Lower automated structural query precision across the vault.',
    affectedTier2Keys: ['duplicateTolerance', 'canonicalOwnershipSensitivity'],
  },
};

export function renderKnobTooltipHtml(knobKey: string): string {
  const details = TIER1_TOOLTIPS[knobKey];
  if (!details) return '';

  return `
    <div class="knob-tooltip-wrapper knob-tooltip-trigger" data-knob="${knobKey}" tabindex="0" role="tooltip" aria-label="${details.name} Info">
      <span class="tooltip-icon knob-tooltip-icon" aria-hidden="true">i</span>
      <div class="knob-tooltip-popover">
        <div class="tooltip-popover-title">${details.name}</div>
        <p class="tooltip-popover-desc">${details.description}</p>
        <div class="tooltip-impact-section">
          <div class="impact-row"><span class="impact-label">▲ Higher:</span> ${details.higherImpact}</div>
          <div class="impact-row"><span class="impact-label">▼ Lower:</span> ${details.lowerImpact}</div>
        </div>
        <div class="tooltip-tradeoff-section">
          <span class="tradeoff-badge">Trade-off:</span> ${details.tradeOff}
        </div>
        <div class="tooltip-tier2-links">
          <span class="tier2-link-label">Affected Tier 2 parameters:</span>
          ${details.affectedTier2Keys.map((k) => `<span class="tier2-tag" data-target-slider="${k}">${k}</span>`).join(' ')}
        </div>
      </div>
    </div>
  `;
}
