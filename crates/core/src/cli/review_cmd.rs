//! CLI command implementation for `llm-wiki-core review resolve <doc_id>`.
//! Records review receipts for affected documentation notes.

use std::path::Path;
use crate::drift::review::resolve_document_review;

/// Executes `review resolve` to approve a documentation review obligation without edits.
pub fn execute_review_resolve(
    workspace_root: &Path,
    doc_id: &str,
    commit_or_ref: Option<&str>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    resolve_document_review(workspace_root, &[doc_id.to_string()], commit_or_ref)
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
            format!("Failed to record review receipt for '{}': {}", doc_id, e).into()
        })?;

    println!(
        "Successfully recorded review receipt for '{}' (ref: {}) in .context-vault/reviews.json",
        doc_id,
        commit_or_ref.unwrap_or("HEAD")
    );
    Ok(())
}
