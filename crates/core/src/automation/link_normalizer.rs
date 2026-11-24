//! Deterministic link clickability normalizer.
//!
//! Converts valid non-clickable links (Obsidian `[[target#fragment|label]]`, missing `.md`
//! extensions, vault-root/basename relative shortcuts, case mismatches, unencoded spaces,
//! and unslugified anchors) into exact, universally clickable CommonMark/GFM relative
//! Markdown links `[Label](relative/path.md#slug)`.
//!
//! Guarantees:
//! - Preserves YAML frontmatter, fenced code blocks, and inline code spans untouched.
//! - Skips broken links whose target file or fragment does not exist.
//! - Skips Obsidian embed transclusions (`![[...]]`), external URLs, and pure valid anchors.
//! - Produces exact contextual unified diffs for human-in-the-loop review before apply.

use crate::ast::parser::{offset_to_line, parse_markdown_ast};
use crate::ast::wikilink::parse_wikilinks;
use crate::cli::scan::build_globset;
use crate::config::ContextVaultConfig;
use crate::graph::resolver::{resolve_edge_integrity_with_policy, ResolverIndex};
use crate::model::{LinkPortabilityMode, LinkType, NonClickableReason, NoteEdge};
use anyhow::{Context, Result};
use pulldown_cmark::{Event, LinkType as CmarkLinkType, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkRewriteItem {
    pub line: u32,
    pub original: String,
    pub replacement: String,
    pub resolved_target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_fragment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<NonClickableReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentLinkNormalizationResult {
    pub file_path: String,
    pub updated_content: String,
    pub unified_diff: String,
    pub rewrites: Vec<LinkRewriteItem>,
    pub broken_skipped: usize,
    pub already_clickable: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLinkNormalizationReport {
    pub applied: bool,
    pub files_scanned: usize,
    pub files_modified: usize,
    pub total_rewrites: usize,
    pub total_broken_skipped: usize,
    pub total_already_clickable: usize,
    pub documents: Vec<DocumentLinkNormalizationResult>,
}

/// Precomputed workspace snapshot used to resolve targets & heading anchors deterministically.
pub struct WorkspaceLinkIndex {
    pub known_nodes: HashMap<String, PathBuf>,
    pub headings: HashMap<String, Vec<String>>,
    pub files: Vec<(PathBuf, String)>,
}

impl WorkspaceLinkIndex {
    pub fn build(workspace_root: &Path) -> Result<Self> {
        let config = ContextVaultConfig::load_from_workspace(workspace_root);
        let include_set = build_globset(&config.get_scan_include())?;
        let exclude_set = build_globset(&config.get_scan_exclude())?;

        let walker = WalkDir::new(workspace_root).into_iter().filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                if matches!(
                    name.as_ref(),
                    ".git"
                        | "node_modules"
                        | "target"
                        | "dist"
                        | "out"
                        | "build"
                        | ".venv"
                        | "venv"
                        | ".tox"
                        | ".mypy_cache"
                        | ".pytest_cache"
                        | ".next"
                        | ".nuxt"
                        | ".cache"
                        | ".turbo"
                        | ".code-review-graph"
                        | ".understand-anything"
                        | "coverage"
                ) {
                    return false;
                }
            }
            true
        });

        let mut files: Vec<(PathBuf, String)> = walker
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| {
                let abs_path = e.path().to_path_buf();
                let rel_path = abs_path.strip_prefix(workspace_root).ok()?;
                let rel_str = rel_path.to_string_lossy().replace('\\', "/");
                if !include_set.is_match(&rel_str) || exclude_set.is_match(&rel_str) {
                    return None;
                }
                Some((abs_path, rel_str))
            })
            .collect();

        files.sort_by(|a, b| a.1.cmp(&b.1));

        let mut known_nodes = HashMap::with_capacity(files.len());
        let mut headings = HashMap::with_capacity(files.len());

        for (abs_path, rel_str) in &files {
            known_nodes.insert(rel_str.clone(), abs_path.clone());
            if let Ok(content) = std::fs::read_to_string(abs_path) {
                let parsed = parse_markdown_ast(rel_str, &content);
                headings.insert(rel_str.clone(), parsed.headings);
            }
        }

        Ok(Self {
            known_nodes,
            headings,
            files,
        })
    }
}

