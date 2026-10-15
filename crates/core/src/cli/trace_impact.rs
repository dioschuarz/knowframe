//! CLI command implementation for `llm-wiki-core trace-impact`.
//! Performs deterministic reverse impact analysis mapping changed code files to dependent markdown documents.

use std::path::Path;
use crate::delta::updater::{parse_git_diff_range, parse_git_diff_staged, FileDeltaItem, FileDeltaStatus};
use crate::drift::detector::{build_reverse_impact_index, detect_deterministic_code_drift};
use crate::drift::extractor::validate_path_containment;
use crate::drift::review::calculate_affected_documents;
use crate::model::AffectedDocumentsReport;

/// Generates the affected documents report for the given paths, range, or staged changes.
pub fn generate_trace_impact_report(
    workspace_root: &Path,
    paths: Option<Vec<String>>,
    range: Option<String>,
    _staged: bool,
) -> Result<AffectedDocumentsReport, Box<dyn std::error::Error + Send + Sync>> {
    let mut changed_items: Vec<FileDeltaItem> = Vec::new();

    if let Some(explicit_paths) = paths {
        for p in explicit_paths {
            let clean = p.replace('\\', "/");
            match validate_path_containment(&clean) {
                Ok(norm) => {
                    let abs = workspace_root.join(&norm);
                    let status = if abs.exists() {
                        FileDeltaStatus::Modified
                    } else {
                        FileDeltaStatus::Deleted
                    };
                    changed_items.push(FileDeltaItem { path: norm, status });
                }
                Err(err) => {
                    eprintln!("Warning: skipping invalid path '{}': {}", clean, err);
                }
            }
        }
    } else if let Some(r) = range {
        match parse_git_diff_range(workspace_root, &r) {
            Ok(items) => changed_items = items,
            Err(e) => return Err(format!("Failed to parse git diff range '{}': {}", r, e).into()),
        }
    } else {
        // Default or --staged
        match parse_git_diff_staged(workspace_root) {
            Ok(items) => changed_items = items,
            Err(e) => return Err(format!("Failed to parse staged git diff: {}", e).into()),
        }
    }

    // Discover all markdown documents in the workspace
    let mut doc_paths = Vec::new();
    discover_markdown_docs(workspace_root, workspace_root, &mut doc_paths);
    doc_paths.sort();

    // Build reverse impact index
    let reverse_index = build_reverse_impact_index(workspace_root, &doc_paths);

    // Calculate affected documents
    let mut report = calculate_affected_documents(
        workspace_root,
        &changed_items,
        &reverse_index,
        None, // Default gating config
        None, // In-memory active receipts
        None, // Commit message trailer
    );

    // Also detect any code drift findings (e.g. DELETED_CODE_FILE, OBSOLETE_CODE_REFERENCE)
    let drift_findings = detect_deterministic_code_drift(workspace_root, &changed_items, &reverse_index);
    for finding in drift_findings {
        if !report.findings.iter().any(|f| f.subject == finding.subject && f.rule_id == finding.rule_id) {
            report.findings.push(finding);
        }
    }

    // Sort report fields for deterministic bit-exact output
    report.changed_code_files.sort();
    report.obligations.sort_by(|a, b| a.doc_id.cmp(&b.doc_id));
    report.findings.sort_by(|a, b| a.finding_id.cmp(&b.finding_id));

    Ok(report)
}

/// Executes reverse impact analysis given paths, a git diff range, or staged changes.
pub fn execute_trace_impact(
    workspace_root: &Path,
    paths: Option<Vec<String>>,
    range: Option<String>,
    staged: bool,
    json: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let report = generate_trace_impact_report(workspace_root, paths, range, staged)?;

    if json {
        let serialized = serde_json::to_string_pretty(&report)?;
        println!("{}", serialized);
    } else {
        print_human_readable_report(&report);
    }

    Ok(())
}

fn discover_markdown_docs(root: &Path, current: &Path, acc: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(current) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(file_type) = entry.file_type() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                if name.starts_with('.') || name == "target" || name == "node_modules" {
                    continue;
                }
                if file_type.is_dir() {
                    discover_markdown_docs(root, &path, acc);
                } else if file_type.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    if let Ok(rel) = path.strip_prefix(root) {
                        acc.push(rel.to_string_lossy().replace('\\', "/"));
                    }
                }
            }
        }
    }
}

fn print_human_readable_report(report: &AffectedDocumentsReport) {
    println!("=== Trace Impact Analysis ===");
    println!("Changed Code Files: {}", report.changed_code_files.len());
    for f in &report.changed_code_files {
        println!("  • {}", f);
    }
    println!();
    println!("Affected Document Obligations: {}", report.obligations.len());
    for ob in &report.obligations {
        println!("  [{:?}] {}", ob.status, ob.doc_id);
        println!("    Triggered by: {}", ob.triggering_code_files.join(", "));
        let sources: Vec<String> = ob.dependency_sources.iter().map(|s| format!("{:?}", s)).collect();
        println!("    Sources:      {}", sources.join(", "));
    }
    println!();
    println!("Findings: {}", report.findings.len());
    for f in &report.findings {
        println!("  • [{:?}] {}: {} ({})", f.severity, f.rule_id, f.subject, f.required_action);
    }
    println!();
    println!("Summary: {} pending, {} reviewed, {} blocked",
        report.pending_count, report.reviewed_count, report.blocked_count);
}
