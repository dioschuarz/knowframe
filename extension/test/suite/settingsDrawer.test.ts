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

  test('renders 5 tabs: Knowledge, Excluded Paths, Cluster Groups, Git Sync, Diagnostics', () => {
    const html = renderSettingsDrawerHtml(sampleProps);

    assert.ok(html.includes('Knowledge'), 'Must have Knowledge tab');
    assert.ok(html.includes('Excluded Paths'), 'Must have Excluded Paths tab');
    assert.ok(html.includes('Cluster Groups'), 'Must have Cluster Groups tab');
    assert.ok(html.includes('Git Sync'), 'Must have Git Sync tab');
    assert.ok(html.includes('Diagnostics'), 'Must have Diagnostics tab');
  });

  test('renders active tab content correctly for excluded paths', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'exclusions',
    });

    assert.ok(html.includes('packages/**/README.md'), 'Must render graph exclusion rule');
    assert.ok(html.includes('vendor/**'), 'Must render vendor rule');
    assert.ok(html.includes('+ Add Rule'), 'Must render add rule button');
  });

  test('renders Git Sync controls with hook status', () => {
    const html = renderSettingsDrawerHtml({
      ...sampleProps,
      activeTab: 'git_sync',
    });

    assert.ok(html.includes('Git Hook Integration'), 'Must display Git hook header');
    assert.ok(html.includes('Installed') || html.includes('Active'), 'Must show hook status');
    assert.ok(html.includes('Reinstall') || html.includes('Install'), 'Must have install/reinstall button');
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
    assert.ok(html.includes('All (2)'), 'Must render All (2) filter');
    assert.ok(html.includes('Integrity (1)'), 'Must render Integrity (1) filter');
    assert.ok(html.includes('Discovery (1)'), 'Must render Discovery (1) filter');

    // 2. Accordions & issues
    assert.ok(html.includes('INTEGRITY'), 'Must render Integrity accordion');
    assert.ok(html.includes('DISCOVERABILITY'), 'Must render Discoverability accordion');
    assert.ok(html.includes('00_meta/wiki_index.md:86'), 'Must display exact file location with line');
    assert.ok(html.includes('missing_target.md'), 'Must display target reference');
    assert.ok(html.includes('Open Note'), 'Must offer Open Note action button');
    assert.ok(html.includes('Fix Reference'), 'Must offer Fix Reference action button for broken links');
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
    assert.ok(htmlWithDebt.includes('Custom (Advanced)'), 'Must render Custom mode card');

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
});

