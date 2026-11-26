//! Repository filesystem scanner with canonical path sorting, UTF-8 normalization,
//! and excluded path filtering.
//! Implements FR-001 and FR-002.

use crate::ast::visitor::AstVisitor;
use crate::epistemic::analysis_id::{compute_analysis_id, METHODOLOGY_VERSION};
use crate::graph::resolver::{resolve_edge_integrity, ResolverIndex};
use crate::model::{DocumentNode, GraphTopology, LinkType, NoteEdge, OptimizationProfile, ReferenceEdge};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolves every outgoing reference of every node against the scanned document set,
/// using the same resolver as the graph engine (relative path, vault-root, case-folded
/// and Obsidian shortest-path basename lookup, plus heading-anchor validation).
/// Same-document anchor links (`#heading`, `[[#heading]]`) resolve against the source note.
fn resolve_document_references(nodes: &mut [DocumentNode], aliases: &HashMap<String, String>) {
    let known_nodes: HashMap<String, PathBuf> = nodes
        .iter()
        .map(|n| (n.relative_path.clone(), PathBuf::from(&n.relative_path)))
        .collect();
    let headings: HashMap<String, Vec<String>> = nodes
        .iter()
        .map(|n| {
            (
                n.relative_path.clone(),
                n.fragments.iter().map(|f| f.heading.clone()).collect(),
            )
        })
        .collect();
    let index = ResolverIndex::new(&known_nodes).with_aliases(aliases.clone());

    for node in nodes.iter_mut() {
        let source = node.relative_path.clone();
        for reference in node.outgoing_references.iter_mut() {
            let raw_target = reference.target_document.trim();
            let target = if raw_target.is_empty() { source.as_str() } else { raw_target };
            let link_type = if reference.relation_type.as_deref() == Some("wikilink") {
                LinkType::Wikilink
            } else {
                LinkType::Markdown
            };
            // Markdown links may be percent-encoded (e.g. `my%20note.md`).
            let decoded = target.replace("%20", " ");
            let mut edge = NoteEdge::new(
                source.as_str(),
                decoded,
                link_type,
                "",
                reference.target_fragment.clone(),
                Some(reference.line_number as u32),
            );
            resolve_edge_integrity(&mut edge, &index, &headings);
            reference.is_resolved = !edge.is_broken;
            if reference.is_resolved {
                reference.target_document = edge.target;
                reference.target_fragment = edge.target_fragment;
            }
        }
    }
}

/// Configuration options for repository scanner.
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Exclusions. A plain name (no `/`, `*`, `?`, `[` or `{`) prunes every directory with that
    /// exact name (legacy behavior); anything else is a glob matched against the
    /// workspace-relative POSIX path (e.g. `**/graphify-out/**`).
    pub excluded_patterns: Vec<String>,
    /// Follow symlinks that resolve inside the root (cycle-safe, deduplicated). Default `true`.
    pub follow_symlinks: bool,
    /// Skip linked git worktrees (duplicate checkouts). Default `true`.
    pub skip_linked_worktrees: bool,
}

fn is_glob_pattern(pattern: &str) -> bool {
    pattern.contains(['/', '*', '?', '[', '{'])
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            excluded_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                ".obsidian".to_string(),
            ],
            follow_symlinks: true,
            skip_linked_worktrees: true,
        }
    }
}

/// Result of scanning a repository.
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub analysis_id: String,
    pub topology: GraphTopology,
    pub total_documents: usize,
    pub total_tokens: u64,
}

