/**
 * Semantic Collision Contextual Inspector Component
 *
 * Implements FR-022 and FR-032:
 * Displays pairwise collision diagnostics:
 * - Composite similarity percentage and classification badge
 * - Shared key terms
 * - Heuristic canonical candidate proposal
 * - [Propose as Canonical] action button
 */

export interface CollisionInspectorProps {
  sourceNote: string;
  targetNote: string;
  compositeSimilarity: number;
  classification: string;
  sharedTerms: string[];
  canonicalCandidate?: string;
  canonicalConfidence?: number;
  canonicalEvidence?: string[];
  onProposeCanonical?: (noteId: string) => void;
  onOpenNote?: (noteId: string) => void;
  onReviewWithAi?: (filePath: string, issue?: unknown) => void;
}

export function renderCollisionInspectorHtml(props: CollisionInspectorProps): string {
  const simPct = (props.compositeSimilarity * 100).toFixed(1);
  const confPct = props.canonicalConfidence
    ? (props.canonicalConfidence * 100).toFixed(0)
    : '85';

  const badgeClass = props.compositeSimilarity >= 0.85 ? 'badge-block' : 'badge-warn';

  const sharedTermsHtml = props.sharedTerms.length > 0
    ? props.sharedTerms.map((t) => `<span class="shared-term-chip">${t}</span>`).join(' ')
    : '<em>None detected</em>';

  const canonicalCandidate = props.canonicalCandidate || props.targetNote;

  const evidenceHtml = props.canonicalEvidence && props.canonicalEvidence.length > 0
    ? `<ul class="canonical-evidence-list">
        ${props.canonicalEvidence.map((e) => `<li>${e}</li>`).join('')}
       </ul>`
    : '';

  return `
    <div class="collision-inspector-card" id="collision-inspector">
      <div class="collision-header">
        <div class="collision-title-row">
          <span class="collision-icon">⚡</span>
          <span class="collision-title">Semantic Collision Detected</span>
          <span class="collision-badge ${badgeClass}">${simPct}% (${props.classification})</span>
        </div>
      </div>

      <div class="collision-body">
        <div class="collision-pair-row">
          <div class="collision-node" id="btn-open-source" title="Click to view note">
            <span class="node-label">Candidate:</span>
            <span class="node-id link-style">${props.sourceNote}</span>
          </div>
          <span class="collision-vs">⇄</span>
          <div class="collision-node" id="btn-open-target" title="Click to view note">
            <span class="node-label">Existing:</span>
            <span class="node-id link-style">${props.targetNote}</span>
          </div>
        </div>

        <div class="collision-detail-section">
          <span class="detail-label">Shared Key Terms:</span>
          <div class="shared-terms-container">${sharedTermsHtml}</div>
        </div>

        <div class="canonical-proposal-section">
          <div class="canonical-header">
            <span class="canonical-star">★</span>
            <span class="canonical-title">Recommended Canonical:</span>
            <strong class="canonical-name">${canonicalCandidate}</strong>
            <span class="canonical-conf">(${confPct}% confidence)</span>
          </div>
          ${evidenceHtml}
          <div class="canonical-actions">
            <button class="btn-canonical-propose" id="btn-propose-canonical" data-candidate="${canonicalCandidate}">
              Propose as Canonical
            </button>
            <button class="btn-secondary" id="btn-collision-review-ai" data-action="review-with-ai" data-file="${props.sourceNote}" title="Resolve issue with AI">
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:4px;">
                <path d="M7.5 1.5L9 5l3.5 1.5L9 8l-1.5 3.5L6 8 2.5 6.5 6 5z" fill="currentColor" fill-opacity="0.3"/>
                <path d="M12.5 10.5l.75 1.75 1.75.75-1.75.75-.75 1.75-.75-1.75-1.75-.75 1.75-.75z" fill="currentColor"/>
              </svg>
              <span>Review with AI</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  `;
}

export class CollisionInspector {
  private container: HTMLElement | null = null;
  private currentProps: CollisionInspectorProps | null = null;

  public mount(container: HTMLElement, props: CollisionInspectorProps): void {
    this.container = container;
    this.update(props);
  }

  public update(props: CollisionInspectorProps): void {
    this.currentProps = props;
    if (!this.container) return;

    this.container.innerHTML = renderCollisionInspectorHtml(props);
    this.bindEvents();
  }

  private bindEvents(): void {
    if (!this.container || !this.currentProps) return;

    const sourceBtn = this.container.querySelector('#btn-open-source');
    if (sourceBtn) {
      sourceBtn.addEventListener('click', () => {
        this.currentProps?.onOpenNote?.(this.currentProps.sourceNote);
      });
    }

    const targetBtn = this.container.querySelector('#btn-open-target');
    if (targetBtn) {
      targetBtn.addEventListener('click', () => {
        this.currentProps?.onOpenNote?.(this.currentProps.targetNote);
      });
    }

    const proposeBtn = this.container.querySelector('#btn-propose-canonical');
    if (proposeBtn) {
      proposeBtn.addEventListener('click', () => {
        const candidate = this.currentProps?.canonicalCandidate || this.currentProps?.targetNote;
        if (candidate) {
          this.currentProps?.onProposeCanonical?.(candidate);
        }
      });
    }

    const reviewAiBtn = this.container.querySelector('#btn-collision-review-ai');
    if (reviewAiBtn) {
      reviewAiBtn.addEventListener('click', () => {
        if (this.currentProps) {
          this.currentProps.onReviewWithAi?.(this.currentProps.sourceNote, {
            category: 'semantic_collision',
            file_path: this.currentProps.sourceNote,
            target: this.currentProps.targetNote,
            description: `Semantic collision (${Math.round(this.currentProps.compositeSimilarity * 100)}% ${this.currentProps.classification}) between '${this.currentProps.sourceNote}' and '${this.currentProps.targetNote}'.`,
            remediation_hint: `Consolidate overlapping concepts into canonical candidate '${this.currentProps.canonicalCandidate || this.currentProps.targetNote}'.`,
          });
        }
      });
    }
  }
}
