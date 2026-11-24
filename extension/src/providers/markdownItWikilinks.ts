import type { ScanResultPayload } from '../types/protocol.ts';
import { resolveWikilinkTarget } from './markdownLinkResolver.ts';

export interface MarkdownItWikilinkContext {
  isEnabled: () => boolean;
  getWorkspaceRoot: (sourcePath?: string) => string | null;
  getFallbackDocumentPath: () => string | null;
  getScanResult: () => ScanResultPayload | null;
}

/**
 * Registers a markdown-it inline plugin that transforms valid Obsidian wikilinks
 * (`[[target]]`, `[[target|label]]`, `[[target#fragment|label]]`) into real clickable
 * `<a href="relative/path.md#slug">label</a>` hyperlinks inside VS Code's Markdown Preview.
 *
 * Only links whose target file and fragment anchor actually exist become clickable links.
 */
export function registerKnowFrameWikilinkPlugin(
  md: any,
  ctx: MarkdownItWikilinkContext
): any {
  if (!md || !md.inline || !md.inline.ruler) {
    return md;
  }

  md.inline.ruler.before('link', 'knowframe_wikilink', (state: any, silent: boolean) => {
    if (!ctx.isEnabled()) {
      return false;
    }

    const src: string = state.src;
    const pos: number = state.pos;
    const max: number = state.posMax;

    // Must start with `[[` and not be an embed `![[`
    if (pos + 4 >= max || src.charCodeAt(pos) !== 0x5b || src.charCodeAt(pos + 1) !== 0x5b) {
      return false;
    }
    if (pos > 0 && src.charCodeAt(pos - 1) === 0x21) {
      return false;
    }

    const closeIdx = src.indexOf(']]', pos + 2);
    if (closeIdx === -1 || closeIdx + 2 > max) {
      return false;
    }

    const rawInner = src.slice(pos + 2, closeIdx);
    if (!rawInner.trim() || rawInner.includes('\n') || rawInner.includes('\r') || rawInner.includes('[')) {
      return false;
    }

    // Parse target, optional #anchor, optional |label (supporting escaped `\|` in tables)
    let rawTargetPart = rawInner.trim();
    let alias: string | null = null;
    const pipeIdx = rawTargetPart.indexOf('|');
    if (pipeIdx !== -1) {
      alias = rawTargetPart.slice(pipeIdx + 1).trim();
      rawTargetPart = rawTargetPart.slice(0, pipeIdx).trim().replace(/\\+$/, '').trim();
    }

    let rawTarget = rawTargetPart;
    let anchor: string | null = null;
    const hashIdx = rawTargetPart.indexOf('#');
    if (hashIdx !== -1) {
      rawTarget = rawTargetPart.slice(0, hashIdx).trim();
      anchor = rawTargetPart.slice(hashIdx + 1).trim();
    }

    const label = alias && alias.length > 0
      ? alias
      : (rawTarget.length > 0 ? rawTarget : (anchor || rawInner.trim()));

    const envDoc = state.env?.currentDocument;
    const sourcePath: string | null =
      (typeof envDoc === 'string' ? envDoc : envDoc?.fsPath || envDoc?.path) ||
      ctx.getFallbackDocumentPath();

    if (!sourcePath) {
      return false;
    }

    const workspaceRoot = ctx.getWorkspaceRoot(sourcePath);
    if (!workspaceRoot) {
      return false;
    }

    const resolved = resolveWikilinkTarget(
      workspaceRoot,
      sourcePath,
      rawTarget,
      anchor,
      ctx.getScanResult()
    );

    // Only render as clickable <a> if target file AND fragment anchor genuinely exist
    if (!resolved) {
      return false;
    }

    if (!silent) {
      const href = resolved.fragmentSlug
        ? (resolved.relativePosixPath
            ? `${resolved.relativePosixPath}#${resolved.fragmentSlug}`
            : `#${resolved.fragmentSlug}`)
        : resolved.relativePosixPath;

      const title = resolved.fragmentSlug
        ? `${resolved.canonicalId}#${resolved.fragmentSlug}`
        : resolved.canonicalId;

      const openToken = state.push('link_open', 'a', 1);
      openToken.attrSet('href', href);
      openToken.attrSet('title', title);
      openToken.attrSet('class', 'knowframe-wikilink');

      const textToken = state.push('text', '', 0);
      textToken.content = label;

      state.push('link_close', 'a', -1);
    }

    state.pos = closeIdx + 2;
    return true;
  });

  return md;
}
