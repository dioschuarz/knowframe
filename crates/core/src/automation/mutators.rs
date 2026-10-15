//! Deterministic AST and filesystem mutators generating unified diffs.
//! Implements FR-038 and FR-039.

use anyhow::Result;
use crate::model::TransformationAction;
use std::fs;
use std::path::Path;

/// Applies a single discrete transformation action to the filesystem.
pub fn apply_transformation_action(action: &TransformationAction) -> Result<()> {
    match action {
        TransformationAction::MoveFile { source, destination } => {
            let src = Path::new(source);
            let dst = Path::new(destination);
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::rename(src, dst)?;
            Ok(())
        }
        TransformationAction::UpdateFrontmatter { file, key, value } => {
            let path = Path::new(file);
            let content = if path.exists() {
                fs::read_to_string(path)?
            } else {
                String::new()
            };

            let updated = update_frontmatter_text(&content, key, value);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, updated)?;
            Ok(())
        }
        TransformationAction::RetargetLink { file, old_target, new_target, .. } => {
            let path = Path::new(file);
            let content = fs::read_to_string(path)?;
            let search_wikilink = format!("[[{}]]", old_target);
            let replace_wikilink = format!("[[{}]]", new_target);
            let updated = content.replace(&search_wikilink, &replace_wikilink);
            fs::write(path, updated)?;
            Ok(())
        }
        TransformationAction::ExtractSectionToFile { source_file, anchor, new_file } => {
            let src_path = Path::new(source_file);
            let dst_path = Path::new(new_file);
            let content = fs::read_to_string(src_path)?;

            // Simple section extraction based on heading
            let heading_prefix = format!("# {}", anchor);
            let mut extracted = String::new();
            let mut remaining = String::new();
            let mut in_section = false;

            for line in content.lines() {
                if line.contains(&heading_prefix) {
                    in_section = true;
                    extracted.push_str(line);
                    extracted.push('\n');
                } else if in_section && line.starts_with('#') {
                    in_section = false;
                    remaining.push_str(line);
                    remaining.push('\n');
                } else if in_section {
                    extracted.push_str(line);
                    extracted.push('\n');
                } else {
                    remaining.push_str(line);
                    remaining.push('\n');
                }
            }

            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dst_path, extracted)?;
            fs::write(src_path, remaining)?;
            Ok(())
        }
    }
}

fn update_frontmatter_text(content: &str, key: &str, value: &serde_json::Value) -> String {
    let formatted_val = match value {
        serde_json::Value::String(s) => s.clone(),
        _ => value.to_string(),
    };

    if content.starts_with("---\n") {
        if let Some(end_idx) = content[4..].find("\n---\n") {
            let fm_body = &content[4..4 + end_idx];
            let after_fm = &content[4 + end_idx + 5..];
            let new_line = format!("{}: {}", key, formatted_val);
            let mut lines: Vec<&str> = fm_body.lines().filter(|l| !l.starts_with(&format!("{}:", key))).collect();
            lines.push(&new_line);
            return format!("---\n{}\n---\n{}", lines.join("\n"), after_fm);
        }
    }

    // No existing frontmatter: prepend new frontmatter
    format!("---\n{}: {}\n---\n{}", key, formatted_val, content)
}

/// Generates a standard unified diff between two text strings.
pub fn generate_unified_diff(path: &str, old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let mut diff = String::new();
    diff.push_str(&format!("--- a/{}\n", path));
    diff.push_str(&format!("+++ b/{}\n", path));

    let max_len = old_lines.len().max(new_lines.len());
    for i in 0..max_len {
        let old_line = old_lines.get(i);
        let new_line = new_lines.get(i);

        match (old_line, new_line) {
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

    diff
}
