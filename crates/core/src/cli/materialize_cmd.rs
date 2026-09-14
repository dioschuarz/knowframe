//! CLI command: llm-wiki-core materialize <path>
//! Materializes section fragment metadata into Markdown YAML frontmatter.

use crate::ast::parser::parse_markdown_ast;
use crate::fragment::extractor::extract_fragments_from_document;
use crate::fragment::frontmatter::inject_materialized_frontmatter;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Materializes fragment metadata into a single markdown file on disk.
/// Returns true if the file was modified/updated.
pub fn materialize_file(file_path: &Path, stage: bool) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let content = fs::read_to_string(file_path)?;
    let rel_str = file_path.to_string_lossy().to_string();
    let parsed = parse_markdown_ast(&rel_str, &content);
    let fragments = extract_fragments_from_document(&content, &parsed.edges);

    let updated = inject_materialized_frontmatter(&content, &fragments)?;
    if updated != content {
        fs::write(file_path, &updated)?;
        if stage {
            let parent = file_path.parent().unwrap_or_else(|| Path::new("."));
            let _ = std::process::Command::new("git")
                .arg("add")
                .arg(file_path)
                .current_dir(parent)
                .output();
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Executes the materialize CLI command for a file or directory.
pub fn execute_materialize(
    target_path: &Path,
    stage: bool,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let mut updated_count = 0;

    if target_path.is_file() {
        if target_path.extension().is_some_and(|ext| ext == "md") {
            if materialize_file(target_path, stage)? {
                println!("[materialize] Materialized fragment metadata for '{}'", target_path.display());
                updated_count += 1;
            } else {
                println!("[materialize] Up to date: '{}'", target_path.display());
            }
        }
    } else if target_path.is_dir() {
        for entry in WalkDir::new(target_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                // Skip .git directory
                if path.components().any(|c| c.as_os_str() == ".git") {
                    continue;
                }
                if materialize_file(path, stage)? {
                    println!("[materialize] Materialized fragment metadata for '{}'", path.display());
                    updated_count += 1;
                }
            }
        }
        println!("[materialize] Completed: {} document(s) materialized.", updated_count);
    } else {
        return Err(format!("Target path '{}' does not exist", target_path.display()).into());
    }

    Ok(updated_count)
}