/// Computes the byte end offset of YAML frontmatter if the document begins with `---`.
fn frontmatter_end_offset(content: &str) -> usize {
    if content.starts_with("---\n") {
        if let Some(pos) = content[4..].find("\n---") {
            let after_marker = 4 + pos + 4;
            if content[after_marker..].starts_with("\r\n") {
                return after_marker + 2;
            } else if content[after_marker..].starts_with('\n') {
                return after_marker + 1;
            } else if after_marker == content.len() {
                return after_marker;
            }
        }
    } else if content.starts_with("---\r\n") {
        if let Some(pos) = content[5..].find("\r\n---") {
            let after_marker = 5 + pos + 5;
            if content[after_marker..].starts_with("\r\n") {
                return after_marker + 2;
            } else if content[after_marker..].starts_with('\n') {
                return after_marker + 1;
            } else if after_marker == content.len() {
                return after_marker;
            }
        }
    }
    0
}

/// Masks YAML frontmatter, fenced/indented code blocks, and inline code spans with ASCII spaces
/// (preserving byte offsets and newlines) so regex link scanners never touch code or metadata.
fn mask_protected_content(content: &str) -> String {
    let mut masked = content.as_bytes().to_vec();

    // 1. Mask YAML frontmatter
    let fm_end = frontmatter_end_offset(content);
    let fm_limit = fm_end.min(masked.len());
    for b in &mut masked[..fm_limit] {
        if *b != b'\n' && *b != b'\r' {
            *b = b' ';
        }
    }

    // 2. Mask code blocks & inline code spans via pulldown-cmark
    let mut in_code_block = false;
    for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                let end = range.end.min(masked.len());
                for b in &mut masked[range.start.min(end)..end] {
                    if *b != b'\n' && *b != b'\r' {
                        *b = b' ';
                    }
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            Event::Code(_) => {
                let end = range.end.min(masked.len());
                for b in &mut masked[range.start.min(end)..end] {
                    if *b != b'\n' && *b != b'\r' {
                        *b = b' ';
                    }
                }
            }
            Event::Text(_) if in_code_block => {
                let end = range.end.min(masked.len());
                for b in &mut masked[range.start.min(end)..end] {
                    if *b != b'\n' && *b != b'\r' {
                        *b = b' ';
                    }
                }
            }
            _ => {}
        }
    }

    String::from_utf8(masked).unwrap_or_else(|_| content.to_string())
}

/// Rewrites ONLY the destination URL inside a standard inline Markdown link slice `[label](dest ...)`
/// while preserving label formatting and any optional title string.
fn rewrite_inline_markdown_destination(raw_slice: &str, new_dest: &str) -> Option<String> {
    // Walk `[label]` respecting nested brackets and escaped characters
    let bytes = raw_slice.as_bytes();
    if bytes.first() != Some(&b'[') {
        return None;
    }

    let mut depth = 0i32;
    let mut idx = 0usize;
    let mut close_bracket = None;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\\' => {
                idx += 2;
                continue;
            }
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    close_bracket = Some(idx);
                    break;
                }
            }
            _ => {}
        }
        idx += 1;
    }

    let close_idx = close_bracket?;
    if bytes.get(close_idx + 1) != Some(&b'(') || bytes.last() != Some(&b')') {
        return None;
    }

    let label_part = &raw_slice[..=close_idx];
    let inner_paren = &raw_slice[close_idx + 2..raw_slice.len() - 1];
    let trimmed_inner = inner_paren.trim();

    // Preserve optional title (e.g. `dest "Title"` or `<dest> "Title"`)
    let title_suffix = if let Some(rest) = trimmed_inner.strip_prefix('<') {
        rest.find('>').map(|end| rest[end + 1..].to_string())
    } else {
        // Look for whitespace followed by quote `"` or `'` or `(`
        let mut found_title = None;
        for (i, ch) in trimmed_inner.char_indices() {
            if ch.is_whitespace() {
                let after = trimmed_inner[i..].trim_start();
                if after.starts_with('"') || after.starts_with('\'') || after.starts_with('(') {
                    found_title = Some(format!(" {}", after));
                    break;
                }
            }
        }
        found_title
    };

    match title_suffix {
        Some(t) if !t.trim().is_empty() => Some(format!("{}({}{})", label_part, new_dest, t)),
        _ => Some(format!("{}({})", label_part, new_dest)),
    }
}

