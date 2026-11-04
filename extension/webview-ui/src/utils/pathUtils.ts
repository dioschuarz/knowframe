/**
 * Path normalization and equality utilities for KnowFrame Webview UI.
 */

/**
 * Normalizes a file path for consistent cross-platform and relative/absolute comparisons.
 */
export function normalizePath(p?: string | null): string {
  if (!p) return '';
  return p
    .trim()
    .replace(/\\/g, '/')
    .replace(/^\.\//, '')
    .replace(/\/+$/, '')
    .toLowerCase();
}

/**
 * Checks whether a normalized path is absolute (POSIX root, Windows drive letter, or UNC).
 */
export function isAbsolutePath(p: string): boolean {
  return p.startsWith('/') || /^[a-z]:\//.test(p) || p.startsWith('//');
}

/**
 * Determines whether two paths refer to the exact same file.
 * Guarded against path-boundary and identical-basename collisions across different folders.
 *
 * Rules:
 * 1. If either path is empty, returns false.
 * 2. Exact match after normalization returns true.
 * 3. If one is absolute and one is relative, matches only on exact '/' path boundary suffix.
 * 4. If both are relative, allows suffix matching ONLY if the longer path is prefixed by
 *    at most one directory segment (e.g. workspace folder prefix like 'projects/notes/...').
 * 5. Files with identical basenames (e.g., 'report_nb.md' or '01_index.md') in different folders
 *    will NEVER match.
 */
export function isSameFilePath(pathA?: string | null, pathB?: string | null): boolean {
  if (!pathA || !pathB) return false;
  const normA = normalizePath(pathA);
  const normB = normalizePath(pathB);

  if (!normA || !normB) return false;
  if (normA === normB) return true;

  const isAbsA = isAbsolutePath(normA);
  const isAbsB = isAbsolutePath(normB);

  // If one is absolute and the other relative, require exact path-separator boundary match
  if (isAbsA && !isAbsB) {
    return normA.endsWith('/' + normB);
  }
  if (isAbsB && !isAbsA) {
    return normB.endsWith('/' + normA);
  }

  // If both are relative (or both are absolute), but one includes a workspace prefix:
  // e.g. "projects/notes/06_studies/cmaas/assets/report_nb.md" vs "notes/06_studies/cmaas/assets/report_nb.md"
  // The prefix must be exactly one leading directory segment (the workspace root folder).
  // This strictly prevents "assets/report_nb.md" from matching "study/assets/report_nb.md".
  if (!isAbsA && !isAbsB) {
    const longer = normA.length > normB.length ? normA : normB;
    const shorter = normA.length > normB.length ? normB : normA;
    if (longer.endsWith('/' + shorter)) {
      const prefix = longer.slice(0, longer.length - shorter.length - 1);
      // The prefix cannot contain slashes and must not be empty
      if (prefix.length > 0 && !prefix.includes('/')) {
        return true;
      }
    }
  }

  return false;
}
