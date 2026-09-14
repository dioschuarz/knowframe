import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderConflictBannerHtml } from '../../webview-ui/src/components/conflictBanner.ts';
import type { ConstraintConflict } from '../../src/types/evolution.ts';

suite('Conflicting Constraint Banner Unit Test Suite (T052)', () => {
  test('renders clean green status when no conflicts are present', () => {
    const html = renderConflictBannerHtml({ conflicts: [] });

    assert.ok(html.includes('constraint-status-valid'));
    assert.ok(html.includes('All active constraint rules are mutually satisfiable'));
  });

  test('surfaces conflicting rules, reasons, and remediation suggestions', () => {
    const sampleConflicts: ConstraintConflict[] = [
      {
        conflictingRules: ['never_split_canonical_docs', 'always_decompose_large_files'],
        reason: 'Rules directly contradict: cannot simultaneously forbid and mandate document splitting.',
        suggestion: 'Lower one rule to a Preference level or remove the contradictory constraint.',
      },
    ];

    const html = renderConflictBannerHtml({ conflicts: sampleConflicts });

    assert.ok(html.includes('constraint-conflict-banner'));
    assert.ok(html.includes('Conflicting Hard Constraints Detected (1)'));
    assert.ok(html.includes('never_split_canonical_docs'));
    assert.ok(html.includes('always_decompose_large_files'));
    assert.ok(html.includes('Rules directly contradict'));
    assert.ok(html.includes('Lower one rule to a Preference level'));
  });
});
