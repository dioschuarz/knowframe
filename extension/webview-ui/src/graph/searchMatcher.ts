import type { NoteNode } from '../../../src/types/graph.ts';

export interface SearchMatchOptions {
  exact?: boolean;
}

export function matchesQuery(node: NoteNode, query: string, options: SearchMatchOptions = {}): boolean {
  const clean = query.trim();
  if (!clean) return false;

  const exact = !!options.exact;
  const q = exact ? clean : clean.toLowerCase();

  const label = exact ? node.label : node.label.toLowerCase();
  const id = exact ? node.id : node.id.toLowerCase();
  const filePath = exact ? (node.filePath || '') : (node.filePath || '').toLowerCase();
  const tags = node.tags || [];

  if (exact) {
    // 1. Exact title or ID or path
    if (label === q || id === q || filePath === q) return true;

    // 2. Exact filename without .md
    const filenameStem = node.id.split('/').pop()?.replace(/\.md$/, '') || '';
    if (filenameStem === q) return true;

    // 3. Exact folder match (any directory segment)
    const parts = (node.filePath || node.id).split('/');
    if (parts.slice(0, -1).includes(q)) return true;

    // 4. Exact tag match
    if (tags.some((t) => t === q || t.replace(/^#/, '') === q)) return true;

    return false;
  }

  // Substring containing match
  if (label.includes(q)) return true;
  if (id.includes(q)) return true;
  if (filePath.includes(q)) return true;
  if (tags.some((t) => t.toLowerCase().includes(q) || t.toLowerCase().replace(/^#/, '').includes(q))) return true;

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
