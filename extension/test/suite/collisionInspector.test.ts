import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderCollisionInspectorHtml } from '../../webview-ui/src/components/collisionInspector.ts';

suite('CollisionInspector Component Test Suite', () => {
  test('Renders collision card with similarity percentage, shared terms, and canonical candidate', () => {
    const html = renderCollisionInspectorHtml({
      sourceNote: 'docs/new_auth.md',
      targetNote: 'docs/auth.md',
      compositeSimilarity: 0.912,
      classification: 'potential_duplicate',
      sharedTerms: ['authentication', 'jwt', 'session', 'token'],
      canonicalCandidate: 'docs/auth.md',
      canonicalConfidence: 0.88,
      canonicalEvidence: [
        'Referenced from entrypoint AGENTS.md',
        'Higher graph betweenness centrality (0.65 vs 0.00)',
      ],
    });

    assert.ok(html.includes('Semantic Collision Detected'), 'Must show collision header');
    assert.ok(html.includes('91.2%'), 'Must show composite similarity percentage');
    assert.ok(html.includes('badge-block'), 'Must apply blocker badge class for >= 85%');
    assert.ok(html.includes('docs/new_auth.md'), 'Must display candidate note');
    assert.ok(html.includes('docs/auth.md'), 'Must display existing note');
    assert.ok(html.includes('jwt'), 'Must render shared term chip');
    assert.ok(html.includes('88% confidence'), 'Must show canonical confidence');
    assert.ok(html.includes('Propose as Canonical'), 'Must render propose canonical action');
  });

  test('Applies badge-warn for moderate overlap (< 85%)', () => {
    const html = renderCollisionInspectorHtml({
      sourceNote: 'docs/auth_guide.md',
      targetNote: 'docs/auth.md',
      compositeSimilarity: 0.785,
      classification: 'potential_specialization',
      sharedTerms: ['authentication'],
      canonicalCandidate: 'docs/auth.md',
    });

    assert.ok(html.includes('78.5%'), 'Must show 78.5% similarity');
    assert.ok(html.includes('badge-warn'), 'Must apply warning badge class for < 85%');
  });
});
