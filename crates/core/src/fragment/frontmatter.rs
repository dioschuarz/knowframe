//! Non-destructive YAML frontmatter parsing, updating, and AST authority verification.

use super::FragmentMetadata;
use super::MaterializedFragmentEntry;
use super::MaterializedWikiFrontmatter;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Computes the SHA-256 hash of a string.
pub fn compute_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Splits a Markdown document into optional frontmatter (between first and second `---`)
/// and the remaining body.
pub fn split_frontmatter_and_body(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    // Must be at the start of content
    let rest = &trimmed[3..];
    if let Some(end_idx) = rest.find("\n---") {
        let fm_yaml = &rest[..end_idx];
        let body_start = end_idx + 4; // skip \n---
        let body = if body_start < rest.len() {
            rest[body_start..].trim_start_matches('\n').to_string()
        } else {
            String::new()
        };
        (Some(fm_yaml.trim().to_string()), body)
    } else {
        (None, content.to_string())
    }
}

/// Verifies whether the materialized frontmatter is valid for the given document body.
pub fn is_frontmatter_cache_valid(frontmatter: &MaterializedWikiFrontmatter, body: &str) -> bool {
    if frontmatter.fragments.is_empty() {
        return false;
    }
    if let Some(ref hash) = frontmatter.content_hash {
        let current_hash = compute_content_hash(body);
        hash == &current_hash
    } else {
        false
    }
}

