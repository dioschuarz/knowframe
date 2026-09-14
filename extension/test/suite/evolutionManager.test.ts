import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { EvolutionManager, type EvolutionUiBridge } from '../../src/evolution/evolutionManager.ts';
import type { IpcClient } from '../../src/ipc/client.ts';
import type {
  DocumentationOrientation,
  GetOrientationResponse,
  SetOrientationResponse,
  GetCustomConfigResponse,
  UpdateCustomConfigResponse,
  ValidateConstraintsResponse,
  GetStructuralProblemsResponse,
  GetRecommendationsResponse,
  GeneratePlanResponse,
  ApplyPlanResponse,
  TransformationPlan,
} from '../../src/types/evolution.ts';

suite('EvolutionManager Unit Test Suite (Phase 8)', () => {
  function createMockIpcClient(responses: Record<string, unknown>): {
    client: IpcClient;
    sentRequests: { method: string; params: unknown }[];
  } {
    const sentRequests: { method: string; params: unknown }[] = [];
    const client = {
      sendRequest: async (method: string, params?: unknown) => {
        sentRequests.push({ method, params });
        if (method in responses) {
          return responses[method];
        }
        throw new Error(`Unexpected IPC method: ${method}`);
      },
    } as unknown as IpcClient;
    return { client, sentRequests };
  }

  test('T043: getOrientation and setOrientation dispatch RPC and notify listeners', async () => {
    const mockProfile = {
      orientation: 'ai_oriented' as DocumentationOrientation,
      cohesionWeight: 0.15,
      couplingWeight: 0.15,
      concentrationWeight: 0.1,
      contextEfficiencyWeight: 0.35,
      discoverabilityWeight: 0.25,
      narrativeCoherenceWeight: 0.05,
      maintenanceSimplicityWeight: 0.1,
      maxTraceDepth: 4,
      maxContextSurfaceTokens: 12000,
      fragmentSizeTarget: 3000,
      splitSensitivity: 0.8,
      canonicalOwnershipSensitivity: 0.75,
      constraints: [],
    };

    const getRes: GetOrientationResponse = {
      orientation: 'ai_oriented',
      profile: mockProfile,
    };
    const setRes: SetOrientationResponse = {
      success: true,
      orientation: 'human_oriented',
      profile: { ...mockProfile, orientation: 'human_oriented', fragmentSizeTarget: 8000 },
    };

    const { client, sentRequests } = createMockIpcClient({
      'profile/getOrientation': getRes,
      'profile/setOrientation': setRes,
    });

    const manager = new EvolutionManager(client);

    // Initial get
    const initial = await manager.getOrientation();
    assert.strictEqual(initial.orientation, 'ai_oriented');
    assert.strictEqual(manager.getCachedOrientation(), 'ai_oriented');
    assert.strictEqual(manager.getCachedProfile()?.fragmentSizeTarget, 3000);

    // Subscribe to changes
    let notifiedOrientation: DocumentationOrientation | null = null;
    manager.onDidChangeOrientation((o) => {
      notifiedOrientation = o;
    });

    // Set orientation
    const updated = await manager.setOrientation('human_oriented');
    assert.strictEqual(updated.orientation, 'human_oriented');
    assert.strictEqual(manager.getCachedOrientation(), 'human_oriented');
    assert.strictEqual(manager.getCachedProfile()?.fragmentSizeTarget, 8000);
    assert.strictEqual(notifiedOrientation, 'human_oriented');

    assert.strictEqual(sentRequests.length, 2);
    assert.strictEqual(sentRequests[0].method, 'profile/getOrientation');
    assert.strictEqual(sentRequests[1].method, 'profile/setOrientation');
  });

  test('T044: Custom profile and constraint synchronization logic', async () => {
    const mockCustomConfig: GetCustomConfigResponse = {
      tier1: {
        aiDiscoverability: 'high',
        contextEfficiency: 'high',
        humanReadability: 'medium',
        knowledgeReuse: 'high',
        narrativeCoherence: 'low',
        maintenanceSimplicity: 'medium',
      },
      tier2: {
        cohesionThreshold: 0.75,
        semanticDiversityThreshold: 0.7,
        referenceConcentrationThreshold: 0.65,
        fragmentSizeTarget: 4000,
        maxTraceDepth: 4,
        candidateExpansionFactor: 1.5,
        maxContextSurfaceTokens: 15000,
        splitSensitivity: 0.7,
        canonicalOwnershipSensitivity: 0.8,
        duplicateTolerance: 0.15,
        fragmentationTolerance: 0.2,
      },
      constraints: [
        {
          id: 'c1',
          level: 'hard_constraint',
          ruleKey: 'never_split_canonical_docs',
          description: 'Never split canonical documents',
        },
      ],
    };

    const updateRes: UpdateCustomConfigResponse = {
      success: true,
      profile: {
        orientation: 'custom',
        cohesionWeight: 0.2,
        couplingWeight: 0.2,
        concentrationWeight: 0.1,
        contextEfficiencyWeight: 0.3,
        discoverabilityWeight: 0.2,
        narrativeCoherenceWeight: 0.1,
        maintenanceSimplicityWeight: 0.1,
        maxTraceDepth: 4,
        maxContextSurfaceTokens: 15000,
        fragmentSizeTarget: 4000,
        splitSensitivity: 0.7,
        canonicalOwnershipSensitivity: 0.8,
        constraints: mockCustomConfig.constraints,
      },
    };

    const validateRes: ValidateConstraintsResponse = {
      isValid: true,
      conflicts: [],
    };

    const { client } = createMockIpcClient({
      'profile/getCustomConfig': mockCustomConfig,
      'profile/updateCustomConfig': updateRes,
      'profile/validateConstraints': validateRes,
    });

    const manager = new EvolutionManager(client);

    const config = await manager.getCustomConfig();
    assert.strictEqual(config.tier1.aiDiscoverability, 'high');
    assert.strictEqual(config.tier2.fragmentSizeTarget, 4000);

    const updated = await manager.updateCustomConfig(config.tier1, config.tier2, config.constraints);
    assert.strictEqual(updated.success, true);
    assert.strictEqual(updated.profile.orientation, 'custom');

    const validation = await manager.validateConstraints(config.constraints);
    assert.strictEqual(validation.isValid, true);
    assert.strictEqual(validation.conflicts.length, 0);
  });

  test('T045: Problem and recommendation subscription store', async () => {
    const problemsRes: GetStructuralProblemsResponse = {
      signatures: [
        {
          signatureId: 'sig_god_1',
          documentPath: 'docs/god.md',
          problemType: 'god_document',
          confidence: 0.95,
          evidence: {
            cohesion: 0.6,
            coupling: 0.8,
            referenceConcentration: 0.75,
            granularity: 0.3,
            contextEfficiency: 0.25,
            discoverability: 0.8,
            reusability: 0.2,
            semanticDiversity: 0.85,
            navigationalComplexity: 0.7,
            maintenanceCoupling: 0.65,
          },
          rationale: 'High semantic diversity and concentrated references.',
        },
      ],
    };

    const recsRes: GetRecommendationsResponse = {
      recommendations: [
        {
          recommendationId: 'rec_1',
          signatureId: 'sig_god_1',
          strategy: 'topic_decomposition',
          confidence: 0.92,
          expectedBenefits: ['Eliminates God Document by splitting topics'],
          tradeOffs: ['Increases file count'],
        },
      ],
    };

    const { client } = createMockIpcClient({
      'analytics/getStructuralProblems': problemsRes,
      'recommendation/getRecommendations': recsRes,
    });

    const manager = new EvolutionManager(client);

    let observedProblemsCount = 0;
    manager.onDidUpdateProblems((probs) => {
      observedProblemsCount = probs.length;
    });

    let observedRecsCount = 0;
    manager.onDidUpdateRecommendations((recs) => {
      observedRecsCount = recs.length;
    });

    const problems = await manager.getStructuralProblems();
    assert.strictEqual(problems.length, 1);
    assert.strictEqual(observedProblemsCount, 1);
    assert.strictEqual(manager.getCachedProblems().length, 1);

    const recs = await manager.getRecommendations();
    assert.strictEqual(recs.length, 1);
    assert.strictEqual(observedRecsCount, 1);
    assert.strictEqual(manager.getCachedRecommendations()[0].strategy, 'topic_decomposition');
  });

  test('T046: Transformation plan generation and user approval workflow (Invariant XVI)', async () => {
    const mockPlan: GeneratePlanResponse = {
      planId: 'plan_decomp_1',
      recommendationId: 'rec_1',
      actions: [
        {
          type: 'create_document',
          payload: {
            filePath: 'docs/subtopic.md',
            initialContent: '# Subtopic\n',
          },
        },
      ],
      diffPreview: '+++ docs/subtopic.md\n@@ -0,0 +1,1 @@\n+# Subtopic',
      projectedDelta: {
        efficiencyDelta: 0.2,
        traceDepthDelta: -0.5,
        couplingDelta: -0.15,
        docCountDelta: 1,
      },
      requiresApproval: true,
    };

    const mockApplyRes: ApplyPlanResponse = {
      success: true,
      actualDelta: mockPlan.projectedDelta,
    };

    const { client } = createMockIpcClient({
      'transformation/generatePlan': mockPlan,
      'transformation/applyPlan': mockApplyRes,
    });

    // Case 1: User approves transformation
    let confirmedPlanId = '';
    const approvingBridge: EvolutionUiBridge = {
      confirmPlanApproval: async (id) => {
        confirmedPlanId = id;
        return true;
      },
    };

    const manager1 = new EvolutionManager(client, approvingBridge);
    const plan = await manager1.generatePlan('rec_1');
    assert.strictEqual(plan.planId, 'plan_decomp_1');
    assert.strictEqual(plan.requiresApproval, true);

    const applyResult = await manager1.applyPlan(plan);
    assert.strictEqual(applyResult.success, true);
    assert.strictEqual(confirmedPlanId, 'plan_decomp_1');

    // Case 2: User rejects / cancels transformation
    const rejectingBridge: EvolutionUiBridge = {
      confirmPlanApproval: async () => false,
    };
    const manager2 = new EvolutionManager(client, rejectingBridge);

    await assert.rejects(
      async () => manager2.applyPlan(plan),
      /cancelled by the user/,
      'Must abort when user denies approval'
    );
  });

  test('T047: Git index staging coordination after transformation application', async () => {
    const mockPlan: TransformationPlan = {
      planId: 'plan_git_stage',
      recommendationId: 'rec_1',
      actions: [
        {
          type: 'create_document',
          payload: {
            filePath: 'docs/new_chapter.md',
            initialContent: '# New Chapter\n',
          },
        },
        {
          type: 'rewrite_link_references',
          payload: {
            sourcePath: 'docs/main.md',
            oldTarget: '[[Old]]',
            newTarget: '[[New]]',
          },
        },
      ],
      diffPreview: 'diff',
      projectedDelta: {
        efficiencyDelta: 0.1,
        traceDepthDelta: 0,
        couplingDelta: 0,
        docCountDelta: 1,
      },
      requiresApproval: true,
    };

    const mockApplyRes: ApplyPlanResponse = {
      success: true,
      actualDelta: mockPlan.projectedDelta,
    };

    const { client } = createMockIpcClient({
      'transformation/applyPlan': mockApplyRes,
    });

    const stagedList: string[] = [];
    let stagedRoot = '';

    const stagingBridge: EvolutionUiBridge = {
      confirmPlanApproval: async () => true,
      stageFiles: async (root, files) => {
        stagedRoot = root;
        stagedList.push(...files);
      },
    };

    const manager = new EvolutionManager(client, stagingBridge);
    const result = await manager.applyPlan(mockPlan, { rootPath: '/vault' });

    assert.strictEqual(result.success, true);
    assert.strictEqual(stagedRoot, '/vault');
    assert.deepStrictEqual(stagedList, ['docs/new_chapter.md', 'docs/main.md']);
  });
});
