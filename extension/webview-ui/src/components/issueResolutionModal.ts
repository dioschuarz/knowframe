/**
 * Issue Resolution Modal Component
 *
 * Implements Constitution Principle VI (Zero Action Duplication & State-Driven Emergence).
 * Surfaces detailed identification points, root cause explanations, and actionable
 * resolution options with explicit trade-offs for any active knowledge issue or finding.
 */

import type { KnowledgeIssue } from '../../../src/types/admission.ts';

export interface IssueResolutionModalProps {
  issue: KnowledgeIssue;
  nodeLabel: string;
  nodeFilePath: string;
}

export interface IssueResolutionModalCallbacks {
  onOpenSource?: (filePath: string, line?: number) => void;
  onFixBrokenLink?: (source: string, target: string, line?: number) => void;
  onMarkReviewed?: (docId: string) => void;
  onClose?: () => void;
}

export interface IssueResolutionModalController {
  show: (props: IssueResolutionModalProps) => void;
  close: () => void;
  isOpen: () => boolean;
}

interface ResolutionOption {
  title: string;
  recommended: boolean;
  actionSummary: string;
  tradeOff: string;
}

function getOptionsForCategory(issue: KnowledgeIssue): ResolutionOption[] {
  const cat = (issue.category || '').toLowerCase();

  if (cat.includes('broken_reference') || cat.includes('broken_link')) {
    return [
      {
        title: 'Re-target Reference to Existing Note',
        recommended: true,
        actionSummary: 'Use Quick-Pick to re-target this link to an existing document or heading fragment.',
        tradeOff: 'Preserves graph topology and repairs link integrity instantly without creating stub notes. Requires confirming semantic equivalence.',
      },
      {
        title: 'Create Missing Note File',
        recommended: false,
        actionSummary: 'Create a new markdown note at the referenced target path.',
        tradeOff: 'Resolves the broken link immediately, but creates an initial stub that will require authoring context and tags.',
      },
      {
        title: 'Convert to Plain Text / Code Entity',
        recommended: false,
        actionSummary: 'Remove wikilink double brackets `[[...]]` or wrap in backticks.',
        tradeOff: 'Eliminates the broken reference finding completely. Drops graph connectivity for this reference.',
      },
    ];
  }

  if (cat.includes('unanchored')) {
    return [
      {
        title: 'Add Section Heading Fragment (#anchor)',
        recommended: true,
        actionSummary: 'Refine link to point directly to a specific section heading (e.g. `[[target#section|label]]`).',
        tradeOff: 'Guarantees directed LLM context retrieval and minimizes token consumption. Requires maintaining heading stability.',
      },
      {
        title: 'Retain Document-Level Reference',
        recommended: false,
        actionSummary: 'Leave reference pointing to the document root.',
        tradeOff: 'Simpler syntax, but causes wider context ingestion and slight token overhead when traversed during subcalls.',
      },
    ];
  }

  if (cat.includes('stale') || cat.includes('freshness') || cat.includes('review')) {
    return [
      {
        title: 'Review and Confirm Accuracy',
        recommended: true,
        actionSummary: 'Inspect document contents, verify referenced concepts, and mark the review obligation resolved.',
        tradeOff: 'Confirms vault freshness for CI/CD gates and updates review timestamp. Requires manual inspection.',
      },
      {
        title: 'Exclude from Automated Freshness Scans',
        recommended: false,
        actionSummary: 'Add document path to scan exclusions.',
        tradeOff: 'Eliminates review obligations and warnings, but disables continuous staleness auditing for this file.',
      },
    ];
  }

  // Generic fallback
  return [
    {
      title: 'Apply Remediation Guidance',
      recommended: true,
      actionSummary: issue.remediation_hint || 'Update document to align with workspace knowledge conventions.',
      tradeOff: 'Restores vault quality score and resolves deduction points.',
    },
  ];
}

