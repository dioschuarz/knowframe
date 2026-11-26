//! AST heading extraction, anchor normalization, and fragment metadata derivation.

use crate::ast::parser::offset_to_line;
use crate::fragment::FragmentMetadata;
use crate::model::NoteEdge;
use crate::tokens::counter::count_bpe_tokens;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;

/// Converts a HeadingLevel enum into an integer (1..=6).
pub fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Normalizes a heading title into a URL-safe, lowercase ASCII slug anchor.
/// Strips punctuation, collapses whitespace and hyphens, and removes leading/trailing hyphens.
pub fn slugify_heading(heading: &str) -> String {
    let mut slug = String::with_capacity(heading.len());
    let mut prev_is_hyphen = false;

    for c in heading.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            prev_is_hyphen = false;
        } else if (c.is_whitespace() || c == '-' || c == '_') && !prev_is_hyphen && !slug.is_empty()
        {
            slug.push('-');
            prev_is_hyphen = true;
        }
        // Punctuation characters are stripped
    }

    // Trim trailing hyphens
    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "section".to_string()
    } else {
        slug
    }
}

/// GitHub-flavored Markdown heading slug (the algorithm GitHub and VS Code use):
/// lowercase, drop every character that is not a letter, mark, number, connector
/// punctuation (`_`), space or hyphen, then turn each space into a hyphen. Hyphens are
/// **not** collapsed, so `Skill Catalog & Canonical Location` becomes
/// `skill-catalog--canonical-location`.
pub fn github_slug(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// Disambiguates duplicate anchors within a single document by suffixing -1, -2, etc.
#[derive(Debug, Default, Clone)]
pub struct AnchorDisambiguator {
    counts: HashMap<String, usize>,
}

impl AnchorDisambiguator {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn disambiguate(&mut self, base_slug: &str) -> String {
        let entry = self.counts.entry(base_slug.to_string()).or_insert(0);
        let count = *entry;
        *entry += 1;
        if count == 0 {
            base_slug.to_string()
        } else {
            format!("{}-{}", base_slug, count)
        }
    }
}

/// Raw AST heading data extracted during parsing.
#[derive(Debug, Clone)]
struct RawHeading {
    level: u8,
    heading_text: String,
    start_line: usize,
}

/// Extracts raw headings and computes their line ranges [start_line, end_line].
/// Each heading's scope extends up to the next heading of equal or higher rank (level <= current), or EOF.
fn extract_raw_headings_with_ranges(content: &str) -> Vec<(RawHeading, usize)> {
    let mut raw_headings = Vec::new();
    let mut in_heading = false;
    let mut current_level = HeadingLevel::H1;
    let mut current_text = String::new();
    let mut current_start_offset = 0usize;

    for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_level = level;
                current_text.clear();
                current_start_offset = range.start;
            }
            Event::Text(t) => {
                if in_heading {
                    current_text.push_str(&t);
                }
            }
            Event::Code(c) => {
                if in_heading {
                    current_text.push_str(&c);
                }
            }
            Event::End(TagEnd::Heading(_)) if in_heading => {
                in_heading = false;
                let start_line = offset_to_line(content, current_start_offset) as usize;
                raw_headings.push(RawHeading {
                    level: heading_level_to_u8(current_level),
                    heading_text: current_text.trim().to_string(),
                    start_line,
                });
            }
            _ => {}
        }
    }

    let total_lines = content.lines().count().max(1);
    let n = raw_headings.len();
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        let current = &raw_headings[i];
        let mut end_line = total_lines;

        for next in raw_headings.iter().skip(i + 1) {
            if next.level <= current.level {
                if next.start_line > current.start_line {
                    end_line = next.start_line - 1;
                } else {
                    end_line = current.start_line;
                }
                break;
            }
        }

        let clamped_end = end_line.max(current.start_line);
        result.push((current.clone(), clamped_end));
    }

    result
}

