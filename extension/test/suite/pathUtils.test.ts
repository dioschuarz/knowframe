import { describe, it } from 'node:test';
import assert from 'node:assert';
import { isSameFilePath, normalizePath, isAbsolutePath } from '../../webview-ui/src/utils/pathUtils.ts';

describe('Path Utilities & Collision Prevention Suite', () => {
  it('normalizes paths across platforms and formats', () => {
    assert.strictEqual(normalizePath('notes\\06_studies\\assets\\report_nb.md'), 'notes/06_studies/assets/report_nb.md');
    assert.strictEqual(normalizePath('./notes/01_projects/core.md'), 'notes/01_projects/core.md');
    assert.strictEqual(normalizePath('NOTES/00_META/WIKI_INDEX.MD'), 'notes/00_meta/wiki_index.md');
    assert.strictEqual(normalizePath('  docs/guide/  '), 'docs/guide');
    assert.strictEqual(normalizePath(''), '');
    assert.strictEqual(normalizePath(null), '');
    assert.strictEqual(normalizePath(undefined), '');
  });

  it('correctly detects absolute paths', () => {
    assert.strictEqual(isAbsolutePath('/home/ds/projects/notes/doc.md'), true);
    assert.strictEqual(isAbsolutePath('c:/users/projects/doc.md'), true);
    assert.strictEqual(isAbsolutePath('//wsl$/ubuntu/doc.md'), true);
    assert.strictEqual(isAbsolutePath('notes/06_studies/doc.md'), false);
    assert.strictEqual(isAbsolutePath('doc.md'), false);
  });

  it('matches identical paths exactly', () => {
    assert.strictEqual(
      isSameFilePath(
        'notes/06_studies/cmaas_linkedin_growth/assets/report_nb.md',
        'notes/06_studies/cmaas_linkedin_growth/assets/report_nb.md'
      ),
      true
    );
    assert.strictEqual(isSameFilePath('AGENTS.md', 'agents.md'), true);
    assert.strictEqual(isSameFilePath('./AGENTS.md', 'AGENTS.md'), true);
    assert.strictEqual(isSameFilePath('notes\\01_projects\\core.md', 'notes/01_projects/core.md'), true);
  });

  it('STRICTLY PREVENTS matching files with identical basenames across different directories', () => {
    // 21 study directories have assets/report_nb.md - none should cross-match
    const studyA = 'notes/06_studies/cmaas_linkedin_growth/assets/report_nb.md';
    const studyB = 'notes/06_studies/cmaas_youtube_walkthroughs/assets/report_nb.md';
    const studyC = 'notes/06_studies/vscode_extensions/assets/report_nb.md';
    const rootFile = 'report_nb.md';

    assert.strictEqual(isSameFilePath(studyA, studyB), false);
    assert.strictEqual(isSameFilePath(studyA, studyC), false);
    assert.strictEqual(isSameFilePath(studyB, studyC), false);
    assert.strictEqual(isSameFilePath(studyA, rootFile), false);
    assert.strictEqual(isSameFilePath(studyB, rootFile), false);
    assert.strictEqual(isSameFilePath(studyC, rootFile), false);
  });

  it('prevents substring suffix collisions without directory boundaries', () => {
    assert.strictEqual(isSameFilePath('my_notes/report_nb.md', 'notes/report_nb.md'), false);
    assert.strictEqual(isSameFilePath('other_assets/report_nb.md', 'assets/report_nb.md'), false);
  });

  it('matches absolute path against relative workspace path with exact boundary', () => {
    const absPath = '/home/ds/projects/notes/06_studies/cmaas_linkedin_growth/assets/report_nb.md';
    const relPath = 'notes/06_studies/cmaas_linkedin_growth/assets/report_nb.md';
    const wrongRel = 'notes/06_studies/other_study/assets/report_nb.md';

    assert.strictEqual(isSameFilePath(absPath, relPath), true);
    assert.strictEqual(isSameFilePath(relPath, absPath), true);
    assert.strictEqual(isSameFilePath(absPath, wrongRel), false);
  });

  it('handles empty, null, or undefined inputs gracefully', () => {
    assert.strictEqual(isSameFilePath(null, 'notes/doc.md'), false);
    assert.strictEqual(isSameFilePath('notes/doc.md', undefined), false);
    assert.strictEqual(isSameFilePath('', ''), false);
  });
});
