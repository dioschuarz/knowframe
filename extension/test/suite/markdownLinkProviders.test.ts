import { describe, it } from 'node:test';
import * as assert from 'node:assert/strict';
import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';
import {
  extractWikilinksInMarkdown,
  resolveWikilinkTarget,
  computeRelativePosixPath,
  slugifyHeading,
} from '../../src/providers/markdownLinkResolver.ts';
import { registerKnowFrameWikilinkPlugin } from '../../src/providers/markdownItWikilinks.ts';

describe('Markdown Link Clickability & Preview Plugin Test Suite', () => {
  it('extracts wikilinks while ignoring YAML frontmatter, inline code, and fenced code blocks', () => {
    const content = [
      '---',
      'title: Test',
      'ref: "[[frontmatter_ignored]]"',
      '---',
      '# Title',
      'Inline code `[[inline_code_ignored|No]]` should be skipped.',
      '```markdown',
      '[[fenced_code_ignored|No]]',
      '```',
      '- [[00_meta/wiki_index|Master Index]]',
      '- [[07_c_suite/ctaas/foundations/product_management#the-5-core-tenets|Core Tenets]]',
      '| [[docs/spec\\|Table Label]] |',
      '![[image_embed.png]]',
    ].join('\n');

    const links = extractWikilinksInMarkdown(content);
    const nonEmbeds = links.filter((l) => !l.isEmbed);
    assert.equal(nonEmbeds.length, 3);
    assert.equal(nonEmbeds[0].rawTarget, '00_meta/wiki_index');
    assert.equal(nonEmbeds[0].label, 'Master Index');
    assert.equal(nonEmbeds[1].rawTarget, '07_c_suite/ctaas/foundations/product_management');
    assert.equal(nonEmbeds[1].anchor, 'the-5-core-tenets');
    assert.equal(nonEmbeds[1].label, 'Core Tenets');
    assert.equal(nonEmbeds[2].rawTarget, 'docs/spec');
    assert.equal(nonEmbeds[2].label, 'Table Label');
    assert.equal(links.filter((l) => l.isEmbed).length, 1);
  });

  it('resolves valid files and fragments to exact relative paths & heading lines, and rejects broken targets/anchors', () => {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'knowframe-links-'));
    try {
      fs.mkdirSync(path.join(tmp, 'notes', '00_meta'), { recursive: true });
      fs.mkdirSync(path.join(tmp, 'knowframe'), { recursive: true });
      fs.writeFileSync(path.join(tmp, 'AGENTS.md'), '# Root AGENTS\n');
      fs.writeFileSync(path.join(tmp, 'notes', 'AGENTS.md'), '# Notes AGENTS\n');
      fs.writeFileSync(
        path.join(tmp, 'notes', '00_meta', 'wiki_index.md'),
        '# Master Index\n\n## The 5 Core Tenets\nContent.\n'
      );
      fs.writeFileSync(path.join(tmp, 'knowframe', 'README.md'), '# KnowFrame\n');

      const sourcePath = path.join(tmp, 'notes', 'AGENTS.md');

      // 1. Bare [[AGENTS]] from notes/AGENTS.md resolves to root ../AGENTS.md
      const rootRes = resolveWikilinkTarget(tmp, sourcePath, 'AGENTS', null, null);
      assert.ok(rootRes);
      assert.equal(rootRes.canonicalId, 'AGENTS.md');
      assert.equal(rootRes.relativePosixPath, '../AGENTS.md');

      // 2. Valid target + valid fragment resolves with exact line number (line 3)
      const fragRes = resolveWikilinkTarget(
        tmp,
        sourcePath,
        '00_meta/wiki_index',
        'the-5-core-tenets',
        null
      );
      assert.ok(fragRes);
      assert.equal(fragRes.relativePosixPath, '00_meta/wiki_index.md');
      assert.equal(fragRes.fragmentSlug, 'the-5-core-tenets');
      assert.equal(fragRes.targetLine, 3);

      // 3. Cross-folder target [[knowframe/README]] -> ../knowframe/README.md
      const crossRes = resolveWikilinkTarget(tmp, sourcePath, 'knowframe/README', null, null);
      assert.ok(crossRes);
      assert.equal(crossRes.relativePosixPath, '../knowframe/README.md');

      // 4. Broken target -> null (not clickable)
      const missingRes = resolveWikilinkTarget(tmp, sourcePath, 'non_existent_doc', null, null);
      assert.equal(missingRes, null);

      // 5. Existing file + non-existent fragment -> null (not clickable)
      const brokenFrag = resolveWikilinkTarget(
        tmp,
        sourcePath,
        '00_meta/wiki_index',
        'does-not-exist-heading',
        null
      );
      assert.equal(brokenFrag, null);
    } finally {
      fs.rmSync(tmp, { recursive: true, force: true });
    }
  });

  it('markdown-it preview plugin emits clickable <a> link tokens for valid wikilinks and skips broken ones', () => {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'knowframe-mdit-'));
    try {
      fs.mkdirSync(path.join(tmp, 'notes', '00_meta'), { recursive: true });
      fs.writeFileSync(path.join(tmp, 'notes', 'AGENTS.md'), '# Notes\n');
      fs.writeFileSync(
        path.join(tmp, 'notes', '00_meta', 'wiki_index.md'),
        '# Index\n\n## Architecture Overview\nText.\n'
      );

      let registeredRule: ((state: any, silent: boolean) => boolean) | null = null;
      const mockMd = {
        inline: {
          ruler: {
            before: (_before: string, _name: string, fn: (state: any, silent: boolean) => boolean) => {
              registeredRule = fn;
            },
          },
        },
      };

      registerKnowFrameWikilinkPlugin(mockMd, {
        isEnabled: () => true,
        getWorkspaceRoot: () => tmp,
        getFallbackDocumentPath: () => path.join(tmp, 'notes', 'AGENTS.md'),
        getScanResult: () => null,
      });

      assert.ok(registeredRule, 'Plugin must register markdown-it inline rule');

      const tokens: any[] = [];
      const makeState = (src: string) => ({
        src,
        pos: 0,
        posMax: src.length,
        env: { currentDocument: { fsPath: path.join(tmp, 'notes', 'AGENTS.md') } },
        push: (type: string, tag: string, nesting: number) => {
          const attrs = new Map<string, string>();
          const tok = {
            type,
            tag,
            nesting,
            content: '',
            attrs,
            attrSet: (k: string, v: string) => attrs.set(k, v),
          };
          tokens.push(tok);
          return tok;
        },
      });

      // Valid wikilink with fragment -> produces <a href="00_meta/wiki_index.md#architecture-overview">
      const validState = makeState('[[00_meta/wiki_index#architecture-overview|Arch]]');
      const matchedValid = registeredRule!(validState, false);
      assert.equal(matchedValid, true);
      assert.equal(tokens.length, 3);
      assert.equal(tokens[0].type, 'link_open');
      assert.equal(tokens[0].attrs.get('href'), '00_meta/wiki_index.md#architecture-overview');
      assert.equal(tokens[1].content, 'Arch');
      assert.equal(tokens[2].type, 'link_close');

      // Broken wikilink -> returns false (not rendered as clickable link)
      tokens.length = 0;
      const brokenState = makeState('[[missing/file|Missing]]');
      assert.equal(registeredRule!(brokenState, false), false);
      assert.equal(tokens.length, 0);

      // Helper assertions
      assert.equal(slugifyHeading('2. Product Design & Feature Architecture'), '2-product-design-feature-architecture');
      assert.equal(computeRelativePosixPath('notes/AGENTS.md', 'AGENTS.md'), '../AGENTS.md');
    } finally {
      fs.rmSync(tmp, { recursive: true, force: true });
    }
  });
});
