import * as fs from 'fs';
import * as path from 'path';
import type { ScanResultPayload } from '../types/protocol.ts';

export interface ParsedWikilink {
  startOffset: number;
  endOffset: number;
  rawInner: string;
  rawTarget: string;
  anchor: string | null;
  label: string;
  isEmbed: boolean;
}

export interface ResolvedMarkdownLink {
  canonicalId: string;
  absolutePath: string;
  relativePosixPath: string;
  fragmentSlug: string | null;
  targetLine: number | null;
}

/**
 * Matches KnowFrame Rust core `slugify_heading` for deterministic fragment resolution.
 */
export function slugifyHeading(heading: string): string {
  let slug = '';
  let prevIsHyphen = false;

  for (const ch of heading) {
    if (/[a-zA-Z0-9]/.test(ch)) {
      slug += ch.toLowerCase();
      prevIsHyphen = false;
    } else if ((/\s/.test(ch) || ch === '-' || ch === '_') && !prevIsHyphen && slug.length > 0) {
      slug += '-';
      prevIsHyphen = true;
    }
  }

  slug = slug.replace(/-+$/, '');
  return slug.length > 0 ? slug : 'section';
}

/**
 * Computes GitHub/VS Code GFM heading slug (preserving unicode letters, stripping punctuation).
 */
export function gfmSlugifyHeading(heading: string): string {
  return heading
    .trim()
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s_-]+/gu, '')
    .replace(/\s+/g, '-')
    .replace(/^-+|-+$/g, '');
}

/**
 * Masks YAML frontmatter, fenced code blocks, and inline code spans with spaces
 * (preserving exact character offsets and newlines).
 */
