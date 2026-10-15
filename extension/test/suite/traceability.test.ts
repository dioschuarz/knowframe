import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { EngineRpcClient } from '../../src/rpc/client.ts';
import { renderEvolutionTabHtml } from '../../webview-ui/src/components/evolutionTab.ts';
import { renderNodeInspectionHtml } from '../../webview-ui/src/components/inspectorDrawer.ts';
import type { NoteNode } from '../../src/types/graph.ts';
import type {
  GetAffectedDocumentsParams,
  ResolveDocumentReviewParams,
  AffectedDocumentsReport,
  ResolveDocumentReviewResult,
} from '../../src/types/engine.ts';

suite('Traceability & PR/CI Review Lifecycle Test Suite (T032, T033, FR-007, FR-008, FR-016)', () => {
  test('EngineRpcClient sends trace/getAffectedDocuments and parses response (T031, FR-016)', async () => {
    const mockReport: AffectedDocumentsReport = {
      affected_documents: [
        {
          doc_id: 'notes/architecture.md',
          doc_path: 'notes/architecture.md',
          status: 'pending',
          changed_code_paths: ['crates/core/src/main.rs'],
          reasons: ['Direct code dependency modified: crates/core/src/main.rs'],
        },
      ],
      obligations: [
        {
          doc_id: 'notes/architecture.md',
          changed_code_paths: ['crates/core/src/main.rs'],
          dependency_sources: ['frontmatter'],
          rule_id: null,
          status: 'pending',
        },
      ],
      unreviewed_count: 1,
      gate_decision: 'BLOCK',
    };

    const engineClient = new EngineRpcClient();
    const params: GetAffectedDocumentsParams = {
      base_ref: 'main',
      head_ref: 'HEAD',
      staged: true,
    };

    const reqPromise = engineClient.getAffectedDocuments(params);

    engineClient.handleMessage(JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      result: mockReport,
    }));

    const result = await reqPromise;

    assert.strictEqual(result.gate_decision, 'BLOCK');
    assert.strictEqual(result.unreviewed_count, 1);
    assert.strictEqual(result.affected_documents[0].doc_id, 'notes/architecture.md');
    assert.strictEqual(result.affected_documents[0].status, 'pending');
    engineClient.dispose();
  });

  test('EngineRpcClient sends review/resolveDocumentReview and parses result (T031, FR-016)', async () => {
    const mockResult: ResolveDocumentReviewResult = {
      resolved_count: 1,
      receipts_persisted: 1,
      storage_path: '.context-vault/reviews.json',
      active_head_sha: 'a1b2c3d4e5f6',
    };

    const engineClient = new EngineRpcClient();
    const params: ResolveDocumentReviewParams = {
      doc_ids: ['notes/architecture.md'],
      reason: 'Reviewed and confirmed current with crates/core/src/main.rs',
    };

    const reqPromise = engineClient.resolveDocumentReview(params);

    engineClient.handleMessage(JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      result: mockResult,
    }));

    const result = await reqPromise;

    assert.strictEqual(result.resolved_count, 1);
    assert.strictEqual(result.receipts_persisted, 1);
    assert.strictEqual(result.storage_path, '.context-vault/reviews.json');
    engineClient.dispose();
  });

  test('Evolution tab renders Review Obligation badge and [Mark Reviewed] button for review_required findings (T032, FR-007, FR-008)', () => {
    const html = renderEvolutionTabHtml({
      activeOrientation: 'ai_context_efficiency',
      vaultFindings: [
        {
          file_path: 'notes/architecture.md',
          category: 'review_required',
          description: 'Document references crates/core/src/main.rs which was modified in Git diff',
          remediation_hint: 'Verify architectural note and click [Mark Reviewed] to clear review obligation',
          review_status: 'pending',
          pillar: 'freshness',
        },
      ],
      structuralSignatures: [],
      structuralRecommendations: [],
    });

    assert.ok(html.includes('Review: pending') || html.includes('Review: Required'), 'Must render review status badge');
    assert.ok(html.includes('data-action="mark-reviewed"'), 'Must render [Mark Reviewed] action hook');
    assert.ok(html.includes('btn-mark-reviewed'), 'Must include btn-mark-reviewed class');
    assert.ok(html.includes('data-doc-id="notes/architecture.md"'), 'Must bind doc-id to button data attribute');
  });

  test('Inspector drawer renders Review Status badge and [Mark Reviewed] action in footer (T032, FR-007)', () => {
    const sampleNode: NoteNode = {
      id: 'notes/03_architecture/pipeline.md',
      label: 'Pipeline Architecture',
      filePath: 'notes/03_architecture/pipeline.md',
      tags: ['architecture', 'pipeline'],
      color: '#38bdf8',
      status: 'active',
      tokens: 3400,
      hopDistance: 1,
      inDegree: 4,
      outDegree: 2,
      betweenness: 0.12,
      brokenLinks: 0,
      reviewStatus: 'pending',
    };

    const html = renderNodeInspectionHtml({
      node: sampleNode,
      inLinks: ['notes/00_meta/wiki_index.md'],
      outLinks: ['notes/01_projects/core.md'],
      isHub: false,
    });

    assert.ok(html.includes('insp-review-badge'), 'Must render review status badge in node inspector');
    assert.ok(html.includes('Review: pending'), 'Must display review status pending text');
    assert.ok(html.includes('insp-btn-mark-reviewed'), 'Must render [Mark Reviewed] button in footer');
  });
});
