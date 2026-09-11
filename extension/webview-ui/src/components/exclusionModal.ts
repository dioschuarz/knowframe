/**
 * Exclusion Modal Component
 *
 * Displays evaluated glob pattern, affected file count, sample path chips,
 * and [Cancel] / [Apply Exclusion] buttons.
 */

export interface ExclusionModalProps {
  pattern: string;
  matchingCount: number;
  samplePaths: string[];
  scope: 'scan' | 'graph';
}

export interface ExclusionModalCallbacks {
  onApply?: (pattern: string, scope: 'scan' | 'graph') => void;
  onCancel?: () => void;
}

export interface ExclusionModalController {
  show: (props: ExclusionModalProps) => void;
  close: () => void;
  isOpen: () => boolean;
}

export function renderExclusionModalHtml(props: ExclusionModalProps): string {
  const { pattern, matchingCount, samplePaths, scope } = props;

  const sampleItemsHtml = samplePaths.length > 0
    ? samplePaths.map((p) => `<div class="excl-sample-chip" title="${p}">📄 ${p}</div>`).join('')
    : '<div class="excl-empty-samples">No matching files in workspace</div>';

  return `
    <div class="modal-backdrop" id="excl-modal-backdrop">
      <div class="modal-dialog excl-modal-dialog" role="dialog" aria-modal="true" aria-labelledby="excl-modal-title">
        <div class="modal-header">
          <div class="modal-title-row">
            <span class="modal-icon">🚫</span>
            <h3 id="excl-modal-title" class="modal-title">Exclude from Graph</h3>
          </div>
          <button class="modal-close-btn" id="excl-btn-close" aria-label="Close modal">✕</button>
        </div>

        <div class="modal-body">
          <p class="excl-description">
            Excluded notes remain physically indexed for link resolution (preserving link integrity),
            but will be hidden from the graph canvas and removed from the orphan noise count.
          </p>

          <div class="excl-form-group">
            <label for="excl-pattern-input" class="excl-label">Glob Pattern:</label>
            <input type="text" id="excl-pattern-input" class="excl-input" value="${pattern}" />
          </div>

          <div class="excl-matches-row">
            <span class="excl-matches-label">Affected Notes:</span>
            <span class="excl-matches-count ${matchingCount > 0 ? 'count-highlight' : 'count-zero'}">
              ${matchingCount} ${matchingCount === 1 ? 'file' : 'files'} match
            </span>
          </div>

          <div class="excl-samples-container">
            <div class="excl-samples-label">Preview Matching Paths (up to 10):</div>
            <div class="excl-samples-list">
              ${sampleItemsHtml}
            </div>
          </div>

          <div class="excl-scope-row">
            <label class="excl-scope-label">Exclusion Tier:</label>
            <div class="excl-scope-options">
              <label class="excl-radio-label">
                <input type="radio" name="excl-scope" value="graph" ${scope === 'graph' ? 'checked' : ''} />
                <span>Graph Only (Visual & Context)</span>
              </label>
              <label class="excl-radio-label">
                <input type="radio" name="excl-scope" value="scan" ${scope === 'scan' ? 'checked' : ''} />
                <span>Scan Tier (Completely Ignore)</span>
              </label>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" id="excl-btn-cancel">Cancel</button>
          <button class="btn-danger" id="excl-btn-apply">Apply Exclusion</button>
        </div>
      </div>
    </div>
  `;
}

export function renderExclusionModal(
  container: HTMLElement,
  callbacks: ExclusionModalCallbacks,
): ExclusionModalController {
  let isVisible = false;

  function close(): void {
    container.innerHTML = '';
    isVisible = false;
    callbacks.onCancel?.();
  }

  return {
    show(props: ExclusionModalProps) {
      container.innerHTML = renderExclusionModalHtml(props);
      isVisible = true;

      const backdrop = container.querySelector('#excl-modal-backdrop');
      const closeBtn = container.querySelector('#excl-btn-close');
      const cancelBtn = container.querySelector('#excl-btn-cancel');
      const applyBtn = container.querySelector('#excl-btn-apply');
      const patternInput = container.querySelector('#excl-pattern-input') as HTMLInputElement;

      backdrop?.addEventListener('click', (e) => {
        if (e.target === backdrop) close();
      });

      closeBtn?.addEventListener('click', () => close());
      cancelBtn?.addEventListener('click', () => close());

      applyBtn?.addEventListener('click', () => {
        const pattern = patternInput?.value.trim() || props.pattern;
        const selectedScopeRadio = container.querySelector('input[name="excl-scope"]:checked') as HTMLInputElement;
        const scope = (selectedScopeRadio?.value as 'scan' | 'graph') || props.scope;
        callbacks.onApply?.(pattern, scope);
        close();
      });
    },

    close() {
      close();
    },

    isOpen() {
      return isVisible;
    },
  };
}
