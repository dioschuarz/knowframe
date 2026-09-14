import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  renderEvolutionTabHtml,
  formatMetricValue,
  getProblemTypeBadge,
  type EvolutionTabProps,
} from '../../webview-ui/src/components/evolutionTab.ts';

suite('EvolutionTab Unit Test Suite (T053)', () => {
  const sampleProps: EvolutionTabProps = {
    signatures: [
      {
        signatureId: 'sig_god_1',
        documentPath: 'docs/architecture_monolith.md',
        problemType: 'god_document',
        confidence: 0.94,
        evidence: {
          cohesion: 0.55,
          coupling: 0.85,
          referenceConcentration: 0.78,
          granularity: 0.32,
          contextEfficiency: 0.28,
          discoverability: 0.75,
          reusability: 0.22,
          semanticDiversity: 0.88,
          navigationalComplexity: 0.65,
          maintenanceCoupling: 0.7,
        },
        rationale: 'Overloaded document with divergent topics and concentrated references.',
      },
    ],
    recommendations: [
      {
        recommendationId: 'rec_decomp_1',
        signatureId: 'sig_god_1',
        strategy: 'topic_decomposition',
        confidence: 0.91,
        expectedBenefits: [
          'Isolates topics into standalone modular notes',
          'Reduces prompt context overhead during traces',
        ],
        tradeOffs: [
          'Increases document node count in vault',
          'Requires updating outbound references',
        ],
      },
      {
        recommendationId: 'rec_adv_1',
        signatureId: 'sig_god_1',
        strategy: 'lightweight_index',
        confidence: 0.62, // Advisory < 0.70
        expectedBenefits: ['Introduces index router note'],
        tradeOffs: ['Maintains existing monolithic text'],
      },
    ],
  };

  test('renders empty state when no problem signatures are detected', () => {
    const html = renderEvolutionTabHtml({ signatures: [], recommendations: [] });
    assert.ok(html.includes('Vault Structure is Balanced'));
    assert.ok(html.includes('No structural problem signatures'));
  });

  test('renders problem signature cards with badges and confidence', () => {
    const html = renderEvolutionTabHtml(sampleProps);

    assert.ok(html.includes('docs/architecture_monolith.md'));
    assert.ok(html.includes('God Document'));
    assert.ok(html.includes('94%'));
    assert.ok(html.includes('Overloaded document with divergent topics'));
  });

  test('enforces raw numeric integrity presentation invariant (<= 5 chars)', () => {
    const html = renderEvolutionTabHtml(sampleProps);

    // All evidence pills must have values formatted <= 5 characters
    const matches = html.matchAll(/<span class="ep-val">([^<]+)<\/span>/g);
    let count = 0;
    for (const match of matches) {
      const valStr = match[1];
      assert.ok(valStr.length <= 5, `Metric '${valStr}' exceeds 5 chars`);
      count++;
    }
    assert.strictEqual(count, 10, 'Must render all 10 structural dimensions');
  });

  test('renders candidate recommendations with trade-off matrices and advisory labels', () => {
    const html = renderEvolutionTabHtml(sampleProps);

    assert.ok(html.includes('Topic Decomposition'));
    assert.ok(html.includes('91% Fit'));
    assert.ok(html.includes('Isolates topics into standalone modular notes'));
    assert.ok(html.includes('Increases document node count'));

    // Advisory recommendation (< 0.70)
    assert.ok(html.includes('Lightweight Index'));
    assert.ok(html.includes('62% Fit'));
    assert.ok(html.includes('Advisory'));

    // Generate Plan button
    assert.ok(html.includes('data-rec-id="rec_decomp_1"'));
    assert.ok(html.includes('Generate Transformation Plan'));
  });
});
