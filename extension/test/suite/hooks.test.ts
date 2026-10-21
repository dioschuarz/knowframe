import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  generatePreCommitConfigContent,
  generateFallbackPostCommitScript,
} from '../../src/hooks/prekInstaller.ts';

suite('Git Hooks & prek Integration Test Suite', () => {
  test('Pre-commit config generates valid YAML with regex markdown filter', () => {
    const yaml = generatePreCommitConfigContent();
    assert.ok(yaml.includes("files: '(?i)\\.md$'"), 'Must include case-insensitive markdown regex filter');
    assert.ok(yaml.includes('entry: knowframe-core git-delta'), 'Must invoke knowframe-core git-delta CLI');
    assert.ok(yaml.includes('id: llm-wiki-sync'), 'Hook id must be llm-wiki-sync');
  });

  test('Fallback post-commit shell script contains fail-open non-blocking invocation', () => {
    const script = generateFallbackPostCommitScript();
    assert.ok(script.startsWith('#!/bin/sh'), 'Must be a valid POSIX shell script');
    assert.ok(script.includes('knowframe-core git-delta'), 'Must invoke knowframe-core git-delta');
    assert.ok(script.includes('exit 0'), 'Must always exit 0 to guarantee fail-open behavior');
  });
});
