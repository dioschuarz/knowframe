import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderTransformationPlanModalHtml,
  renderDiffPreviewHtml,
  formatDeltaNumber,
  type TransformationPlanModalProps,
} from '../../webview-ui/src/components/transformationPlanModal.ts';
import type { TransformationPlan } from '../../src/types/evolution.ts';

suite('TransformationPlanModal Unit Test Suite (T054)', () => {
  const samplePlan: TransformationPlan = {
    planId: 'plan_decomp_001',
    recommendationId: 'rec_001',
    actions: [
      {
        type: 'create_document',
        payload: {
          filePath: 'docs/networking.md',
          initialContent: '# Networking Subtopic\n\nContent details.\n',
        },
      },
      {
        type: 'update_document_content',
        payload: {
          filePath: 'docs/monolith.md',
          updatedContent: '# Trimmed Monolith\n\nSee [[docs/networking.md]].\n',
        },
      },
    ],
    diffPreview: `--- a/docs/monolith.md\n+++ b/docs/monolith.md\n@@ -10,5 +10,1 @@\n-## Deep Networking Details\n+See [[docs/networking.md]].`,
    projectedDelta: {
      efficiencyDelta: 0.22,
      traceDepthDelta: -0.4,
      couplingDelta: -0.18,
      docCountDelta: 1,
    },
    requiresApproval: true,
  };

  test('returns empty string when isOpen is false or plan is null', () => {
    assert.strictEqual(renderTransformationPlanModalHtml({ plan: samplePlan, isOpen: false }), '');
    assert.strictEqual(renderTransformationPlanModalHtml({ plan: null, isOpen: true }), '');
  });

  test('renders Invariant XVI approval gate notice and projected delta cards', () => {
    const html = renderTransformationPlanModalHtml({ plan: samplePlan, isOpen: true });

    assert.ok(html.includes('Invariant XVI Protection'));
    assert.ok(html.includes('Auto-Apply is OFF'));
    assert.ok(html.includes('Δ Efficiency'));
    assert.ok(html.includes('+0.22') || html.includes('+0.2'));
    assert.ok(html.includes('Δ Trace Depth'));
    assert.ok(html.includes('Δ Coupling'));
    assert.ok(html.includes('Δ Document Count'));
    assert.ok(html.includes('+1'));
  });

  test('renders atomic filesystem actions and highlighted unified diff', () => {
    const html = renderTransformationPlanModalHtml({ plan: samplePlan, isOpen: true });

    assert.ok(html.includes('docs/networking.md'));
    assert.ok(html.includes('docs/monolith.md'));
    assert.ok(html.includes('create document'));
    assert.ok(html.includes('update document content'));

    assert.ok(html.includes('diff-viewer'));
    assert.ok(html.includes('diff-hunk'));
    assert.ok(html.includes('diff-deletion'));
    assert.ok(html.includes('diff-addition'));
  });

  test('renders user-controlled Apply Changes button bound to plan ID', () => {
    const html = renderTransformationPlanModalHtml({ plan: samplePlan, isOpen: true });

    assert.ok(html.includes('btn-apply-plan'));
    assert.ok(html.includes('btn-apply-plan-changes'));
    assert.ok(html.includes('btn-cancel-plan'));
    assert.ok(html.includes('data-plan-id="plan_decomp_001"'));
    assert.ok(html.includes('Apply Changes'));
    assert.ok(html.includes('Cancel'));
  });

  test('renders a stable plan identifier for apply approval', () => {
    const snakeCasePlan = { ...samplePlan, planId: undefined, plan_id: 'plan_snake_case' } as any;
    const html = renderTransformationPlanModalHtml({ plan: snakeCasePlan, isOpen: true });
    assert.ok(html.includes('data-plan-id="plan_snake_case"'), 'Must preserve Rust snake_case plan IDs in the apply button');
  });

  test('treats dashed Markdown list items as additions in unified diffs', () => {
    const html = renderDiffPreviewHtml('--- /dev/null\n+++ b/index.md\n@@ -0,0 +1,2 @@\n+# Knowledge Index\n+- [Section](research.md#section)');
    assert.ok(html.includes('diff-addition'));
    assert.ok(!html.includes('diff-deletion'));
  });

  test('displays actual Before-vs-After deltas post-transformation', () => {
    const html = renderTransformationPlanModalHtml({
      plan: samplePlan,
      isOpen: true,
      actualDelta: {
        efficiencyDelta: 0.25,
        traceDepthDelta: -0.5,
        couplingDelta: -0.2,
        docCountDelta: 1,
      },
    });

    assert.ok(html.includes('Actual Before-vs-After Deltas'));
    assert.ok(html.includes('+0.25') || html.includes('+0.3'));
  });

  test('shows a disabled green Applied! state after successful validation without a Close button', () => {
    const html = renderTransformationPlanModalHtml({
      plan: samplePlan,
      isOpen: true,
      isApplied: true,
    });

    assert.ok(html.includes('btn-apply-plan applied'));
    assert.ok(html.includes('Applied!'));
    assert.ok(html.includes('disabled'));
    assert.ok(html.includes('aria-disabled="true"'));
    assert.ok(!html.includes('Applying Atomic Changes...'));

    // Verify Invariant: No button called Close rendered in footer; closing is handled via top-right X button
    assert.ok(!html.includes('id="btn-cancel-plan"'), 'Must not render secondary cancel/close button in footer when applied');
    assert.ok(!html.includes('>Close<'), 'Must never render a button called Close');
    assert.ok(html.includes('id="btn-close-plan-modal"'), 'Must render top-right X button');
    assert.ok(html.includes('aria-label="Close"'), 'X button must have aria-label Close');
    assert.ok(html.includes('title="Close"'), 'X button must have title Close');
    assert.ok(html.includes('✕'), 'X button must contain ✕ character');
  });

  test('renders target document banner and Before -> After impact comparison table (T044, T050)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: samplePlan,
      isOpen: true,
    });

    assert.ok(html.includes('Target Document:'), 'Must show Target Document label');
    assert.ok(html.includes('docs/networking.md') || html.includes('docs/monolith.md'), 'Must display target document path');
    assert.ok(html.includes('impact-comparison-table'), 'Must render impact table');
    assert.ok(html.includes('Context Efficiency'), 'Must compare Context Efficiency');
    assert.ok(html.includes('Navigational Complexity'), 'Must compare Navigational Complexity');
    assert.ok(html.includes('Maintenance Coupling'), 'Must compare Maintenance Coupling');
    assert.ok(html.includes('Documents'), 'Must compare Documents count');
    assert.ok(html.includes('References'), 'Must compare References count');
  });

  test('renders explicit Countervailing Trade-offs section (T045, T050, Invariant XV)', () => {
    const planWithTradeOffs: TransformationPlan = {
      ...samplePlan,
      tradeOffs: [
        'Document count increases by 1',
        'Section anchor stability requires vigilant link updates',
      ],
    };
    const html = renderTransformationPlanModalHtml({
      plan: planWithTradeOffs,
      isOpen: true,
    });

    assert.ok(html.includes('Countervailing Trade-offs (Invariant XV)'), 'Must render trade-offs section header');
    assert.ok(html.includes('Document count increases by 1'), 'Must render specific trade-off');
    assert.ok(html.includes('Section anchor stability'), 'Must render second trade-off');
    assert.ok(html.includes('↓'), 'Must include trade-off marker');
  });

  test('enforces user approval safety gate notice: Auto-Apply OFF, Require Approval ON (T046)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: samplePlan,
      isOpen: true,
    });

    assert.ok(html.includes('Auto-Apply is OFF'), 'Must state Auto-Apply is OFF');
    assert.ok(html.includes('Require Approval is ON'), 'Must state Require Approval is ON');
  });
});
