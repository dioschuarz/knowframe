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
});
