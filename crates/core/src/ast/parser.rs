//! Markdown AST pull parser using pulldown-cmark with code-block link immunity.

use super::tags::{extract_frontmatter_tags, extract_inline_hashtags};
use super::wikilink::{parse_wikilinks, wikilink_to_edge};
use crate::model::{LinkResolutionStatus, LinkType, NoteEdge};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDocument {
    pub file_id: String,
    pub title: Option<String>,
    pub headings: Vec<String>,
    pub edges: Vec<NoteEdge>,
    pub tags: Vec<String>,
}

/// Computes 1-based line number for a byte offset in content.
pub fn offset_to_line(content: &str, offset: usize) -> u32 {
    let safe_offset = offset.min(content.len());
    let newlines = content[..safe_offset]
        .bytes()
        .filter(|&b| b == b'\n')
        .count();
    (newlines + 1) as u32
}

pub fn parse_markdown_ast(file_id: &str, content: &str) -> ParsedDocument {
    let (mut all_tags, fm_title) = extract_frontmatter_tags(content);
    let mut title = fm_title;
    let mut headings = Vec::new();
    let mut edges = Vec::new();
    let mut tag_set = HashSet::new();
    for t in all_tags.drain(..) {
        tag_set.insert(t);
    }

    let mut in_code_block = false;
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut current_heading_level = HeadingLevel::H1;

    let mut current_link_dest: Option<String> = None;
    let mut current_link_text = String::new();
    let mut current_link_offset: usize = 0;

    // Mask for code blocks and code spans to ensure immunity for regexes
    let mut masked_bytes = content.as_bytes().to_vec();

    for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                // Mask start through end when block finishes
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            Event::Code(_) => {
                // Inline code span: mask with spaces
                let end = range.end.min(masked_bytes.len());
                for b in &mut masked_bytes[range.start..end] {
                    if *b != b'\n' {
                        *b = b' ';
                    }
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_heading_level = level;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let trimmed = heading_text.trim().to_string();
                if !trimmed.is_empty() {
                    if current_heading_level == HeadingLevel::H1 && title.is_none() {
                        title = Some(trimmed.clone());
                    }
                    headings.push(trimmed);
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                if !in_code_block {
                    current_link_dest = Some(dest_url.to_string());
                    current_link_text.clear();
                    current_link_offset = range.start;
                }
            }
            Event::End(TagEnd::Link) => {
                if let Some(dest) = current_link_dest.take() {
                    // Ignore external URLs and bare anchors
                    if !dest.starts_with("http://")
                        && !dest.starts_with("https://")
                        && !dest.starts_with("mailto:")
                        && !dest.starts_with("ftp:")
                        && !dest.starts_with('#')
                    {
                        let (path_part, anchor) = if let Some((p, a)) = dest.split_once('#') {
                            (p.to_string(), Some(a.to_string()))
                        } else {
                            (dest, None)
                        };

                        let line = Some(offset_to_line(content, current_link_offset));
                        let raw_text = if current_link_text.trim().is_empty() {
                            path_part.clone()
                        } else {
                            current_link_text.trim().to_string()
                        };

                        edges.push(NoteEdge {
                            id: format!("{}->{}", file_id, path_part),
                            source: file_id.to_string(),
                            target: path_part,
                            link_type: LinkType::Markdown,
                            is_broken: false,
                            raw_text,
                            anchor,
                            line,
                            resolution_status: Some(LinkResolutionStatus::Resolved),
                        });
                    }
                }
            }
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                }
                if current_link_dest.is_some() {
                    current_link_text.push_str(&text);
                }
                if in_code_block {
                    // Mask code block content with spaces to prevent regex false-positives
                    let end = range.end.min(masked_bytes.len());
                    for b in &mut masked_bytes[range.start..end] {
                        if *b != b'\n' {
                            *b = b' ';
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Now extract wikilinks and inline hashtags from masked content (non-code text only)
    if let Ok(masked_text) = std::str::from_utf8(&masked_bytes) {
        let wikilinks = parse_wikilinks(masked_text);
        for w in wikilinks {
            let link_line = Some(offset_to_line(content, w.start_offset));
            edges.push(wikilink_to_edge(file_id, w, link_line));
        }

        let inline_tags = extract_inline_hashtags(masked_text);
        for t in inline_tags {
            tag_set.insert(t);
        }
    }

    let mut final_tags: Vec<String> = tag_set.into_iter().collect();
    final_tags.sort();

    ParsedDocument {
        file_id: file_id.to_string(),
        title,
        headings,
        edges,
        tags: final_tags,
    }
}
