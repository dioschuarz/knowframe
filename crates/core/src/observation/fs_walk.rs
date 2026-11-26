//! Symlink-aware workspace walker shared by every scanner.
//!
//! - Follows symlinked directories and files whose target resolves **inside** the workspace
//!   root (targets outside the root and dangling links are skipped).
//! - Symlink cycles are detected by `walkdir` and skipped.
//! - Files reachable through several logical paths (real path + one or more symlinks) are
//!   deduplicated by canonical path into a single [`WalkedFile`]. The primary path is the
//!   logical path that crosses the fewest symlink hops (the real path has zero; a catalog
//!   symlink beats a symlink to that catalog), ties broken lexicographically. Every other
//!   logical path becomes an alias so links written with any of them still resolve.
//! - Linked git worktrees (a directory whose `.git` is a file pointing into
//!   `<repo>/.git/worktrees/`) are duplicate checkouts and can be skipped.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// One unique file found by [`walk_workspace_files`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalkedFile {
    /// Absolute logical path of the primary entry (readable on disk).
    pub abs_path: PathBuf,
    /// Workspace-relative POSIX path used as the node id.
    pub rel_path: String,
    /// Other workspace-relative POSIX paths that reach the same canonical file.
    pub aliases: Vec<String>,
}

/// Walks `root` and returns the unique files accepted by `accept_rel`, sorted by `rel_path`.
///
/// * `follow_symlinks` — follow links that resolve inside `root`.
/// * `skip_linked_worktrees` — skip linked git worktrees (duplicate checkouts).
/// * `prune_dir_name` — returns `true` for directory names whose whole subtree is skipped.
/// * `accept_rel` — returns `true` for workspace-relative POSIX file paths to keep.
pub fn walk_workspace_files<P, A>(
    root: &Path,
    follow_symlinks: bool,
    skip_linked_worktrees: bool,
    prune_dir_name: P,
    accept_rel: A,
) -> Vec<WalkedFile>
where
    P: Fn(&str) -> bool,
    A: Fn(&str) -> bool,
{
    let canonical_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    let walker = WalkDir::new(root)
        .follow_links(follow_symlinks)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.path_is_symlink() {
                if !follow_symlinks {
                    // Preserve legacy behavior: symlinked files are still listed by type below
                    // (walkdir reports them as symlinks, which are never `is_file`).
                    return true;
                }
                match e.path().canonicalize() {
                    Ok(target) if target.starts_with(&canonical_root) => {}
                    _ => return false,
                }
            }
            if !e.file_type().is_dir() {
                return true;
            }
            if prune_dir_name(&e.file_name().to_string_lossy()) {
                return false;
            }
            !(skip_linked_worktrees && is_linked_worktree(e.path()))
        });

    // canonical path -> (logical rel paths, abs path per rel path)
    let mut groups: BTreeMap<PathBuf, Vec<(String, PathBuf)>> = BTreeMap::new();

    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let abs_path = entry.path().to_path_buf();
        let Ok(rel) = abs_path.strip_prefix(root) else {
            continue;
        };
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        if !accept_rel(&rel_str) {
            continue;
        }
        let canonical = abs_path.canonicalize().unwrap_or_else(|_| abs_path.clone());
        groups
            .entry(canonical)
            .or_default()
            .push((rel_str, abs_path));
    }

    let mut files: Vec<WalkedFile> = groups
        .into_iter()
        .map(|(_canonical, mut logical)| {
            logical.sort_by_cached_key(|(rel, _)| (symlink_hops(root, rel), rel.clone()));
            let (rel_path, abs_path) = logical.remove(0);
            WalkedFile {
                abs_path,
                rel_path,
                aliases: logical.into_iter().map(|(rel, _)| rel).collect(),
            }
        })
        .collect();

    files.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    files
}

/// Number of symlink hops crossed by the workspace-relative `rel` path: every symlinked
/// prefix counts once per link in its chain (a symlink pointing at another symlink counts 2).
fn symlink_hops(root: &Path, rel: &str) -> usize {
    const MAX_CHAIN: usize = 40;
    let mut hops = 0;
    let mut prefix = root.to_path_buf();
    for segment in rel.split('/') {
        prefix.push(segment);
        let mut current = prefix.clone();
        for _ in 0..MAX_CHAIN {
            let Ok(target) = std::fs::read_link(&current) else {
                break;
            };
            hops += 1;
            current = match current.parent() {
                Some(parent) if target.is_relative() => parent.join(target),
                _ => target,
            };
        }
    }
    hops
}