/// Generates a compact contextual unified diff (2 context lines per hunk).
pub fn generate_compact_unified_diff(path: &str, old_content: &str, new_content: &str) -> String {
    if old_content == new_content {
        return String::new();
    }
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    let max_len = old_lines.len().max(new_lines.len());

    let mut changed_indices = Vec::new();
    for i in 0..max_len {
        if old_lines.get(i) != new_lines.get(i) {
            changed_indices.push(i);
        }
    }
    if changed_indices.is_empty() {
        return String::new();
    }

    let mut diff = String::new();
    diff.push_str(&format!("--- a/{}\n", path));
    diff.push_str(&format!("+++ b/{}\n", path));

    // Group nearby changed lines into hunks with 2 context lines
    let context = 2usize;
    let mut hunks: Vec<(usize, usize)> = Vec::new();
    for &idx in &changed_indices {
        let start = idx.saturating_sub(context);
        let end = (idx + context + 1).min(max_len);
        if let Some(last) = hunks.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        hunks.push((start, end));
    }

    for (start, end) in hunks {
        let count = end - start;
        diff.push_str(&format!(
            "@@ -{},{} +{},{} @@\n",
            start + 1,
            count,
            start + 1,
            count
        ));
        for i in start..end {
            match (old_lines.get(i), new_lines.get(i)) {
                (Some(o), Some(n)) if o == n => {
                    diff.push_str(&format!(" {}\n", o));
                }
                (Some(o), Some(n)) => {
                    diff.push_str(&format!("-{}\n", o));
                    diff.push_str(&format!("+{}\n", n));
                }
                (Some(o), None) => {
                    diff.push_str(&format!("-{}\n", o));
                }
                (None, Some(n)) => {
                    diff.push_str(&format!("+{}\n", n));
                }
                (None, None) => {}
            }
        }
    }

    diff
}

