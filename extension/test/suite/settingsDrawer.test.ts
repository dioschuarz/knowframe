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
});
