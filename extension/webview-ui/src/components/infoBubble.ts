/**
 * Informational Bubble Component (T035)
 *
 * Implements FR-026, SC-005, US4.
 * Every structural metric, parameter, and strategy fit score answers all 5 mandatory questions:
 * 1. Meaning: What does this mean?
 * 2. Implications: What does a higher/lower value imply?
 * 3. Importance: Why does it matter?
 * 4. Affects: What does it affect?
 * 5. Trade-off: What trade-off does it introduce?
 */

export interface InfoBubbleDefinition {
  title: string;
  meaning: string;
  implications: string;
  importance: string;
  affects: string;
  tradeOff: string;
}

export const INFO_BUBBLE_DEFINITIONS: Record<string, InfoBubbleDefinition> = {
  cohesion: {
    title: 'Cohesion',
    meaning: 'The degree to which headings, concepts, and code citations within a document address a single, unified topic.',
    implications: 'Higher values indicate focused, thematic content; lower values indicate multi-topic sprawling notes.',
    importance: 'High cohesion ensures that retrieving a document loads relevant context rather than unrelated background noise.',
    affects: 'Candidate expansion in Context Trace and topic decomposition recommendations.',
    tradeOff: 'Maximizing cohesion on complex topics can increase document fragmentation.',
  },
  coupling: {
    title: 'Coupling',
    meaning: 'The density of inter-document reference dependencies between this document and others.',
    implications: 'Higher coupling implies tight interdependence; lower coupling implies self-contained knowledge units.',
    importance: 'High coupling increases the risk of ripple-effect link breakage when files are renamed or refactored.',
    affects: 'Maintenance effort estimates and impact radii.',
    tradeOff: 'Extremely low coupling isolates documents, preventing graph traversability.',
  },
  reference_concentration: {
    title: 'Reference Concentration',
    meaning: 'The proportion of inbound references targeting a small subset of sections within a document (C = inbound_fragment / inbound_doc).',
    implications: 'High concentration (≥ 70%) means agents repeatedly visit a monolithic note for only 1 or 2 specific sections.',
    importance: 'Key indicator of context waste. Suggests section addressing or index routing.',
    affects: 'Strategy fit scores for Section Addressing and Lightweight Index Routers.',
    tradeOff: 'Section-level addressing increases URI anchor precision but requires stable heading names.',
  },
  granularity: {
    title: 'Granularity',
    meaning: 'The ratio of document token size to structural section depth.',
    implications: 'Higher granularity implies fine-grained micro-notes; lower granularity implies long-form monolithic documents.',
    importance: 'Dictates chunk sizes during AI context injection.',
    affects: 'Token consumption vs navigational hops.',
    tradeOff: 'High granularity reduces per-document token cost but increases graph traversal depth.',
  },
  context_efficiency: {
    title: 'Context Efficiency',
    meaning: 'The ratio of directly relevant tokens to total tokens loaded during knowledge traversal.',
    implications: 'Higher values indicate that agents consume minimal irrelevant tokens to fulfill tasks.',
    importance: 'Prevents LLM context saturation and reduces API costs / inference latency.',
    affects: 'Context Trace depth penalties and section-level extraction triggers.',
    tradeOff: 'May favor more granular structures and require more explicit link indexing.',
  },
  discoverability: {
    title: 'Discoverability',
    meaning: 'The mathematical ease with which relevant knowledge can be reached from entry points (e.g. AGENTS.md).',
    implications: 'Higher values mean short, high-relevance traversal paths; lower values mean buried or isolated content.',
    importance: 'Ensures AI agents and engineers find canonical answers without exhaustive repository searches.',
    affects: 'Beam search candidate generation and index router recommendations.',
    tradeOff: 'Maximizing discoverability requires maintaining curated hubs and index directories.',
  },
  knowledge_reuse: {
    title: 'Knowledge Reuse',
    meaning: 'The degree to which individual sections or concepts are cross-referenced across multiple workflows.',
    implications: 'High reuse indicates core shared architecture specifications or shared operational rules.',
    importance: 'Identifies single sources of truth that must be protected against canonical drift.',
    affects: 'Canonical candidate identification and collision warnings.',
    tradeOff: 'High reuse components require stricter change control to avoid widespread breakage.',
  },
  semantic_diversity: {
    title: 'Semantic Diversity',
    meaning: 'The variety of distinct vocabulary terms, topic clusters, and code namespaces contained in a single document.',
    implications: 'High diversity within a single file suggests it combines unrelated architectural domains.',
    importance: 'Differentiates healthy monoliths (low diversity, single topic) from overloaded "God Documents" (high diversity, multiple topics).',
    affects: 'Structural problem signature detection.',
    tradeOff: 'Artificially suppressing diversity can break natural narrative explanations.',
  },
  navigational_complexity: {
    title: 'Navigational Complexity',
    meaning: 'The cognitive and computational path length (hops) required to traverse between related concepts.',
    implications: 'High complexity means users and agents must follow convoluted reference chains.',
    importance: 'Direct driver of user frustration and agent lost-in-the-middle phenomena.',
    affects: 'Centrality re-weighting and router synthesis.',
    tradeOff: 'Flattening navigation completely creates giant index files with massive link fan-outs.',
  },
  maintenance_coupling: {
    title: 'Maintenance Coupling',
    meaning: 'The likelihood that changes to code or schema require simultaneous edits across multiple documentation files.',
    implications: 'Higher values mean documentation easily drifts out of sync during software development.',
    importance: 'Minimizes documentation rot and brownfield debt.',
    affects: 'Freshness pillar scoring and reverse code-doc impact alerts.',
    tradeOff: 'Reducing maintenance coupling may require delegating implementation details to inline docstrings.',
  },
  fit_score: {
    title: 'Strategy Fit Score',
    meaning: 'Mathematical alignment of the candidate strategy with the active documentation orientation and configured constraints.',
    implications: 'Higher percentages indicate strong alignment with optimization goals without violating constraints.',
    importance: 'Explains why the engine prioritizes this specific architectural pattern over alternatives.',
    affects: 'Recommendation ranking and default suggested transformation.',
    tradeOff: 'High-fit strategies may still introduce operational trade-offs shown in the comparison matrix.',
  },
};

