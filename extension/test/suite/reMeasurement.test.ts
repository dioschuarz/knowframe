import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderTransformationPlanModalHtml,
} from '../../webview-ui/src/components/transformationPlanModal.ts';
import type { TransformationPlan } from '../../src/types/evolution.ts';

suite('Post-Change Re-Measurement Unit Test Suite (T048, T049, T051)', () => {
  const basePlan: TransformationPlan = {
    planId: 'plan_decomp_100',
    recommendationId: 'rec_100',
    actions: [
      {
        type: 'create_document',
        payload: {
          filePath: 'docs/routing.md',
          initialContent: '# Routing\n',
        },
      },
    ],
    diffPreview: '--- a/docs/old.md\n+++ b/docs/old.md\n@@ -1,1 +1,1 @@\n-old\n+new',
    projectedDelta: {
      efficiencyDelta: 0.35,
      traceDepthDelta: -0.4,
      couplingDelta: -0.15,
      docCountDelta: 1,
    },
    requiresApproval: true,
  };

  test('does not render Post-Change Re-Measurement card before plan is applied', () => {
    const html = renderTransformationPlanModalHtml({
      plan: basePlan,
      isOpen: true,
      isApplied: false,
      actualDelta: null,
    });

    assert.ok(!html.includes('post-change-remeasurement-card'));
    assert.ok(!html.includes('The selected objectives improved, with the following structural trade-offs.'));
  });

  test('renders Post-Change Re-Measurement card when actualDelta is provided (T048, T051)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: basePlan,
      isOpen: true,
      actualDelta: {
        efficiencyDelta: 0.49,
        traceDepthDelta: -0.56,
        couplingDelta: -0.23,
        docCountDelta: 3,
        refCountDelta: 7,
      },
    });

    assert.ok(html.includes('post-change-remeasurement-card'), 'Must render re-measurement card');
    assert.ok(html.includes('Post-Change Re-Measurement'), 'Must render card title');
    assert.ok(html.includes('Empirical In-Memory Evaluation'), 'Must state empirical evaluation');
  });

  test('renders mandatory narrative summary verbatim (T049, Invariant XVI, US6)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: basePlan,
      isOpen: true,
      actualDelta: {
        efficiencyDelta: 0.49,
        traceDepthDelta: -0.56,
        couplingDelta: -0.23,
        docCountDelta: 3,
        refCountDelta: 7,
      },
    });

    assert.ok(
      html.includes('The selected objectives improved, with the following structural trade-offs.'),
      'Must contain verbatim narrative summary per FR-040, US6'
    );
  });

  test('displays all 5 empirical deltas in percentage or count formatting (T048, US6)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: basePlan,
      isOpen: true,
      actualDelta: {
        efficiencyDelta: 0.49,
        traceDepthDelta: -0.56,
        couplingDelta: -0.23,
        docCountDelta: 3,
        refCountDelta: 7,
      },
    });

    assert.ok(html.includes('Δ Context Efficiency'), 'Must include Context Efficiency label');
    assert.ok(html.includes('+49%'), 'Must show +49% Context Efficiency delta');

    assert.ok(html.includes('Δ Navigational Complexity'), 'Must include Navigational Complexity label');
    assert.ok(html.includes('-56%'), 'Must show -56% Navigational Complexity delta');

    assert.ok(html.includes('Δ Maintenance Coupling'), 'Must include Maintenance Coupling label');
    assert.ok(html.includes('-23%'), 'Must show -23% Maintenance Coupling delta');

    assert.ok(html.includes('Δ Documents'), 'Must include Documents delta label');
    assert.ok(html.includes('+3'), 'Must show +3 documents delta');

    assert.ok(html.includes('Δ References'), 'Must include References delta label');
    assert.ok(html.includes('+7'), 'Must show +7 references delta');
  });

  test('renders Invariant I candidate graph notice (ephemeral candidate graph, committed truth preserved)', () => {
    const html = renderTransformationPlanModalHtml({
      plan: basePlan,
      isOpen: true,
      actualDelta: {
        efficiencyDelta: 0.2,
        traceDepthDelta: -0.3,
        couplingDelta: -0.1,
        docCountDelta: 1,
      },
    });

    assert.ok(html.includes('in-memory ephemeral candidate graph'), 'Must state ephemeral graph');
    assert.ok(html.includes('committed truth preserved until Git commit'), 'Must state Invariant I preservation');
  });
});
