import type { IpcClient } from '../ipc/client.ts';
import type {
  DocumentationOrientation,
  GetOrientationResponse,
  SetOrientationResponse,
  GetCustomConfigResponse,
  UpdateCustomConfigResponse,
  ValidateConstraintsResponse,
  GetStructuralProblemsResponse,
  StructuralProblemSignature,
  GetRecommendationsResponse,
  StructuralRecommendation,
  GeneratePlanResponse,
  TransformationPlan,
  ApplyPlanResponse,
  ConstraintRule,
  Tier1Objectives,
  Tier2Parameters,
  OptimizationProfile,
  StrategySet,
  ValidateSelectionResponse,
  CalculatePlanValueResponse,
  KnowledgeOrganizationStrategy,
} from '../types/evolution.ts';

export interface EvolutionUiBridge {
  confirmPlanApproval(planId: string): Promise<boolean>;
  stageFiles?(rootPath: string, files: string[]): Promise<void>;
  log?(message: string): void;
}

export type Listener<T> = (data: T) => void;

/**
 * EvolutionManager coordinates Knowledge Organization Systems (KOS),
 * Documentation Orientations, Custom Profiles, Problem Signatures,
 * Recommendations, and User-Approved Transformations (Invariant XIII - XVI).
 */
export class EvolutionManager {
  private readonly ipcClient: IpcClient;
  private readonly uiBridge?: EvolutionUiBridge;

  // Cached state
  private activeOrientation: DocumentationOrientation = 'ai_oriented';
  private activeProfile: OptimizationProfile | null = null;
  private cachedSignatures: StructuralProblemSignature[] = [];
  private cachedRecommendations: StructuralRecommendation[] = [];

  // Subscription listeners
  private orientationListeners: Set<Listener<DocumentationOrientation>> = new Set();
  private problemListeners: Set<Listener<StructuralProblemSignature[]>> = new Set();
  private recommendationListeners: Set<Listener<StructuralRecommendation[]>> = new Set();
  private transformationListeners: Set<Listener<ApplyPlanResponse>> = new Set();

  constructor(ipcClient: IpcClient, uiBridge?: EvolutionUiBridge) {
    this.ipcClient = ipcClient;
    this.uiBridge = uiBridge;
  }

  // --- Phase 8 / T043: Active orientation state & RPC dispatching ---

  public async getOrientation(): Promise<GetOrientationResponse> {
    const res = await this.ipcClient.sendRequest<GetOrientationResponse>('profile/getOrientation');
    this.activeOrientation = res.orientation;
    this.activeProfile = res.profile;
    return res;
  }

  public async setOrientation(orientation: DocumentationOrientation): Promise<SetOrientationResponse> {
    const res = await this.ipcClient.sendRequest<SetOrientationResponse>('profile/setOrientation', { orientation });
    this.activeOrientation = res.orientation;
    this.activeProfile = res.profile;
    this.notifyOrientationListeners(res.orientation);
    return res;
  }

  public getCachedOrientation(): DocumentationOrientation {
    return this.activeOrientation;
  }

  public getCachedProfile(): OptimizationProfile | null {
    return this.activeProfile;
  }

  public onDidChangeOrientation(listener: Listener<DocumentationOrientation>): () => void {
    this.orientationListeners.add(listener);
    return () => this.orientationListeners.delete(listener);
  }

  private notifyOrientationListeners(orientation: DocumentationOrientation): void {
    for (const l of this.orientationListeners) {
      try {
        l(orientation);
      } catch (e) {
        // Safe dispatch
      }
    }
  }

  // --- Phase 8 / T044: Custom profile and constraint synchronization ---

  public async getCustomConfig(): Promise<GetCustomConfigResponse> {
    return this.ipcClient.sendRequest<GetCustomConfigResponse>('profile/getCustomConfig');
  }

  public async updateCustomConfig(
    tier1: Tier1Objectives,
    tier2: Tier2Parameters,
    constraints: ConstraintRule[],
  ): Promise<UpdateCustomConfigResponse> {
    const res = await this.ipcClient.sendRequest<UpdateCustomConfigResponse>('profile/updateCustomConfig', {
      tier1,
      tier2,
      constraints,
    });
    this.activeProfile = res.profile;
    return res;
  }

  public async validateConstraints(constraints: ConstraintRule[]): Promise<ValidateConstraintsResponse> {
    return this.ipcClient.sendRequest<ValidateConstraintsResponse>('profile/validateConstraints', { constraints });
  }

  // --- Phase 8 / T045: Structural problem and recommendation subscription store ---

  public async getStructuralProblems(filePath?: string): Promise<StructuralProblemSignature[]> {
    const res = await this.ipcClient.sendRequest<GetStructuralProblemsResponse>('analytics/getStructuralProblems', {
      file_path: filePath,
    });
    const changed = JSON.stringify(this.cachedSignatures) !== JSON.stringify(res.signatures);
    this.cachedSignatures = res.signatures;
    if (changed) {
      for (const l of this.problemListeners) {
        try {
          l(res.signatures);
        } catch (e) {
          // Safe dispatch
        }
      }
    }
    return res.signatures;
  }

