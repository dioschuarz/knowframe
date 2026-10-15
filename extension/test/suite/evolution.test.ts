import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import type {
  DocumentationOrientation,
  OptimizationProfile,
  Tier1Objectives,
  Tier2Parameters,
  StructuralProblemSignature,
  StructuralRecommendation,
  TransformationPlan,
  TransformationAction,
} from '../../src/types/evolution.ts';

suite('Evolution Domain Types Test Suite (T006b)', () => {
  test('instantiates DocumentationOrientation', () => {
    const orientations: DocumentationOrientation[] = [
      'ai_oriented',
      'human_oriented',
      'balanced',
      'custom',
    ];
    assert.strictEqual(orientations.length, 4);
  });

  test('instantiates OptimizationProfile with canonical presets', () => {
    const profile: OptimizationProfile = {
      orientation: 'ai_oriented',
      cohesionWeight: 0.85,
      couplingWeight: 0.70,
      concentrationWeight: 0.80,
      contextEfficiencyWeight: 0.90,
      discoverabilityWeight: 0.85,
      narrativeCoherenceWeight: 0.40,
      maintenanceSimplicityWeight: 0.60,
      maxTraceDepth: 3,
      maxContextSurfaceTokens: 12000,
      fragmentSizeTarget: 3000,
      splitSensitivity: 0.70,
      canonicalOwnershipSensitivity: 0.80,
      constraints: [
        {
          id: 'c1',
          level: 'hard_constraint',
          ruleKey: 'never_split_canonical_docs',
          description: 'Preserve canonical documents',
        },
      ],
    };

    assert.strictEqual(profile.orientation, 'ai_oriented');
    assert.strictEqual(profile.fragmentSizeTarget, 3000);
    assert.strictEqual(profile.constraints.length, 1);
    assert.strictEqual(profile.constraints[0].level, 'hard_constraint');
  });

  test('validates TransformationAction discriminated union', () => {
    const actions: TransformationAction[] = [
      {
        type: 'create_document',
        payload: { filePath: 'docs/test.md', initialContent: '# Hello' },
      },
      {
        type: 'update_document_content',
        payload: { filePath: 'docs/existing.md', updatedContent: '# Updated' },
      },
      {
        type: 'rewrite_link_references',
        payload: { sourcePath: 'docs/a.md', oldTarget: 'old.md', newTarget: 'new.md' },
      },
      {
        type: 'inject_fragment_frontmatter',
        payload: { filePath: 'docs/test.md', fragments: [{ tokens: 100, inbound: 1, outbound: 2 }] },
      },
      {
        type: 'delete_document',
        payload: { filePath: 'index.md' },
      },
    ];

    const plan: TransformationPlan = {
      planId: 'p-1',
      recommendationId: 'r-1',
      actions,
      diffPreview: '+++ docs/test.md\n--- index.md',
      projectedDelta: {
        efficiencyDelta: 0.20,
        traceDepthDelta: -1.0,
        couplingDelta: -0.15,
        docCountDelta: 0,
      },
      requiresApproval: true,
    };

    assert.strictEqual(plan.actions.length, 5);
    assert.strictEqual(plan.actions[0].type, 'create_document');
    assert.strictEqual(plan.actions[4].type, 'delete_document');
    assert.strictEqual(plan.requiresApproval, true);
  });
});