/// Extracts all section fragments from Markdown content, calculating line ranges,
/// BPE tokens, and outbound link references originating within each fragment.
/// Uses disjoint heading-boundary slicing with prefix sums so each byte of the document
/// is BPE-tokenized at most once (instead of once per nesting depth).
pub fn extract_fragments_from_document(content: &str, edges: &[NoteEdge]) -> Vec<FragmentMetadata> {
    let raw_ranges = extract_raw_headings_with_ranges(content);
    if raw_ranges.is_empty() {
        return Vec::new();
    }

    let mut disambiguator = AnchorDisambiguator::new();

    // Record byte start offsets of each 0-indexed line for zero-copy slicing
    let mut line_starts: Vec<usize> = Vec::new();
    line_starts.push(0);
    for (idx, b) in content.bytes().enumerate() {
        if b == b'\n' {
            line_starts.push(idx + 1);
        }
    }
    let total_lines = content.lines().count();

    // Collect sorted unique 0-indexed line boundaries across all fragments
    let mut boundaries: Vec<usize> = Vec::with_capacity(raw_ranges.len() * 2);
    for (raw, end_line) in &raw_ranges {
        let s = raw.start_line.saturating_sub(1).min(total_lines);
        let e = (*end_line).min(total_lines);
        boundaries.push(s);
        boundaries.push(e);
    }
    boundaries.sort_unstable();
    boundaries.dedup();

    // Tokenize each disjoint boundary interval ONCE and build prefix sums
    let mut prefix_tokens: Vec<u32> = Vec::with_capacity(boundaries.len());
    prefix_tokens.push(0);
    for win in boundaries.windows(2) {
        let s_line = win[0];
        let e_line = win[1];
        let seg_tokens = if s_line < e_line && s_line < total_lines {
            let start_byte = line_starts[s_line];
            let end_byte = if e_line < line_starts.len() {
                line_starts[e_line].min(content.len())
            } else {
                content.len()
            };
            if start_byte < end_byte {
                let slice = content[start_byte..end_byte].trim_end_matches('\n');
                count_bpe_tokens(slice).unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };
        let acc = prefix_tokens.last().copied().unwrap_or(0).saturating_add(seg_tokens);
        prefix_tokens.push(acc);
    }

    let mut fragments = Vec::with_capacity(raw_ranges.len());

    for (raw, end_line) in raw_ranges {
        let base_slug = slugify_heading(&raw.heading_text);
        let anchor = disambiguator.disambiguate(&base_slug);

        let start_idx = raw.start_line.saturating_sub(1).min(total_lines);
        let end_idx = end_line.min(total_lines);
        let tokens = if start_idx < end_idx {
            let b_start = boundaries.binary_search(&start_idx).unwrap_or(0);
            let b_end = boundaries.binary_search(&end_idx).unwrap_or(b_start);
            prefix_tokens[b_end].saturating_sub(prefix_tokens[b_start])
        } else {
            0
        };

        // Count outbound links originating in [raw.start_line, end_line]
        let outbound = edges
            .iter()
            .filter(|e| {
                if let Some(line) = e.line {
                    (line as usize) >= raw.start_line && (line as usize) <= end_line
                } else {
                    false
                }
            })
            .count() as u32;

        fragments.push(FragmentMetadata {
            anchor,
            heading: raw.heading_text,
            start_line: raw.start_line,
            end_line,
            tokens,
            inbound: 0,
            outbound,
        });
    }

    fragments
}

/// Updates inbound reference counts on fragments across all nodes in the topology.
pub fn populate_fragment_inbounds(
    nodes: &mut HashMap<String, crate::model::NoteNode>,
    edges: &[NoteEdge],
) {
    // Reset inbound counters
    for node in nodes.values_mut() {
        for frag in &mut node.fragments {
            frag.inbound = 0;
        }
    }

    // Accumulate inbounds for valid edges targeting a specific fragment
    for edge in edges {
        if !edge.is_broken {
            if let Some(ref target_frag) = edge.target_fragment {
                if let Some(target_node) = nodes.get_mut(&edge.target) {
                    for frag in &mut target_node.fragments {
                        if &frag.anchor == target_frag {
                            frag.inbound += 1;
                        }
                    }
                }
            }
        }
    }
}
