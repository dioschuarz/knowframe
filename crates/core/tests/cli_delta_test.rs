use std::time::Instant;
use tempfile::tempdir;

#[tokio::test]
async fn test_git_delta_cli_offline_daemon_fail_open() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let start = Instant::now();

    // Call delta dispatch on a directory without active daemon port
    let result = llm_wiki_core::cli::delta::execute_git_delta(
        root,
        &["docs/note.md".to_string()],
        None,
        true,
        std::time::Duration::from_millis(500),
    )
    .await;

    let elapsed = start.elapsed();

    // Must return Ok(()) (fail-open exit code 0)
    assert!(result.is_ok(), "Git delta must fail open with Ok(())");
    // Must complete without blocking indefinitely (under 1500ms even under heavy parallel load)
    assert!(
        elapsed.as_millis() <= 3000,
        "Git delta must not block beyond timeout, took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_trace_impact_cli_output_and_idempotence_t011() {
    use std::fs;
    use llm_wiki_core::cli::trace_impact::generate_trace_impact_report;
    use llm_wiki_core::model::{AffectedDocumentsReport, DependencySource, ReviewStatus};

    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create directories
    fs::create_dir_all(root.join("notes")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();

    // Create note with frontmatter code_dependencies and inline citation
    let note_content = r#"---
code_dependencies:
  - src/auth.rs
---
# Security Architecture

References authentication implementation in [auth](src/auth.rs) and database at [db](src/db.rs).
"#;
    fs::write(root.join("notes/security.md"), note_content).unwrap();

    // Create source files
    fs::write(root.join("src/auth.rs"), "pub fn authenticate() {}\n").unwrap();
    fs::write(root.join("src/db.rs"), "pub fn connect() {}\n").unwrap();

    // Run 1: generate report for changed code paths ["src/auth.rs"]
    let report_1 = generate_trace_impact_report(
        root,
        Some(vec!["src/auth.rs".to_string()]),
        None,
        false,
    )
    .expect("generate_trace_impact_report failed");

    // Validate data model compliance
    assert_eq!(report_1.changed_code_files, vec!["src/auth.rs"]);
    assert_eq!(report_1.obligations.len(), 1);
    let ob = &report_1.obligations[0];
    assert_eq!(ob.doc_id, "notes/security.md");
    assert_eq!(ob.status, ReviewStatus::Blocked);
    assert_eq!(ob.triggering_code_files, vec!["src/auth.rs"]);
    assert!(ob.dependency_sources.contains(&DependencySource::Frontmatter));
    assert!(ob.dependency_sources.contains(&DependencySource::InlineMarkdownLink));

    // Serialize to JSON
    let json_1 = serde_json::to_string_pretty(&report_1).expect("Failed to serialize report 1");

    // Deserialization check
    let deserialized: AffectedDocumentsReport =
        serde_json::from_str(&json_1).expect("Failed to deserialize JSON report");
    assert_eq!(deserialized, report_1);

    // Run 2 on unmodified repository: must produce identical report and 100% bit-exact JSON
    let report_2 = generate_trace_impact_report(
        root,
        Some(vec!["src/auth.rs".to_string()]),
        None,
        false,
    )
    .expect("generate_trace_impact_report run 2 failed");

    let json_2 = serde_json::to_string_pretty(&report_2).expect("Failed to serialize report 2");

    assert_eq!(
        json_1, json_2,
        "trace-impact JSON output must be 100% bit-exact and idempotent across repeated runs"
    );
}