/// True when `dir` is a linked git worktree: its `.git` is a file whose `gitdir:` points
/// into another repository's `.git/worktrees/` folder. Submodules (`.git/modules/`) and
/// regular nested repositories are not affected.
fn is_linked_worktree(dir: &Path) -> bool {
    let dot_git = dir.join(".git");
    if !dot_git.is_file() {
        return false;
    }
    std::fs::read_to_string(&dot_git)
        .ok()
        .and_then(|content| {
            content.lines().find_map(|l| {
                l.strip_prefix("gitdir:")
                    .map(|v| v.trim().replace('\\', "/"))
            })
        })
        .is_some_and(|gitdir| gitdir.contains("/.git/worktrees/"))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::tempdir;

    fn md_only(rel: &str) -> bool {
        rel.ends_with(".md")
    }

    #[test]
    fn follows_directory_symlink_inside_root() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("repo/skills/radar")).unwrap();
        fs::write(root.join("repo/skills/radar/SKILL.md"), "# Radar\n").unwrap();
        fs::create_dir_all(root.join("catalog")).unwrap();
        symlink("../repo/skills/radar", root.join("catalog/radar")).unwrap();

        let files = walk_workspace_files(root, true, true, |_| false, md_only);
        assert_eq!(files.len(), 1, "same canonical file must be deduplicated");
        assert_eq!(files[0].rel_path, "repo/skills/radar/SKILL.md");
        assert_eq!(files[0].aliases, vec!["catalog/radar/SKILL.md".to_string()]);
    }

    #[test]
    fn symlink_path_is_primary_when_real_path_is_rejected() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("repo/skills/radar")).unwrap();
        fs::write(root.join("repo/skills/radar/SKILL.md"), "# Radar\n").unwrap();
        fs::create_dir_all(root.join("catalog")).unwrap();
        symlink("../repo/skills/radar", root.join("catalog/radar")).unwrap();

        let files = walk_workspace_files(
            root,
            true,
            true,
            |_| false,
            |rel| md_only(rel) && !rel.starts_with("repo/"),
        );
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].rel_path, "catalog/radar/SKILL.md");
        assert!(files[0].aliases.is_empty());
    }

    #[test]
    fn primary_is_path_with_fewest_symlink_hops() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("repo/skills/radar")).unwrap();
        fs::write(root.join("repo/skills/radar/SKILL.md"), "# Radar\n").unwrap();
        fs::create_dir_all(root.join("notes/skills")).unwrap();
        symlink("../../repo/skills/radar", root.join("notes/skills/radar")).unwrap();
        // `.agents/skills` -> `notes/skills` (a symlink to the catalog: 2 hops to the file).
        fs::create_dir_all(root.join(".agents")).unwrap();
        symlink("../notes/skills", root.join(".agents/skills")).unwrap();

        let files = walk_workspace_files(
            root,
            true,
            true,
            |_| false,
            |rel| md_only(rel) && !rel.starts_with("repo/"),
        );
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].rel_path, "notes/skills/radar/SKILL.md");
        assert_eq!(
            files[0].aliases,
            vec![".agents/skills/radar/SKILL.md".to_string()]
        );
    }

    #[test]
    fn skips_linked_git_worktrees_but_not_submodules() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("repo/.claude/worktrees/feature")).unwrap();
        fs::write(
            root.join("repo/.claude/worktrees/feature/.git"),
            "gitdir: /abs/repo/.git/worktrees/feature\n",
        )
        .unwrap();
        fs::write(
            root.join("repo/.claude/worktrees/feature/README.md"),
            "# Copy\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("repo/vendor/sub")).unwrap();
        fs::write(
            root.join("repo/vendor/sub/.git"),
            "gitdir: ../../.git/modules/sub\n",
        )
        .unwrap();
        fs::write(root.join("repo/vendor/sub/README.md"), "# Sub\n").unwrap();

        let rels = |skip: bool| -> Vec<String> {
            walk_workspace_files(root, true, skip, |_| false, md_only)
                .into_iter()
                .map(|f| f.rel_path)
                .collect()
        };
        assert_eq!(rels(true), vec!["repo/vendor/sub/README.md".to_string()]);
        assert_eq!(rels(false).len(), 2);
    }

    #[test]
    fn symlink_cycle_terminates() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("a")).unwrap();
        fs::write(root.join("a/note.md"), "# Note\n").unwrap();
        symlink("..", root.join("a/loop")).unwrap();

        let files = walk_workspace_files(root, true, true, |_| false, md_only);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].rel_path, "a/note.md");
    }

    #[test]
    fn skips_symlinks_escaping_root_and_dangling_links() {
        let outside = tempdir().unwrap();
        fs::write(outside.path().join("secret.md"), "# Outside\n").unwrap();
        let dir = tempdir().unwrap();
        let root = dir.path();
        symlink(outside.path(), root.join("escape")).unwrap();
        symlink("missing", root.join("dangling.md")).unwrap();
        fs::write(root.join("inside.md"), "# Inside\n").unwrap();

        let files = walk_workspace_files(root, true, true, |_| false, md_only);
        let rels: Vec<_> = files.iter().map(|f| f.rel_path.as_str()).collect();
        assert_eq!(rels, vec!["inside.md"]);
    }

    #[test]
    fn follow_disabled_keeps_legacy_behavior() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("repo")).unwrap();
        fs::write(root.join("repo/SKILL.md"), "# Skill\n").unwrap();
        symlink("repo", root.join("catalog")).unwrap();

        let files = walk_workspace_files(root, false, true, |_| false, md_only);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].rel_path, "repo/SKILL.md");
        assert!(files[0].aliases.is_empty());
    }
}
