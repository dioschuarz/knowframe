import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderIssueResolutionModalHtml } from '../../webview-ui/src/components/issueResolutionModal.ts';
import type { KnowledgeIssue } from '../../src/types/admission.ts';

suite('IssueResolutionModal Component Test Suite', () => {
  test('renders broken reference modal with root cause, trade-offs, and action buttons', () => {
    const issue: KnowledgeIssue = {
      category: 'broken_reference',
      description: 'The link points to a non-existent document "missing_doc.md"',
      file_path: 'notes/architecture.md',
      target: 'missing_doc.md',
      line_number: 42,
      deduction_points: 5,
      pillar: 'structure',
    };

    const html = renderIssueResolutionModalHtml({
      issue,
      nodeLabel: 'Architecture',
      nodeFilePath: 'notes/architecture.md',
    });

    assert.ok(html.includes('Broken Reference Resolution'), 'Should format modal title');
    assert.ok(html.includes('Line 42'), 'Should display line coordinate');
    assert.ok(html.includes('-5 pts'), 'Should display penalty deduction');
    assert.ok(html.includes('missing_doc.md'), 'Should display target reference');
    assert.ok(html.includes('notes/architecture.md'), 'Should display source file path');
    assert.ok(html.includes('Re-target Reference to Existing Note'), 'Should render recommended option');
    assert.ok(html.includes('RECOMMENDED'), 'Should display recommended badge');
    assert.ok(html.includes('Trade-off:'), 'Should surface trade-offs');
    assert.ok(html.includes('issue-btn-jump-line'), 'Should render jump line button');
    assert.ok(html.includes('issue-btn-fix-ref'), 'Should render fix reference action button');
  });

  test('renders freshness / staleness modal with mark reviewed action', () => {
    const issue: KnowledgeIssue = {
      category: 'stale_concept',
      description: 'Document references deprecated schema that requires verification',
      file_path: 'notes/guide.md',
      line_number: 15,
      deduction_points: 3,
      pillar: 'freshness',
      remediation_hint: 'Review document content and mark resolved',
    };

    const html = renderIssueResolutionModalHtml({
      issue,
      nodeLabel: 'Guide',
      nodeFilePath: 'notes/guide.md',
    });

    assert.ok(html.includes('Stale Concept Resolution'), 'Should format title');
    assert.ok(html.includes('Review and Confirm Accuracy'), 'Should present review option');
    assert.ok(html.includes('issue-btn-mark-reviewed'), 'Should render mark reviewed button');
  });
});
