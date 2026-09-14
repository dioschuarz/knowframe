/**
 * OrientationSelector Component (T049)
 *
 * Implements Constitution Principle XII & FR-001 - FR-008 (US1).
 * Renders 4 selectable radio cards:
 * 1. AI-Oriented (3k token target, max retrieval efficiency)
 * 2. Human-Oriented (8k token target, narrative coherence)
 * 3. Balanced (5k token target, balanced trade-offs)
 * 4. Custom (Advanced 2-tier configuration)
 */

import type { DocumentationOrientation } from '../../../src/types/evolution.ts';

export interface OrientationCardInfo {
  id: DocumentationOrientation;
  title: string;
  badge: string;
  targetSize: string;
  description: string;
  tradeOff: string;
}

export const ORIENTATION_CARDS: OrientationCardInfo[] = [
  {
    id: 'ai_oriented',
    title: 'AI-Oriented',
    badge: '3k tokens',
    targetSize: '3,000 tokens',
    description: 'Optimized for autonomous agent retrieval, high context efficiency, and minimal token overhead.',
    tradeOff: 'Higher fragmentation; splits narrative chapters into modular sections.',
  },
  {
    id: 'human_oriented',
    title: 'Human-Oriented',
    badge: '8k tokens',
    targetSize: '8,000 tokens',
    description: 'Preserves long-form narrative coherence, comprehensive conceptual flow, and authorial layout.',
    tradeOff: 'Lower context efficiency; agents load larger documents with extraneous sections.',
  },
  {
    id: 'balanced',
    title: 'Balanced',
    badge: '5k tokens',
    targetSize: '5,000 tokens',
    description: 'Harmonizes agent retrieval efficiency with human reading flow and navigation speed.',
    tradeOff: 'Moderate balance; intermediate section sizes across all topics.',
  },
  {
    id: 'custom',
    title: 'Custom (Advanced)',
    badge: 'User Defined',
    targetSize: 'Custom',
    description: 'Fine-tune 6 Tier 1 business objectives, 11 Tier 2 technical parameters, and SAT hard constraints.',
    tradeOff: 'Configurable directly in the Custom Settings drawer.',
  },
];

export interface OrientationSelectorProps {
  activeOrientation: DocumentationOrientation;
  onSelect?: (orientation: DocumentationOrientation) => void;
}

export function renderOrientationSelectorHtml(props: OrientationSelectorProps): string {
  const cardsHtml = ORIENTATION_CARDS.map((card) => {
    const isSelected = card.id === props.activeOrientation;
    const selectedClass = isSelected ? 'orientation-card-selected' : '';
    const radioChecked = isSelected ? 'checked' : '';

    return `
      <div class="orientation-card ${selectedClass}" data-orientation="${card.id}">
        <div class="orientation-card-header">
          <label class="orientation-radio-label">
            <input type="radio" name="doc-orientation" value="${card.id}" ${radioChecked} class="orientation-radio" />
            <span class="orientation-card-title">${card.title}</span>
          </label>
          <span class="orientation-badge">${card.badge}</span>
        </div>
        <div class="orientation-card-desc">${card.description}</div>
        <div class="orientation-card-tradeoff">
          <span class="tradeoff-icon">⚖</span>
          <span class="tradeoff-text">${card.tradeOff}</span>
        </div>
      </div>
    `;
  }).join('');

  return `
    <div class="orientation-selector-container">
      <div class="orientation-selector-heading">
        <h3>Documentation Orientation</h3>
        <p class="orientation-subtitle">Select the Knowledge Organization System (KOS) optimization profile for this vault.</p>
      </div>
      <div class="orientation-cards-grid">
        ${cardsHtml}
      </div>
    </div>
  `;
}
