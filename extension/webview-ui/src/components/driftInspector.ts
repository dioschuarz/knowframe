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
          <button class="btn-drift-action" id="btn-action-open-doc">Open Document</button>
          <button class="btn-drift-action secondary" id="btn-action-open-code">Open Code Reference</button>
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
  }
}
