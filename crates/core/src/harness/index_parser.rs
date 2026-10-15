//! Knowledge Index Markdown AST parser and route resolver.
//!
//! Parses `## Knowledge Index` from an AI Harness Entry Point (FR-012, FR-015),
//! extracts domain sections and route targets, and resolves links and anchors
//! deterministically via ResolverIndex and slugify_heading (FR-015, FR-016, FR-019).

use super::{EmbeddedIndexMetadata, KnowledgeIndexRoute, RouteResolutionStatus};
use crate::fragment::extractor::slugify_heading;
use crate::graph::resolver::resolve_relative_path;
use crate::tokens::counter::count_bpe_tokens;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct IndexParser {
    workspace_root: PathBuf,
}

impl IndexParser {
    pub fn new(workspace_root: &Path) -> Self {
        Self {
            workspace_root: workspace_root.to_path_buf(),
        }
    }

    /// Parses embedded knowledge index from harness markdown content.
    pub fn parse_harness_index(
        &self,
        harness_rel_path: &str,
        content: &str,
    ) -> Option<EmbeddedIndexMetadata> {
        let lines: Vec<&str> = content.lines().collect();

        // 1. Locate "## Knowledge Index" heading
        let mut index_start_line = None;
        let mut index_end_line = lines.len();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") {
                let heading_title = trimmed.trim_start_matches("## ").trim();
                let slug = slugify_heading(heading_title);
                if slug == "knowledge-index"
                    || slug.ends_with("-knowledge-index")
                    || heading_title.to_lowercase().contains("knowledge index")
                {
                    index_start_line = Some(idx + 1); // 1-based
                } else if index_start_line.is_some() {
                    // Next H2 heading closes the knowledge index section
                    index_end_line = idx;
                    break;
                }
            }
        }

        let start_line = index_start_line?;
        let end_line = index_end_line;

        // 2. Parse lines between start_line and end_line
        let mut current_domain = "General".to_string();
        let mut routes = Vec::new();
        let mut unique_domains = HashSet::new();

        for line_idx in (start_line - 1)..end_line {
            if line_idx >= lines.len() {
                break;
            }
            let line = lines[line_idx];
            let trimmed = line.trim();

            if trimmed.starts_with("### ") {
                current_domain = trimmed.trim_start_matches("### ").trim().to_string();
                unique_domains.insert(current_domain.clone());
                continue;
            }

            if trimmed.starts_with('-') || trimmed.starts_with('*') {
                if let Some(route) = self.parse_route_line(
                    harness_rel_path,
                    &current_domain,
                    trimmed,
                    line_idx + 1,
                ) {
                    routes.push(route);
                }
            }
        }

        // Domain coverage ratio: represented unique domains vs discovered vault domains
        let domain_coverage_ratio = if unique_domains.is_empty() {
            0.0
        } else {
            1.0 // Normalized fallback; can be enriched against vault structure
        };

        Some(EmbeddedIndexMetadata {
            heading_anchor: "knowledge-index".to_string(),
            start_line,
            end_line,
            routes,
            domain_coverage_ratio,
        })
    }

    fn parse_route_line(
        &self,
        source_rel_path: &str,
        domain: &str,
        line: &str,
        line_number: usize,
    ) -> Option<KnowledgeIndexRoute> {
        // Match Markdown link: [Label](Target)
        if let Some(start_bracket) = line.find('[') {
            if let Some(end_bracket) = line[start_bracket..].find(']') {
                let abs_end_bracket = start_bracket + end_bracket;
                let label = &line[start_bracket + 1..abs_end_bracket];

                if line[abs_end_bracket..].starts_with("](") {
                    let url_start = abs_end_bracket + 2;
                    if let Some(url_end) = line[url_start..].find(')') {
                        let full_target = &line[url_start..url_start + url_end];
                        return Some(self.build_route(
                            source_rel_path,
                            domain,
                            label,
                            full_target,
                            line_number,
                        ));
                    }
                }
            }
        }

        // Match Wikilink: [[Target#Anchor|Label]] or [[Target]]
        if let Some(start) = line.find("[[") {
            if let Some(end) = line[start..].find("]]") {
                let inner = &line[start + 2..start + end];
                let (target_part, label_part) = if let Some(pipe_idx) = inner.find('|') {
                    (&inner[..pipe_idx], &inner[pipe_idx + 1..])
                } else {
                    (inner, inner)
                };
                return Some(self.build_route(
                    source_rel_path,
                    domain,
                    label_part.trim(),
                    target_part.trim(),
                    line_number,
                ));
            }
        }

        None
    }

    fn build_route(
        &self,
        source_rel_path: &str,
        domain: &str,
        label: &str,
        raw_target: &str,
        line_number: usize,
    ) -> KnowledgeIndexRoute {
        let (file_part, anchor_part) = if let Some(hash_idx) = raw_target.find('#') {
            (
                &raw_target[..hash_idx],
                Some(raw_target[hash_idx + 1..].to_string()),
            )
        } else {
            (raw_target, None)
        };

        let resolved_file_rel = resolve_relative_path(source_rel_path, file_part);
        let target_full_path = self.workspace_root.join(&resolved_file_rel);

        let (resolution_status, target_tokens) = if !target_full_path.is_file() {
            (RouteResolutionStatus::BrokenFile, 0)
        } else {
            let file_content = fs::read_to_string(&target_full_path).unwrap_or_default();
            let total_file_tokens = count_bpe_tokens(&file_content).unwrap_or(0) as usize;

            if let Some(ref anchor) = anchor_part {
                // Verify anchor against file headings
                let normalized_anchor = slugify_heading(anchor);
                let heading_slugs = extract_heading_slugs(&file_content);
                if heading_slugs.contains(&normalized_anchor) {
                    (RouteResolutionStatus::Resolved, total_file_tokens)
                } else {
                    (RouteResolutionStatus::BrokenAnchor, total_file_tokens)
                }
            } else {
                (RouteResolutionStatus::Resolved, total_file_tokens)
            }
        };

        KnowledgeIndexRoute {
            domain: domain.to_string(),
            label: label.to_string(),
            target_file: resolved_file_rel,
            target_anchor: anchor_part,
            resolution_status,
            hop_distance_from_entry: 1,
            target_tokens,
            line_number,
        }
    }
}

/// Helper to extract all slugified headings from markdown content.
pub fn extract_heading_slugs(content: &str) -> HashSet<String> {
    let mut slugs = HashSet::new();
    let parser = Parser::new_ext(content, Options::all());
    let mut in_heading = false;
    let mut current_heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading_text.clear();
            }
            Event::Text(t) if in_heading => {
                current_heading_text.push_str(&t);
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let slug = slugify_heading(&current_heading_text);
                if !slug.is_empty() {
                    slugs.insert(slug);
                }
            }
            _ => {}
        }
    }

    slugs
}
