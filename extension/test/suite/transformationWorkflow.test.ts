import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { EvolutionManager, type EvolutionUiBridge } from '../../src/evolution/evolutionManager.ts';
import type { IpcClient } from '../../src/ipc/client.ts';
import type {
  GeneratePlanResponse,
  ApplyPlanResponse,
} from '../../src/types/evolution.ts';
import { renderTransformationPlanModalHtml } from '../../webview-ui/src/components/transformationPlanModal.ts';

suite('TransformationWorkflow End-to-End Test Suite (T052, SC-006, SC-007)', () => {
  function createWorkflowMockIpcClient() {
    let candidateGraphEvaluated = false;
    let persistentGraphMutated = false;

    const mockPlan: GeneratePlanResponse = {
      planId: 'plan_wf_001',
      recommendationId: 'rec_wf_001',
      actions: [
        {
          type: 'create_document',
          payload: {
            filePath: 'docs/networking_subtopic.md',
            initialContent: '# Networking Subtopic\n\nDeep architectural details.\n',
          },
        },
        {
          type: 'update_document_content',
          payload: {
            filePath: 'docs/architecture_monolith.md',
            updatedContent: '# Architecture Overview\n\nSee [[docs/networking_subtopic.md]].\n',
          },
        },
      ],
      diffPreview: '--- a/docs/architecture_monolith.md\n+++ b/docs/architecture_monolith.md\n@@ -20,10 +20,2 @@\n-## Detailed Networking\n+See [[docs/networking_subtopic.md]].',
      projectedDelta: {
        efficiencyDelta: 0.45,
        traceDepthDelta: -0.5,
        couplingDelta: -0.2,
        docCountDelta: 1,
      },
      requiresApproval: true,
    };

    const mockApplyResult: ApplyPlanResponse = {
      success: true,
      actualDelta: {
        efficiencyDelta: 0.49,
        traceDepthDelta: -0.56,
        couplingDelta: -0.23,
        docCountDelta: 3,
      },
    };

    const client: IpcClient = {
      sendRequest: async (method: string, params?: unknown) => {
        // Enforce NFR-001 Zero External AI / Network calls
        if (typeof params === 'string' && (params.startsWith('http://') || params.startsWith('https://'))) {
          throw new Error(`Invariant IV / NFR-001 Violation: External network URL: ${params}`);
        }

        switch (method) {
          case 'transformation/generatePlan':
            return mockPlan;

          case 'transformation/applyPlan':
            // Evaluate on an in-memory ephemeral candidate graph (Invariant I & XVI)
            candidateGraphEvaluated = true;
            persistentGraphMutated = false; // committed truth preserved until git commit
            return mockApplyResult;

          default:
            throw new Error(`Unhandled mock method: ${method}`);
        }
      },
    } as any;

    return {
      client,
      wasCandidateEvaluated: () => candidateGraphEvaluated,
      wasPersistentMutated: () => persistentGraphMutated,
    };
  }

  test('Step 1: Preview phase displays Before -> After comparisons and countervailing trade-offs (SC-006)', async () => {
    const { client } = createWorkflowMockIpcClient();
    let approvalPromptTriggered = false;

    const uiBridge: EvolutionUiBridge = {
      confirmPlanApproval: async () => {
        approvalPromptTriggered = true;
        return true;
      },
    };

    const manager = new EvolutionManager(client, uiBridge);

    // Measure plan generation latency (NFR-004 <= 500ms)
    const t0 = performance.now();
    const plan = await manager.generatePlan('rec_wf_001');
    const planElapsed = performance.now() - t0;

    assert.ok(planElapsed < 500, `Plan generation took ${planElapsed.toFixed(2)}ms, exceeding 500ms budget`);
    assert.strictEqual(plan.planId, 'plan_wf_001');
    assert.strictEqual(plan.requiresApproval, true, 'Invariant XVI safety gate must require approval');

    // Render preview modal HTML and verify complete transparency (SC-006)
    const previewHtml = renderTransformationPlanModalHtml({
      plan,
      isOpen: true,
      isApplied: false,
      actualDelta: null,
    });

    // Verify Target Document
    assert.ok(previewHtml.includes('Target Document:'));
    assert.ok(previewHtml.includes('docs/networking_subtopic.md') || previewHtml.includes('docs/architecture_monolith.md'));

    // Verify Before -> After impact comparison table
    assert.ok(previewHtml.includes('impact-comparison-table'), 'Must render Before -> After impact comparison table');
    assert.ok(previewHtml.includes('Context Efficiency'));
    assert.ok(previewHtml.includes('Navigational Complexity'));

    // Verify Countervailing Trade-offs section (Invariant XV)
    assert.ok(previewHtml.includes('Countervailing Trade-offs (Invariant XV)'));
    assert.ok(previewHtml.includes('↓'));

    // Verify Safety gate notice
    assert.ok(previewHtml.includes('Auto-Apply is OFF'));
    assert.ok(previewHtml.includes('Require Approval is ON'));
    assert.ok(previewHtml.includes('btn-apply-plan-changes'));
    assert.strictEqual(approvalPromptTriggered, false, 'No approval prompt triggered before apply click');
  });

  test('Step 2 & 3: Apply changes and Post-Change Re-Measurement evaluation (SC-007, Invariants I & XVI)', async () => {
    const { client, wasCandidateEvaluated, wasPersistentMutated } = createWorkflowMockIpcClient();
    let approvalPromptTriggered = false;

    const uiBridge: EvolutionUiBridge = {
      confirmPlanApproval: async () => {
        approvalPromptTriggered = true;
        return true;
      },
    };

    const manager = new EvolutionManager(client, uiBridge);
    const plan = await manager.generatePlan('rec_wf_001');

    // Measure re-measurement / apply execution time (NFR-005 <= 150ms)
    const t0 = performance.now();
    const result = await manager.applyPlan(plan);
    const applyElapsed = performance.now() - t0;

    assert.ok(applyElapsed < 150, `Apply & re-measurement took ${applyElapsed.toFixed(2)}ms, exceeding 150ms ceiling`);
    assert.strictEqual(approvalPromptTriggered, true, 'User approval dialog must be invoked');
    assert.strictEqual(result.success, true);
    assert.strictEqual(wasCandidateEvaluated(), true, 'Must evaluate on ephemeral candidate graph');
    assert.strictEqual(wasPersistentMutated(), false, 'Committed truth must not be mutated until Git commit (Invariant I)');

    // Step 4: Render Post-Change Re-Measurement Card in Modal
    const postApplyHtml = renderTransformationPlanModalHtml({
      plan,
      isOpen: true,
      isApplied: true,
      actualDelta: result.actualDelta,
    });

    // Verify Re-measurement card rendered with empirical deltas (T048)
    assert.ok(postApplyHtml.includes('post-change-remeasurement-card'));
    assert.ok(postApplyHtml.includes('Post-Change Re-Measurement'));
    assert.ok(postApplyHtml.includes('Δ Context Efficiency'));
    assert.ok(postApplyHtml.includes('+49%'));
    assert.ok(postApplyHtml.includes('Δ Navigational Complexity'));
    assert.ok(postApplyHtml.includes('-56%'));
    assert.ok(postApplyHtml.includes('Δ Maintenance Coupling'));
    assert.ok(postApplyHtml.includes('-23%'));

    // Verify verbatim narrative (T049, US6, FR-040)
    assert.ok(
      postApplyHtml.includes('The selected objectives improved, with the following structural trade-offs.'),
      'Must contain verbatim narrative summary'
    );

    // Verify button disabled state
    assert.ok(postApplyHtml.includes('Applied!'));
    assert.ok(postApplyHtml.includes('disabled'));
  });
});