/// Recursively scans a Markdown repository and constructs its canonical GraphTopology.
pub fn scan_repository(
    root: &Path,
    options: &ScanOptions,
    profile: &OptimizationProfile,
    commit_hash: &str,
) -> Result<ScanResult> {
    let plain_names: Vec<&str> = options
        .excluded_patterns
        .iter()
        .map(String::as_str)
        .filter(|p| !is_glob_pattern(p))
        .collect();
    let glob_patterns: Vec<String> = options
        .excluded_patterns
        .iter()
        .filter(|p| is_glob_pattern(p))
        .cloned()
        .collect();
    let exclude_globs = crate::cli::scan::build_globset(&glob_patterns)?;

    let walked = crate::observation::fs_walk::walk_workspace_files(
        root,
        options.follow_symlinks,
        options.skip_linked_worktrees,
        |name| plain_names.contains(&name),
        |rel| rel.ends_with(".md") && !exclude_globs.is_match(rel),
    );

    let mut aliases: HashMap<String, String> = HashMap::new();
    let mut file_paths: Vec<(PathBuf, String)> = Vec::with_capacity(walked.len());
    for file in walked {
        for alias in &file.aliases {
            aliases.insert(alias.clone(), file.rel_path.clone());
        }
        file_paths.push((file.abs_path, file.rel_path));
    }

    let mut nodes = Vec::with_capacity(file_paths.len());
    let mut total_tokens = 0u64;

    for (path, rel_path) in &file_paths {
        let rel_path = rel_path.clone();

        let content = std::fs::read_to_string(path).unwrap_or_default();
        let visited = AstVisitor::visit(&rel_path, &content);

        total_tokens += visited.total_tokens;

        let mtime = std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let node = DocumentNode {
            id: rel_path.clone(),
            relative_path: rel_path,
            title: visited.title,
            raw_token_count: visited.total_tokens,
            frontmatter_schema: None,
            fragments: visited.fragments,
            outgoing_references: visited.references,
            git_last_modified_commit: commit_hash.to_string(),
            git_last_modified_timestamp: mtime,
        };

        nodes.push(node);
    }

    resolve_document_references(&mut nodes, &aliases);
    let all_edges: Vec<ReferenceEdge> = nodes
        .iter()
        .flat_map(|n| n.outgoing_references.iter().cloned())
        .collect();

    let topology = GraphTopology {
        nodes,
        edges: all_edges,
    };

    let canonical_config_bytes = b"mode: default";
    let profile_bytes = serde_json::to_vec(profile).unwrap_or_default();
    let analysis_id = compute_analysis_id(
        commit_hash,
        canonical_config_bytes,
        &profile_bytes,
        METHODOLOGY_VERSION,
    );

    let total_documents = topology.nodes.len();

    Ok(ScanResult {
        analysis_id,
        topology,
        total_documents,
        total_tokens,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_scan_repository_canonical_sorting() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        std::fs::write(root.join("z_zebra.md"), "# Zebra\nContent zebra").unwrap();
        std::fs::write(root.join("a_apple.md"), "# Apple\nContent apple").unwrap();
        std::fs::write(root.join("m_monkey.md"), "# Monkey\nContent monkey").unwrap();

        let profile = OptimizationProfile::ai_oriented();
        let result = scan_repository(root, &ScanOptions::default(), &profile, "test_commit").unwrap();

        assert_eq!(result.total_documents, 3);
        assert_eq!(result.topology.nodes[0].relative_path, "a_apple.md");
        assert_eq!(result.topology.nodes[1].relative_path, "m_monkey.md");
        assert_eq!(result.topology.nodes[2].relative_path, "z_zebra.md");
        assert_eq!(result.analysis_id.len(), 64);
    }

    #[test]
    fn test_glob_and_plain_name_excludes() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("pkg/graphify-out")).unwrap();
        std::fs::create_dir_all(root.join("vendor")).unwrap();
        std::fs::write(root.join("pkg/graphify-out/GRAPH_REPORT.md"), "# Report").unwrap();
        std::fs::write(root.join("vendor/lib.md"), "# Vendor").unwrap();
        std::fs::write(root.join("keep.md"), "# Keep").unwrap();

        let options = ScanOptions {
            excluded_patterns: vec!["**/graphify-out/**".to_string(), "vendor".to_string()],
            ..ScanOptions::default()
        };
        let profile = OptimizationProfile::ai_oriented();
        let result = scan_repository(root, &options, &profile, "c").unwrap();
        let ids: Vec<_> = result.topology.nodes.iter().map(|n| n.relative_path.as_str()).collect();
        assert_eq!(ids, vec!["keep.md"]);
    }

    #[cfg(unix)]
    #[test]
    fn test_symlinked_directory_links_resolve() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("repo/skills/radar")).unwrap();
        std::fs::write(root.join("repo/skills/radar/SKILL.md"), "# Radar").unwrap();
        std::fs::create_dir_all(root.join("catalog")).unwrap();
        std::os::unix::fs::symlink("../repo/skills/radar", root.join("catalog/radar")).unwrap();
        std::fs::write(
            root.join("index.md"),
            "# Index\n[via catalog](catalog/radar/SKILL.md)\n[real](repo/skills/radar/SKILL.md)\n[gone](catalog/missing/SKILL.md)\n",
        )
        .unwrap();

        let profile = OptimizationProfile::ai_oriented();
        let result = scan_repository(root, &ScanOptions::default(), &profile, "c").unwrap();
        assert_eq!(result.total_documents, 2, "symlinked file deduplicated by canonical path");
        let index = result
            .topology
            .nodes
            .iter()
            .find(|n| n.relative_path == "index.md")
            .unwrap();
        let resolved: Vec<_> = index
            .outgoing_references
            .iter()
            .map(|r| (r.target_document.as_str(), r.is_resolved))
            .collect();
        assert_eq!(
            resolved,
            vec![
                ("repo/skills/radar/SKILL.md", true),
                ("repo/skills/radar/SKILL.md", true),
                ("catalog/missing/SKILL.md", false),
            ]
        );
    }
}
