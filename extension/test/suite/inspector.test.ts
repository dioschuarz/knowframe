import { describe, it } from 'node:test';
import assert from 'node:assert';
import {
  renderNodeInspectionHtml,
  renderEdgeInspectionHtml,
  renderTokenEconomicsHtml,
  formatHopTier,
} from '../../webview-ui/src/components/inspectorDrawer.ts';
import type { NoteNode, NoteEdge, TokenSummary } from '../../src/types/graph.ts';

describe('InspectorDrawer Test Suite', () => {
  it('formats hop tier names correctly', () => {
    assert.strictEqual(formatHopTier(0), 'Level 0 (Central Hub)');
    assert.strictEqual(formatHopTier(1), 'Level 1 (Direct Reference)');
    assert.strictEqual(formatHopTier(2), 'Level 2 (Transitive Reference)');
    assert.strictEqual(formatHopTier(3), 'Level 3 (Deep Reference)');
    assert.strictEqual(formatHopTier(-1), 'Orphan (Disconnected)');
  });

  it('renders node inspection view with metrics, links, and contextual actions', () => {
    const node: NoteNode = {
      id: '01_projects/my_project.md',
      label: 'My Project',
      filePath: '01_projects/my_project.md',
      tags: ['#architecture', '#specs'],
      color: '#3b82f6',
      status: 'active',
      tokens: 4250,
      hopDistance: 1,
      inDegree: 3,
      outDegree: 5,
      betweenness: 0.125,
      brokenLinks: 0,
    };

    const inLinks = ['index.md', 'notes/active_context.md', 'notes/decisions_log.md'];
    const outLinks = ['docs/setup.md', 'docs/api.md'];

    const html = renderNodeInspectionHtml({
      node,
      inLinks,
      outLinks,
      isHub: false,
    });

    // Check title and file path
    assert.ok(html.includes('My Project'), 'Should display note label');
    assert.ok(html.includes('01_projects/my_project.md'), 'Should display file path');

    // Check tier and token weight
    assert.ok(html.includes('Level 1 (Direct Reference)'), 'Should display correct tier');
    assert.ok(html.includes('4.3k tok') || html.includes('4,250 tok'), 'Should display tokens');

    // Check links
    assert.ok(html.includes('index.md'), 'Should list incoming link');
    assert.ok(html.includes('docs/api.md'), 'Should list outgoing link');

    // Check primary actions
    assert.ok(html.includes('id="insp-btn-open-file"'), 'Should render Open file button');
    assert.ok(html.includes('id="insp-btn-set-hub"'), 'Should render Set as hub button');
    assert.ok(html.includes('id="insp-btn-exclude"'), 'Should render Exclude button');
  });

  it('renders broken link edge inspection view with line-precise coordinates and actions', () => {
    const edge: NoteEdge = {
      id: 'notes/api.md->notes/missing_doc.md:42',
      source: 'notes/api.md',
      target: 'notes/missing_doc.md',
      isBroken: true,
      linkType: 'wikilink',
      line: 42,
      anchor: 'installation-guide',
      resolutionStatus: 'broken_missing_file',
    };

    const html = renderEdgeInspectionHtml({
      edge,
      sourceLabel: 'API Architecture',
    });

    assert.ok(html.includes('API Architecture') || html.includes('notes/api.md'), 'Should display source file');
    assert.ok(html.includes('notes/missing_doc.md'), 'Should display target path');
    assert.ok(html.includes('Line 42') || html.includes('L42') || html.includes('line 42'), 'Should display line number');
    assert.ok(html.includes('broken_missing_file') || html.includes('Missing File'), 'Should display error type');

    // Check action buttons
    assert.ok(html.includes('id="insp-btn-open-source"'), 'Should render Open source button');
    assert.ok(html.includes('id="insp-btn-fix-reference"'), 'Should render Fix reference button');
  });

  it('renders token economics breakdown view with budget bars and top consumers', () => {
    const summary: TokenSummary = {
      activeHarnessTokens: 4920,
      subcallAccessibleTokens: 18500,
      warningThreshold: 60000,
      criticalThreshold: 100000,
      status: 'normal',
    };

    const topNodes: Array<{ label: string; filePath: string; tokens: number }> = [
      { label: 'Active Context', filePath: 'notes/02_memory/active_context.md', tokens: 2800 },
      { label: 'Architecture', filePath: 'notes/03_architecture/system.md', tokens: 1450 },
    ];

    const html = renderTokenEconomicsHtml({
      summary,
      topNodes,
    });

    assert.ok(html.includes('Active Prompt Load'), 'Should display L0 title');
    assert.ok(html.includes('4.9k') || html.includes('4,920'), 'Should display active tokens');
    assert.ok(html.includes('18.5k') || html.includes('18,500'), 'Should display reachable tokens');
    assert.ok(html.includes('Active Context'), 'Should list top consumer node');
    assert.ok(html.includes('2.8k') || html.includes('2,800'), 'Should display top consumer tokens');
  });
});