// Aliases for camelCase or variant keys
INFO_BUBBLE_DEFINITIONS.referenceConcentration = INFO_BUBBLE_DEFINITIONS.reference_concentration;
INFO_BUBBLE_DEFINITIONS.contextEfficiency = INFO_BUBBLE_DEFINITIONS.context_efficiency;
INFO_BUBBLE_DEFINITIONS.semanticDiversity = INFO_BUBBLE_DEFINITIONS.semantic_diversity;
INFO_BUBBLE_DEFINITIONS.navigationalComplexity = INFO_BUBBLE_DEFINITIONS.navigational_complexity;
INFO_BUBBLE_DEFINITIONS.maintenanceCoupling = INFO_BUBBLE_DEFINITIONS.maintenance_coupling;
INFO_BUBBLE_DEFINITIONS.reusability = INFO_BUBBLE_DEFINITIONS.knowledge_reuse;

function buildBubbleHtml(def: InfoBubbleDefinition, title: string): string {
  return `<span class="info-bubble-wrapper" tabindex="0" role="tooltip" aria-label="Informational details for ${title}"><span class="info-bubble-icon" aria-hidden="true">ⓘ</span><div class="info-bubble-tooltip"><div class="info-bubble-title">${title}</div><div class="info-bubble-section"><span class="info-bubble-sec-label">Meaning:</span> <span class="info-bubble-sec-content">${def.meaning}</span></div><div class="info-bubble-section"><span class="info-bubble-sec-label">Implications:</span> <span class="info-bubble-sec-content">${def.implications}</span></div><div class="info-bubble-section"><span class="info-bubble-sec-label">Importance:</span> <span class="info-bubble-sec-content">${def.importance}</span></div><div class="info-bubble-section"><span class="info-bubble-sec-label">Affects:</span> <span class="info-bubble-sec-content">${def.affects}</span></div><div class="info-bubble-section info-bubble-tradeoff"><span class="info-bubble-sec-label" style="color: #f59e0b;">Trade-off:</span> <span class="info-bubble-sec-content">${def.tradeOff}</span></div></div></span>`;
}