  public async getRecommendations(signatureId?: string): Promise<StructuralRecommendation[]> {
    const res = await this.ipcClient.sendRequest<GetRecommendationsResponse>('recommendation/getRecommendations', {
      signature_id: signatureId,
    });
    const changed = JSON.stringify(this.cachedRecommendations) !== JSON.stringify(res.recommendations);
    this.cachedRecommendations = res.recommendations;
    if (changed) {
      for (const l of this.recommendationListeners) {
        try {
          l(res.recommendations);
        } catch (e) {
          // Safe dispatch
        }
      }
    }
    return res.recommendations;
  }

  public getCachedProblems(): StructuralProblemSignature[] {
    return this.cachedSignatures;
  }

  public getCachedRecommendations(): StructuralRecommendation[] {
    return this.cachedRecommendations;
  }

  public onDidUpdateProblems(listener: Listener<StructuralProblemSignature[]>): () => void {
    this.problemListeners.add(listener);
    return () => this.problemListeners.delete(listener);
  }

  public onDidUpdateRecommendations(listener: Listener<StructuralRecommendation[]>): () => void {
    this.recommendationListeners.add(listener);
    return () => this.recommendationListeners.delete(listener);
  }

  // --- Phase 8 / T046: Transformation plan confirmation workflow ---

  public async generatePlan(recommendationId: string): Promise<TransformationPlan> {
    return this.ipcClient.sendRequest<GeneratePlanResponse>('transformation/generatePlan', {
      recommendation_id: recommendationId,
    });
  }

  public async applyPlan(
    plan: TransformationPlan | string,
    options?: { skipDialog?: boolean; rootPath?: string },
  ): Promise<ApplyPlanResponse> {
    const planId = typeof plan === 'string' ? plan : plan.planId;
    const requiresApproval = typeof plan === 'string' ? true : plan.requiresApproval;

    // Invariant XVI: Strict user approval gate
    if (requiresApproval && !options?.skipDialog) {
      if (this.uiBridge) {
        const approved = await this.uiBridge.confirmPlanApproval(planId);
        if (!approved) {
          throw new Error(`Transformation plan '${planId}' execution was cancelled by the user.`);
        }
      }
    }

    const payload: { plan_id: string; plan?: TransformationPlan } = { plan_id: planId };
    if (typeof plan !== 'string') {
      payload.plan = plan;
    }

    const res = await this.ipcClient.sendRequest<ApplyPlanResponse>('transformation/applyPlan', payload);

    // T047: Git index staging coordination
    if (options?.rootPath && this.uiBridge?.stageFiles) {
      const filesToStage: string[] = [];
      if (typeof plan !== 'string' && plan.actions) {
        for (const action of plan.actions) {
          const p = action.payload as any;
          if (p.filePath) {
            filesToStage.push(p.filePath);
          }
          if (p.sourcePath) {
            filesToStage.push(p.sourcePath);
          }
        }
      }
      if (filesToStage.length > 0) {
        await this.uiBridge.stageFiles(options.rootPath, filesToStage);
      }
    }

    for (const l of this.transformationListeners) {
      try {
        l(res);
      } catch (e) {
        // Safe dispatch
      }
    }

    return res;
  }

  public onDidApplyTransformation(listener: Listener<ApplyPlanResponse>): () => void {
    this.transformationListeners.add(listener);
    return () => this.transformationListeners.delete(listener);
  }

  // --- Feature 007 / Strategy Selection & Combination ---

  public async getStrategySet(opportunityId: string): Promise<StrategySet> {
    return this.ipcClient.sendRequest<StrategySet>('evolution/getStrategySet', {
      opportunity_id: opportunityId,
    });
  }

  public async validateStrategySelection(
    selectedStrategies: KnowledgeOrganizationStrategy[],
  ): Promise<ValidateSelectionResponse> {
    return this.ipcClient.sendRequest<ValidateSelectionResponse>('evolution/validateStrategySelection', {
      selected_strategies: selectedStrategies,
    });
  }

  public async calculatePlanValue(
    opportunityId: string,
    selectedStrategies: KnowledgeOrganizationStrategy[],
  ): Promise<CalculatePlanValueResponse> {
    return this.ipcClient.sendRequest<CalculatePlanValueResponse>('evolution/calculatePlanValue', {
      opportunity_id: opportunityId,
      selected_strategies: selectedStrategies,
    });
  }

  public async generateMultiStrategyPlan(
    opportunityId: string,
    selectedStrategies: KnowledgeOrganizationStrategy[],
  ): Promise<TransformationPlan> {
    return this.ipcClient.sendRequest<TransformationPlan>('evolution/generatePlan', {
      opportunity_id: opportunityId,
      selected_strategies: selectedStrategies,
    });
  }

  public dispose(): void {
    this.orientationListeners.clear();
    this.problemListeners.clear();
    this.recommendationListeners.clear();
    this.transformationListeners.clear();
  }
}
