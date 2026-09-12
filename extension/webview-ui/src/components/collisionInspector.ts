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
  }
}
