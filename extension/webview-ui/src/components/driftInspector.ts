/**
 * Technical Drift Contextual Inspector Component
 *
 * Implements FR-026 and FR-032:
 * Displays technical drift diagnostics:
 * - Out-of-sync code target path and doc citation coordinates
 * - Drift explanation and symbol name
 * - [Open Doc] and [Open Code] quick navigation actions
 */

export interface DriftInspectorProps {
  docPath: string;
  codeTarget: string;
  symbolName?: string;
  citationLine: number;
  driftType: string;
  explanation: string;
  onOpenDoc?: (docPath: string, line?: number) => void;
  onOpenCode?: (codePath: string) => void;
  onReviewWithAi?: (filePath: string, issue?: unknown) => void;
}

export function renderDriftInspectorHtml(props: DriftInspectorProps): string {
  const symbolBadge = props.symbolName
    ? `<span class="symbol-badge"><code>${props.symbolName}</code></span>`
    : '';

  return `
    <div class="drift-inspector-card" id="drift-inspector">
      <div class="drift-header">
        <span class="drift-icon">⚠️</span>
        <span class="drift-title">Technical Drift Contradiction</span>
        <span class="drift-badge badge-${props.driftType.toLowerCase()}">${props.driftType}</span>
      </div>

      <div class="drift-body">
        <div class="drift-explanation">
          ${props.explanation}
        </div>

        <div class="drift-coords-row">
          <div class="drift-coord-item">
            <span class="coord-label">Referencing Doc:</span>
            <span class="coord-val link-style" id="btn-drift-open-doc">
              ${props.docPath}:${props.citationLine}
            </span>
          </div>

          <div class="drift-coord-item">
            <span class="coord-label">Missing/Deleted Code:</span>
            <span class="coord-val link-style" id="btn-drift-open-code">
              ${props.codeTarget}
            </span>
            ${symbolBadge}
          </div>
        </div>

        <div class="drift-actions">
          <button class="btn-drift-action" id="btn-action-open-doc" title="Open source file">Open Document</button>
          <button class="btn-drift-action secondary" id="btn-action-open-code" title="Open target file">Open Code Reference</button>
          <button class="btn-drift-action secondary" id="btn-action-review-ai" data-action="review-with-ai" data-file="${props.docPath}" title="Resolve issue with AI">
            <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" style="display:inline-block; vertical-align:middle; margin-right:4px;">
              <path d="M7.5 1.5L9 5l3.5 1.5L9 8l-1.5 3.5L6 8 2.5 6.5 6 5z" fill="currentColor" fill-opacity="0.3"/>
              <path d="M12.5 10.5l.75 1.75 1.75.75-1.75.75-.75 1.75-.75-1.75-1.75-.75 1.75-.75z" fill="currentColor"/>
            </svg>
            <span>Review with AI</span>
          </button>
        </div>
      </div>
    </div>
  `;
}

export class DriftInspector {
  private container: HTMLElement | null = null;
  private currentProps: DriftInspectorProps | null = null;

  public mount(container: HTMLElement, props: DriftInspectorProps): void {
    this.container = container;
    this.update(props);
  }

  public update(props: DriftInspectorProps): void {
    this.currentProps = props;
    if (!this.container) return;

    this.container.innerHTML = renderDriftInspectorHtml(props);
    this.bindEvents();
  }

  private bindEvents(): void {
    if (!this.container || !this.currentProps) return;

    const openDocLink = this.container.querySelector('#btn-drift-open-doc');
    const openDocBtn = this.container.querySelector('#btn-action-open-doc');
    const handleOpenDoc = () => {
      if (this.currentProps) {
        this.currentProps.onOpenDoc?.(this.currentProps.docPath, this.currentProps.citationLine);
      }
    };
    openDocLink?.addEventListener('click', handleOpenDoc);
    openDocBtn?.addEventListener('click', handleOpenDoc);

    const openCodeLink = this.container.querySelector('#btn-drift-open-code');
    const openCodeBtn = this.container.querySelector('#btn-action-open-code');
    const handleOpenCode = () => {
      if (this.currentProps) {
        this.currentProps.onOpenCode?.(this.currentProps.codeTarget);
      }
    };
    openCodeLink?.addEventListener('click', handleOpenCode);
    openCodeBtn?.addEventListener('click', handleOpenCode);

    const reviewAiBtn = this.container.querySelector('#btn-action-review-ai');
    reviewAiBtn?.addEventListener('click', () => {
      if (this.currentProps) {
        this.currentProps.onReviewWithAi?.(this.currentProps.docPath, {
          category: 'technical_drift',
          file_path: this.currentProps.docPath,
          line_number: this.currentProps.citationLine,
          target: this.currentProps.codeTarget,
          description: this.currentProps.explanation,
          remediation_hint: `Update documentation citation in '${this.currentProps.docPath}' at line ${this.currentProps.citationLine} to match '${this.currentProps.codeTarget}'.`,
        });
      }
    });
  }
}