/// Deterministically normalizes all valid non-clickable links inside a single document.
pub fn normalize_document_links(
    file_id: &str,
    content: &str,
    index: &ResolverIndex,
    headings: &HashMap<String, Vec<String>>,
) -> DocumentLinkNormalizationResult {
    let masked = mask_protected_content(content);
    let fm_end = frontmatter_end_offset(content);

    let mut replacements: Vec<(usize, usize, String, LinkRewriteItem)> = Vec::new();
    let mut broken_skipped = 0usize;
    let mut already_clickable = 0usize;

    // 1. Pass A: Obsidian wikilinks `[[target#fragment|label]]`
    for w in parse_wikilinks(&masked) {
        if w.is_embed {
            continue;
        }
        let line = offset_to_line(content, w.start_offset);
        let mut edge = NoteEdge::new(
            file_id,
            if w.target.is_empty() {
                file_id.to_string()
            } else {
                w.target.clone()
            },
            LinkType::Wikilink,
            w.label.clone(),
            w.anchor.clone(),
            Some(line),
        );
        edge.raw_target = Some(w.raw_target.clone());
        resolve_edge_integrity_with_policy(
            &mut edge,
            index,
            headings,
            LinkPortabilityMode::StandardMarkdown,
        );

        if edge.is_broken {
            broken_skipped += 1;
            continue;
        }

        if let Some(clickable_dest) = edge.clickable_target.clone() {
            let label = if w.label.trim().is_empty() {
                edge.target.clone()
            } else {
                w.label.clone()
            };
            let replacement = format!("[{}]({})", label, clickable_dest);
            let original = content[w.start_offset..w.end_offset].to_string();
            if original != replacement {
                replacements.push((
                    w.start_offset,
                    w.end_offset,
                    replacement.clone(),
                    LinkRewriteItem {
                        line,
                        original,
                        replacement,
                        resolved_target: edge.target,
                        resolved_fragment: edge.target_fragment,
                        reason: edge.non_clickable_reason,
                    },
                ));
            }
        }
    }

    // 2. Pass B: Standard inline Markdown links `[label](dest)` that resolve but are non-clickable
    let mut in_code_block = false;
    for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code_block = true,
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::Start(Tag::Link {
                link_type: CmarkLinkType::Inline,
                dest_url,
                ..
            }) if !in_code_block && range.start >= fm_end => {
                let dest = dest_url.as_ref();
                if dest.starts_with("http://")
                    || dest.starts_with("https://")
                    || dest.starts_with("mailto:")
                    || dest.starts_with("ftp:")
                    || dest.starts_with("file://")
                    || dest.starts_with("vscode://")
                    || dest.is_empty()
                {
                    continue;
                }

                let (path_part, anchor) = if let Some((p, a)) = dest.split_once('#') {
                    (p.to_string(), Some(a.to_string()))
                } else {
                    (dest.to_string(), None)
                };

                // Skip pure same-file anchor if empty path
                if path_part.is_empty() && anchor.is_none() {
                    continue;
                }

                let line = offset_to_line(content, range.start);
                let target_for_res = if path_part.is_empty() {
                    file_id.to_string()
                } else {
                    path_part.clone()
                };
                let mut edge = NoteEdge::new(
                    file_id,
                    target_for_res,
                    LinkType::Markdown,
                    "",
                    anchor,
                    Some(line),
                );
                edge.raw_target = Some(path_part);
                resolve_edge_integrity_with_policy(
                    &mut edge,
                    index,
                    headings,
                    LinkPortabilityMode::StandardMarkdown,
                );

                if edge.is_broken {
                    broken_skipped += 1;
                    continue;
                }

                if edge.is_clickable {
                    already_clickable += 1;
                    continue;
                }

                if let Some(clickable_dest) = edge.clickable_target.clone() {
                    let raw_slice = &content[range.start..range.end];
                    if let Some(replacement) =
                        rewrite_inline_markdown_destination(raw_slice, &clickable_dest)
                    {
                        if raw_slice != replacement {
                            replacements.push((
                                range.start,
                                range.end,
                                replacement.clone(),
                                LinkRewriteItem {
                                    line,
                                    original: raw_slice.to_string(),
                                    replacement,
                                    resolved_target: edge.target,
                                    resolved_fragment: edge.target_fragment,
                                    reason: edge.non_clickable_reason,
                                },
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Apply replacements in reverse offset order to keep byte offsets valid
    replacements.sort_by(|a, b| b.0.cmp(&a.0));
    let mut updated_content = content.to_string();
    let mut applied_rewrites = Vec::new();
    let mut last_start = usize::MAX;

    for (start, end, replacement, item) in replacements {
        if end > last_start || end > updated_content.len() || start > end {
            continue;
        }
        updated_content.replace_range(start..end, &replacement);
        last_start = start;
        applied_rewrites.push(item);
    }

    applied_rewrites.reverse();
    let unified_diff = generate_compact_unified_diff(file_id, content, &updated_content);

    DocumentLinkNormalizationResult {
        file_path: file_id.to_string(),
        updated_content,
        unified_diff,
        rewrites: applied_rewrites,
        broken_skipped,
        already_clickable,
    }
}

/// Normalizes links across a single file (if `target_file` is `Some`) or the entire workspace.
pub fn normalize_workspace_links(
    workspace_root: &Path,
    target_file: Option<&str>,
    apply: bool,
) -> Result<WorkspaceLinkNormalizationReport> {
    let ws_index = WorkspaceLinkIndex::build(workspace_root)?;
    let resolver_index = ResolverIndex::new(&ws_index.known_nodes);

    let normalized_filter = target_file.map(|tf| {
        let p = Path::new(tf);
        if p.is_absolute() {
            p.strip_prefix(workspace_root)
                .map(|rel| rel.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| tf.replace('\\', "/"))
        } else {
            tf.trim_start_matches("./").replace('\\', "/")
        }
    });

    let mut documents = Vec::new();
    let mut files_scanned = 0usize;
    let mut files_modified = 0usize;
    let mut total_rewrites = 0usize;
    let mut total_broken_skipped = 0usize;
    let mut total_already_clickable = 0usize;

    for (abs_path, rel_str) in &ws_index.files {
        if let Some(ref filter) = normalized_filter {
            if rel_str != filter {
                continue;
            }
        }
        files_scanned += 1;
        let content = std::fs::read_to_string(abs_path)
            .with_context(|| format!("Failed to read {}", abs_path.display()))?;
        let res = normalize_document_links(rel_str, &content, &resolver_index, &ws_index.headings);

        total_rewrites += res.rewrites.len();
        total_broken_skipped += res.broken_skipped;
        total_already_clickable += res.already_clickable;

        if !res.rewrites.is_empty() {
            if apply {
                std::fs::write(abs_path, &res.updated_content)
                    .with_context(|| format!("Failed to write {}", abs_path.display()))?;
                files_modified += 1;
            }
            documents.push(res);
        }
    }

    Ok(WorkspaceLinkNormalizationReport {
        applied: apply,
        files_scanned,
        files_modified,
        total_rewrites,
        total_broken_skipped,
        total_already_clickable,
        documents,
    })
}
