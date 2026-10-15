//! CommonMark AST visitor extracting headings, normalized fragment anchors,
//! wikilinks, code citations, and BPE token counts using pulldown-cmark and tiktoken-rs.
//! Implements FR-004 and FR-005.

use crate::ast::parser::offset_to_line;
use crate::ast::tags::extract_frontmatter_tags;
use crate::ast::wikilink::parse_wikilinks;
use crate::fragment::extractor::{slugify_heading, AnchorDisambiguator};
use crate::fragment::FragmentMetadata;
use crate::model::{CodeCitation, ReferenceEdge};
use crate::tokens::counter::count_bpe_tokens;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Result of visiting a CommonMark document AST.
#[derive(Debug, Clone, PartialEq)]
pub struct VisitedDocument {
    pub relative_path: String,
    pub title: String,
    pub total_tokens: u64,
    pub fragments: Vec<FragmentMetadata>,
    pub references: Vec<ReferenceEdge>,
    pub code_citations: Vec<CodeCitation>,
    pub tags: Vec<String>,
}

/// Visitor that traverses CommonMark AST and extracts structural metadata.
pub struct AstVisitor;

impl AstVisitor {
    /// Visits markdown content and extracts headings, anchors, references, citations, and token counts.
    pub fn visit(relative_path: &str, content: &str) -> VisitedDocument {
        let (tags, fm_title) = extract_frontmatter_tags(content);
        let mut title = fm_title;

        let total_tokens = count_bpe_tokens(content).unwrap_or(0) as u64;

        let mut disambiguator = AnchorDisambiguator::new();
        let mut raw_headings: Vec<(HeadingLevel, String, usize, usize)> = Vec::new(); // level, text, start_offset, start_line
        let mut references: Vec<ReferenceEdge> = Vec::new();
        let mut code_citations: Vec<CodeCitation> = Vec::new();
        let mut code_ranges: Vec<std::ops::Range<usize>> = Vec::new();

        let mut in_heading = false;
        let mut current_heading_level = HeadingLevel::H1;
        let mut current_heading_text = String::new();
        let mut current_heading_start_offset = 0;

        let mut current_link_dest: Option<String> = None;
        let mut current_link_text = String::new();
        let mut current_link_start_offset = 0;

        for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    current_heading_level = level;
                    current_heading_text.clear();
                    current_heading_start_offset = range.start;
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    let trimmed = current_heading_text.trim().to_string();
                    if !trimmed.is_empty() {
                        let line = offset_to_line(content, current_heading_start_offset) as usize;
                        if current_heading_level == HeadingLevel::H1 && title.is_none() {
                            title = Some(trimmed.clone());
                        }
                        raw_headings.push((
                            current_heading_level,
                            trimmed,
                            current_heading_start_offset,
                            line,
                        ));
                    }
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    current_link_dest = Some(dest_url.to_string());
                    current_link_text.clear();
                    current_link_start_offset = range.start;
                }
                Event::End(TagEnd::Link) => {
                    if let Some(dest) = current_link_dest.take() {
                        let line = offset_to_line(content, current_link_start_offset) as usize;

                        let clean_dest = dest.trim_start_matches("file://");
                        let path_part = clean_dest.split_once('#').map(|(p, _)| p).unwrap_or(clean_dest);
                        let is_code_citation = dest.starts_with("file://")
                            || path_part.ends_with(".rs")
                            || path_part.ends_with(".ts")
                            || path_part.ends_with(".js")
                            || path_part.ends_with(".py")
                            || path_part.ends_with(".go")
                            || path_part.ends_with(".c")
                            || path_part.ends_with(".cpp")
                            || path_part.ends_with(".h");

                        if is_code_citation {
                            let (file_path, line_no) = parse_code_citation_target(&dest);
                            code_citations.push(CodeCitation {
                                target_file_path: file_path,
                                symbol_name: if current_link_text.is_empty() { None } else { Some(current_link_text.clone()) },
                                line_number: line_no,
                            });
                        } else if !dest.starts_with("http://") && !dest.starts_with("https://") {
                            // Relative markdown link reference
                            let (target_doc, target_frag) = parse_link_target(&dest);
                            references.push(ReferenceEdge {
                                target_document: target_doc,
                                target_fragment: target_frag,
                                relation_type: Some("markdown_link".to_string()),
                                is_resolved: false, // Resolved later against graph topology
                                line_number: line,
                            });
                        }
                    }
                }
                Event::Text(t) => {
                    if in_heading {
                        current_heading_text.push_str(&t);
                    } else if current_link_dest.is_some() {
                        current_link_text.push_str(&t);
                    }
                }
                Event::Code(c) => {
                    if in_heading {
                        current_heading_text.push_str(&c);
                    }
                    code_ranges.push(range.clone());
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    code_ranges.push(range.clone());
                }
                Event::InlineHtml(_) | Event::Html(_) => {
                    // HTML comments / raw HTML are not rendered as links by Obsidian.
                    code_ranges.push(range.clone());
                }
                _ => {}
            }
        }

        // Also extract [[wikilinks]] using dedicated wikilink scanner.
        // Code spans, fenced/indented code blocks, and raw HTML are blanked out first
        // (byte offsets and newlines preserved) so literal `[[...]]` inside code is ignored.
        let masked = mask_ranges(content, &code_ranges);
        let wikilinks = parse_wikilinks(&masked);
        for wl in wikilinks {
            let target_frag = wl.anchor.map(|a| a.trim_start_matches('#').to_string());
            let line_no = offset_to_line(content, wl.start_offset) as usize;
            references.push(ReferenceEdge {
                target_document: wl.target,
                target_fragment: target_frag,
                relation_type: Some("wikilink".to_string()),
                is_resolved: false,
                line_number: line_no,
            });
        }

        // Derive fragments from raw headings with line ranges and per-fragment token counts
        let total_lines = content.lines().count().max(1);
        let mut fragments = Vec::new();

        for i in 0..raw_headings.len() {
            let (_level, heading, start_offset, start_line) = &raw_headings[i];
            let raw_anchor = slugify_heading(heading);
            let anchor = disambiguator.disambiguate(&raw_anchor);

            let end_line = if i + 1 < raw_headings.len() {
                let next_start_line = raw_headings[i + 1].3;
                if next_start_line > *start_line {
                    next_start_line - 1
                } else {
                    *start_line
                }
            } else {
                total_lines
            };

            // Slice content for fragment to count tokens
            let next_offset = if i + 1 < raw_headings.len() {
                raw_headings[i + 1].2
            } else {
                content.len()
            };

            let frag_content = if *start_offset < next_offset && next_offset <= content.len() {
                &content[*start_offset..next_offset]
            } else {
                ""
            };
            let frag_tokens = count_bpe_tokens(frag_content).unwrap_or(0);

            fragments.push(FragmentMetadata {
                anchor,
                heading: heading.clone(),
                start_line: *start_line,
                end_line,
                tokens: frag_tokens,
                inbound: 0,
                outbound: 0,
            });
        }

        let doc_title = title.unwrap_or_else(|| {
            std::path::Path::new(relative_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });

        VisitedDocument {
            relative_path: relative_path.to_string(),
            title: doc_title,
            total_tokens,
            fragments,
            references,
            code_citations,
            tags,
        }
    }

    /// Resolves cross-document heading anchors in-memory for a document given target document fragment metadata.
    /// Returns any BROKEN_FRAGMENT_ANCHOR findings for references with invalid anchors.
    pub fn resolve_cross_document_anchors(
        document: &mut VisitedDocument,
        target_fragments: &std::collections::HashMap<String, Vec<FragmentMetadata>>,
    ) -> Vec<crate::model::DeterministicFinding> {
        let mut findings = Vec::new();
        for (idx, reference) in document.references.iter_mut().enumerate() {
            if let Some(ref target_frag) = reference.target_fragment {
                let clean_frag = target_frag.trim_start_matches('#');
                let frag_slug = slugify_heading(clean_frag);

                let target_key = if reference.target_document.ends_with(".md") {
                    reference.target_document.clone()
                } else {
                    format!("{}.md", reference.target_document)
                };

                if let Some(fragments) = target_fragments
                    .get(&target_key)
                    .or_else(|| target_fragments.get(&reference.target_document))
                {
                    let mut matched_anchor = None;
                    for f in fragments {
                        if f.anchor == frag_slug
                            || slugify_heading(&f.heading) == frag_slug
                            || f.heading.trim().eq_ignore_ascii_case(clean_frag)
                        {
                            matched_anchor = Some(f.anchor.clone());
                            break;
                        }
                    }

                    if let Some(anchor) = matched_anchor {
                        reference.target_fragment = Some(anchor);
                        reference.is_resolved = true;
                    } else {
                        reference.is_resolved = false;
                        findings.push(crate::model::DeterministicFinding {
                            finding_id: format!("FINDING-ANCHOR-{}", idx + 1),
                            rule_id: "BROKEN_FRAGMENT_ANCHOR".to_string(),
                            severity: crate::model::DeterministicFindingSeverity::Error,
                            subject: document.relative_path.clone(),
                            evidence: crate::model::FindingEvidence {
                                changed_paths: Vec::new(),
                                dependency_sources: vec!["inline_markdown_link".to_string()],
                                rule_description: format!(
                                    "Broken fragment anchor '#{}' in target document '{}'",
                                    clean_frag, reference.target_document
                                ),
                                line_number: Some(reference.line_number as u32),
                                anchor: Some(clean_frag.to_string()),
                                target: Some(reference.target_document.clone()),
                            },
                            required_action: format!(
                                "Update heading anchor '#{}' or restore heading in '{}'",
                                clean_frag, reference.target_document
                            ),
                            epistemic_tier: Some(crate::model::EpistemicTier::TierAFullyDeterministic),
                        });
                    }
                }
            }
        }
        findings
    }

    /// Resolves cross-document heading anchors in-memory given target document headings map.
    pub fn resolve_cross_document_headings(
        document: &mut VisitedDocument,
        target_headings: &std::collections::HashMap<String, Vec<String>>,
    ) -> Vec<crate::model::DeterministicFinding> {
        let mut findings = Vec::new();
        for (idx, reference) in document.references.iter_mut().enumerate() {
            if let Some(ref target_frag) = reference.target_fragment {
                let clean_frag = target_frag.trim_start_matches('#');
                let frag_slug = slugify_heading(clean_frag);

                let target_key = if reference.target_document.ends_with(".md") {
                    reference.target_document.clone()
                } else {
                    format!("{}.md", reference.target_document)
                };

                if let Some(headings) = target_headings
                    .get(&target_key)
                    .or_else(|| target_headings.get(&reference.target_document))
                {
                    let mut matched_anchor = None;
                    let mut disambiguator = AnchorDisambiguator::new();
                    for h in headings {
                        let h_slug = slugify_heading(h);
                        let disambiguated = disambiguator.disambiguate(&h_slug);
                        if h_slug == frag_slug
                            || disambiguated == frag_slug
                            || h.trim().eq_ignore_ascii_case(clean_frag)
                        {
                            matched_anchor = Some(disambiguated);
                            break;
                        }
                    }

                    if let Some(anchor) = matched_anchor {
                        reference.target_fragment = Some(anchor);
                        reference.is_resolved = true;
                    } else {
                        reference.is_resolved = false;
                        findings.push(crate::model::DeterministicFinding {
                            finding_id: format!("FINDING-ANCHOR-{}", idx + 1),
                            rule_id: "BROKEN_FRAGMENT_ANCHOR".to_string(),
                            severity: crate::model::DeterministicFindingSeverity::Error,
                            subject: document.relative_path.clone(),
                            evidence: crate::model::FindingEvidence {
                                changed_paths: Vec::new(),
                                dependency_sources: vec!["inline_markdown_link".to_string()],
                                rule_description: format!(
                                    "Broken fragment anchor '#{}' in target document '{}'",
                                    clean_frag, reference.target_document
                                ),
                                line_number: Some(reference.line_number as u32),
                                anchor: Some(clean_frag.to_string()),
                                target: Some(reference.target_document.clone()),
                            },
                            required_action: format!(
                                "Update heading anchor '#{}' or restore heading in '{}'",
                                clean_frag, reference.target_document
                            ),
                            epistemic_tier: Some(crate::model::EpistemicTier::TierAFullyDeterministic),
                        });
                    }
                }
            }
        }
        findings
    }
}