function capitalizeWords(str: string): string {
  return str.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

export function renderIssueResolutionModalHtml(props: IssueResolutionModalProps): string {
  const { issue, nodeLabel, nodeFilePath } = props;
  const catFormatted = capitalizeWords(issue.category || 'Quality Finding');
  const filePath = issue.file_path || nodeFilePath;
  const lineCoord = issue.line_number ? `Line ${issue.line_number}` : 'Unspecified line';
  const options = getOptionsForCategory(issue);
  const isBroken = (issue.category || '').toLowerCase().includes('broken_reference');
  const isReview = (issue.category || '').toLowerCase().includes('review') || (issue.category || '').toLowerCase().includes('stale');

  const optionsHtml = options.map((opt, idx) => `
    <div class="issue-opt-card ${opt.recommended ? 'opt-recommended' : ''}">
      <div class="issue-opt-header">
        <span class="issue-opt-title">
          ${opt.recommended ? '<span class="issue-opt-badge-rec">RECOMMENDED</span>' : `<span class="issue-opt-num">Option ${idx + 1}</span>`}
          ${opt.title}
        </span>
      </div>
      <div class="issue-opt-summary">${opt.actionSummary}</div>
      <div class="issue-opt-tradeoff">
        <span class="issue-opt-tradeoff-lbl">Trade-off:</span> ${opt.tradeOff}
      </div>
    </div>
  `).join('');

  return `
    <div class="modal-backdrop" id="issue-modal-backdrop">
      <div class="modal-dialog issue-modal-dialog" role="dialog" aria-modal="true" aria-labelledby="issue-modal-title">
        <div class="modal-header">
          <div class="modal-title-row">
            <span class="modal-icon">
              <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="#ef4444" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M8 1.5L14.5 13.5H1.5L8 1.5Z"/><line x1="8" y1="6" x2="8" y2="9.5"/><circle cx="8" cy="11.5" r="0.75" fill="#ef4444"/>
              </svg>
            </span>
            <h3 id="issue-modal-title" class="modal-title" style="text-transform: capitalize;">${catFormatted} Resolution</h3>
          </div>
          <button class="modal-close-btn" id="issue-btn-close" aria-label="Close modal">✕</button>
        </div>

        <div class="modal-body issue-modal-body">
          <!-- Identification Points Grid -->
          <div class="issue-points-grid">
            <div class="issue-point-box">
              <span class="issue-point-lbl">Category</span>
              <span class="issue-point-val issue-cat-val">${catFormatted}</span>
            </div>
            <div class="issue-point-box">
              <span class="issue-point-lbl">Coordinates</span>
              <span class="issue-point-val">${lineCoord}</span>
            </div>
            <div class="issue-point-box">
              <span class="issue-point-lbl">Penalty</span>
              <span class="issue-point-val issue-penalty-val">-${issue.deduction_points || 0} pts</span>
            </div>
            ${issue.target ? `
            <div class="issue-point-box issue-point-span-full">
              <span class="issue-point-lbl">Target Reference</span>
              <span class="issue-point-val issue-target-code">${issue.target}</span>
            </div>` : ''}
            <div class="issue-point-box issue-point-span-full">
              <span class="issue-point-lbl">Source File</span>
              <span class="issue-point-val" title="${filePath}">${filePath}</span>
            </div>
          </div>

          <!-- Description / Root Cause -->
          <div class="issue-desc-card">
            <div class="issue-desc-title">Finding Description & Context</div>
            <div class="issue-desc-text">${issue.description}</div>
            ${issue.remediation_hint ? `
            <div class="issue-hint-row">
              <span class="issue-hint-icon">💡</span>
              <span class="issue-hint-text">${issue.remediation_hint}</span>
            </div>` : ''}
          </div>

          <!-- Resolution Options & Trade-offs -->
          <div class="issue-options-section">
            <div class="issue-options-title">Resolution Options & Trade-offs</div>
            <div class="issue-options-list">
              ${optionsHtml}
            </div>
          </div>
        </div>

        <div class="modal-footer issue-modal-footer">
          <div class="issue-modal-footer-left">
            ${issue.line_number ? `
            <button class="btn-secondary" id="issue-btn-jump-line" title="Jump to line ${issue.line_number} in editor">
              <svg viewBox="0 0 16 16" width="13" height="13" fill="none" stroke="currentColor" stroke-width="1.5" style="vertical-align: -1px; margin-right: 4px;"><path d="M3 2.5A1.5 1.5 0 0 1 4.5 1h7A1.5 1.5 0 0 1 13 2.5v11a1.5 1.5 0 0 1-1.5 1.5h-7A1.5 1.5 0 0 1 3 13.5v-11Z"/><line x1="5.5" y1="4.5" x2="10.5" y2="4.5"/><line x1="5.5" y1="7.5" x2="10.5" y2="7.5"/></svg>
              Jump to Line ${issue.line_number}
            </button>` : ''}
          </div>
          <div class="issue-modal-footer-right">
            ${isBroken && issue.target ? `
            <button class="btn-primary" id="issue-btn-fix-ref" title="Select target note from quick-pick">
              Fix Reference
            </button>` : ''}
            ${isReview ? `
            <button class="btn-primary" id="issue-btn-mark-reviewed" title="Mark review resolved">
              Mark Reviewed
            </button>` : ''}
            <button class="btn-secondary" id="issue-btn-cancel">Close</button>
          </div>
        </div>
      </div>
    </div>
  `;
}

export function renderIssueResolutionModal(
  container: HTMLElement,
  callbacks: IssueResolutionModalCallbacks,
): IssueResolutionModalController {
  let isVisible = false;

  function close(): void {
    container.innerHTML = '';
    isVisible = false;
    callbacks.onClose?.();
  }

  return {
    show(props: IssueResolutionModalProps): void {
      container.innerHTML = renderIssueResolutionModalHtml(props);
      isVisible = true;

      const backdrop = container.querySelector('#issue-modal-backdrop');
      const closeBtn = container.querySelector('#issue-btn-close');
      const cancelBtn = container.querySelector('#issue-btn-cancel');
      const jumpLineBtn = container.querySelector('#issue-btn-jump-line');
      const fixRefBtn = container.querySelector('#issue-btn-fix-ref');
      const markRevBtn = container.querySelector('#issue-btn-mark-reviewed');

      const handleClose = (e?: Event) => {
        e?.stopPropagation();
        close();
      };

      backdrop?.addEventListener('click', (e) => {
        if (e.target === backdrop) handleClose(e);
      });

      closeBtn?.addEventListener('click', handleClose);
      cancelBtn?.addEventListener('click', handleClose);

      jumpLineBtn?.addEventListener('click', (e) => {
        e.stopPropagation();
        const p = props.issue.file_path || props.nodeFilePath;
        callbacks.onOpenSource?.(p, props.issue.line_number);
        handleClose();
      });

      fixRefBtn?.addEventListener('click', (e) => {
        e.stopPropagation();
        const src = props.issue.file_path || props.nodeFilePath;
        const tgt = props.issue.target || '';
        callbacks.onFixBrokenLink?.(src, tgt, props.issue.line_number || 1);
        handleClose();
      });

      markRevBtn?.addEventListener('click', (e) => {
        e.stopPropagation();
        const p = props.issue.file_path || props.nodeFilePath;
        callbacks.onMarkReviewed?.(p);
        handleClose();
      });
    },

    close(): void {
      close();
    },

    isOpen(): boolean {
      return isVisible;
    },
  };
}