const STATIC_BUBBLE_CACHE: Record<string, string> = {};
for (const [key, def] of Object.entries(INFO_BUBBLE_DEFINITIONS)) {
  STATIC_BUBBLE_CACHE[key] = buildBubbleHtml(def, def.title);
}

/**
 * Renders an informational bubble HTML element answering all 5 mandatory questions.
 * Uses a precomputed static cache for O(1) performance during multi-card 60 FPS renders.
 */
export function renderInfoBubbleHtml(dimensionKey: string, customTitle?: string): string {
  if (!customTitle && STATIC_BUBBLE_CACHE[dimensionKey]) {
    return STATIC_BUBBLE_CACHE[dimensionKey];
  }

  const def = INFO_BUBBLE_DEFINITIONS[dimensionKey];
  if (!def) {
    return '';
  }

  const title = customTitle || def.title;
  return buildBubbleHtml(def, title);
}

/**
 * Dynamically adjusts the tooltip's horizontal and vertical position relative to its
 * scroll/drawer container to prevent clipping by the drawer's left border, right screen edge,
 * or top header bar.
 */
export function adjustTooltipPosition(wrapper: HTMLElement): void {
  if (!wrapper || typeof wrapper.querySelector !== 'function') return;

  const tooltip = (wrapper.querySelector('.info-bubble-tooltip') ||
                   wrapper.querySelector('.knob-tooltip-popover')) as HTMLElement | null;
  if (!tooltip) return;

  // 1. Temporarily display to measure natural dimensions
  const initialDisplay = tooltip.style.display;
  const initialVisibility = tooltip.style.visibility;
  tooltip.style.display = 'flex';
  tooltip.style.visibility = 'hidden';

  // Reset transforms and positions to baseline for clean measurement
  tooltip.style.left = '50%';
  tooltip.style.right = 'auto';
  tooltip.style.transform = 'translateX(-50%)';

  // 2. Identify clipping container boundaries
  const container = (wrapper.closest('.settings-content') ||
                     wrapper.closest('.settings-drawer') ||
                     document.body) as HTMLElement;
  const cRect = container.getBoundingClientRect();
  const tRect = tooltip.getBoundingClientRect();
  const wRect = wrapper.getBoundingClientRect();

  // Headless test guard
  if (cRect.width === 0 || tRect.width === 0) {
    tooltip.style.visibility = initialVisibility;
    tooltip.style.display = initialDisplay;
    return;
  }

  // Safety margins
  const horizontalPadding = 12;
  const minLeft = cRect.left + horizontalPadding;
  const maxRight = cRect.right - horizontalPadding;

  // 3. Constrain width if container is narrow
  const availableWidth = cRect.width - (horizontalPadding * 2);
  if (tRect.width > availableWidth && availableWidth > 120) {
    tooltip.style.maxWidth = `${Math.floor(availableWidth)}px`;
    tooltip.style.width = 'auto';
  }

  const updatedTRect = tooltip.getBoundingClientRect();

  // 4. Horizontal boundary clamping
  if (updatedTRect.left < minLeft) {
    // Overflows left boundary: shift rightward
    const shift = minLeft - updatedTRect.left;
    tooltip.style.transform = `translateX(calc(-50% + ${Math.ceil(shift)}px))`;
  } else if (updatedTRect.right > maxRight) {
    // Overflows right boundary: shift leftward
    const shift = updatedTRect.right - maxRight;
    tooltip.style.transform = `translateX(calc(-50% - ${Math.ceil(shift)}px))`;
  } else {
    tooltip.style.transform = 'translateX(-50%)';
  }

  // 5. Vertical boundary adjustment (flip below if not enough space on top)
  const spaceAbove = wRect.top - cRect.top;
  const requiredHeight = updatedTRect.height + 12;

  if (spaceAbove < requiredHeight) {
    tooltip.classList.add('tooltip-flipped');
    tooltip.style.bottom = 'auto';
    tooltip.style.top = 'calc(100% + 6px)';
  } else {
    tooltip.classList.remove('tooltip-flipped');
    tooltip.style.bottom = 'calc(100% + 6px)';
    tooltip.style.top = 'auto';
  }

  // 6. Make tooltip visible
  tooltip.style.visibility = 'visible';
}