export function maskProtectedMarkdown(content: string): string {
  const chars = content.split('');

  // 1. Mask YAML frontmatter
  if (content.startsWith('---\n') || content.startsWith('---\r\n')) {
    const startIdx = content.startsWith('---\r\n') ? 5 : 4;
    const endMarker = content.indexOf('\n---', startIdx);
    if (endMarker !== -1) {
      let fmEnd = endMarker + 4;
      if (content.slice(fmEnd).startsWith('\r\n')) {
        fmEnd += 2;
      } else if (content.slice(fmEnd).startsWith('\n')) {
        fmEnd += 1;
      }
      for (let i = 0; i < Math.min(fmEnd, chars.length); i++) {
        if (chars[i] !== '\n' && chars[i] !== '\r') {
          chars[i] = ' ';
        }
      }
    }
  }

  // 2. Mask fenced code blocks (``` or ~~~)
  const lines = content.split('\n');
  let offset = 0;
  let fenceChar: string | null = null;
  let fenceLen = 0;

  for (let li = 0; li < lines.length; li++) {
    const line = lines[li];
    const trimmed = line.trimStart();
    if (!fenceChar) {
      const m = trimmed.match(/^(`{3,}|~{3,})/);
      if (m) {
        fenceChar = m[1][0];
        fenceLen = m[1].length;
        for (let i = offset; i < offset + line.length; i++) {
          if (chars[i] !== '\r') {
            chars[i] = ' ';
          }
        }
      }
    } else {
      for (let i = offset; i < offset + line.length; i++) {
        if (chars[i] !== '\r') {
          chars[i] = ' ';
        }
      }
      const closeRe = new RegExp(`^${fenceChar}{${fenceLen},}\\s*$`);
      if (closeRe.test(trimmed)) {
        fenceChar = null;
        fenceLen = 0;
      }
    }
    offset += line.length + 1;
  }

  // 3. Mask inline code spans (`...`)
  let i = 0;
  while (i < chars.length) {
    if (chars[i] === '`') {
      let ticks = 0;
      while (i + ticks < chars.length && chars[i + ticks] === '`') {
        ticks++;
      }
      const closeSeq = '`'.repeat(ticks);
      const rest = chars.slice(i + ticks).join('');
      const closeRel = rest.indexOf(closeSeq);
      if (closeRel !== -1) {
        const end = i + ticks + closeRel + ticks;
        for (let j = i; j < end; j++) {
          if (chars[j] !== '\n' && chars[j] !== '\r') {
            chars[j] = ' ';
          }
        }
        i = end;
        continue;
      }
      i += ticks;
      continue;
    }
    i++;
  }

  return chars.join('');
}

/**
 * Extracts all Obsidian wikilinks `[[target#fragment|label]]` outside frontmatter & code.
 */
export function extractWikilinksInMarkdown(content: string): ParsedWikilink[] {
  const masked = maskProtectedMarkdown(content);
  const re = /\[\[([^\[\]\r\n]+)\]\]/g;
  const results: ParsedWikilink[] = [];

  let match: RegExpExecArray | null;
  while ((match = re.exec(masked)) !== null) {
    const startOffset = match.index;
    const endOffset = startOffset + match[0].length;
    const isEmbed = startOffset > 0 && content[startOffset - 1] === '!';
    const rawInner = content.slice(startOffset + 2, endOffset - 2).trim();
    if (!rawInner) {
      continue;
    }

    let rawTargetPart = rawInner;
    let alias: string | null = null;
    const pipeIdx = rawInner.indexOf('|');
    if (pipeIdx !== -1) {
      rawTargetPart = rawInner.slice(0, pipeIdx).trim().replace(/\\+$/, '').trim();
      alias = rawInner.slice(pipeIdx + 1).trim();
    }

    let rawTarget = rawTargetPart;
    let anchor: string | null = null;
    const hashIdx = rawTargetPart.indexOf('#');
    if (hashIdx !== -1) {
      rawTarget = rawTargetPart.slice(0, hashIdx).trim();
      anchor = rawTargetPart.slice(hashIdx + 1).trim();
    }

    const label = alias !== null && alias.length > 0
      ? alias
      : (rawTarget.length > 0 ? rawTarget : (anchor || rawInner));

    results.push({
      startOffset,
      endOffset,
      rawInner,
      rawTarget,
      anchor,
      label,
      isEmbed,
    });
  }

  return results;
}

/**
 * Computes POSIX relative path from `sourceRelPosix` to `targetRelPosix`.
 */
export function computeRelativePosixPath(sourceRelPosix: string, targetRelPosix: string): string {
  const src = sourceRelPosix.replace(/\\/g, '/');
  const tgt = targetRelPosix.replace(/\\/g, '/');
  if (src === tgt) {
    return '';
  }
  const srcDir = path.posix.dirname(src);
  const rel = path.posix.relative(srcDir === '.' ? '' : srcDir, tgt);
  return rel.replace(/\\/g, '/');
}

/**
 * Scans a Markdown file on disk for headings and locates the exact 1-based line number & slug
 * matching `requestedAnchor`. Returns null if the anchor does not exist in the file.
 */
export function findHeadingInFile(
  absolutePath: string,
  requestedAnchor: string
): { line: number; slug: string } | null {
  const cleanAnchor = requestedAnchor.replace(/^#/, '').trim();
  if (!cleanAnchor) {
    return null;
  }
  const targetSlug = slugifyHeading(cleanAnchor);
  const targetGfmSlug = gfmSlugifyHeading(cleanAnchor);

  let text: string;
  try {
    text = fs.readFileSync(absolutePath, 'utf8');
  } catch {
    return null;
  }

  const lines = text.split(/\r?\n/);
  const seenCounts = new Map<string, number>();
  let inFence = false;

  for (let idx = 0; idx < lines.length; idx++) {
    const line = lines[idx];
    if (/^\s*(`{3,}|~{3,})/.test(line)) {
      inFence = !inFence;
      continue;
    }
    if (inFence) {
      continue;
    }
    const m = line.match(/^#{1,6}\s+(.+?)\s*#*$/);
    if (!m) {
      continue;
    }
    const headingText = m[1].trim();
    const baseSlug = slugifyHeading(headingText);
    const count = seenCounts.get(baseSlug) || 0;
    seenCounts.set(baseSlug, count + 1);
    const disambiguated = count === 0 ? baseSlug : `${baseSlug}-${count}`;
    const gfmSlug = gfmSlugifyHeading(headingText);

    if (
      baseSlug === targetSlug ||
      disambiguated === targetSlug ||
      gfmSlug === targetGfmSlug ||
      headingText.toLowerCase() === cleanAnchor.toLowerCase()
    ) {
      return { line: idx + 1, slug: disambiguated };
    }
  }

  return null;
}

/**
 * Resolves a wikilink target + optional anchor against the live KnowFrame scan graph
 * (with fast filesystem fallback when scan data is unavailable or stale).
 * Returns null if the target file or requested heading anchor does not exist.
 */
export function resolveWikilinkTarget(
  workspaceRoot: string,
  sourceAbsolutePath: string,
  rawTarget: string,
  anchor: string | null,
  scanResult: ScanResultPayload | null
): ResolvedMarkdownLink | null {
  const sourceRel = path.relative(workspaceRoot, sourceAbsolutePath).replace(/\\/g, '/');
  const cleanRaw = rawTarget.trim().replace(/\\/g, '/').replace(/%20/g, ' ');
  const hasAnchor = Boolean(anchor && anchor.trim().length > 0);

  // Same-file anchor `[[#heading]]`
  if (!cleanRaw) {
    if (!hasAnchor) {
      return null;
    }
    const foundHeading = findHeadingInFile(sourceAbsolutePath, anchor!);
    if (!foundHeading) {
      return null;
    }
    return {
      canonicalId: sourceRel,
      absolutePath: sourceAbsolutePath,
      relativePosixPath: '',
      fragmentSlug: foundHeading.slug,
      targetLine: foundHeading.line,
    };
  }

  // Build candidate canonical paths relative to workspace root
  const sourceDirRel = path.posix.dirname(sourceRel);
  const relFromSource = sourceDirRel === '.'
    ? path.posix.normalize(cleanRaw)
    : path.posix.normalize(path.posix.join(sourceDirRel, cleanRaw));

  const candidates: string[] = [relFromSource];
  const firstSeg = sourceRel.split('/')[0];
  if (firstSeg && !cleanRaw.startsWith(`${firstSeg}/`) && cleanRaw !== firstSeg) {
    candidates.push(path.posix.normalize(path.posix.join(firstSeg, cleanRaw)));
  }
  const rootCand = path.posix.normalize(cleanRaw);
  if (!candidates.includes(rootCand)) {
    candidates.push(rootCand);
  }

  let matchedCanonical: string | null = null;
  let matchedAbsPath: string | null = null;
  let matchedFragments: { anchor: string; heading: string; start_line: number }[] | null = null;

  // 1. Lookup against live KnowFrame scan graph (O(1) map + basename fallback)
  if (scanResult && scanResult.nodes.length > 0) {
    const byId = new Map<string, (typeof scanResult.nodes)[number]>();
    const byLower = new Map<string, (typeof scanResult.nodes)[number]>();
    const byBasename = new Map<string, (typeof scanResult.nodes)[number][]>();

    for (const node of scanResult.nodes) {
      const id = node.id.replace(/\\/g, '/');
      byId.set(id, node);
      byLower.set(id.toLowerCase(), node);
      const base = path.posix.basename(id).toLowerCase();
      const stem = base.endsWith('.md') ? base.slice(0, -3) : base;
      const arr1 = byBasename.get(base) || [];
      arr1.push(node);
      byBasename.set(base, arr1);
      if (stem !== base) {
        const arr2 = byBasename.get(stem) || [];
        arr2.push(node);
        byBasename.set(stem, arr2);
      }
    }

    let selfFallback: (typeof scanResult.nodes)[number] | null = null;
    let foundNode: (typeof scanResult.nodes)[number] | null = null;

    // Step A: Direct exact match
    for (const cand of candidates) {
      const hit = cand.endsWith('.md')
        ? byId.get(cand)
        : (byId.get(`${cand}.md`) || byId.get(cand));
      if (hit) {
        if (!hasAnchor && !cleanRaw.includes('/') && hit.id.replace(/\\/g, '/') === sourceRel) {
          if (!selfFallback) {
            selfFallback = hit;
          }
          continue;
        }
        foundNode = hit;
        break;
      }
    }
    if (!foundNode && selfFallback) {
      foundNode = selfFallback;
    }

    // Step B: Case-folded match
    if (!foundNode) {
      for (const cand of candidates) {
        const low = cand.toLowerCase();
        const hit = low.endsWith('.md')
          ? byLower.get(low)
          : (byLower.get(`${low}.md`) || byLower.get(low));
        if (hit) {
          foundNode = hit;
          break;
        }
      }
    }

    // Step C: Basename shortest-path match
    if (!foundNode) {
      const baseLower = path.posix.basename(cleanRaw).toLowerCase();
      const cands = byBasename.get(baseLower.endsWith('.md') ? baseLower : `${baseLower}.md`)
        || byBasename.get(baseLower);
      if (cands && cands.length > 0) {
        const nonSelf = !hasAnchor ? cands.filter((n) => n.id.replace(/\\/g, '/') !== sourceRel) : cands;
        const pool = nonSelf.length > 0 ? nonSelf : cands;
        pool.sort((a, b) => a.id.split('/').length - b.id.split('/').length || a.id.localeCompare(b.id));
        foundNode = pool[0];
      }
    }

    if (foundNode) {
      matchedCanonical = foundNode.id.replace(/\\/g, '/');
      matchedAbsPath = foundNode.file_path || path.join(workspaceRoot, matchedCanonical);
      matchedFragments = foundNode.fragments || null;
    }
  }

  // 2. Filesystem fallback (works immediately even before daemon scan finishes)
  if (!matchedAbsPath) {
    let selfFallbackCand: { canonical: string; abs: string } | null = null;
    for (const cand of candidates) {
      const variants = cand.endsWith('.md') ? [cand] : [`${cand}.md`, cand];
      for (const v of variants) {
        const abs = path.resolve(workspaceRoot, v);
        if (fs.existsSync(abs) && fs.statSync(abs).isFile()) {
          const canonical = path.relative(workspaceRoot, abs).replace(/\\/g, '/');
          if (!hasAnchor && !cleanRaw.includes('/') && canonical === sourceRel) {
            if (!selfFallbackCand) {
              selfFallbackCand = { canonical, abs };
            }
            continue;
          }
          matchedCanonical = canonical;
          matchedAbsPath = abs;
          break;
        }
      }
      if (matchedAbsPath) {
        break;
      }
    }
    if (!matchedAbsPath && selfFallbackCand) {
      matchedCanonical = selfFallbackCand.canonical;
      matchedAbsPath = selfFallbackCand.abs;
    }
  }

  if (!matchedCanonical || !matchedAbsPath || !fs.existsSync(matchedAbsPath)) {
    return null;
  }

  // 3. Validate anchor/fragment if requested — ONLY return clickable link if fragment exists!
  let fragmentSlug: string | null = null;
  let targetLine: number | null = null;

  if (hasAnchor) {
    const cleanAnchor = anchor!.replace(/^#/, '').trim();
    const targetSlug = slugifyHeading(cleanAnchor);

    if (matchedFragments && matchedFragments.length > 0) {
      const frag = matchedFragments.find(
        (f) =>
          f.anchor === targetSlug ||
          slugifyHeading(f.heading) === targetSlug ||
          f.heading.trim().toLowerCase() === cleanAnchor.toLowerCase()
      );
      if (frag) {
        fragmentSlug = frag.anchor;
        targetLine = frag.start_line;
      }
    }

    if (!fragmentSlug) {
      const foundHeading = findHeadingInFile(matchedAbsPath, cleanAnchor);
      if (!foundHeading) {
        // Fragment does NOT exist in target document -> NOT a valid clickable link
        return null;
      }
      fragmentSlug = foundHeading.slug;
      targetLine = foundHeading.line;
    }
  }

  const relativePosixPath = computeRelativePosixPath(sourceRel, matchedCanonical).replace(/ /g, '%20');

  return {
    canonicalId: matchedCanonical,
    absolutePath: matchedAbsPath,
    relativePosixPath,
    fragmentSlug,
    targetLine,
  };
}
