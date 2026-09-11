//! Relative path resolver, multi-repo/submodule traversal, and dead link & anchor detector.

use crate::model::{LinkResolutionStatus, LinkType, NoteEdge};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Precomputed lookup index for O(1) canonical, case-folded, and basename resolution.
pub struct ResolverIndex<'a> {
    pub known_nodes: &'a HashMap<String, PathBuf>,
    pub case_folded: HashMap<String, String>,
    pub basename_map: HashMap<String, String>,
    pub excluded_nodes: HashSet<String>,
}

impl<'a> ResolverIndex<'a> {
    pub fn new(known_nodes: &'a HashMap<String, PathBuf>) -> Self {
        Self::with_exclusions(known_nodes, HashSet::new())
    }

    pub fn with_exclusions(
        known_nodes: &'a HashMap<String, PathBuf>,
        excluded_nodes: HashSet<String>,
    ) -> Self {
        let mut case_folded = HashMap::with_capacity(known_nodes.len());
        let mut basename_map = HashMap::with_capacity(known_nodes.len());

        for k in known_nodes.keys() {
            case_folded.insert(k.to_lowercase(), k.clone());
            if let Some(file_name) = Path::new(k).file_name() {
                let name = file_name.to_string_lossy().to_lowercase();
                basename_map.entry(name).or_insert_with(|| k.clone());
            }
        }

        Self {
            known_nodes,
            case_folded,
            basename_map,
            excluded_nodes,
        }
    }
}

/// Normalizes a relative path from the source file directory to a canonical vault POSIX path.
pub fn resolve_relative_path(source_id: &str, target_path: &str) -> String {
    let source_path = Path::new(source_id);
    let parent = source_path.parent().unwrap_or_else(|| Path::new(""));
    let combined = parent.join(target_path);

    // Normalize components (e.g. eliminating "." and "..")
    let mut normalized = Vec::new();
    for component in combined.components() {
        match component {
            std::path::Component::Normal(c) => normalized.push(c.to_string_lossy().to_string()),
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {}
            _ => {}
        }
    }

    normalized.join("/")
}

/// Slugifies a heading string for anchor matching (e.g. "System Architecture" -> "system-architecture").
pub fn slugify_heading(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Resolves edges against registered nodes and validates section heading anchors.
/// Supports multi-repo cross-submodule traversal, case-insensitive normalization,
/// and Obsidian shortest-path vault lookups with O(1) indexed lookups.
pub fn resolve_edge_integrity(
    edge: &mut NoteEdge,
    index: &ResolverIndex,
    node_headings: &HashMap<String, Vec<String>>,
) {
    let raw_target = &edge.target;
    let rel_target = resolve_relative_path(&edge.source, raw_target);

    // Candidate search strategy with O(1) indexing:
    // 1. Direct relative match
    // 2. Direct absolute/root match
    // 3. Case-insensitive relative match
    // 4. Case-insensitive root match
    // 5. Obsidian shortest-path match (filename or suffix match)
    let matched_target = if index.known_nodes.contains_key(&rel_target) {
        Some(rel_target.clone())
    } else if index.known_nodes.contains_key(raw_target) {
        Some(raw_target.clone())
    } else if let Some(k) = index.case_folded.get(&rel_target.to_lowercase()) {
        Some(k.clone())
    } else if let Some(k) = index.case_folded.get(&raw_target.to_lowercase()) {
        Some(k.clone())
    } else {
        // Vault shortest-path or case-folded filename lookup
        let target_stem = Path::new(raw_target)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| raw_target.clone());

        let target_stem_md = if !target_stem.ends_with(".md") {
            format!("{}.md", target_stem)
        } else {
            target_stem.clone()
        };

        index
            .basename_map
            .get(&target_stem.to_lowercase())
            .or_else(|| index.basename_map.get(&target_stem_md.to_lowercase()))
            .cloned()
    };

    let canonical_target = match matched_target {
        Some(found) => found,
        None => {
            edge.target = rel_target.clone();
            edge.id = format!("{}->{}", edge.source, edge.target);
            edge.is_broken = true;
            edge.resolution_status = Some(if edge.link_type == LinkType::Wikilink {
                LinkResolutionStatus::BrokenWikilink
            } else {
                LinkResolutionStatus::BrokenMissingFile
            });
            return;
        }
    };

    edge.target = canonical_target.clone();
    edge.id = format!("{}->{}", edge.source, edge.target);

    // Validate section heading anchor if present
    if let Some(ref anchor) = edge.anchor {
        let anchor_slug = slugify_heading(anchor);
        let mut anchor_found = false;

        if let Some(headings) = node_headings.get(&canonical_target) {
            for h in headings {
                if slugify_heading(h) == anchor_slug || h.trim().eq_ignore_ascii_case(anchor.trim())
                {
                    anchor_found = true;
                    break;
                }
            }
        }

        if !anchor_found {
            edge.is_broken = true;
            edge.resolution_status = Some(LinkResolutionStatus::BrokenMissingAnchor);
            return;
        }
    }

    edge.is_broken = false;
    if index.excluded_nodes.contains(&canonical_target) {
        edge.resolution_status = Some(LinkResolutionStatus::ResolvedExcluded);
    } else {
        edge.resolution_status = Some(LinkResolutionStatus::Resolved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::LinkType;

    #[test]
    fn test_target_resolvability_invariant_with_excluded_target() {
        let mut known_nodes = HashMap::new();
        known_nodes.insert(
            "docs/index.md".to_string(),
            PathBuf::from("/vault/docs/index.md"),
        );
        known_nodes.insert(
            "packages/lib/README.md".to_string(),
            PathBuf::from("/vault/packages/lib/README.md"),
        );

        let mut excluded_nodes = HashSet::new();
        excluded_nodes.insert("packages/lib/README.md".to_string());

        let index = ResolverIndex::with_exclusions(&known_nodes, excluded_nodes);
        let headings = HashMap::new();

        let mut edge = NoteEdge::new(
            "docs/index.md",
            "../packages/lib/README.md",
            LinkType::Markdown,
            "Lib Readme",
            None,
            Some(10),
        );

        resolve_edge_integrity(&mut edge, &index, &headings);

        assert_eq!(edge.target, "packages/lib/README.md");
        assert!(!edge.is_broken, "Excluded target must not be marked broken");
        assert_eq!(
            edge.resolution_status,
            Some(LinkResolutionStatus::ResolvedExcluded)
        );
    }
}
