import type { NoteNode } from '../../../src/types/graph.ts';

export type SearchMode = 'contains' | 'keywords' | 'exact';

export interface SearchMatchOptions {
  mode?: SearchMode;
  caseSensitive?: boolean;
}

export function matchesQuery(node: NoteNode, query: string, options: SearchMatchOptions = {}): boolean {
  const clean = query.trim();
  if (!clean) return false;

  const mode = options.mode || 'contains';
  const caseSensitive = !!options.caseSensitive;

  const q = caseSensitive ? clean : clean.toLowerCase();

  const label = caseSensitive ? node.label : node.label.toLowerCase();
  const id = caseSensitive ? node.id : node.id.toLowerCase();
  const filePath = caseSensitive ? (node.filePath || '') : (node.filePath || '').toLowerCase();
  const tags = (node.tags || []).map((t) => (caseSensitive ? t : t.toLowerCase()));
  const filenameStem = id.split('/').pop()?.replace(/\.md$/, '') || '';

  if (mode === 'exact') {
    // 1. Exact title or ID or file path
    if (label === q || id === q || filePath === q) return true;

    // 2. Exact filename without .md
    if (filenameStem === q) return true;

    // 3. Exact folder match (any directory segment)
    const pathParts = (node.filePath || node.id).split('/');
    const dirSegments = pathParts.slice(0, -1).map((s) => (caseSensitive ? s : s.toLowerCase()));
    if (dirSegments.includes(q)) return true;

    // 4. Exact tag match (with or without #)
    if (tags.some((t) => t === q || t.replace(/^#/, '') === q)) return true;

    return false;
  }

  if (mode === 'keywords') {
    const rawTokens = q.split(/\s+/).filter(Boolean);
    if (rawTokens.length === 0) return false;

    // Build complete searchable corpus for the note
    const pathParts = (node.filePath || node.id).split('/');
    const dirSegments = pathParts.slice(0, -1).map((s) => (caseSensitive ? s : s.toLowerCase()));
    const strippedTags = tags.map((t) => t.replace(/^#/, ''));
    const corpus = [label, id, filePath, filenameStem, ...dirSegments, ...tags, ...strippedTags].join(' ');

    // All keywords separated must be present in the node corpus
    return rawTokens.every((token) => corpus.includes(token));
  }

  // mode === 'contains' (Default)
  if (label.includes(q)) return true;
  if (id.includes(q)) return true;
  if (filePath.includes(q)) return true;
  if (tags.some((t) => t.includes(q) || t.replace(/^#/, '').includes(q))) return true;

  return false;
}

export function filterNodesByQuery(
  nodes: NoteNode[],
  query: string,
  options: SearchMatchOptions = {},
): Set<string> | null {
  const clean = query.trim();
  if (!clean) return null;

  const matched = new Set<string>();
  for (const node of nodes) {
    if (matchesQuery(node, clean, options)) {
      matched.add(node.id);
    }
  }
  return matched;
}
