//! Parser for Obsidian-style wikilinks: [[target]], [[target|label]], [[target#heading|label]].

use crate::model::{LinkResolutionStatus, LinkType, NoteEdge};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawWikilink {
    pub target: String,
    pub raw_target: String,
    pub label: String,
    pub has_explicit_alias: bool,
    pub anchor: Option<String>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub is_embed: bool,
}

pub fn parse_wikilinks(text: &str) -> Vec<RawWikilink> {
    let re = Regex::new(r"\[\[([^\[\]\r\n]+)\]\]").unwrap();
    let bytes = text.as_bytes();
    let mut links = Vec::new();

    for cap in re.captures_iter(text) {
        let mat = cap.get(0).unwrap();
        let is_embed = mat.start() > 0 && bytes[mat.start() - 1] == b'!';
        let inner = cap.get(1).unwrap().as_str().trim();
        if inner.is_empty() {
            continue;
        }

        // Split target and alias: [[target|alias]]. Inside Markdown tables the pipe is
        // escaped as `[[target\|alias]]`, so drop the trailing backslash from the target.
        let (raw_target_part, alias) = if let Some((tgt, label)) = inner.split_once('|') {
            (tgt.trim().trim_end_matches('\\').trim_end(), Some(label.trim().to_string()))
        } else {
            (inner, None)
        };

        // Split target and anchor: [[target#anchor]]
        let (raw_target, anchor) = if let Some((tgt, anch)) = raw_target_part.split_once('#') {
            (tgt.trim(), Some(anch.trim().to_string()))
        } else {
            (raw_target_part, None)
        };

        // Normalize target path: append .md if missing extension
        let mut target_path = raw_target.to_string();
        if !target_path.is_empty() && !target_path.contains('.') {
            target_path.push_str(".md");
        }

        let has_explicit_alias = alias.is_some();
        let label = alias.unwrap_or_else(|| {
            if raw_target.is_empty() {
                anchor.clone().unwrap_or_default()
            } else {
                raw_target.to_string()
            }
        });

        links.push(RawWikilink {
            target: target_path,
            raw_target: raw_target.to_string(),
            label,
            has_explicit_alias,
            anchor,
            start_offset: mat.start(),
            end_offset: mat.end(),
            is_embed,
        });
    }

    links
}

pub fn wikilink_to_edge(source: &str, link: RawWikilink, line: Option<u32>) -> NoteEdge {
    let target_fragment = link
        .anchor
        .as_ref()
        .map(|a| a.trim_start_matches('#').to_string());
    NoteEdge {
        id: format!("{}->{}", source, link.target),
        source: source.to_string(),
        target: link.target,
        target_fragment,
        link_type: LinkType::Wikilink,
        is_broken: false,
        raw_text: link.label,
        anchor: link.anchor,
        line,
        resolution_status: Some(LinkResolutionStatus::Resolved),
        raw_target: Some(link.raw_target),
        is_clickable: false,
        non_clickable_reason: Some(crate::model::NonClickableReason::WikilinkSyntax),
        clickable_target: None,
    }
}