/// Parses the materialized frontmatter from a YAML frontmatter string, if present.
pub fn parse_materialized_frontmatter(fm_str: &str) -> Option<MaterializedWikiFrontmatter> {
    let value: serde_yaml::Value = serde_yaml::from_str(fm_str).ok()?;
    let wiki_val = value.get("wiki")?;
    let mut fragments = BTreeMap::new();

    let content_hash = wiki_val
        .get("content_hash")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if let Some(frag_map) = wiki_val.get("fragments").and_then(|v| v.as_mapping()) {
        for (k, v) in frag_map {
            if let Some(anchor) = k.as_str() {
                let tokens = v.get("tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32;
                let inbound = v.get("inbound").and_then(|i| i.as_u64()).unwrap_or(0) as u32;
                let outbound = v.get("outbound").and_then(|o| o.as_u64()).unwrap_or(0) as u32;

                fragments.insert(
                    anchor.to_string(),
                    MaterializedFragmentEntry {
                        tokens,
                        inbound,
                        outbound,
                    },
                );
            }
        }
    }

    Some(MaterializedWikiFrontmatter {
        content_hash,
        fragments,
    })
}

/// Non-destructively injects or updates the `wiki.fragments` and `wiki.content_hash` keys
/// in the document's YAML frontmatter. Preserves all other user frontmatter keys and the body text.
pub fn inject_materialized_frontmatter(
    content: &str,
    fragments: &[FragmentMetadata],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (fm_opt, body) = split_frontmatter_and_body(content);
    let body_hash = compute_content_hash(&body);

    let mut root_mapping: serde_yaml::Mapping = match fm_opt {
        Some(ref yaml_str) if !yaml_str.trim().is_empty() => serde_yaml::from_str(yaml_str)
            .map_err(|err| format!("Cannot safely update existing YAML frontmatter: {err}"))?,
        _ => serde_yaml::Mapping::new(),
    };

    // Build fragments mapping
    let mut frag_map = serde_yaml::Mapping::new();
    for frag in fragments {
        let mut entry = serde_yaml::Mapping::new();
        entry.insert(
            serde_yaml::Value::String("tokens".to_string()),
            serde_yaml::Value::Number(frag.tokens.into()),
        );
        entry.insert(
            serde_yaml::Value::String("inbound".to_string()),
            serde_yaml::Value::Number(frag.inbound.into()),
        );
        entry.insert(
            serde_yaml::Value::String("outbound".to_string()),
            serde_yaml::Value::Number(frag.outbound.into()),
        );
        frag_map.insert(
            serde_yaml::Value::String(frag.anchor.clone()),
            serde_yaml::Value::Mapping(entry),
        );
    }

    // Build wiki mapping
    let mut wiki_map = serde_yaml::Mapping::new();
    wiki_map.insert(
        serde_yaml::Value::String("content_hash".to_string()),
        serde_yaml::Value::String(body_hash),
    );
    wiki_map.insert(
        serde_yaml::Value::String("fragments".to_string()),
        serde_yaml::Value::Mapping(frag_map),
    );

    // Insert or replace wiki in root_mapping
    root_mapping.insert(
        serde_yaml::Value::String("wiki".to_string()),
        serde_yaml::Value::Mapping(wiki_map),
    );

    let serialized_fm = serde_yaml::to_string(&root_mapping)?;
    let clean_fm = serialized_fm.trim();

    let new_content = if body.is_empty() {
        format!("---\n{}\n---\n", clean_fm)
    } else {
        format!("---\n{}\n---\n{}", clean_fm, body)
    };

    Ok(new_content)
}

/// Non-destructively injects or updates document tags in YAML frontmatter.
/// Existing frontmatter keys and Markdown body content are preserved.
pub fn inject_tags_frontmatter(
    content: &str,
    tags: &[&str],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (fm_opt, body) = split_frontmatter_and_body(content);
    let mut root_mapping: serde_yaml::Mapping = match fm_opt {
        Some(ref yaml_str) if !yaml_str.trim().is_empty() => serde_yaml::from_str(yaml_str)
            .map_err(|err| format!("Cannot safely update existing YAML frontmatter: {err}"))?,
        _ => serde_yaml::Mapping::new(),
    };

    let tag_values = tags
        .iter()
        .map(|tag| serde_yaml::Value::String((*tag).to_string()))
        .collect();
    root_mapping.insert(
        serde_yaml::Value::String("tags".to_string()),
        serde_yaml::Value::Sequence(tag_values),
    );

    let serialized_fm = serde_yaml::to_string(&root_mapping)?;
    let clean_fm = serialized_fm.trim();
    if body.is_empty() {
        Ok(format!("---\n{}\n---\n", clean_fm))
    } else {
        Ok(format!("---\n{}\n---\n{}", clean_fm, body))
    }
}

/// Extracts `code_dependencies` from Markdown document frontmatter.
/// Returns Ok(Vec<String>) if declared (or empty if no key / no frontmatter),
/// or Err if frontmatter YAML is malformed or `code_dependencies` is not a sequence of strings.
pub fn parse_code_dependencies(content: &str) -> Result<Vec<String>, String> {
    let (fm_opt, _) = split_frontmatter_and_body(content);
    let fm_str = match fm_opt {
        Some(s) if !s.trim().is_empty() => s,
        _ => return Ok(Vec::new()),
    };

    let value: serde_yaml::Value = serde_yaml::from_str(&fm_str)
        .map_err(|err| format!("Malformed YAML frontmatter: {err}"))?;

    let mapping = match value.as_mapping() {
        Some(m) => m,
        None => return Ok(Vec::new()),
    };

    let code_deps_val = match mapping.get(&serde_yaml::Value::String("code_dependencies".to_string())) {
        Some(v) => v,
        None => return Ok(Vec::new()),
    };

    match code_deps_val {
        serde_yaml::Value::Sequence(seq) => {
            let mut deps = Vec::new();
            for item in seq {
                if let Some(s) = item.as_str() {
                    deps.push(s.to_string());
                } else {
                    return Err("Elements in `code_dependencies` must be strings".to_string());
                }
            }
            Ok(deps)
        }
        serde_yaml::Value::Null => Ok(Vec::new()),
        _ => Err("`code_dependencies` must be a YAML array of strings".to_string()),
    }
}

/// Non-destructively injects or updates `code_dependencies` in YAML frontmatter.
pub fn inject_code_dependencies(
    content: &str,
    dependencies: &[&str],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (fm_opt, body) = split_frontmatter_and_body(content);
    let mut root_mapping: serde_yaml::Mapping = match fm_opt {
        Some(ref yaml_str) if !yaml_str.trim().is_empty() => serde_yaml::from_str(yaml_str)
            .map_err(|err| format!("Cannot safely update existing YAML frontmatter: {err}"))?,
        _ => serde_yaml::Mapping::new(),
    };

    let dep_values = dependencies
        .iter()
        .map(|dep| serde_yaml::Value::String((*dep).to_string()))
        .collect();
    root_mapping.insert(
        serde_yaml::Value::String("code_dependencies".to_string()),
        serde_yaml::Value::Sequence(dep_values),
    );

    let serialized_fm = serde_yaml::to_string(&root_mapping)?;
    let clean_fm = serialized_fm.trim();
    if body.is_empty() {
        Ok(format!("---\n{}\n---\n", clean_fm))
    } else {
        Ok(format!("---\n{}\n---\n{}", clean_fm, body))
    }
}

