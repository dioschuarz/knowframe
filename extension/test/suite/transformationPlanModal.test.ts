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
    assert.ok(html.includes('data-plan-id="plan_decomp_001"'));
    assert.ok(html.includes('Apply Changes'));
    assert.ok(html.includes('Cancel'));
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
});
