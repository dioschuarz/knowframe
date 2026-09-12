import type { IpcClient } from '../ipc/client.ts';
import type {
  CandidateValidationReport,
  ContextTraceParams,
  ContextTraceResult,
  KnowledgeDebtBaselineDetails,
  KnowledgeQualityScore,
  UpdateBaselineParams,
  UpdateBaselineResult,
  ValidateCandidateParams,
} from '../types/admission.ts';

/**
 * AdmissionManager coordinates pre-commit candidate validation,
 * debt baseline lifecycle, context trace, and knowledge quality score.
 */
export class AdmissionManager {
  private readonly ipcClient: IpcClient;

  constructor(ipcClient: IpcClient) {
    this.ipcClient = ipcClient;
  }

  /**
   * Evaluates staged changes against candidate graph overlay (pre-commit).
   */
  public async validateCandidate(
    params: ValidateCandidateParams,
  ): Promise<CandidateValidationReport> {
    return this.ipcClient.sendRequest<CandidateValidationReport>(
      'workspace/validateCandidate',
      params,
    );
  }

  /**
   * Fetches current Knowledge Debt Baseline details and defect counts.
   */
  public async getBaseline(): Promise<KnowledgeDebtBaselineDetails> {
    return this.ipcClient.sendRequest<KnowledgeDebtBaselineDetails>(
      'workspace/getBaseline',
    );
  }

  /**
   * Captures a fresh baseline snapshot or ratchets down resolved debt.
   */
  public async updateBaseline(
    params?: UpdateBaselineParams,
  ): Promise<UpdateBaselineResult> {
    return this.ipcClient.sendRequest<UpdateBaselineResult>(
      'workspace/updateBaseline',
      params,
    );
  }

  /**
   * Performs graph-constrained context trace from designated entrypoint.
   */
  public async traceContext(
    params: ContextTraceParams,
  ): Promise<ContextTraceResult> {
    return this.ipcClient.sendRequest<ContextTraceResult>(
      'workspace/traceContext',
      params,
    );
  }

  /**
   * Computes network-aware 4-pillar Knowledge Quality Score.
   */
  public async getQualityScore(): Promise<KnowledgeQualityScore> {
    return this.ipcClient.sendRequest<KnowledgeQualityScore>(
      'workspace/getQualityScore',
    );
  }
}