/**
 * Resets inline styles applied by dynamic tooltip positioning.
 */
export function resetTooltipPosition(wrapper: HTMLElement): void {
  if (!wrapper || typeof wrapper.querySelector !== 'function') return;

  const tooltip = (wrapper.querySelector('.info-bubble-tooltip') ||
                   wrapper.querySelector('.knob-tooltip-popover')) as HTMLElement | null;
  if (!tooltip) return;

  tooltip.style.display = '';
  tooltip.style.visibility = '';
  tooltip.style.transform = '';
  tooltip.style.left = '';
  tooltip.style.right = '';
  tooltip.style.top = '';
  tooltip.style.bottom = '';
  tooltip.style.maxWidth = '';
  tooltip.style.width = '';
  tooltip.classList.remove('tooltip-flipped');
}

/**
 * Registers delegated hover, focus, blur, scroll, and escape-key listeners on a container
 * to ensure all informational tooltips (ⓘ) adjust dynamically without clipping.
 */
export function wireInfoBubbleListeners(container: HTMLElement): () => void {
  if (!container || (container as any).__infoBubblesWired) {
    return () => {};
  }
  (container as any).__infoBubblesWired = true;

  let activeWrapper: HTMLElement | null = null;

  const onMouseOver = (e: MouseEvent) => {
    const target = e.target as HTMLElement | null;
    if (!target) return;
    const wrapper = target.closest(
      '.info-bubble-wrapper, .knob-tooltip-wrapper, .knob-tooltip-trigger',
    ) as HTMLElement | null;
    if (wrapper && wrapper !== activeWrapper) {
      if (activeWrapper && activeWrapper !== wrapper) {
        resetTooltipPosition(activeWrapper);
      }
      activeWrapper = wrapper;
      adjustTooltipPosition(wrapper);
    }
  };

  const onMouseOut = (e: MouseEvent) => {
    const related = e.relatedTarget as Node | null;
    if (activeWrapper && (!related || !activeWrapper.contains(related))) {
      resetTooltipPosition(activeWrapper);
      activeWrapper = null;
    }
  };

  const onFocusIn = (e: FocusEvent) => {
    const target = e.target as HTMLElement | null;
    if (!target) return;
    const wrapper = target.closest(
      '.info-bubble-wrapper, .knob-tooltip-wrapper, .knob-tooltip-trigger',
    ) as HTMLElement | null;
    if (wrapper) {
      if (activeWrapper && activeWrapper !== wrapper) {
        resetTooltipPosition(activeWrapper);
      }
      activeWrapper = wrapper;
      adjustTooltipPosition(wrapper);
    }
  };

  const onFocusOut = (e: FocusEvent) => {
    const related = e.relatedTarget as Node | null;
    if (activeWrapper && (!related || !activeWrapper.contains(related))) {
      resetTooltipPosition(activeWrapper);
      activeWrapper = null;
    }
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape' && activeWrapper) {
      resetTooltipPosition(activeWrapper);
      activeWrapper.blur?.();
      activeWrapper = null;
    }
  };

  const onScroll = () => {
    if (activeWrapper) {
      adjustTooltipPosition(activeWrapper);
    }
  };

  container.addEventListener('mouseover', onMouseOver);
  container.addEventListener('mouseout', onMouseOut);
  container.addEventListener('focusin', onFocusIn);
  container.addEventListener('focusout', onFocusOut);
  container.addEventListener('keydown', onKeyDown);
  container.addEventListener('scroll', onScroll, { capture: true, passive: true });

  // Cleanup callback
  return () => {
    container.removeEventListener('mouseover', onMouseOver);
    container.removeEventListener('mouseout', onMouseOut);
    container.removeEventListener('focusin', onFocusIn);
    container.removeEventListener('focusout', onFocusOut);
    container.removeEventListener('keydown', onKeyDown);
    container.removeEventListener('scroll', onScroll, { capture: true });
    delete (container as any).__infoBubblesWired;
  };
}
