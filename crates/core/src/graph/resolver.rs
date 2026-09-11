//! Relative path resolver and dead link & anchor detector.

use crate::model::NoteEdge;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
pub fn resolve_edge_integrity(
    edge: &mut NoteEdge,
    known_nodes: &HashMap<String, PathBuf>,
    node_headings: &HashMap<String, Vec<String>>,
) {
    let canonical_target = resolve_relative_path(&edge.source, &edge.target);
    edge.target = canonical_target.clone();
    edge.id = format!("{}->{}", edge.source, edge.target);

    // 1. Validate file existence
    let target_exists = known_nodes.contains_key(&canonical_target);
    if !target_exists {
        edge.is_broken = true;
        return;
    }

    // 2. Validate section heading anchor if present
    if let Some(ref anchor) = edge.anchor {
        let anchor_slug = slugify_heading(anchor);
        let mut anchor_found = false;

        if let Some(headings) = node_headings.get(&canonical_target) {
            for h in headings {
                if slugify_heading(h) == anchor_slug || h.trim().eq_ignore_ascii_case(anchor.trim()) {
                    anchor_found = true;
                    break;
                }
            }
        }

        if !anchor_found {
            edge.is_broken = true;
        }
    }
}