/// Replaces every character inside `ranges` with a space (newlines kept) so byte offsets
/// and line numbers stay identical while masked text can no longer match link patterns.
fn mask_ranges(content: &str, ranges: &[std::ops::Range<usize>]) -> String {
    if ranges.is_empty() {
        return content.to_string();
    }
    let mut bytes = content.as_bytes().to_vec();
    for r in ranges {
        let end = r.end.min(bytes.len());
        for b in &mut bytes[r.start.min(end)..end] {
            if *b != b'\n' && *b != b'\r' {
                *b = b' ';
            }
        }
    }
    // pulldown-cmark ranges always fall on char boundaries, so whole multi-byte
    // characters are replaced and the result stays valid UTF-8.
    String::from_utf8(bytes).unwrap_or_else(|_| content.to_string())
}

fn parse_link_target(dest: &str) -> (String, Option<String>) {
    if let Some((path, frag)) = dest.split_once('#') {
        let frag_str = if frag.is_empty() { None } else { Some(frag.to_string()) };
        (path.to_string(), frag_str)
    } else {
        (dest.to_string(), None)
    }
}

fn parse_code_citation_target(dest: &str) -> (String, Option<usize>) {
    let clean_dest = dest.trim_start_matches("file://");
    if let Some((path, anchor)) = clean_dest.split_once('#') {
        let line_no = anchor
            .trim_start_matches('L')
            .split('-')
            .next()
            .and_then(|s| s.parse::<usize>().ok());
        (path.to_string(), line_no)
    } else if let Some((path, line_str)) = clean_dest.split_once(':') {
        let line_no = line_str.parse::<usize>().ok();
        (path.to_string(), line_no)
    } else {
        (clean_dest.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_visitor_headings_and_anchors() {
        let content = r#"# Architecture Overview

This is the main architecture doc.

## Core Engine

The engine is written in Rust.

### Memory Layout

Details about memory layout.

## Core Engine

Duplicate heading test.
"#;
        let visited = AstVisitor::visit("docs/arch.md", content);

        assert_eq!(visited.title, "Architecture Overview");
        assert_eq!(visited.fragments.len(), 4);
        assert_eq!(visited.fragments[0].anchor, "architecture-overview");
        assert_eq!(visited.fragments[1].anchor, "core-engine");
        assert_eq!(visited.fragments[2].anchor, "memory-layout");
        assert_eq!(visited.fragments[3].anchor, "core-engine-1"); // Disambiguated!
        assert!(visited.total_tokens > 0);
    }

    #[test]
    fn test_ast_visitor_references_and_citations() {
        let content = r#"# Interop

See [[api_reference#endpoints]] and [Guide](guide.md#getting-started).
Also see implementation in [EngineCore](crates/core/src/lib.rs#L10-L20).
"#;
        let visited = AstVisitor::visit("docs/interop.md", content);

        assert_eq!(visited.references.len(), 2);
        assert_eq!(visited.references[0].target_document, "guide.md");
        assert_eq!(visited.references[0].target_fragment, Some("getting-started".to_string()));
        assert_eq!(visited.references[1].target_document, "api_reference.md");
        assert_eq!(visited.references[1].target_fragment, Some("endpoints".to_string()));

        assert_eq!(visited.code_citations.len(), 1);
        assert_eq!(visited.code_citations[0].target_file_path, "crates/core/src/lib.rs");
        assert_eq!(visited.code_citations[0].line_number, Some(10));
    }
}
