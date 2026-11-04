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

  test('renders Review & Sync Harnesses action button with data-target for harness route drift', () => {
    const propsWithDrift: any = {
      signatures: [],
      recommendations: [],
      structureFindings: [
        {
          file_path: '08_showcases/mcp-runner/.agents/rules/GEMINI.md',
          category: 'harness_drift',
          debt_type: 'harness_drift',
          description: "AI harness route drift: '08_showcases/mcp-runner/.agents/rules/GEMINI.md' has 0 routes but is missing 19 route(s) configured in peer harnesses.",
          remediation_hint: "Restructure document or materialize fragment metadata with 'llm-wiki-core materialize'",
        },
      ],
    };

    const html = renderEvolutionTabHtml(propsWithDrift);
    assert.ok(html.includes('btn-gov-sync-harnesses'), 'Must render sync harnesses button class');
    assert.ok(html.includes('data-target="08_showcases/mcp-runner/.agents/rules/GEMINI.md"'), 'Must specify target harness file in data-target');
    assert.ok(html.includes('⚡ Review & Sync Harnesses'), 'Must label button as Review & Sync Harnesses');
  });

  test('renders all 18 KOS methodologies in evolution menu with chip counters and select dropdown', () => {
    const findings = [
      { file_path: 'docs/a.md', description: 'Taxonomy cycle detected', category: 'taxonomy' },
      { file_path: 'docs/b.md', description: 'Facet dimension missing', category: 'facet' },
      { file_path: 'docs/c.md', description: 'Thesaurus reciprocity broken', category: 'thesaurus' },
      { file_path: 'docs/d.md', description: 'Metadata missing frontmatter', category: 'metadata' },
    ];
    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: findings,
    });

    // 18 KOS Menu Card must be rendered
    assert.ok(html.includes('evolution-kos-menu-card'), 'Must render 18 KOS menu card');
    assert.ok(html.includes('Knowledge Organization Systems'), 'Must render 18 KOS title');
    assert.ok(html.includes('select-kos-filter'), 'Must render KOS filter dropdown');
    assert.ok(html.includes('kos-chips-container'), 'Must render KOS chips container');

    // Must include methodologies
    assert.ok(html.includes('Taxonomy'), 'Must include Taxonomy in menu');
    assert.ok(html.includes('Faceted Classification'), 'Must include Faceted Classification in menu');
    assert.ok(html.includes('Thesaurus'), 'Must include Thesaurus in menu');
    assert.ok(html.includes('Semantic Networks'), 'Must include Semantic Networks in menu');
    assert.ok(html.includes('Metadata'), 'Must include Metadata in menu');
    assert.ok(html.includes('Context Routing'), 'Must include Context Routing in menu');
    assert.ok(html.includes('Information Retrieval'), 'Must include Information Retrieval in menu');
    assert.ok(html.includes('Network Analysis'), 'Must include Network Analysis in menu');
  });

  test('paginates findings with selectable page sizes (10, 25, 50) and navigation controls', () => {
    // Generate 65 mock findings to test multi-page pagination
    const mockFindings = Array.from({ length: 65 }, (_, i) => ({
      file_path: `docs/item_${i + 1}.md`,
      description: `Structural issue in document ${i + 1}`,
      category: i % 2 === 0 ? 'taxonomy' : 'metadata',
    }));

    // Page 1 with pageSize 25 (default)
    const htmlPage1 = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: mockFindings,
      findingsPage: 1,
      findingsPageSize: 25,
    });

    assert.ok(htmlPage1.includes('Showing <strong>1–25</strong> of <strong>65</strong> findings'));
    assert.ok(htmlPage1.includes('Page <strong>1</strong> of <strong>3</strong>'));
    assert.ok(htmlPage1.includes('docs/item_1.md'));
    assert.ok(htmlPage1.includes('docs/item_25.md'));
    assert.ok(!htmlPage1.includes('docs/item_26.md'), 'Page 1 must not include item 26');

    // Custom popover page size dropdown options 10, 25, 50
    assert.ok(htmlPage1.includes('btn-page-size-toggle'), 'Must render custom page size toggle button');
    assert.ok(htmlPage1.includes('page-size-menu'), 'Must render custom page size menu');
    assert.ok(htmlPage1.includes('data-value="10"'));
    assert.ok(htmlPage1.includes('data-value="25"'));
    assert.ok(htmlPage1.includes('data-value="50"'));

    // Navigation buttons
    assert.ok(htmlPage1.includes('data-page="1" disabled'), 'First and Prev must be disabled on page 1');
    assert.ok(htmlPage1.includes('data-page="2"'), 'Next button points to page 2');
    assert.ok(htmlPage1.includes('data-page="3"'), 'Last button points to page 3');

    // Page 2 with pageSize 10
    const htmlPage2Size10 = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: mockFindings,
      findingsPage: 2,
      findingsPageSize: 10,
    });

    assert.ok(htmlPage2Size10.includes('Showing <strong>11–20</strong> of <strong>65</strong> findings'));
    assert.ok(htmlPage2Size10.includes('Page <strong>2</strong> of <strong>7</strong>'));
    assert.ok(htmlPage2Size10.includes('docs/item_11.md'));
    assert.ok(htmlPage2Size10.includes('docs/item_20.md'));
    assert.ok(!htmlPage2Size10.includes('docs/item_10.md'));
    assert.ok(!htmlPage2Size10.includes('docs/item_21.md'));

    // Page 2 with pageSize 50 (last page with 15 items)
    const htmlPage2Size50 = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: mockFindings,
      findingsPage: 2,
      findingsPageSize: 50,
    });

    assert.ok(htmlPage2Size50.includes('Showing <strong>51–65</strong> of <strong>65</strong> findings'));
    assert.ok(htmlPage2Size50.includes('Page <strong>2</strong> of <strong>2</strong>'));
    assert.ok(htmlPage2Size50.includes('docs/item_51.md'));
    assert.ok(htmlPage2Size50.includes('docs/item_65.md'));
  });

  test('filters findings by specific KOS methodology and updates counts', () => {
    const mockFindings = [
      { file_path: 'docs/tax_1.md', description: 'Taxonomy hierarchy error', category: 'taxonomy' },
      { file_path: 'docs/tax_2.md', description: 'Taxonomy cycle', category: 'taxonomy' },
      { file_path: 'docs/facet_1.md', description: 'Facet dimension tagging error', category: 'facet' },
      { file_path: 'docs/thes_1.md', description: 'Thesaurus BT/NT conflict', category: 'thesaurus' },
    ];

    const htmlFiltered = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: mockFindings,
      selectedKosFilter: 'taxonomy',
      findingsPage: 1,
      findingsPageSize: 25,
    });

    assert.ok(htmlFiltered.includes('Showing <strong>1–2</strong> of <strong>2</strong> findings'));
    assert.ok(htmlFiltered.includes('docs/tax_1.md'));
    assert.ok(htmlFiltered.includes('docs/tax_2.md'));
    assert.ok(!htmlFiltered.includes('docs/facet_1.md'), 'Must filter out non-taxonomy finding');
    assert.ok(!htmlFiltered.includes('docs/thes_1.md'), 'Must filter out non-taxonomy finding');
    assert.ok(htmlFiltered.includes('Discipline: Taxonomy'));
  });

  test('renders KOS methodology badges on finding cards', () => {
    const mockFindings = [
      { file_path: 'docs/tax.md', description: 'Taxonomy issue', category: 'taxonomy' },
      { file_path: 'docs/sec.md', description: 'Secondary routing layer to remove', category: 'secondaryroutinglayer' },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      structureFindings: mockFindings,
    });

    assert.ok(html.includes('badge-kos-taxonomy'), 'Must render badge-kos-taxonomy class');
    assert.ok(html.includes('badge-kos-information_architecture'), 'Must render badge-kos-information_architecture class');
  });

  test('categorizes multi-pillar vault findings across 18 KOS disciplines and renders pillar badges', () => {
    const multiPillarFindings = [
      { file_path: 'notes/broken.md', description: 'Broken reference to missing target', category: 'broken_link', pillar: 'integrity' },
      { file_path: 'notes/drift.md', description: 'Cites modified/deleted code', category: 'technical_drift', pillar: 'freshness' },
      { file_path: 'notes/collision.md', description: 'Semantic collision with note', category: 'collision_potentialduplicate', pillar: 'consistency' },
      { file_path: 'notes/orphan.md', description: 'Isolated orphan note', category: 'orphan_node', pillar: 'discoverability' },
      { file_path: 'notes/struct.md', description: 'Missing fragment metadata', category: 'missingfragmentmetadata', pillar: 'structure' },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: multiPillarFindings,
      findingsPage: 1,
      findingsPageSize: 25,
    });

    // Must render Vault Knowledge & KOS Findings card header
    assert.ok(html.includes('Vault Knowledge & KOS Findings (5)'), 'Must render multi-pillar header title');
    assert.ok(html.includes('5 Active Issues'), 'Must render active issues count badge');

    // Must render 18 KOS badges
    assert.ok(html.includes('badge-kos-thesaurus'), 'Must map broken_link to Thesaurus');
    assert.ok(html.includes('badge-kos-knowledge_drift'), 'Must map technical_drift to Knowledge Drift');
    assert.ok(html.includes('badge-kos-terminology_management'), 'Must map collision to Terminology Management');
    assert.ok(html.includes('badge-kos-knowledge_graph_network_analysis'), 'Must map orphan to Network Analysis');
    assert.ok(html.includes('badge-kos-metadata_application_profiles'), 'Must map metadata to DCAP');

    // Must render pillar badges
    assert.ok(html.includes('badge-pillar-integrity'), 'Must render integrity pillar badge');
    assert.ok(html.includes('badge-pillar-freshness'), 'Must render freshness pillar badge');
    assert.ok(html.includes('badge-pillar-consistency'), 'Must render consistency pillar badge');
    assert.ok(html.includes('badge-pillar-discoverability'), 'Must render discoverability pillar badge');
    assert.ok(html.includes('badge-pillar-structure'), 'Must render structure pillar badge');
  });

  test('paginates large 3,000+ finding sets without rendering excess DOM nodes', () => {
    const largeFindings = Array.from({ length: 3500 }, (_, i) => ({
      file_path: `notes/file_${i}.md`,
      description: `Issue description ${i}`,
      category: i % 2 === 0 ? 'broken_link' : 'technical_drift',
      pillar: i % 2 === 0 ? 'integrity' : 'freshness',
    }));

    // Page 1 with pageSize 10
    const html10 = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: largeFindings,
      findingsPage: 1,
      findingsPageSize: 10,
    });

    assert.ok(html10.includes('Showing <strong>1–10</strong> of <strong>3500</strong> findings'));
    assert.ok(html10.includes('Page <strong>1</strong> of <strong>350</strong>'));
    assert.ok(html10.includes('notes/file_0.md'));
    assert.ok(html10.includes('notes/file_9.md'));
    assert.ok(!html10.includes('notes/file_10.md'), 'Must not render item beyond page size 10');

    // Page 1 with pageSize 50
    const html50 = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: largeFindings,
      findingsPage: 1,
      findingsPageSize: 50,
    });

    assert.ok(html50.includes('Showing <strong>1–50</strong> of <strong>3500</strong> findings'));
    assert.ok(html50.includes('Page <strong>1</strong> of <strong>70</strong>'));
    assert.ok(html50.includes('notes/file_49.md'));
    assert.ok(!html50.includes('notes/file_50.md'), 'Must not render item beyond page size 50');
  });

  test('renders custom DOM popover dropdowns for 18 KOS and page size ensuring Linux Webview compatibility', () => {
    const findings = [
      { file_path: 'notes/a.md', description: 'Broken link', category: 'broken_link', pillar: 'integrity' },
    ];
    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: findings,
      selectedKosFilter: 'thesaurus',
      findingsPage: 1,
      findingsPageSize: 25,
    });

    // KOS discipline filter label and tags (combo box removed per user feedback)
    assert.ok(!html.includes('id="kos-discipline-dropdown"'), 'Must NOT render combo box dropdown');
    assert.ok(html.includes('Filter by KOS Discipline:'), 'Must render Filter by KOS Discipline label');
    assert.ok(html.includes('findings-kos-filter-bar'), 'Must render findings-kos-filter-bar');
    assert.ok(html.includes('data-kos-id="thesaurus"'), 'Must render thesaurus KOS tag');
    assert.ok(html.includes('kos-filter-chip active'), 'Must mark selected KOS tag as active');
    assert.ok(html.includes('id="select-kos-filter"'), 'Must maintain select-kos-filter hidden input');

    // Custom page size dropdown
    assert.ok(html.includes('class="settings-scope-dropdown page-size-dropdown"'), 'Must render page-size-dropdown');
    assert.ok(html.includes('class="settings-scope-btn page-size-btn btn-page-size-toggle"'), 'Must render page size button');
    assert.ok(html.includes('class="settings-scope-menu page-size-menu"'), 'Must render page size menu popover');
    assert.ok(html.includes('class="findings-page-size-select"'), 'Must maintain findings-page-size-select hidden input');
  });

  test('paginates large 850+ structural opportunities in unified findings list without rendering excess DOM nodes', () => {
    const largeSignatures = Array.from({ length: 853 }, (_, i) => ({
      signatureId: `sig_silo_${i}`,
      documentPath: `notes/orphan_${i}.md`,
      problemType: 'isolated_silo',
      confidence: 0.85,
      evidence: {
        cohesion: 0.1,
        coupling: 0.0,
        referenceConcentration: 0.0,
        granularity: 0.2,
        contextEfficiency: 0.3,
        discoverability: 0.1,
        reusability: 0.1,
        semanticDiversity: 0.2,
        navigationalComplexity: 0.1,
        maintenanceCoupling: 0.0,
      },
      rationale: `Isolated silo note ${i} without inbound or outbound links`,
    }));

    // Page 1 with default pageSize 10
    const html10 = renderEvolutionTabHtml({
      signatures: largeSignatures as any,
      recommendations: [],
      vaultFindings: [],
      findingsPage: 1,
      findingsPageSize: 10,
    });

    assert.ok(html10.includes('Showing <strong>1–10</strong> of <strong>853</strong> findings'));
    assert.ok(html10.includes('Page <strong>1</strong> of <strong>86</strong>'));
    assert.ok(html10.includes('notes/orphan_0.md'));
    assert.ok(html10.includes('notes/orphan_9.md'));
    assert.ok(!html10.includes('notes/orphan_10.md'), 'Must not render signature card beyond page size 10');
    assert.strictEqual((html10.match(/findings-pagination-bar/g) || []).length, 1, 'Must render exactly one pagination bar');
    assert.ok(!html10.includes('pagination-top'), 'Must not render duplicate top pagination bar');

    // Page 2 with pageSize 10
    const htmlPage2 = renderEvolutionTabHtml({
      signatures: largeSignatures as any,
      recommendations: [],
      vaultFindings: [],
      findingsPage: 2,
      findingsPageSize: 10,
    });

    assert.ok(htmlPage2.includes('Showing <strong>11–20</strong> of <strong>853</strong> findings'));
    assert.ok(htmlPage2.includes('Page <strong>2</strong> of <strong>86</strong>'));
    assert.ok(htmlPage2.includes('notes/orphan_10.md'));
    assert.ok(htmlPage2.includes('notes/orphan_19.md'));
    assert.ok(!htmlPage2.includes('notes/orphan_0.md'), 'Page 2 must not render page 1 items');

    // Page 1 with pageSize 50
    const html50 = renderEvolutionTabHtml({
      signatures: largeSignatures as any,
      recommendations: [],
      vaultFindings: [],
      findingsPage: 1,
      findingsPageSize: 50,
    });

    assert.ok(html50.includes('Showing <strong>1–50</strong> of <strong>853</strong> findings'));
    assert.ok(html50.includes('Page <strong>1</strong> of <strong>18</strong>'));
    assert.ok(html50.includes('notes/orphan_49.md'));
    assert.ok(!html50.includes('notes/orphan_50.md'), 'Must not render item beyond page size 50');
  });

  test('filters structural opportunities by problem type chip in unified findings list', () => {
    const mixedSignatures = [
      ...Array.from({ length: 853 }, (_, i) => ({
        signatureId: `sig_silo_${i}`,
        documentPath: `notes/silo_${i}.md`,
        problemType: 'isolated_silo',
        confidence: 0.85,
        evidence: {},
        rationale: 'Isolated silo',
      })),
      {
        signatureId: 'sig_god_1',
        documentPath: 'notes/god_doc.md',
        problemType: 'god_document',
        confidence: 0.95,
        evidence: {},
        rationale: 'Overloaded document',
      },
    ];

    // Filter by god_document
    const htmlGod = renderEvolutionTabHtml({
      signatures: mixedSignatures as any,
      recommendations: [],
      selectedSignatureTypeFilter: 'god_document',
      findingsPage: 1,
      findingsPageSize: 10,
    });

    assert.ok(htmlGod.includes('Showing <strong>1–1</strong> of <strong>1</strong> finding'));
    assert.ok(htmlGod.includes('notes/god_doc.md'));
    assert.ok(!htmlGod.includes('notes/silo_0.md'), 'Filtered view must not include isolated silos');
    assert.ok(htmlGod.includes('Type: God Document'), 'Filter pill must use concise God Document label');
    assert.ok(htmlGod.includes('badge-god-doc'));
    assert.ok(htmlGod.includes('>God Document<'), 'Badge must render concise God Document text');
  });

  test('filters findings by quality pillar chip and displays consistent Title Case badges', () => {
    const mixedFindings = [
      { file_path: 'notes/broken_1.md', description: 'Broken link', category: 'broken_link', pillar: 'integrity' },
      { file_path: 'notes/broken_2.md', description: 'Broken link', category: 'broken_link', pillar: 'integrity' },
      { file_path: 'notes/orphan_1.md', description: 'Orphan note', category: 'isolated_orphan', pillar: 'discoverability' },
    ];

    // Filter by integrity
    const htmlIntegrity = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: mixedFindings,
      selectedPillarFilter: 'integrity',
      findingsPage: 1,
      findingsPageSize: 10,
    });

    assert.ok(htmlIntegrity.includes('Showing <strong>1–2</strong> of <strong>2</strong> findings'));
    assert.ok(htmlIntegrity.includes('notes/broken_1.md'));
    assert.ok(htmlIntegrity.includes('notes/broken_2.md'));
    assert.ok(!htmlIntegrity.includes('notes/orphan_1.md'), 'Must exclude discoverability findings');
    assert.ok(htmlIntegrity.includes('data-pillar-id="integrity"'));
    assert.ok(htmlIntegrity.includes('findings-pillar-chip active has-findings'));
    assert.ok(htmlIntegrity.includes('Pillar: Integrity'), 'Must display active filter pill with Title Case');
    assert.ok(htmlIntegrity.includes('badge-pillar-integrity'));
    assert.ok(htmlIntegrity.includes('>Integrity<'), 'Pillar badge must be Title Case (not uppercase yelling)');
    assert.ok(!htmlIntegrity.includes('>INTEGRITY<'), 'Must not render all-caps INTEGRITY');

    // Filter by discoverability
    const htmlDiscovery = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: mixedFindings,
      selectedPillarFilter: 'discoverability',
      findingsPage: 1,
      findingsPageSize: 10,
    });

    assert.ok(htmlDiscovery.includes('Showing <strong>1–1</strong> of <strong>1</strong> finding'));
    assert.ok(htmlDiscovery.includes('notes/orphan_1.md'));
    assert.ok(!htmlDiscovery.includes('notes/broken_1.md'), 'Must exclude integrity findings');
    assert.ok(htmlDiscovery.includes('data-pillar-id="discoverability"'));
    assert.ok(htmlDiscovery.includes('findings-pillar-chip active has-findings'));
    assert.ok(htmlDiscovery.includes('Pillar: Discoverability'));
    assert.ok(htmlDiscovery.includes('badge-pillar-discoverability'));
    assert.ok(htmlDiscovery.includes('>Discoverability<'));
    assert.ok(!htmlDiscovery.includes('>DISCOVERABILITY<'));
  });

  test('harmonizes finding card tags with filter chips: Title Case pillar, KOS shortLabel without parentheses, and concise problem badges', () => {
    const findings = [
      {
        file_path: 'notes/metadata_test.md',
        description: 'Missing fragment metadata frontmatter in oversized note',
        category: 'missing_fragment_metadata',
        pillar: 'structure',
      },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: findings,
    });

    // 1. Pillar badge must match filter chip ('Structure', not 'STRUCTURE' or 'STRCUTURE')
    assert.ok(html.includes('badge-pillar-structure'));
    assert.ok(html.includes('>Structure<'), 'Badge must render Title Case "Structure" matching filter chip');
    assert.ok(!html.includes('>STRUCTURE<'), 'Must not render ALL-CAPS "STRUCTURE"');

    // 2. KOS badge must match filter chip ('Metadata', not 'Metadata Profiles (DCAP)')
    assert.ok(html.includes('badge-kos-metadata_application_profiles'));
    assert.ok(html.includes('>Metadata<'), 'Badge must render clean "Metadata" in Capitalized Wording');
    assert.ok(!html.includes('(DCAP)'), 'Badge and UI must never contain verbose parentheses "(DCAP)"');

    // 3. Problem badges must match opportunity filter names without parentheses
    const sigHtml = renderEvolutionTabHtml({
      signatures: [
        {
          signatureId: 'sig_router',
          documentPath: 'notes/router.md',
          problemType: 'implicit_router',
          confidence: 0.88,
          rationale: 'Implicit router detected',
        },
      ] as any,
      recommendations: [],
    });
    assert.ok(sigHtml.includes('badge-router'));
    assert.ok(sigHtml.includes('>Implicit Router<'), 'Badge must render concise "Implicit Router" matching filter');
    assert.ok(!sigHtml.includes('Weak Discovery Path (Implicit Router)'), 'Must not include verbose parentheses description');

    // 4. Confidence badge must use Capitalized Wording
    assert.ok(sigHtml.includes('badge-confidence'));
    assert.ok(sigHtml.includes('88% Confidence'), 'Confidence badge must render Capitalized Wording "88% Confidence"');

    // 5. Pillar filter chips must NOT have inline colorful fonts
    assert.ok(!sigHtml.includes('style="color: #38bdf8;"'), 'Pillar chip names must not have inline colorful font');
    assert.ok(!sigHtml.includes('style="color: #ef4444;"'), 'Pillar chip names must not have inline colorful font');
  });

  test('strictly renders 1 primary button + 1 Open File button on finding card without duplicate strategy buttons', () => {
    const mockSig = {
      signatureId: 'opp_god_001',
      documentPath: 'notes/architecture/god_doc.md',
      problemType: 'god_document',
      confidence: 0.95,
      rationale: 'Overloaded document requires splitting.',
      strategySet: {
        primary: {
          strategy: 'section_addressing',
          fitScore: 85,
          transformationCost: 10,
          expectedBenefits: ['Context reduction'],
          countervailingTradeOffs: ['Index lookup'],
        },
        supporting: [
          {
            strategy: 'lightweight_index',
            fitScore: 75,
            transformationCost: 5,
            expectedBenefits: ['Wayfinding'],
            countervailingTradeOffs: [],
          },
        ],
        alternatives: [],
        compositePlanValue: {
          baseFitSum: 160,
          synergyBonus: 15,
          overlapPenalty: 5,
          transformationCost: 15,
          netValue: 155,
        },
      },
    };

    const html = renderEvolutionTabHtml({
      signatures: [mockSig as any],
      recommendations: [],
      vaultFindings: [],
    });

    // Strategy options and CPV card rendered
    assert.ok(html.includes('strategy-set-container'), 'Must render strategy container');
    assert.ok(html.includes('Composite Plan Value & Trade-Off Valuation'), 'Must render CPV valuation');

    // Strategy container itself must NOT have a duplicate generate-multi-plan button inside
    assert.ok(!html.includes('strategy-set-actions-bar'), 'Must NOT have duplicate actions bar inside strategy set container');

    // Finding actions row must have strictly 1 primary action button and 1 Open File button
    const multiPlanBtns = (html.match(/btn-generate-multi-plan/g) || []).length;
    assert.strictEqual(multiPlanBtns, 1, 'Must render exactly 1 btn-generate-multi-plan button on the entire card');

    const openFileBtns = (html.match(/btn-finding-open-file/g) || []).length;
    assert.strictEqual(openFileBtns, 1, 'Must render exactly 1 Open File button on the card');

    // Zero parentheses in button text
    assert.ok(html.includes('⚡ Generate Transformation Plan'), 'Must have clean label without parentheses');
    assert.ok(!html.includes('(Preview Change)'), 'Must not have (Preview Change)');
    assert.ok(!html.includes('(Generate Transformation Plan)'), 'Must not have (Generate Transformation Plan)');
    assert.ok(!html.includes('(Open Note)'), 'Must not have (Open Note)');
    assert.ok(!html.includes('(Safely Remove)'), 'Must not have (Safely Remove)');
    assert.ok(!html.includes('(Fix Reference)'), 'Must not have (Fix Reference)');
  });

  test('renders Review with AI and Open File buttons when finding has no automated plan action, avoiding fake duplicate button', () => {
    const findingsWithoutAction = [
      {
        file_path: 'notes/simple_doc.md',
        description: 'Informational note with minor format notice',
        category: 'formatting',
        pillar: 'structure',
      },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: findingsWithoutAction,
    });

    assert.ok(!html.includes('btn-generate-plan'), 'Must not render plan button');
    assert.ok(!html.includes('btn-generate-multi-plan'), 'Must not render multi-plan button');
    assert.ok(!html.includes('Read & Apply'), 'Must not render fake Read & Apply button');
    assert.ok(html.includes('btn-finding-review-ai'), 'Must always render Review with AI button');
    assert.ok(html.includes('Review with AI'), 'Must display Review with AI label');
    assert.ok(html.includes('btn-finding-open-file'), 'Must render Open File button');
  });

  test('renders finding tags on line 1 and file location on a new line (separate lines) inside finding header', () => {
    const findings = [
      {
        file_path: 'specs/009-deterministic-knowledge-engine/RESEARCH_2.md',
        line_number: 42,
        description: 'Document exceeds structural threshold',
        category: 'god_document',
        pillar: 'structure',
      },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: findings,
    });

    // Finding header row must be present
    assert.ok(html.includes('finding-header-row'), 'Must contain finding-header-row');

    // Tags row must appear BEFORE finding-loc in the DOM so file name is rendered on a new line
    const tagsIdx = html.indexOf('finding-tags-row');
    const locIdx = html.indexOf('finding-loc');

    assert.ok(tagsIdx !== -1, 'Must contain finding-tags-row');
    assert.ok(locIdx !== -1, 'Must contain finding-loc');
    assert.ok(tagsIdx < locIdx, 'finding-tags-row must precede finding-loc so file name is on a new line below tags');

    // Finding-loc must contain the file path with line number
    assert.ok(html.includes('specs/009-deterministic-knowledge-engine/RESEARCH_2.md:42'), 'Must contain file path with line number');
  });

  test('renders Review with AI and Open Target buttons for technical drift findings with target reference', () => {
    const findings = [
      {
        file_path: 'notes/00_meta/wiki_index.md',
        line_number: 275,
        description: 'Missing code citation `hitch_consult.py`',
        category: 'technical_drift',
        target: 'hitch_consult.py',
        pillar: 'freshness',
      },
    ];

    const html = renderEvolutionTabHtml({
      signatures: [],
      recommendations: [],
      vaultFindings: findings,
    });

    assert.ok(html.includes('btn-finding-review-ai'), 'Must render Review with AI button');
    assert.ok(html.includes('data-action="review-with-ai"'), 'Must have review-with-ai action hook');
    assert.ok(html.includes('btn-finding-open-target'), 'Must render Open Target button');
    assert.ok(html.includes('data-action="open-target"'), 'Must have open-target action hook');
    assert.ok(html.includes('Open Target (hitch_consult.py) ↗'), 'Must display target basename on button');
    assert.ok(html.includes('btn-finding-open-file'), 'Must render Open File button');
  });
});


