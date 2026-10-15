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
use walkdir::WalkDir;

/// Resolves every outgoing reference of every node against the scanned document set,
/// using the same resolver as the graph engine (relative path, vault-root, case-folded
/// and Obsidian shortest-path basename lookup, plus heading-anchor validation).
/// Same-document anchor links (`#heading`, `[[#heading]]`) resolve against the source note.
fn resolve_document_references(nodes: &mut [DocumentNode]) {
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
    let index = ResolverIndex::new(&known_nodes);

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
    pub excluded_patterns: Vec<String>,
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
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let is_excluded_name = |name: &str| options.excluded_patterns.iter().any(|p| p == name);

    let walker = WalkDir::new(root).follow_links(false).into_iter().filter_entry(|e| {
        // Never prune the root itself; prune excluded directory components below it.
        e.depth() == 0 || !(e.file_type().is_dir() && is_excluded_name(&e.file_name().to_string_lossy()))
    });

    for entry in walker.filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        file_paths.push(path.to_path_buf());
    }

    // Canonical sorting by relative POSIX path for strict determinism
    file_paths.sort_by(|a, b| {
        let rel_a = a.strip_prefix(root).unwrap_or(a).to_string_lossy().replace('\\', "/");
        let rel_b = b.strip_prefix(root).unwrap_or(b).to_string_lossy().replace('\\', "/");
        rel_a.cmp(&rel_b)
    });

    let mut nodes = Vec::with_capacity(file_paths.len());
    let mut total_tokens = 0u64;

    for path in &file_paths {
        let rel_path = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

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

    resolve_document_references(&mut nodes);
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
}
