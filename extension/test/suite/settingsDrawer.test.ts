import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import { renderSettingsDrawerHtml, type SettingsTab } from '../../webview-ui/src/components/settingsDrawer.ts';

suite('SettingsDrawer Test Suite', () => {
  const sampleProps = {
    activeTab: 'knowledge' as SettingsTab,
    primaryHub: '00_meta/wiki_index.md',
    brokenLinksCount: 3,
    scanExclusions: ['**/.git/**', '**/node_modules/**'],
    graphExclusions: ['packages/**/README.md', 'vendor/**'],
    colorGroups: [
      { id: '1', name: 'Meta', query: 'path:00_meta/**', color: '#3B82F6', enabled: true },
    ],
    gitHooksInstalled: true,
    gitCommitLatencyMs: 18,
  };

  test('renders two-tiered Insights vs Settings navigation header (T022-T024, T030)', () => {
    const html = renderSettingsDrawerHtml(sampleProps);

    // Tiers
    assert.ok(html.includes('Insights'), 'Must have Insights group label');
    assert.ok(html.includes('Settings'), 'Must have Settings group label');

    // INSIGHTS sub-tabs
    assert.ok(html.includes('Health'), 'Must have Health tab');
    assert.ok(html.includes('Structure'), 'Must have Structure tab');
    assert.ok(html.includes('Evolution'), 'Must have Evolution tab');
    assert.ok(html.includes('Load'), 'Must have Load tab');
    assert.ok(html.includes('data-tab="load"'), 'Must have data-tab="load" attribute');

    // SETTINGS sub-tabs
    assert.ok(html.includes('Knowledge'), 'Must have Knowledge tab');
    assert.ok(html.includes('Exclusions'), 'Must have Exclusions tab');
    assert.ok(html.includes('Clusters'), 'Must have Clusters tab');
    assert.ok(html.includes('Git'), 'Must have Git tab');
    assert.ok(html.includes('Harness'), 'Must have Harness tab');
    assert.ok(!html.includes('data-tab="advanced"'), 'Advanced tab must not be present');
  });

  test('renders active tab content correctly for excluded paths', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'exclusions',
    });

    assert.ok(html.includes('packages/**/README.md'), 'Must render graph exclusion rule');
    assert.ok(html.includes('vendor/**'), 'Must render vendor rule');
    assert.ok(html.includes('+ Add Rule'), 'Must render add rule button');
    assert.ok(html.includes('id="select-rule-scope"'), 'Must render select-rule-scope input');
    assert.ok(html.includes('id="btn-scope-toggle"'), 'Must render scope toggle button');
    assert.ok(html.includes('id="menu-scope-options"'), 'Must render scope options menu');
    assert.ok(html.includes('class="scope-item-title">Graph</span>'), 'Must render short Graph option');
    assert.ok(html.includes('class="scope-item-title">Scan</span>'), 'Must render short Scan option');
    assert.ok(!html.includes('Visual Hiding'), 'Must not contain prolix description in options');
    assert.ok(!html.includes('Completely Ignored)'), 'Must not contain prolix description in options');
    assert.ok(html.includes('class="rule-del-btn"'), 'Must render delete rule buttons');
    assert.ok(html.includes('data-scope="graph"'), 'Must have graph scope on delete buttons');
    assert.ok(html.includes('data-scope="scan"'), 'Must have scan scope on delete buttons');
  });

  test('renders Git Sync controls with hook status', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git_sync',
    });

    assert.ok(html.includes('Git Hook Integration'), 'Must display Git hook header');
    assert.ok(html.includes('threshold-toggles-list'), 'Must render threshold options as a dedicated vertical list');
    assert.strictEqual((html.match(/class="threshold-toggle-item"/g) || []).length, 3, 'Must render one row per threshold checkbox');
    assert.ok(html.includes('Installed') || html.includes('Active'), 'Must show hook status');
    assert.ok(html.includes('Reinstall') || html.includes('Install'), 'Must have install/reinstall button');
    assert.ok(html.includes('Knowledge Debt Baseline'), 'Must display Knowledge Debt Baseline');
  });

  test('renders Cluster Groups tab with Add Rule form, configured groups, and vault suggestions', () => {
    const sampleNodes = [
      { id: '00_meta/wiki_index.md', label: 'Wiki Index', filePath: '00_meta/wiki_index.md', tags: ['#meta'], color: '#3B82F6', status: 'active' as const, tokens: 1000, hopDistance: 0, inDegree: 5, outDegree: 5, betweenness: 0.5, brokenLinks: 0 },
      { id: '01_projects/proj.md', label: 'Project', filePath: '01_projects/proj.md', tags: ['#projects'], color: '#3B82F6', status: 'active' as const, tokens: 800, hopDistance: 1, inDegree: 2, outDegree: 2, betweenness: 0.2, brokenLinks: 0 },
      { id: '02_memory/context.md', label: 'Context', filePath: '02_memory/context.md', tags: ['#memory'], color: '#3B82F6', status: 'active' as const, tokens: 600, hopDistance: 1, inDegree: 3, outDegree: 1, betweenness: 0.3, brokenLinks: 0 },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'clusters',
      nodes: sampleNodes,
    });

    // 1. Add Group form controls
    assert.ok(html.includes('Add Color Group'), 'Must display Add Color Group subheading');
    assert.ok(html.includes('input-cluster-query'), 'Must render query input');
    assert.ok(html.includes('input-cluster-label'), 'Must render label input');
    assert.ok(html.includes('input-cluster-color'), 'Must render color picker input');
    assert.ok(html.includes('+ Add'), 'Must render + Add button');

    // 2. Configured groups
    assert.ok(html.includes('Configured Groups'), 'Must display Configured Groups subheading');
    assert.ok(html.includes('Meta'), 'Must render configured group label');
    assert.ok(html.includes('path:00_meta/**'), 'Must render configured group query');
    assert.ok(html.includes('chk-cluster-enabled'), 'Must render enable/disable toggle');
    assert.ok(html.includes('btn-delete-cluster'), 'Must render delete button');

    // 3. Vault suggestions
    assert.ok(html.includes('Suggestions from Vault'), 'Must display Suggestions from Vault header');
    assert.ok(html.includes('suggestion-chip'), 'Must render suggestion chips');
    assert.ok(html.includes('path:01_projects/**') || html.includes('01_projects'), 'Must suggest 01_projects folder');
    assert.ok(html.includes('path:02_memory/**') || html.includes('02_memory'), 'Must suggest 02_memory folder');
    assert.ok(html.includes('#projects') || html.includes('#memory'), 'Must suggest discovered tags');
    assert.ok(html.includes('Auto-Populate'), 'Must offer Auto-Populate action');
  });

  test('renders empty configured groups message when colorGroups is empty but provides suggestions', () => {
    const sampleNodes = [
      { id: '01_projects/proj.md', label: 'Project', filePath: '01_projects/proj.md', tags: ['#projects'], color: '#3B82F6', status: 'active' as const, tokens: 800, hopDistance: 1, inDegree: 2, outDegree: 2, betweenness: 0.2, brokenLinks: 0 },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'clusters',
      colorGroups: [],
      nodes: sampleNodes,
    });

    assert.ok(
      html.includes('No custom cluster color groups defined'),
      'Must display helpful empty state message',
    );
    assert.ok(html.includes('input-cluster-query'), 'Must still offer add form when empty');
    assert.ok(html.includes('suggestion-chip'), 'Must still offer suggestions when empty');
  });

  test('renders Diagnostics tab with itemized issues, pillar accordions, and action buttons', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      issuesReport: {
        overall_score: 82.5,
        total_issues: 2,
        integrity_count: 1,
        consistency_count: 0,
        discoverability_count: 1,
        freshness_count: 0,
        issues: [
          {
            id: 'broken:00_meta/wiki_index.md:86',
            pillar: 'integrity',
            category: 'broken_link',
            file_path: '00_meta/wiki_index.md',
            line_number: 86,
            target: 'missing_target.md',
            deduction_points: 6.5,
            weight: 1.3,
            description: "Broken link referencing missing target 'missing_target.md'",
            remediation_hint: "Create target note or update link",
          },
          {
            id: 'orphan:notes/lonely.md',
            pillar: 'discoverability',
            category: 'orphan_node',
            file_path: 'notes/lonely.md',
            line_number: null,
            target: null,
            deduction_points: 8.0,
            weight: 1.0,
            description: 'Isolated orphan node without incoming or outgoing links',
            remediation_hint: 'Add wiki links to connect this note',
          },
        ],
      },
    });

    // 1. Overview card & scores
    assert.ok(html.includes('Quality &amp; Defect Overview') || html.includes('Quality & Defect Overview'), 'Must display Quality & Defect Overview');
    assert.ok(html.includes('83/100') || html.includes('82.5'), 'Must render score');
    assert.ok(html.includes('diag-pillar-filter-row'), 'Must render the compact pillar filter row');
    assert.strictEqual((html.match(/class="diag-pillar-chip /g) || []).length, 5, 'Must render five clickable pillar filters');
    assert.ok(html.includes('data-filter="integrity"'), 'Must render Integrity filter tile');
    assert.ok(html.includes('data-filter="discoverability"'), 'Must render Discovery filter tile');
    assert.ok(html.includes('id="diag-clear-filters"'), 'Must render Clear filter button');
    assert.ok(html.includes('data-pillar="integrity"'), 'Must identify Integrity issue group for live filtering');
    assert.ok(html.includes('id="diag-toggle-all"'), 'Must render expand/collapse all button');
    assert.ok(html.includes('diag-accordion-header collapsed'), 'Issue groups must start collapsed');
    assert.ok(html.includes('diag-accordion-body collapsed'), 'Issue bodies must start collapsed');

    // 2. Accordions & issues
    assert.ok(html.includes('Integrity'), 'Must render Integrity accordion');
    assert.ok(html.includes('Discoverability'), 'Must render Discoverability accordion');
    assert.ok(html.includes('00_meta/wiki_index.md:86'), 'Must display exact file location with line');
    assert.ok(html.includes('missing_target.md'), 'Must display target reference');
    assert.ok(html.includes('Open Note'), 'Must offer Open Note action button');
    assert.ok(html.includes('Fix Reference'), 'Must offer Fix Reference action button for broken links');
  });

  test('renders Safely Remove action option for Secondary Routing Layer issues in Diagnostics and Git/Harness tabs', () => {
    const issuesReport = {
      overall_score: 75.0,
      total_issues: 1,
      integrity_count: 0,
      structure_count: 1,
      consistency_count: 0,
      discoverability_count: 0,
      freshness_count: 0,
      issues: [
        {
          id: 'structure:index.md:secondaryroutinglayer',
          pillar: 'structure',
          category: 'secondaryroutinglayer',
          file_path: 'index.md',
          deduction_points: 15,
          weight: 1.0,
          description: "Secondary routing layer detected at 'index.md' with 12 outgoing routes. Consolidate routes directly into primary AI Harness Entry Point.",
          remediation_hint: 'Consolidate routes directly into primary AI Harness Entry Point',
        },
      ],
    };

    const diagHtml = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      issuesReport,
    });
    assert.ok(diagHtml.includes('btn-gov-eliminate-secondary'), 'Must render eliminate secondary button in Diagnostics');
    assert.ok(diagHtml.includes('Safely Remove index.md'), 'Must display Safely Remove index.md label in Diagnostics');

    const harnessHtml = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      harnessStatus: {
        detected: true,
        primary_harness: 'AGENTS.md',
        harness_files: ['AGENTS.md'],
        active_harness: {
          file_path: 'AGENTS.md',
          harness_type: 'agents_md',
          is_primary: true,
          total_tokens: 3200,
          heading_count: 14,
          secondary_layers_detected: [
            {
              file_path: 'index.md',
              outgoing_routes_count: 12,
              intermediate_hops: 1,
              penalty_points: 15,
              description: 'Intermediate index routing hop',
              recommendation_hint: 'Consolidate routes directly into AGENTS.md',
            },
          ],
        } as any,
      },
    });
    assert.ok(harnessHtml.includes('btn-gov-eliminate-secondary'), 'Must render eliminate secondary button in Harness alert card');
    assert.ok(harnessHtml.includes('Safely Remove index.md'), 'Must display Safely Remove index.md label in Harness alert card');
  });

  test('strictly displays 0 on Discovery chip when active discoverability issues is 0, even when orphanNotes > 0 and baselined debt exists', () => {
    const issuesReport = {
      overall_score: 100,
      total_issues: 178,
      active_issues_count: 0,
      legacy_debt_count: 178,
      integrity_count: 0,
      structure_count: 0,
      consistency_count: 0,
      discoverability_count: 0,
      freshness_count: 0,
      issues: [],
    };

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      selectedDebtScope: 'active',
      orphanNotes: 2,
      brokenLinksCount: 0,
      issuesReport,
    });

    assert.ok(
      html.includes('>0</span>\n                <span class="diag-chip-lbl">Discovery</span>'),
      'Discovery chip must strictly display 0 in active scope when there are 0 active regressions'
    );
    assert.ok(
      !html.includes('>2</span>\n                <span class="diag-chip-lbl">Discovery</span>'),
      'Must NOT display 2 on Discovery chip in active scope'
    );
    assert.ok(html.includes('Zero Active Knowledge Regressions'), 'Must render zero active regressions card');
  });

  test('filters rendered issue groups by one or more selected pillars', () => {
    const issuesReport = {
      overall_score: 82.5,
      total_issues: 2,
      integrity_count: 1,
      structure_count: 0,
      consistency_count: 0,
      discoverability_count: 1,
      freshness_count: 0,
      issues: [
        {
          id: 'broken:note.md',
          pillar: 'integrity',
          category: 'broken_link',
          file_path: 'note.md',
          deduction_points: 1,
          weight: 1,
          description: 'Broken link',
          remediation_hint: 'Fix link',
        },
        {
          id: 'orphan:other.md',
          pillar: 'discoverability',
          category: 'orphan_node',
          file_path: 'other.md',
          deduction_points: 1,
          weight: 1,
          description: 'Orphan note',
          remediation_hint: 'Link note',
        },
      ],
    };

    const integrityOnly = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      issuesReport,
      selectedPillarFilter: 'integrity',
    });
    assert.ok(integrityOnly.includes('id="diag-body-integrity"'), 'Must keep selected pillar visible');
    assert.ok(!integrityOnly.includes('id="diag-body-discoverability"'), 'Must hide unselected pillar');

    const combined = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      issuesReport,
      selectedPillarFilter: 'integrity,discoverability',
    });
    assert.ok(combined.includes('id="diag-body-integrity"'), 'Must show first selected pillar');
    assert.ok(combined.includes('id="diag-body-discoverability"'), 'Must show second selected pillar');
  });

  test('scopes drawer-entering class strictly to isEntering state to prevent re-animation on updates', () => {
    const defaultHtml = renderSettingsDrawerHtml(sampleProps);
    assert.ok(!defaultHtml.includes('drawer-entering'), 'Must not have drawer-entering class by default or on property updates');

    const enteringHtml = renderSettingsDrawerHtml(sampleProps, true);
    assert.ok(enteringHtml.includes('drawer-entering'), 'Must include drawer-entering class when entering');
  });

  test('renders Structural Evolution tab with Orientation Selector and synchronized Pillar 2 Structural findings', () => {
    const htmlWithDebt = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'evolution',
      activeOrientation: 'ai_oriented',
      issuesReport: {
        timestamp: '2026-09-14T00:00:00Z',
        overall_score: 78,
        integrity_score: 95,
        consistency_score: 90,
        discoverability_score: 85,
        freshness_score: 80,
        legacy_debt_count: 1,
        regressions_count: 0,
        issues: [
          {
            id: 'struct:notes/giant.md',
            pillar: 'structure',
            category: 'oversized_document',
            file_path: 'notes/giant.md',
            deduction_points: 10,
            weight: 1.0,
            description: 'Document exceeds 50,000 tokens without fragment addressing',
            remediation_hint: 'Decompose or add section fragment anchors',
          },
        ],
      },
    });

    // 1. Must render Orientation Selector cards
    assert.ok(htmlWithDebt.includes('Documentation Orientation'), 'Must display Documentation Orientation heading');
    assert.ok(htmlWithDebt.includes('AI-Oriented'), 'Must render AI-Oriented mode card');
    assert.ok(htmlWithDebt.includes('Human-Oriented'), 'Must render Human-Oriented mode card');
    assert.ok(htmlWithDebt.includes('Balanced'), 'Must render Balanced mode card');
    assert.ok(htmlWithDebt.includes('Custom'), 'Must render Custom mode card');
    assert.ok(!htmlWithDebt.includes('Custom (Advanced)'), 'Must not include (Advanced) in Custom mode card');

    // 2. Must render Pillar 2 Structural findings matching Diagnostics
    assert.ok(htmlWithDebt.includes('Pillar 2 Structural Findings (1)'), 'Must render Pillar 2 Structural Findings card');
    assert.ok(htmlWithDebt.includes('notes/giant.md'), 'Must contain path of structural finding');
    assert.ok(htmlWithDebt.includes('Decompose or add section fragment anchors'), 'Must contain remediation hint');

    // 3. Must NOT say "Vault Structure is Balanced & Clean" when structure debt exists
    assert.ok(!htmlWithDebt.includes('Vault Structure is Balanced & Clean'), 'Must NOT report vault is balanced when structure debt exists');
    assert.ok(htmlWithDebt.includes('Pillar 2 Structural Debt Active (1)'), 'Must report active structure debt notice');
  });

  test('renders Custom Settings knobs and sliders when custom drawer is open or mode is custom', () => {
    const htmlCustom = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'evolution',
      activeOrientation: 'custom',
      isCustomDrawerOpen: true,
    });

    assert.ok(htmlCustom.includes('Custom KOS Profile Configuration'), 'Must render Custom KOS Profile heading');
    assert.ok(htmlCustom.includes('AI Discoverability'), 'Must render Tier 1 knobs');
    assert.ok(htmlCustom.includes('Cohesion Threshold'), 'Must render Tier 2 sliders');
    assert.ok(htmlCustom.includes('Save &amp; Compile Profile') || htmlCustom.includes('Save & Compile Profile'), 'Must render Save button');
  });

  test('renders Structural Evolution tab with active signatures and recommendations from sidecar without errors', () => {
    const rawRustSignatures: any = [
      {
        signature_id: 'sig_god_test',
        document_path: 'docs/test_doc.md',
        problem_type: 'god_document',
        confidence: 0.95,
        evidence: {
          cohesion: 0.5,
          coupling: 0.8,
          reference_concentration: 0.75,
          granularity: 0.3,
          context_efficiency: 0.25,
          discoverability: 0.8,
          reusability: 0.2,
          semantic_diversity: 0.85,
          navigational_complexity: 0.7,
          maintenance_coupling: 0.65,
        },
        rationale: 'High semantic diversity and concentrated references.',
      },
    ];

    const rawRustRecommendations: any = [
      {
        recommendation_id: 'rec_test_1',
        signature_id: 'sig_god_test',
        strategy: 'topic_decomposition',
        confidence: 0.92,
        expected_benefits: ['Eliminates God Document by splitting topics'],
        trade_offs: ['Increases file count'],
      },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'evolution',
      activeOrientation: 'ai_oriented',
      structuralSignatures: rawRustSignatures,
      structuralRecommendations: rawRustRecommendations,
    });

    assert.ok(html.includes('docs/test_doc.md'), 'Must render signature document path');
    assert.ok(html.includes('God Document'), 'Must render problem badge');
    assert.ok(html.includes('Topic Decomposition'), 'Must render recommendation strategy');
    assert.ok(html.includes('Eliminates God Document by splitting topics'), 'Must render expected benefits');
    assert.ok(html.includes('Increases file count'), 'Must render countervailing trade-offs');
  });

  test('renders Structure tab with 4-stat metrics grid, central hub, and top notes (T028c)', () => {
    const sampleNodes = [
      { id: 'a.md', label: 'Alpha', filePath: 'a.md', tags: [], color: '#3B82F6', status: 'active' as const, tokens: 100, hopDistance: 0, inDegree: 4, outDegree: 2, betweenness: 0.5, brokenLinks: 0 },
      { id: 'b.md', label: 'Beta', filePath: 'b.md', tags: [], color: '#3B82F6', status: 'active' as const, tokens: 100, hopDistance: -1, inDegree: 0, outDegree: 0, betweenness: 0, brokenLinks: 0 },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'structure',
      totalNotes: 2,
      connectedNotes: 1,
      orphanNotes: 1,
      nodes: sampleNodes,
    });

    assert.ok(html.includes('Knowledge Graph Structure'), 'Must render title');
    assert.ok(html.includes('Total Documents'), 'Must render Total Documents stat');
    assert.ok(html.includes('Connected'), 'Must render Connected stat');
    assert.ok(html.includes('Isolated'), 'Must render Isolated stat (never orphan)');
    assert.ok(html.includes('Central Hub Note'), 'Must render central hub card');
    assert.ok(html.includes('Top Connected Notes'), 'Must render Top Connected Notes section');
    assert.ok(html.includes('Alpha'), 'Must render top connected note');
    assert.ok(html.includes('Beta'), 'Must render isolated document');
  });

  test('gracefully routes legacy advanced tab alias to git_harness', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'advanced',
      totalNotes: 50,
      connectedNotes: 45,
      orphanNotes: 5,
      excludedNotes: 10,
    });

    assert.ok(html.includes('Admission Blocking Thresholds'), 'Must render Admission Blocking Thresholds');
    assert.ok(html.includes('toggle-block-broken'), 'Must render admission threshold checkbox');
    assert.ok(html.includes('Knowledge Debt Baseline'), 'Must render Knowledge Debt Baseline section');
    assert.ok(!html.includes('chk-threshold-adv'), 'Must not render redundant adv checkboxes');
  });

  test('renders Diagnostics tab with two-tier debt scopes and baselined badge', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      selectedDebtScope: 'all',
      issuesReport: {
        overall_score: 100,
        raw_score: 85,
        total_issues: 2,
        active_issues_count: 1,
        legacy_debt_count: 1,
        integrity_count: 1,
        freshness_count: 1,
        structure_count: 0,
        consistency_count: 0,
        discoverability_count: 0,
        issues: [
          {
            id: 'broken:active.md',
            pillar: 'integrity',
            category: 'broken_link',
            file_path: 'active.md',
            deduction_points: 5.0,
            weight: 1.0,
            description: 'New broken link regression',
            remediation_hint: 'Fix link',
            is_legacy: false,
          },
          {
            id: 'drift:legacy.md',
            pillar: 'freshness',
            category: 'technical_drift_contradiction',
            file_path: 'specs/legacy.md',
            deduction_points: 15.0,
            weight: 1.0,
            description: 'Historical drift',
            remediation_hint: 'Update doc or baseline',
            is_legacy: true,
            is_excluded: true,
          },
        ],
      },
    });

    assert.ok(html.includes('Active (1)'), 'Must render Active scope button with count');
    assert.ok(html.includes('Baselined (1)'), 'Must render Baselined button with count');
    assert.ok(html.includes('All (2)'), 'Must render All scope button with count');
    assert.ok(html.includes('Include Excluded'), 'Must render Include Excluded checkbox');
    assert.ok(html.includes('Baselined'), 'Must render Baselined badge for legacy issue');
    assert.ok(html.includes('Excluded'), 'Must render Excluded badge for excluded issue');
    assert.ok(html.includes('Audit: 85/100'), 'Must render raw unmasked audit score label');
    assert.ok(html.includes('100/100'), 'Must render effective 100/100 score');
  });

  test('renders Zero Active Knowledge Regressions empty state card when activeCount is 0 but legacy debt exists', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'diagnostics',
      selectedDebtScope: 'active',
      issuesReport: {
        overall_score: 100,
        raw_score: 85,
        total_issues: 176,
        active_issues_count: 0,
        legacy_debt_count: 176,
        integrity_count: 0,
        freshness_count: 176,
        structure_count: 0,
        consistency_count: 0,
        discoverability_count: 0,
        issues: [],
      },
    });

    assert.ok(html.includes('Zero Active Knowledge Regressions'), 'Must render quiet-by-default zero active regressions card');
    assert.ok(html.includes('Inspect Baselined (176)'), 'Must offer button to inspect baselined issues');
  });

  test('renders AI Harness Entry Point card when active harness is present (T025)', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git_harness',
      harnessStatus: {
        harnesses: [
          {
            file_path: 'AGENTS.md',
            harness_type: 'agents_md',
            is_primary: true,
            total_tokens: 3200,
            heading_count: 14,
            embedded_index: {
              heading_anchor: '#knowledge-index',
              start_line: 45,
              end_line: 75,
              routes: [
                {
                  domain: 'Architecture',
                  label: 'System Topology',
                  target_file: '03_architecture/system.md',
                  target_anchor: null,
                  resolution_status: 'resolved',
                  hop_distance_from_entry: 1,
                  target_tokens: 450,
                  line_number: 48,
                },
              ],
              domain_coverage_ratio: 0.85,
            },
            secondary_layers_detected: [
              {
                file_path: 'docs/index.md',
                parent_harness: 'AGENTS.md',
                outgoing_routes_count: 12,
                intermediate_hops: 1,
                penalty_points: 15,
                description: 'Intermediate index layer adding extra hop distance',
                recommendation_hint: 'Consolidate routing directly into AGENTS.md',
              },
            ],
            quality_ratings: {
              integrity_score: 95,
              structure_score: 90,
              consistency_score: 88,
              discoverability_score: 92,
              freshness_score: 100,
              context_efficiency: 78,
            },
          },
        ],
        active_harness: {
          file_path: 'AGENTS.md',
          harness_type: 'agents_md',
          is_primary: true,
          total_tokens: 3200,
          heading_count: 14,
          embedded_index: {
            heading_anchor: '#knowledge-index',
            start_line: 45,
            end_line: 75,
            routes: [
              {
                domain: 'Architecture',
                label: 'System Topology',
                target_file: '03_architecture/system.md',
                target_anchor: null,
                resolution_status: 'resolved',
                hop_distance_from_entry: 1,
                target_tokens: 450,
                line_number: 48,
              },
            ],
            domain_coverage_ratio: 0.85,
          },
          secondary_layers_detected: [
            {
              file_path: 'docs/index.md',
              parent_harness: 'AGENTS.md',
              outgoing_routes_count: 12,
              intermediate_hops: 1,
              penalty_points: 15,
              description: 'Intermediate index layer adding extra hop distance',
              recommendation_hint: 'Consolidate routing directly into AGENTS.md',
            },
          ],
          quality_ratings: {
            integrity_score: 95,
            structure_score: 90,
            consistency_score: 88,
            discoverability_score: 92,
            freshness_score: 100,
            context_efficiency: 78,
          },
        },
      },
    });

    assert.ok(html.includes('harness-entry-card'), 'Must render harness entry card');
    assert.ok(html.includes('AI Harness Entry Point'), 'Must render AI Harness title');
    assert.ok(html.includes('Primary Entry'), 'Must render Primary Entry badge');
    assert.ok(html.includes('AGENTS.md'), 'Must render harness path');
    assert.ok(html.includes('3,200 tokens'), 'Must render formatted token count');
    assert.ok(html.includes('14 headings'), 'Must render heading count');
    assert.ok(html.includes('harness-ratings-grid'), 'Must render 6-rating grid');
    assert.ok(html.includes('Context Efficiency'), 'Must render Context Efficiency rating label');
    assert.ok(html.includes('78%'), 'Must render Context Efficiency percentage');
    assert.ok(html.includes('1 routes'), 'Must render embedded index route count');
    assert.ok(html.includes('85% domain coverage'), 'Must render domain coverage');
    assert.ok(html.includes('secondary-layer-alert-card'), 'Must render secondary layer alert');
    assert.ok(html.includes('docs/index.md'), 'Must render secondary layer file path');
  });

  test('renders Not Detected state with template initializer button when no harness is present (T025)', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git_harness',
      harnessStatus: {
        harnesses: [],
        active_harness: null,
      },
    });

    assert.ok(html.includes('harness-entry-card'), 'Must render harness entry card');
    assert.ok(html.includes('Not Detected'), 'Must show Not Detected pill');
    assert.ok(html.includes('btn-init-harness'), 'Must render template initialization button');
  });

  test('renders Agent Skills Catalog and Procedure Candidates (T034)', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git_harness',
      skills: [
        {
          skill_id: 'deploy-service',
          dir_path: '.agents/skills/deploy-service',
          manifest_path: '.agents/skills/deploy-service/SKILL.md',
          name: 'deploy-service',
          description: 'Deploy service to production environment.',
          assets: [
            {
              asset_type: 'script',
              file_path: 'scripts/deploy.sh',
              exists_on_disk: true,
              byte_size: 1024,
            },
          ],
          status: 'valid',
          unreferenced: false,
          overlap_ids: [],
        },
        {
          skill_id: 'broken-skill',
          dir_path: '.agents/skills/broken-skill',
          manifest_path: '.agents/skills/broken-skill/SKILL.md',
          name: 'broken-skill',
          description: 'Skill with missing script.',
          assets: [
            {
              asset_type: 'script',
              file_path: 'scripts/missing.sh',
              exists_on_disk: false,
              byte_size: 0,
            },
          ],
          status: 'broken_asset_reference',
          unreferenced: true,
          overlap_ids: [],
        },
      ],
      procedureCandidates: [
        {
          candidate_id: 'proc_123',
          step_signatures: [
            'run backup script scripts/db_backup.sh',
            'apply migration schema',
            'verify index integrity',
          ],
          occurrence_count: 3,
          source_documents: ['docs/setup.md', 'docs/recovery.md', 'docs/release.md'],
          associated_scripts: ['scripts/db_backup.sh'],
          recurrence_score: 0.85,
          recommended_skill_name: 'database-migration',
        },
      ],
    });

    assert.ok(html.includes('skills-catalog-card'), 'Must render skills catalog card');
    assert.ok(html.includes('Agent Skills Catalog'), 'Must render Skills Catalog title');
    assert.ok(html.includes('deploy-service'), 'Must render valid skill name');
    assert.ok(html.includes('Valid'), 'Must render Valid badge');
    assert.ok(html.includes('scripts/deploy.sh'), 'Must render asset path');
    assert.ok(html.includes('broken-skill'), 'Must render broken skill name');
    assert.ok(html.includes('Broken Asset'), 'Must render Broken Asset badge');
    assert.ok(html.includes('missing on disk'), 'Must indicate asset missing on disk');
    assert.ok(html.includes('Procedure Candidates for Skill Promotion'), 'Must render Procedure Candidates section');
    assert.ok(html.includes('database-migration'), 'Must render recommended skill name');
    assert.ok(html.includes('85% Recurrence'), 'Must render recurrence percentage');
    assert.ok(html.includes('3 docs'), 'Must render document occurrence count');
  });

  test('renders separate Git and Harness tab navigation buttons', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git',
    });

    assert.ok(html.includes('data-tab="git">Git</button>'), 'Must render separate Git tab button');
    assert.ok(html.includes('data-tab="harness">Harness</button>'), 'Must render separate Harness tab button');
    assert.ok(html.includes('Git Hook Integration & Dual Hooks'), 'Must render Git hooks section in git tab');
    assert.ok(!html.includes('harness-base-prep-card'), 'Must not render harness base prep card in git tab');
  });

  test('renders AI Tool & Base Preparation Card in Harness tab', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      harnessStatus: {
        harnesses: [
          {
            file_path: 'AGENTS.md',
            harness_type: 'agents_md',
            is_primary: true,
            total_tokens: 3000,
            heading_count: 10,
            embedded_index: null,
            secondary_layers_detected: [],
            quality_ratings: {
              integrity_score: 100,
              structure_score: 100,
              consistency_score: 100,
              discoverability_score: 100,
              freshness_score: 100,
              context_efficiency: 100,
            },
          },
        ],
        active_harness: null,
      },
    });

    assert.ok(html.includes('harness-base-prep-card'), 'Must render harness base prep card');
    assert.ok(html.includes('AI Tool & Base Preparation'), 'Must render AI Tool & Base Preparation title');
    assert.ok(html.includes('id="chk-tool-cursor"'), 'Must render Cursor checkbox');
    assert.ok(html.includes('id="chk-tool-codex"'), 'Must render Codex checkbox');
    assert.ok(html.includes('id="chk-tool-agy"'), 'Must render Agy checkbox');
    assert.ok(html.includes('id="chk-tool-claude"'), 'Must render Claude Code checkbox');
    assert.ok(html.includes('id="btn-prepare-harness-base"'), 'Must render Prepare & Configure Base button');
    assert.ok(html.includes('id="chk-prep-clean-unused"'), 'Must render clean unused checkbox');
    assert.ok(html.includes('id="chk-prep-consolidate-index"'), 'Must render consolidate index checkbox');
    assert.ok(html.includes('id="select-prep-active-harness"'), 'Must render Primary Active combo box');
  });

  test('renders Primary Active combo box with candidates matching tool selection and respects selectedPrepActiveHarness', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      toolSelection: {
        cursor: true,
        codex: true,
        agy: true,
        claude: false,
        copilot: false,
      },
      selectedPrepActiveHarness: 'GEMINI.md',
      harnessStatus: {
        harnesses: [
          {
            file_path: 'AGENTS.md',
            harness_type: 'agents_md',
            is_primary: true,
            total_tokens: 3000,
            heading_count: 10,
            embedded_index: null,
            secondary_layers_detected: [],
            quality_ratings: {
              integrity_score: 100,
              structure_score: 100,
              consistency_score: 100,
              discoverability_score: 100,
              freshness_score: 100,
              context_efficiency: 100,
            },
          },
        ],
        active_harness: {
          file_path: 'AGENTS.md',
          harness_type: 'agents_md',
          is_primary: true,
          total_tokens: 3000,
          heading_count: 10,
          embedded_index: null,
          secondary_layers_detected: [],
          quality_ratings: {
            integrity_score: 100,
            structure_score: 100,
            consistency_score: 100,
            discoverability_score: 100,
            freshness_score: 100,
            context_efficiency: 100,
          },
        },
      },
    });

    assert.ok(html.includes('id="select-prep-active-harness"'), 'Must render combo box');
    assert.ok(html.includes('id="btn-prep-active-toggle"'), 'Must render custom toggle button');
    assert.ok(html.includes('id="menu-prep-active-options"'), 'Must render custom options menu');
    assert.ok(html.includes('data-value="AGENTS.md"'), 'Must include AGENTS.md');
    assert.ok(html.includes('data-value="GEMINI.md"'), 'Must include GEMINI.md');
    assert.ok(html.includes('id="label-prep-active-selected">GEMINI.md</span>'), 'GEMINI.md must be selected in combo box');
    assert.ok(!html.includes('data-value="CLAUDE.md"'), 'CLAUDE.md must not be present when unchecked');
  });

  test('synchronizes Primary Active selection with select-active-harness when multiple harnesses exist', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      selectedPrepActiveHarness: 'GEMINI.md',
      harnessStatus: {
        harnesses: [
          {
            file_path: 'AGENTS.md',
            harness_type: 'agents_md',
            is_primary: false,
            total_tokens: 3000,
            heading_count: 10,
            embedded_index: null,
            secondary_layers_detected: [],
            quality_ratings: {
              integrity_score: 100,
              structure_score: 100,
              consistency_score: 100,
              discoverability_score: 100,
              freshness_score: 100,
              context_efficiency: 100,
            },
          },
          {
            file_path: 'GEMINI.md',
            harness_type: 'gemini_md',
            is_primary: true,
            total_tokens: 2500,
            heading_count: 8,
            embedded_index: null,
            secondary_layers_detected: [],
            quality_ratings: {
              integrity_score: 100,
              structure_score: 100,
              consistency_score: 100,
              discoverability_score: 100,
              freshness_score: 100,
              context_efficiency: 100,
            },
          },
        ],
        active_harness: {
          file_path: 'GEMINI.md',
          harness_type: 'gemini_md',
          is_primary: true,
          total_tokens: 2500,
          heading_count: 8,
          embedded_index: null,
          secondary_layers_detected: [],
          quality_ratings: {
            integrity_score: 100,
            structure_score: 100,
            consistency_score: 100,
            discoverability_score: 100,
            freshness_score: 100,
            context_efficiency: 100,
          },
        },
      },
    });

    assert.ok(html.includes('id="select-active-harness"'), 'Must render select-active-harness when >1 harnesses exist');
    assert.ok(html.includes('id="active-harness-dropdown"'), 'Must render active-harness-dropdown');
    assert.ok(html.includes('id="btn-active-harness-toggle"'), 'Must render active-harness toggle button');
    assert.ok(html.includes('id="label-active-harness-selected">GEMINI.md · gemini_md</span>'), 'select-active-harness must have GEMINI.md selected');
    assert.ok(html.includes('id="label-prep-active-selected">GEMINI.md</span>'), 'select-prep-active-harness must have GEMINI.md selected');
  });

  test('renders Load tab with Token Economics breakdown, gauges, and top consumers', () => {
    const sampleNodes = [
      { id: '00_meta/wiki_index.md', label: 'Wiki Index', filePath: '00_meta/wiki_index.md', tags: ['#meta'], color: '#3B82F6', status: 'active' as const, tokens: 12500, hopDistance: 0, inDegree: 5, outDegree: 5, betweenness: 0.5, brokenLinks: 0 },
      { id: '01_projects/proj.md', label: 'Big Doc', filePath: '01_projects/proj.md', tags: ['#projects'], color: '#3B82F6', status: 'active' as const, tokens: 45000, hopDistance: 1, inDegree: 2, outDegree: 2, betweenness: 0.2, brokenLinks: 0 },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'load',
      nodes: sampleNodes,
      tokenSummary: {
        activeHarnessTokens: 12500,
        subcallAccessibleTokens: 57500,
        warningThreshold: 60000,
        criticalThreshold: 100000,
        status: 'normal',
        contextReductionPercent: 78,
      },
    });

    assert.ok(html.includes('Active Prompt Load — L0 Budget'), 'Must render Active Prompt Load header');
    assert.ok(html.includes('Reachable Subcall Context — L1–L2'), 'Must render Reachable Subcall Context header');
    assert.ok(html.includes('Top Token Consumers'), 'Must render Top Token Consumers header');
    assert.ok(html.includes('12.5k'), 'Must render active prompt load tokens');
    assert.ok(html.includes('57.5k'), 'Must render reachable subcall tokens');
    assert.ok(html.includes('Normal'), 'Must render budget status badge');
    assert.ok(html.includes('~78% reduction'), 'Must render reduction badge');
    assert.ok(html.includes('Big Doc'), 'Must list top consumer label');
    assert.ok(html.includes('01_projects/proj.md'), 'Must list top consumer file path');
    assert.ok(html.includes('class="load-consumer-item"'), 'Must have clickable consumer items');
  });

  test('renders Evolution tab with pagination bars, filter chips, and custom popover menus', () => {
    const sampleSignatures = Array.from({ length: 25 }, (_, i) => ({
      signatureId: `sig_${i}`,
      documentPath: `notes/silo_${i}.md`,
      problemType: 'isolated_silo',
      confidence: 0.88,
      evidence: {},
      rationale: `Silo ${i}`,
    }));

    const sampleFindings = [
      { file_path: 'notes/broken.md', description: 'Broken link', category: 'broken_link', pillar: 'integrity' },
      { file_path: 'notes/silo_0.md', description: 'Orphan note', category: 'isolated_orphan', pillar: 'discoverability' },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'evolution',
      structuralSignatures: sampleSignatures as any,
      structuralRecommendations: [],
      issuesReport: {
        total_findings: 2,
        active_findings: 2,
        legacy_debt_count: 0,
        baseline_commit: null,
        debt_limit_exceeded: false,
        findings: sampleFindings,
      } as any,
      evolutionFindingsPage: 1,
      evolutionFindingsPageSize: 10,
      evolutionSignaturesPage: 1,
      evolutionSignaturesPageSize: 10,
      evolutionPillarFilter: 'all',
      selectedSignatureTypeFilter: 'all',
    });

    // Evolution tab header & orientation
    assert.ok(html.includes('evolution-settings-wrapper'), 'Must render evolution-settings-wrapper');
    assert.ok(html.includes('Balanced'), 'Must render active orientation');

    // Unified Findings header & single pagination bar
    assert.ok(html.includes('Vault Knowledge & KOS Findings (2)'), 'Must render unified findings title with vault findings count');
    assert.ok(html.includes('single-pagination-bar'), 'Must render single unified pagination bar');
    assert.ok(html.includes('findings-pagination-bar'), 'Must render findings pagination bar');
    assert.ok(html.includes('Showing <strong>1–2</strong> of <strong>2</strong> findings'), 'Must paginate unified list');

    // Quality Pillar filter chips in KOS menu bar
    assert.ok(html.includes('findings-pillar-chip'), 'Must render pillar filter chips');
    assert.ok(html.includes('data-pillar-id="integrity"'), 'Must have integrity filter chip');
    assert.ok(html.includes('data-pillar-id="discoverability"'), 'Must have discoverability filter chip');
  });

  test('renders Harness Required & Suggested Tools catalog card with badges, details, and buttons in Harness tab', () => {
    const sampleHarnessTools = [
      {
        id: 'prek',
        name: 'prek (Rust Git Hook Orchestrator)',
        category: 'core' as const,
        binary: 'prek',
        description: 'Rust-native Git hook orchestrator.',
        is_installed: true,
        version: '0.2.4',
        bin_path: '/usr/local/bin/prek',
        install_command: 'cargo install prek',
        required_for: 'Dual Git Hooks (Tier 1)',
      },
      {
        id: 'claude',
        name: 'Claude Code (Anthropic CLI Agent)',
        category: 'agent' as const,
        binary: 'claude',
        description: 'Anthropic agentic CLI for code modifications.',
        is_installed: false,
        version: null,
        bin_path: null,
        install_command: 'npm install -g @anthropic-ai/claude-code',
        required_for: 'CLAUDE.md Agent Harness',
      },
    ];

    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      harnessTools: sampleHarnessTools,
    });

    // 1. Catalog Card Container & Header
    assert.ok(html.includes('id="harness-tools-catalog-card"'), 'Must render harness tools catalog card');
    assert.ok(html.includes('AI Harness & Agent Tooling (Optional Accelerators)'), 'Must display catalog card title');
    assert.ok(html.includes('id="btn-refresh-harness-tools"'), 'Must render Scan Tools refresh button');
    assert.ok(html.includes('Scan Tools'), 'Button text must be Scan Tools');

    // 2. Installed tool (prek)
    assert.ok(html.includes('data-tool-id="prek"'), 'Must render item card for prek');
    assert.ok(html.includes('prek (Rust Git Hook Orchestrator)'), 'Must render prek tool name');
    assert.ok(html.includes('Version: <strong>0.2.4</strong>'), 'Must display installed tool version');
    assert.ok(html.includes('(/usr/local/bin/prek)'), 'Must display installed tool binary path');
    assert.ok(html.includes('Reinstall'), 'Must render Reinstall button for installed tool');

    // 3. Uninstalled tool (claude)
    assert.ok(html.includes('data-tool-id="claude"'), 'Must render item card for claude');
    assert.ok(html.includes('Claude Code (Anthropic CLI Agent)'), 'Must render claude tool name');
    assert.ok(html.includes('Install: <code>npm install -g @anthropic-ai/claude-code</code>'), 'Must display install command');
    assert.ok(html.includes('Install claude'), 'Must render Install <binary> button for uninstalled tool');
  });

  test('renders complete fallback harness tools catalog including graphify and ai-memory when harnessTools is empty', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      harnessTools: [],
    });

    assert.ok(html.includes('id="harness-tools-catalog-card"'), 'Must render catalog card from fallback');
    assert.ok(html.includes('data-tool-id="graphify"'), 'Must include graphify in fallback catalog');
    assert.ok(html.includes('graphify (Code Architecture Graph Extractor)'), 'Must render graphify name');
    assert.ok(html.includes('data-tool-id="ai_memory"'), 'Must include ai-memory in fallback catalog');
    assert.ok(html.includes('data-tool-id="notebooklm"'), 'Must include notebooklm in fallback catalog');
    assert.ok(html.includes('data-tool-id="uv"'), 'Must include uv in fallback catalog');
    assert.ok(html.includes('data-tool-id="prek"'), 'Must include prek in fallback catalog');
  });

  test('renders prek status and install button in Git tab when uninstalled vs installed', () => {
    // Case 1: prek is not installed
    const htmlMissing = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git',
      harnessTools: [
        {
          id: 'prek',
          name: 'prek (Rust Git Hook Orchestrator)',
          category: 'core' as const,
          binary: 'prek',
          description: 'Rust-native Git hook orchestrator.',
          is_installed: false,
          version: null,
          bin_path: null,
          install_command: 'cargo install prek',
        },
      ],
    });

    assert.ok(htmlMissing.includes('prek Orchestrator (Rust):'), 'Must render prek orchestrator row');
    assert.ok(htmlMissing.includes('Install prek'), 'Must render Install prek button when missing');
    assert.ok(htmlMissing.includes('data-tool-id="prek"'), 'Must specify data-tool-id="prek" on install button');

    // Case 2: prek is installed
    const htmlInstalled = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git',
      harnessTools: [
        {
          id: 'prek',
          name: 'prek (Rust Git Hook Orchestrator)',
          category: 'core' as const,
          binary: 'prek',
          description: 'Rust-native Git hook orchestrator.',
          is_installed: true,
          version: '0.2.4',
          bin_path: '/usr/local/bin/prek',
          install_command: 'cargo install prek',
        },
      ],
    });

    assert.ok(htmlInstalled.includes('prek Orchestrator (Rust):'), 'Must render prek orchestrator row');
    assert.ok(!htmlInstalled.includes('Install prek'), 'Must not render Install prek button when already active');
  });

  test('gracefully resolves active harness from graph nodes notes/AGENTS.md when harnessStatus has not yet loaded', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'harness',
      harnessStatus: null,
      primaryHub: 'notes/AGENTS.md',
      nodes: [
        {
          id: 'notes/AGENTS.md',
          label: 'AGENTS.md — AI Harness Entry Point',
          filePath: 'notes/AGENTS.md',
          tags: [],
          color: '#38bdf8',
          status: 'indexed',
          tokens: 4200,
          hopDistance: 0,
          inDegree: 10,
          outDegree: 5,
          betweenness: 0.85,
          brokenLinks: 0,
        },
      ],
    });

    assert.ok(html.includes('Primary Entry'), 'Must render Primary Entry badge');
    assert.ok(html.includes('notes/AGENTS.md'), 'Must display notes/AGENTS.md as the active harness');
    assert.ok(!html.includes('Not Detected'), 'Must not render Not Detected when graph node exists');
  });
});





