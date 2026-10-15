//! Secondary routing layer detector.
//!
//! Enforces FR-013 and SC-003 by identifying and flagging intermediate routing hops
//! (e.g. `docs/index.md`, `memory.md`, `hot-index.md`) as structural defects.

use super::SecondaryRoutingLayer;
use crate::graph::resolver::resolve_relative_path;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SecondaryLayerDetector {
    workspace_root: PathBuf,
}

impl SecondaryLayerDetector {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            workspace_root: workspace_root.to_path_buf(),
        }
    }

    /// Detects secondary routing layers linked from the given harness entry point.
    pub fn detect_secondary_layers(&self, harness_rel_path: &str) -> Vec<SecondaryRoutingLayer> {
        let full_path = self.workspace_root.join(harness_rel_path);
        let content = fs::read_to_string(&full_path).unwrap_or_default();

        let mut secondary_layers = Vec::new();

        // Extract markdown links from harness content
        let outgoing_links = extract_links_from_text(&content);

        for target in outgoing_links {
            let (target_file, _) = if let Some(hash) = target.find('#') {
                (&target[..hash], Some(&target[hash + 1..]))
            } else {
                (target.as_str(), None)
            };

            let rel_target = resolve_relative_path(harness_rel_path, target_file);
            let target_path = self.workspace_root.join(&rel_target);

            if !target_path.is_file() {
                continue;
            }

            let file_name = target_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            let is_known_secondary = file_name.eq_ignore_ascii_case("index.md")
                || file_name.eq_ignore_ascii_case("memory.md")
                || file_name.eq_ignore_ascii_case("hot-index.md")
                || file_name.eq_ignore_ascii_case("routing.md");

            if is_known_secondary {
                let target_content = fs::read_to_string(&target_path).unwrap_or_default();
                let target_outgoing = extract_links_from_text(&target_content);

                let penalty_points = 15.0;
                let description = format!(
                    "Secondary routing layer detected at '{}' introducing unnecessary hop distance.",
                    rel_target
                );
                let recommendation_hint = format!(
                    "Consolidate routes from '{}' directly into '{}' via EliminateSecondaryRoutingLayer.",
                    rel_target, harness_rel_path
                );

                secondary_layers.push(SecondaryRoutingLayer {
                    file_path: rel_target,
                    parent_harness: harness_rel_path.to_string(),
                    outgoing_routes_count: target_outgoing.len(),
                    intermediate_hops: 1,
                    penalty_points,
                    description,
                    recommendation_hint,
                });
            }
        }

        secondary_layers
    }
}

/// Helper to extract link targets from markdown text.
fn extract_links_from_text(content: &str) -> Vec<String> {
    let mut targets = Vec::new();

    // Standard markdown links: [label](target)
    let mut rest = content;
    while let Some(start) = rest.find("](") {
        let after_bracket = &rest[start + 2..];
        if let Some(end) = after_bracket.find(')') {
            let target = &after_bracket[..end].trim();
            if !target.starts_with("http://")
                && !target.starts_with("https://")
                && !target.starts_with('#')
            {
                targets.push(target.to_string());
            }
            rest = &after_bracket[end + 1..];
        } else {
            break;
        }
    }

    // Wikilinks: [[target|label]] or [[target]]
    let mut wiki_rest = content;
    while let Some(start) = wiki_rest.find("[[") {
        let after = &wiki_rest[start + 2..];
        if let Some(end) = after.find("]]") {
            let inner = &after[..end];
            let target = if let Some(pipe) = inner.find('|') {
                &inner[..pipe]
            } else {
                inner
            };
            let target = target.trim();
            if !target.is_empty() && !target.starts_with('#') {
                targets.push(target.to_string());
            }
            wiki_rest = &after[end + 2..];
        } else {
            break;
        }
    }

    targets
}
