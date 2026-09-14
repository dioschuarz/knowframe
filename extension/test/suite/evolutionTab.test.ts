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

  test('renders raw snake_case Rust objects without errors or undefined badges', () => {
    const rawRustProps: any = {
      signatures: [
        {
          signature_id: 'sig_god_raw',
          document_path: 'docs/raw_god.md',
          problem_type: 'god_document',
          confidence: 0.88,
          evidence: {
            cohesion: 0.5,
            coupling: 0.8,
            reference_concentration: 0.72,
            granularity: 0.4,
            context_efficiency: 0.3,
            discoverability: 0.7,
            reusability: 0.3,
            semantic_diversity: 0.8,
            navigational_complexity: 0.6,
            maintenance_coupling: 0.65,
          },
          rationale: 'Overloaded document in raw Rust format.',
        },
      ],
      recommendations: [
        {
          recommendation_id: 'rec_raw_1',
          signature_id: 'sig_god_raw',
          strategy: 'topic_decomposition',
          confidence: 0.85,
          expected_benefits: ['Decomposes raw document'],
          trade_offs: ['More files'],
        },
      ],
    };

    const html = renderEvolutionTabHtml(rawRustProps);
    assert.ok(html.includes('docs/raw_god.md'), 'Must render document path from document_path');
    assert.ok(html.includes('God Document'), 'Must render badge from problem_type');
    assert.ok(html.includes('data-signature-id="sig_god_raw"'), 'Must render signature id');
    assert.ok(html.includes('data-rec-id="rec_raw_1"'), 'Must render recommendation id');
    assert.ok(html.includes('Decomposes raw document'), 'Must render expected_benefits without crashing');
    assert.ok(html.includes('More files'), 'Must render trade_offs without crashing');
    assert.ok(!html.includes('undefined'), 'Must never output literal undefined in badges or text');
  });
});
